<p align="center">
  <img alt="ai-profiles-remote" src="apps/ai-profiles/public/icon.svg" width="96">
</p>

<h1 align="center">ai-profiles-remote</h1>

<p align="center">
  <b>Manage Claude Code CLI for Remote Control, across several servers and accounts.</b><br>
  One Mac app to sign in, start, stop, restart and move Claude Code sessions on your Linux machines,<br>
  and to keep several Claude Desktop profiles side by side.
</p>

<p align="center">
  <a href="https://github.com/marcusadolfsson/ai-profiles-remote/releases/latest"><b>Download for macOS</b></a>
  &nbsp;·&nbsp;
  <a href="#get-started-in-three-steps">Get started</a>
  &nbsp;·&nbsp;
  <a href="#how-this-differs">How this differs</a>
</p>

<p align="center">
  <img alt="A Claude account on a Linux host: running and previous sessions" src="docs/screenshots/remote-profile.png" width="860">
</p>

Remote Control lets you work with Claude Code from the Claude app on your phone, another Mac or the web.
But the session still has to run somewhere, and someone has to start it, keep it alive, restart it
when Claude updates, and decide which account it runs under. **ai-profiles-remote does that job, for every
server and every account, from your Mac.**

## What you get

### 🔑 Easy login
Add a Claude account to a server and sign in **in your Mac's own browser**, with Claude's usual OAuth page.
Paste the code it shows into the app, and you're done. You don't need an ssh session or a browser on the
server, and you won't be copying links out of a headless terminal.

### ▶️ Start, stop, restart, with Remote Control on
Every session runs in its own tmux window with **Remote Control enabled from the first second**, so it keeps
going when nothing's attached and shows up in the Claude apps right away. Start a new one in any folder,
resume an old one, rename it (the name follows it to Remote Control and tmux), stop it or restart it.
Sessions come back by themselves after the server reboots.

<table>
  <tr>
    <td width="50%"><img alt="New session on a remote host" src="docs/screenshots/remote-new-session.png"><br><sub>A new session: a folder on the host, a name, and the folder trusted for you.</sub></td>
    <td width="50%"><img alt="A session's tmux window inside the app" src="docs/screenshots/remote-session-window.png"><br><sub>A session waiting on a question? Its window, live, inside the app.</sub></td>
  </tr>
</table>

### 📱 Launch right into Remote Control
One click opens a running session in the Claude app for that account on your Mac. It's already in the
Claude app on your phone. Need the terminal after all? **Open in Terminal** attaches over ssh.

### 🔀 Move sessions between accounts
Hit a usage limit on one account? Move the session to another one, on the same server or between profiles
on your Mac, and carry on where you left off. Everything goes with it: the transcript, subagents, file
history, todos and plans.

- **Project memory merges.** Notes only one side changed merge by themselves. For notes both sides
  changed, you choose: keep the newer, take either side, or let Claude write a merge you read first.
- **Nothing's lost.** Anything replaced is backed up first. The copy left behind can be archived, deleted
  once the moved copy checks out identical, or kept.
- **Archive and restore.** Archives are compressed: a 300 MB transcript keeps in about 100.

<table>
  <tr>
    <td width="50%"><img alt="Moving a session to another account" src="docs/screenshots/remote-move-session.png"><br><sub>Moving a running session: it exits first, and anything replaced is backed up.</sub></td>
    <td width="50%"><img alt="Archived sessions" src="docs/screenshots/remote-archived.png"><br><sub>Archived sessions, compressed, with restore and delete.</sub></td>
  </tr>
</table>

### 🔔 Restart-to-update alerts
Claude Code updates itself on the server, but a running session keeps the old version until it restarts.
The app spots those sessions and **flags them in amber**. **Restart all** brings them up on the new version,
back in the same conversation and with Remote Control on.

<p align="center">
  <img alt="Running sessions, two flagged Restart to update, and Restart all · 2 to update" src="docs/screenshots/remote-update-alert.png" width="720">
