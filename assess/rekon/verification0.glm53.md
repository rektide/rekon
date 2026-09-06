---
type: VerificationReport
title: Narrow verification of tooling0.glm53 load-bearing claims
description: Source-and-runtime bounds on the beads "complete substrate", ACP --server/persistence, native session-ID self-knowledge, and hyalo claims; assess-subtree link/frontmatter check results.
resource: /assess/rekon/verification0.glm53.md
tags: [rekon, verification, beads, dolt, history, acp, opencode, session-id, hyalo, links]
status: draft
generated: { by: "model:glm-5.3-flash", at: 2026-09-06 }
stale_after: 2026-12-06
sources:
  - { resource: /assess/rekon/tooling0.glm53.md, title: Report under verification (sha256 13ce8218), author: "model:glm-5.3-max (per frontmatter)" }
  - { resource: "~/archive/steveyegge/beads @ 772a656888 (v1.0.3-168)", title: Beads source checkout, author: "project:steveyegge/beads" }
  - { resource: "installed bd 1.0.3 (dev: 772a656888d3)", title: Runtime surface, author: "host:rektide" }
  - { resource: "~/src/opencode-acp (branch v2, @opencode-ai/cli 1.18.4)", title: OpenCode V2 source, author: "project:anomalyco/opencode" }
  - { resource: "~/src/opencode @ f109c00a64 (v1.1.49)", title: OpenCode V1 source, author: "project:anomalyco/opencode" }
  - { resource: "installed opencode2 v0.0.0-local-202609051106 (channel local)", title: Installed runtime, author: "host:rektide" }
  - { resource: "~/archive/ractive/hyalo @ b50f828 (v0.20.0-13)", title: hyalo source checkout, author: "project:ractive/hyalo" }
---

<a id="verification0"></a>

# Verification: tooling0.glm53 load-bearing claims

Scope: narrow, read-only verification of four claims in
[`tooling0.glm53.md`](/assess/rekon/tooling0.glm53.md) (pinned at sha256
`13ce8218…f92c`, plus a full `assess/**` snapshot — see
[`.test-agent/assess-verification/snapshot.txt`](/.test-agent/assess-verification/snapshot.txt),
15 files, taken 2026-09-06T23:30Z; the parent's concurrent authors were writing during this
check, so later additions are outside this basis). Verdicts: **confirmed**, **qualified**,
**corrected**. No independent architecture review, no mutations to source reports, product
code, or beads.

<a id="verification0-beads"></a>

## 1. "bd history + snapshot differencing is a complete substrate" — QUALIFIED

