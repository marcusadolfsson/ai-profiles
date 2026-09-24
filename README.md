<p align="center">
  <img alt="Remote Control Conductor" src="apps/ai-profiles/public/icon.svg" width="96">
</p>

<h1 align="center">Remote Control Conductor</h1>

<p align="center">
  <b>Run Claude Code CLI on your servers, with Remote Control, across several accounts.</b><br>
  One Mac app and a small Linux server: sign in, start, stop, restart and switch accounts<br>
  for Claude Code sessions on every server. With a built-in MCP server, Claude can do it for you.
</p>

<p align="center">
  <a href="https://github.com/marcusadolfsson/remote-control-conductor/releases/latest"><b>Download for macOS</b></a>
  &nbsp;·&nbsp;
  <a href="#get-started-in-three-steps">Get started</a>
  &nbsp;·&nbsp;
  <a href="#-let-claude-run-it-over-mcp">MCP server</a>
  &nbsp;·&nbsp;
  <a href="#how-this-differs">How this differs</a>
</p>

<p align="center">
  <img alt="A project on a Linux host, FOAWA on xjopa1: its running and previous sessions" src="docs/screenshots/remote-profile.png" width="860">
</p>

Remote Control lets you work with a Claude Code CLI session from the Claude app on your phone, your Mac or
the web. On a server, though, that session still has to run somewhere, and someone has to start it, keep
it alive, restart it when Claude updates, and decide which account it runs under. **Remote Control
Conductor does that job for Claude Code on your servers, from your Mac,** with a small server on each Linux
machine.

(Between Claude Desktop installs, Remote Control already works well as it is. This is for the CLI on
servers.) And because it's also an MCP server, you can just ask Claude:
*"restart the claudemulti session on xjopa1"*.

## How to set it up: accounts and projects

- **Add each of your Claude accounts once, as a desktop profile on your Mac.** One per login, each with
  its own Claude app. That's where a server session on that account opens when you click Open in Claude.
- **Group each project's sessions into its own profile on a host.** A profile on a server is a Claude Code
  config folder with its own sign-in, sessions and memory: `brain`, `foawa`, `home-assistant`. Projects can
  use the same account or different ones.
- **When an account runs out of usage, switch that project's account.** Its sessions stop, it signs in as
  the other account, and the same sessions come back, in the same conversations. Nothing moves, and your
  other projects keep running.

Switching is also the safer way. Signing a Claude Code install out and in again is something Claude Code
supports. Moving a session's files from one config folder to another isn't: it relies on how Claude Code
lays out its files today, which can change with any update. Moving sessions between profiles is still
there, for splitting a project up, but switching the account is the way to carry on when an account hits
its limit.

## What you get

### 🔑 Easy login
Add a profile to a server and sign it in **in your Mac's own browser**, with Claude's usual OAuth page.
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
    <td width="50%"><img alt="A session's tmux window inside the app" src="docs/screenshots/remote-session-window.png"><br><sub>The session's tmux window, live in the app: see and answer whatever is holding up a session that didn't start cleanly, without opening ssh.</sub></td>
  </tr>
</table>

### 📱 Launch right into Remote Control
One click opens a running session in the Claude app signed in to the same account on your Mac. It's
already in the Claude app on your phone. Need the terminal after all? **Open in Terminal** attaches over ssh.

Each desktop profile also lists the Remote Control sessions running on its account, on every host, so you
can see what Claude (Marcus2) can open and jump straight in. It follows the account: switch a project's
account, and its sessions move to the other desktop profile's list.

<p align="center">
  <img alt="A desktop profile with the Remote Control sessions running on its account" src="docs/screenshots/desktop-remote-control.png" width="720">
</p>

### 🔄 Switch accounts, not sessions
Hit a usage limit? **Switch account…** on the project: its running sessions stop, it signs out, and you
sign it in as another account in your browser. The same sessions then resume under the new account, in
the same conversations, with Remote Control on. Nothing moves, and no other project is touched: it's
Claude Code's own sign-out and sign-in, so it doesn't depend on its internals. If the
browser signs you in to the same account again, the app notices and says so.

<p align="center">
  <img alt="Switch FOAWA to another account: its two sessions stop and resume under the new account" src="docs/screenshots/switch-account.png" width="720">
</p>

### 🔀 Move sessions between profiles
To split a project up, move a session to another profile on the same server. Everything goes with it: the transcript, subagents, file history, todos and plans. Claude Code
doesn't officially support moving sessions, so this works with its files as they're laid out today, with
backups and checks along the way.

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
your NAS. Each shows up in the sidebar under its name, with its projects as profiles you can color and put
on ⌘1…⌘9.

