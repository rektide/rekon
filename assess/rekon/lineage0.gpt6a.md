---
type: LineageAssessment
title: "Rekon's cumulative work: learning to keep the practice alive"
description: Reconstructs the recent constitutional and practical lineage, connects the September 6 DL stewardship and Lastact-to-Rekon sessions, and examines the emerging docs-and-tickets wiki without declaring a universal model.
resource: /assess/rekon/lineage0.gpt6a.md
tags: [rekon, lineage, praxis, stewardship, happenings, wiki, decomposition, tooling]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-10-06
extensions:
  session: urn:opencode:session:ses_f8725fb70ffeCVmF4JxXaA4W4N
  commissioned_by: urn:opencode:session:ses_f872bf58affeS5IBlC38cA6iYX
  historical_baseline: cf036c5a995d09317b98f5450f12101b0d45ccd5
  scope: "2026-08-28 through 2026-09-06; selected earlier antecedents"
  independence: "No concurrent Lastact review artifact or tooling peer report read. Not the gated broader synthesis."
  tracker_mutations: none
sources:
  - { resource: /assess/rekon/brief0.gpt6a.md, title: Rekon becoming cumulative research brief }
  - { resource: /constitution/index.md, title: Status-aware constitutional reading map }
  - { resource: /constitution/reassessment0.gpt6a.md, title: Coming up for air }
  - { resource: /constitution/threads0.glm53m.md, title: Recovered constitutional threads }
  - { resource: /constitution/work-map0.gpt6a.md, title: Existing work and hypothetical connections }
  - { resource: "urn:opencode:session:ses_f876a85afffeeKH82rtZMoWGsx", title: DL timeline picker and architectural stewardship direction }
  - { resource: "urn:opencode:session:ses_f872bf58affeS5IBlC38cA6iYX", title: Lastact assessment and broader Rekon praxis direction }
  - { resource: /AGENTS.md, title: Current guidance and the three verified stewardship commits }
  - { resource: /.beads/issues.jsonl, title: Read-only inspection of the current exported work records; not a lifecycle event log }
---

<a id="situation"></a>

# What is happening

Rekon is trying to make technical work **cumulative without making its past
compulsory**. A session should leave more than an answer: a useful distinction,
an addressable argument, an improved capability, or a better way to do the next
piece of work. But preserving that learning must not require preserving every
implementation, loading every document, or turning every thought into a ticket.

