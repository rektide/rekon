---
type: DesignOverview
title: Ordered capability pipelines
description: Preserve a reusable composition inquiry without collapsing distinct domain semantics into one engine.
resource: /design/ordered-pipelines/README.md
tags: [composition, rules, pipelines, authority]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
extensions:
  ticket: rekon-session-mementos-composition
sources:
  - { resource: /design/ordered-pipelines/memento0.gpt6a.md, title: Architecture memento }
---

# Ordered capability pipelines

The recurring ambition: assemble named contributions, run them in order, obtain
facts only when needed, and make each contribution's authority and outcome
inspectable. The recurring danger: first-match policy, narrowing streams,
fallback lookup, accumulating output and effectful work do not have the same
composition laws.

Start with the [architecture memento](/design/ordered-pipelines/memento0.gpt6a.md).
It grounds the idea in Watchwoman's implemented classifier and the proposed
session epilogue, distinguishes termination requests from grants, and proposes a
small cross-domain explanation/trace trial before any shared runtime extraction.

**Work anchor: `rekon-session-mementos-composition`.** No universal framework,
new package, deployed skill or common evaluator has been accepted or implemented.

## Immediate application

[Session mementos](/design/session-mementos/README.md) applies ordered filters and
lookers locally. Its implementation can progress without resolving this wider
inquiry. The memento keeps the possibility and its counterexamples reachable.
