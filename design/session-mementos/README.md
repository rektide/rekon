---
type: DesignOverview
title: Session mementos
description: Recover work through multi-session terminal epilogues, explicit message marks and summary retrieval.
resource: /design/session-mementos/README.md
tags: [sessions, epilogue, bookmarks, tools]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-10-06
extensions:
  ticket: rekon-session-mementos
sources:
  - { resource: /design/session-mementos/brief0.gpt6a.md, title: Initial intent }
  - { resource: /design/session-mementos/checkpoint0.gpt6a.md, title: Capability and termination checkpoint }
  - { resource: /design/session-mementos/marking0.gpt6a.md, title: Marking-tool feasibility }
---

# Session mementos

Leave enough context to resume: which sessions mattered, when they were active,
what was asked and answered, and which response was explicitly marked as a
summary. The first display is OpenCode's **terminal exit epilogue**.

**Work anchor: `rekon-session-mementos`** (`bd show rekon-session-mementos`).
Tickets hold evolving intent, dependencies and acceptance; these documents hold
the arguments. The feature and marking contract are not accepted merely because
the drafts exist.

## Current direction

- Current session plus this client's visible tabs, not recent sessions everywhere.
- A distinct **process-local visit record** is wanted and separately ticketed.
- Newest-active-first candidates pass through ordered filtering stages. A
  filter's stopping request is honored only with explicit terminating authority.
- Ordered session **lookers** supply modular fields, including last activity,
  question/response excerpts and eventually bookmarked summary text.
- Cotail owns source-qualified bookmarks and explicit selectable storage; Rekon
  owns marking semantics and the authoring practice. No bookmark runtime or
  marking guidance has been installed.
- `rekon-session-mementos-tools` separately tracks how deployable tools belong in
  Rekon. The subagent precedent already separates generic host-read patches from
  the standalone `opencode-subagent-control` plugin; a Rekon home need not mean
  copying that code or rebuilding the tool into OpenCode core.

## Read in this order

1. [Retained multi-session implementation](file:///home/rektide/src/opencode-term-v2/.design/term-v2/multi-session-implementation0.gpt56sx.md)
   — current display/reporting code, scoped registries and verification. The
   [capability checkpoint](/design/session-mementos/checkpoint0.gpt6a.md) preserves
   the earlier ticket split and unresolved bookmark-contract discussion.
2. [Initial brief](/design/session-mementos/brief0.gpt6a.md) — original request,
   ownership and review gates.
3. [Rekon marking proposal](/design/session-mementos/marking0.gpt6a.md) — exact and
   child targets, pending next-response intent, external tool feasibility and gaps.
4. [Cotail key/value links and tools](file:///home/rektide/src/cotail/.design/bookmarks/links-tools0.gpt6a.md)
   — proposed reference-valued metadata and exact-write work; the direct read
   tool/RPC was subsequently implemented and reviewed in Cotail.
5. [Cotail bookmark proposal](file:///home/rektide/src/cotail/.design/bookmarks/summary0.gpt6a.md)
   — verified source/channel limits and selectable `rektide_cotail_*` storage.
6. [OpenCode display evidence](file:///home/rektide/src/opencode-term-v2/.design/term-v2/multi-session-evidence0.gpt6a.md)
   — retained batch seam, shared tabs, loaded-history limitations and test plan.

## Executable progress

The [retained multi-session implementation](file:///home/rektide/src/opencode-term-v2/.design/term-v2/multi-session-implementation0.gpt56sx.md)
now connects the earlier selector kernel to current/visible-tab inventory, separate
global/per-session reporter registries, retained publication, live rule edits and
atomic exit output. Sol implemented it and applied parent/GX review corrections;
the final full TUI suite reported **1,302 passed, 4 skipped, 0 failed**. Real
exit/signal, narrow/wide and live-expiry checks passed. The TUI typecheck still
has the unchanged Core FFI blocker; no live deployment or push is claimed.

The user subsequently authorized the deferred exit-time collection direction. Its
[bounded callback implementation](file:///home/rektide/src/opencode-term-v2/.design/term-v2/callbacks-implementation0.gpt6a.md)
dispatches one preselected-Session batch, waits up to four shared seconds through
`waitUntil` and an `AbortSignal`, and reuses retained handles for output. The full
TUI suite reported **1,306 passed, 4 skipped, 0 failed** after review corrections.
Visit records, Q/response excerpts, Cotail summaries and bookmark integration
remain separate work; the Cotail durable foundation is still at its schema
checkpoint.

## Related inquiry

[Ordered capability pipelines](/design/ordered-pipelines/README.md) preserves the
broader reusable-pattern ambition and the Watchwoman precedent. It is a memento
and possible future trial, not a framework dependency for this feature.

## Cross-session links and temporal context

The later [session-linking desire](file:///home/rektide/archive/doc/opencode/patches.md#session-links)
is tracked separately as `rekon-session-mementos-links`: make one canonical
session referable from other sessions without copying, moving or reparenting it.
The requested ownership direction is now **Cotail bookmarks with key/value
metadata**, including exact reference values for links. Links can later supply
an explicit epilogue inventory alongside tabs and visits and be browsed in
[Opensesser](file:///home/rektide/src/opensesser/design/bookmark-links/consumer0.gpt6a.md).
The wire/schema choices remain under design. Moving this coordinating session
to Rekon remains undecided.

[Lastact](file:///home/rektide/src/opencode-lastact/README.md) already contains
activity clocks and transcript time-shift notices. It is relevant to modular
last-activity lookers, but actual prompt/part/settled evidence, metadata recency
and local visit time are different clocks. No Lastact integration or installation
has been requested unambiguously or performed here.

## Tool prerequisite and namespace application

**Cotail's agent-callable surface is now the near-term prerequisite**: existing
bounded query tools first, then exact bookmark/key-value link tools and shared
browser reads. Do not wait for automatic next-response binding or a universal
namespace implementation. Rekon owns the authoring practice and alignment, not a
duplicate bookmark tool implementation.

[Native references across sessions, tickets and knowledge](/design/session-mementos/references0.gpt6a.md)
connects this real application to the existing `rekon-con-naming` inquiry and
documents Beads' native external-capability/hydration mechanisms. It distinguishes
foreign-ticket references from operational cross-project dependency satisfaction;
no such bridge has been configured or tested here yet.
