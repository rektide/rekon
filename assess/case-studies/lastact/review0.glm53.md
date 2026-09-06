---
type: Review
title: Lastact whole-system architectural and close review — GLM independent report
description: Whole-system assessment of opencode-lastact at 43515611 per form0; findings, evidence, and protocol reflection.
resource: /assess/case-studies/lastact/review0.glm53.md
tags: [lastact, review, architecture, evidence, wave0]
status: draft
generated: { by: "model:glm-5.3-max (dispatch label glm-max-z; opencode harness; session ses_f8725fb74ffeV2l1GCO27OGAB6)", at: 2026-09-06 }
verified: { by: none, at: never }
stale_after: 2026-12-06
sources:
  - { resource: /assess/review/form0.gpt6a.md, title: Review form 0 }
  - { resource: /assess/case-studies/lastact/wave0.gpt6a.md, title: Wave 0 assignment }
  - { resource: "file:///home/rektide/src/opencode-lastact", title: Target at 43515611e7dd243d3815e2530147d5dfd45283f5 }
  - { resource: "file:///home/rektide/src/opencode-lastact/node_modules/@opencode-ai", title: Installed pinned host contract 0.0.0-dev-18695 }
  - { resource: "file:///home/rektide/archive/anomalyco/opencode", title: Host source (archive anomalyco, HEAD 4306c07b) }
---

# Lastact whole-system review — GLM (glm-5.3-max) independent report

## 1. Assessment in brief

