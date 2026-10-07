---
type: Record
title: "doc/ cull — 31 to 5, dl-owned files left to rektide/dl"
description: Durable record of the 2026-10-06 removal of 26 dl-owned files from rekon doc/, the byte-level evidence behind it, pinned recovery links for dl-retired drafts, and the inbound-citation repairs that accompanied it.
resource: /design/rekon/doc-cull0.glm53m.md
tags: [documentation, dl, cull, preservation, doc-homes]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-10-06 }
extensions:
  commission: user authorized culling dl design files from rekon doc/; parent session's architectural checkpoint authorized this implementation
  audit: ".test-agent/doc-homes/preservation0.glm53m.md (ignored scratch; findings summarized here)"
sources:
  - { resource: "rekon commit 9a1501c", title: Remove dl utility, forked into its own project }
  - { resource: "https://github.com/rektide/dl/tree/main/doc", title: dl doc tree (github mirror; local main ahead) }
  - { resource: /doc/README.md, title: Doc module landing page with remaining inventory }
---

<a id="doc-cull0"></a>
# doc/ cull — 31 to 5

The user commissioned culling dl's design files out of rekon `doc/` "as we
go". A read-only preservation audit (same agent, same day) compared every
file against `~/src/dl` byte-for-byte before anything was removed; its
evidence is condensed here because the scratch report itself is ignored.

## What left — 26 files, all dl-owned

**Flow, execution, and reporting (8):** `flow.md`, `flow-runtime-stage-plan.md`,
`stream-core.md`, `walk.md`, `dl-preliminary-execution-design.md`,
`existing-state.md`, `log2.md`, `simplify-symlinks.md`.

**Phase and expansion notes (8):** `phase3.md`, `phase3-arch.md`,
`phase3-more.md`, `phase3-direction.md` (11,333-line transcript),
`expansion-space.md`, `expansion-serial-exhaustive.md`,
`expansion-heads.md`, `fancy-graph.md`.

**Discovery and plans (6):** `discovery/dexport-integrate.md`,
`discovery/gunshi-decomposition.md`, `discovery/gunshi-dl-decompose.md`,
`discovery/gunshi-plugin.md`, `discovery/repo-merge-plan.md`,
`plan/dl-parser-repo.md`.

**Transcripts (1):** `transcript/pluginification.md` (4,134-line session
transcript).

**Retired by dl itself (3):** `log.md`, `types-todo.md`, `output.md` — dl
received these in the fork and deliberately deleted them on 2026-06-20
(commits `b9b1aa5`/`f4a076b`, "Remove marginal dl docs"); rekon honored
that disposition rather than restoring them or creating new work.

The empty `doc/adr/` directory (never tracked) was removed too.

## Evidence

- rekon's fork-out commit `9a1501c` (2026-06-26) removed dl **code only** —
  zero `doc/` entries in its stat; the docs were left-behind vestige.
- 22 of the 23 filenames shared with `dl/doc/` were byte-identical
  (`cmp`), and blob-identical at dl's pushed github tip `e9c7c401`
  ("de-rekon", 2026-06-20). dl's `doc/index.md` catalogs them.
- `expansion-heads.md` differed by exactly one rekon section ("Later
  composition inquiry", added 2026-09-06, linking the ordered-pipelines
  memento). That appendix relationship is preserved by the memento's own
  cross-references ([`memento0.gpt6a.md`](/design/ordered-pipelines/memento0.gpt6a.md)
  cites the historical inquiry with staleness caveats), so no content
  transfer was needed; the pinned dl blob omits only that backlink.
- The six files with no current dl/doc counterpart were byte-identical to
  dl's pre-deletion blobs (`f4a076b^`, `4ddbe77^`) — no rekon-side edits
  after 2026-06-20. dl called `interpolate.md` and `plan-parallelize.md`
  "non-dl docs" (`4ddbe77`) — they stayed.

## Pinned recovery for the dl-retired three

