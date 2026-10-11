---
type: Assessment
title: What changed when the operating manual became fragments
description: Revision-bound analysis of ambient guidance growth, compression, relocated detail, weakened instructions, and inherited contradictions, with reproducible evidence and workshop commit links.
resource: /design/context-compilation/review0.gpt6a.md
status: draft
generated: { by: "model:openai/gpt-6-astra", at: 2026-10-11 }
sources:
  - { resource: "urn:opencode:session:ses_f908bad7fffeW0pr4FLFa4VFyn", title: Recovery implementation, human correction about workshopped wording, and commissioned revision-bound decomposition review }
  - { resource: "https://github.com/rektide/rekon/blob/ab672848a82170a5575dc32c227a5056008e3f37/AGENTS.md", title: Exact pre-cutover operating manual }
  - { resource: "https://github.com/rektide/rekon/commit/ed2b366b71e67e9e6eec6ec1fd3170645aca9f2a", title: Human-directed five-fragment cutover }
  - { resource: "https://github.com/rektide/rekon/commit/00cc45a247f26b7598864cd76a2b621f0927b107", title: This session's operational guidance recovery }
  - { resource: "https://github.com/rektide/rekon/commit/3e99a5ad102531dbfaa98b572eb09f80fd592242", title: Coverage auditor landing }
  - { resource: "https://github.com/rektide/rekon/commit/fe78712b42beec706e5f08eb05657cdc577d025f", title: Fixed later committed observation, separating ongoing semantic-markup work }
  - { resource: /design/context-compilation/review0/README.md, title: Reproducible measurements, source pairs, exact diffs, and line-origin records }
  - { resource: /design/rekon/context-weave/inventory0.gpt56s.md, title: Earlier 32-cohort inventory and preservation-before-extraction contract }
---

# What changed when the operating manual became fragments

<a id="conclusion"></a>
## Main finding and recommended reading order

**The recovery restored important operating capabilities, but it also rewrote
already-workshopped guidance. Section coverage is not fidelity to those decisions.**
The highest-value review is of instruction strength and missing distinctions,
not a contest to minimize words.

Read in this order:

1. **Collaboration/stewardship:** half the words remain, and specific workshop
   distinctions were compressed away. Revisit the original edits, not merely
   the newest summary.
2. **Writing/waves:** the operational fragment is 77% smaller than the broad
   legacy documentation section. Some duties were softened and procedural
   cues disappeared, while conceptual material gained more ambient weight.
3. **The glossary/constitution allocation:** these two exposures occupy 41.5%
   of the recovered fragment words. The question is whether that distribution
   serves ordinary work; their existence is not evidence they should be cut.
4. **JS and jj linked detail:** large reductions rely on existing READMEs that
   themselves contain contradictions. A link does not establish a safe handoff.
5. **Beads:** IDs and graph direction are restored. Review the routed commands
   and storage refinement, rather than treating its reduced size as lost ID policy.

This is an analysis of the named revisions, including my own recovery work.
Judgments below are recommendations for review, not accepted changes to policy.
The [evidence bundle](/design/context-compilation/review0/README.md) lets another
reader inspect every before/after domain without trusting these summaries.

<a id="revisions"></a>
## Revision map: which change did what?

All links refer to `rektide/rekon`. The GitHub URLs identify exact commits and
files; some local commits may not yet be published, so local `jj`/`git` commands
are the immediately usable fallback. No web availability is claimed.

| Name used here | Commit | What it establishes |
| --- | --- | --- |
| **L — legacy** | [ab672848](https://github.com/rektide/rekon/commit/ab672848a82170a5575dc32c227a5056008e3f37) | Last pre-cutover tree. Its [AGENTS.md][legacy] is byte-identical to `AGENTS.old.md` at recovery: SHA-256 `c474204ac00f45345bd27fb754e53bd767f51361633e4420f59eb4cb82f7c383`. This commit's subject is unrelated; it is a tree checkpoint, not authorship of the manual. |
| **C — cutover** | [ed2b366b](https://github.com/rektide/rekon/commit/ed2b366b71e67e9e6eec6ec1fd3170645aca9f2a) | October 7 selection of five fragments and preservation of the monolith as `AGENTS.old.md`. The commit explicitly records human-directed activation ahead of full extraction. |
| **R — recovery** | [00cc45a2](https://github.com/rektide/rekon/commit/00cc45a247f26b7598864cd76a2b621f0927b107) | This session's twelve-fragment restoration and summaries. Most reductions assessed here were introduced by this recovery, not by a mechanical move. |
| **T — audit** | [3e99a5ad](https://github.com/rektide/rekon/commit/3e99a5ad102531dbfaa98b572eb09f80fd592242) | Adds literal coverage checking and documentation. `AGENTS.md` is byte-identical to R. It does not add further ambient advice. |
| **P — provenance** | [5a2d6e9e](https://github.com/rektide/rekon/commit/5a2d6e9ef3a946e03eb7e31bc08081ccf502fa1b) | Later required native-session provenance in writing guidance and refs, with regeneration. This is a new substantive addition, separate from R. |
| **H — later committed observation** | [fe78712b](https://github.com/rektide/rekon/commit/fe78712b42beec706e5f08eb05657cdc577d025f) | Committed semantic definition-list flow correction. Its source `CONTEXT.md` differs from its stored `AGENTS.md`; the report distinguishes those two surfaces. |

### Direct comparisons

| Question | Canonical comparison | Locally retained exact patch |
| --- | --- | --- |
| What disappeared from activation at cutover? | [L → C](https://github.com/rektide/rekon/compare/ab672848a82170a5575dc32c227a5056008e3f37...ed2b366b71e67e9e6eec6ec1fd3170645aca9f2a) | [legacy-cutover.diff](/design/context-compilation/review0/diffs/legacy-cutover.diff) |
| What did recovery restore or rephrase? | [C → R](https://github.com/rektide/rekon/compare/ed2b366b71e67e9e6eec6ec1fd3170645aca9f2a...00cc45a247f26b7598864cd76a2b621f0927b107) | [cutover-recovery.diff](/design/context-compilation/review0/diffs/cutover-recovery.diff) |
| What did later work change? | [R → H](https://github.com/rektide/rekon/compare/00cc45a247f26b7598864cd76a2b621f0927b107...fe78712b42beec706e5f08eb05657cdc577d025f) | [recovery-later.diff](/design/context-compilation/review0/diffs/recovery-later.diff) |

The patches intentionally cover operating-context sources and output, not all
unrelated repository changes in those commit ranges. Normal diffs cannot align
one old section with several new files well; the [paired domain excerpts][pairs]
provide that cross-file view with exact source line links.

```sh
# Exact tree comparisons; readable now even if GitHub has not received these commits.
jj diff --git --from ab672848a82170a5575dc32c227a5056008e3f37 --to ed2b366b71e67e9e6eec6ec1fd3170645aca9f2a AGENTS.md agents.manifest.json
jj diff --git --from ed2b366b71e67e9e6eec6ec1fd3170645aca9f2a --to 00cc45a247f26b7598864cd76a2b621f0927b107 agents constitution code vcs js praxis/beads agents.manifest.json AGENTS.md
jj diff --git --from 00cc45a247f26b7598864cd76a2b621f0927b107 --to fe78712b42beec706e5f08eb05657cdc577d025f CONTEXT.md constitution/practice/GLOBAL.md refs/GLOBAL.md AGENTS.md
```

<a id="method"></a>
## Method and confidence boundaries

The [reproduction script](/design/context-compilation/review0/measure.ts) uses
`git show` at full commit IDs, Markdown tokenization to locate real H1 sections,
the generated provenance boundaries to separate assembled fragments, and
`git blame --line-porcelain` to recover last-touch line history. It verifies:

- all 371 legacy lines have line-origin records;
- the pre-cutover manuscript equals the preserved legacy file;
- all 13 old H1 sections and all 12 R fragments occur exactly once in the
  disjoint quantitative allocation;
- each assembled fragment can be compared against its selected source at the
  **same revision**, rather than assuming they match;
- R and T have identical generated guidance.

**Measurements:** words are whitespace-separated strings in raw Markdown,
including code examples and syntax; bytes are UTF-8, not model tokens. Whole-file
counts include generated comments; fragment counts exclude metadata, provenance
comments, and boundary whitespace. Line counts reflect wrapping and HTML layout,
so they are a poor proxy for informational growth. HTML changes can change raw
word counts without changing definitions.

**Judgments:** I read the legacy and recovered domain text, checked the linked
JS, commits, hunk, and beads guides at R, and inspected workshop origins for
the consequential changes below. This is a 32-cohort review, not a certified
sentence-by-sentence semantic equivalence proof. “Absent” below means absent
from the named recovered exposure or inspected destination, not nonexistent
anywhere in the repository. Blame identifies the last touching commit; a restore
commit is not necessarily the original author of an idea.

Verification receipt: [`verify.ts`](/design/context-compilation/review0/verify.ts)
checked **91 unique references**, **15 referenced commit objects**, **19 pinned
file blobs**, contiguous **32-cohort coverage of all 371 baseline lines**, and
byte-for-byte reproduction of **six generated artifacts**. The two research
scripts also passed scoped `tsgo` typechecking and oxlint. This validates the
evidence mechanics, not every semantic judgment in the report.

The prior [32-row inventory](/design/rekon/context-weave/inventory0.gpt56s.md)
was a useful decomposition precedent, but its September 6 line numbers and hash
are not this baseline. The ranges in this report were updated against L.

<a id="sizes"></a>
## Size and allocation: what actually got bigger or smaller?

### Whole output by stage

| Stage | Whole-file words | UTF-8 bytes | Lines | Fragment-body words |
| --- | ---: | ---: | ---: | ---: |
| L, handwritten | 4,752 | 31,877 | 371 | Not fragmented |
| C, five-fragment cutover | 2,194 | 16,701 | 321 | 2,126 |
| R, recovered twelve-fragment output | 4,048 | 31,244 | 564 | 3,931 |
| T, audit landing | 4,048 | 31,244 | 564 | 3,931 |
| H, later stored output | 4,203 | 32,357 | 580 | 4,086 |

The cutover removed almost half the bytes while adding conceptual/reference
material; its small output was not evidence of equivalent compression. Recovery
returned the file to within 2% of the legacy byte size. Yet the allocation of
attention changed dramatically.

### Disjoint L → R domain allocation

| Domain / source pair | Old words | R words | Delta | Reading implication |
| --- | ---: | ---: | ---: | --- |
| [Collaboration][pair-collaboration] | 558 | 281 | −277 / −49.6% | Also receives discussion preference from old documentation, so its retained original material is smaller still. |
| [Documentation + current constitution][pair-documentation] | 1,554 | 963 | −591 / −38.0% | 360 words of operational practice plus 603 of broader constitutional context; not interchangeable content. |
| [Workspace + experiments + shell][pair-workspace] | 279 | 170 | −109 / −39.1% | Mostly straightforward consolidation. |
| [Planning + code + jj + hunks][pair-code] | 958 | 574 | −384 / −40.1% | Grouped to avoid falsely counting internal transfers as additions. |
| [JS/TS + Web Components][pair-js] | 704 | 246 | −458 / −65.1% | Heavy reliance on routed examples. |
| [Rust][pair-rust] | 125 | 137 | +12 / +9.6% | Small growth, but an actual config-policy revision. |
| [Beads][pair-beads] | 574 | 266 | −308 / −53.7% | Basic loop stays inline; graph/recovery depth moves. |
| [Standalone glossary][pair-glossary] | No matching section | 1,028 | +1,028 exposure | New ambient allocation, not proof that all concepts are new. |
| [Ticket pointers][pair-tickets] | No matching section | 90 | +90 exposure | Overlaps some old notes/lifecycle ideas. |
| [Curated references][pair-refs] | No matching section | 176 | +176 exposure | New skill/source routing surface. |
| **Total** | **4,752** | **3,931** | **−821 / −17.3%** | Whole-file metadata accounts for the different total above. |

The seven legacy-oriented groups lose **2,115 words**. Three additional
standalone exposures add **1,294**, yielding the net −821. This arithmetic
explains why a similar-sized file can still feel as if practical advice vanished.
It does not label the additional exposures unnecessary.

Two useful alternate views, **not extra rows to sum into that table**:

- Documentation operational practice alone: 1,554 → 360 (−76.8%); adding the
  glossary as well as constitutional context gives a broad knowledge cluster
  of 1,991 words (+28.1%). The cluster got bigger while its operational portion
  got much smaller.
- Beads plus the separate pointer practice: 574 → 356 (−38.0%). IDs are restored;
  source/route/verification detail is where the meaningful comparison remains.

<a id="findings"></a>
## Consequential findings with workshop evidence

### F1 — Accepted-tip instruction became optional

**Before:** “if ‘accepted’ please make a prefix-less symlink for the accepted wave.”
**After:** “a suffix-less symlink **can** identify it.”

Compare [legacy L47][wave-before] with [practice R39–41][wave-after]. The earlier
[wave clarification commit](https://github.com/rektide/rekon/commit/26aa166151394e9945718d1dcdbfe41c23e6a45f)
also established same-wave isolation. Isolation remains imperative; creating
the accepted-tip marker does not. This is a change of obligation, not merely
shortening. **Recommendation:** restore the request strength unless the user
deliberately chooses optional tip creation. Naming clarification can happen
without weakening the action.

### F2 — A negotiated architectural distinction was omitted

The legacy annealing paragraph says **“preserve meaningful differences within
shared machinery.”** [The new stewardship text][agents] retains exploration,
assumptions/costs/opportunities, authorized scope, and coherent slices, but not
that distinction. Its [originating amendment](https://github.com/rektide/rekon/commit/cf036c5a995d09317b98f5450f12101b0d45ccd5)
specifically replaced “Generalize from real needs, not hypothetical futures”
with affirmative exploration of plausible futures **while preserving meaningful
differences**. That history makes the missing clause particularly consequential.

**Recommendation:** revisit the whole original annealing amendment, not just
append a keyword. The desired combination is ambitious exploration plus good
abstraction boundaries, not generic permission to imagine future features.

### F3 — Trusting reported checks lost some of its explicit counterweight

[Legacy L17][stewardship-before] distinguished trusting that tests were run from
judging whether they cover intended behavior; it asked for important edge-case
inspection and concrete feedback. [R][agents] says “Trust reported checks,”
then “Read relevant code and evidence, and add verification when it answers a
specific concern.” That preserves the broad boundary but weakens the explicit
instruction to judge the adequacy of the checks and give concrete feedback.

The precise [workshop correction](https://github.com/rektide/rekon/commit/852ee56a3932120c62e1753f86659c4a1b1d4d25)
is worth more than a freshness check here. The surrounding
[stewardship introduction](https://github.com/rektide/rekon/commit/80bcfdc62805165f5dbe76cf60da827b1b818898)
also required checkpoint reports with references and review before steering the
next slice. Some of those requirements now depend on combining separate bullets.
**Recommendation:** preserve the paired distinction together in ambient text.

### F4 — Doc-pass retains the goal but loses several execution cues

The [old seven-step process][docpass-before] explicitly said to derive search
terms from the new work, inspect indexes first, verify surrounding text, and ask
a search agent for file/line references, relationship types, and notable
absences. [The new paragraph][practice] retains concept search, meaningful
relationships, cross-references, reciprocal links, indexes, and link verification.
It no longer gives those exact execution/return cues.

Some are reasonable depth candidates, but R did not create a dedicated complete
doc-pass procedure behind its pointer. The prior inventory had already warned
that the constitution mentions doc-pass without owning the full recipe.
**Recommendation:** restore the tuned process or place its complete operational
form at a deliberate, directly linked home before calling this a lossless move.

### F5 — Checkpoint discipline survives, with one routed qualifier

[R code guidance][code] preserves coherent-stage commits, prompts/research/drafts,
checkpoint-before-revision, separate revision commits, and human discussion.
The explicit **“Wave gates delay dependent analysis, not authors' commits”**
from [the checkpoint amendment](https://github.com/rektide/rekon/commit/18ab95b34452b12778f5b31afe8d613397f3073b)
is no longer stated verbatim ambiently, but its meaning is present in the linked
[commit guide][commits] (evidence gates do not delay saving authors' results).
This is a verified routing change, not demonstrated knowledge loss.
**Recommendation:** decide whether that short qualifier is costly enough to miss
that it should remain with the ambient rule.

### F6 — The linked JS recipe contradicts its own nonblocking guidance

The smaller [JS GLOBAL][js] preserves asynchronous `realpath`, `.then`, dynamic
main loading, direct `.ts` execution, and distribution-only builds. The linked
[JS README's CLI recipe][js-cli] contains:

```ts
const isMain = await realpath(process.argv[1]).then(/* … */)
if (isMain) {
    await import('./foo.main.ts')
}
```

It then instructs “Use `.then` rather than top-level `await` … so importing the
module as a library never blocks.” The example itself uses top-level await;
it also calls `realpath(process.argv[1])` without first checking whether the
argument exists. **This is an inherited support-document problem**, not code
introduced in the recovery. But routing agents there increases its relevance.
**Recommendation:** repair and exercise the recipe before relying on it as the
replacement for tuned inline CLI guidance.

### F7 — The linked commit guide is stricter than the operator's push exception

[Legacy L241][push-before] and [R's jj GLOBAL][jj] allow pushing when explicitly
directed by the operator. The [linked commits guide][commits] says “Do not push
work; the human pushes when they want to,” without that exception. The older
[inventory](/design/rekon/context-weave/inventory0.gpt56s.md#ambient-guidance-inventory)
explicitly identified the operator-edited exception as something not to regress.

This divergence predates R. **Recommendation:** reconcile the owner and exposure
with the operator's rule; migration must not silently select the stricter copy
because it happens to be the linked body.

### F8 — The earlier jj-hunk extraction was already a deliberate ambient cut

[Commit e148e0f5](https://github.com/rektide/rekon/commit/e148e0f57a00837f6548cb1b6cd9ca3b4eeb63a4)
had moved full syntax to [the hunk guide][hunks] while leaving an ambient spec
shape, full-ID warning, mandatory dry-run, whole-rewrite caveat, and route.
R preserves IDs/dry-run/caveat/route, but drops the inline spec shape and explicit
“file boundaries align → prefer plain commit” comparison. Both remain in the
guide; the new wholesale-rewrite cue improves preservation by saying to save
the rewritten version first.

**Recommendation:** review this against the already-cut e148 state, not the
earlier long hunk manual. The question is whether the remaining cheap spec and
tool-choice cue should have been cut a second time.

### F9 — Beads IDs are actually restored; recipe and storage changes need separate judgments

The [legacy policy][beads-before], introduced in the
[beads integration](https://github.com/rektide/rekon/commit/0ee759061622b93c53b771fe218a748d2fdc0ee7),
requires stable human-readable `--id`, epic-child namespaces, dependent-first
blocking edges, append-notes, and no `bd prime`. All are explicit in [R][beads],
with runnable-shaped create/update/link/close examples. This is strong recovery,
not a remaining ID-policy gap.

The [new README][beads-readme] owns epic/feature creation, ready listing,
supersession, rename, and graph checks. Full command expansions were routed;
that is a separate activation decision. The old SQLite-file delete/import
recovery recipe was deliberately replaced by Dolt-aware version guidance.
The [auto-export amendment](https://github.com/rektide/rekon/commit/27888e1b2b7b447662b671094c91901893bdfb89)
still matters: R keeps `export.auto=true`, but adds “where supported.”

**Recommendation:** preserve the recovered basics; review command depth and the
storage change on their own merits. This report confirms the textual change,
not a newly executed backup/restore procedure. No destructive procedure was run.

### F10 — Rust growth contains a policy revision, not just extra explanation

The [legacy config paragraph][rust-before] directs a fixed derive set and a
`Serialized::defaults(CliArgs::parse())` provider while stating CLI > env > defaults.
[R][rust] says derive those types **“where useful”** and warns that one defaults
provider cannot alone establish the claimed layering. It preserves the intended
precedence and publishing order.

The qualification is technically motivated, but “where useful” relaxes the
derive instruction. **Recommendation:** judge that relaxation separately from
correcting the incomplete example, and validate a replacement against the
actual Figment2 version if an executable recipe is desired. This review does
not certify an unrun configuration integration.

### F11 — Expanded conceptual exposure is an allocation decision, not automatic bloat

At R, the [glossary][glossary] is 1,028 words and [constitution][constitution]
603: 1,631 / 3,931 = **41.5%** of fragment words. They define durable names,
knowledge/exposure/activation, work, evidence, and ownership. They were already
selected at C; the recovery did not invent their whole footprint.

The useful review is task-shaped: which concepts avert real errors before an
agent loads a practice, which benefit from local repetition, and which displace
equally important operational conditions? **Recommendation:** inspect the
combined reading experience rather than declare either the glossary or all
repetition expendable. The user explicitly allowed repetition during honing.

### F12 — Later growth must not be misattributed to the recovery

[The provenance requirement](https://github.com/rektide/rekon/commit/5a2d6e9ef3a946e03eb7e31bc08081ccf502fa1b)
adds **105 words** to documentation practice and **50** to refs in stored output:
R 3,931 → H 4,086 fragment words. That is a later human-directed addition, not
evidence that recovery grew those fragments by that amount.

Semantic glossary conversion began at
[f649d5e9](https://github.com/rektide/rekon/commit/f649d5e91f24ce6c3c9a8f1282258e9af1abc53e)
and was flow-corrected at [H](https://github.com/rektide/rekon/commit/fe78712b42beec706e5f08eb05657cdc577d025f).
The definitions' element representation changed; raw markup/line growth should
not be equated with additional conceptual obligations. At H the stored output
still contains the earlier glossary bytes. Ongoing working-tree regeneration
and Rust tool edits are outside this fixed committed observation.

<a id="cohorts"></a>
## All 32 functional cohorts: disposition map

Ranges refer to [L's exact AGENTS.md][legacy]. This is finer than the audit's
13 H1 buckets and follows the earlier workshop inventory's grain. Rows cover
lines 1–371 contiguously, including headings and separators. `Retained` means
the principal behavior is explicit; it does not certify every phrase unchanged.

| # | Legacy lines / cohort | Current destination at R | Disposition and review focus |
| ---: | --- | --- | --- |
| 1 | 1–12, dispatch | [agents][agents] | **Compressed/routed.** Background, priming, owned artifacts, references remain. Model-quality detail routes to `agents/general.md`; continuation vocabulary corrected. |
| 2 | 13–21, stewardship | [agents][agents] | **Compressed with omissions.** F2/F3 are the highest-value workshop comparisons. |
| 3 | 22–27, experiments | [workspace][workspace] | **Retained/consolidated.** rg/fd, variant workspace, domain scratch + README. |
| 4 | 28–35, documentation basics | [constitution][constitution] + [practice][practice] | **Refined/partly omitted.** Canonical links and placement retained; explicit top-level addendum/pandoc wording replaced with general addendum guidance. WIP summarizer becomes an honest toolkit route. |
| 5 | 36–58, wave method/names | [practice][practice] | **Compressed/softened.** Model suffix, 0 round, independence remain; accepted-tip obligation weakened (F1); detailed naming/iteration examples reduced. |
| 6 | 59–72, doc-pass | [practice][practice] | **Compressed with missing cues.** Goal and relationships survive; see F4 for procedure/return-shape differences. |
| 7 | 73–77, discussion | [agents][agents] | **Relocated/retained.** Multiple selections and context before questions. This transfer complicates simple per-heading percentages. |
| 8 | 78–106, writing/OKF | [practice][practice] + [constitution][constitution] | **Compressed/routed.** Honest metadata, canonical links, index/log retained; full YAML example removed from ambient context. Later mandatory session provenance is P, not R. |
| 9 | 107–119, known locations | [workspace][workspace] | **Retained/consolidated.** Archive-first, wiki/src/doc/ff/opt/srv/dl and executable scratch retained. Self-negating llms.txt aside normalized to “if present.” |
| 10 | 120–126, planning | [code][code] | **Retained/qualified.** No estimates, diagrams, domain layouts, multiple groupings; adds actual consumer/data caveat to pre-1.0 breakage. |
| 11 | 127–134, coding/checkpoints | [code][code] + [commits][commits] | **Retained/routed.** Let It Fail qualification, deliberate cleanup, stages, human discussion. Gate-versus-saving qualifier is in depth (F5). Compfuzor-specific cleanup framing generalized. |
| 12 | 135–138, Rust publish | [rust][rust] | **Retained.** Bump (patch default), commit, publish ordering. |
| 13 | 139–149, Rust tools | [rust][rust] | **Retained/condensed.** Library/check/benchmark preferences survive. |
| 14 | 150–153, Figment integration | [rust][rust] | **Revised.** Fixed derive imperative softened; precedence explanation corrected (F10). |
| 15 | 154–157, TS imports | [JS][js] | **Retained.** Explicit `.ts` local imports. |
| 16 | 158–170, npm tools/package edits | [JS][js] + [JS README][js-readme] | **Compressed/qualified.** Tools, pnpm/latest preserved; broad “do not edit package.json” becomes dependency-entry restriction, following existing README wording. Native-project-tooling exception added. |
| 17 | 171–175, async | [JS][js] | **Retained/condensed.** Promise APIs, async fs, deferred await; adds explicit concurrent start wording. |
| 18 | 176–183, build/source | [JS][js] + [JS README][js-readme] | **Retained/routed.** Source `.ts` execution, distribution-only builds, compiler flags; examples move behind route. |
| 19 | 184–187, public members | [JS][js] | **Retained.** Public visibility without stability guarantee. |
| 20 | 188–216, scripts/concurrently | [JS][js] + [JS README][js-readme] | **Compressed/routed.** Namespaces, glob composition, dev preserved. Detailed examples mostly survive in README; old `--names` label explanation and `vici` example not reproduced there. |
| 21 | 217–223, CLI main pattern | [JS][js] + [JS README][js-readme] | **Compressed plus inherited conflict.** Direct-source/realpath/then survive; the linked example contradicts the nonblocking rule (F6). |
| 22 | 224–247, jj/git | [jj][jj] + [commits][commits] | **Compressed/routed.** Main guardrails, dates and DSL remain; log recipes move to depth. Push exception conflicts with that depth (F7). |
| 23 | 248–255, jj-hunk ambient cut | [code][code], [jj][jj], [hunk guide][hunks] | **Second reduction of an already-cut block.** IDs/dry-run remain; inline spec/tool choice move out; save-before-restore cue improves (F8). |
| 24 | 256–265, beads basics | [beads][beads] | **Restored/qualified.** Explicit stable IDs, namespace, dependencies, notes, prime prohibition; export preference gets version qualifier (F9). |
| 25 | 266–274, minimal beads create | [beads][beads] | **Retained.** Explicit `--id` command stays ambient. |
| 26 | 275–299, linked work | [beads][beads] + [beads README][beads-readme] | **Split by depth.** Prefix and edge direction inline; epic/feature creation and ready listing in README. Post-edge inspection/cycle checks are in depth. |
| 27 | 300–315, investigate/close | [beads][beads] + [beads README][beads-readme] | **Mostly retained.** Search/show/claim/append/close remain; sample `--add-label` omitted, not a new ban on labeling. |
| 28 | 316–334, supersede | [beads README][beads-readme] | **Routed/condensed.** Replacement → old edge and close rationale explained; full verbose shell recipe becomes prose. |
| 29 | 335–342, rename | [beads][beads] + [beads README][beads-readme] | **Retained/routed.** Preference inline; exact rename form and reference behavior in depth. |
| 30 | 343–364, broad DB recovery | [beads README][beads-readme] | **Deliberately retired as current recipe.** Historical command retained in old file; version-aware Dolt recovery route replaces it. Requires a separate technical judgment, not “compression.” |
| 31 | 365–368, Web Components | [JS][js] | **Relocated/retained.** wc-toolkit/JSDoc/CEM generation remains explicit. |
| 32 | 369–371, shell environment | [workspace][workspace] | **Relocated/retained.** Per-tool-execution environment lifetime explicit. |

<a id="audit-limits"></a>
## Why the coverage audit does not settle this review

At T the [coverage declaration](https://github.com/rektide/rekon/blob/3e99a5ad102531dbfaa98b572eb09f80fd592242/tool/assembly/coverage.json)
requires 108 literal excerpts under 13 source headings. It catches missing
selection and deleted chosen instructions, including beads IDs. It does not
compare every old sentence with the new text or validate the destination docs.

Three concrete limits exposed here:

1. The assertions do not encode the accepted-tip obligation or preservation of
   meaningful differences. Their weakening/omission still passes.
2. They do not inspect the JS example or conflicting push wording in linked
   support documents. A valid pointer can lead to contradictory advice.
3. [The pinned T auditor](https://github.com/rektide/rekon/blob/3e99a5ad102531dbfaa98b572eb09f80fd592242/tool/rust/src/coverage.rs)
   collects Markdown text/code events and ignores raw HTML events. That excludes
   comments, but can also exclude meaningful content in definition-list markup.
   Future element-based assertions need a shared parser that understands that
   distinction; the existing ledger does not currently assert glossary terms.

Accordingly, preserve three separate claims: **the output is freshly generated;
declared excerpts are covered; reviewed meaning is faithfully retained.** The
first two are mechanized today at T; this report advances the third without
pretending to complete an exhaustive proof. Later Rust markup work was in
progress during this review and must be evaluated on its own committed evidence.

<a id="next-review"></a>
## Suggested decisions for the next human review

| Priority | Read together | Decision to make |
| --- | --- | --- |
| 1 | Stewardship workshop commits + [paired collaboration][pair-collaboration] | Which exact distinctions should be restored verbatim rather than summarized again? |
| 1 | [Wave/doc-pass before and after][pair-documentation] | Restore accepted-tip strength; choose an actual full home for the operational doc-pass. |
| 2 | [Glossary][pair-glossary] + constitution and practical guidance | Is this attention allocation helpful? Keep, reorganize, or selectively route based on real tasks, not raw size alone. |
| 2 | JS CLI and commits guide versus their GLOBALs | Resolve inherited contradictions before relying on those routes. |
| 2 | [Already-cut hunk block][pair-code] and e148 history | Decide whether spec shape/tool-choice cues should remain ambient. |
| 3 | [Beads][pair-beads] + current-version storage evidence | Keep ID practice fixed; separately judge command depth and recovery modernization. |
| 3 | [Rust][pair-rust] + installed Figment2 API | Separate example correctness from relaxing derive policy. |
| 4 | [Workspace][pair-workspace] | Spot-check the consolidation; no comparably significant semantic loss found in the inspected text. |

This report proposes those decisions; it does not silently amend the workshopped
guidance while measuring it. The user can now choose a cohort, open its source
pair, inspect its originating commit, and review a correction in isolation.

<a id="cross-references"></a>
## Cross-references

- [Recovery assessment](/design/context-compilation/repair0.gpt6a.md) explains the
  original activation failure and what the recovery actually implemented. This
  report adds fidelity and instruction-strength analysis after the user's correction.
- [Evidence bundle](/design/context-compilation/review0/README.md) supplies the
  reproducible measurements, 371-line origin map, and exact transition patches.
- [Earlier inventory](/design/rekon/context-weave/inventory0.gpt56s.md) supplies
  the fine-grained ownership model and the warning that moving ownership does not
  authorize reducing activation. This report measures where actual work diverged.
- [Rust combine handoff](/design/toolchain/handoff0.gpt6a.md) owns the next
  implementation direction. The semantic review can inform its input contract;
  it is not a replacement for finishing the combiner.

[legacy]: https://github.com/rektide/rekon/blob/ab672848a82170a5575dc32c227a5056008e3f37/AGENTS.md
[agents]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/agents/GLOBAL.md#L8-L47
[practice]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/constitution/practice/GLOBAL.md#L8-L54
[constitution]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/constitution/GLOBAL.md
[workspace]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/code/workspace/GLOBAL.md
[code]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/code/GLOBAL.md
[jj]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/vcs/jj/GLOBAL.md
[js]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/js/GLOBAL.md
[rust]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/code/rust/GLOBAL.md
[beads]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/praxis/beads/GLOBAL.md
[beads-readme]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/praxis/beads/README.md
[commits]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/code/commits.md
[hunks]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/code/jj-hunk.md
[js-readme]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/js/README.md
[js-cli]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/js/README.md#L158-L178
[glossary]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/CONTEXT.md
[wave-before]: https://github.com/rektide/rekon/blob/ab672848a82170a5575dc32c227a5056008e3f37/AGENTS.md#L36-L58
[wave-after]: https://github.com/rektide/rekon/blob/00cc45a247f26b7598864cd76a2b621f0927b107/constitution/practice/GLOBAL.md#L24-L41
[stewardship-before]: https://github.com/rektide/rekon/blob/ab672848a82170a5575dc32c227a5056008e3f37/AGENTS.md#L13-L21
[docpass-before]: https://github.com/rektide/rekon/blob/ab672848a82170a5575dc32c227a5056008e3f37/AGENTS.md#L59-L72
[push-before]: https://github.com/rektide/rekon/blob/ab672848a82170a5575dc32c227a5056008e3f37/AGENTS.md#L224-L247
[beads-before]: https://github.com/rektide/rekon/blob/ab672848a82170a5575dc32c227a5056008e3f37/AGENTS.md#L256-L265
[rust-before]: https://github.com/rektide/rekon/blob/ab672848a82170a5575dc32c227a5056008e3f37/AGENTS.md#L150-L153
[pairs]: /design/context-compilation/review0/evidence0.gpt6a.md
[pair-collaboration]: /design/context-compilation/review0/evidence0.gpt6a.md#collaboration
[pair-documentation]: /design/context-compilation/review0/evidence0.gpt6a.md#documentation
[pair-workspace]: /design/context-compilation/review0/evidence0.gpt6a.md#workspace
[pair-code]: /design/context-compilation/review0/evidence0.gpt6a.md#code-jj
[pair-js]: /design/context-compilation/review0/evidence0.gpt6a.md#javascript
[pair-rust]: /design/context-compilation/review0/evidence0.gpt6a.md#rust
[pair-beads]: /design/context-compilation/review0/evidence0.gpt6a.md#beads
[pair-glossary]: /design/context-compilation/review0/evidence0.gpt6a.md#glossary
[pair-tickets]: /design/context-compilation/review0/evidence0.gpt6a.md#ticket-pointers
[pair-refs]: /design/context-compilation/review0/evidence0.gpt6a.md#references
