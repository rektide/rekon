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
    resource: /code/reconstruct/README.md
    title: Reconstruct from the present
    author: model:openai-gpt-5.6-sol
  - id: opencode-session
    resource: /doc/opencode-session.md
    title: opencode session storage, recovery, and Phase A recovery guide
    author: project:rekon
  - id: transcript-pluginification
    resource: /doc/transcript/pluginification.md
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
([`/code/reconstruct/README.md`](/code/reconstruct/README.md),
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

And the one-line commit workflow is the ancestor of Thread 8 (commits as
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
6. **Narrating change** — commit messages as exposition (Thread 8).

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
see Thread 7's "reconstruction is not replay."

**Handoff/resumption vs universal transcript retention.** The workspace saves
*selected* transcripts ([`/doc/transcript/pluginification.md`](/doc/transcript/pluginification.md),
[`/doc/opencode-session.md`](/doc/opencode-session.md)) and mines the session
database under loss, but never treats retention as the default; scratch is
"pre-history" and evaporates unless promoted
([`draft0.glm53m.md` lines 389-395](/design/doc-constitution/draft0.glm53m.md#c-lifecycle)).
Resumption is achieved by *distilled re-entry documents* (recovery guides,
present-tense briefs), not by keeping every word. See Thread 7.

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
3. **Ceremony budget** (Thread 5): mandating stage-setting or addenda for
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
