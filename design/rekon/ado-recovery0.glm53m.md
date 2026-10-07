---
type: Report
title: Compare doc/opencode-session.md against ~/ado and propose a deletion path
description: Coverage audit, citation inventory, and minimal-transfer plan for retiring doc/opencode-session.md into ~/archive/doc/opencode.
resource: /design/rekon/ado-recovery0.glm53m.md
tags: [docs, cull, opencode, session-storage, recovery]
status: draft
generated: { by: llm:glm-5.3, at: 2026-10-06 }
sources:
  - id: rekon-session-doc
    resource: /doc/opencode-session.md
    title: opencode session storage and recovery (the document under review)
    author: project:rekon
    last_modified: 2026-05-08
  - id: ado
    resource: file:///home/rektide/archive/doc/opencode/
    title: ~/ado — canonical opencode notes directory (README, history, store, sessions, reap, session-readers)
    author: human:rektide
  - id: doc-cull0
    resource: /design/rekon/doc-cull0.glm53m.md
    title: Prior cull wave that kept this document
    author: llm:glm-5.3
  - id: reconstruct
    resource: /code/task-reconstruction/philosophy.md
    title: Reconstruct from the present (principal inbound citant)
    author: model:openai-gpt-5.6-sol
---

# ado-recovery0: can rekon delete doc/opencode-session.md?

