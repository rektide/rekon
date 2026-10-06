---
type: ResearchWave
title: Constitutional threads - thinking and talking in Markdown, and other latent candidates
description: Recovers the thinking-and-talking-in-Markdown thread from the rekon corpus and surveys additional recurring concerns (loudness over silence, anti-conflation, independence as evidence, ceremony budget, self-demonstration, resumption over transcript retention, commits as narration, wiki-ness) that a broader core constitution should consider. Use when reassessing constitutional scope beyond naming.
resource: /constitution/threads0.glm53m.md
tags: [constitution, threads, thinking-in-markdown, referentiality, values, reassessment, waves]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-09-04T19:50:00-04:00 }
extensions:
  wave: constitution threads 0
  commissioned_by: human:rektide
  related_decision_handles: rekon-doc-constitution-hierarchy, rekon-con-naming
sources:
  - id: constitution
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution
    author: model:gpt-5.6-terra
  - id: constitution-index
    resource: /constitution/index.md
    title: Constitutional reading map
    author: model:gpt-5.6-terra
  - id: agents
    resource: /AGENTS.md
    title: Workspace agent guidance (waves, doc-pass, scratch, commits)
    author: human:rektide
  - id: glm-draft
    resource: /design/doc-constitution/draft0.glm53m.md
    title: Workspace knowledge-module constitution (independent draft)
    author: model:glm-5.3-max
  - id: sol-draft
    resource: /design/doc-constitution/draft0.sol56m.md
    title: Workspace document constitution vision
    author: model:gpt-5.6-sol-medium
  - id: adoption1
    resource: /design/doc-constitution/adoption1.glm53m.md
    title: Adoption revision 1 decisions
    author: model:glm-5.3-max
  - id: prompt-archive-readme
    resource: /prompt/.archive/README.md
    title: Retired prompt archive inventory
    author: human:rektide
  - id: workflow-draft
    resource: /prompt/.archive/workflows/draft.md
    title: draft-the-plan workflow with plan philosophy and short-names
    author: human:rektide
  - id: workflow-discovery
    resource: /prompt/.archive/workflows/discovery.md
    title: Discovery journal workflow
    author: human:rektide
  - id: workflow-review-appendix
    resource: /prompt/.archive/workflows/review-appendix.md
    title: Self-review appendix workflow
    author: human:rektide
  - id: workflow-commit
    resource: /prompt/.archive/workflows/commit.md
    title: Verbose-commit workflow one-liner
    author: human:rektide
  - id: hierarchy1
    resource: /constitution/README-heirarchy1.glm53m.md
    title: Topic-first hierarchy with README presence and frontmatter disposition
    author: model:glm-5.3-max
  - id: naming-readme
    resource: /constitution/naming/README.md
    title: Naming topic README
    author: model:glm-5.3-max
  - id: naming-draft0-glm
    resource: /constitution/naming/exploration/draft0.glm53m.md
    title: Naming draft 0, herald
    author: model:glm-5.3-max
  - id: naming-discussion
    resource: /constitution/naming/exploration/draft2-discussion0.sol56x.md
    title: Draft 2 discussion with full disagreement ledger
    author: model:gpt-5.6-sol-max
  - id: naming-proposal4
    resource: /constitution/naming/exploration/proposal4.glm53m.md
    title: Naming revision 4, the human override
    author: model:glm-5.3-max
  - id: naming-ask-audit
    resource: /constitution/naming/exploration/ask-audit0.glm53m.md
    title: Naming ask audit revision 0
    author: model:glm-5.3-max
  - id: beads-offerings
    resource: /constitution/naming/exploration/beads-offerings0.sol56x.md
    title: Beads system offerings study
    author: model:gpt-5.6-sol-xhigh
  - id: skillify1
    resource: /design/skillify/README1.sol56x.md
    title: Skillification revision 1
    author: model:gpt-5.6-sol-max
  - id: agents-modularization
    resource: /design/rekon/agents-modularization0.sol56x.md
    title: Modularizing rekon's global agent context
    author: model:gpt-5.6-sol-max
  - id: reconstruct
    resource: /code/task-reconstruction/philosophy.md
    title: Reconstruct from the present
    author: model:openai-gpt-5.6-sol
  - id: opencode-session
    resource: /doc/opencode-session.md
    title: opencode session storage, recovery, and Phase A recovery guide
    author: project:rekon
  - id: transcript-pluginification
    resource: https://github.com/rektide/dl/blob/e9c7c401725a93d2c4d56f41851fdb832d772b09/doc/transcript/pluginification.md
    title: Saved pluginification session transcript
    author: project:rekon
  - id: plan-combine
    resource: file:///home/rektide/src/rekon/.transcript/PLAN-combine.md
    title: Plan for combine.sh script with completion log
    author: project:rekon
  - id: global-fragment
    resource: /constitution/GLOBAL.md
    title: Constitution GLOBAL fragment
    author: project:rekon
  - id: design-record-index
    resource: /design/doc-constitution/README.md
    title: Document constitution design record
    author: model:gpt-5.6-terra
---

<a id="threads"></a>

# Constitutional Threads: Thinking And Talking In Markdown, And Other Latent Candidates

<a id="threads-what-is-up"></a>

## What Is Up

The workspace has spent an intense stretch inside one concern: naming. The
naming wave produced drafts, adversarial reviews, syntheses, a human override
ruling ([`proposal4`](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling)),
an ask audit, and a reading map
([`/constitution/index.md`](/constitution/index.md)). The human direction for
this round: come up for air, and co-tug toward *other* principles, values, and
threads — including ones not yet named or codified. Naming is one kernel; the
broader constitution should not be silently identified with it.

One candidate was offered by the human and is treated here as the primary
object of recovery: **thinking / talking in Markdown** — the idea that Markdown
is the place where thinking and exchange *happen*, not merely an archive
produced after reasoning. Threads can be offered, answered, criticized,
linked, revised, resumed. Two more directions frame the survey: intentional
clear commits as we go are themselves part of this thinking-out-loud practice
rather than final housekeeping; and identity/names/referentiality may be a
*family* of several principles that should not be flattened by default.

A compact research prompt for this round, in the wave stage-setting spirit of
[`AGENTS.md`](/AGENTS.md) line 37:

> Trace "thinking and talking in Markdown" through concrete passages of the
> rekon corpus, from the retired prompt system through the current
> constitution. Then find other recurring, under-codified concerns — with
> source references, situations of origin, recurrence across contexts,
> distinctions from neighbors, present constitutional coverage, strongest
> counterpressure, and a provisional classification as value, principle,
> method, or tool choice. Do not propose one universal rule per document.

<a id="threads-method"></a>

## Method, Scope, And Honesty

I read the maintained account
([`/constitution/README.md`](/constitution/README.md)) and
[`/constitution/index.md`](/constitution/index.md) first, then worked through:
the founding drafts
([`draft0.glm53m.md`](/design/doc-constitution/draft0.glm53m.md),
[`draft0.sol56m.md`](/design/doc-constitution/draft0.sol56m.md)), the adoption
record ([`adoption1`](/design/doc-constitution/adoption1.glm53m.md)), the
hierarchy line
([`README-heirarchy1`](/constitution/README-heirarchy1.glm53m.md)), the
capability line
([`skillify`](/design/skillify/README1.sol56x.md),
[`agents-modularization`](/design/rekon/agents-modularization0.sol56x.md)),
the naming exploration (topic README, `draft0.glm53m`, `draft2-discussion0`,
`proposal4`, `ask-audit0`, `beads-offerings0`, `draft3` frontmatter), the
reconstruction neighbors
([`/code/task-reconstruction/philosophy.md`](/code/task-reconstruction/philosophy.md),
[`/doc/opencode-session.md`](/doc/opencode-session.md)), and the historical
prompt archive ([`/prompt/.archive/`](/prompt/.archive/README.md)) with its
workflows — the oldest evidence in the corpus.

