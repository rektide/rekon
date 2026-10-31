---
id: atproto-cli-guidance
type: DesignProposal
title: "ATProto CLI auth, sessions, and whoami — proposal for atproto/cli.md and atproto/GLOBAL.md"
description: "Researched, verified guidance for atgc / goat / tg CLI authentication, session status, and whoami, with ready-to-apply file bodies and assembly integration steps."
resource: /design/atproto/cli0.glm53fx.md
status: draft
generated: { by: "model:zai-coding-plan/glm-5.3-flash#max", at: 2026-10-31 }
extensions:
  ticket: none recorded
  proposed_files: ["/atproto/cli.md", "/atproto/GLOBAL.md"]
  manifest_change: agents.manifest.json fragments += "atproto/GLOBAL.md" after "code/rust/GLOBAL.md"
sources:
  - id: this-session
    resource: "opencode://ses_e6bf5d354ffelsII2ETAWRk4md"
    title: "Commissioned research: ATProto CLI auth/session/status/whoami guidance; verified installed and archived clients; drafted cli.md and GLOBAL.md bodies"
  - id: atgc-checkout
    resource: "file:///home/rektide/archive/permadeath.com/atgc"
    title: "atgc source checkout (local install origin, ~v0.15.0-v0.16.0 boundary, Aug 2026 commits)"
  - id: atgc-help
    resource: "file:///home/rektide/archive/permadeath.com/atgc/src/cmd/auth.rs"
    title: "atgc auth verb implementation and module docs (login/status/switch/refresh/token/logout)"
  - id: atgc-docs-accounts
    resource: "file:///home/rektide/archive/permadeath.com/atgc/docs/accounts.md"
    title: "atgc — who atgc acts as (five-rank account resolution, worktree identity, agents)"
  - id: atgc-docs-env
    resource: "file:///home/rektide/archive/permadeath.com/atgc/docs/environment.md"
    title: "atgc — the eighteen ATGC_* environment variables"
  - id: atgc-docs-output
    resource: "file:///home/rektide/archive/permadeath.com/atgc/docs/output.md"
    title: "atgc — output shapes; auth status --json active→resolved rename; login piped-URL behavior"
  - id: atgc-plan-credentials
    resource: "file:///home/rektide/archive/permadeath.com/atgc/plan/credential-store.md"
    title: "atgc credential store: sessions.json contents, 0700/0600, atomic+locked, keyring rejected with measurements"
  - id: atgc-upstream
    resource: "https://tangled.org/permadeath.com/atgc"
    title: "atgc upstream README (v0.16.0 command surface: auth default, doctor local|remote, api, key create) and tags"
  - id: goat-checkout
    resource: "file:///home/rektide/archive/bluesky-social/goat"
    title: "goat local archive checkout (0.2.3-era, June 2026)"
  - id: goat-auth
    resource: "file:///home/rektide/archive/bluesky-social/goat/auth.go"
    title: "goat AuthSession persistence (~/.local/state/goat/auth-session.json, cleartext password)"
  - id: goat-account
    resource: "file:///home/rektide/archive/bluesky-social/goat/account.go"
    title: "goat account verbs: login/logout/activate/deactivate/status/update-handle/check-auth/missing-blobs/service-auth*"
  - id: goat-xrpc
    resource: "file:///home/rektide/archive/bluesky-social/goat/xrpc.go"
    title: "goat xrpc command: @pds auth mode, public/admin modes, HTTPie-style args"
  - id: goat-upstream
    resource: "https://github.com/bluesky-social/goat/releases/tag/v0.2.8"
    title: "goat v0.2.8 release (2026-10-27) — upstream currency"
  - id: goat-changelog
    resource: "https://raw.githubusercontent.com/bluesky-social/goat/main/CHANGELOG.md"
    title: "goat CHANGELOG (0.1.2 status→check-auth rename; 0.2.6 persisted refreshed tokens; 0.2.7 xrpc -o)"
  - id: tg-skill
    resource: "file:///home/rektide/archive/oaustegard/claude-tangled-spoke/skills/tg/SKILL.md"
    title: "tg skill: bootstrap, command surface, rkey discipline, self-hosted-PDS federation tax"
  - id: tg-auth-sh
    resource: "file:///home/rektide/archive/oaustegard/claude-tangled-spoke/skills/tg/auth.sh"
    title: "tg bootstrap: BSKY_*→ATPROTO_* mapping, login via stdin, idempotent"
  - id: tg-source
    resource: "file:///home/rektide/archive/oaustegard/claude-tangled-spoke/bin/tg"
    title: "tg implementation: createSession/refreshSession, ~/.config/tg/session.json 0600, with_refresh on 401"
  - id: atproto-oauth-skill
    resource: "/refs/skills.md"
    title: "atproto-oauth skill (DPoP/PAR/PKCE profile; 14-day public-client refresh cap; 3600s access tokens)"
  - id: lookup-skill
    resource: "/atproto/README.md"
    title: "Rekon ATProto lookup skill — the module body cli.md extends"
