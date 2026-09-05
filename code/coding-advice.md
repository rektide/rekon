---
type: Guide
title: "Coding posture"
description: The standing posture rules for writing code: Let It Fail when it makes sense, and no automatic pruning or reconciliation of undeclared artifacts unless explicitly requested.
resource: /code/coding-advice.md
tags: [code, error-handling, let-it-fail, cleanup, posture]
status: draft
generated: { by: model:zai-coding-plan/glm-5.3-max, at: 2026-09-04 }
stale_after: 2027-03-04
sources:
  - id: agents-operating-manual
    resource: /AGENTS.md
    title: "The coding-advice rules of the workspace operating manual (human-codified)"
    author: human:rektide
    last_modified: 2026-09-04
---

<a id="coding-advice"></a>

# Coding posture

<a id="coding-advice-let-it-fail"></a>

## Let It Fail

When it makes sense, let the code fail. Do **not** create a try-catch just
to print an error: the stack trace already shows what is wrong, and a
swallowed-and-reprinted exception hides the path that produced it.

Good failure is loud, located, and unhandled-by-default. Handle an error
only when there is a genuine recovery or a boundary (process edge, user
surface) that must produce a clean message.

<a id="coding-advice-no-pruning"></a>

## No automatic pruning

Pruning stale or undeclared artifacts is usually outside scope. Do not add
automatic prune/reconciliation behavior unless explicitly requested.

Create and update **declared** state; leave cleanup deliberate. If stale
artifacts accumulate, surface them to the human and let the removal be its
own reviewed decision — not a side effect of an unrelated change.

<a id="coding-advice-cross-references"></a>

## Cross-references

- [`/code/README.md#code-coding-advice`](/code/README.md#code-coding-advice)
  — the common-weight summary.
- [`/code/commits.md`](/code/commits.md) — deliberate cleanup lands as its
  own commits, like everything else.