Exclusions, per commission: I did not read the current-round sibling notes
(`design/doc-constitution/lineage-reassessment0.glm53m.md`,
`design/doc-constitution/context-practice0.glm53f.md`,
`constitution/reassessment0.gpt6a.md`, `constitution/work-map0.gpt6a.md`), and
I did not research predecessor project constitutions outside rekon (another
agent owns that). One honesty note: while grepping the corpus for the phrase
"thinking," ripgrep surfaced fragmentary preview lines of
`lineage-reassessment0.glm53m.md` in my terminal output. I did not open, read,
or build on that file; everything argued below is grounded in the permitted
sources cited beside each claim. Where a claim rests on thin evidence, I say
so.

No tickets were created or modified. Hypothetical work is described in prose
only, in deference to the commission and to the
[index](/constitution/index.md)'s caution that the work map is "a dated
orientation, not a second work tracker."

Claim maturity in this document, following the constitution's evidence
classes ([`README.md` lines 638-642](/constitution/README.md#doc-constitution-references-evidence)):
quotations and line citations are **Observed**; characterizations of recurrence
are **Derived**; candidate elevations and classifications are **Recommendation**
or **Hypothesis**, marked as such. Line numbers refer to the file states
inspected on 2026-09-04.

---

<a id="threads-markdown"></a>

# Thread 1: Thinking And Talking In Markdown

**Provisional classification: value (with method consequences).** This is the
strongest latent thread in the corpus: older than the constitution, visible in
every era, and never stated as a principle anywhere in the maintained account.

<a id="threads-markdown-origin"></a>

## Where it comes from: the prompt system already believed it

The oldest evidence is not in `constitution/` or `design/` at all. It is the
retired prompt archive, whose README says of `workflows/`: "The most valuable
material here; several techniques live on in evolved form in `AGENTS.md`
practice (design waves, syn reviews, subagent stage-setting)"
([`/prompt/.archive/README.md` lines 17-21](/prompt/.archive/README.md)).

The `draft-the-plan` workflow
([`/prompt/.archive/workflows/draft.md`](/prompt/.archive/workflows/draft.md))
states the thread almost exactly, in the workspace's own voice:

> "## a good plan is the heart of the work
>
> this sub-prompt is a- perhaps THE- heart of the `rekon` prompt system of
> which it is a part. **ideas flow from drafting plans**, and it is where many
> SHORT-NAMES are often begat. drafting a plan is not set in stone, and
> indeed, we want to try to encourage and leave lots of wiggle room, **leave a
> long trail of how we got where we were (a WORK LOG)**, while still
> maintaining overall conceise easy to drop into topics when we build the
> initial DRAFT and then REFINE THE DRAFT." (lines 23-25)

Two companion passages from the same file:

> "## conceptualization not concretization
>
> our goal is not yet to be absolutely sure, uncertainty and more importantly
> possibility is to be regarded & given respect." (lines 15-17)

> "proactively create names for things. this gives the developer and the LLM
> stable identifiers to talk to." (line 100)

The discovery workflow makes the *talking* side concrete — the document is a
journal written during the investigation, not after it:

> "as you go, collect observations and findings in a narrative fashion, by
> creating markdown headers marked `# journal - <subtopic>` to talk to
> sub-topics of research, investigation or findings you explored. **keep a log
> as you go** of what you've done by adding these journal sections." —
> [`/prompt/.archive/workflows/discovery.md` line 16](/prompt/.archive/workflows/discovery.md)

The review-appendix workflow is the ancestor of today's `syn` waves, and it is
explicitly a *conversation between documents* — answer, criticize, revise:

> "append to your plan (the first file attached) with a `# appendix -
> self-review after reading [other files]`.
>
> **do not be concise.** this appendix should serve as a good standalone
> document that is informative and helpful, that considers alternate
> possibilities / ways to get wins, explorative, even when it is making
> recommendations. **we want to expand our picture of the problem through this
> appendix.**
>
> talk about how you might update your plan, based off what other reviews /
> plans you see." —
> [`/prompt/.archive/workflows/review-appendix.md` lines 3-7](/prompt/.archive/workflows/review-appendix.md)

And the one-line commit workflow is the ancestor of Thread 9 (commits as
thinking): "jj commit your changes with a verbose justification / exposition on
your changes"
([`/prompt/.archive/workflows/commit.md` line 1](/prompt/.archive/workflows/commit.md)).

**Situation of origin (Observed):** a solo human + LLM workspace that plans in
Markdown, wants wiggle room and a visible trail while thinking is still warm,
and treats peer documents as interlocutors. Nothing here is about publishing
or archiving; it is about *doing the thinking in the file*.

<a id="threads-markdown-constitution"></a>

## How the constitution inherited it — and demoted it to mechanism

The founding drafts both carry the thread into the constitution era, in their
opening moves:

- `draft0.sol56m.md`: "It is a **self-explaining knowledge flow**. A question
  can be explored, argued over, stabilized, loaded as an on-demand skill, and
  selectively made ambient without being rewritten into four unrelated forms.
  **Markdown documents remain the durable substrate throughout.**"
  ([lines 68-72](/design/doc-constitution/draft0.sol56m.md#doc-constitution)).
  And: "`design/` and `doc/` are not merely stages in a publishing pipeline"
  (line 94).
- `draft0.glm53m.md`: `design/` as "the laboratory... Waves of independent
  model drafts, adversarial and synthesis rounds, **prototypes of argument**.
  High churn is healthy here; wave files are immutable evidence once written —
  refinement appends, rounds are new files."
  ([C3.1, lines 199-203](/design/doc-constitution/draft0.glm53m.md#c-lifecycle)).
- The canonical constitution's own second paragraph:
  "Documentation here is not inert output. It is the durable substrate through
  which a workspace learns, explains itself, activates specialized behavior,
  and preserves enough history to learn again."
  ([`/constitution/README.md` lines 87-89](/constitution/README.md#doc-constitution)).

But note what happens as the constitution matures: the *thinking* verbs
(explore, argue, revise, learn) migrate into lifecycle postures
(`design/` vs `doc/`), exposure channels (README/SKILL/GLOBAL), and opt-in
patterns. The maintained account's compact promise — "Edit locally. Assemble
globally. Load selectively. Cite durably. Coordinate deliberately."
([lines 100-101](/constitution/README.md#doc-constitution)) — is entirely
about *products and delivery*. Wave mechanics survive only as an opt-in
pattern row: "Independent model-suffixed waves — useful disagreement or
independent coverage matters"
([line 791](/constitution/README.md#doc-constitution-patterns)). The living
practice — stage-setting, journaling, addenda, doc-pass — stayed in
[`AGENTS.md`](/AGENTS.md) as undocumented-in-the-constitution operational
guidance.

The strongest *current* statements of the thread are in AGENTS.md, not the
constitution:

- Wave stage-setting: "start waves by capturing what is up... if there are
  specific vectors/directions/instructions your user provided, please capture
  them... talking about what's new or whats changed or what's emerged from the
  previous is helpful stage setting!" (line 37).
- Documents in deliberative sequence: "i tend to prefer documents get built
  deliberatively, given the benefit of previous context that could become
  available that could help latter documents to be better structured."
  (line 4).
- Citable thoughts: "have agents write/maintain their own md files often in
  design/<topic>/ **so their work can be cited**." (line 7).
- Evolution inside one artifact: the addendum — "a new top level h1 section at
  the end of a markdown that packages new information / in the document, but
  not inline" (line 22).
- The corpus as interlocutor: the whole `doc-pass` skill
  (lines 48-60), including "Add reciprocal links to older documents when the
  new work materially changes how they should be navigated or understood"
  (line 56) and "A doc-pass may use a subagent to independently search the
  corpus" (line 60).

The naming wave went furthest toward making documents *perform* rather than
report. Its draft0 opens: "This document practices what it explores, so I have
done the thing under study first: **I name myself `herald`**"
([`draft0.glm53m.md` lines 88-89](/constitution/naming/exploration/draft0.glm53m.md)),
and the override ruling's author section — "Practicing the system under
repair, I name myself **`steward/glm-5.3-max`**" with declared hats
([`proposal4.glm53m.md` lines 58-69](/constitution/naming/exploration/proposal4.glm53m.md#who-is-writing-this)) —
is the same move at the constitutional scale.

<a id="threads-markdown-affordances"></a>

## What the medium is being used for (Derived)

Reading the passages above as one arc, the affordances the workspace actually
relies on are:

1. **Offering a thought** — a wave draft, a plan, a journal section, staged
   with "what is up" so a reader can enter the situation.
2. **Answering and criticizing** — self-review appendices, adversarial
   reviews, disagreement ledgers, the human override reversing syntheses.
3. **Linking while thinking** — short-names minted mid-thought ("it is where
   many SHORT-NAMES are often begat"), anchors proposed before acceptance.
4. **Revising with a trail** — addenda, new rounds as new files, refinement
   appends, supersession rather than overwrite.
5. **Resuming** — recovery guides, present-tense briefs, ask audits: picking a
   thread back up after loss, sessions, or months.
6. **Narrating change** — commit messages as exposition (Thread 9).

The human's framing — "Markdown is the place thinking and exchange happens,
not merely an archive produced after reasoning" — is therefore not an
innovation being proposed; it is a *recovery* of the workspace's oldest
self-understanding, which the constitution era translated into delivery
mechanics without ever stating the underlying value.

<a id="threads-markdown-distinctions"></a>

## Sub-distinctions the thread must not flatten

Commissioned distinctions, each grounded:

**Referable vs authoritative.** A thought becomes *referable* the moment it is
written with a name: "you can cite `herald/glm-5.3-max` and mean me, distinct
from any sibling running the same model on another seat... the cheapest
possible proof that self-naming needs no infrastructure — only a convention
slot to land in"
([`draft0.glm53m.md` lines 363-367](/constitution/naming/exploration/draft0.glm53m.md)).
It becomes *authoritative* only through a different, rarer mechanism — human
acceptance, verification, ruling status: `verified` "MUST mean checked against
sources or the resource, not reread by its author"
([`README-heirarchy1` lines 225-227](/constitution/README-heirarchy1.glm53m.md#doc-constitution-hierarchy1-disposition));
"law (ruling) — settled by human ruling in proposal4" as a distinct status
class ([`ask-audit0` lines 53-59](/constitution/naming/exploration/ask-audit0.glm53m.md#what-this-audits)).
Conflating these two is exactly the failure the index warns about: "Do not
infer acceptance from filename, date, symlink, model agreement, or ticket
closure alone" ([`/constitution/index.md` lines 99-100](/constitution/index.md)).

**Coherent mutable present account vs exact past thought.** The constitution
already holds this pair explicitly: "canonical READMEs are coherent mutable
accounts of what to use now; exact evidence artifacts preserve what an author
argued with the context available then"
([`/constitution/README.md` lines 757-761](/constitution/README.md#doc-constitution-history)).
The naming discussion sharpens it into typed citation intents — exact
artifact, accepted-tip *channel*, maintained README — where "the suffixless
path has a canonical identity, but it is the identity of the **channel**, not
a second identity for every target it selects"
([`draft2-discussion0` lines 327-337](/constitution/naming/exploration/draft2-discussion0.sol56x.md#tip)).
The thinking-in-Markdown thread adds the *process* reading: the present
account is not just a publishing convenience, it is re-thought understanding —
see Thread 8's "reconstruction is not replay."

**Handoff/resumption vs universal transcript retention.** The workspace saves
*selected* transcripts ([pluginification in rektide/dl](https://github.com/rektide/dl/blob/e9c7c401725a93d2c4d56f41851fdb832d772b09/doc/transcript/pluginification.md),
[`/doc/opencode-session.md`](/doc/opencode-session.md)) and mines the session
database under loss, but never treats retention as the default; scratch is
"pre-history" and evaporates unless promoted
([`draft0.glm53m.md` lines 389-395](/design/doc-constitution/draft0.glm53m.md#c-lifecycle)).
Resumption is achieved by *distilled re-entry documents* (recovery guides,
present-tense briefs), not by keeping every word. See Thread 8.

**Authorship/voice vs model/session.** Every durable artifact names its actual
model, never the harness ([`AGENTS.md` lines 38-41](/AGENTS.md)); actors
self-name with addresses kept distinct from names
([`draft0.glm53m.md` "address is not identity," lines 373-387](/constitution/naming/exploration/draft0.glm53m.md));
hats let one voice speak for several concerns
([`proposal4` lines 96-100](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling)).
Voice is part of the thinking medium: who is arguing matters to how the
argument is weighed.

**Doc-pass/re-finding vs links as an undifferentiated graph.** The corpus
practice is *searched, explained, few-link* connection: "Prefer a few
explained references over a long undifferentiated link list"
([`AGENTS.md` line 54](/AGENTS.md)); "A truthful absence of relationship is
better than invented cohesion"
([`/constitution/README.md` lines 625-626](/constitution/README.md#doc-constitution-references-links));
"Inventing a relationship to make the corpus look connected is worse than
recording that no material edge was found"
([`draft0.sol56m.md` lines 339-341](/design/doc-constitution/draft0.sol56m.md#doc-constitution-references-links)).
The doc-pass is a *re-finding* discipline (search by concept, verify against
source, explain relevance), not link agriculture.

<a id="threads-markdown-coverage"></a>

## Present coverage vs gap

Covered today: lifecycle postures, promotion-as-synthesis, history surfaces,
opt-in wave patterns, and AGENTS.md's operational wave/doc-pass/addendum
practice. All of the *mechanics* exist.

The gap is the *principle*: nowhere does the maintained constitution say that
writing is where thinking happens — that drafts are prototypes of argument,
that stage-setting is epistemic, that journals written during investigation
are first-class, that addenda and rounds exist because *revision is thought
continuing*, not because history is precious. The constitution's framing
("Documentation here is not inert output," line 87) gestures at it once and
then never operationalizes it. A broader core constitution that states this
would explain *why* waves, doc-pass, addenda, and commits exist, rather than
listing them as available machinery.

<a id="threads-markdown-counter"></a>

## Strongest counterpressure

1. **Context is a budget.** The constitution's own ambient cut exists because
   not everything can be loaded: "ambient context must not become a live
   transcript of an unsettled design"
   ([`/constitution/README.md` lines 296-298](/constitution/README.md#doc-constitution-lifecycle-design);
   same warning in
   [`draft0.sol56m.md` lines 244-246](/design/doc-constitution/draft0.sol56m.md#doc-constitution-lifecycle-design)).
   Skillify's placement table routes "exploratory, one-off, or
   transcript-like" material explicitly *away* from skill machinery
   ([`README1.sol56x.md` line 161](/design/skillify/README1.sol56x.md#skillify-placement)).
   Thinking-in-Markdown must not become retention-of-everything.
2. **Scratch mortality.** `.test-agent/` is gitignored pre-history; evidence
   that stays load-bearing must be promoted or it evaporates
   ([`draft0.glm53m.md` C7.3](/design/doc-constitution/draft0.glm53m.md#c-lifecycle)).
   The value is not "write everything down," it is "make the thinking that
   matters citable before it cools."
3. **Ceremony budget** (Thread 6): mandating stage-setting or addenda for
   every note would strangle the small-document case — "It should not demand
   research-program ceremony from a two-paragraph user guide"
   ([`draft0.sol56m.md` lines 113-115](/design/doc-constitution/draft0.sol56m.md#doc-constitution-principles)).
4. **Independence requirements** cut against premature linking: same-wave
   authors "are expected to do your own work and not pollute your thinking
   with their results" ([`AGENTS.md` line 46](/AGENTS.md)). The medium is
   conversational, but the conversation has choreography — early isolation,
   late synthesis.

**Recommendation:** elevate as a named principle of the broader constitution,
phrased as a value with proportional practice — something like: *thinking
happens in the files; keep the medium of thought citable while it is still
thinking.* Individual mechanics (waves, addenda, doc-pass) remain opt-in
below it.

---

<a id="threads-family"></a>

# Thread 2: Referentiality And Identity Are A Family, Not A Law

**Provisional classification: a family of principles (value + method
members) — do not flatten.** The commission explicitly warns against
flattening this one by default, and the corpus agrees with the warning: every
time the wave tried to compress identity into a single rule, the disagreement
ledger grew. The current ledger runs to fifty numbered disagreements
([`draft2-discussion0` lines 405-456](/constitution/naming/exploration/draft2-discussion0.sol56x.md#disagreements)).

The family, as the corpus actually splits it:

1. **Referability** (make thoughts citable): anchors, short-names, exact
   citations. Founder evidence: short-names are "a hyperlink internal to the
   textual world of this project... we want to be increasing the hypermedia
   referentiality of the rekon system"
   ([`draft.md` line 96](/prompt/.archive/workflows/draft.md)). Current law:
   semantic anchors with compatibility obligations
   ([`/constitution/README.md` lines 494-518](/constitution/README.md#doc-constitution-references-anchors)).
2. **Ownership/authority** (who may mint): "At canonical tips, one qualified
   ID has one semantic owner"
   ([line 577-580](/constitution/README.md#doc-constitution-namespace-ownership));
   "Canonical authority URI; `rekon` is local elision" (ledger #5).
3. **Channel vs artifact** (accepted tips): the suffixless symlink is "the
   mutable accepted-adoption channel," not a second identity for its target
   ([`draft2-discussion0` lines 322-329](/constitution/naming/exploration/draft2-discussion0.sol56x.md#tip)).
4. **Citation intents** (current / exact / channel / maintained): the typed
   citation rule table
   ([lines 331-337](/constitution/naming/exploration/draft2-discussion0.sol56x.md#tip));
   ledger #24-26.
5. **Address vs name**: session IDs and routing strings are "exact-layer
   handles doing semantic-layer work" — the confusion named
   `naming-address-vs-identity`
   ([`draft0.glm53m.md` lines 373-387](/constitution/naming/exploration/draft0.glm53m.md)).
6. **Voice/actor naming**: model suffixes, self-names, hats
   ([`proposal4` lines 96-100](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling));
   promise/currency split for self-naming (ledger, ask-audit line 69).
7. **Cross-modal aliasing**: `rekon-con-naming` living inside a beads ID is
   "the endorsed practice of cross-modal sympathy"
   ([`proposal4` lines 101-108](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling)).
8. **Plane separation**: beads is the work/coordination plane, documents the
   knowledge plane, "without pretending that a ticket and the knowledge it
   commissions are always the same entity" (Derived,
   [`beads-offerings0` lines 124-128](/constitution/naming/exploration/beads-offerings0.sol56x.md#beads-offerings-answer)).
9. **Migration/compatibility**: inbound inventories, compatibility stubs,
   tombstones ([`README-heirarchy1` lines 355-366](/constitution/README-heirarchy1.glm53m.md#doc-constitution-hierarchy1-migration));
   ledger #32-35, #37-40.
10. **Ruling genre**: human direction recorded as law distinct from proposal
    or synthesis ([`proposal4`](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling);
    [`adoption1`](/design/doc-constitution/adoption1.glm53m.md)).
11. **Ask accounting**: "Every ask the human has made across six direction
    rounds, what the three syntheses did to it, and where it stands now...
    **no ask silently disappears again**"
    ([`ask-audit0` lines 48-51](/constitution/naming/exploration/ask-audit0.glm53m.md#what-this-audits)).

**Situation of origin:** each member crystallized under a different pressure —
anchor rot under rewording, duplicate minting during waves, ticket/doc
coupling, actor churn across sessions, human asks lost in synthesis. That is
precisely why one law keeps failing: the pressures are independent.

**Coverage vs gap:** the naming wave holds members 2-11 in unsettled
proposals; the maintained constitution covers members 1, 2, and parts of 9.
The *family shape itself* — that identity is several coordinated principles —
is the finding this round contributes; the broader core should present it as
a family with a map, not seek one grammar to rule them.

**Counterpressure:** the ceremony budget again — fifty ledger rows is itself
evidence of how expensive full distinction-holding is; the ruling explicitly
priced "ceremony-free uniqueness" as an argument for location identity
([`proposal4` lines 124-126](/constitution/naming/exploration/proposal4.glm53m.md#the-case-for-location-identity)).

**Hypothesis worth testing:** some members (referability, ownership,
migration) are constitutional; others (citation-intent typing, alias
registers) may be tool contracts; others (actor naming, hats) are still
exploration. The broader constitution should say *which kind* each member is
rather than constitutionalizing all of them.

---

<a id="threads-loud"></a>

# Thread 3: Loud Over Silent — Visibility As A Change-Mode Value

**Provisional classification: value.** Scattered through the corpus as
clauses, never named as a principle: *when something breaks or changes, it
should break or change visibly.*

Strongest statement, from the human override ruling on rename ripple:

> "Breakage under this ruling is loud — a stale link is visible rot — and
> **loud breakage is the kind a plain-markdown workspace can live with.
> Silent drift was the enemy**; the duplicate-drift finding stands untouched."
> — [`proposal4` lines 93-95](/constitution/naming/exploration/proposal4.glm53m.md#what-the-rename-ripple-costs)

The same preference recurs as a family of "visible X" clauses:

- visible **cost**: "make per-fragment and total context cost visible"
  ([`/constitution/README.md` line 735](/constitution/README.md#doc-constitution-global-assembly));
  "an invisible ambient budget" as a monolith cost
  ([`draft0.glm53m.md` lines 65-68](/design/doc-constitution/draft0.glm53m.md#c-vision));
- visible **drift**: assembler "detects unassembled hand edits"
  ([line 734](/constitution/README.md#doc-constitution-global-assembly));
- visible **staleness**: "`stale_after` makes review debt visible instead of
  latent" ([`README-heirarchy1` line 223](/constitution/README-heirarchy1.glm53m.md#doc-constitution-hierarchy1-disposition));
- visible **parity**: the migration inventory as "parity ledger... prevents
  attractive new module prose from quietly dropping awkward but important
  guardrails"
  ([`agents-modularization0` lines 252-254](/design/rekon/agents-modularization0.sol56x.md#agents-maintenance-skillify-procedure-inventory));
- loud **non-change**: "Nothing demotes silently: retiring a doc/ topic is
  explicit... never deletion"
  ([`draft0.glm53m.md` lines 238-239](/design/doc-constitution/draft0.glm53m.md#c-lifecycle));
  "Do not break knowledge links merely to mirror work-tracker cleanup"
  ([`/constitution/README.md` lines 584-585](/constitution/README.md#doc-constitution-namespace-ownership));
- deliberate **cleanup**: "Adoption never prunes stale or undeclared
  artifacts" ([line 456](/constitution/README.md#doc-constitution-adoption-proposal));
  "leave cleanup deliberate" ([`AGENTS.md` line 119](/AGENTS.md)); skillify's
  assembler requirements end with "no automatic pruning of undeclared
  artifacts" ([`README1.sol56x.md` line 456](/design/skillify/README1.sol56x.md#skillify-host-global)).

**Situation of origin:** agentic editing at scale — many actors, model churn,
session mortality. When change is constant, the survivable failure mode is the
*noticeable* one. The reconstruct guide states the epistemic version: loud
breakage is checkable, silent drift is not.

**Distinction from neighbors:** not the context budget (that is about cost of
*presence*); not referentiality (that is about *identity*). This is about the
*mode of failure and change*: prefer gates, markers, ledgers, and rot you can
see over automation that quietly heals or quietly loses.

**Coverage vs gap:** every document *practices* it; none *names* it. It is
arguably the implicit justification for half the constitution's machinery
(markers, validation, parity, human gates).

**Counterpressure:** noise fatigue — if everything is loud, nothing is; the
naming ruling itself warns governance is "earned when volume demands it, not
purchased in advance" ([`proposal4` lines 106-108](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling)).

---

<a id="threads-unconflate"></a>

# Thread 4: Anti-Conflation — Split The Question Before Writing The Rule

**Provisional classification: method (with principle backing).** The corpus's
characteristic move when confused is not to compromise; it is to *de-compose
the question* and coin a term for each part.

Cleanest statement, from the naming discussion:

> "A canonical grammar answers seven independent questions: ... Treating
> those as one punctuation question **caused much of the draft2 confusion.
> They should be decided separately.**"
> — [`draft2-discussion0` lines 82-95](/constitution/naming/exploration/draft2-discussion0.sol56x.md#grammar)

The same move recurs everywhere:

- hierarchy revision 1's stage-setting: "What was missing was **vocabulary** —
  for the root document directory, for legacy structures, for disposition,
  for migration moves. This revision adds that terms-of-art layer and
  re-grounds the hierarchy on it"
  ([`README-heirarchy1` lines 52-55](/constitution/README-heirarchy1.glm53m.md#doc-constitution-hierarchy1));
- claim classes: "distinguish Observed, Derived, Inference, Recommendation,
  and Hypothesis where confusing them would change a decision"
  ([`/constitution/README.md` lines 638-642](/constitution/README.md#doc-constitution-references-evidence));
- reference types: "sources ground claims... knowledge links connect...
  tickets connect" (three different jobs,
  [lines 630-637](/constitution/README.md#doc-constitution-references-evidence));
- reconstruction modes: "The word 'reconstruct' is ambiguous. **Classify the
  requested outcome before planning implementation**" with a five-mode table
  ([`/code/task-reconstruction/philosophy.md` lines 97-106](/code/task-reconstruction/philosophy.md#reconstruct-job));
- skillify's delivery-channel split: "Documentation, automation, ambient
  policy, and skills overlap, but they solve different delivery problems"
  ([`README1.sol56x.md` lines 92-94](/design/skillify/README1.sol56x.md#skillify-not));
- the oldest instance: short-names minted "as we go... so we are clear that
  we are referring to the same thing"
  ([`draft.md` lines 19-21](/prompt/.archive/workflows/draft.md)).

**Situation of origin:** multi-model work makes conflation expensive — when
several independent authors share a word that bundles two questions, their
agreements and disagreements become uninterpretable. The disagreement ledger
is the diagnostic instrument: most rows are *separable questions that had been
glued together*.

**Distinction from neighbors:** not naming (Thread 2) — anti-conflation is
what naming is *for*; and not the disagreement practice (Thread 5) — this is
the pre-condition that makes disagreement legible.

**Coverage vs gap:** practiced constantly; stated nowhere in rekon's
constitution. (Predecessor lineage reportedly had "terminology and
anti-conflation rules" in first-compute, per
[`draft0.sol56m.md` lineage table line 580-583](/design/doc-constitution/draft0.sol56m.md#doc-constitution-history-lineage) —
external verification out of scope this round.)

**Counterpressure:** vocabulary inflation — every split mints terms, and
terms are ceremony (Thread 6); skillify names the failure mode "Sediment:
old branches and caveats accumulate because deletion feels risky"
([`README1.sol56x.md` line 684](/design/skillify/README1.sol56x.md#skillify-failures)).
The discipline needs a pairing rule: split only where the distinction would
change a decision (the claim-class wording already models this).

---

<a id="threads-independence"></a>

# Thread 5: Independence Is Evidence; Disagreement Is A Product; Convergence Is Not Authority

**Provisional classification: principle + method (epistemic machinery).**
Under this value, multiple models are not redundant labor — their
independence is the *instrument*, their disagreement is the *data*, and their
convergence proves less than it appears to.

- Independence as evidence: same-wave authors "are expected to do your own
  work and not pollute your thinking with their results; we want independent
  results, that's why we are asking multiple models"
  ([`AGENTS.md` line 46](/AGENTS.md)); "The agents wrote these independently
  and did not read one another's same-wave artifact"
  ([`/design/doc-constitution/README.md` lines 42-43](/design/doc-constitution/README.md)).
- Disagreement as a deliverable: the full disagreement ledger is an artifact
  in its own right — "This is the complete disagreement ledger after the new
  direction"
  ([`draft2-discussion0` lines 400-403](/constitution/naming/exploration/draft2-discussion0.sol56x.md#disagreements)) —
  and the ask-audit records which human asks each synthesis *killed*,
  restoring several by ruling
  ([lines 65-85](/constitution/naming/exploration/ask-audit0.glm53m.md#the-matrix)).
- Convergence is not authority, twice over: "convergence under a shared
  corpus is evidence of a shared reading, **not of correctness**"
  ([`proposal4` lines 114-116](/constitution/naming/exploration/proposal4.glm53m.md#the-case-for-location-identity));
  ledger #49: "Shared reading is evidence, not authority. Human acceptance
  decides"
  ([line 455](/constitution/naming/exploration/draft2-discussion0.sol56x.md#disagreements)).
- Synthesis reconciles rather than averages: "Adversarial and synthesis waves
  — drafts must be challenged and reconciled rather than averaged"
  ([`/constitution/README.md` line 793](/constitution/README.md#doc-constitution-patterns)).
- The rhythm is constitutional: "we ping pong between design... and synthesis
  review. design -> syn -> design2 -> design2-syn"
  ([`AGENTS.md` line 43](/AGENTS.md)).

**Situation of origin:** the wave system itself, plus the naming episode where
three syntheses converged on a position the human then overruled — the
sharpest demonstration that model consensus is one input to acceptance, not
the acceptance mechanism.

**Coverage vs gap:** mechanics are opt-in patterns; the epistemic stance
(independence is *why*; disagreement is kept, not averaged away; convergence
is weak evidence) is unstated. The ask-audit practice — accounting for every
human ask across rounds — is an even younger, unmapped member of this family.

**Counterpressure:** cost and latency (waves multiply work); deliberative
sequencing requires waiting ("documents get built deliberatively,"
[`AGENTS.md` line 4](/AGENTS.md)); and over-formalization risk — the seal and
manifest machinery of predecessor projects was deliberately left as opt-in.

---

<a id="threads-ceremony"></a>

# Thread 6: The Ceremony Budget — Proportionality As An Explicit Currency

**Provisional classification: value with tool consequences.** The corpus
repeatedly reasons about process cost as a *budget* that rules compete for,
the same way context-as-budget disciplines attention.

- "Process is proportional | Strong research protocols are available without
  burdening every note or user guide"
  ([`/constitution/README.md` line 167](/constitution/README.md#doc-constitution-principles));
- "It should not demand research-program ceremony from a two-paragraph user
  guide" ([`draft0.sol56m.md` lines 113-115](/design/doc-constitution/draft0.sol56m.md#doc-constitution-principles));
- the ruling's "ceremony-free uniqueness... The **ceremony budget** — a
  discipline all three syntheses claimed to honor — argues for the cheaper
  settlement"
  ([`proposal4` lines 124-126](/constitution/naming/exploration/proposal4.glm53m.md#the-case-for-location-identity));
- skillify's canvas is "a design prompt, not mandatory ceremony"
  ([`README1.sol56x.md` line 690](/design/skillify/README1.sol56x.md#skillify-canvas));
- hierarchy tension: "How much frontmatter is justified for small support
  docs before the ceremony exceeds the audience?"
  ([`README-heirarchy1` lines 387-389](/constitution/README-heirarchy1.glm53m.md#doc-constitution-hierarchy1-tensions)).

The most instructive evidence is a *reversal*: adoption revision 1, Decision 3
removed a codification the agents had invented — "The phrase 'explicitly
documented local equivalent' was an agent invention from the wave-0 drafting,
not a human requirement... demanding a statement would add noise and one more
thing to maintain" — deviations stay tacit
([`adoption1` lines 79-89](/design/doc-constitution/adoption1.glm53m.md#doc-constitution-adoption-revision-deviations)).
The constitution era has already once over-documentation'd and been corrected.

**Distinction from neighbors:** context-as-budget governs *attention paid by
readers/agents*; ceremony budget governs *effort paid by authors*. They point
in the same direction (less is more) but fail differently (an unreadable
skill vs. an unowned deviation).

**Coverage vs gap:** named as a principle ("Process is proportional") but not
developed as a budget with the same rigor as the ambient cut's
corruption/cheapness/universality tests
([`/constitution/README.md` lines 693-705](/constitution/README.md#doc-constitution-context-ambient)).
A broader constitution could give ceremony an admission test of its own.

---

<a id="threads-selfdemo"></a>

# Thread 7: Self-Demonstration — The Document Practices What It Proposes

**Provisional classification: principle.** Founding statement:

> "The constitution must land practicing its own rules... **A constitution
> whose storage shape contradicts its content is a memo, not a
> constitution.**"
> — [`draft0.glm53m.md` lines 110-115 (C1.4)](/design/doc-constitution/draft0.glm53m.md#c-vision)

Recurrences: the naming discussion "uses short explicit anchors and cites
them actively, so **the document demonstrates the proposed practice rather
than merely describing it**"
([`draft2-discussion0` lines 74-76](/constitution/naming/exploration/draft2-discussion0.sol56x.md#discussion));
the naming draft0 "practices what it explores, so I have done the thing under
study first: I name myself `herald`"
([lines 88-89](/constitution/naming/exploration/draft0.glm53m.md)); the
ruling's own frontmatter is "the worked example" of hats
([`proposal4` lines 162-165](/constitution/naming/exploration/proposal4.glm53m.md#hats-and-cross-modal-aliases-restored)).

**Situation of origin:** constitutions about documents are unusually testable
— their claims are checkable against their own bytes. The workspace learned
to exploit that: every major proposal ships with its own worked specimen.

**Coverage vs gap:** stated in a founding draft, *absent from the maintained
constitution*. The canonical README anchors itself and links its sources but
never requires self-demonstration of future amendments. An easy, cheap
candidate for the broader core.

**Counterpressure:** performing can shade into theater (skillify's "rubric
theater" failure mode is the adjacent warning,
[line 683](/design/skillify/README1.sol56x.md#skillify-failures)); a
demonstration is evidence, not proof — the ruling's own caution that
convergence is not correctness applies to self-demonstrations too.

---

<a id="threads-resumption"></a>

# Thread 8: Sessions, Loss, And Resumption — The Present Account Over The Transcript

**Provisional classification: method, with a principle core.** The workspace
has a well-developed practice for *resuming* thought across session death,
history resets, and months of drift — and it deliberately does not practice
universal transcript retention. The principle core: resumption is achieved by
distilled re-entry documents, not by keeping everything.

The loss story is real, not hypothetical: a "massive architectural refactor
that was lost when jj/git history was reset" motivated the Phase A recovery
guide, whose stance is:

> "The goal is NOT to mechanically replay diffs. The goal is for the agent to
> **understand what was done and why**, then implement equivalent changes
> fresh against whatever the current codebase looks like."
> — [`/doc/opencode-session.md` line 591](/doc/opencode-session.md)

Its procedure ranks *reasoning over patches*:

> "1. **Read the reasoning first.** Start with the 97 reasoning blocks. They
> contain the agent's design decisions, vocabulary choices, and architectural
> rationale. **Understanding these is more valuable than any patch.**"
> (line 1330) ... "6. **Adapt, don't copy.**"
> (line 1350)

The same stance was later promoted into a standing guide:

> "Reconstruction is not replay... **Recover the intent. Recheck the
> constraints. Redesign the destination. Use history as evidence, not
> gravity.**"
> — [`/code/task-reconstruction/philosophy.md` lines 42-57](/code/task-reconstruction/philosophy.md#reconstruct)

with a sharp test for whether a brief still describes the ask rather than the
path: "Test the brief by removing historical proper nouns"
([lines 163-166](/code/task-reconstruction/philosophy.md#reconstruct-present-brief)).

The retention boundary is equally deliberate:

- transcripts are saved *selectively* —
  [pluginification in rektide/dl](https://github.com/rektide/dl/blob/e9c7c401725a93d2c4d56f41851fdb832d772b09/doc/transcript/pluginification.md)
  is one curated session;
  [`.transcript/PLAN-combine.md`](file:///home/rektide/src/rekon/.transcript/PLAN-combine.md)
  pairs a plan with its completion log; the big loss dumps
  (`loss1.md`, `loss2.md`, `rebuild.md` in `.test-agent/`) live in ignorable
  scratch;
- the session database is treated as a *mine* for recovery, never a canon
  ([`/doc/opencode-session.md`](/doc/opencode-session.md), "Quick Start:
  Finding where something was built");
- "Scratch is pre-history" — evidence that stays load-bearing at acceptance
  must be absorbed or promoted, and that rule "binds this very effort"
  ([`draft0.glm53m.md` lines 389-395](/design/doc-constitution/draft0.glm53m.md#c-lifecycle);
  [`/constitution/README.md` lines 778-780](/constitution/README.md#doc-constitution-history));
- session URNs appear in frontmatter as *sources of direction*
  (`urn:opencode:session:...` in
  [`draft0.glm53m.md` lines 10-13](/design/doc-constitution/draft0.glm53m.md)
  and [`adoption1` lines 29-32](/design/doc-constitution/adoption1.glm53m.md)) —
  the session is citable as provenance while being explicitly not the medium
  of record.

**Situation of origin:** catastrophic loss (the history reset) plus routine
loss (session death, compaction). The workspace's answer was a genre: the
recovery guide / present-tense brief that converts a dead transcript back
into living direction.

**Distinction from neighbors:** this is Thread 1's "coherent present vs exact
past" distinction under failure conditions; and it is the quiet partner of
Thread 3 — silent loss is the enemy just as silent drift is.

**Coverage vs gap:** the [index](/constitution/index.md) links the
reconstruct/reconstruction pair as "a neighboring practice," but the
constitution proper never connects session mortality to its own
scratch/pre-history and promotion rules. The genre (recovery guide) and its
stance (reasoning over patches) are uncodified.

**Counterpressure:** distillation costs effort and can launder history — the
reconstruct guide guards this with "mine history by claim, not chronology"
([lines 188-201](/code/task-reconstruction/philosophy.md#reconstruct-mine-history)) and
the evidence-authority ladder; a present account that forgets *why* something
was rejected will re-litigate it.

---

<a id="threads-commits"></a>

# Thread 9: Commits As Thinking-Out-Loud — The Version-Control Stream As Reasoning Surface

**Provisional classification: method.** Commissioned explicitly this round:
"intentional clear commits as we go, with distinct explained changes/new
revisions, are themselves part of this thinking-out-loud practice, not final
housekeeping." The corpus has carried this since the one-line workflow "jj
commit your changes with a verbose justification / exposition on your
changes" ([`/prompt/.archive/workflows/commit.md` line 1](/prompt/.archive/workflows/commit.md)).

Evidence of practice:

- "please jj commit as you go! commit pro-actively in logical groups. when
  comments come in for review, check in before making edits. **break up
  corrections from review into individual well argued cohesive commits.**"
  ([`AGENTS.md` lines 120-121](/AGENTS.md));
- the empty-commit license: "do not be afraid of empty commits!" with the
  explanation that jj's auto-staging keeps the stream moving
  ([line 217](/AGENTS.md)); "you do not need to verify your commit has
  happened... move on" (line 218) — commit narration is cheap enough to be
  casual;
- commit *content* discipline: ticket ids in the body but never the title;
  no "phases" in titles because that is "short-term planning information, not
  long term meaningful" ([lines 228-230](/AGENTS.md));
- the planning-era rationale: "having a stream of commits made during the
  plan will help insure maximum flexibility in revising, allowing us to
  re-do specific narrow bits of work" ([`draft.md` lines 37-39](/prompt/.archive/workflows/draft.md));
- jj-hunk exists specifically to make *logical* commits when file boundaries
  and logical changes diverge ([`AGENTS.md` lines 234-259](/AGENTS.md));
- live practice in this very repository's log: subjects like "naming:
  override response draft1 — glm53m" (per-wave, per-model commits) and the
  checkpoint commits composing this document.

**Situation of origin:** agentic work sessions that die without warning; the
commit stream is the one narration layer that survives automatically, is
ordered, and is diffable. It became the de facto heartbeat of reasoning.

**Distinction from neighbors:** the constitution treats version control as
*history* — "Version control remains the complete file history"
([`/constitution/README.md` line 272](/constitution/README.md#doc-constitution-module-support));
"File history under jj is the real log" ([`draft0.glm53m.md` line 389](/design/doc-constitution/draft0.glm53m.md#c-lifecycle)).
The thread's claim is stronger and different: commit messages are *thinking
in real time* (forward-looking, argumentative, directed at future resumption),
not just an audit trail. History is what commits leave behind; narration is
what they are.

**Coverage vs gap:** entirely operational (AGENTS.md), absent from the
constitution. Note also the friction: humans must review all changes before
squash, never `jj edit` ([`AGENTS.md` line 219](/AGENTS.md)) — the stream's
integrity is protected precisely because it is a reasoning record.

**Counterpressure:** narration can bloat; the hunk-tooling gotcha section
shows how easily commit logic and file logic tangle; and diffs do not carry
*why* unless the author writes it — which is why the "verbose justification"
instinct matters more than the mechanics.

---

<a id="threads-wiki"></a>

# Thread 10: Wiki-Ness As Founding Aspiration — The Unrecorded Pre-History

**Provisional classification: value (identity-level).** The workspace's
referentiality values have a *pre-history* that the constitution's lineage
section does not record, because it predates the constitution lineage
entirely. The oldest artifact in the corpus states it in the workspace's own
unrevised voice:

> "[a short-name] is **a hyperlink internal to the textual world of this
> project**. **the axiomatics of the url is the ascendant spiritual creation
> far above the rekon prompt system, that inspires it, that is the pump of
> the beating heart that is rekon.** we want to be increasing the hypermedia
> referentiality of the rekon system by referring to things by their SHORT
> NAMES."
> — [`/prompt/.archive/workflows/draft.md` line 96](/prompt/.archive/workflows/draft.md)

Corroborating pre-history: the January 2026 creative project
([`/prompt/.archive/uri-axioms/`](/prompt/.archive/README.md)) — a
two-model parallel effort (glm47/k2 suffixes, "which predates the formal wave
conventions in `AGENTS.md`," archive README line 57) generating image prompts
for Tim Berners-Lee's URI axioms and *Cool URIs don't change*. The
workspace's first multi-model wave was an homage to durable naming.

The aspiration reappears as settled language in the constitution era:

- "The workspace should behave like a plain-Markdown wiki. A reader can cite
  a section, follow its evidence... and return later without relying on one
  renderer's generated heading slug"
  ([`/constitution/README.md` lines 486-490](/constitution/README.md#doc-constitution-references));
- the ruling: "This is **the wiki settlement**, and the stated ambition of
  this effort has been a wiki-ish system from the start... the tree is the
  namespace"
  ([`proposal4` lines 86-88](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling));
- and the wiki's economics accepted as livable: stale links as "visible rot"
  ([lines 93-95](/constitution/naming/exploration/proposal4.glm53m.md#what-the-rename-ripple-costs)).

**Situation of origin:** the prompt-system era, where "rekon" *was* its
prompt system and hypermedia referentiality was the explicit telos.

**Coverage vs gap:** the constitution's [lineage section](/constitution/README.md#doc-constitution-lineage)
begins at the predecessor project constitutions (is-tree, dolt-rs); the
workspace's own pre-constitutional roots — short-names, the URI-axioms
project, the workflow set — are unrecorded there. This matters for a broader
core: the wiki settlement is being treated as law without its own history
being part of the record (the archive README is the only account, and it is a
retirement notice).

**Counterpressure:** romantic pre-history can over-bind the present; the
ruling's OKF case shows the aspiration must still be argued on current
mechanics, not ancestry. Record it as lineage, not as authority.

---

<a id="threads-weaker"></a>

# Weaker Or Single-Site Observations (Honest Bounds)

These recurred weakly or live in single documents; recorded so the next round
can test rather than rediscover them:

- **Structured questions to the human.** "prefer multiple choice to single
  choice questions... give the user a chance to weigh the options with
  multiple choice options" ([`AGENTS.md` lines 64-65](/AGENTS.md)); the
  reconstruct guide operationalizes it ("ask a short multiple-choice
  question... Several answers may apply. State which objective wins when
  they conflict," [lines 117-124](/code/task-reconstruction/philosophy.md#reconstruct-job)).
  Candidate member of the thinking-and-talking family: even *questions* are
  structured documents. Evidence thin: two sites.
- **Deliberative pacing.** Waiting for subagents so later documents benefit
  from earlier ones ([`AGENTS.md` line 4](/AGENTS.md)); the ruling's "The
  ruling does not skip the process; it orients it"
  ([`proposal4` lines 213-215](/constitution/naming/exploration/proposal4.glm53m.md#the-process-from-here));
  the design→syn→design2 rhythm ([`AGENTS.md` line 43](/AGENTS.md)). Possibly
  just Thread 5's rhythm under another name. The current "come up for air"
  round is itself the newest instance — a scheduled zoom-out — but I found no
  corpus rule *mandating* periodic reassessment. Hypothesis only.
- **No time estimates, ever** ([`AGENTS.md` line 111](/AGENTS.md)); **never
  flat layouts** (line 114); **let it fail** (line 118); **assume sole
  actor** (line 231). Each is a strong rule in AGENTS.md with constitutional
  gravity but no anchoring argument recorded anywhere. If the broader core
  wants them, their *why* needs recovery first.
- **Index-maintenance honesty.** "maintaining an index.md during this is hard
  but is encouraged once we start accepting the work as good"
  ([`AGENTS.md` line 45](/AGENTS.md)) — a rare admission of practice
  difficulty; the `summarizer` tool note (line 23) shows the intended relief
  is tooling, not more rules. Relevant to whoever owns the reading-map
  maintenance burden.
- **The recovery-guide genre as knowledge type.**
  [`/doc/opencode-session.md`](/doc/opencode-session.md) is simultaneously
  reference (database schemas), procedure (queries), and recovery brief
  (Phase A). None of the current lifecycle vocabularies (posture, disposition,
  wave role) name it. Weak signal, real gap in vocabulary.

---

<a id="threads-table"></a>

# Summary Table

| # | Thread | Class (provisional) | Present coverage | Strongest counterpressure |
|---|---|---|---|---|
| 1 | Thinking/talking in Markdown | value | mechanics only; principle unstated | context budget; scratch mortality |
| 2 | Referentiality/identity as family | family (values + methods) | naming wave (unsettled); anchors/namespace in law | ceremony budget; fifty-row ledger cost |
| 3 | Loud over silent | value | practiced everywhere, named nowhere | noise fatigue; governance earned by volume |
| 4 | Anti-conflation | method | practiced; unstated in rekon law | vocabulary inflation / sediment |
| 5 | Independence as evidence; convergence is not authority | principle + method | opt-in patterns; stance unstated | cost; deliberative latency |
| 6 | Ceremony budget | value (+tool) | one principle row; reversal precedent exists | under-documentation risk |
| 7 | Self-demonstration | principle | founding draft only; absent from canon | demonstration is not proof; theater risk |
| 8 | Resumption over transcript retention | method (+principle core) | neighbor guides; unlinked to constitution | distillation can launder history |
| 9 | Commits as thinking-out-loud | method | AGENTS.md only; constitution sees history, not narration | bloat; hunk/file logic tangles |
| 10 | Wiki-ness as founding aspiration | value (identity) | one aspiration sentence; pre-history unrecorded | ancestry is not authority |

<a id="threads-open"></a>

# Open Questions For The Broader Constitution (Discussion, Not Rules)

1. Which threads are *constitutional* (stated in the core) versus *procedural*
   (belonging in skills like doc-pass or reconstruct) versus *tool contracts*
   (anchor tooling, assembler)? My provisional splits are marked per thread;
   they are inputs to the discussion, not conclusions.
2. Can Thread 1 be stated without collapsing Threads 3-10 into it — is
   "thinking in Markdown" the genus of which the others are species, or a
   peer value among several? The corpus suggests peer: loudness, ceremony,
   and independence each constrain *any* medium, not only Markdown.
3. Does the identity family (Thread 2) want a map in the core (a "family
   register"), or does a register reintroduce the registry machinery the
   ruling just declined?
4. Should ask-accounting (the ask-audit genre) generalize beyond naming —
   "no ask silently disappears" as a workspace-wide accountability rule for
   commissioned waves? It is one round old; evidence is thin, but the failure
   it answers (human asks killed silently by syntheses) is general.
5. Where does the pre-history (Thread 10) get recorded? The constitution's
   lineage section currently starts one era too late; the archive README is a
   retirement notice, not an account.
6. Hypothetical work this suggests (explicitly *not* filed, per commission
   and the [index](/constitution/index.md)'s caution against a second work
   tracker): a constitution-principles wave drafting candidate statements for
   threads 1, 3, 5, 7; a pre-history lineage note; a recovery-guide
   vocabulary decision.

<a id="threads-cross-references"></a>

# Cross-References

- The [maintained constitution](/constitution/README.md) **is the account this
  round reassesses**; its opt-in patterns table holds the mechanics that
  Thread 1 argues are the residue of an unstated value.
- The [reading map](/constitution/index.md) **orients** this document among
  the maintained account, recorded decisions, and the naming exploration; its
  "do not infer acceptance" rule (lines 99-100) grounds Thread 1's
  referable-vs-authoritative distinction.
- The [naming override](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling)
  **supplies** the wiki-settlement, loud-breakage, ceremony-budget, and
  convergence-not-correctness passages; it is the single richest source for
  Threads 2, 3, 5, 6, and 10.
- The [ask audit](/constitution/naming/exploration/ask-audit0.glm53m.md#what-this-audits)
  **evidences** ask-accounting (Thread 2, member 11; Thread 5) and the
  restored/killed status vocabulary.
- The [draft2 discussion](/constitution/naming/exploration/draft2-discussion0.sol56x.md#disagreements)
  **evidences** anti-conflation (the seven-questions move, Thread 4), the
  channel/artifact split (Thread 2, member 3), and the full disagreement
  ledger (Thread 5).
- The founding drafts ([`glm`](/design/doc-constitution/draft0.glm53m.md#c-vision),
  [`sol`](/design/doc-constitution/draft0.sol56m.md#doc-constitution))
  **carry** the Thread 1 openings, the self-similarity requirement (Thread 7),
  scratch mortality, and the mutable-tips-over-durable-evidence pairing.
- [`AGENTS.md`](/AGENTS.md) **remains the operational home** of the wave,
  doc-pass, addendum, and commit practices (Threads 1, 5, 9); nothing here
  proposes moving it — only connecting the constitution to what it already
  says.
- The [prompt archive](/prompt/.archive/README.md) and its
  [workflows](/prompt/.archive/workflows/draft.md) **are the pre-history**
  (Thread 10 and Thread 1's origin): plan-as-heart, journaling, self-review
  dialogue, verbose commits, short-names-as-hypermedia.
- The [reconstruct guide](/code/task-reconstruction/philosophy.md#reconstruct) and the
  [session recovery doc](/doc/opencode-session.md) **form** Thread 8's
  evidence pair; the latter's Phase A guide is the genre specimen.
- [`adoption1`](/design/doc-constitution/adoption1.glm53m.md#doc-constitution-adoption-revision-deviations)
  **records** the ceremony-budget reversal (Thread 6) and the
  recorded-decision genre (Thread 2, member 10).
- The [hierarchy revision](/constitution/README-heirarchy1.glm53m.md#doc-constitution-hierarchy1-terms)
  **demonstrates** anti-conflation by terms-of-art (Thread 4) and carries the
  disposition/trust-tier machinery cited in Thread 1's
  referable-vs-authoritative distinction.
- The [skillify design](/design/skillify/README1.sol56x.md#skillify-not) and
  the [agents modularization plan](/design/rekon/agents-modularization0.sol56x.md#agents-maintenance-skillify-two-transformations)
  **bound** Threads 1 and 3: transcript-like material stays out of skill
  machinery; parity ledgers and no-automatic-pruning are loudness applied to
  delivery.