This is a small plugin with unusually strong host-contract discipline. The
architecture — a pure core (config decode, time formatting, line rendering,
monotone activity ledger) behind two thin host seams (a server prompt-hook
that writes durable time-shift markers; a TUI slot-claiming view whose
fixed-cadence "bucket" re-derives everything from ledger + host store) — fits
the stated purpose well: a minimal temporal-cue plugin cut from openlastcode.
The event path is genuinely O(1) table touches, derivation is centralized in
the tick, and every load-bearing host assumption I could check against the
pinned installed contract (slot tree, event vocabulary, `PluginInfo.id`,
config-document ordering, storage namespacing, durable-clock semantics,
synthetic-admission ordering) checks out. I found **no substantiated
defects**. The consequential items are two coverage gaps (the TUI
reconciliation function `snapshotFor` has no tests; no live-host acceptance
exists in-tree — an open bead), one deliberate-but-worth-eyeballing
availability coupling (a failing prompt hook fails the user's prompt), and a
seed-vs-live divergence nuance around `time.updated`. Confidence in the
contract verification is high; confidence that runtime behavior matches in
every pane scenario rests on the repo's own prior live-host research plus
user testimony, which this review did not (and per the rules could not)
re-establish.

## 2. Basis and boundaries

- **Target:** `file:///home/rektide/src/opencode-lastact`, whole system, clean
  working copy. Baseline `43515611e7dd243d3815e2530147d5dfd45283f5` ("Close
  the time-shift records feature") = HEAD; `d0db0f7a79cacf38b619b790e56061c03d1d055b`
  confirmed as its empty parent, matching wave0.
- **Intent sources:** target `README.md` (read fully); `.design/settled-clock/`
  (`settled-clock0.glm53.md`, `activity-ledger0.unk.md` + addendum; subagent
  waves skimmed via the addendum and README); read-only beads (`.beads/issues.jsonl`:
  16 issues; `oplast-time-shift-records` still `in_progress`,
  `oplast-live-acceptance` open); user testimony that the plugin works as
  desired. No new feature requirement is under review.
- **Host basis, per seam:** pinned installed packages
  `@opencode-ai/plugin`/`schema`/`client` `0.0.0-dev-18695` under the target's
  `node_modules` (primary); archive host source
  `file:///home/rektide/archive/anomalyco/opencode` (HEAD `4306c07b`, a
  *different* generation) for behavioral semantics (hook execution, projector
  writes, synthetic admission, companion resolution); the repo's own
  2026-08-30/31 live-host research in `.design/` for TUI-layout facts
  (child-pane suppression) I could not re-derive from the non-matching archive.
  Three host generations in play; every claim below states which basis proves
  it.
- **Checks run (allowed):** `pnpm test` → **86/86 pass** (6 files);
  `pnpm run check:type` (tsgo --noEmit) → **clean**. No installs, product
  edits, live sessions, or issue mutations; no scratch diagnostics were
  needed, so no `.test-agent/` directory was created.
- **Identity:** model `glm-5.3-max` (Z.AI coding plan), dispatch label
  `glm-max-z`, running in the opencode harness, session
  `ses_f8725fb74ffeV2l1GCO27OGAB6`. Dispatch labels are not attested
  identities per wave0; this one is stated honestly.
- **Exclusions:** peer reports, the parent review, and any synthesis were not
  read (independence gate). Child-pane suppression and the "advertised
  companion gets `{}` options" host behavior are taken from the repo's design
  docs, not re-verified live.

## 3. Architectural account

**Purpose:** two temporal cues — a subdued per-session status line ("how long
since the LLM last did anything") and durable transcript markers when a
prompt resumes a session after a ≥4h gap. Non-goals are explicit: no
`home` surface, no host-UI replacement (additive `append` only), no new slot
paths (plugins cannot publish them), no pruning of the plugin's own storage.

**Domain boundaries** (domain-grouped, mirrors openlastcode at ~1/10 scale):

| Module | Role | State | Lifetime |
| --- | --- | --- | --- |
| `config/` | options decode, surface/view vocabulary | none (pure) | — |
| `time/` | duration formatting; gap threshold + marker text | none (pure) | — |
| `render/` | snapshot → one short line | none (pure) | — |
| `activity/` | per-session monotone ledger (`touch`/`settle`) | `Map<sessionID, row>` | TUI process |
| `server/` | prompt hook → durable `[time-shift …]` marker | per-session semaphores; plugin storage | server process |
| `tui/` | host seam: drift-typed context, event→clock route table, bucket view, slot claims | per-surface tickers | claim mounts |

**Control flow (TUI):** SSE events → flat route table
(`src/tui/events.ts:58-93`) → one monotone ledger cell
(`src/activity/tracker.ts:58-84`) → per-surface fixed-cadence ticker
(`src/tui/status.tsx:38-44`) re-derives a snapshot via fresher-of reconcile
against the host store (`snapshotFor`, `src/tui/status.tsx:61-97`) → pure
render (`src/render/render.ts:99-149`) → additive slot claim gated per session
kind (`src/tui/plugin.tsx:138-145`, gate check per tick at
`src/tui/status.tsx:120-131`). No timer is ever created or reset by activity;
missed events self-heal at the next tick from the store. This delivers the
"at rest unless there is something to say" property the design waves asked
for, and the monotone cells make replayed/out-of-order events harmless by
construction.

**Control flow (server):** prompt hook (serialized per session by the
plugin's own semaphore, `src/server/plugin.ts:115-144`) reads
`info.time.idle` + stored prompt clock, computes the gap, admits a synthetic
marker inheriting delivery with `resume: false`, and only then writes the
stored prompt clock (`src/server/plugin.ts:96-99`) so a failed marker is
retried by the next prompt. Verified against host source: the hook fires
inside `SessionPrompt.prepare` *before* inbox admission
(`packages/core/src/session/prompt.ts:39-45`), so the marker is durably
ordered before the prompt; `session.synthetic` admits via the inbox and never
triggers prompt hooks (no recursion; `packages/core/src/session/session.ts:282-320`);
host hook callbacks run sequentially per prompt but concurrent prompts'
hooks may overlap (`packages/core/src/plugin/hooks.ts:98-107`), which is
exactly why the plugin's semaphore exists (its own comment says hooks are
"not exactly-once").

**Seams and drift discipline:** the locally typed `SlotMap`
(`src/tui/context.ts:40-49`) is **identical** to the installed
`@opencode-ai/plugin@0.0.0-dev-18695` `dist/tui/context.d.ts` (all 8 paths,
same inputs) — the drift-catcher is currently in sync, and
`SLOT_PATHS` is compile-time checked against it in two places. All 22 default
(and 32 total) subscribed event types exist in the pinned client `V2Event`
union. The `./tui` package export is what the host resolves for companions
(`packages/core/src/plugin/host.ts:17-44`), and `tui: true` on the server
definition is what advertises it (matches the TUI's
`features.tui === true` filter). Companion-option recovery matches the host:
`PluginInfo.id` is the definition id (`rektide.opencode-lastact`; supervisor
maps it directly), and `config.get` returns documents "lowest to highest
priority" (`packages/core/src/config.ts:36`), validating the
last-match-wins loop in `companionOptions` (`src/tui/plugin.tsx:102-108`).
The plugin correctly avoids the deprecated, publisher-less `session.idle` /
`session.status` events (`packages/schema/src/session-status-event.ts:44-51`)
in favor of `data.session.status()` and execution terminals.

**Where change lands:** the beads show the intended futures — stall
markers/toasts, family roll-up surfaces, quota-finer parity, openlastcode
sync. The family table (`refreshOn`), the surface vocabulary + `surfaceGate`,
and the ledger all look like the right joints for those; none of them
require touching the pure core. That is a good sign the shape fits.

**Strengths with evidence:** monotonicity everywhere (tracker tests; settle
projection in host is strict-monotone too, `projector.ts:405-432`, and
shutdown-interrupt is deliberately not written to `time_idle` — the plugin's
snapshot comment models this correctly); failure-ordered storage write with
tests for overlap, permit-holder failure/interruption, prompt interruption,
and scope close (`tests/server.test.ts:204-353`); deterministic synthetic id
making admission idempotent (host `admit` returns the existing input on id
collision).

## 4. Close findings

No substantiated defects. Grouped by consequence:

**LA-F1 — architectural pressure: the prompt hook sits on the critical path
of every prompt.** Impact: high if it ever fires; confidence: high on the
coupling, medium on real-world likelihood. `hooks.trigger` awaits each
callback (`packages/core/src/plugin/hooks.ts:98-107`) and session hooks have
no failure channel, and the plugin `orDie`s its record path
(`src/server/plugin.ts:143`): a defect in `session.get`/`storage.get`/`synthetic`
fails the *user's* prompt. This is deliberate (test
`tests/server.test.ts:268-280` asserts the prompt rejects; repo's "Let It
Fail" stance; storage-after-synthetic ordering guarantees retry). Source-derived,
not a defect. Smallest response if hardening is ever wanted: catch storage
*read* failures and treat as "no stored clock" while still failing on
synthetic admission. Verification: fault-inject `storage.get` in a test.

**LA-F2 — coverage gap: `snapshotFor` and `LastactStatusLine` are untested.**
Impact: medium (this is the reconcile heart — fresher-of per clock,
store-vs-ledger settle/outcome selection, the strict `>` compare, the
per-tick child gate); confidence: high. No test file imports
`src/tui/status.tsx`; `pnpm test` confirms nothing exercises it (all green
while the component module is unreferenced). A regression in restart
agreement or the gate would pass CI. Smallest response: export
`snapshotFor` (or lift it next to the pure renderers) and unit-test over
ledger/store fixtures, per the repo's own pure-core convention. Verification:
mutate the reconcile order and watch the new tests fail.

**LA-F3 — coverage gap / open question: no live-host acceptance in-tree.**
Impact: medium; confidence: high that it's absent. The design docs' live
verification boxes are unchecked
(`.design/settled-clock/settled-clock0.glm53.md` §Verification plan;
`activity-ledger0` §Verification — only typecheck/vitest checked), and beads
`oplast-live-acceptance` is open while `oplast-time-shift-records` is still
`in_progress` at a commit whose message says the feature closed. User
testimony ("works well") partially covers it but is not per-behavior.
Everything mock-verified here is consistent with the pinned contracts; the
untested-by-anything layer is exactly the TUI-layout behavior (child-pane
suppression, `composer` mounting) that only lives on a real host. Smallest
response: execute the two live checks the design docs already specify
(long-turn liveness; restart agreement), read-only, and tick the boxes.

**LA-F4 — contract/documentation nuance: prompt-clock durable seed is
`time.updated`, which metadata writes also bump.** Impact: low (cosmetic
divergence); confidence: high on mechanism, medium on observability. The
host pins `time_updated` on renames/selections/reverts as well as prompt
admission (projector), so after a metadata-only change a *restarted* TUI
shows a fresher "prompt" age than a *live* one (default `refreshOn.updated:
false` ignores those events live). The README documents the seed but not
this nuance. Not a defect — the host publishes no separate durable
"last prompt" timestamp, and the server plugin's own stored clock is not
readable from the TUI. Smallest response: one README sentence; optionally
discuss defaulting `updated` on for consistency. Verification: rename a
stalled session, restart the TUI, compare lines.

**LA-F5 — open question: `companionOptions` resilience.** Impact: low;
confidence: low (hypothetical, host-dependent). The advertised-companion
path awaits two client calls during TUI setup
(`src/tui/plugin.tsx:91-102`); a transient rejection would fail plugin load
for the session (tests cover only the non-querying direct-options path,
`tests/tui.test.ts:405-410`). Smallest response: catch and degrade to
defaults. Verification: stub a rejecting `plugin.list` and assert setup
still returns claims.

**Watch (not findings):** process-lifetime unbounded state — TUI ledger map,
server `locks` map, and durable `session-prompt/*` entries unpruned on
session delete (README acknowledges; consistent with the no-auto-prune
house rule). Also: "ended"-family events (`tool.success`, `step.ended`, …)
are unsubscribed, so during a long quiet step the part clock ages — arguably
the honest "stall" reading, and the next `*.started` refreshes it. The
`oc-plugin: ["tui"]` package field appears unused by the pinned host
generation (no reference found in archive source) — harmless metadata, worth
knowing if hosts change.

## 5. Evidence and coverage

Established by checks + source against the *pinned* contract: slot-tree
sync; event-vocabulary completeness (32/32 types present in
`V2Event`); envelope `created` timestamps; `SessionInfo`
`parentID`/`time`/`outcome` shape; `PluginInfo.id` semantics; config
ordering; storage namespacing (`plugin:<hex-id>:`, archive
`plugin/host.ts:561-565`); `synthetic` signature incl. `resume: false`;
marker-before-prompt durable ordering; hook overlap justifying the
semaphore; `session.idle` deprecated/unpublished. Established by the 86
unit tests: pure core behavior, server concurrency/failure matrix, setup
claim/subscription/disposal, companion-option recovery. **Source-only /
testimony-backed:** child-pane suppression and composer mounting (repo
design docs' live-host research, 2026-08-30/31, against a host build I
cannot confirm matches `0.0.0-dev-18695`); "advertised companion receives
`{}`" ambiguity handling; end-to-end rendering. Passing mocks are not an
end-to-end certificate here, and LA-F2/LA-F3 name exactly the residual
uncertainty. Nothing I found contradicts the user's testimony.

## 6. Direction, not a rewrite mandate

- **Preserve:** the ledger/bucket split and monotone discipline; the
  drift-catcher full-`SlotMap` typing; failure-ordered server writes; the
  pure-core/`tests/` layout; the surface-gate design (it makes the upcoming
  family-rollup and quota-finer-parity work local).
- **Fix/verify next:** LA-F2 (export + test `snapshotFor`) — smallest,
  highest value; then LA-F3 (two live checks from the design docs), which
  also closes `oplast-live-acceptance` honestly.
- **Watch:** LA-F4 seed divergence (document now, decide default later);
  LA-F1 coupling if plugin scope ever grows; unbounded session-keyed state
  on long-lived servers.
- **Deliberately defer:** pruning `session-prompt/*` storage (house rule:
  no auto-prune); ended-event subscriptions; `oc-plugin` field disposition.
  Findings are not implementation authorization; the parent retains
  architectural responsibility.

## 7. Reflection on this review form

The form's insistence on separating published docs / installed types /
local upstream source / live observation was the single most valuable
constraint: with three host generations in play (pinned dist, archive HEAD,
`.design`-era live host), "check the exact installed contract" decided
several claims (slot sync, `PluginInfo.id`) that reading the archive alone
would have gotten subtly wrong in either direction. The lens set (state
ownership, restart/recovery, disposal) is where LA-F1/LA-F4/LA-F6 came from;
"no finding per lens" permission kept the report honest. Costs: the largest
effort went into *deciding which artifact is authoritative* for behavioral
semantics the installed types don't express (hook execution, projector
writes) — the form says distinguish bases but not how to rank them when the
archive generation ≠ pinned generation. Also, "existing tests allowed; no
live experiments" leaves an awkward middle ground: I could have Bun-imported
the TUI plugin in scratch to strengthen the seam claim, and declined as
too close to the line. Proposals (≤3): (1) add an explicit "host-basis
resolution" item to §2 — state, per load-bearing seam, which artifact
proves it and what to do when generations disagree; (2) name an allowed
"harness probe" category (import-only scratch diagnostics, no host process)
so seam checks aren't orphaned between tests and forbidden experiments;
(3) require §5 to carry a small three-column evidence table
(testimony / test / source) — the form implies it, and making it structural
made my own uncertainty auditable.
