# ai-profiles-remote

A fork of [bartekczyz/ai-profiles](https://github.com/bartekczyz/ai-profiles), the macOS app that runs several
Claude and ChatGPT accounts side by side, each with its own desktop launcher and CLI wrapper.

This README covers **only what this fork adds**. For what the app is, how to install a release and how the
upstream project is laid out, read the [upstream README](https://github.com/bartekczyz/ai-profiles#readme).

Every change below is a pull request upstream, meant to be reviewed on its own, except **remote hosts**,
which lives here first.

<p align="center">
  <img alt="A Claude account on a Linux host, in ai-profiles: running and previous sessions" src="docs/screenshots/remote-profile.png" width="820">
</p>

**One place to move a Claude Code session between accounts**, whether they're profiles on this Mac
([sessions](#50--sessions-list-move-archive-restore)) or accounts on a Linux machine where Claude runs in a
terminal ([remote hosts](#remote-hosts)).

## Download

From [Releases](https://github.com/marcusadolfsson/ai-profiles-remote/releases/latest):

- **Mac:** the `.dmg`, a universal build that's signed and notarized. Open it and drag `ai-profiles-remote.app` to
  Applications. It doesn't update itself (the updater would fetch upstream's build); take the next one from
  Releases.
- **Linux host:** the server, for x86_64 or arm64, statically linked. Then run its setup:

  ```sh
  mkdir -p ~/.local/bin && curl -fsSL https://github.com/marcusadolfsson/ai-profiles-remote/releases/latest/download/ai-profiles-server-$(uname -m)-linux -o ~/.local/bin/ai-profiles-server && chmod +x ~/.local/bin/ai-profiles-server
  ai-profiles-server setup
  ```

## How this differs

There are good tools near each half of this, but I didn't find one that does both, or that moves a session between accounts:

- **Multi-account switchers** ([clauth](https://github.com/uwuclxdy/clauth), [ClaudeCodeMultiAccounts](https://github.com/Leuconoe/ClaudeCodeMultiAccounts), and many small `CLAUDE_CONFIG_DIR` switchers) change which account a terminal uses. They don't know about sessions, and don't move one from account to account.
- **Session managers and remotes** ([happy](https://github.com/slopus/happy), [Claude Code UI](https://github.com/siteboon/claudecodeui), [claude-squad](https://github.com/smtg-ai/claude-squad), [ccmanager](https://github.com/kbwo/ccmanager), [hive](https://github.com/latagore/hive), [remy](https://github.com/padamchopra/remy)) run or mirror sessions, mostly for one account, and mostly through a UI or relay of their own.

What this adds:

1. **One place for accounts and their sessions,** on the Mac and on a server, side by side in the same sidebar.
2. **Moving a session between accounts,** with its transcript, subagents and file history, project memory merged (you decide what both sides changed), the desktop app's own session list, and archive and restore.
3. **Remote Control, not another client.** Sessions on the server start with Claude's own Remote Control on, so you work with them in the official Claude apps on your phone, another Mac or the web. ai-profiles starts, stops, restarts, renames and moves them, and shows their window when one needs an answer.
4. **Built for a Linux host:** a small server with pinned TLS, reachable over Tailscale or WireGuard, running as a systemd service that brings sessions back after a reboot.

## The changes

### [#47](https://github.com/bartekczyz/ai-profiles/pull/47) — Security hardening *(merged)*

Found while auditing the app before building it locally. None of it is remotely exploitable: each needs
control of a profile name, of `profiles.json`, or of the webview.

- A profile name was interpolated into the generated CLI wrapper's header comment, so a name with a newline
  in it became a live command that ran on every `claude-<slug>` call.
- A profile name was interpolated into the launcher's path under `/Applications`, so `../` could place (and
  later delete) a launcher outside it. Names are now validated on create, rename and import.
- Deleting a migration backup accepted any path whose file name looked like a backup. It now has to sit
  directly inside the app's data directory.
- The webview dropped the unused filesystem plugin and gained a strict Content Security Policy.

### [#48](https://github.com/bartekczyz/ai-profiles/pull/48) — Each Claude desktop profile gets its own config directory *(merged)*

The Claude desktop launcher passed only `--user-data-dir`, so the Claude Code agent inside the desktop app
(the Code tab) fell back to the stock `~/.claude`. Every Claude desktop profile, and the plain `claude` CLI,
therefore shared one session history and one set of hooks, permissions, plugins and MCP servers — while the
README promised those were per profile. A profile's desktop and CLI halves also disagreed: a session started
in the desktop app could not be resumed with `claude-<slug> --resume`.

Claude desktop launchers now export `CLAUDE_CONFIG_DIR`, the way Codex launchers already export `CODEX_HOME`.

**Existing launchers pick this up when they are next rebuilt. Desktop sessions created before that stay in
`~/.claude`** — which is what led to the session tooling in #50.

### [#49](https://github.com/bartekczyz/ai-profiles/pull/49) — The default profile can be renamed *(merged)*

The stock-install entry was always labelled "Default", which is ambiguous once several accounts are in the
list. It can now be named. The name is a label only, stored per app in `app-state.json`; the stock app, its
data directory and the plain CLI binary are untouched.

### [#50](https://github.com/bartekczyz/ai-profiles/pull/50) — Sessions: list, move, archive, restore

*Built on #48.*

Every Claude profile, including the stock install, gets a **Sessions** panel listing its Claude Code
sessions — name, a Desktop or CLI tag, folder, when it was last active, and whether something has it open.
A session can be **moved** to another Claude profile, **archived**, and **restored** afterwards.

Modelled on the transfer in [claudemulti](https://github.com/marcusadolfsson/claudemulti), a Linux CLI tool
for the same job, and extended to cover what the desktop app adds.

- **A move copies everything the session owns:** the transcript, its subagent and tool-result folder,
  `file-history`, `session-env`, `tasks`, `todos` and the plans it wrote.
- **Nothing is overwritten quietly.** Anything a move replaces goes to
  `<config>/session-transfer-backups/<id>/<time>/` first, every item is written under a temporary name and
  renamed into place, and the transcript is copied last, so an interrupted move leaves no half-session.
  A destination copy that is newer than the source is only replaced when you say so.
- **Project memory is merged, not replaced:** missing files are copied, the index gains the source's entries
  for files the destination has, and files both sides changed are kept as the destination has them and
  reported. (In `main`, with remote hosts, notes both sides changed are yours to decide instead: see below.)
- **The desktop app's own record is handled too.** The Code tab keeps one per session at
  `gui-data/claude-code-sessions/<account>/<org>/local_<uuid>.json`, pointing at the transcript by
  `cliSessionId`. A move writes a record for the destination's account and drops the fields tied to the
  source account (connectors, approved directories, prompt and tool snapshots), which the app fills in again.
- **Archiving and restoring** move the transcript, and any record, to and from
  `session-transfer-backups/<id>/<time>-archived/`, keeping their paths. Nothing is deleted.
- **Apps are quit when they have to be**, with your say-so: the dialog offers "Quit and move" / "Quit and
  archive", and ai-profiles quits only that profile's app (every profile's app shares one bundle id, so
  quitting by name would close them all).
- **Refused:** sessions a terminal has open, sessions whose folder is a desktop scratch workspace, and
  Codex profiles.

**A finding worth knowing, whether or not you use this:** if the desktop app can't find the transcript a
record points at, it does not drop the record. It quietly starts a new, empty CLI session under it. That is
how a session "disappears" after a launcher is rebuilt (see #48): the record stays, the history doesn't.

### [#51](https://github.com/bartekczyz/ai-profiles/pull/51) — A color and ⌘1 for the default profile

*Built on #49.*

The stock-install entry can be given a color as well as a name, from the same dialog, using the same picker
and the same `#rrggbb` validation as a managed profile. With one set, it shows a color dot in the sidebar and
command palette and a color tile in its header, instead of the hollow ring and brand mark.

⌘1…⌘9 now follow the order the sidebar shows its rows — each app's Default first, then that app's profiles —
so the Default entry is ⌘1. The badges and the key bindings read one helper, so they can't drift apart.

### [#52](https://github.com/bartekczyz/ai-profiles/pull/52) — A wrapper is named after its profile in the menu bar

A profile with its own Dock icon got its name in the Dock and in Cmd-Tab, but the menu bar still said
"Claude", because macOS takes that name from `CFBundleName` — which has to stay the vendor's, since Electron
finds its helper apps by it.

The wrapper now writes the profile's name into the `InfoPlist.strings` of every language the bundle ships,
which is where macOS reads the name from when a localization exists. The vendor's other strings in those
files are kept, `Info.plist` is untouched so Electron still finds its helpers, and the step runs before
signing so the strings are covered by the wrapper's signature.

Existing wrappers pick this up the next time they are rebuilt.

### [#53](https://github.com/bartekczyz/ai-profiles/pull/53) — The account a profile is signed in under

A Claude profile's header shows the account it's signed in as (email and plan, with the organization in the
tooltip), read from the files Claude Code keeps, or "Not signed in".

### Remote hosts

*Built on #50 and #53. Not yet proposed upstream: it's a big add-on, so it lives here first.*

Claude Code accounts on a Linux machine, one `CLAUDE_CONFIG_DIR` each under `~/.claude-accounts/`, show up in
the sidebar as profiles under the machine's name. A small server, `ai-profiles-server`, runs there. Sessions
run in tmux with Remote Control on, so they keep going with nothing attached and follow you to the Claude app
on your phone.

From the Mac:

- **Start, resume, stop and restart** sessions, and rename them. The name follows the session to Remote
  Control and tmux; a session with no name is named after its folder.
- **See and answer a session's window** inside ai-profiles when it's waiting on a question, or attach in
  Terminal over ssh.
- **Move a session to another account** on the host. Everything under the session's id goes and anything
  replaced is backed up. The copy left behind can be archived, deleted once the moved copy checks out
  identical, or kept.
- **Project memory is merged:**
  - A note only one side changed since the last move merges by itself.
  - A note both changed is yours to decide: keep the newer, take either side, or a merge Claude writes (safe
    mode, no tools) that you read first.
  - Moves between profiles on this Mac now work the same way.
- **Archive, restore and delete archives.** Archives are compressed, here and on this Mac: a 300 MB
  transcript keeps in about 100.
- **Add an account and sign it in** from the Mac. You can also sign it out, rename it, give it a color and
  ⌘-number, or delete it (it goes to `.trash`).
- **Open a Remote Control session** in the Claude app of the matching profile on this Mac.

<table>
  <tr>
    <td width="50%"><img alt="A session's tmux window inside ai-profiles" src="docs/screenshots/remote-session-window.png"><br><sub>A session's window, live, inside ai-profiles.</sub></td>
    <td width="50%"><img alt="New session on a remote host" src="docs/screenshots/remote-new-session.png"><br><sub>New session: a folder on the host, a name, trust.</sub></td>
  </tr>
  <tr>
    <td width="50%"><img alt="Moving a session to another account" src="docs/screenshots/remote-move-session.png"><br><sub>Moving a running session: exited first, then archived, deleted or kept.</sub></td>
    <td width="50%"><img alt="Archived sessions" src="docs/screenshots/remote-archived.png"><br><sub>Archived sessions, compressed, with restore and delete.</sub></td>
  </tr>
</table>

#### Setting up a host

The machine needs **tmux 3.0 or newer** and **Claude Code**.

```sh
sudo apt install tmux
curl -fsSL https://claude.ai/install.sh | bash
mkdir -p ~/.local/bin && curl -fsSL https://github.com/marcusadolfsson/ai-profiles-remote/releases/latest/download/ai-profiles-server-$(uname -m)-linux -o ~/.local/bin/ai-profiles-server && chmod +x ~/.local/bin/ai-profiles-server
ai-profiles-server setup
```

Or build the server from source (with a Rust toolchain):
`cargo install --locked --git https://github.com/marcusadolfsson/ai-profiles-remote ai-profiles-server`.

`setup` walks through it step by step:

1. It checks tmux and Claude Code, and finds your accounts.
2. It asks which networks may connect. Tailscale and WireGuard peers can by default; a LAN only if you say so.
3. It installs a systemd user service that survives a reboot.
4. It prints a pairing code. Paste that into ai-profiles under **Settings → Remote hosts → Pair a host**.

The same steps are in the app, under **How to set up a host**. Later, `ai-profiles-server doctor` checks the
install, and `ai-profiles-server pair` makes a new code.

<p align="center">
  <img alt="Settings, Remote hosts, with the setup guide" src="docs/screenshots/remote-setup-guide.png" width="720">
</p>

#### How it connects

- **Transport:** HTTPS (TLS 1.3) with the server's own certificate. The pairing code carries its
  fingerprint, and the Mac trusts that certificate and no other.
- **Pairing and tokens:** a code works once, for 10 minutes. The token it's exchanged for lives in the
  Keychain; the server keeps only its hash, and `ai-profiles-server revoke` cuts a Mac off.
- **Who can connect:** only addresses you allow, by default Tailscale, WireGuard and the machine itself.
  Everyone else is dropped before TLS.
- **Running commands:** `claude` and tmux start with argument lists, never through a shell. The server
  reads and types only into windows it opened, and starts new sessions only in the folders it's told it
  may use.

## Building this fork

```sh
git clone https://github.com/marcusadolfsson/ai-profiles-remote.git
cd ai-profiles-remote
pnpm install
pnpm --filter ai-profiles tauri build
```

The `.app` lands in `apps/ai-profiles/src-tauri/target/release/bundle/macos/`, the `.dmg` beside it in
`dmg/`. A local build is not notarized, so macOS warns on first launch: right-click the app → Open → Open.

The server for a Linux host builds on that machine with `cargo install` (see [Setting up a host](#setting-up-a-host)),
or from a Mac for Linux: `cargo zigbuild --release -p ai-profiles-server --target x86_64-unknown-linux-musl`
in `apps/ai-profiles/src-tauri`.

**Running the tests:** `cargo test` and `cargo clippy --all-targets -- -D warnings` in
`apps/ai-profiles/src-tauri`, and `pnpm test` in `apps/ai-profiles`.

Under Node 25 and later, vitest needs `NODE_OPTIONS=--no-experimental-webstorage`. Node's own global
`localStorage` otherwise shadows happy-dom's and 67 unrelated tests fail. One test,
*"shows the authoritative Codex reset count 2"*, fails on upstream `main` as well.

## Branches

- `main`: upstream's, plus #50, #53 and remote hosts (the `feat/remote-host` branch), and this README.
- One branch per pull request above.
- `local` — all of them merged together, which is what I run. It also carries one commit that is not for
  upstream: the auto-updater is switched off, because an update would replace a local build that carries
  these changes.
