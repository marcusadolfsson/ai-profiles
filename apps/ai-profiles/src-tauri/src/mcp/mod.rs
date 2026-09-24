// SPDX-License-Identifier: MIT

//! `ai-profiles mcp`: an MCP server on stdin and stdout, for Claude Desktop,
//! Claude Code or any other MCP client on this Mac.
//!
//! It is the app without its window: the same code the app's commands call,
//! run in this process, so it works whether or not the app is open. The
//! app's lists catch up on their own (a host's every 15 seconds, this Mac's
//! when the window is next focused). Stdout carries the protocol, so
//! anything else goes to stderr.

pub mod install;
mod refs;
mod tools;

use rmcp::ServiceExt;

/// Serve MCP until the client closes stdin.
pub fn serve() -> Result<(), String> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|err| format!("could not start: {err}"))?;
    runtime.block_on(async {
        let service = tools::AiProfiles::new()
            .serve(rmcp::transport::stdio())
            .await
            .map_err(|err| format!("could not start the MCP session: {err}"))?;
        service
            .waiting()
            .await
            .map_err(|err| format!("the MCP session ended badly: {err}"))?;
        Ok(())
    })
}