True for the **issue-record lifecycle**; overstated as "complete". `bd history <id>` reads
`dolt_history_issues` with an explicit issues-table column list
([`internal/storage/dolt/history.go`](file:///home/rektide/archive/steveyegge/beads/internal/storage/dolt/history.go),
`getIssueHistory`). Runtime confirms the JSON shape: 55 entries for
`rekon-agents-maintenance-assembler`, each `{CommitHash, CommitDate, Commiter, Issue}` where
`Issue` carries exactly the issues-table fields (title, description, notes,
acceptance_criteria, status, priority, assignee, close_reason, closed_at, …).

Bounds the synthesis should carry:

- **Comments, relationships, and labels are invisible to `bd history`.** They live in separate
  tables (schema migrations `0002`–`0005`: `dependencies`, `comments`, `labels`, `events`), and
  their write paths never touch the `issues` row — comment add
  ([`issueops/comments.go`](file:///home/rektide/archive/steveyegge/beads/internal/storage/issueops/comments.go)
  `AddCommentEventInTx` inserts an event only), dep add
  ([`issueops/dependencies.go`](file:///home/rektide/archive/steveyegge/beads/internal/storage/issueops/dependencies.go):178
  inserts into `dependencies` only), label add
  ([`issueops/labels.go`](file:///home/rektide/archive/steveyegge/beads/internal/storage/issueops/labels.go):135).
  A comment-only or dep-only change therefore produces **no** `bd history` entry and none of
  its content. "Creations, revisions, closures, reopens" are derivable; "rework" via comments
  is not.
- **Repeated snapshots dominate.** Of the 55 entries, **50 adjacent pairs are fully identical**
  on every record field — history is per-commit row-touch (e.g. `updated_at` bumps), not
  per-event. Differencing is mandatory, and even a correct diff under-counts
  comment/relationship work (point above).
- **Uncommitted state.** Global flag default is `--dolt-auto-commit off` ("uncommitted changes
  persist in the working set until `bd dolt commit`"; `batch` defers too). This repo sets
  `dolt.auto-commit=on` (`bd config get`), so rekon's history is current — but the substrate's
  completeness is **configuration-dependent**, not native.
- **Deleted entities.** `dolt_history_*` retains deleted rows (Dolt semantics), but there is no
  undelete surface in the CLI (`bd delete` "cleans up references"; only compaction has
  `bd restore`); recovering a deleted issue needs a raw Dolt history query and a known id.
  Derived from source + Dolt behavior, not runtime-verified (would require a destructive test).
- **Compaction is safe for history.** Compaction truncates description/notes via
  `UPDATE issues` ([`issueops/compaction.go`](file:///home/rektide/archive/steveyegge/beads/internal/storage/issueops/compaction.go):92)
  — it *appears* in history as a truncation snapshot, and pre-compaction content remains in
  earlier snapshots (`bd restore` reads it back).
- **A richer substrate exists below the command.** Dolt versions *every* table
  (`dolt_history_comments`/`_dependencies`/`_labels`/`_events` are queryable), and an `events`
  table exists (migration `0005`) written by comment events and bulk ops
  ([`bulk_ops.go`](file:///home/rektide/archive/steveyegge/beads/internal/storage/issueops/bulk_ops.go):273).
  The generic `recordEvent` helper is **dead code** (defined
  [`dolt/issues.go`](file:///home/rektide/archive/steveyegge/beads/internal/storage/dolt/issues.go):470,
  zero call sites), and there is no `bd events` command in installed 1.0.3 (runtime:
  `unknown command "events"`). So "complete substrate" is defensible at the raw-Dolt level;
  via `bd history --json` alone it is not.

Suggested synthesis wording: *complete substrate for the issue-record lifecycle now; comments,
relationships, and labels require raw Dolt history-table queries (or future upstream surfaces);
snapshot differencing mandatory; completeness of uncommitted work depends on
`dolt.auto-commit`.*

<a id="verification0-acp"></a>

## 2. ACP "always private server ⇒ never shares sessions/config" — CORRECTED (data plane)

Confirmed exactly as reported: the `acp` handler unconditionally calls
`Standalone.start()` ([`handlers/acp.ts`](file:///home/rektide/src/opencode-acp/packages/cli/src/commands/handlers/acp.ts):14);
installed `opencode2 acp --help` offers no `--server`/`--standalone` (only generic globals);
standalone spawns `<self> serve --stdio --port 0` with a random lease password
([`services/standalone.ts`](file:///home/rektide/src/opencode-acp/packages/cli/src/services/standalone.ts)).
The **attachment gap is real**.

But "never share the elected service's **sessions, config**" conflates process isolation with
data isolation:

- **Same database file.** DB path resolution is mode-independent — the identical block serves
  `default`, `service`, and `stdio` modes
  ([`server-process.ts`](file:///home/rektide/src/opencode-acp/packages/cli/src/server-process.ts):84-92):
  `$OPENCODE_DB`, else `opencode.db` (channels latest/beta/next/prod or channel-DB disabled),
  else `opencode-<channel>.db`; relative names resolve against the XDG data root
  `~/.local/share/opencode` ([`util/global.ts`](file:///home/rektide/src/opencode-acp/packages/util/src/global.ts),
  `app = "opencode"`; [`database/database.ts`](file:///home/rektide/src/opencode-acp/packages/core/src/database/database.ts):50-53).
  The installed build logs `channel=local`, so an ACP standalone and a same-build background
  service both use `~/.local/share/opencode/opencode-local.db`. Sessions an ACP job creates are
  persisted to the same store the elected service reads — visible afterwards (and concurrently,
  SQLite being a multi-process file store; WAL with a `disableWAL` opt-out in
  [`sqlite.node.ts`](file:///home/rektide/src/opencode-acp/packages/core/src/database/sqlite.node.ts)).
  Not established here (out of narrow scope): whether two live processes contend on that file
  or the service caches; treat "visible after the ACP process exits" as the safe claim.
- **Same config surface.** Config resolution is likewise mode-independent
  (`server-process.ts:98-104`, same `OPENCODE_CONFIG_DIR`/file handling) — an ACP standalone
  reads the same global config as the service.
- What is genuinely **not** shared: the *live service endpoint* — leases, incumbent
  registration, the service's HTTP session surface and its password. The ACP client talks to
  its own ephemeral server process; it just reads/writes the same durable store.

Suggested synthesis wording: *the `--server` gap is an attachment gap (which server process the
ACP client speaks to), not a data-plane isolation; delegated ACP sessions persist into the same
global store and config, but cannot ride the elected service's live session surface.*

<a id="verification0-session-id"></a>

## 3. "An agent inside a session cannot natively state its own session id" — OVERSTATED (wrong for the installed V2)

V2 **natively injects the session id into the agent's context**: the `core/environment`
builtin instruction renders `Current conversation session ID: ${sessionID}` into every
session's `<env>` block
([`instructions/builtins.ts`](file:///home/rektide/src/opencode-acp/packages/cli/../core/src/instructions/builtins.ts):31)
— entrypoint-independent, so an ACP-delegated session gets it too. Runtime corroboration is as
direct as it gets: this verification session itself runs on the installed
`opencode2 v0.0.0-local-202609051106` and its environment block states its native `ses_…` id.
V1 genuinely lacks it (V1's env block in
[`session/system.ts`](file:///home/rektide/src/opencode/packages/opencode/src/session/system.ts):33-38
has directory/git/platform/date only), so the claim is V1-accurate, V2-wrong.

What survives of the original three-way split:

1. **Protocol id return** — native, both generations (spot-verified
   [`acp/service.ts`](file:///home/rektide/src/opencode-acp/packages/cli/src/acp/service.ts):151
   `sessionId: state.id`). Parent-side receipts need nothing.
2. **Ambient self-knowledge** — native in V2 via the env instruction (above); plugin not
   required for the *model* to state its id.
3. **Child-process env** — still plugin territory: the builtin reaches model context, not
   `$OPENCODE_SESSION_ID` in spawned subprocesses; `opencode-session-id-plugin` remains the
   mechanism there, with its documented coverage gaps.

<a id="verification0-hyalo-links"></a>

## 4. hyalo claims and link hygiene

- **hyalo: CONFIRMED** at `b50f828` / `v0.20.0-13-gb50f828`: profiles including `okf` with
  deterministic `okf index`/`okf log` generators (managed markers, dry-run default, non-zero
  exit on drift — README ~L217) and `changelog` with `changelog add`/`release` (README ~L223);
  Claude Code integration (~L183); not installed (`hyalo not found`); no `llms.txt`. No
  corrections.
- **Host-absolute links: CONFIRMED, count 12.** All twelve `/home/…` markdown-link targets in
  the wave are in `tooling0.glm53.md` itself (list in
  [`check-results.txt`](/.test-agent/assess-verification/check-results.txt)); per repo link
  conventions they should be `file:///home/…` (as `beads-offerings0`-style external refs) —
  note as-written they resolve as bundle-relative and are dead in any Markdown consumer.
- **Assess-subtree check (2026-09-06T23:30Z, 15 files / 152 links):** 0 broken local links
  (every `file://` target resolves on this machine), 0 frontmatter issues (all files carry
  `type:`/`title:`; local `resource:` targets exist). Durability caveats, not defects:
  `file://` links bind to this machine; `review0.gpt6a.md` links into a gitignored
  `.test-agent/…` path (resolves today, ephemeral by design). Checker + pinned snapshot:
  [`.test-agent/assess-verification/`](/.test-agent/assess-verification/README.md).
- One labeling oddity for the parent to adjudicate: `tooling0.glm53.md` frontmatter says
  `generated.by: model:glm-5.3-max` while the wave suffix convention (`glm53`) covers the
  glm-5.3 family; this report uses `glm53` for **glm-5.3-flash**.

<a id="verification0-receipt"></a>

## Receipt

Read-only throughout: bd `version/--help/history/config get/dolt status` reads only; no bd
mutations, no `bd prime`, no installs, no servers, no live sessions beyond this one; no edits
to any wave report, AGENTS.md, product code, or issues. Evidence commands: source reads in
`~/archive/steveyegge/beads` @ `772a656888`, `~/src/opencode-acp` (v2), `~/src/opencode`
@ `f109c00a64`, `~/archive/ractive/hyalo` @ `b50f828`; runtime surfaces `bd 1.0.3-dev`,
`opencode2 acp --help`, one `bd history --json` over an existing rekon ticket; link/frontmatter
check via `node .test-agent/assess-verification/check-links.mjs`.

Native session id of this verification session: `ses_f86fbcd95ffelYIFLKbw5gXFrc`.
