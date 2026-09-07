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

1. [Capability checkpoint](/design/session-mementos/checkpoint0.gpt6a.md) — latest
   human refinements, bounded selector authorization, ticket split and contract
   disagreements still needing review.
2. [Initial brief](/design/session-mementos/brief0.gpt6a.md) — original request,
   ownership and review gates.
3. [Rekon marking proposal](/design/session-mementos/marking0.gpt6a.md) — exact and
   child targets, pending next-response intent, external tool feasibility and gaps.
4. [Cotail bookmark proposal](file:///home/rektide/src/cotail/.design/bookmarks/summary0.gpt6a.md)
   — verified source/channel limits and selectable `rektide_cotail_*` storage.
5. [OpenCode display evidence](file:///home/rektide/src/opencode-term-v2/.design/term-v2/multi-session-evidence0.gpt6a.md)
   — retained batch seam, shared tabs, loaded-history limitations and test plan.

## Executable progress

The [pure selector checkpoint](file:///home/rektide/src/opencode-term-v2/.design/term-v2/selection-checkpoint0.gpt6a.md)
implements ordered narrowing and terminating authority in OpenCode commit
`1b1ff39bb787`. The [parent and independent review](file:///home/rektide/src/opencode-term-v2/.design/term-v2/selection-review0.gpt6a.md)
found no actionable Standards or Spec issues and accepts the bounded kernel as
the next integration basis. The full TUI suite passed once: **1,291 passed,
4 skipped, 0 failed**; the package typecheck has an unchanged Core blocker.
**The visible epilogue is still unchanged**: inventory,
batch output, lookers, visit recording and bookmarks remain separate work.

## Related inquiry

[Ordered capability pipelines](/design/ordered-pipelines/README.md) preserves the
broader reusable-pattern ambition and the Watchwoman precedent. It is a memento
and possible future trial, not a framework dependency for this feature.

## Cross-session links and temporal context

The later [session-linking desire](file:///home/rektide/archive/doc/opencode/patches.md#session-links)
is tracked separately as `rekon-session-mementos-links`: make one canonical
session referable from other sessions without copying, moving or reparenting it.
Links can later supply an explicit epilogue inventory alongside tabs and visits;
Cotail's source-qualified targets are a possible shared foundation, not a chosen
link schema. Moving this coordinating session to Rekon remains undecided.

[Lastact](file:///home/rektide/src/opencode-lastact/README.md) already contains
activity clocks and transcript time-shift notices. It is relevant to modular
last-activity lookers, but actual prompt/part/settled evidence, metadata recency
and local visit time are different clocks. No Lastact integration or installation
has been requested unambiguously or performed here.