Exact public-history copies (verified byte-equal to rekon's copies before
deletion; `e9c7c401` is dl's pushed github main):

- [`log.md`](https://github.com/rektide/dl/blob/e9c7c401725a93d2c4d56f41851fdb832d772b09/doc/log.md)
  — 337-line reporting-subsystem redesign draft, superseded by
  [`log2.md`](https://github.com/rektide/dl/blob/e9c7c401725a93d2c4d56f41851fdb832d772b09/doc/log2.md);
  dl's `src/report/` now exists.
- [`types-todo.md`](https://github.com/rektide/dl/blob/e9c7c401725a93d2c4d56f41851fdb832d772b09/doc/types-todo.md)
  — 210-line gunshi type-cast TODO against code that lives only in dl.

`output.md` was a single line, preserved verbatim:

```markdown
- TODO: make a ticket a ticket to add to `dl` a flag --no-log-cache that disables logging of cached file names.
```

No claim is made here that this TODO is implemented or obsolete; it was
retired with its file and is recoverable at
[`output.md`](https://github.com/rektide/dl/blob/e9c7c401725a93d2c4d56f41851fdb832d772b09/doc/output.md).

## What stayed — 5 files, rekon-owned

- [`doc/README.md`](/doc/README.md) — doc landing page; now notes this cull
  and the remaining inventory.
- [`doc/GLOBAL.md`](/doc/GLOBAL.md) — candidate ambient exposure (honing).
- [`doc/interpolate.md`](/doc/interpolate.md) — documents the live rekon
  command `src/command/interpolate.ts`; dl itself classified it "non-dl".
- [`doc/plan-parallelize.md`](/doc/plan-parallelize.md) — plan for rekon's
  `install-commands`; also "non-dl" per dl.
- [`doc/opencode-session.md`](/doc/opencode-session.md) — 1,367-line opencode
  session-storage/recovery guide, load-bearing for rekon citations
  (constitution threads, naming exploration, task-reconstruction
  philosophy); dl retired it as marginal. Kept path-stable to preserve
  `#L…` citations.

## Citation repairs made alongside the cull

All retargets are commit-pinned to `e9c7c401…` with line fragments
preserved, and name rektide/dl in link text at first mention:

- [`memento0.gpt6a.md`](/design/ordered-pipelines/memento0.gpt6a.md) —
  frontmatter sources and three cross-references (expansion-heads,
  flow-runtime-stage-plan, dl-preliminary-execution-design).
- [`constitution/README.md`](/constitution/README.md) — frontmatter source
  for log2; the log2/log worked example rewritten: the drafts illustrate
  supersession and now belong to dl, not an outstanding rekon migration
  candidate. Anchors and surrounding principles unchanged; authorship
  (`project:rekon`) retained — moving evidence does not change its author.
- [`threads0.glm53m.md`](/constitution/threads0.glm53m.md) — three
  pluginification links (frontmatter, body ×2).
- [`philosophy.md`](/code/task-reconstruction/philosophy.md) — frontmatter
  plus line-range evidence links for `phase3.md#L18-L32`, `#L66-L87`,
  `fancy-graph.md#L79-L105`.
- [`precedents0.gpt56s.md`](/remake/research/precedents0.gpt56s.md) —
  frontmatter source and the `phase3.md#L18-L62` precedent link; found by
  the post-deletion unbounded sweep (the audit's original grep output was
  truncated before this file).

Historical wave files received link repair only; no arguments revised.
A revision-pinned count claim mentioning `doc/log.md`
(`draft2-review0.sol56x.md:525`) was intentionally left as history.

## Checks performed

Inventory of all 31 files, with byte comparisons for the 29 fork-era files
against dl's current tree or pre-deletion blobs and all 26 removed files
against pinned `e9c7c401`; inbound-link sweep for every removed path
across rekon including `.beads` (no ticket references to removed files);
pinned-blob existence and line-count validity for every fragment-bearing
URL; post-edit sweep confirming no active rekon links remain to removed
files; `.test-agent/` excluded from the commit and the unrelated dirty praxis file untouched;
`~/src/dl` never modified.

## References

- Audit scratch report: `.test-agent/doc-homes/preservation0.glm53m.md`.
- dl entry points:
  [doc/ tree](https://github.com/rektide/dl/tree/main/doc) and, in dl's
  local (unpushed) tree, `doc/index.md`.
- Posture governing removal: [prospective adoption and existing
  files](/constitution/README.md#doc-constitution-lifecycle-existing).
