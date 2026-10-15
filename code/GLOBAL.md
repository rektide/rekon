---
id: code
order: 200
source: /code/README.md
status: draft
type: ContextFragment
title: Code — stage and change discipline
description: Active shared commit, review, planning, and coding discipline.
resource: /code/GLOBAL.md
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
extensions:
  ticket: rekon-phoenix
  related_work: [rekon-agents-maintenance, rekon-session-mementos]
  posture: active production fragment; guidance remains open to refinement
sources:
  - { resource: /code/README.md, title: Coding practice }
  - { resource: /code/commits.md, title: Capture revise compare }
  - { resource: /CONTEXT.md, title: Shared working vocabulary }
  - { resource: "opencode://ses_eb9709a91ffeKsq7fEUI1ptzt2", title: Ambient scaffold rule naming the pkg-prefix anti-pattern with layout depth }
---

# Code — stage and change discipline

Selected for production assembly. The README/SKILL body owns the fuller practice;
these rules must be usable before an agent chooses deeper reading.

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
- scaffold repos with domain-grouped top-level dirs — never a flat dump, never
  `<pkg>-` prefixes repeated in the tree (`crates/parser`, not
  `crates/<pkg>-parser`); a registry-name prefix belongs to the package name,
  not the directory. when offering layout options, offer multiple plausible
  domain groupings. depth: /code/layout.md
- Mermaid diagrams should use meaningful node identifiers. Breaking changes
  are fine below 1.0; assess actual persisted-data or consumer obligations
  rather than inventing them.
- Let It Fail when appropriate: do not catch an error merely to print it; retain
  its stack and cause. Handle errors where recovery or a real boundary needs it.
- Do not add automatic prune/reconciliation behavior unless explicitly requested.
  Create/update declared state and leave cleanup deliberate.
- module entry point: /code/README.md
