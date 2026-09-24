//! Signing an account in from the Mac.
//!
//! `claude auth login --claudeai` prints a sign-in URL, then reads the code
//! the sign-in page shows from stdin. So the server runs it with pipes,
//! hands the URL to the client (which opens it in the Mac's browser), and
//! types in the code the user pastes back. Whether it worked is decided by
//! `claude auth status --json` saying the account is logged in, not by what
//! the command printed.
//!
//! A failed code ends the command, so a retry starts a new sign-in (with a
//! new URL). Sign-ins are kept 10 minutes at most, one per account and two
//! at once, and their processes are killed when abandoned.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use crate::accounts::AccountDir;

/// How long the command gets to print its URL.
const URL_TIMEOUT: Duration = Duration::from_secs(45);
/// How long a sign-in waits for its code.
pub const LOGIN_TTL: Duration = Duration::from_secs(10 * 60);
/// How long the command gets to finish once the code is in.
const FINISH_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_ACTIVE: usize = 2;

/// Where the sign-in page may be. A URL anywhere else is refused, whatever
/// the command printed.
const ALLOWED_HOSTS: &[&str] = &[
    "claude.com",
    "claude.ai",
    "platform.claude.com",
    "console.anthropic.com",
];

#[derive(Debug, PartialEq, Eq)]
pub enum LoginError {
    /// Two sign-ins are already under way.
    Busy,
    /// No sign-in with that id (finished, cancelled or expired).
    NotFound,
    /// The code doesn't look like one.
    BadCode,
    /// It didn't work. `retryable`: starting again may.
    Failed { message: String, retryable: bool },
}

struct Login {
    account: String,
    child: Mutex<Child>,
    stdin: Mutex<Option<ChildStdin>>,
    output: Arc<Mutex<String>>,
    started: Instant,
}

impl Drop for Login {
    fn drop(&mut self) {
        let mut child = self.child.lock().unwrap_or_else(PoisonError::into_inner);
        let _ = child.kill();
        let _ = child.wait();
    }
}

#[derive(Default)]
pub struct Logins {
    active: Mutex<HashMap<String, Arc<Login>>>,
}

pub struct Started {
    pub id: String,
    pub url: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl Logins {
    fn active(&self) -> std::sync::MutexGuard<'_, HashMap<String, Arc<Login>>> {
        let mut active = self.active.lock().unwrap_or_else(PoisonError::into_inner);
        active.retain(|_, login| login.started.elapsed() < LOGIN_TTL);
        active
    }

    /// A sign-in is under way for `account`.
    pub fn in_progress(&self, account: &str) -> bool {
        self.active().values().any(|login| login.account == account)
    }

    /// Start signing `account` in. A sign-in already under way for it is
    /// dropped (the user started over).
    pub fn start(
        &self,
        account: &AccountDir,
        claude: &Path,
        cwd: &Path,
    ) -> Result<Started, LoginError> {
        {
            let mut active = self.active();
            active.retain(|_, login| login.account != account.name);
            if active.len() >= MAX_ACTIVE {
                return Err(LoginError::Busy);
            }
        }
        let mut command = Command::new(claude);
        command
            .args(["auth", "login", "--claudeai"])
            .current_dir(cwd)
            .env("BROWSER", "/bin/false")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if account.is_default {
            command.env_remove("CLAUDE_CONFIG_DIR");
        } else {
            command.env("CLAUDE_CONFIG_DIR", &account.dir);
        }
        let mut child = command.spawn().map_err(|err| LoginError::Failed {
            message: format!("could not run claude: {err}"),
            retryable: false,
        })?;
        let output = Arc::new(Mutex::new(String::new()));
        for stream in [
            child
                .stdout
                .take()
                .map(|s| Box::new(s) as Box<dyn Read + Send>),
            child
                .stderr
                .take()
                .map(|s| Box::new(s) as Box<dyn Read + Send>),
        ]
        .into_iter()
        .flatten()
        {
            let output = output.clone();
            thread::spawn(move || collect(stream, &output));
        }
        let login = Arc::new(Login {
            account: account.name.clone(),
            stdin: Mutex::new(child.stdin.take()),
            child: Mutex::new(child),
            output,
            started: Instant::now(),
        });

        let deadline = Instant::now() + URL_TIMEOUT;
        let url = loop {
            if let Some(url) = sign_in_url(&text(&login.output)) {
                break url;
            }
            if exited(&login).is_some() || Instant::now() > deadline {
                return Err(LoginError::Failed {
                    message: format!(
                        "claude didn't offer a sign-in link. It printed: {}",
                        last_line(&text(&login.output))
                    ),
                    retryable: true,
                });
            }
            thread::sleep(Duration::from_millis(100));
        };
        if !url_allowed(&url) {
            return Err(LoginError::Failed {
                message:
                    "claude offered a sign-in link to an unexpected site, so it was not passed on."
                        .into(),
                retryable: false,
            });
        }
        let id = uuid::Uuid::new_v4().to_string();
        self.active().insert(id.clone(), login);
        Ok(Started {
            id,
            url,
            expires_at: chrono::Utc::now() + chrono::Duration::from_std(LOGIN_TTL).expect("fits"),
        })
    }

