---
type: EvidenceBundle
title: Reproducible ambient-guidance comparison
description: Pinned measurements, line origins, source pairs, and diffs supporting the decomposition review.
resource: /design/context-compilation/review0/README.md
status: draft
generated: { by: "model:openai/gpt-6-astra", at: 2026-10-11 }
sources:
  - { resource: "urn:opencode:session:ses_f908bad7fffeW0pr4FLFa4VFyn", title: Human-requested revision-bound deep review and reproducible evidence collection }
---

# Evidence for the ambient-guidance review

Read [the report](/design/context-compilation/review0.gpt6a.md) for interpretation.
These artifacts preserve the observations separately from those judgments:

- [metrics.json](/design/context-compilation/review0/metrics.json): five pinned
  states, source/output consistency, words/bytes/lines, source hashes, and a
  disjoint domain allocation covering all 13 old H1 sections and 12 recovery
  fragments. A complete allocation is not proof of preserved meaning.
- [lineage.json](/design/context-compilation/review0/lineage.json): all 371
  baseline lines with their original commit, original line, and commit subject.
  Use it to revisit the workshop edits rather than infer intent from age.
- [paired excerpts](/design/context-compilation/review0/evidence0.gpt6a.md): full
  before/after domain text with revision-bound source line links.
- [legacy → cutover diff](/design/context-compilation/review0/diffs/legacy-cutover.diff):
  the activation change, including the new production selection.
- [cutover → recovery diff](/design/context-compilation/review0/diffs/cutover-recovery.diff):
  the restored fragments and rewritten prose.
- [recovery → later diff](/design/context-compilation/review0/diffs/recovery-later.diff):
  subsequent semantic HTML and session-provenance changes, kept separate from
  the recovery's reductions.

## Reproduce

From the repository root, with the existing Node dependencies installed:

```sh
node design/context-compilation/review0/measure.ts
```

The [script](/design/context-compilation/review0/measure.ts) reads only full commit
IDs through `git show`, `git diff`, and `git blame`. It never reads mutable
working-tree guidance or runs a historical assembler. It writes only this
bundle's generated evidence files. The diff files retain exact source edits;
the paired excerpts make cross-file decomposition readable without pretending
the files have normal rename ancestry.

Words are whitespace-separated strings in raw Markdown, including code and
markup; these are **not model tokens or rendered natural-language word counts**.
Bytes are UTF-8. Fragment bodies exclude metadata/comments and boundary
whitespace; whole-file sizes include the generated header/provenance. Real
Markdown H1 tokens determine old sections, excluding shell comments in fences.

Five states are pinned: pre-cutover, cutover, recovery, audit landing, and a
later **committed** observation. The later observation is not a claim about
ongoing uncommitted tool work. Its stored `AGENTS.md` and selected `CONTEXT.md`
disagree; the metrics identify this rather than silently substituting intended
source contents for what was actually assembled.

This is a report-specific research aid, not a new production compilation tool.
Keep this revision set fixed. A later comparison should create a new report
and evidence bundle rather than retarget this evidence to a moving tip.
Canonical GitHub URLs identify commits/files; local-only commits may not yet be
published there. The local reproduction commands work without publishing.
