// SPDX-License-Identifier: MIT

//! Pure: what Claude calls a profile or a session, turned into the ids the
//! rest of the app works with. People name things; the tools take names.

use crate::remote::hosts::RemoteHost;

/// A profile on this Mac, as the tools know it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProfile {
    /// The profile's id, or `default:claude` / `default:codex`.
    pub id: String,
    /// What the sidebar calls it.
    pub name: String,
    /// Other names it answers to: its slug, `Default`, the app's name.
    pub aliases: Vec<String>,
}

/// Where a profile reference points.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    Local {
        id: String,
        name: String,
    },
    Remote {
        host_id: String,
        host_label: String,
        account: String,
    },
}

/// Pure: the profile `reference` names. `host/account` is an account on a
/// paired host (by the host's label or host name); anything else is a profile
/// on this Mac (by name, alias or id). Case doesn't matter.
pub fn profile(
    reference: &str,
    locals: &[LocalProfile],
    hosts: &[RemoteHost],
) -> Result<Target, String> {
    let reference = reference.trim();
    if let Some((host, account)) = reference.split_once('/') {
        let host = remote_host(host.trim(), hosts)?;
        let account = account.trim();
        if account.is_empty() {
            return Err(format!(
                "{reference:?} names no account: write it as {}/<account>.",
                host.label
            ));
        }
        return Ok(Target::Remote {
            host_id: host.id.clone(),
            host_label: host.label.clone(),
            account: account.to_owned(),
        });
    }
    let matches: Vec<&LocalProfile> = locals
        .iter()
        .filter(|local| {
            local.id == reference
                || local.name.eq_ignore_ascii_case(reference)
                || local
                    .aliases
                    .iter()
                    .any(|alias| alias.eq_ignore_ascii_case(reference))
        })
        .collect();
    match matches.as_slice() {
        [one] => Ok(Target::Local {
            id: one.id.clone(),
            name: one.name.clone(),
        }),
        [] => Err(format!(
            "No profile is called {reference:?}. Profiles on this Mac: {}. An account on a host is written host/account{}.",
            names(locals.iter().map(|local| local.name.as_str())),
            if hosts.is_empty() {
                String::new()
            } else {
                format!(
                    " (hosts: {})",
                    names(hosts.iter().map(|host| host.label.as_str()))
                )
            }
        )),
        several => Err(format!(
            "{reference:?} could be {}: use the profile's id.",
            names(several.iter().map(|local| local.id.as_str()))
        )),
    }
}

/// Pure: the paired host `name` means, by label or host name.
pub fn remote_host<'a>(name: &str, hosts: &'a [RemoteHost]) -> Result<&'a RemoteHost, String> {
    let matches: Vec<&RemoteHost> = hosts
        .iter()
        .filter(|host| {
            host.id == name
                || host.label.eq_ignore_ascii_case(name)
                || host.hostname.eq_ignore_ascii_case(name)
        })
        .collect();
    match matches.as_slice() {
        [one] => Ok(one),
        [] if hosts.is_empty() => Err(format!(
            "No host is called {name:?}: no hosts are paired with this Mac."
        )),
        [] => Err(format!(
            "No host is called {name:?}. Paired hosts: {}.",
            names(hosts.iter().map(|host| host.label.as_str()))
        )),
        several => Err(format!(
            "{name:?} could be {}: rename one of them in Settings.",
            names(several.iter().map(|host| host.label.as_str()))
        )),
    }
}

/// A session, as the tools see it for resolving a reference.
#[derive(Debug, Clone, Copy)]
pub struct SessionName<'a> {
    pub id: &'a str,
    pub title: Option<&'a str>,
}

/// The shortest id prefix a reference may use: long enough that it's plainly
/// meant as an id.
const MIN_PREFIX: usize = 8;