---

# ATProto CLI auth, sessions, and whoami — proposal

## Situation

The user-approved ask: propose actionable ATProto CLI
authentication / session / status / whoami guidance for a new
`/atproto/cli.md`, plus a super-condensed just-use fragment at
`/atproto/GLOBAL.md` for ambient assembly. Investigate locally installed and
archived clients — especially `atgc` and `tg` — with precise supported
commands and API currency, keeping credentials out of harm's way. This
document is the deliverable; per instruction I have not edited any existing
file and have not committed.

## Findings

### Tool inventory and currency (checked 2026-10-31)

| Tool | Installed here? | Local form | Upstream | Currency verdict |
| --- | --- | --- | --- | --- |
| `atgc` | yes, `~/.cargo/bin/atgc` **0.15.0** (cargo-installed from `~/archive/permadeath.com/atgc`) | checkout at `~/archive/permadeath.com/atgc` (Aug 2026 commits, sits between v0.15.0 and v0.16.0) | [tangled.org/permadeath.com/atgc](https://tangled.org/permadeath.com/atgc), latest tag **v0.16.0** (~2026-10-01), commits 3 days ago | actively developed; local binary one minor release behind |
| `goat` | **not installed** | archive checkout `~/archive/bluesky-social/goat`, 0.2.3-era (June 2026) | [github.com/bluesky-social/goat](https://github.com/bluesky-social/goat) **v0.2.8** (2026-10-27) | maintained; local copy stale but auth surface unchanged (app-password createSession) |
| `tg` | **not on PATH** | vendored script `~/archive/oaustegard/claude-tangled-spoke/bin/tg` (1112-line Python stdlib), with skill wrapper `skills/tg/` | [oaustegard/claude-tangled-spoke](https://github.com/oaustegard/claude-tangled-spoke), bin/tg last touched 2026-08-10 | niche headless tool; app-password only; subset of onevcat/tang |

The "tg" identification: `tg` is **oaustegard's minimal Tangled CLI** —
self-described as "subset of onevcat/tang's surface, written for headless use
in Claude Code on the Web". It is not the same family as `atgc` (which is the
full-featured Tangled client from permadeath.com); both talk to Tangled, but
`tg` authenticates with **app passwords** while `atgc` authenticates with
**ATProto OAuth (DPoP)** and refuses app passwords entirely.

### atgc — precise surface (installed 0.15.0, verified against binary `--help`)

Auth verbs (`atgc auth …`), all verified:

- `atgc auth login <HANDLE>` — OAuth via browser + loopback callback. Refuses
  non-interactive sessions (`ATGC_NO_INPUT`, `CI`, piped stdin) rather than
  hanging. Piped stdout: prints the bare authorization URL alone on a line
  (prose moves to stderr). Additive: logging in does not touch other accounts.
- `atgc auth status` — lists logged-in accounts and the acting one.
  `--json` prints `{accounts: [{did, handle, active, session-state, scopes
  this build asks for that the login predates}], active: {…}|null}`.
- `atgc auth switch <HANDLE|DID>` — choose the **default** (fallback) account.
- `atgc auth refresh` — refresh the acting account's access token.
- `atgc auth token` — **prints the acting account's access token** (DPoP-bound,
  ~60 min life). A secret on stdout; treat accordingly.
- `atgc auth logout [HANDLE|DID]`, `--all` for every account.

Supporting verbs: `atgc doctor` (seven checks: acting account, session
usability, scope gaps, push-key path, checkout identity, origin resolution,
knot/appview health; `--json`), `atgc key list|add|delete` (SSH push keys as
`sh.tangled.publicKey` records — **login does not enable push**),
`atgc logs oauth|pds|git [-f] [--json]` (`auth log` was renamed to
`logs oauth`), `atgc agent` (working notes for agents — the binary's own
briefing; re-read after upgrades).

Who-atgc-acts-as precedence (docs/accounts.md, five ranks, first wins):
1. `--account` flag (one command)
2. `ATGC_ACCOUNT` env
3. the checkout's git config `user.email` — a checkout naming an account you
   are not logged in as **stops**; no silent fallback
4. the default account (`auth switch` in 0.15.0; `auth default` in 0.16)
5. the only account, when exactly one is logged in

Credential store: `$XDG_CONFIG_HOME/atgc` else `~/.config/atgc` — account
registry + jacquard `sessions.json` + logs; directory 0700, files 0600,
atomic writes under one advisory lock (fails closed). Per account: access
token (~60 min), rotating refresh token, P-256 DPoP private JWK. Public
loopback client ⇒ ~2-week session (matches the OAuth profile's 14-day public
cap); upstream targets 180-day confidential-client sessions, marked
*unverified*. OS keyring was researched and rejected with measurements
(recorded in plan/credential-store.md) — the file, hardened, is the design.

A refused refresh deletes the session: the next command reports "no OAuth
session", and `auth status` still names the account. Re-login fixes it.

### atgc 0.15.0 → v0.16.0 deltas (observed upstream + checkout)

- `auth switch` → **`auth default`** (pointer renamed to "default account");
  `auth status --json` field `active` → **`resolved`** (breaking, no alias).
- `doctor` splits into **`doctor local` / `doctor remote`**.
- New **`atgc api <NSID>`** — raw XRPC with `--host pds|bobbin|appview|knot:…`;
  the credential is derived from the host, never user-chosen: OAuth token +
  DPoP for own-PDS calls, PDS-minted service-auth JWT (with `lxm`) for knot
  procedures, nothing for public reads. `--dry-run` prints the full request.
- **`atgc key create`** — mint + publish a push key for an identity with no
  `~/.ssh` (agent accounts, e.g. via did.bot); worktree-local identity via
  `atgc repo configure` (writes the worktree's own git config).
- New command families: `issue`, `release`, `run`, `string`; `auth login
  --port` names the callback port; Claude Code plugin under `plugin/`.

The guidance should teach 0.15.0 verbs (what is installed) and flag the 0.16
renames so a post-upgrade reader does not stumble.

### goat — precise surface (verified against checkout; upstream 0.2.8)

- `goat account login` — flags `-u/--username` and `-p/--password`
  (aliases `GOAT_USERNAME`/`GOAT_PASSWORD`, `ATP_USERNAME`/`ATP_PASSWORD`,
  plus `ATP_AUTH_FACTOR_TOKEN` for 2FA and `ATP_PDS_HOST`). App-password
  session against the PDS (`com.atproto.server.createSession`).
- Session persistence: `~/.local/state/goat/auth-session.json` (xdg state),
  mode 0600, storing **DID, cleartext password, access token, refresh token,
  PDS**. The README warns about this in caps ("cleartext"); `goat account
  logout` wipes the file. 0.2.6 fixed "resumed login sessions have new tokens
  persisted to disk" — the local 0.2.3-era copy predates it.
- **`goat account check-auth`** — the whoami-equivalent: prints `DID:`,
  `Host:`, and the `com.atproto.server.checkAccountStatus` JSON. (Named
  `status` before 0.1.2; that name now means the public hosting-status lookup:
  `goat account status <identifier>` works for *any* account unauthenticated.)
- Raw whoami: `goat xrpc query @pds com.atproto.server.getSession`.
- Also: `goat identity resolve <handle>` (public DID doc), `goat account
  service-auth --aud <did> [--lxm NSID]` (mint short-lived service tokens),
  `goat xrpc` generic caller with HTTPie-style args (`key==value` query,
  `key=value` JSON body, `-` stdin body), `--admin-password` for PDS admin.

### tg — precise surface (verified against bin/tg)

- `bash skills/tg/auth.sh` bootstraps: symlinks `bin/tg` onto PATH, maps
  `BSKY_HANDLE`/`BSKY_APP_PASSWORD` → `ATPROTO_*` (CCotw/claude.ai names),
  runs `tg auth login` only when `tg auth status` fails.
- `tg auth login [--handle H]` — password from `ATPROTO_APP_PASSWORD` env,
  else stdin (the skill passes it as `tg auth login --handle H <<< "$PW"`),
  else getpass; resolves the PDS from the DID document; calls
  `com.atproto.server.createSession`.
- `tg auth status` — **local file read only**: prints Handle/DID/PDS; exit 1
  "Not authenticated." when absent. `tg auth refresh` calls
  `com.atproto.server.refreshSession` and rotates both tokens. `tg auth
  token` prints the access JWT (secret). `tg auth logout` deletes the file.
- Store: `~/.config/tg/session.json` (override `TG_CONFIG_DIR`), chmod 0600.
  Auto-refresh on 401 wraps every authenticated call (`with_refresh`).
- Reads of PR/issue listings go through Constellation; identifiers are
  **rkeys**, never the AppView `#N` numbers. No merge/close/checkout
  (knot service-auth not wired); self-hosted-PDS owners may be unreachable
  from proxied environments — the skill documents the fallback (web UI).

### whoami landscape

No atproto XRPC is literally named whoami; the session-who-am-I methods are
`com.atproto.server.getSession` and `com.atproto.server.checkAccountStatus`
(authenticated), with public identity via
`com.atproto.identity.resolveHandle` + `plc.directory` (already covered in
[/atproto/README.md](/atproto/README.md)). None of the three CLIs ships a
`whoami` verb — atgc's old `whoami` was deliberately deleted upstream ("The
deleted README described `atgc whoami`, which no longer exists") in favor of
`auth status`; goat uses `account check-auth`; tg uses `auth status`.

### Credential hygiene (cross-tool)

- Never pass passwords as argv (`-p`/positional) — argv is world-readable via
  `ps`. Use env vars or stdin. tg's skill models the right shape
  (`<<< "$PW"`); goat exposes env Sources precisely so `-p` can be avoided.
- `atgc auth token` / `tg auth token` put bearer-grade material on stdout;
  never paste into tickets, logs, commits, or chat. DPoP-bound tokens are
  unusable without the stored key, but treat them as secrets anyway.
- App passwords: create dedicated, narrow ones at
  <https://bsky.app/settings/app-passwords>; revoke when finished. The
  protocol direction is OAuth (atgc is already OAuth-only) — prefer OAuth
  clients where available.
- Access tokens live ~60 minutes everywhere; public-client sessions cap
  around 14 days. Auto-refresh exists in all three (atgc restores in place;
  tg retries on 401; goat 0.2.6+ persists refreshed tokens); a *refused*
  refresh means re-login, not retries.
- Storage is 0600-file plaintext on disk in all three (~/.config/atgc,
  ~/.config/tg/session.json, ~/.local/state/goat/auth-session.json). atgc's
  store is the hardened one (atomic, locked, 0700 dir); goat's holds the
  cleartext *password* — most sensitive; don't back these files up casually.

## Proposed `/atproto/cli.md` body (ready to apply)

````markdown
---
id: atproto-cli
type: Guide
title: "ATProto CLIs — login, sessions, status, whoami"
description: "Just-use recipes for atgc, goat and tg: which tool for which job, how to log in, how to ask who you are, and how to keep credentials out of argv, logs and tickets."
resource: /atproto/cli.md
status: draft
generated: { by: "model:zai-coding-plan/glm-5.3-flash#max", at: 2026-10-31 }
sources:
  - { resource: "opencode://ses_e6bf5d354ffelsII2ETAWRk4md", title: "Verified installed atgc 0.15.0 verbs and archived goat/tg sources; drafted this guide" }
  - { resource: "file:///home/rektide/archive/permadeath.com/atgc/docs/accounts.md", title: "atgc — who atgc acts as" }
  - { resource: "file:///home/rektide/archive/permadeath.com/atgc/plan/credential-store.md", title: "atgc credential store design and measurements" }
  - { resource: "https://tangled.org/permadeath.com/atgc", title: "atgc upstream README (v0.16.0 surface)" }
  - { resource: "file:///home/rektide/archive/oaustegard/claude-tangled-spoke/bin/tg", title: "tg implementation (auth verbs, session store)" }
  - { resource: "https://github.com/bluesky-social/goat/releases/tag/v0.2.8", title: "goat v0.2.8 (upstream currency)" }
---

# ATProto CLIs — login, sessions, status, whoami

No-credential lookup recipes (resolve a handle, find likes, check a record)
live in [the lookup guide](/atproto/README.md). OAuth protocol internals are
the `atproto-oauth` skill's business; this page is what you type.

## Which tool

| | `atgc` | `goat` | `tg` |
| --- | --- | --- | --- |
| job | Tangled forge: repos, PRs, issues, stacks, push keys | general atproto CLI: identity, firehose, lexicons, xrpc, PDS admin | headless Tangled writes (patch PRs, issues) from locked-down environments |
| auth | ATProto OAuth (DPoP), no app passwords | app password | app password via env/stdin |
| get it | `cargo install --git https://tangled.org/permadeath.com/atgc` | brew / deb / `go install github.com/bluesky-social/goat@latest` | vendored: `~/archive/oaustegard/claude-tangled-spoke/bin/tg` |
| currency (2026-10) | local 0.15.0; upstream v0.16.0 | upstream v0.2.8 (2026-10-27) | last touched 2026-08 |

Reads need no auth anywhere: public XRPC (`curl 'https://public.api.bsky.app/xrpc/…'`)
or the unauthenticated goat/tg verbs.

## Log in

```sh
# atgc — OAuth, browser + loopback callback. Additive; other accounts stay.
atgc auth login <handle>
# non-interactive (ATGC_NO_INPUT=1, CI, piped stdin): refuses, or with stdout
# piped prints the bare authorization URL alone on a line.

# goat — app password. Env, never -p (argv is visible in ps).
GOAT_USERNAME=<handle> GOAT_PASSWORD=<app-password> goat account login

# tg — app password through stdin (skill bootstrap does all of this):
ATPROTO_HANDLE=<handle> ATPROTO_APP_PASSWORD=<app-password> \
  tg auth login --handle <handle> <<< "$ATPROTO_APP_PASSWORD"
```

App passwords come from <https://bsky.app/settings/app-passwords> — make a
dedicated narrow one, revoke it when done, never the account password.

## Who am I / is my session alive

```sh
atgc auth status            # accounts, the acting one, and WHICH rule chose it
atgc auth status --json     # scripts: {accounts:[…], active:{…}}  (v0.16 renames active→resolved)
atgc doctor                 # the full check: session usable, scope gaps, push key, checkout identity

goat account check-auth     # DID + PDS host + checkAccountStatus JSON  (pre-0.1.2 name: status)
goat xrpc query @pds com.atproto.server.getSession   # raw session facts
goat account status <identifier>                     # public hosting status, no auth

tg auth status              # local session file: Handle/DID/PDS; exit 1 = not logged in
tg auth refresh             # rotate both tokens (auto-happens on 401 too)
```

Access tokens live ~60 min everywhere; public-client sessions cap ≈14 days.
All three refresh transparently (atgc restores in place; tg retries on 401;
goat ≥0.2.6 persists refreshed tokens — the local 0.2.3-era copy does not).
A *refused* refresh deletes the atgc session: re-run `atgc auth login`; the
account registry survives.

## Where credentials live (all 0600 plaintext — don't casually back up)

| tool | store | contents |
| --- | --- | --- |
| atgc | `~/.config/atgc` (0700 dir, locked+atomic writes) | access token (~60 min), rotating refresh, P-256 DPoP JWK per account |
| goat | `~/.local/state/goat/auth-session.json` | tokens **and the cleartext app password** (README warns) |
| tg | `~/.config/tg/session.json` (`TG_CONFIG_DIR` overrides) | tokens per session |

Secrets on stdout — `atgc auth token`, `tg auth token`, goat's session file —
never get pasted into tickets, logs, commits, or chat.

## Multiple accounts and agents (atgc's model)

Acting-as precedence, first that answers wins: `--account` flag →
`ATGC_ACCOUNT` env → checkout git config `user.email` → default account →
the only account. A checkout naming an account you are not logged in as
**stops** instead of publishing as someone else. Git worktrees get their own
identity via `atgc repo configure` run inside the worktree. Agent accounts
(did.bot identities) log in like any account; v0.16's `atgc key create`
mints their push keys without a `~/.ssh`. `atgc agent` prints the tool's own
briefing — re-read it after upgrades.

## Push keys are separate from login (Tangled)

`atgc auth login` never enables `git push`: knots authenticate pushes by SSH
key published as a `sh.tangled.publicKey` record.

```sh
atgc key add                      # publish a key (key create in v0.16 for agent identities)
ssh -T git@<handle>.tangled       # "Hi @you!" = registered key worked
```

## Version notes

- atgc local 0.15.0 → upstream v0.16.0 renames: `auth switch` → `auth default`;
  `auth status --json` field `active` → `resolved`; `doctor` → `doctor local|remote`;
  new `atgc api <NSID>` (raw XRPC; the `--host` decides the credential class:
  OAuth+DPoP for your PDS, PDS-minted service JWT for knot procedures, none for
  public reads). `atgc logs oauth` (not `auth log`) holds the OAuth event log;
  `--incident` shows the client_id/overlap comparison.
- goat: `account status` → `account check-auth` rename happened at 0.1.2;
  prefer upstream v0.2.8 over the stale archive copy.
- tg: rkeys identify issues/PRs (AppView `#N` numbers don't exist at the
  record layer); reads resolve through owners' PDSes, so self-hosted-PDS
  repos can be unreachable from proxied environments — use the web UI
  fallback (documented in the tg skill).
````

## Proposed `/atproto/GLOBAL.md` body (ready to apply)

````markdown
---
id: atproto
order: 280
source: /atproto/cli.md
status: draft
type: ContextFragment
title: ATProto CLIs — auth and whoami
description: Which CLI for which job (atgc/goat/tg), login and whoami one-liners, and credential hygiene.
resource: /atproto/GLOBAL.md
generated: { by: "model:zai-coding-plan/glm-5.3-flash#max", at: 2026-10-31 }
sources:
  - { resource: "opencode://ses_e6bf5d354ffelsII2ETAWRk4md", title: "Researched installed/archived ATProto CLIs and condensed the guidance" }
  - { resource: /atproto/cli.md, title: "ATProto CLIs — login, sessions, status, whoami" }
---

# ATProto CLIs — auth and whoami

- Tool choice: `atgc` for Tangled forge work (OAuth, no app passwords);
  `goat` for general atproto plumbing (identity, firehose, lexicons, xrpc,
  PDS admin); `tg` only for headless Tangled patch-writes (app passwords).
  Unauthenticated reads just use `curl` on public XRPC.
- Who am I: `atgc auth status` (names the rule that chose the account;
  `doctor` checks the session end to end) · `goat account check-auth` ·
  `tg auth status`. Raw session: `goat xrpc query @pds com.atproto.server.getSession`.
- Logins: atgc opens a browser OAuth loopback and refuses CI; passwords go
  through env or stdin, never argv. App passwords: dedicated + revoked after;
  OAuth is the direction.
- Credentials: `atgc auth token` / `tg auth token` print live tokens — stdout
  is a secret channel, nothing gets pasted into tickets or logs. Stores are
  0600 plaintext under `~/.config/{atgc,tg}` and `~/.local/state/goat/`.
- Sessions expire on ~60-min access tokens and ≈14-day refresh caps; refresh
  is automatic until the server refuses — then re-login, don't retry.
- Tangled pushes need `atgc key add` per account, independent of login.
- Depth: [/atproto/cli.md](/atproto/cli.md) · lookups:
  [/atproto/README.md](/atproto/README.md) · OAuth internals: the
  `atproto-oauth` skill.
````

## Assembly integration

1. Apply `/atproto/cli.md` first, then `/atproto/GLOBAL.md` (the fragment's
   `sources` points at it; the okf checker resolves local source paths).
2. Edit `agents.manifest.json`: insert `"atproto/GLOBAL.md"` between
   `"code/rust/GLOBAL.md"` and `"praxis/beads/GLOBAL.md"` — domain tooling
   knowledge sits with the other code/tooling fragments, before praxis.
   The fragment's `order: 280` slots between rust (260) and beads (390).
3. One-line companion edit (proposed, not applied): in `/atproto/README.md`,
   after the intro paragraph, add a pointer line such as:
   `Auth and CLI session recipes (atgc/goat/tg): [cli.md](cli.md).`
   — a real link when applied; quoted as inline code here so this proposal's
   own link check does not chase the not-yet-existing target.
4. Regenerate and verify:
   ```sh
   R="cargo run --quiet --manifest-path ~/src/rekon/tool/rust/Cargo.toml"
   $R combine            # regenerate AGENTS.md from the manifest
   $R combine --check    # assembly fresh
   $R check --okf atproto/cli.md atproto/GLOBAL.md
   $R check atproto/README.md design/atproto/cli0.glm53fx.md
   ```
5. Optional later steps, deliberately out of scope here: add the atgc Claude
   plugin or tg skill to the refs [skills table](/refs/GLOBAL.md) if
   they become routinely used; the codes glossary needs no entry (atgc/goat/
   tg are tool names, not scoped concerns).

## Risks and open questions

- **Verb drift**: the installed atgc is 0.15.0; v0.16 renames `switch`→
  `default` and `active`→`resolved`. The cli.md body teaches 0.15.0 and flags
  the renames; if the machine upgrades first, invert the emphasis.
- **goat copy staleness**: the archive checkout predates the token-persistence
  fix (0.2.6); install upstream 0.2.8 rather than `go install` from the stale
  tree. Not installed here at all today.
- **tg viability**: untouched since August and app-password-only; guidance
  keeps it scoped to headless patch-writes and says so.
- No personal handles, DIDs, or tokens appear in the proposed bodies; this
  machine's atgc is not logged in (verified read-only), so no live session
  state leaked into the research.

## Verification performed

- Ran `atgc --help`, `atgc auth --help`, and each auth verb's `--help` against
  the installed 0.15.0 binary; confirmed no `api` verb (unrecognized subcommand)
  and `atgc auth status` reports "not logged in" here.
- Read atgc sources/docs: `src/cmd/auth.rs`, `src/config/dir.rs`,
  `src/cmd/api.rs`, `docs/accounts.md`, `docs/environment.md`,
  `docs/output.md`, `plan/credential-store.md`, checkout git log (Aug 2026),
  and the upstream Tangled page (tags v0.15.0/v0.16.0, README command table).
- Read goat sources: `auth.go`, `account.go`, `xrpc.go`, README, local
  CHANGELOG; upstream CHANGELOG and v0.2.8 release metadata via web.
- Read tg: `bin/tg` (auth verbs, session class, storage, with_refresh),
  `skills/tg/SKILL.md`, `skills/tg/auth.sh`, repo git log.
- Consulted the `atproto-oauth` skill for the session-lifetime and DPoP
  facts (3600s access tokens; 14-day public-client refresh cap; OAuth 2.1
  profile) that back the "sessions expire" guidance.
