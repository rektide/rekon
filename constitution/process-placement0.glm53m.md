---
type: Research
title: Process placement — where the constitutional threads live now
description: Maps each process idea from the September constitutional exploration (threads0, reassessment0, work-map0, lineage-reassessment0, and the retrospective README candidate) to its specific present-day home in Rekon — what landed, what remains a gap, and which owner holds each piece.
resource: /constitution/process-placement0.glm53m.md
tags: [constitution, threads, placement, praxis, context, coverage, fronts, glossary]
status: draft
generated: { by: "model:zai-coding-plan/glm-5.3-max", at: 2026-10-11T18:30:00-04:00 }
extensions:
  wave: process placement 0
  commissioned_by: "parent session ses_f910e26f3ffeAerti8u4nZci16 (Draft broader constitution README after recent work, fork #1)"
  scope: placement research only; no tickets created or modified; no existing documents edited
sources:
  - resource: "urn:opencode:session:ses_ed304ae3effekKMRHBDXOtD7cr"
    title: This placement research session (happy-falcon), commissioned by the parent README-candidate session
  - resource: "urn:opencode:session:ses_f910e26f3ffeAerti8u4nZci16"
    title: Parent session direction — map each process idea to its specific home; parent owns the README candidate
  - { resource: /constitution/threads0.glm53m.md, title: Constitutional threads research (the ideas being placed) }
  - { resource: /constitution/reassessment0.gpt6a.md, title: Coming up for air (the September organizing reframe) }
  - { resource: /constitution/work-map0.gpt6a.md, title: Existing commitments and hypothetical connections }
  - { resource: /design/doc-constitution/lineage-reassessment0.glm53m.md, title: Predecessor contract reassessment (thinking-in-Markdown and identity fragments) }
  - { resource: /constitution/README-draft0.gpt6a.md, title: The parent's retrospective README candidate, committed as mxlkkmwstplv }
  - { resource: /constitution/index.md, title: Constitutional reading map }
  - { resource: /constitution/structure.md, title: Rekon's working structure (placement contract) }
  - { resource: /CONTEXT.md, title: Working vocabulary (shared glossary, now semantic definition lists) }
  - { resource: /AGENTS.md, title: Generated operating guidance assembled from domain fragments }
  - { resource: /constitution/practice/GLOBAL.md, title: Writing, waves, and doc-pass ambient fragment }
  - { resource: /agents/GLOBAL.md, title: Collaboration and architectural stewardship fragment }
  - { resource: /code/README.md, title: Coding practice home (commits, review, planning, posture) }
  - { resource: /code/GLOBAL.md, title: Code stage-and-change ambient fragment }
  - { resource: /praxis/README.md, title: Praxis home (chase the future and related practices) }
  - { resource: /praxis/fronts/README.md, title: Fronts and horizons, hub of the fronts thread }
  - { resource: /praxis/parallax/README.md, title: Parallax — independent views and parent synthesis }
  - { resource: /praxis/candidates/README.md, title: Candidates — fair hearing of possibilities }
  - { resource: /praxis/bookmarking/README.md, title: Bookmarking practice and cotail engine }
  - { resource: /digest/README.md, title: Digest — present state and intent fronts }
  - { resource: /assess/README.md, title: Assessment home }
  - { resource: /assess/gates/README.md, title: Evidence gates }
  - { resource: /remake/README.md, title: Remake — defensible build-and-landing order }
  - { resource: /evidence/README.md, title: Evidence and case stories }
  - { resource: /refs/skills.md, title: Skill curation and composition }
  - { resource: /refs/sources.md, title: Sources practice, including required session provenance }
  - { resource: /tool/assembly/coverage.md, title: Operating-context coverage contract }
  - { resource: /design/context-compilation/repair0.gpt6a.md, title: The October 11 coverage diagnosis and repair }
  - { resource: /design/context-compilation/research0.glm53fx.md, title: Prior structure/tools/paths research for the cutover }
  - { resource: /design/toolchain/handoff0.gpt6a.md, title: Rust combine handoff }
  - { resource: /design/toolchain/enriched-assembly0.gpt6a.md, title: Enriched whole-document assembly commission }
  - { resource: /design/feature-lines/init0.glm53f.md, title: Glossary surfaces and vcs vocabulary research }
  - { resource: /design/rekon/context-weave/README.md, title: Context weave workshop }
  - { resource: /design/session-mementos/README.md, title: Session mementos design }
  - { resource: /README.md, title: Root project overview }
  - { resource: /okf/README.md, title: Open Knowledge Format home }
---

<a id="pp"></a>
# Process Placement — Where The Constitutional Threads Live Now

<a id="pp-what"></a>
## What is up

The September constitutional exploration recovered a set of process ideas —
chief among them *thinking and talking in Markdown* and *intentional commits as
thinking-out-loud* — and deliberately deferred the broader README candidate
until the thread and lineage research landed
([threads0](/constitution/threads0.glm53m.md),
[reassessment0](/constitution/reassessment0.gpt6a.md),
[work-map0](/constitution/work-map0.gpt6a.md),
[lineage-reassessment0](/design/doc-constitution/lineage-reassessment0.glm53m.md)).
The work then moved on for five weeks: the AGENTS cutover, the praxis family,
the shared glossary, the coverage audit, and the Rust toolchain direction all
landed. Today the parent session captured the deferred candidate as
[README-draft0.gpt6a.md](/constitution/README-draft0.gpt6a.md) (commit
`mxlkkmwstplv`), explicitly marked retrospective, unaccepted, and partly
outmoded.

This note is the complementary half of that capture. It takes **each process
idea from the exploration** and states where it *specifically* lives now: the
existing file and section that owns it (or the clearly proposed placement when
none exists), what has landed since September, what remains a genuine gap, and
which owners hold the neighboring pieces. The parent's candidate says its
placement research "should determine which paragraphs are already covered,
which need a reciprocal reference, and which reveal a real gap"
([§Reading this candidate in the present](/constitution/README-draft0.gpt6a.md#today));
the
[candidate reading](#pp-candidate) section below answers exactly that,
paragraph by paragraph.

No new universal module, omnibus constitution, or automatic home is proposed
here. Where a gap is real, the note names the *existing* owner where the
missing piece belongs, and marks every such suggestion as a proposal that
needs its owner's and the human's decision.

<a id="pp-fishing"></a>
## The `#fishing` label — search limits, stated honestly

The commissioning direction spoke of "newer threads0.md follow-up" and "`#fishing`
new work." The follow-up work itself is abundant and locally verifiable (it is
the corpus mapped below). The **`#fishing` label is not**. Searches performed:

- Content search across this repository (including hidden files): no
  `fishing`-tagged artifact; the only occurrences are unrelated (Garmin
  chartplotter keys and a fishing analogy in a fetched web article inside
  saved session parts).
- The live OpenCode session database: the only mention is the user's own
  prompt in the parent session (2026-10-11 22:00 UTC), plus the parent's
  acknowledgment. No session title, slug, or bookmark carries it.
- All three cotail bookmark stores reachable locally
  (`~/.local/share/cotail/bookmarks.db` and the two `tmp-opencode` scratch
  stores): active tags are `bus-smart`, `decision-point`, `crossroads`,
  `review`, `summary`, `design-gate`, `#syn`, `tmux-interface-control` — no
  `fishing`.
- Filesystem and archive directory searches: no `fishing` directory or file.

**Conclusion, flagged as inference:** `#fishing` appears to be the operator's
name for the ongoing thread-recovery effort itself — the September reframe of
"re-finding the less-visible threads" as "a lot of what i see as our role
right now" (parent session prompt, 2026-09-04). Its "new work" is therefore
treated here as the discoverable late-September-through-October corpus:
the praxis family and fronts thread, the glossary and its semantic-definition
conversion, the assembly cutover and coverage repair, the session-provenance
rule, and the enriched-assembly toolchain direction. If `#fishing` names
something outside these systems (a channel or tag in a service not reachable
from here), that work is **not** represented in this map, and this note claims
nothing about it.

<a id="pp-method"></a>
## Method and honesty

Read in full or in relevant part: the five September/October source documents
named in the commission, the parent's committed candidate, and the current
maintained homes they must be placed against — `CONTEXT.md`, the assembled
`AGENTS.md` and every selected fragment, `constitution/structure.md`,
`constitution/index.md`, the `praxis/`, `digest/`, `assess/`, `remake/`,
`evidence/`, `refs/`, `code/` homes, the tool/assembly contracts, and the
October design records (`repair0`, `research0`, `handoff0`,
`enriched-assembly0`). The maintained constitution
([/constitution/README.md](/constitution/README.md)) was read by section for
the placements that touch it; its opt-in pattern table remains the compression
layer the lineage reassessment checked against, and this note re-checks the
same rows against today's state.

Standing distinctions kept throughout: current maintained guidance versus
recorded decision versus proposal versus implementation evidence. Statuses
below say *landed* only where a maintained home now carries the idea;
*partial* where a home carries part of it; *gap* where no home does. Line and
anchor citations refer to the working copy inspected on 2026-10-11; the
working copy contains uncommitted enriched-assembly implementation by another
author, which this note treats as **evidence of direction, not landed work**
(and does not touch).

No tickets were created or modified. Only this file is committed by this
session.

<a id="pp-map"></a>
## Placement table

| # | Process idea (source) | Present home | Status |
|---|---|---|---|
| 1 | Thinking/talking in Markdown (Thread 1) | [constitution/practice/GLOBAL.md](/constitution/practice/GLOBAL.md) (mechanics) + parent candidate §markdown (principle, unaccepted) | mechanics landed; principle statement still unowned |
| 2 | Coherent checkpoints / intentional commits (Thread 9) | [code/GLOBAL.md](/code/GLOBAL.md), [code/README.md §Committing](/code/README.md#code-committing), [code/commits.md](/code/commits.md), [code/review-handling.md](/code/review-handling.md) | landed |
| 3 | Independent inquiry & fair synthesis (Thread 5; lineage A4, D6) | [praxis/parallax](/praxis/parallax/README.md), [praxis/candidates](/praxis/candidates/README.md), [assess/gates](/assess/gates/README.md), practice GLOBAL *Waves* | mostly landed; seals/manifests still opt-in only |
| 4 | Re-finding threads / doc-pass (Thread 1 + weaker obs.) | practice GLOBAL *Doc-pass*, [constitution/index.md](/constitution/index.md), [fronts thread](/praxis/fronts/README.md#fronts-thread) ⇄ convention | landed; validator deliberately unpromoted |
| 5 | Claim/evidence/acceptance/activation (Thread 2 distinctions; reassessment) | [CONTEXT.md](/CONTEXT.md) glossary group, [evidence/README.md](/evidence/README.md), claim classes in constitution | landed as vocabulary; specific claim classes (source-stated vs built) still gap |
| 6 | Identity/reference/continuity/actors (Thread 2 family) | [proposal4 ruling](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling), [CONTEXT.md](/CONTEXT.md), [structure.md](/constitution/structure.md), naming exploration | ruling landed; draft3 grammar still pending |
| 7 | Context delivery and skills (Thread 1 counterpressure; skillify) | [AGENTS.md](/AGENTS.md) assembly + [refs/skills.md](/refs/skills.md) + [tool/assembly/coverage.md](/tool/assembly/coverage.md) | landed and mechanized; skillify contract still open |
| 8 | Navigation / ongoing fronts (reassessment map row) | [fronts thread](/praxis/fronts/README.md#fronts-thread) hub + [digest](/digest/README.md); `rekon-navigation` still open | landed as practice family; Navigation module still unaccepted |
| 9 | Proportional coordination / ceremony budget (Thread 6) | distributed clauses in every praxis home + [gates §Proportionality](/assess/gates/README.md) + beads GLOBAL | landed as posture; no admission test (probably rightly) |
| 10 | Loud over silent (Thread 3) | practiced everywhere; named nowhere — now includes the [coverage audit](/tool/assembly/coverage.md) as its strongest mechanism | still unnamed as principle |
| 11 | Anti-conflation (Thread 4) | [CONTEXT.md](/CONTEXT.md) term discipline, glossary-first rule, fronts three lenses | landed as practice; unnamed |
| 12 | Self-demonstration (Thread 7) | practiced (this corpus demonstrates itself); not required by canon | still uncodified |
| 13 | Sessions/loss/resumption over retention (Thread 8) | [digest](/digest/README.md), [bookmarking](/praxis/bookmarking/README.md), [session-mementos](/design/session-mementos/README.md), [task-reconstruction](/code/task-reconstruction/philosophy.md), [session provenance](/refs/sources.md#session-provenance) | largely landed; recovery-guide genre still unnamed |
| 14 | Wiki-ness / pre-history (Thread 10) | [structure.md](/constitution/structure.md) placement contract; pre-history still unrecorded in lineage | half landed |
| 15 | Ask-accounting & structured questions (Thread 2 m11; weaker obs.) | structured questions landed in [agents/GLOBAL.md](/agents/GLOBAL.md); ask-audit still naming-specific | partial |
| 16 | Predecessor machinery (lineage datapoints D1–D8) | scattered: gates (D1), parallax (D6), sources.md (D4), coverage audit (new species of D8) | mixed; details below |

---

<a id="pp-markdown"></a>
# 1. Thinking and talking in Markdown

**Where it lives now.** The *mechanics* threads0 located in the old monolith
AGENTS.md now have a domain-owned, assembled home: the constitution practice
fragment, [`constitution/practice/GLOBAL.md`](/constitution/practice/GLOBAL.md)
(*Durable writing*, *Waves*, *Doc-pass*), selected into every session's
ambient context since the October 7 cutover. Its wave rules carry the
stage-setting epistemics ("Waves are independent research/design outputs…
Start with the situation, present ask, directions of inquiry, and what changed
since the previous work"), the model-suffix honesty rule, and the
synthesis-over-averaging instruction. The doc-pass section restores the
re-finding discipline verbatim in spirit ("search indexes and the surrounding
corpus by concept, not just exact keywords… add explained Cross-references…
useful reciprocal links, and index updates").

Two adjacent pieces complete the mechanics:

- **Session thoughts are now formally citable.**
  [`refs/sources.md#session-provenance`](/refs/sources.md#session-provenance)
  (2026-10-11) makes the writing session a *required* OKF source: "the
  session preserves the human direction, reasoning, and choices that a
  finished document often compresses away." This is Thread 1's handoff
  sub-distiction made into writing practice — the session is citable
  provenance without being the medium of record.
- **The corpus is demonstrably conversational.** The parent's own README
  candidate, this note, and the enriched-assembly design all answer, cite, and
  revise one another within one working day — the interlocutor pattern
  threads0 traced back to the
  [review-appendix workflow](/prompt/.archive/workflows/review-appendix.md).

**What landed since September.** The AGENTS cutover itself is the largest
single event for this thread: wave/doc-pass/addendum practice stopped being
undocumented-in-the-constitution monolith text and became
[domain-fragment-owned](/constitution/practice/GLOBAL.md) guidance whose
selection, provenance, and byte-cost are visible in the generated
[AGENTS.md](/AGENTS.md) header. The glossary
([CONTEXT.md](/CONTEXT.md)) became the shared sense-map threads0's
"sense-map" affordance wanted, and was converted to semantic definition lists
(commit `f649d5e`) as the first case of the enriched-assembly vocabulary
direction.

**What remains a gap.** The *principle statement* — "writing is where
thinking happens; drafts are prototypes of argument; revision is thought
continuing" — still has no maintained home. The maintained
[constitution README](/constitution/README.md) still carries only its one
gesture ("Documentation here is not inert output," line 101 today; lines
87–89 when threads0 inspected it in September). The parent
candidate's [§Think And Talk In Markdown](/constitution/README-draft0.gpt6a.md#markdown)
states it well but is explicitly unaccepted and retrospective. **Proposed
placement (proposal, not enacted):** if and when the human accepts a broader
compact, the statement belongs in the maintained constitution's
[Governing Principles](/constitution/README.md#doc-constitution-principles)
section (one principle row, citing threads0 Thread 1 and lineage Thread A as
evidence), with the mechanics staying in the practice fragment. It should not
be duplicated into the practice fragment ahead of that acceptance.

---

<a id="pp-checkpoints"></a>
# 2. Coherent checkpoints and intentional commits

**Where it lives now.** This is the most completely landed thread. The
September in-session corrections ("commit as you go," "break up corrections
into individual well argued cohesive commits") are now codified in three
layers:

- Ambient: [`code/GLOBAL.md`](/code/GLOBAL.md) — "commit each coherent stage
  when available, including prompts, research, drafts, and guidance…
  checkpoint is not acceptance," plus the hunk rule and review-comment
  protocol.
- Maintained depth: [`code/README.md` §Committing and §Responding to
  review](/code/README.md#code-committing) ("A checkpoint preserves an
  argument; it does not declare acceptance"), with
  [`code/commits.md`](/code/commits.md) and
  [`code/review-handling.md`](/code/review-handling.md) owning the
  nitty-gritty.
- Vocabulary: [`CONTEXT.md`](/CONTEXT.md) terms *checkpoint*, *acceptance*,
  *verification*, *promotion* now disambiguate exactly the acts threads0
  worried were conflated.

**What landed.** The recovery-repair workflow of October 11 demonstrated the
practice under failure conditions: diagnosis (`492cb7ff`), restoration
(`00cc45a2`), audit (`3e99a5ad`), history note (`56a19bab`) — each a coherent,
argued stage (per [`handoff0`](/design/toolchain/handoff0.gpt6a.md) recovery
commits). The empty-commit license and no-squash/no-edit guardrails are in
[`vcs/jj/GLOBAL.md`](/vcs/jj/GLOBAL.md) and the jj guide.

**What remains a gap.** Small: the *narrativity* claim — commit messages as
forward-looking reasoning aimed at future resumption, not merely an audit
trail (threads0 Thread 9's "History is what commits leave behind; narration
is what they are") — is implied by "a checkpoint preserves an argument" but
never stated as a *why*. **Proposed placement:** one short rationale paragraph
in [`code/commits.md`](/code/commits.md) (the depth doc), citing
[prompt/.archive/workflows/commit.md](/prompt/.archive/workflows/commit.md)
and threads0 Thread 9. Not constitutional material; it is a practice
explanation.

---

<a id="pp-inquiry"></a>
# 3. Independent inquiry and fair synthesis

**Where it lives now.** This idea graduated from "opt-in wave pattern with
unstated epistemics" to **three cooperating maintained homes plus ambient
rules**:

- [`praxis/parallax/README.md`](/praxis/parallax/README.md) owns the method:
  commissioning deliberately different positions, preserving acquisition
  before cooperation ("An independent author writes and commits its own
  model-marked inquiry before reading sibling outputs"), gates that delay
  dependent reading rather than independent work, rate-limit fairness, and a
  five-part **parent synthesis** shape (common ground — including "whether
  agreement comes from distinct evidence or simply the same supplied
  framing" — useful differences, tensions, selection and delta, next action).
- [`praxis/candidates/README.md`](/praxis/candidates/README.md) owns fair
  comparison: "Do not force a winner just to make the answer look finished…
  Agreement between models may reflect shared inputs and framing; the
  [naming override](/constitution/naming/exploration/proposal4.glm53m.md) is
  a concrete reminder that convergence can miss what the human wanted."
- [`assess/gates/README.md`](/assess/gates/README.md) owns the timing
  machinery: complete-wave versus dependency-readiness gates, "receipts, not
  findings," deliberate two-step release (arrival, then readiness).
- Ambient wave rules in
  [`constitution/practice/GLOBAL.md`](/constitution/practice/GLOBAL.md):
  same-wave independence, no cross-reading unless directed, synthesis
  "follows the evidence: integrate common themes and tensions, then compare
  sources' strengths, weaknesses, and distinctive ideas."
- [`agents/GLOBAL.md`](/agents/GLOBAL.md) owns dispatch and return: run
  delegated agents in the background, prime with existing research, preserve
  other authors' files, prefer new files over overwriting independent drafts.

**What landed since September.** Lineage datapoint D6 (fair synthesis vs
consensus counting) — flagged *missing* in September — is now substantively
held by parallax's synthesis checklist and candidates' anti-flattening rules.
The reassessment's "delegation must not crowd out deliberation" correction is
ambient in agents/GLOBAL stewardship and discussion rules.

**What remains a gap.** The heavy coordinator machinery (lineage D5) —
assignment manifests, wave seals, the stop-on-model-mismatch rule — is still
only an opt-in pattern row in the
[constitution](/constitution/README.md#doc-constitution-patterns), exactly as
the lineage reassessment left it, and that now looks *deliberate* rather than
neglected: every practice home that could host it instead carries
proportionality language. The one cheap remainder from the lineage survey —
one sentence in the constitution's promotion section to the effect "model
count is not confidence" — is still unwritten. **Proposed placement:** that
single sentence belongs in
[Promotion Is Synthesis, Not Movement](/constitution/README.md#doc-constitution-lifecycle-promotion)
whenever that section is next revised; the manifest/seal template should wait
for a demonstrated need (the ceremony budget's own test).

---

<a id="pp-docpass"></a>
# 4. Re-finding threads and doc-pass

**Where it lives now.**
[`constitution/practice/GLOBAL.md`](/constitution/practice/GLOBAL.md) *Doc-pass*
is the restored ambient home (the coverage repair's October 11 table lists it
as "Restore operational wave rules, provenance, and corpus linking beside the
current constitution"). The reading map
[`constitution/index.md`](/constitution/index.md) is the maintained index the
work map hypothesized — updated after significant waves (the parent linked
the README candidate into it today), with its "Keeping This Map Useful"
rules owning index honesty: "Link the current account and significant
unresolved alternatives, not every artifact in a long wave."

**What landed beyond threads0's account.** Two developments threads0 could
not have seen:

- The **⇄ bidirectional-link convention**
  ([CONTEXT.md#bidirectional-link](/CONTEXT.md#bidirectional-link), first
  adopted for the [fronts thread](/praxis/fronts/README.md#fronts-thread)) —
  doc-pass's "useful reciprocal links" made into a *visible maintenance duty*
  with a glyph, plus a filed checker desire (`rekon-bidi-link-checker`).
- **Bookmarking as durable re-finding across sessions**
  ([praxis/bookmarking](/praxis/bookmarking/README.md)): cotail's landed
  bookmark engine marks decision-dense junctions, summaries, and review gates
  so the *finding* no longer depends on re-running the search. Session
  mementos ([design/session-mementos](/design/session-mementos/README.md)) is
  the designed consumer of those marks.

**What remains a gap.** Corpus validation mechanics (lineage D8) remain
deliberately unpromoted — the is-tree checker is still scratch, and the
bidi-link checker is a desire, not a tool. This is the lineage's own
restraint working ("promote tooling only after stable need is demonstrated")
and should not be "fixed" by this note. Note, though, that October 11 added a
*new* validation species the lineage did not foresee: the
[operating-context coverage audit](/tool/assembly/coverage.md), which checks
that required instructional excerpts survived assembly — a doc-pass duty
mechanized for the one surface where silent loss was observed.

---

<a id="pp-grounds"></a>
# 5. Claim, evidence, acceptance, activation — and their distinctions

**Where it lives now.** This family landed as *vocabulary plus practice*:

- [`CONTEXT.md`](/CONTEXT.md) owns the distinctions as shared terms:
  *claim*, *evidence*, *provenance*, *case story*, *honing*, *checkpoint*,
  *acceptance*, *promotion*, *activation*, *verification* — each with a
  "do not conflate" boundary. The root [README](/README.md#rekon-sense-map)
  surfaces them above the fold ("Honing → promotion… actual activation is a
  separate fact").
- [`evidence/README.md`](/evidence/README.md) owns the relational account:
  evidence as a source's role under stated conditions, the five honest
  questions (including "Is this observed, reported, source-derived, or
  inferred?" and "What does it not prove?"), and case stories as the
  expected-versus-actual learning loop.
- The maintained constitution still owns the claim classes (Observed,
  Derived, Inference, Recommendation, Hypothesis) in
  [Evidence And Claim Maturity](/constitution/README.md#doc-constitution-references-evidence).
- [`okf/README.md`](/okf/README.md) and
  [`refs/sources.md`](/refs/sources.md) own honest metadata: "Omit `verified`
  until a real check against named sources… never use placeholder
  verification events."

**What landed.** The reassessment's "signals must not inherit larger
promises" connection is now *distributed with mechanisms*, not just stated:

- [coverage.md](/tool/assembly/coverage.md): "Exit 0 means the **declared
  checks** passed. It does not prove semantic equivalence… Never weaken a
  required excerpt just to make the check green."
- [repair0](/design/context-compilation/repair0.gpt6a.md): "Preservation in
  version control or a legacy file did not preserve activation" — the
  acceptance/activation split demonstrated by an actual incident and then
  guarded by a check.
- [digest](/digest/README.md): "Never infer acceptance from freshness,
  confidence from length, or progress from the number of documents produced."
- [fronts failure modes](/praxis/fronts/README.md): "readiness laundering —
  tests imply adoption… a label check implies durable identity."

**What remains a gap.** Lineage datapoint D2's specific extensions — a
*source-stated design vs built behavior* claim class, *inherited-and-unverified*
labeling, and the "code without a passing test is evidence of shape, not
proof of working behavior" aphorism — are still absent from the constitution's
claim-maturity guidance. The spirit is present (chase-future's achievement
ladder; the questions above), the classes are not. **Proposed placement:** the
claim-class extension belongs in the maintained constitution's
[Evidence And Claim Maturity](/constitution/README.md#doc-constitution-references-evidence)
section whenever it is next revised — one row for source-stated-vs-built and
one sentence for inherited-premise labeling — with the aphorism as its
memorable form. This is a revision of an existing owner, not a new module.

---

<a id="pp-identity"></a>
# 6. Identity, reference, continuity, and actors

**Where it lives now.** The family threads0 refused to flatten now has a
layered set of owners, matching lineage Thread B's "different fragments,
different layers" verdict:

- **Law (unchanged):** the
  [naming override](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling)
  — location identity, deliberate rename cost, hats, cross-modal aliases.
- **Working vocabulary:** [`CONTEXT.md`](/CONTEXT.md) — *name, authority,
  reference, alias, hat, role, home, channel, revision binding, sense-map,
  effort, work anchor* — functions as the lightweight "family register"
  threads0's open question 3 wondered about, without registry machinery.
- **Placement contract:** [`constitution/structure.md`](/constitution/structure.md)
  — "Moves change names. Repair current citations and use revision-bound
  references when a reader needs the earlier artifact."
- **Concrete namespace trial:**
  [design/session-mementos/references0.gpt6a.md](/design/session-mementos/references0.gpt6a.md)
  (Cotail key/value bookmark links, cross-project Beads references).
- **Channel-in-practice:** the adoption symlink channel wording in
  [index.md](/constitution/index.md) ("The accepted channel selects this
  artifact; the model-suffixed link names the specific record").

**What landed.** The actor side got its strongest everyday enforcement in the
wave rules ("Never use the harness name as the model or copy another model's
suffix") and in required session provenance
([sources.md](/refs/sources.md#session-provenance)): every knowledge document
now names the actual session that produced it, making "who argued this"
recoverable by rule rather than convention.

**What remains a gap / deliberately open.** Draft3's structured grammar,
actor/hat/role formalization, guarded tools, and `/core/README.md` home remain
**proposals pending human acceptance** — the reassessment already declined to
enact them and nothing since changes that. The open `rekon-con-naming`
decision is the right vehicle (lineage D7's verdict still holds). Migration
machinery likewise waits on that decision. No placement change is proposed
here.

---

<a id="pp-context"></a>
# 7. Context delivery, skills, and ambient guidance

**Where it lives now.** The September state ("implemented but not cut over")
is gone; this idea now has *landed machinery with contracts*:

- **Assembly:** [AGENTS.md](/AGENTS.md) is generated from twelve
  domain-owned fragments selected by [`agents.manifest.json`](/agents.manifest.json);
  producer `node tool/agents.ts` (Effect CLI,
  [tool/assembly/README.md](/tool/assembly/README.md)); cutover recorded
  2026-10-07; the legacy reference retains the binding fragment/manifest
  contract ([constitution/agents-assembly.md](/constitution/agents-assembly.md)).
- **Coverage:** [tool/assembly/coverage.md](/tool/assembly/coverage.md) +
  the Rust `audit-context` command separate *output freshness* from
  *operating coverage* — the mechanized form of "attention is part of
  correctness": "A file merely existing outside the manifest cannot satisfy
  coverage."
- **Skills:** [refs/skills.md](/refs/skills.md) owns selection, composition,
  the KISS GLOBAL contract ("a useful cue plus a direct link is the default"),
  and registration-as-disposable-derivation; the
  [refs/GLOBAL.md](/refs/GLOBAL.md) table is the availability record.
- **Writing contract:** the maintained constitution's
  [GLOBAL.md section](/constitution/README.md#doc-constitution-global) owns
  the cue-plus-link default and scoped honing exception.

**What landed since September.** The October 11 repair is the defining event:
the five-fragment cutover silently dropped beads, collaboration, workspace,
writing/waves, jj, and language conventions from ambient context; the
diagnosis named the failure ("a composition failure, not a byte-rendering
failure"), restored the advice under domain owners, and added the audit so
the class of failure is now *loud*. The enriched-assembly direction
([design/toolchain/enriched-assembly0.gpt6a.md](/design/toolchain/enriched-assembly0.gpt6a.md),
with uncommitted implementation in the working copy as of this writing)
extends delivery further: whole-file composition, scoped OKF metadata,
semantic definition lists (CONTEXT.md already converted), and a ToC pass.

**What remains a gap.** The general skillification contract (`rekon-skillify`,
open) is still a design, not a deployed universal skill system — by record,
not neglect. The Rust producer flip is in progress and unfinished. Activation
tiering (the second transformation of the modularization design) has not
happened beyond manual selection. These stay with their owners
(`rekon-tools-endstate`, `rekon-agents-maintenance`) and need nothing from
this note except the observation that the *coverage* concept they will need
already exists and is tested.

---

<a id="pp-fronts"></a>
# 8. Navigation and ongoing fronts

**Where it lives now.** The reassessment's map row ("Navigation's founding
synthesis… a possibility map and a chosen course are different artifacts")
became a **practice family with a hub** rather than a single module:

- [`praxis/fronts/README.md`](/praxis/fronts/README.md) is the hub: strategic
  directions × tactical moves × payoff horizons, yield questions, the
  reusable prompt, failure modes — and its
  [fronts-thread addendum](/praxis/fronts/README.md#fronts-thread) groups the
  whole map→choose→act→return family with ⇄ maintained links (candidates,
  parallax, chase-future, post-build, high/low, digest, lineage fronts,
  bookmarking, case stories).
- [`digest/README.md`](/digest/README.md) maintains the map's *living
  instance* — intent fronts — between passes ("a digest's intent fronts are
  the map's living instance between passes; the fronts-and-horizons maneuver
  is how that map is restructured").

**What landed.** Crossroads (the junction) and bookmarking (its recovery)
completed the loop with [`CONTEXT.md`](/CONTEXT.md) terms and the cotail
engine. External prior art is held as pointers only (kube-kubeless,
opencoattails, ocyc), respecting owner boundaries.

**What remains open.** There is still **no accepted Navigation module** — the
`rekon-navigation` epic's five children remain open and the index correctly
says "There is not yet an accepted Navigation module to link." The practice
family absorbed the *need*; the type contract and pilots remain future work
if the human still wants them. This note proposes nothing here beyond that
observation.

---

<a id="pp-proportion"></a>
# 9. Proportional coordination and the ceremony budget

**Where it lives now.** Distributed, by design, exactly where decisions about
effort get made:

- Every praxis home carries its own anti-ceremony clause: digest ("More
  agents are not inherently a better digest"), parallax ("not a requirement
  to use several agents for every task"), candidates ("not a mandate to run a
  multi-agent wave for every small decision"), fronts ("not a mandatory
  planning ceremony").
- [`assess/gates` §Proportionality and ownership](/assess/gates/README.md):
  "Gates should make the next decision legible, not create a universal
  scheduler."
- [work-map0](/constitution/work-map0.gpt6a.md): "not one issue per
  philosophical principle" and the five filing questions.
- Ambient: [beads GLOBAL](/praxis/beads/GLOBAL.md) "Ticketing is
  user-shaped… ask when tracking scope is unclear";
  [ticket-pointers](/praxis/ticket-pointers/ticket-pointers0.glm53fx.md)
  "not every edit needs a ticket note"; [agents/GLOBAL](/agents/GLOBAL.md)
  delegation and discussion rules.
- The constitution's [Balanced Ticket Coordination](/constitution/README.md#doc-constitution-tickets)
  proportionality test remains the shared root.

**What landed.** The September→October record itself is the best evidence
the budget is being spent well: the repair restored guidance *under domain
owners* rather than concatenating the legacy file; the enriched-assembly
commission replaced a home-grown directive language with standard elements
after the human's "it's weird having our own system" correction — ceremony
*removed* by recorded direction.

**What remains a gap.** threads0 suggested ceremony "an admission test of its
own" with the ambient-cut tests as model. None exists. **Assessment:** leave
it distributed. A formal admission test would itself be ceremony about
ceremony; the distributed clauses have held for five weeks of heavy use. If
the human ever wants the test, its home would be the maintained
constitution's principles section — proposed only as a possibility, with the
counterpressure recorded.

---

<a id="pp-loud"></a>
# 10. Loud over silent

**Where it lives now.** Still "practiced everywhere, named nowhere" — but the
practice got materially stronger:

- The [coverage audit](/tool/assembly/coverage.md) is loud-breakage doctrine
  as a mechanism: declared omissions reported together, pinned baseline
  ("changed evidence requires deliberate re-review"), and the explicit
  instruction not to weaken checks to make them green.
- The assembler's drift detection (hand-edits to generated output fail
  `--check`) remains; `stale_after` frontmatter is visible across the praxis
  homes; the no-automatic-prune rules ([code/GLOBAL](/code/GLOBAL.md),
  coverage contract "The audit… never rewrites guidance") keep cleanup
  deliberate.
- The doc/ retirement was performed loudly: cull records, citation repair,
  pinned blob URLs for the deleted opencode-session doc
  ([ado-recovery1](/design/rekon/ado-recovery1.glm53m.md)).
- [repair0](/design/context-compilation/repair0.gpt6a.md) turned the silent
  loss into a named, recorded incident rather than quietly re-adding text.

**Gap.** Unnamed as a principle, as in September. The naming ruling's
"loud breakage is the kind a plain-markdown workspace can live with" remains
its law-shaped statement. If the broader compact is ever accepted, this is a
candidate principle row next to thinking-in-Markdown — same placement
proposal as idea 1, same acceptance gate.

---

<a id="pp-unconflate"></a>
# 11. Anti-conflation — split the question before writing the rule

**Where it lives now.** As *practice shape* rather than named method:

- [`CONTEXT.md`](/CONTEXT.md) is anti-conflation as a maintained artifact:
  every term carries its "do not conflate" boundary (hat vs new actor;
  channel vs artifact; bookmark vs session; checkpoint vs acceptance).
- The glossary-first requirement — "The glossary belongs before the
  instructions that use it… A link saying 'see glossary elsewhere' is not the
  whole local element" (human direction recorded in
  [feature-lines init1](/design/feature-lines/init0.glm53f.md) and quoted in
  [context-compilation research0](/design/context-compilation/research0.glm53fx.md))
  — makes the split a *writing* obligation, now visible in every fragment's
  Terms-in-use section.
- The [fronts thread](/praxis/fronts/README.md#fronts-thread) keeps "three
  lenses… worth keeping unconfused" (altitude/horizon/structure);
  [digest](/digest/README.md) keeps observation/interpretation/recommendation
  separable; [remake](/remake/README.md) separates
  historical/learning/construction/assessment orders;
  [task-reconstruction](/code/task-reconstruction/philosophy.md) classifies
  the requested outcome before planning.

**Gap.** Unnamed, and probably best left that way — its own counterpressure
(vocabulary inflation) is recorded in threads0 and nothing in five weeks
contradicts it. No placement proposed.

---

<a id="pp-selfdemo"></a>
# 12. Self-demonstration — the document practices what it proposes

**Where it lives now.** Practiced pervasively: the assembled AGENTS.md
describes its own generation and instructs its regeneration; the coverage
audit checks the guidance that tells agents to run it; CONTEXT.md was the
first semantic-definition conversion; the enriched-assembly design
demonstrates scoped OKF metadata in its own markup; this note carries the
required session-provenance source it describes; the parent's candidate
declares its own retrospective standing in its opening lines.

**Gap.** Still absent from the canon as a *requirement* ("a constitution
whose storage shape contradicts its content is a memo, not a constitution" —
[design/doc-constitution/draft0.glm53m.md](/design/doc-constitution/draft0.glm53m.md)
C1.4, per threads0). If the compact is accepted, this is the cheapest
principle row to add, and it pairs naturally with idea 1's placement
proposal.

---

<a id="pp-resumption"></a>
# 13. Sessions, loss, and resumption — the present account over the transcript

**Where it lives now.** This thread fanned out into exactly the homes
threads0's evidence pointed at:

- **Resumption by distilled re-entry:** [digest](/digest/README.md)
  ("re-steer, resume, or prepare to ship again") and
  [task-reconstruction](/code/task-reconstruction/philosophy.md) (recover the
  intent; test the brief by removing historical proper nouns) — plus the
  living genre: [handoff0.gpt6a.md](/design/toolchain/handoff0.gpt6a.md) is
  a worked recovery brief, and the parent session's "recap this session"
  practice produced exactly the present-tense re-entry account threads0
  described.
- **Durable marks instead of retention:**
  [bookmarking](/praxis/bookmarking/README.md) (engine landed) and
  [session-mementos](/design/session-mementos/README.md) (designed consumer;
  exit epilogues, summary marks).
- **Selective transcript preservation:** pinned blob URLs for deleted docs;
  [.transcript/PLAN-combine.md](/.transcript/PLAN-combine.md) still pairs a
  plan with its completion log; scratch mortality rules live on in
  [code/workspace/GLOBAL.md](/code/workspace/GLOBAL.md) (`.test-agent/`
  gitignored; promote load-bearing findings deliberately).
- **The session as citable provenance:**
  [refs/sources.md#session-provenance](/refs/sources.md#session-provenance)
  now *requires* the session URN in OKF sources — threads0's "the session is
  citable as provenance while being explicitly not the medium of record,"
  promoted from observation to rule.

**Gap.** The *recovery-guide genre* still has no name in the lifecycle
vocabularies (posture, disposition, wave role — none covers "handoff brief").
The genre is thriving in practice. **Proposed placement (weak, marked as
such):** only if recurrence continues, a short term entry in
[`CONTEXT.md`](/CONTEXT.md) (*handoff brief* or *recovery brief*) with the
practice paragraph living in `praxis/` beside bookmarking — not a new module
until a second or third consumer outside toolchain handoffs shows the need.

---

<a id="pp-prehistory"></a>
# 14. Wiki-ness as founding aspiration, and the unrecorded pre-history

**Where it lives now.** The wiki settlement is fully law: the
[naming ruling](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling)
("the tree is the namespace"),
[structure.md](/constitution/structure.md)'s wiki-native placement contract,
[CONTEXT.md](/CONTEXT.md)'s "location and section participate in the name,"
and the ⇄ convention as its newest expression.

**Gap (unchanged since September).** The constitution's
[Lineage](/constitution/README.md#doc-constitution-lineage) section still
begins at the predecessor project constitutions; the workspace's own
pre-constitutional roots — short-names-as-hypermedia, the URI-axioms
two-model wave, the workflow set — remain recorded only in the
[prompt archive's retirement notice](/prompt/.archive/README.md) and in
threads0's own account. **Proposed placement:** a short pre-history
sub-section in the constitution's lineage section, or a history note under
`design/doc-constitution/`, citing the archive and threads0 Thread 10; it is
lineage recording, not authority (threads0's own caution). This remains
worth one small, deliberate pass.

---

<a id="pp-asks"></a>
# 15. Ask-accounting and structured questions to the human

**Structured questions landed.** [agents/GLOBAL.md](/agents/GLOBAL.md)
*Discussion*: "Prefer multiple-choice questions allowing multiple selections
when several directions may matter. Explain the situation before the
questions." [task-reconstruction](/code/task-reconstruction/philosophy.md)
operationalizes the classify-first variant. threads0's two-site observation
is now ambient guidance.

**Ask-accounting has not generalized.** The ask-audit
([ask-audit0](/constitution/naming/exploration/ask-audit0.glm53m.md)) is
still a naming-wave artifact; no workspace-wide "no ask silently disappears"
rule exists. Two partial mitigations landed: required session provenance
makes the *human direction* behind a document recoverable (the audit's
underlying failure mode — asks compressed away by synthesis — is now
partially guarded at the writing layer), and the parent session's own recap
practice shows the accounting being done conversationally. **Assessment:**
wait for a second natural instance before proposing a home; if it recurs,
the natural owner is `assess/` (it is an accountability practice for
commissioned waves), not the constitution.

---

<a id="pp-predecessor"></a>
# 16. Predecessor machinery — the lineage datapoints, re-checked

| Lineage datapoint | September verdict | October state |
|---|---|---|
| D1 dependency-gated dispatch | opt-in row only | **Partial:** [assess/gates](/assess/gates/README.md) dependency-readiness gates + [agents/GLOBAL](/agents/GLOBAL.md) "deliberate integration once prerequisite results are available" hold the substance; the `prerequisite` *knowledge relation* (draft3 gap) still absent, still belongs under open `rekon-con-naming`. |
| D2 claim authority (source-stated vs built; inherited premises) | partial | **Still partial** — see [idea 5](#pp-grounds); proposed home: constitution's claim-maturity section. |
| D3 inherited/value-add/stop layering | adequately absorbed | Unchanged; pattern row + promotion steps. |
| D4 source freeze / revision pinning | partial | **Mostly landed as practice:** [refs/sources.md](/refs/sources.md) "Choose the reference for the job" (canonical URL *with commit*; "keep source-specific findings revision-bound"), pinned blob URLs after deletions, [upstream mirrors](/upstream/README.md) provenance. The heavier "record the inspected revision for every mutable repo" obligation stays opt-in judgment — proportionate. |
| D5 manifests, seals, model-identity stop rule | clearest disconnection | **Unchanged and now visibly deliberate** — see [idea 3](#pp-inquiry). |
| D6 fair synthesis vs consensus counting | missing | **Landed** in parallax + candidates — see [idea 3](#pp-inquiry). |
| D7 cross-repository resource identity | right vehicle exists | Unchanged; `rekon-con-naming` open; draft3 pending acceptance. |
| D8 corpus validation mechanics | unpromoted by design | **Unchanged for link/anchor validation; new species landed** for operating coverage — [coverage.md](/tool/assembly/coverage.md) with `audit-context` in `pnpm check`; `rekon-bidi-link-checker` filed for the ⇄ duty. |

---

<a id="pp-candidate"></a>
# Reading the parent's candidate against today's homes

[README-draft0.gpt6a.md](/constitution/README-draft0.gpt6a.md) §today asks
for exactly this triage. Paragraph by paragraph:

| Candidate section | Verdict | Specifics |
|---|---|---|
| [#markdown Think And Talk In Markdown](/constitution/README-draft0.gpt6a.md#markdown) | **real gap (principle) over landed mechanics** | Mechanics live in [practice GLOBAL](/constitution/practice/GLOBAL.md); the principle statement exists nowhere maintained. If accepted, fold into constitution principles (see [idea 1](#pp-markdown)). |
| [#checkpoints Preserve Coherent Stages Of Thought](/constitution/README-draft0.gpt6a.md#checkpoints) | **already covered** | [code/GLOBAL](/code/GLOBAL.md) + [code/README §Committing](/code/README.md#code-committing) + CONTEXT terms carry it fully; candidate needs only reciprocal reference if accepted. |
| [#relations Recoverable Names And Relationships](/constitution/README-draft0.gpt6a.md#relations) | **covered as vocabulary; grammar still open** | [CONTEXT.md](/CONTEXT.md) + ruling + structure.md; draft3 pending. Candidate correctly defers grammar to naming's own work. |
| [#grounds Claims, Grounds, And Authority](/constitution/README-draft0.gpt6a.md#grounds) | **covered, with one real gap** | CONTEXT terms + [evidence/README](/evidence/README.md) + claim classes; gap = D2 classes (see [idea 5](#pp-grounds)). |
| [#inquiry Let Difference Improve The Inquiry](/constitution/README-draft0.gpt6a.md#inquiry) | **covered** | parallax/candidates/gates/practice-GLOBAL hold it; the one-sentence promotion-section anti-majority line is the only missing crumb. |
| [#homes Maintain A Present Account Without Erasing The Conversation](/constitution/README-draft0.gpt6a.md#homes) | **covered** | constitution promotion/history sections + structure.md placement contract; doc-cull/ado-recovery are the worked precedents. |
| [#attention Make Knowledge Available When It Can Help](/constitution/README-draft0.gpt6a.md#attention) | **covered and mechanized** | assembly + coverage audit + skills curation; the candidate's "changing source ownership and changing when guidance is seen are independently significant" is the two-transformations rule, honored by the cutover-then-repair sequence. |
| [#direction Keep Direction Revisable And Work Concrete](/constitution/README-draft0.gpt6a.md#direction) | **covered as practice family; module still open** | fronts thread + digest + balanced ticket coordination; `rekon-navigation` remains open by record. |
| [#learning Return Experience To The Practice](/constitution/README-draft0.gpt6a.md#learning) | **covered** | case stories ([evidence/README](/evidence/README.md#evidence-case-stories)) + doc-pass + fronts §Return from use; the "re-finding threads is itself maintenance" sentence is practice-GLOBAL *Doc-pass* verbatim in spirit. |
| [#measure Let The Work Earn Its Machinery](/constitution/README-draft0.gpt6a.md#measure) | **covered as distributed posture** | see [idea 9](#pp-proportion); the only uncovered piece is the (probably rightly missing) formal admission test. |

Net: of the candidate's ten commitments, **seven are already owned**, one
(identity grammar) is correctly left open, and two — the thinking-in-Markdown
*principle* and the loud-over-silent *principle* — are the genuine residual
gaps, both of which are statements about why the machinery exists rather
than new machinery. That is a strong argument for the candidate's own claim
that it is "a statement of relationships, not a fresh implementation plan."

---

<a id="pp-gaps"></a>
# Where the remaining gaps belong — consolidated proposals

All items below are **proposals needing their owner's and the human's
decision**; none is enacted by this note.

1. **Thinking-in-Markdown principle statement** → maintained
   [constitution principles](/constitution/README.md#doc-constitution-principles),
   one row, on acceptance of a broader compact (evidence: threads0 Thread 1,
   lineage Thread A, candidate §markdown).
2. **Loud-over-silent principle statement** → same section, same gate
   (evidence: threads0 Thread 3; the coverage audit as its strongest modern
   mechanism).
3. **Self-demonstration requirement** → same section, cheapest row
   (evidence: founding draft C1.4 via threads0 Thread 7; five weeks of lived
   practice).
4. **Source-stated-vs-built claim class + inherited-premise labeling** →
   [Evidence And Claim Maturity](/constitution/README.md#doc-constitution-references-evidence)
   on next revision (lineage D2).
5. **"Model count is not confidence" sentence** →
   [promotion section](/constitution/README.md#doc-constitution-lifecycle-promotion)
   on next revision (lineage D6 remainder).
6. **Commit-narration rationale paragraph** →
   [code/commits.md](/code/commits.md) (threads0 Thread 9).
7. **Pre-history lineage note** → constitution lineage section or
   `design/doc-constitution/` history note (threads0 Thread 10, open q5).
8. **(Weak, deferred) recovery-brief/handoff-genre term** → CONTEXT.md +
   praxis neighbor of bookmarking, only on recurrence (threads0 weaker obs.).
9. **(Deferred) ask-accounting generalization** → assess/, only on a second
   natural instance.

Explicitly *not* proposed: any new module, registry, seal requirement,
navigation schema, or universal skill contract; any edit to the naming
grammar questions; any change to the deliberately unpromoted validator
posture.

---

<a id="pp-cross"></a>
# Cross-references

- [README-draft0.gpt6a.md](/constitution/README-draft0.gpt6a.md) — the
  parent's retrospective candidate this map serves; owns the *argument*
  relationships, while this note owns the *placement* facts.
- [threads0](/constitution/threads0.glm53m.md) — the September evidence base
  for every idea placed here; its "coverage vs gap" columns are the
  before-picture.
- [reassessment0](/constitution/reassessment0.gpt6a.md) and
  [work-map0](/constitution/work-map0.gpt6a.md) — the organizing reframe and
  the hypotheticals whose fate is recorded above (index upkeep became real;
  the compact candidate is today's retrospective draft; claim/acceptance
  disambiguation largely landed as vocabulary; the learning loop lives as
  case stories; projection alignment was performed de facto by the
  cutover/repair sequence).
- [structure.md](/constitution/structure.md) — the placement contract this
  note's proposals must respect.
- [index.md](/constitution/index.md) — the maintained reading surface; the
  parent links the candidate there, and this note expects the same treatment
  if the human directs it (this session did not edit the index).
- [tool/assembly/coverage.md](/tool/assembly/coverage.md) and
  [repair0](/design/context-compilation/repair0.gpt6a.md) — the incident and
  mechanism that most changed threads0's Threads 3, 7, and the
  acceptance/activation landscape.
- [praxis/fronts README](/praxis/fronts/README.md#fronts-thread) — the hub of
  the largest single follow-up family (navigation/fronts).
- [refs/sources.md#session-provenance](/refs/sources.md#session-provenance) —
  the newest landed rule answering threads0's session-provenance and
  ask-accounting pressures.
