---
type: Report
title: Execute the opencode-session retirement — corrected and completed
description: Corrections to ado-recovery0, execution record of the ~/ado transfer and the rekon deletion with pinned citation repairs, and verification evidence.
resource: /design/rekon/ado-recovery1.glm53m.md
tags: [docs, cull, opencode, session-storage, recovery]
status: stable
generated: { by: llm:glm-5.3, at: 2026-10-06 }
sources:
  - id: predecessor
    resource: /design/rekon/ado-recovery0.glm53m.md
    title: ado-recovery0 (checkpoint report; superseded on the points corrected below)
    author: llm:glm-5.3
  - id: transfer
    resource: file:///home/rektide/archive/doc/opencode/recovery.md
    title: The executed transfer (~/archive/doc jj repo commit 957a905b)
    author: llm:glm-5.3
  - id: deletion-commit
    resource: https://github.com/rektide/rekon/commit/a7bd3064a4b940cdeb74f3cba1d1a897e407c8e4
    title: Deletion and pinned-citation-repairs commit (receipt; resolves once pushed)
    author: llm:glm-5.3
  - id: diff-removal
    resource: https://github.com/sst/opencode/commit/2f2fcc165439aec88a0e8e09c836d89c96977c6b
    title: 'fix(opencode): remove automatic full session diffs (#30127)'
    author: upstream:sst
    last_modified: 2026-05-31
---

# ado-recovery1: retirement executed, predecessor corrected

Parent approved ado-recovery0 with four corrections. This revision records
the corrections, the execution, and the verification evidence. The
predecessor stays as committed history.

## Corrections to ado-recovery0

1. **Withdrawn: "architecture never re-landed."** I inferred fate from the
   absence of `src/planner` in today's rekon — invalid, because dl forked out
   with its pipeline to `~/src/dl`; existence outside rekon cannot establish
   recovery failure. Correct statement: the Phase A appendix is a
   *historical recovery brief predating the fork*; no claim about the
   architecture's fate is made here without separate evidence.
2. **Soften the diffs-vs-parts doctrine.** The old doc's "diffs ≈ committed;
   parts = everything" is too categorical. The transfer instead says:
   summaries/diffs can omit useful work, inspect both sources; tool inputs
   show *attempted* edits — successful results or the final tree are needed
   to establish applied/final state. No claim that `session_diff` intrinsically
   tracks Git commits or that raw logs are complete.
3. **Extraction hygiene.** JSON-decode `part.data` exactly once, then split
   blocks per file. The old doc's raw `.replace('\\n','\n')` and naive
   all-plus-lines stripping are documented as *failure modes*, not recipes —
   both reproduced on live data (below).
4. **Version labels and provenance.** All transferred SQL is labeled V1
   projection with links to ado `store.md`/`history.md` for V2 shapes;
   extraction keeps `state.status` filtering (completed vs error) and
   part/session/time provenance; `synthetic` filtering is presented as a
   view choice, originals preserved as evidence.

## What was discovered during verification (new since ado-recovery0)

