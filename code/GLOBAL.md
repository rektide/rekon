---
id: code
order: 200
source: /code/README.md
status: draft
type: ContextFragment
title: Code — stage and change discipline
description: A glossary-first candidate exposure of existing commit, review, and planning discipline while the shared context is honed.
resource: /code/GLOBAL.md
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
extensions:
  ticket: rekon-phoenix
  related_work: [rekon-agents-maintenance, rekon-session-mementos]
  posture: honing; not promoted; not selected for production assembly by this change
sources:
  - { resource: /code/README.md, title: Coding practice }
  - { resource: /code/commits.md, title: Capture revise compare }
  - { resource: /CONTEXT.md, title: Shared working vocabulary }
---

# Code — stage and change discipline

**Honing candidate, not a promoted GLOBAL.** Repetition with the README/SKILL
body and the live manuscript is intentional while we examine the combined view.

## Terms in use

- **Stage / checkpoint:** a coherent state of work / its preserved reference.
  Drafts, prompts, and research have stages too; saving one is not acceptance.
- **Correction:** a response to feedback, with a reason and an intended delta.
- **Scope:** the owned work a change may touch, not everything visible nearby.
- **Hunk:** the selection unit when logical changes share a file.

For the common [acceptance, promotion, and activation distinctions](/CONTEXT.md#acceptance),
use the shared vocabulary; this fragment adds only the terms its instructions need.

## Operating weight

- commit each coherent stage when available, including prompts, research, drafts, and guidance: `jj commit -m "<description>" <files...>`. do not wait for the task or wave to end. before a review-driven revamp, save the present owned state; commit the revision separately and explain its delta and destination. checkpoint is not acceptance. the empty working-copy commit is expected; do not verify, move on. depth: /code/commits.md#commits-checkpoints
- changes intermingled **within** a file → `jj-hunk`: full `hunk-<sha256>` IDs only, always dry-run the spec first. load /code/jj-hunk.md before hunk-level work.
- review comments: discuss direction as well as committing the predecessor before editing; land each correction as its own well-argued commit. /code/review-handling.md
- plans carry no time estimates, ever. /code/planning.md
- module entry point: /code/README.md
