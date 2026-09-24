// SPDX-License-Identifier: MIT

//! `ai-profiles mcp`, run as a client runs it: the built binary, spoken to over
//! stdin and stdout, in a home of its own with nothing in it.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

use serde_json::{json, Value};

struct Server {
    child: Child,
    stdin: ChildStdin,
    lines: mpsc::Receiver<String>,
    _home: tempfile::TempDir,
}

impl Server {
    fn start() -> Server {
        let home = tempfile::tempdir().expect("a home");
        let mut child = Command::new(env!("CARGO_BIN_EXE_ai-profiles"))
            .arg("mcp")
            .env("HOME", home.path())
            .env_remove("CLAUDE_CONFIG_DIR")
            .env_remove("CODEX_HOME")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("the server starts");
        let stdin = child.stdin.take().expect("stdin");
        let stdout = child.stdout.take().expect("stdout");
        let (send, lines) = mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                if send.send(line).is_err() {
                    break;
                }
            }
        });
        Server {
            child,
            stdin,
            lines,
            _home: home,
        }
    }

    fn send(&mut self, message: Value) {
        writeln!(self.stdin, "{message}").expect("the server reads");
        self.stdin.flush().expect("flushed");
    }

    /// The reply to request `id`, skipping any notification before it.
    fn reply(&mut self, id: u64) -> Value {
        loop {
            let line = self
                .lines
                .recv_timeout(Duration::from_secs(20))
                .expect("a reply within 20 seconds");
            let message: Value = serde_json::from_str(&line).expect("stdout carries only JSON-RPC");
            if message["id"] == id {
                return message;
            }
        }
    }

    fn initialize(&mut self) -> Value {
        self.send(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": { "name": "test", "version": "0" },
            },
        }));
        let reply = self.reply(1);
        self.send(json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }));
        reply
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn it_introduces_itself_and_lists_its_tools() {
    let mut server = Server::start();
    let hello = server.initialize();
    assert_eq!(hello["result"]["serverInfo"]["name"], "ai-profiles");
    assert!(hello["result"]["capabilities"]["tools"].is_object());
    assert!(hello["result"]["instructions"]
        .as_str()
        .is_some_and(|text| text.contains("host/account")));

    server.send(json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" }));
    let tools = server.reply(2);
    let tools = tools["result"]["tools"].as_array().expect("tools");
    let named = |name: &str| {
        tools
            .iter()
            .find(|tool| tool["name"] == name)
            .unwrap_or_else(|| panic!("no tool {name}"))
    };
    for name in [
        "list_profiles",
        "get_usage",
        "list_sessions",
        "host_info",
        "list_folders",
        "new_session",
        "resume_session",
        "restart_session",
        "restart_outdated",
        "stop_session",
        "rename_session",
        "read_window",
        "send_to_window",
        "open_in_claude",
        "open_profile",
        "plan_move",
        "move_session",
        "archive_session",
        "restore_session",
        "delete_archive",
    ] {
        named(name);
    }
    assert_eq!(named("list_sessions")["annotations"]["readOnlyHint"], true);
    assert_eq!(
        named("delete_archive")["annotations"]["destructiveHint"],
        true
    );
    assert_eq!(
        named("move_session")["inputSchema"]["required"],
        json!(["profile", "session", "to"])
    );
}

#[test]
fn it_answers_a_call_and_explains_a_bad_name() {
    let mut server = Server::start();
    server.initialize();

    server.send(json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": { "name": "list_profiles", "arguments": {} },
    }));
    let listed = server.reply(3);
    assert_eq!(listed["result"]["isError"], json!(false), "{listed}");
    let text = listed["result"]["content"][0]["text"]
        .as_str()
        .expect("text");
    let profiles: Value = serde_json::from_str(text).expect("JSON");
    assert_eq!(profiles, json!({ "local": [], "hosts": [] }));

    server.send(json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": { "name": "list_sessions", "arguments": { "profile": "nas/marcus1" } },
    }));
    let refused = server.reply(4);
    assert_eq!(refused["result"]["isError"], true, "{refused}");
    let text = refused["result"]["content"][0]["text"]
        .as_str()
        .expect("text");
    assert!(text.contains("no hosts are paired"), "{text}");
}
