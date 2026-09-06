---
type: Guide
title: "Handling review comments"
description: The protocol for responding to code review: check in before editing, and land each correction as its own well-argued cohesive commit.
resource: /code/review-handling.md
tags: [code, review, commits, feedback]
status: draft
generated: { by: model:zai-coding-plan/glm-5.3-max, at: 2026-09-04 }
stale_after: 2027-03-04
sources:
  - id: agents-operating-manual
    resource: /AGENTS.md
    title: "The review-comment rules of the workspace operating manual (human-codified)"
    author: human:rektide
    last_modified: 2026-09-04
  - id: checkpoint-direction
    resource: urn:opencode:session:ses_f872bf58affeS5IBlC38cA6iYX
    title: Distinguish discussing review direction from committing the pre-revision state
    author: human:rektide
    last_modified: 2026-09-06
---

<a id="review-handling"></a>

# Handling review comments

<a id="review-handling-protocol"></a>

## The protocol

1. **Capture the present stage before revising it.** Commit current owned work,
   or identify its existing commit, before applying feedback or a revamp. Draft
   prompts and documents deserve the same before/after history as code. A
   checkpoint does not imply acceptance. [Checkpoint discipline](/code/commits.md#commits-checkpoints).
2. **Discuss the direction before making edits.** Do not silently rework.
   Some comments are wrong, some are questions in disguise, and some conflict
   with each other—the human decides what lands. Conversation and committing
   the predecessor are separate obligations; neither substitutes for the other.
3. **Break corrections up into individual, well-argued, cohesive commits.**
   One logical correction per commit. The commit message argues the change:
   what the comment was, what was done about it, and why this is the right
   fix. Explain what was preserved, what changed, and the destination the
   revision heads toward.

<a id="review-handling-why"></a>

## Why one commit per correction

- The reviewer can verify each response independently, instead of
  untangling a bulk "address review" commit.
- A rejected correction is easy to drop or redo in isolation.
- The history shows the shape of the review exchange: which concerns
  produced which changes.

<a id="review-handling-commits"></a>

## Commit shape

- Use the normal commit discipline: [`commits.md`](/code/commits.md) —
  file-scoped `jj commit`, honest messages, tickets in the body.
- Intermingled hunks from different corrections in one file:
  [`jj-hunk`](/code/jj-hunk.md) separates them.
- If a comment requires discussion rather than code, respond in the
  thread and leave the tree alone.

This protocol applies to reviews of code, prompts, research, and guidance.
Keep its machinery proportional while preserving the states that make an
evolving argument understandable.

<a id="review-handling-cross-references"></a>

## Cross-references

- [`/code/README.md#code-review-handling`](/code/README.md#code-review-handling)
  — the common-weight summary.
- [`/code/commits.md`](/code/commits.md) — the commit mechanics
  corrections land with.