    /// Type the code in and wait for the command to finish. Ok (with the
    /// account's name) means it finished happily; the caller confirms the
    /// account really is signed in with [`signed_in`].
    pub fn submit(&self, id: &str, code: &str) -> Result<String, LoginError> {
        let code = code.trim();
        let plausible =
            !code.is_empty() && code.len() <= 512 && code.chars().all(|c| c.is_ascii_graphic());
        if !plausible {
            return Err(LoginError::BadCode);
        }
        let login = self.active().remove(id).ok_or(LoginError::NotFound)?;
        {
            let mut stdin = login.stdin.lock().unwrap_or_else(PoisonError::into_inner);
            let Some(mut pipe) = stdin.take() else {
                return Err(LoginError::NotFound);
            };
            let _ = pipe.write_all(format!("{code}\n").as_bytes());
            let _ = pipe.flush();
            // Dropped: the command sees end of input after the code.
        }
        let deadline = Instant::now() + FINISH_TIMEOUT;
        let status = loop {
            if let Some(status) = exited(&login) {
                break Some(status);
            }
            if Instant::now() > deadline {
                break None;
            }
            thread::sleep(Duration::from_millis(100));
        };
        if status.is_some_and(|status| status.success()) {
            return Ok(login.account.clone());
        }
        let printed = last_line(&text(&login.output));
        Err(LoginError::Failed {
            message: if printed.contains("Login failed") || printed.contains("400") {
                "The code wasn't accepted. Codes work once and only for a few minutes: sign in again for a new one.".into()
            } else if status.is_none() {
                "claude didn't finish signing in. Sign in again.".into()
            } else {
                format!("Signing in didn't work. claude printed: {printed}")
            },
            retryable: true,
        })
    }

    pub fn cancel(&self, id: &str) -> bool {
        self.active().remove(id).is_some()
    }
}

fn collect(stream: Box<dyn Read + Send>, output: &Mutex<String>) {
    for line in BufReader::new(stream).split(b'\n').map_while(Result::ok) {
        let mut output = output.lock().unwrap_or_else(PoisonError::into_inner);
        output.push_str(&String::from_utf8_lossy(&line));
        output.push('\n');
    }
}

fn text(output: &Mutex<String>) -> String {
    output
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone()
}

fn exited(login: &Login) -> Option<std::process::ExitStatus> {
    login
        .child
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .try_wait()
        .ok()
        .flatten()
}

/// The first https URL in what the command printed.
pub fn sign_in_url(output: &str) -> Option<String> {
    output
        .split_whitespace()
        .find(|word| word.starts_with("https://"))
        .map(str::to_owned)
}

/// An https URL on one of the sign-in hosts.
pub fn url_allowed(url: &str) -> bool {
    let Some(rest) = url.strip_prefix("https://") else {
        return false;
    };
    let host = rest.split(['/', '?', '#']).next().unwrap_or_default();
    !host.contains('@') && !host.contains(':') && ALLOWED_HOSTS.contains(&host)
}

fn last_line(output: &str) -> String {
    output
        .lines()
        .map(str::trim)
        .rfind(|line| !line.is_empty())
        .unwrap_or("nothing")
        .chars()
        .take(300)
        .collect()
}

/// How long `claude auth status` gets.
const STATUS_TIMEOUT: Duration = Duration::from_secs(15);

/// `claude auth status --json` says the account is logged in, within a few
/// seconds.
pub fn signed_in(claude: &Path, account: &AccountDir) -> bool {
    let mut command = Command::new(claude);
    command.args(["auth", "status", "--json"]);
    if account.is_default {
        command.env_remove("CLAUDE_CONFIG_DIR");
    } else {
        command.env("CLAUDE_CONFIG_DIR", &account.dir);
    }
    ai_profiles_core::child::run_within(&mut command, Vec::new(), STATUS_TIMEOUT)
        .ok()
        .flatten()
        .and_then(|output| serde_json::from_slice::<serde_json::Value>(&output.stdout).ok())
        .and_then(|status| status.get("loggedIn")?.as_bool())
        .unwrap_or(false)
}

/// A folder of the server's own to run sign-ins in, so a stray file in the
/// user's home can't affect them.
pub fn login_cwd(state_dir: &Path) -> PathBuf {
    let dir = state_dir.join("login-cwd");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_sign_in_link_in_what_claude_prints() {
        let printed = "Opening browser to sign in…\nIf the browser didn't open, visit: https://claude.com/cai/oauth/authorize?code=true&state=x\nPaste code here if prompted > ";
        assert_eq!(
            sign_in_url(printed).as_deref(),
            Some("https://claude.com/cai/oauth/authorize?code=true&state=x")
        );
        assert_eq!(sign_in_url("no link here"), None);
    }

    #[test]
    fn passes_on_only_links_to_the_sign_in_sites() {
        assert!(url_allowed("https://claude.com/cai/oauth/authorize?x=1"));
        assert!(url_allowed("https://platform.claude.com/oauth"));
        assert!(!url_allowed("http://claude.com/x"));
        assert!(!url_allowed("https://claude.com.evil.example/x"));
        assert!(!url_allowed("https://evil.example/claude.com"));
        assert!(!url_allowed("https://user@claude.com/x"));
        assert!(!url_allowed("https://claude.com:8443/x"));
    }

    #[test]
    fn keeps_the_last_thing_said() {
        assert_eq!(last_line("a\nLogin failed: 400\n\n"), "Login failed: 400");
        assert_eq!(last_line(""), "nothing");
    }
}
