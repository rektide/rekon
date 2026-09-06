---
type: Review
title: Architectural and close review — opencode-lastact
description: Independent whole-system assessment of opencode-lastact at the fixed Lastact wave baseline.
resource: /assess/case-studies/lastact/review0.gpt56l.md
tags: [review, architecture, correctness, lastact, independent]
status: draft
generated: { by: "model:openai/gpt-5.6-luna#xhigh", at: 2026-09-06 }
verified: { by: none, at: never }
sources:
  - { resource: /assess/review/form0.gpt6a.md, title: Architectural and close review — form 0 }
  - { resource: /assess/case-studies/lastact/wave0.gpt6a.md, title: Lastact review wave 0 }
  - { resource: file:///home/rektide/src/opencode-lastact/README.md, title: Target intent and usage }
---

# Independent review

### 1. Assessment in brief

At baseline `43515611e7dd243d3815e2530147d5dfd45283f5`, Lastact has a good
shape for a small host-coupled plugin: pure time/config/rendering is separated
from a process-local ledger, while the server and TUI adapters keep the
OpenCode-specific seams visible. The monotone ledger, additive claims, quiet
unknown-state behavior, and per-session prompt serialization are all worth
preserving. The user’s testimony that the plugin works is consistent with this
architecture and with the passing unit suite.

The close review finds two consequential semantic boundaries. First, the TUI
calls every `session.inbox.enqueued` a user prompt even though the host event
admits synthetic, compaction, and move items; it also uses the host’s broad
`time.updated` clock as a prompt seed. That can make the primary stall display
look fresh for non-user activity. Second, the server commits a time-shift
marker and prompt baseline inside a pre-admission hook; a later attachment or
skill failure can leave a durable marker for a prompt that was never admitted.
These are source-derived findings, not live-session reproductions.

Overall judgment: architecturally fit and maintainable, but not yet fully
truthful at the two host admission/clock boundaries above. Confidence is high
for the findings because both follow the target and host source directly; the
absence of live integration means runtime frequency and presentation impact
remain unmeasured.

### 2. Basis and boundaries

- **Repository/scope:** `/home/rektide/src/opencode-lastact`, whole system, not
  a diff review. The checkout was clean at the requested immutable baseline
  `43515611e7dd243d3815e2530147d5dfd45283f5`.
- **Intent:** the target [`README.md`](file:///home/rektide/src/opencode-lastact/README.md#L1-L17)
  and the settled-clock/activity-ledger material under
  [`.design/settled-clock/`](file:///home/rektide/src/opencode-lastact/.design/settled-clock/),
  plus the stated user testimony that the plugin works as desired. The design
  documents are draft intent, not independently verified requirements.
- **Host basis:** the target pins `@opencode-ai/plugin` and
  `@opencode-ai/schema` to `0.0.0-dev-18695` in
  [`package.json`](file:///home/rektide/src/opencode-lastact/package.json#L51-L57),
  and the installed node_modules packages report that exact version. I treated
  those installed declarations as the baseline contract ([installed plugin
  metadata](file:///home/rektide/src/opencode-lastact/node_modules/@opencode-ai/plugin/package.json#L1-L6),
  [installed TUI contract](file:///home/rektide/src/opencode-lastact/node_modules/@opencode-ai/plugin/dist/tui/context.d.ts#L144-L159)). The archive checkout
  was also inspected at commit
  [`4306c07b340b9a0504e65785f366d2793cd1b169`](file:///home/rektide/archive/anomalyco/opencode),
  and the uncommitted `/home/rektide/src/opencode-working` source snapshot was
  used where the target design cites it. The latter is explicitly not a
  commit-addressable source; its package generation is 1.18.15, while the
  installed target contract is the pinned dev package. Current archive source
  has an additional `session.panel` slot, so it must not silently replace the
  pinned contract for this assessment.
- **Identity:** native session `ses_f8725fb71ffePziOw12QMOEVib`; requested
  model identity `openai/gpt-5.6-luna#xhigh`. No peer report, parent review, or
  synthesis was read.
- **Checks:** `pnpm test` passed 6 files / 86 tests; `pnpm run check:type`
  passed with the installed dependencies; baseline diff/status remained clean.
  Direct Node import of the TSX entry is not supported by Node’s type-stripping
  loader (`ERR_UNKNOWN_FILE_EXTENSION`), but Vitest transformed and exercised
  the module imports.
- **Exclusions:** no install, source edit, issue mutation, push, live prompt,
  or live-session experiment. The report therefore distinguishes source
  reasoning from runtime observation.

### 3. Architectural account

The system is two companions sharing portable domain code:

```text
server prompt hook ──> durable session synthetic + plugin storage
                              │
pure config/time/render ──────┴──── TUI event route ──> per-session ledger
                                                        │
                         host session store + status ───┴──> fixed ticker/slot line
```

The server companion resolves options, registers a prompt hook, and maintains
`session-prompt/<id>` in host storage. It compares the last settled turn with
the last observed prompt, admits a `resume: false` synthetic marker before a
large-gap prompt, and writes the new prompt baseline only after synthetic
admission ([`src/server/plugin.ts`](file:///home/rektide/src/opencode-lastact/src/server/plugin.ts#L47-L100)).
Its keyed semaphore and scope interruption behavior are a sensible ownership
boundary for overlapping prompts ([same file](file:///home/rektide/src/opencode-lastact/src/server/plugin.ts#L102-L145)).

The TUI companion resolves direct or advertised options, subscribes to terminal
events plus configured activity families, and touches only one cell in an
`ActivityTracker` row per event ([`src/tui/plugin.tsx`](file:///home/rektide/src/opencode-lastact/src/tui/plugin.tsx#L112-L150),
[`src/activity/tracker.ts`](file:///home/rektide/src/opencode-lastact/src/activity/tracker.ts#L51-L89)).
The status component is the bucket: on a fixed cadence it reconciles live
ledger timestamps with durable `time.updated`/`time.idle`, derives running
status and outcome, and renders a pure line ([`src/tui/status.tsx`](file:///home/rektide/src/opencode-lastact/src/tui/status.tsx#L56-L97)).
Claims are additive and deduplicated across regular and child surface lists;
`parentID` gates the composer claim so the same path does not paint twice
([`src/config/config.ts`](file:///home/rektide/src/opencode-lastact/src/config/config.ts#L136-L149),
[`src/tui/plugin.tsx`](file:///home/rektide/src/opencode-lastact/src/tui/plugin.tsx#L138-L150)).

This division fits the problem. Pure rendering is easy to test, the event path
does not do timing arithmetic, the host store gives restart continuity, and
cleanup owns subscriptions and claims. The main coupling is unavoidable but
well-located: event vocabulary and timestamps, session durable-clock meaning,
TUI slot inputs, `status()`, and pre-admission hook ordering. Future changes
should land at those seams rather than in the formatting or tracker core.

### 4. Close findings

#### F-01 — TUI prompt clock accepts non-prompt activity

- **Type:** defect
- **Impact/priority:** medium-to-high truthfulness impact on the primary line;
  fix/verify next
- **Confidence:** high, source-derived; not live-reproduced
- **Evidence/requirement:** The stated model calls `session.inbox.enqueued` a
  user prompt and says `updated` is optional ([`README.md`](file:///home/rektide/src/opencode-lastact/README.md#L26-L31),
  [`README.md`](file:///home/rektide/src/opencode-lastact/README.md#L179-L191)).
  The route table maps the entire event type to the prompt clock and extracts
  only `sessionID` ([`events.ts`](file:///home/rektide/src/opencode-lastact/src/tui/events.ts#L57-L80),
  [`events.ts`](file:///home/rektide/src/opencode-lastact/src/tui/events.ts#L134-L143)).
  The pinned/current host schema makes `Item` a tagged union of `user`,
  `synthetic`, `compaction`, and `move`
  ([host schema at commit 4306c07](file:///home/rektide/archive/anomalyco/opencode/packages/schema/src/session-inbox.ts#L37-L50)).
  All are published through `InboxEnqueued`
  ([host inbox](file:///home/rektide/archive/anomalyco/opencode/packages/core/src/session/inbox.ts#L169-L181));
  `Session.synthetic` specifically uses that path
  ([host session](file:///home/rektide/archive/anomalyco/opencode/packages/core/src/session/session.ts#L282-L320)).
- **Trigger/consequence:** a built-in or third-party synthetic notice,
  compaction, or move touches the prompt clock; synthetic also belongs to the
  target’s response family. A quiet session can therefore display recent
  prompt/part activity without a user prompt or LLM part. Separately,
  `snapshotFor` always seeds the prompt clock from `time.updated`
  ([`status.tsx`](file:///home/rektide/src/opencode-lastact/src/tui/status.tsx#L66-L75)),
  while the host advances that same column for inbox admission and for agent,
  model, and rename metadata ([host projector](file:///home/rektide/archive/anomalyco/opencode/packages/core/src/session/projector.ts#L544-L570),
  [host projector](file:///home/rektide/archive/anomalyco/opencode/packages/core/src/session/projector.ts#L623-L637)).
  Thus `refreshOn.updated: false` cannot prevent durable metadata pollution.
- **Smallest response/verification:** either rename the clock’s contract to
  “last session activity,” or make strict prompt semantics explicit: filter
  `InboxEnqueued` on `item.type === "user"`, do not use broad `time.updated`
  as a claimed user-prompt seed, and obtain a durable user-prompt clock from a
  host-supported source. Add a host-backed fixture covering user, synthetic,
  compaction, move, rename, and model selection, asserting both live touches
  and restart reconciliation.

#### F-02 — Pre-admission hook finalizes durable gap state

- **Type:** defect
- **Impact/priority:** high for durable transcript correctness; fix/verify next
- **Confidence:** high, source-derived; not live-reproduced
- **Evidence/requirement:** The target admits the synthetic and then writes its
  prompt baseline inside the hook effect ([`server/plugin.ts`](file:///home/rektide/src/opencode-lastact/src/server/plugin.ts#L61-L100)).
  The host documents and implements the prompt hook before attachment/skill
  resolution and before inbox admission: `SessionPrompt.prepare` triggers the
  hook at lines 37–51, then resolves files/skills at lines 52–76
  ([host prompt](file:///home/rektide/archive/anomalyco/opencode/packages/core/src/session/prompt.ts#L37-L76));
  `Session.prompt` admits only after preparation succeeds
  ([host session](file:///home/rektide/archive/anomalyco/opencode/packages/core/src/session/session.ts#L154-L179)).
- **Trigger/consequence:** after a long gap, submit a prompt with an invalid
  attachment or missing skill. Lastact can persist a time-shift synthetic (and
  `session-prompt` baseline), then host preparation fails and no user inbox
  item is admitted. The transcript contains a marker for a nonexistent
  resumed prompt, and a later successful prompt uses the failed attempt as its
  baseline. This is not covered by the server tests, which invoke the hook
  directly and make synthetic admission the only downstream operation.
- **Smallest response/verification:** finalize the marker/baseline only from a
  post-admission signal, or stage state by message ID and commit it when the
  host emits successful `session.inbox.enqueued`; otherwise do not claim
  exactly-before-prompt durability. Add an integration test that lets the hook
  succeed, fails preparation afterward, then submits a later prompt and checks
  for neither a phantom marker nor a poisoned gap baseline.

#### F-03 — Process-local ledgers and lock tables have no lifecycle bound

- **Type:** architectural pressure
- **Impact/priority:** medium only for long-lived/high-session processes;
  watch and measure
- **Confidence:** high for unbounded ownership, medium for practical impact
- **Evidence/trigger:** `ActivityTracker` owns a `Map` with only insert/read
  operations and no session deletion ([`tracker.ts`](file:///home/rektide/src/opencode-lastact/src/activity/tracker.ts#L51-L89)); TUI cleanup removes
  claims/subscriptions but not rows ([`tui/plugin.tsx`](file:///home/rektide/src/opencode-lastact/src/tui/plugin.tsx#L138-L150)).
  The server similarly retains one semaphore record per touched session
  ([`server/plugin.ts`](file:///home/rektide/src/opencode-lastact/src/server/plugin.ts#L109-L119)).
  A long-lived process observing many sessions therefore retains historical
  rows even after deletion or no further use.
- **Smallest response/verification:** decide and document process lifetime as
  the owner, or handle a host session-deleted event and remove only ephemeral
  tracker/lock state after in-flight work settles. A soak test with many
  created/deleted sessions and heap sampling would establish whether this is
  operationally material; do not conflate this with deliberate persistent
  storage retention, which the README already calls out separately.

### 5. Evidence and coverage

The tests strongly establish pure duration formatting, config coercion, route
table behavior, monotonic tracker writes, server semaphore interruption/retry
behavior, cleanup, and option recovery. The typecheck establishes the target’s
hand-maintained host slice against the pinned installed declarations. These
are meaningful checks, not an end-to-end certificate.

The TUI tests do not mount `LastactStatusLine`, advance its ticker, or exercise
`snapshotFor` against a real reactive store. Setup tests record claim paths and
subscriptions but never invoke a claim render. The server tests use a context
fixture and do not run the host’s post-hook attachment/skill/admission pipeline.
There is no live validation of child-pane rendering, server companion discovery,
SSE event filtering, or restart during a running turn. These omissions are
exactly where the two defects above live. The version distinction also matters:
the installed dev-18695 declaration is the baseline contract, while the
current archive source has moved on (including a `session.panel` slot).

### 6. Direction, not a rewrite mandate

- **Preserve:** the domain-grouped pure modules; monotone per-session ledger;
  fixed-cadence bucket; quiet empty state; additive slot claims; explicit
  regular/child gating; server per-session serialization; and “write baseline
  after synthetic admission” behavior for the operation it currently controls.
- **Fix/verify next:** settle the clock vocabulary at the host boundary (F-01)
  and make transcript markers transactional with successful prompt admission
  (F-02). Add the smallest host-backed tests before broadening surfaces.
- **Watch:** tracker and semaphore lifecycle (F-03), and the manually copied
  TUI contract as hosts gain slots. Keep the installed package version and host
  source version explicit when upgrading.
- **Deliberately defer:** new global/family rollups, more child surfaces,
  quota-finer parity, or a host redesign. None is needed to resolve the
  correctness boundaries found here.

### 7. Reflection on this review form

The form usefully forced an end-to-end walk rather than accepting the passing
unit suite: it made me inspect event payload discriminants, durable projection
semantics, admission ordering, restart seeds, cleanup, and the distinction
between installed declarations and nearby host source. Its explicit
“source-derived versus reproduced” language prevented overstating the two
findings.

The costly part was reconciling several host generations and a manually copied
contract; a reader can easily cite the wrong current source for a pinned
package. The form also repeats evidence/coverage distinctions across sections,
although that repetition helps comparison. Three concrete improvements:

1. Add a compact required version table with “installed declaration,” “source
   checkout,” and “live runtime” columns.
2. Add an admission-boundary prompt asking whether each durable side effect is
   before, during, or after host commit, and require one failure-after-hook
   counterexample where applicable.
3. Require event findings to name the discriminant fields that were inspected,
   not only the event type; this would make the inbox-item issue harder to miss.