### 🤖 Let Claude run it, over MCP
The app is also an MCP server, so Claude Desktop and Claude Code can do all of this for you: *"restart
everything on xjopa1 that's waiting on an update"*, *"what's the brain session stuck on?"*, *"switch
foawa to my other account"*. It lists profiles, sessions and usage; starts, stops, restarts and renames
sessions; reads and answers a session's window; switches a project's account; and plans and makes moves,
merging memory the way you tell it. Claude starts
the server itself, so the app doesn't need to be open.

Set it up under **Settings → MCP server**: **Add to my Claude profiles** adds it to every Claude profile on
this Mac, desktop app and Claude Code both. Or add it by hand:

```sh
claude mcp add --scope user remote-control-conductor -- '/Applications/Remote Control Conductor.app/Contents/MacOS/remote-control-conductor' mcp
```

### 🧑‍🤝‍🧑 One desktop profile per account
It's [ai-profiles](https://github.com/bartekczyz/ai-profiles) underneath, so each of your Claude accounts
gets its own Claude app on your Mac, with its own launcher, color and usage meters. Each one lists the
Remote Control sessions running on its account on your servers, ready to open.

## Requirements

- **Your Mac:** macOS, Apple silicon or Intel (one universal build).
- **Each server:** Linux on x86_64 or arm64, with **tmux 3.0+** and **Claude Code**. A systemd user service
  keeps it running. Anything that restarts a process works too, such as a container's start script.
- **The network:** **the Mac and the servers must be on the same Tailnet, WireGuard VPN or LAN.** The server
  isn't meant for the open internet: it answers only addresses you allow (Tailscale, WireGuard and the
  machine itself by default, a LAN if you add it), and drops everyone else before TLS.

## Get started in three steps

**1. Install the app.** Download the `.dmg` from
[Releases](https://github.com/marcusadolfsson/remote-control-conductor/releases/latest). It's signed and notarized.
Drag `Remote Control Conductor.app` to Applications.

**2. Install the server** on each Linux machine:

```sh
sudo apt install tmux
curl -fsSL https://claude.ai/install.sh | bash
mkdir -p ~/.local/bin && curl -fsSL https://github.com/marcusadolfsson/remote-control-conductor/releases/latest/download/remote-control-conductor-server-$(uname -m)-linux -o ~/.local/bin/remote-control-conductor-server && chmod +x ~/.local/bin/remote-control-conductor-server
remote-control-conductor-server setup
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

Later, `remote-control-conductor-server doctor` checks an install, `pair` makes a new code, and `revoke` cuts a Mac off.
The server is a single static binary. You can also build it from source with
`cargo install --locked --git https://github.com/marcusadolfsson/remote-control-conductor remote-control-conductor-server`.

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

What Remote Control Conductor adds:

1. **One place for Claude Code on your servers,** every server, project and account in one sidebar.
2. **Switching a project's account** when one runs out, sessions and all, and moving sessions between
   profiles, memory and all.
3. **Remote Control, not another client.** You work in the official Claude apps. This app starts, stops,
   restarts, renames and moves the sessions behind them.
4. **Built for a Linux host:** a small server with pinned TLS that brings sessions back after a reboot,
   and accounts that sign in through your Mac's browser.
5. **Claude can run it.** The app is an MCP server too, so Claude Desktop and Claude Code can list,
   start, restart, read, archive, move and switch your sessions on every server, when you ask.

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
its [README](https://github.com/bartekczyz/ai-profiles#readme) covers that part. Like ai-profiles, it's MIT-licensed, and
keeps its copyright notice (see [LICENSE](LICENSE)).

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
git clone https://github.com/marcusadolfsson/remote-control-conductor.git
cd remote-control-conductor
pnpm install
pnpm --filter ai-profiles tauri build
```

The `.app` lands in `apps/ai-profiles/src-tauri/target/release/bundle/macos/`, and the `.dmg` beside it in
`dmg/`. A local build isn't notarized, so macOS warns on first launch: right-click the app → Open → Open.

To build the server for Linux from a Mac, run
`cargo zigbuild --release -p remote-control-conductor-server --target x86_64-unknown-linux-musl` in
`apps/ai-profiles/src-tauri`.

**Tests:** run `cargo test` and `cargo clippy --all-targets -- -D warnings` in `apps/ai-profiles/src-tauri`,
and `pnpm test` in `apps/ai-profiles`.
- Under Node 25 and later, vitest needs `NODE_OPTIONS=--no-experimental-webstorage`.
- One test, *"shows the authoritative Codex reset count 2"*, fails on upstream `main` as well.

**Branches:**
- `main` is upstream's, plus #50, #53, remote hosts, and the fork's own changes (its name and icon,
  releases, the updater switched off).
- Each upstream pull request has its own branch.