- **Automatic full session diffs were removed in the cited lineage; legacy
  files remain useful.** Upstream removed automatic full session diffs
  2026-05-31
  ([sst/opencode `2f2fcc16`](https://github.com/sst/opencode/commit/2f2fcc165439aec88a0e8e09c836d89c96977c6b),
  #30127) — verified in that lineage, not a universal claim. Monthly scan of
  all 3,361 local files: populated through June 2026, then **zero non-empty
  files in July/August 2026** on this install. Files through June 2026
  remain useful recovery artifacts; the transfer says so explicitly.
- **Legacy entry shape is legal.** `session-diff-missing-patch.test.ts` in
  the upstream tree shows entries without `patch`/`status` must be handled.
- **V1 `part` is still alive on this install.** The newest session in
  `opencode-local.db` has 58 `part` rows (and 7 `session_message` rows), so
  V1-projection queries remain locally useful even though upstream V2 moved
  on — this refines ado-recovery0's "V1-only = stale" framing.
- **Verified failure modes** (live DB, read-only): raw
  `replace('\\n','\n')` before `json.loads` → `JSONDecodeError: Invalid
  control character` on a real patch whose content contains literal `\n`;
  cross-file `+` stripping merges files — a real Phase A part
  (`prt_e0447bb76001nVO19DbjppBHlG`) carries two `Add File` blocks (60 + 65
  lines) that the naive recipe would concatenate.
- **Verified extraction recipe.** Per-block parsing on real multi-file
  patches (Add and Update) produced correct per-file content with
  provenance; `Update File` blocks confirmed as `@@` hunks with context /
  `+` / `-` lines, not full files.
- **Status split is real.** A recent session shows 409 completed vs 5 error
  tool parts; completed *write* tools, confirmed by their output, record
  application — completed status alone does not make a generic tool call a
  write, and final state still requires the tree/VCS.
- **Summaries omit work (inspected records).** Tool records for
  `ses_1fba1ccd1ffe…` (`summary_files=3`) contain twelve completed
  `Add File` operations — nine source files plus tests, with `doc/log2.md`
  added twice as separate operations. Established by reading the tool
  records, not by comparing summary counts with message/part totals.

## Execution record

1. **Transfer** — `~/archive/doc/opencode/recovery.md` + `README.md` index
   entry, committed in the `~/archive/doc` jj repo as **`957a905b`**
   ("docs: add session work recovery guide distilled from rekon"). ~170
   lines, recovery-focused per the parent's `cotails` note (no generic
   search SQL copied — that lives in ado `history.md`, cross-linked).
2. **Deletion + repairs** — rekon commit **`a7bd3064a4b9`**
   ("Delete doc/opencode-session.md; retarget citations to pinned bytes"):
   - [`code/task-reconstruction/philosophy.md`](/code/task-reconstruction/philosophy.md):
     frontmatter `phase-a-recovery` + the `#L589-L591` and `#L1326-L1357`
     evidence links → pinned blob URLs at `ece454d44301…`; argument and
     quotes verbatim.
   - [`constitution/threads0.glm53m.md`](/constitution/threads0.glm53m.md):
     frontmatter + 7 body sites pinned, including the L591 blockquote
     attribution and the "Quick Start" section-title citation.
   - [`constitution/naming/exploration/draft0.sol56x.md`](/constitution/naming/exploration/draft0.sol56x.md):
     frontmatter + 2 body sites pinned (naming-actor-braid substrate
     evidence).
   - [`design/rekon/doc-cull0.glm53m.md`](/design/rekon/doc-cull0.glm53m.md):
     opencode-session link pinned to historical bytes; interpolate and
     plan-parallelize links follow their later `tool/cmd/` homes, with a
     note that the five-file disposition records the first cull. Historical
     count (5) and argument preserved; GLOBAL noted as later redirect.
   - `doc/opencode-session.md` deleted (1,367 lines). Phase A preservation
     is **pinned-only** at
     [`ece454d44301…`](https://github.com/rektide/rekon/blob/ece454d44301fdf1a7b00a21eaf6b42ad6eeee90/doc/opencode-session.md)
     (verified contained in `origin/main`); no `.transcript` copy, per
     parent decision.
3. **Post-repair sweep** — `rg 'opencode-session\.md'` unbounded: only
   pinned blob URLs and ado-recovery0/1's own references remain. No live
   `/doc/opencode-session.md` links.

## Incident: change-id collision during concurrent commits

My first deletion commit (`b045c4f0`) raced concurrent colocated
Git/JJ activity; jj's auto-import cross-tagged change ids (my change id
landed on the parent's `f5c9f925`, later on their `ab6dc162`). Repair
performed: re-committed my five files from the working copy as `a7bd3064`
(byte-identical diff verified with `diff <(jj diff -r b045c4f0 --git) …`),
then abandoned the orphaned `b045c4f0`. At that checkpoint,
`change_id(ormunqlq)` was flagged divergent because the
parent's unrelated "design: factual corrections to shortcodes0"
(`ab6dc162`) shares the tag with my `a7bd3064`. I did not touch the
parent's commit. All content on both sides was verified intact. Parent
reconciliation subsequently gave the distinct detached commits new change IDs
with `jj metaedit --update-change-id`, leaving active-line commits unchanged.
The two affected IDs (`ormunqlq`, `ntsmtqtu`) each resolve without divergence.
Reviewed shortcode corrections and ticket notes were carried onto the active
line in `8ef498ec`; the earlier side commits remain preserved.

## Verification summary

| Check | Result |
|---|---|
| Pin commit reachable on origin | `ece454d44301…` ∈ `origin/main` (also dl/main, tngl/main) |
| Phase A evidence on disk | both session rows + 539 parts in `opencode-local.db`; both `session_diff` JSONs present (665 KB / 88 KB) |
| Unbounded reference sweep | no live links left; only pinned URLs + these reports |
| Transfer readability | committed cleanly in doc repo (`957a905b`) |
| Extraction recipe | per-block parse verified on real multi-file Add/Update patches |

## Limits and handoff

- The empty-after-June-2026 `session_diff` bound is empirical (this
  install's files). Other installs differ by build lineage.
- V1/V2 duality on this install (parts still written locally) may end when
  the local build updates; recovery.md states the check to run.
- `doc/README.md` inventory update is the parent's, per instructions.
- The divergence flag on `change_id(ormunqlq)` awaited parent
  reconciliation, as above. (Since resolved: `a7bd3064` reads `div=false`
  after the parent's metadata reconciliation.)

# Addendum: review round 2 (2026-10-06)

Parent review of the executed transfer and this report required narrow
factual/recipe corrections. Rekon commits were held while the parent
reconciled metadata; the parent then integrated these corrections. Applied in
this round:

1. **Counts are not evidence.** The summary-omits-work claim in
   "What was discovered" and in the transfer now rests on inspected tool
   records (twelve completed `Add File` operations behind
   `summary_files=3`), with an explicit note that message/part totals mix
   dialogue, reads, and searches and are not comparable.
2. **Path claim corrected.** "Paths are absolute" was a sample
   generalization; upstream docs specify project-root-relative paths and
   the apply-patch parser accepts either. The transfer retains recorded
   paths and resolves relative names against the session's `directory`.
3. **Extractor rewritten and tested.** Ordered per-operation JSON records
   (no path-keyed merging — real repeated `Add` of `doc/log2.md` stays
   separate), `+`-prefix validation on `Add File` bodies with raw fallback,
   `Move to:` as rename metadata, in-hunk `*** End of File` kept raw,
   `Update`/`Delete` as raw deltas, explicit no-final-file claim. Markers
   verified against the V1 parser
   (`packages/opencode/src/patch/index.ts`). Tests: synthetic
   repeated-Add/Delete + Move-to + End-of-File + literal `\n`; real
   two-`Add`-block part (`prt_e0447bb76001…`, 60/65-line contents); full
   Phase A run (60 ordered ops, 6 repeated paths); **published snippet
   extracted from the committed markdown and executed verbatim** against
   the gap-assessment session.
4. **Deletion-commit receipt fixed.** The blob URL pointed at a commit
   where the file is absent; now the canonical
   [`/commit/a7bd3064a4b94…`](https://github.com/rektide/rekon/commit/a7bd3064a4b940cdeb74f3cba1d1a897e407c8e4)
   receipt (resolves once pushed).
5. **Universality softened.** "session_diff dead going forward" now states
   the removal was in the cited upstream lineage with the June-2026
   installation cutoff as local evidence, and legacy files remain useful;
   "only completed parts establish applied edits" now distinguishes
   completed *write* tools confirmed by output from generic completed
   calls, with final state left to the tree/VCS.
6. **Source date corrected.** `ece454d` is 2026-05-15, not 2026-05-08 (the
   latter was a session_diff file mtime); fixed in the transfer's
   frontmatter and in ado-recovery0's.

Transfer corrections committed in the `~/archive/doc` jj repo as
**`93e82c1e`** ("docs: corrections to recovery guide from review") — ado
commits are safe there, no concurrent work. The rekon-side edits in this
round (this addendum, the four report fixes above, ado-recovery0's date)
were included in the parent's serialized documentation checkpoint.