/// Pure: the session `reference` names among `sessions`: its id, a unique
/// prefix of it, or its exact title.
pub fn session(reference: &str, sessions: &[SessionName<'_>]) -> Result<String, String> {
    let reference = reference.trim();
    if reference.is_empty() {
        return Err("Name a session: its id or its title.".into());
    }
    if let Some(found) = sessions.iter().find(|session| session.id == reference) {
        return Ok(found.id.to_owned());
    }
    let lowered = reference.to_ascii_lowercase();
    if reference.len() >= MIN_PREFIX {
        let prefixed: Vec<&SessionName> = sessions
            .iter()
            .filter(|session| session.id.starts_with(&lowered))
            .collect();
        if let [one] = prefixed.as_slice() {
            return Ok(one.id.to_owned());
        }
    }
    let titled: Vec<&SessionName> = sessions
        .iter()
        .filter(|session| {
            session
                .title
                .is_some_and(|title| title.trim().eq_ignore_ascii_case(reference))
        })
        .collect();
    match titled.as_slice() {
        [one] => Ok(one.id.to_owned()),
        [] => Err(format!(
            "No session is called {reference:?}. Sessions here: {}.",
            if sessions.is_empty() {
                "none".to_owned()
            } else {
                names(
                    sessions
                        .iter()
                        .take(15)
                        .map(|session| session.title.unwrap_or(session.id)),
                )
            }
        )),
        several => Err(format!(
            "More than one session is called {reference:?}: use its id ({}).",
            names(several.iter().map(|session| session.id))
        )),
    }
}

fn names<'a>(names: impl Iterator<Item = &'a str>) -> String {
    let all: Vec<&str> = names.collect();
    if all.is_empty() {
        "none".to_owned()
    } else {
        all.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local(id: &str, name: &str, aliases: &[&str]) -> LocalProfile {
        LocalProfile {
            id: id.into(),
            name: name.into(),
            aliases: aliases.iter().map(|alias| (*alias).into()).collect(),
        }
    }

    fn host(id: &str, label: &str, hostname: &str) -> RemoteHost {
        serde_json::from_value(serde_json::json!({
            "id": id,
            "label": label,
            "hostname": hostname,
            "addresses": ["10.0.0.1:7443"],
            "fingerprint": "00",
            "clientId": "c",
            "pairedAt": "2026-09-22T00:00:00Z",
        }))
        .expect("a host")
    }

    fn locals() -> Vec<LocalProfile> {
        vec![
            local("p1", "Marcus1", &["marcus1"]),
            local("p2", "Marcus2", &["marcus2"]),
            local("default:claude", "Default", &["default", "Claude"]),
        ]
    }

    #[test]
    fn a_local_profile_by_name_alias_or_id_any_case() {
        let hosts = [host("h1", "xjopa1", "xjopa1")];
        for reference in ["Marcus1", "marcus1", "MARCUS1", "p1", " Marcus1 "] {
            assert_eq!(
                profile(reference, &locals(), &hosts),
                Ok(Target::Local {
                    id: "p1".into(),
                    name: "Marcus1".into()
                }),
                "{reference}"
            );
        }
        assert_eq!(
            profile("default", &locals(), &hosts),
            Ok(Target::Local {
                id: "default:claude".into(),
                name: "Default".into()
            })
        );
    }

    #[test]
    fn a_remote_account_by_host_label_or_hostname() {
        let hosts = [
            host("h1", "xjopa1", "xjopa1"),
            host("h2", "Home Assistant", "a0d7b954-ssh"),
        ];
        assert_eq!(
            profile("xjopa1/marcus1", &locals(), &hosts),
            Ok(Target::Remote {
                host_id: "h1".into(),
                host_label: "xjopa1".into(),
                account: "marcus1".into()
            })
        );
        assert_eq!(
            profile("home assistant/marcus2", &locals(), &hosts),
            Ok(Target::Remote {
                host_id: "h2".into(),
                host_label: "Home Assistant".into(),
                account: "marcus2".into()
            })
        );
        assert_eq!(
            profile("a0d7b954-ssh/ marcus2", &locals(), &hosts),
            Ok(Target::Remote {
                host_id: "h2".into(),
                host_label: "Home Assistant".into(),
                account: "marcus2".into()
            })
        );
    }

    #[test]
    fn unknown_names_list_what_there_is() {
        let hosts = [host("h1", "xjopa1", "xjopa1")];
        let error = profile("Marcus3", &locals(), &hosts).unwrap_err();
        assert!(error.contains("Marcus1, Marcus2, Default"), "{error}");
        assert!(error.contains("xjopa1"), "{error}");
        let error = profile("nas/marcus1", &locals(), &hosts).unwrap_err();
        assert!(error.contains("Paired hosts: xjopa1"), "{error}");
        let error = profile("xjopa1/", &locals(), &hosts).unwrap_err();
        assert!(error.contains("xjopa1/<account>"), "{error}");
        let error = profile("x/y", &locals(), &[]).unwrap_err();
        assert!(error.contains("no hosts are paired"), "{error}");
    }

    #[test]
    fn two_profiles_with_one_name_ask_for_the_id() {
        let both = vec![local("p1", "Work", &[]), local("p9", "work", &[])];
        let error = profile("work", &both, &[]).unwrap_err();
        assert!(error.contains("p1, p9"), "{error}");
    }

    #[test]
    fn a_session_by_id_prefix_or_title() {
        let sessions = [
            SessionName {
                id: "4f1c2d3e-0000-4000-8000-000000000001",
                title: Some("Brain-Dev-Server (xJOPA)"),
            },
            SessionName {
                id: "4f1c9999-0000-4000-8000-000000000002",
                title: Some("vb4"),
            },
            SessionName {
                id: "77aa0000-0000-4000-8000-000000000003",
                title: None,
            },
        ];
        assert_eq!(
            session("4f1c2d3e-0000-4000-8000-000000000001", &sessions),
            Ok("4f1c2d3e-0000-4000-8000-000000000001".into())
        );
        assert_eq!(
            session("4F1C2D3E", &sessions),
            Ok("4f1c2d3e-0000-4000-8000-000000000001".into())
        );
        assert_eq!(
            session("brain-dev-server (xjopa)", &sessions),
            Ok("4f1c2d3e-0000-4000-8000-000000000001".into())
        );
        assert_eq!(
            session("VB4", &sessions),
            Ok("4f1c9999-0000-4000-8000-000000000002".into())
        );
        // A short prefix is too loose to be an id.
        assert!(session("4f1c", &sessions).is_err());
        let error = session("nope", &sessions).unwrap_err();
        assert!(error.contains("vb4"), "{error}");
        assert!(error.contains("77aa0000"), "{error}");
    }

    #[test]
    fn two_sessions_with_one_title_ask_for_the_id() {
        let sessions = [
            SessionName {
                id: "aaaaaaaa-0000-4000-8000-000000000001",
                title: Some("FOAWA"),
            },
            SessionName {
                id: "bbbbbbbb-0000-4000-8000-000000000002",
                title: Some("foawa"),
            },
        ];
        let error = session("FOAWA", &sessions).unwrap_err();
        assert!(error.contains("aaaaaaaa-0000"), "{error}");
        assert!(error.contains("bbbbbbbb-0000"), "{error}");
    }
}