</p>

### 🖥️ Several servers, several accounts
Pair as many Linux machines as you like: a VM in the cloud, a box under the desk, a Docker container on
your NAS. Each shows up in the sidebar under its name, with its accounts as profiles you can color and put
on ⌘1…⌘9.

### 🧑‍🤝‍🧑 Multiple profiles for Claude Desktop
It's still [ai-profiles](https://github.com/bartekczyz/ai-profiles) underneath. Run several Claude Desktop
(and ChatGPT) accounts side by side, each with its own launcher tinted in its color, its own CLI wrapper
and its own usage meters. Each Claude profile keeps its own Claude Code sessions and settings. A
**Sessions** panel lists them and moves them between profiles.

## Requirements

- **Your Mac:** macOS, Apple silicon or Intel (one universal build).
- **Each server:** Linux on x86_64 or arm64, with **tmux 3.0+** and **Claude Code**. A systemd user service
  keeps it running. Anything that restarts a process works too, such as a container's start script.
- **The network:** **the Mac and the servers must be on the same Tailnet, WireGuard VPN or LAN.** The server
  isn't meant for the open internet: it answers only addresses you allow (Tailscale, WireGuard and the
  machine itself by default, a LAN if you add it), and drops everyone else before TLS.

## Get started in three steps

**1. Install the app.** Download the `.dmg` from
[Releases](https://github.com/marcusadolfsson/ai-profiles-remote/releases/latest). It's signed and notarized.
Drag `ai-profiles-remote.app` to Applications.

**2. Install the server** on each Linux machine:

```sh
sudo apt install tmux
curl -fsSL https://claude.ai/install.sh | bash
mkdir -p ~/.local/bin && curl -fsSL https://github.com/marcusadolfsson/ai-profiles-remote/releases/latest/download/ai-profiles-server-$(uname -m)-linux -o ~/.local/bin/ai-profiles-server && chmod +x ~/.local/bin/ai-profiles-server
ai-profiles-server setup
```

`setup` walks you through it:
1. It checks tmux and Claude Code, and finds your accounts.
2. It asks which networks may connect.
3. It installs the service.
4. It ends with a pairing code.

**3. Pair.** In the app, go to **Settings → Remote hosts → Pair a host** and paste the code. The same steps
are in the app under **How to set up a host**.

<p align="center">
  <img alt="Settings, Remote hosts, with the setup guide" src="docs/screenshots/remote-setup-guide.png" width="720">
</p>

Later, `ai-profiles-server doctor` checks an install, `pair` makes a new code, and `revoke` cuts a Mac off.
The server is a single static binary. You can also build it from source with
`cargo install --locked --git https://github.com/marcusadolfsson/ai-profiles-remote ai-profiles-server`.

The app doesn't update itself. The upstream updater would replace it with a build that doesn't have these
features, so it's switched off. Take new versions from Releases.

## How this differs

There are good tools near each half of this, but I didn't find one that does both, or that moves a session
between accounts:

- **Multi-account switchers** ([clauth](https://github.com/uwuclxdy/clauth),
  [ClaudeCodeMultiAccounts](https://github.com/Leuconoe/ClaudeCodeMultiAccounts), and many small
  `CLAUDE_CONFIG_DIR` switchers) change which account a terminal uses. They don't know about sessions,
  and they don't move one from account to account.
- **Session managers and remotes** ([happy](https://github.com/slopus/happy),
  [Claude Code UI](https://github.com/siteboon/claudecodeui), [claude-squad](https://github.com/smtg-ai/claude-squad),
  [ccmanager](https://github.com/kbwo/ccmanager), [hive](https://github.com/latagore/hive),
  [remy](https://github.com/padamchopra/remy)) run or mirror sessions. Most work with one account, and most
  go through a UI or relay of their own.

What ai-profiles-remote adds:

1. **One place for accounts and their sessions,** on the Mac and on your servers, side by side.
2. **Moving a session between accounts,** memory and all.
3. **Remote Control, not another client.** You work in the official Claude apps. This app starts, stops,
   restarts, renames and moves the sessions behind them.
4. **Built for a Linux host:** a small server with pinned TLS that brings sessions back after a reboot,
   and accounts that sign in through your Mac's browser.

## Security

- **Transport:** HTTPS (TLS 1.3) with the server's own certificate. The pairing code carries its
  fingerprint, and the Mac trusts that certificate and no other.
- **Pairing and tokens:** a code works once, within 10 minutes. The token it's exchanged for lives in the
  Keychain, and the server keeps only its hash.
- **Who can connect:** only addresses you allow. Everyone else is dropped before TLS, and failed attempts
  are rate-limited.
- **Running commands:** `claude` and tmux start with argument lists, never through a shell. The server
  types only into windows it opened, and starts sessions only in folders you've allowed.

## Built on ai-profiles

This is a fork of [bartekczyz/ai-profiles](https://github.com/bartekczyz/ai-profiles) by Bartek Czyż, the
macOS app for running several Claude and ChatGPT accounts side by side. Everything it does, this does too;
its [README](https://github.com/bartekczyz/ai-profiles#readme) covers that part. If you find it useful,
the About dialog links to Bartek's Buy Me a Coffee.

The Mac-side changes are proposed upstream, one pull request each:

| PR | What it adds | Status |
|---|---|---|
| [#47](https://github.com/bartekczyz/ai-profiles/pull/47) | Security hardening | merged |
| [#48](https://github.com/bartekczyz/ai-profiles/pull/48) | Each Claude Desktop profile gets its own Claude Code config | merged |
| [#49](https://github.com/bartekczyz/ai-profiles/pull/49) | The default profile can be renamed | merged |
| [#50](https://github.com/bartekczyz/ai-profiles/pull/50) | Sessions: list, move, archive, restore | open |
| [#51](https://github.com/bartekczyz/ai-profiles/pull/51) | A color and ⌘1 for the default profile | open |
| [#52](https://github.com/bartekczyz/ai-profiles/pull/52) | A wrapper is named after its profile in the menu bar | open |
| [#53](https://github.com/bartekczyz/ai-profiles/pull/53) | The account a profile is signed in under | open |

**Remote hosts** is a big add-on, so it lives here first (branch `feat/remote-host`).

<details>
<summary><b>Worth knowing about the Claude desktop app, whether or not you use this</b></summary>

If the desktop app can't find the transcript one of its session records points at, it doesn't drop the
record. It quietly starts a new, empty CLI session under it. That's how a session "disappears" after a
profile's launcher is rebuilt: the record stays, but the history doesn't. The Sessions panel moves the
record together with the transcript, so this can't happen on a move.
</details>

## Building from source

```sh
git clone https://github.com/marcusadolfsson/ai-profiles-remote.git
cd ai-profiles-remote
pnpm install
pnpm --filter ai-profiles tauri build
```

The `.app` lands in `apps/ai-profiles/src-tauri/target/release/bundle/macos/`, and the `.dmg` beside it in
`dmg/`. A local build isn't notarized, so macOS warns on first launch: right-click the app → Open → Open.

To build the server for Linux from a Mac, run
`cargo zigbuild --release -p ai-profiles-server --target x86_64-unknown-linux-musl` in
`apps/ai-profiles/src-tauri`.

**Tests:** run `cargo test` and `cargo clippy --all-targets -- -D warnings` in `apps/ai-profiles/src-tauri`,
and `pnpm test` in `apps/ai-profiles`.
- Under Node 25 and later, vitest needs `NODE_OPTIONS=--no-experimental-webstorage`.
- One test, *"shows the authoritative Codex reset count 2"*, fails on upstream `main` as well.

**Branches:**
- `main` is upstream's, plus #50, #53, remote hosts, and the fork's own changes (its name and icon,
  releases, the updater switched off).
- Each upstream pull request has its own branch.