That is my **inference**, extending the September 4
[reassessment](/constitution/reassessment0.gpt6a.md#recenter), rather than a new
accepted definition. The distinctive emphasis today is that *the manner of
working is itself being improved in public*: the human notices a failure or
possibility during ordinary project work, negotiates its meaning with an agent,
and brings the resulting practice back into Rekon. Rekon is both a source of
guidance and a place where experience changes that guidance.

The emerging wiki is therefore not merely a collection to be organized. It is
a shared environment in which people and agents can **find the situation,
think together, act with appropriate authority, and return what they learned**.
Markdown, tickets, sessions, commits, skills, and tools contribute different
parts of that environment. Their relationships matter more than making them
instances of one universal entity type.

The commission asks for the soul of the recent work, not another naming
grammar. This report supplies a source-grounded interpretation for the later
gated discussion. It does not judge the concurrent Lastact reviews, choose a
tooling home, adopt a constitution, or create work items.

<a id="arc"></a>

# The recoverable arc

The investigation covered the requested August 28–September 6 window. The
bounded Rekon ancestor log through `cf036c5a` contains **no commits with
committer dates August 28–31**; its next earlier commit is August 23. That is
not evidence of an idle workspace. The directly inspected cross-project
constitutional evidence becomes substantial on August 31. I have not filled
August 28–30 with an invented daily story.

| Movement | Observed evidence | What it contributes to this reading |
|---|---|---|
| **Aug 31–Sep 1: projects learn how to exchange research** | `dolt-rs` commits its dependency-ordered wiki at `beb1923d4ef8fbcce91cbd8673168cb8f20d00f3`; `first-compute` adds fair synthesis and wave barriers at `5f3b702160aa2e86c7d186df37a0880572bafe4f`; `is-tree` reconciles practices at `befd4be6f87366b1d9e8fc5e8c93a05221cf4143`. | Different project needs produce different dispatch shapes. Shared obligations emerge through comparison, not by declaring all research one process. |
| **Sep 1–2: Rekon becomes the place to assemble those lessons** | Native session `ses_fa19cd46bffeBW6dPrBzttf4Gv`; first constitution commit `d4448d7e8717`; assembler `5140aaf9b4c5`, followed by hardening `0b22127ea2a5` and `36317f1cf5dc`; modularization design `c232d0c384ff`; skillification synthesis `3840a762f5f7`. | A manuscript can become domain-owned knowledge, exact on-demand exposure, and a derived ambient contribution. This is a change in ownership and use, not just document formatting. |
| **Sep 2–4: naming asks what can remain referable across that world** | Native session `ses_f9cf5c4bbffep3k7RE7wv2frLM`; naming epic `513711d9e407`; human override `048d8458b06a`; ask audit `74dffb84cddf`; later discussion `416f2ec4c84f` and draft `b25511218aab`. | The human can recover ideas that model syntheses suppressed. A useful reference system must serve the wiki ambition rather than replacing it with premature governance. |
| **Sep 3–4: ordinary work supplies mechanisms and counterexamples** | Reconstruction philosophy `f875eeba61d2`, preservation procedure `392d7872a911`, jj reference promotion `478852731bd8`; Navigation work `f36c8f50eb32` and `196d292a4010`; coding module `7cb3fb539e26`. | Preserving understanding can enable redesign; a map of possibilities differs from the route being taken; domain practice needs its own maintained home. |
| **Sep 4: come up for air** | Native session `ses_f91498b20ffeI55m95bDMRZI6z`; work map `05a0b9525684`; threads completed at `463afe88e350`. | The human explicitly asks to recover less-visible threads, including thinking and talking in Markdown, rather than allowing naming to stand for the whole constitution. |
| **Sep 5: support becomes a flexible companion to work** | `ff10b14d90bda49e1fccdacbea7cac8ec40656da` replaces the preflight scout with [followup](/agents/followup.md). | A before/during/after support role supersedes a fixed preliminary checklist. The artifact calls itself a “working spirit,” with scope and evidence boundaries but no mandatory report quota. |
| **Sep 6: project experience revises the shared practice again** | The DL stewardship session produces the three commits detailed below. A separate session begins in Lastact and moves to Rekon to investigate assessment, gates, happenings, and tooling. | The subject is no longer just how agents exchange documents. It is how the human and agents remain jointly responsible for the direction and timing of work. |

These are selected turning points, not a complete changelog. Dates above use
the local `-04:00` calendar and inspected committer dates, not the birth time
of a jj working-copy change. The [design record](/design/doc-constitution/README.md)
and [constitutional index](/constitution/index.md) distinguish pilot guidance,
recorded decisions, proposals, and implementation; later dates do not promote
an artifact's authority.

The early Rekon session is especially explicit about the purpose of
decomposition. In user message `msg_05eaaa5a7001G5izcCTfGio3qQ` (September 1,
20:30:40Z), `GLOBAL.md` and assembly are proposed for **“data-locality for our
updates, history.”** The same message asks the agent to stand back and recover
the work's “energy/activation/flow systems.” This is primary evidence that
local ownership and usable knowledge—not token reduction alone—were the aim.

There is also an older root. The retired
[drafting workflow](/prompt/.archive/workflows/draft.md), especially lines
15–25, 39, 50–63, and 96–100, already treats possibility as valuable, plans as
the place ideas form, commits as reversible thought, tickets as evolving
architectural explanations, and short names as hypermedia. Its obsolete
tracker instructions are not current commands. Its importance is historical:
the wiki and deliberation ambitions were not invented by last week's
constitution wave.

<a id="today"></a>

# Today's two sessions are one connected episode, not one session

## First occurrence: stewardship is negotiated inside DL work

The earlier session is **`ses_f876a85afffeeKH82rtZMoWGsx`**, titled
**“Designing Timeline Picks for Cyclical Pick Engine”**, with native directory
`/home/rektide/src/dl`. It began September 6 at **21:17:37Z**. This was not a
Rekon-titled session that happened to discuss agents: it was concrete picker
work in another project.

The bounded user-message sequence establishes the connection:

- `msg_078afa4e5001FStakKHByQBeUE`, **21:46:12Z**: asks the parent to validate
  and steer the implementation agent in pieces rather than hand it everything.
- `msg_078b09ffd001xqMsltX1OiBOrO`, **21:47:16Z**: distinguishes trusting checks
  from the parent's responsibility for the code's shape and what is possible.
- `msg_078b10f2c00141gfg6UFZMw1VA`, **21:47:44Z**: explicitly requests adding
  that guidance to `~/src/rekon/AGENTS.md`.
- `msg_078b4ab34001G8Clhyg12WDvpS`, **21:52:09Z**: corrects wording that could
  imply disinterest in implementation details.
- `msg_078b73545001BnOdldDzx98Uyf`, **21:54:27Z**: corrects the ban on
  hypothetical futures: **“it's not verboten: it's encouraged, but being
  deliberate, and negotiating.”**

Actual Rekon diffs and the session's successful commit-tool results agree:

| Rekon commit, full identity | Local commit time | Change established | Native assistant message containing the commit receipt |
|---|---|---|---|
| `80bcfdc62805165f5dbe76cf60da827b1b818898` | 17:48:40 | Adds architectural stewardship: parent responsibility, consequential checkpoints, evidence-bearing reports. | `msg_078b1d106001pZMRf9kGEXQJkx` |
| `852ee56a3932120c62e1753f86659c4a1b1d4d25` | 17:52:53 | Replaces broad trust wording with trust that checks ran **and** continued close implementation review. | `msg_078b5abce001bBNySGrVEWsws0` |
| `cf036c5a995d09317b98f5450f12101b0d45ccd5` | 17:54:57 | Replaces “not hypothetical futures” with cooperative exploration and deliberate negotiation of broader solutions. | `msg_078b792340012KH6RUfTzKUGYt` |

This is a particularly good specimen of **practice correcting its own rule**.
The first rule is not treated as a finished interpretation of the human's
intent. Two small, separately committed corrections preserve how its meaning
was negotiated. The resulting [AGENTS stewardship section](/AGENTS.md#architectural-stewardship)
does not equate architectural responsibility with either duplicated testing
or hands-off delegation.

**Provenance boundary:** the push-policy line was pre-existing user work.
Each of these commits changes only the stewardship material; the session
receipts explicitly say the unrelated edit was left untouched. The inspected
working diff still contained that push-policy edit. It is not a fourth product
of this research or evidence that a push occurred.

## Second occurrence: Lastact becomes a case study for Rekon's practice

The commissioning session is **`ses_f872bf58affeS5IBlC38cA6iYX`**, titled
**“Rekon Architecture Review and Agent Praxis Planning.”** Its originating
user message is `msg_078d40a7d001nwO2V6fjP4Z65b`, **22:25:57Z**. The user
reports that Lastact is working as desired, then asks to workshop architectural
plus close assessment, run independent reviews, and gate analysis on their
completion. That is a user report and commission, not this report's product
verification.

The native **`location-switched`** message
`msg_078d4947c002b6l10BiAp1TQH2`, sequence **57**, at **22:26:32.700Z**, records
the move from `/home/rektide/src/opencode-lastact` to
`/home/rektide/src/rekon`. The native session ID remains the same. Its present
directory would therefore conceal part of its origin if used as the sole
historical classification.

The user explicitly identifies this as the day's second major stewardship
session and asks to connect them. **Observed:** one session changes Rekon's
guidance while remaining in DL; the other relocates from a plugin project to
Rekon. **Inference:** together they form an episode of learning how to conduct
agent-assisted work. **Aspiration:** making such episodes navigable as
“happenings,” potentially through epics and changelog tooling, is now being
proposed. No native `Happening` object or accepted schema follows from the word.

This distinction is productive. The episode has an originating pressure,
related occurrences, changes in understanding, and durable results. Its
meaning cannot be reconstructed from a session title, repository path, ticket
status, or commit list alone. But neither does it require copying all the
transcripts: the few direction messages, location event, commit receipts, and
resulting artifacts already support a credible re-entry account.

<a id="praxis"></a>

# Praxis: becoming equipped, without outsourcing judgment

The new word *praxis* gives a center to an existing pattern: prepare enough to
think usefully, act under an understood boundary, inspect the result, and alter
the practice when experience warrants it. This is my proposed reading of the
user's **“well equipped”** language, not a request to institute another rigid
sequence for every task.

Three different concerns must survive the compression:

1. **Delegation allocates work, not all responsibility.** The parent can trust
   execution reports while reading implementation and checking whether the
   solution serves the actual purpose. A checkpoint earns its place when a
   consequential architectural choice becomes reviewable.
2. **A gate protects a particular next activity.** Independent assessment
   comparison needs all included reviews; a dependent investigation needs its
   prerequisite evidence; authorization to change a product may need a human
   decision. Those are not interchangeable “wait until everything is done” rules.
3. **Exploration is not commitment.** Possible futures are legitimate inputs to
   design. They become implementation scope only through deliberate discussion
   and authorization. Automatically narrowing to the immediate case makes
   architectural bets just as automatically generalizing does.

There is direct precedent for distinct gates. The August 31
[`first-compute` amendment](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-wave-seal)
requires all included assignments and an immutable synthesis boundary; its
[fair-synthesis section](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-fair-synthesis)
warns that complementary lenses are not independent votes. In contrast,
[`dolt-rs` dependency dispatch](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-topology-gates)
waits for the documents a particular next author must inherit. Neither example
requires importing its full manifest or sealing machinery into every Rekon task.

September 4 supplies another important counterweight. In
`ses_f91498b20ffeI55m95bDMRZI6z`, user message
`msg_06eba10d8001xmcGpbSxDzcPtg` asks for the parent's own follow-through and
discussion instead of reflexive agent dispatch. Today's full-wave gate does
not contradict that request: **waiting before interpreting a comparison is
different from abandoning thought while others work**. Here the parent can
prepare its independent assessment and gather unrelated prerequisite evidence
without reacting to incomplete peer findings.

The [threads study](/constitution/threads0.glm53m.md#threads-independence)
already explains independence as an epistemic instrument. Today's addition
is operational specificity: receipt of a result, readiness to interpret a
corpus, and authority to act on it are different transitions. Keeping those
separate preserves deliberation without worshipping delay.

<a id="decomposition"></a>

# Decomposition and tooling serve the same purpose

The strongest decomposition principle in the corpus remains
**[ownership first, context behavior second](/design/rekon/agents-modularization0.sol56x.md#agents-maintenance-skillify-two-transformations)**.
It protects a useful monolith's behavior while making domain history, review,
and maintenance more local. It does not demand that every directory become a
skill, every skill receive a global fragment, or every concern become a package.

The recent [coding-practice module](/code/README.md#code-inventory) demonstrates
a concrete version: common practice in the landing page, specialized procedures
in support documents, and jj and JavaScript as neighbors rather than absorbed
subsections. The [reconstruction philosophy](/code/task-reconstruction/philosophy.md#reconstruct)
explains why retained evidence is compatible with replacing a mechanism. The
result is not preservation for its own sake; preserving understanding gives
future work room to change.

**Observed tooling already exists.** The
[assembler](/src/agents/assemble.ts) loads declared fragments, rejects duplicate
identities and orders, renders source provenance, and computes UTF-8 byte costs
(lines 390–478 inspected). The
[replay script](/code/task-reconstruction/replay-docs.sh) is an executable
preservation mechanism. Cotail itself made this report's cross-session recovery
practical. None was executed here to change project state; implementation
existence is not a fresh claim of end-to-end correctness.

The proposed wiki/change-history/ACP ecosystem is much less established in this
assignment. The user's “no tooling yet” is best read **in that projected scope**,
not as a literal inventory of Rekon. General skillification already says
[deterministic behavior should become tools or validation](/design/skillify/README1.sol56x.md#skillify-placement),
not ever-longer prose instructions. This is a strongly pro-tooling stance:
tools can lower the cost of the desired practice and make new work possible.
The care about authority and provenance specifies what good tooling owes its
users; it is not an argument against building it.

Two small present-tense observations show why maintenance matters:

- No `constitution/agents.manifest.json` exists, and AGENTS remains
  hand-maintained. The [assembly status](/constitution/agents-assembly.md#agents-assembly-status)
  remains right about the missing cutover, but its “only one fragment” sentence
  has aged: `code/GLOBAL.md` now also exists. Implemented assembler, extracted
  fragments, and production adoption are three different facts.
- The Watchman synthesis path cited by the September 4 work no longer exists
  under `~/src/opencode-watchman`. This investigation found and read the
  [earlier artifact in `opencode-watchman-old`](file:///home/rektide/src/opencode-watchman-old/.design/watchman/nav0-syn0.gpt56sol.md).
  Its account separates newly discovered problem spaces from immediate route
  commitments (lines 112–128). The re-finding difficulty is itself evidence for
  careful source identity and navigation, not permission to silently repair all
  old cross-project citations.

<a id="wiki"></a>

# Tickets and documents as a wiki: preserve the productive tension

“Tickets coordinate; documents explain” is a useful
[constitutional division of responsibility](/constitution/README.md#doc-constitution-principles).
It would become harmful if interpreted as **tickets must not explain**. The
archived drafting workflow asks epics to describe subsystems, interfaces, and
use cases; today's commission asks for self-explaining architecture through
Beads. An epic's account of purpose and boundaries is part of coordination,
not an illicit duplicate of documentation.

The desired wiki can therefore include both:

- **Durable knowledge:** what a capability means, how it works, evidence and
  alternatives, and what someone should understand now.
- **Situated work:** why this change is being attempted, its scope, decisions,
  lifecycle, results, and relationship to other commitments.

The important boundary is who owns a fact and how it changes—not whether both
surfaces contain prose. A useful epic can orient a reader through an
architecture and point to its maintained explanations. A closed implementation
ticket need not turn into a permanently open proxy for the module's existence.

The inspected [export](/.beads/issues.jsonl) already has architectural homes:
`rekon-doc-constitution`, `rekon-con-naming`, `rekon-navigation`,
`rekon-skillify`, and the AGENTS maintenance work. These records and the
[work map](/constitution/work-map0.gpt6a.md#existing) are a starting point for
later discussion, not grounds for a new epic per noun. Some current exported
descriptions still point at old `/doc/README.md` or `/doc/GLOBAL.md` destinations.
That is a navigation tension between work's historical account and its present
entry points, not a reason to erase the old work or quietly rewrite it here.

**“The tickets are the changelog” remains an aspiration with an ownership
question.** It can mean deriving a readable history from ticket creations,
revisions, decisions, reopening, and closure rather than keeping a competing
manual lifecycle. It cannot safely mean treating current issue rows as a
complete event history. The prior
[Beads offerings investigation](/constitution/naming/exploration/beads-offerings0.sol56x.md#beads-offerings-events)
distinguishes per-issue audit, Dolt history, mutation journal, hooks, and other
event-shaped surfaces, with materially different availability and retention.
Those are inherited September 2 findings, not newly verified installed
capabilities in this assignment.

Similarly, an episode's chronology must not be encoded as false blocking
dependencies. Today's stewardship commits led into the second conversation in
time and meaning; that does not make every Lastact task depend on every DL
task. A *happening* could supply a bounded explanatory grouping across those
owners. Whether it deserves an epic, a document, or a small linked projection
is an open design decision—not an ontological obligation.

<a id="fronts"></a>

# Research fronts worth preserving, not tickets to file

1. **Use today's episode as a re-entry specimen.** Can a new reader recover the
   pressure, user corrections, changed guidance, and present unanswered question
   from two native sessions, three commits, and a short account? This tests the
   value of a happening before deciding its universal representation.
2. **Investigate owner-native change history before designing its projection.**
   For one existing epic, which creations, edits, decisions, and reopenings are
   actually recoverable from the installed tracker? What identity, ordering,
   retention, and missing-history limits must a derived changelog report?
   Do not make a current-state export pretend to be a complete journal.
3. **Test practice delivery without losing inherited obligations.** Where should
   assessment and gating rationale live, what minimum guidance must be ambient,
   and what tools can make the practice cheap? The existing modularization work
   owns much of this question. Today's additions must be accounted for in any
   later parity exercise rather than lost behind an older fragment inventory.
4. **Compare assessment outputs only after their agreed gate.** The current
   Lastact case can test whether a shared form produces useful architectural
   judgment and close evidence, not just similarly headed documents. Tooling
   research should then inform feasibility. This report deliberately supplies
   neither the comparison nor conclusions from the concurrent tooling front.

The ACP/native-session and server-selection questions belong to that tooling
investigation. This recovery contributes a reason they matter: the native
session, its current directory, its origin, and the store through which it was
found are not interchangeable handles. Reporting those honestly would make
cross-session work easier to re-find. It does not establish any uninspected
transport or `--server` implementation.

<a id="crossrefs"></a>

# Cross-references

- [Coming up for air](/constitution/reassessment0.gpt6a.md#connections)
  **supplies the cumulative-work thesis** and preservation/attention/evidence
  connections. This report extends it with exact September 6 provenance and
  the practical negotiation of stewardship.
- [Constitutional threads](/constitution/threads0.glm53m.md#threads-markdown)
  **recovers the older voice**: writing as thinking, independence, commits, and
  wiki aspiration. Today's episode shows those threads acting together, without
  reducing them to one rule.
- [Work map](/constitution/work-map0.gpt6a.md#bounds)
  **constrains the next work discussion**: existing commitments first; explained
  relationships are not automatically blocking edges or new tickets.
- [Skillification](/design/skillify/README1.sol56x.md#skillify-flow) and
  [AGENTS modularization](/design/rekon/agents-modularization0.sol56x.md#agents-maintenance-skillify-two-transformations)
  **explain how knowledge reaches action** and why changing ownership must not
  silently change what agents know.
- [Beads offerings](/constitution/naming/exploration/beads-offerings0.sol56x.md#beads-offerings-boundary)
  **bounds the wiki/changelog ambition with an existing capability study**.
  Work ownership is useful infrastructure, not proof that every kind of knowledge
  or occurrence should be a ticket.
- [Fronts and horizons](/praxis/fronts/README.md#fronts-thread) ⇄ **carries
  the research-fronts vocabulary forward**: the numbered fronts in this report
  are where that vocabulary first crystallized in rekon; digest's intent
  fronts and the fronts-and-horizons praxis generalized it.

<a id="method"></a>

# Evidence and operational boundaries

This is a local-source investigation. Prior research was used as a map, with
load-bearing passages followed into the cited primary documents, session
messages, commit diffs, and selected source files. Interpretations are labeled
as such; user aspirations are not deployed capabilities. No independent reviewer
has verified this report, and no `verified` frontmatter claim is made.

Cotail was inspected before use: its help and installed pnpm launcher lead,
through the resolved `opencoattails` package symlink, to
`/home/rektide/src/opencoattails/src/cli.ts`. Its
[search implementation](file:///home/rektide/src/opencoattails/src/commands/search.ts)
searches several text surfaces, including injected system/synthetic text.
Consequently, stewardship wording appearing in a session is not enough to
identify its authoring session. A Rekon-directory-only search also missed the
DL source because that native session remained in DL.

The discriminating lookup was:

```sh
cotail search '80bcfdc6' --type tool --since 2026-09-06 --json --no-snippet --limit 15
```

Cotail's selected profile points to `opencode-local.db`, not `opencode.db`.
After Cotail located the candidate, exact-session SQLite reads used that
profile-selected source with `mode=ro` and `PRAGMA query_only=ON` to inspect
bounded user messages, commit receipts, and the location event. No raw transcript
was exported into this repository, and no peer-review session content was read.
Session URNs above are exact native handles, not promises of a public web viewer.

`jj help -k revsets` and `jj help -k templates` were consulted before custom
history expressions. Commit correlation used actual diffs and committer dates;
the exported Beads records were read without tracker commands or mutations.
The recovered Watchman file is an explicitly local cross-project reference,
not a declaration that its old path has been canonically renamed.

The owned artifact is this file only. No code, AGENTS guidance, older reports,
indexes, or issues were changed; the pre-existing push-policy edit remains
outside this work. Structural/link checking of this report is separate from
verification of any cited product or proposed practice.

Checks completed before commit: frontmatter decoded with `gray-matter`, Markdown
parsed with `marked`, 28 local links and their supplied fragments resolved, nine
explicit report anchors were unique, and all six cited native session handles
and twelve message handles existed in the profile-selected source. Whitespace
and conflict-marker checks passed. Product test suites were not rerun.