**Mandate.** Parent asked: compare [`doc/opencode-session.md`, pinned](https://github.com/rektide/rekon/blob/ece454d44301fdf1a7b00a21eaf6b42ad6eeee90/doc/opencode-session.md)
(1,367 lines) against `~/ado/` and its parent; identify covered / stale / uniquely
useful material and the minimal transfer that would let rekon delete the file;
propose faithful durable replacements for every inbound citation. Read-only
checkpoint — no deletions, no transfers, no citation edits made.

## 1. What ado actually is

- `~/ado` → symlink chain resolving to `~/archive/doc/opencode` — the canonical
  opencode notes directory inside the `~/archive/doc` jj repo. No `AGENTS.md`
  there or in the parent; conventions are visible in the files themselves:
  OKF frontmatter, filenames omit the `opencode` prefix, a curated
  [`README.md`](file:///home/rektide/archive/doc/opencode/README.md) index,
  and "canonical document" designations.
- Relevant neighbors: [`history.md`](file:///home/rektide/archive/doc/opencode/history.md)
  (tested SQLite search guide, 2026-08 era, covers V1 **and** V2),
  [`store.md`](file:///home/rektide/archive/doc/opencode/store.md) (canonical
  storage/schema/CDC overview incl. pragmas, ghost-reading patterns),
  `sessions.md` (lifecycle/presence), `reap.md` (deletion/compaction),
  `session-readers.md` (tool catalog: `opencode-session-fab`/`occ`, `cotails`,
  `opencode-session-rs`). The parent `~/archive/doc` tree holds broad-topic
  notes; nothing else there covers opencode session storage.

## 2. Coverage comparison (rekon doc → ado)

| rekon doc section (lines) | ado coverage | verdict |
|---|---|---|
| Database layout + "identifying the right DB" (5–38) | `store.md` §1, `history.md` §1 — incl. `$OPENCODE_DB` override, channel naming, newest-by-mtime | **Covered, better.** Rekon's hard-coded `opencode-.db` is dead: only `opencode-local.db` (23 GB, `IS-CURRENT` marker) and `opencode.db` (`IS-STALE`) exist now |
| Table structure session/message/part (78–161) | `store.md` §2 (V1 **and** V2 + event journal) | **Covered, better.** Rekon doc is V1-only — stale for any new session |
| Quick Start + finding-sessions SQL (40–76, 175–277) | `history.md` §3 (NOCASE, multi-word EXISTS/INTERSECT, snippets, perf on 4.8 GB, V2 event paths) | **Covered, better** |
| `session_diff/` JSON files: layout, schema, `added/deleted/modified`, git-apply format (145–161) | **none** — zero hits for `session_diff` anywhere in ado | **Uniquely useful.** Verified live: Phase A's 665 KB diff JSON still on disk; directory still receiving files |
| "Session diffs vs raw parts" doctrine — uncommitted-work recovery, `summary_files = 0` caveat (163–173, 497–499) | **none** | **Uniquely useful** — the core recovery insight (diffs ≈ tracked, `part` table = everything incl. uncommitted `apply_patch` inputs) |
| apply_patch extraction: `$.state.input.patchText`, `*** Add/Update/Delete File` headers, escaped `\n`, final-state reconstruction (302–389, 909–1026) | **none** — no `patchText`/`apply_patch` coverage in ado | **Uniquely useful** |
| `synthetic` filter on text parts (831–841) | **none** | Unique small nugget (excludes auto tool-result summaries from conversation) |
| Fork chains, `parent_id`, `@code-reviewer` matching, duration heuristics (422–476, 521–534) | `parent_id` appears in `store.md` schema only; heuristics absent | Minor, partially unique — cheap to carry in a transfer |
| Recovery workflow + pitfalls (501–586) | ms-timestamps/WAL covered by ado; "part data can be large" caution implicit in `history.md` perf notes | Mostly covered; pitfalls list is derivable |
| **Phase A recovery guide (589–1367, ~57 % of the file)** | **none — rekon-specific historical evidence** | See §4 |

Bottom line: roughly the first 587 lines are a stale-dated, V1-only subset of
what ado now maintains better, **except** four genuinely unique recovery
nuggets (session_diff files, diffs-vs-parts doctrine, patchText extraction,
synthetic filter). The Phase A appendix is rekon history, not ado material.

Evidence-health note: the Phase A session rows and parts **migrated intact**
into `opencode-local.db` (both session IDs present; `ses_1fbd28fa3ffe…` still
has exactly 539 parts, matching the doc's claim), and both cited
`session_diff` JSONs still exist. Only the doc's DB *path* rotted. The
architecture itself never re-landed: no `src/planner/` or `src/action/` in
today's rekon (src is `agents/ command/ symlink/`), so the appendix remains a
lapsed recovery brief — historical evidence, not live instruction.

## 3. Inbound citations and durable replacements

Git anchor for pinning: **`ece454d44301fdf1a7b00a21eaf6b42ad6eeee90`** (last
commit touching the file, "Add quick start, cross-session analysis, and search
guidance"). Verified **already contained in `origin/main`** (also dl/main,
tngl/main), so pinned blob URLs resolve immediately. This mirrors the
precedent doc-cull0 set for the dl cull ("commit-pinned to `e9c7c401…` with
line fragments preserved").

| Citant | What it uses | Replacement |
|---|---|---|
| [`code/task-reconstruction/philosophy.md`](/code/task-reconstruction/philosophy.md) frontmatter `phase-a-recovery` (L20); L577–580 links `#L589-L591` and `#L1326-L1357`; quote "adapt, don't copy" | The doc as *precursor evidence* | Pin both line-range links to `…/blob/ece454d4…/doc/opencode-session.md#L589-L591` / `#L1326-L1357`; frontmatter resource likewise. Argument untouched — pure retarget |
| [`constitution/threads0.glm53m.md`](/constitution/threads0.glm53m.md) frontmatter L98–99; body L177, L417; L854 blockquote of L591; L885 ("Quick Start: Finding where something was built" section title); L1070; L1157 (Thread 8 "genre specimen") | The doc as historical evidence / genre specimen | All pin to `ece454d4…` blob URLs with fragments; quoted text stays verbatim in threads0, so nothing is lost by the retarget |
| [`constitution/naming/exploration/draft0.sol56x.md`](/constitution/naming/exploration/draft0.sol56x.md) frontmatter L44–45; L497; L1090 — "session rows carry an opaque ID, parent/fork ID, title, project, timestamps; message metadata records agent and model" | The doc as **substrate evidence** for `naming-actor-braid` | Two options: (a) pin to `ece454d4…` (faithful, zero argument change); (b) retarget to ado `store.md` §2 schema, which now evidences the same claim more canonically — but that's a `file://` machine-local citation. **Recommend (a)**; note (b) for the naming owners |
| [`doc/README.md`](/doc/README.md) L43 — "five rekon-owned files remain" inventory | Live index | Rewrite the inventory line at deletion time (the cull paragraph is already historical narrative; adjust counts) |
| [`design/rekon/doc-cull0.glm53m.md`](/design/rekon/doc-cull0.glm53m.md) L101 — "Kept path-stable to preserve `#L…` citations" | Historical record of round 1 | Leave unchanged (wave history); its claim remains true of *that* commit |

General principle: quotes and line fragments live on in the citants; pinning
preserves them without rewriting any argument — consistent with both
doc-cull0's repair practice and the "evidence, not gravity" stance of
[`philosophy.md`](/code/task-reconstruction/philosophy.md).

## 4. Recommended minimal transfer (for parent review — nothing executed)

1. **New file `~/archive/doc/opencode/recovery.md`** (~100–150 lines), carrying
   only the four unique nuggets + minor heuristics: `storage/session_diff/`
   layout and JSON schema; diffs-vs-parts doctrine incl. `summary_files = 0`
   caveat; patchText extraction recipe (headers, escaped newlines,
   final-state reconstruction); `synthetic` filter; fork-chain/`@code-reviewer`
   heuristics; cross-link to `history.md` for search SQL and DB resolution.
   OKF frontmatter crediting rekon `doc/opencode-session.md@ece454d` as
   source. Update the ado `README.md` index with the new entry.
2. **Phase A appendix: do not transfer to ado.** It is rekon history. Options:
   - **(a) pinned citations only** (recommended — cheapest, matches
     precedent; the underlying evidence — session rows, 539 parts, diff
     JSONs — remains on disk in `opencode-local.db` / `storage/`), or
   - (b) archive L589–1367 into rekon [`.transcript/`](/transcript) (already
     home to curated session artifacts like `PLAN-combine.md`) if a locally
     browsable copy is wanted. Pick one, not both.
3. **Citation repairs** exactly per the table in §3, done in the same change
   as the deletion.
4. **Delete `doc/opencode-session.md`** and update `doc/README.md`'s
   inventory, completing the doc-cull that round 1 deliberately deferred.

## 5. Verification checklist before deletion

- [ ] Pin commit `ece454d4…` remains reachable on origin (verified today;
      re-verify at execution — this repo's jj history already shows GC'd
      objects from abandoned changes, so never pin to an unpushed/abandoned
      commit).
- [ ] Apply every §3 retarget; then re-grep for `opencode-session.md` (plain,
      `file://`, and `#L` fragments) across the repo — round 1's audit missed
      a file (`precedents0`) to truncated grep output; use `rg` unbounded.
- [ ] Confirm Phase A evidence still queryable in `opencode-local.db`
      (spot-check the two session IDs and part count 539).
- [ ] ado transfer committed in the `~/archive/doc` jj repo (separate repo,
      separate commit; no AGENTS.md there — follow its OKF/README
      conventions).
- [ ] `doc/README.md` inventory updated in the deletion commit.

## 6. Verdict and unresolved issues

**Safe deletion: yes, conditionally** — after the §4 transfer + §3 pinned
citations + §5 checks. The only content at risk is the Phase A appendix; the
recommended disposition (pinned citations, evidence on disk) preserves it as
historical evidence without keeping a 1,367-line marginal doc load-bearing.

Unresolved, for the parent:

1. **Phase A disposition (a) vs (b)** in §4.2 — pinned-only vs `.transcript/`
   archive. This is the one real judgment call; everything else is mechanical.
2. **naming draft0 retarget** (a) vs (b) in §3 — I recommend pinned-rekon, but
   the naming exploration owners may prefer the ado schema as living evidence.
3. **`cotails` overlap**: ado's search tooling (`cotails`, `occ`) is going to
   absorb the "find sessions" workflow entirely; if the parent agrees, the
   transfer should stay recovery-focused and skip generic search SQL.
4. ado has no AGENTS.md — if transfers there become routine, one may be worth
   writing (out of scope here).

Ownership: this report is mine; `doc/opencode-session.md`, all citants, and
`~/ado` were treated read-only per instructions.
