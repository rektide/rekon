---
type: ConstitutionDiscussion
title: "Coming up for air: a broader Rekon constitution"
description: Reassesses the naming core against recent knowledge, navigation, skillification, assurance, and reconstruction work, and draws the boundary of a new constitutional README.
tags: [constitution, synthesis, navigation, skills, evidence, reconstruction]
status: draft
generated: { by: "cartographer/gpt-6-astra", at: 2026-09-04T19:23:17-04:00 }
stale_after: 2026-12-04T00:00:00-05:00
extensions:
  wave: constitutional reassessment 0
  model_suffix: gpt6a
  model_basis: "Session-supplied openrouter/openai/gpt-6-astra#xhigh label; underlying model identity is not independently exposed."
  session: urn:opencode:session:ses_f91498b20ffeI55m95bDMRZI6z
  actor_claim: { name: "cartographer/gpt-6-astra", status: self-proposed, scope: constitutional reassessment }
  baseline: b0fc711738296988c96e8cd65cad4448e80beb38
  related_work: [rekon-doc-constitution, rekon-doc-constitution-hierarchy, rekon-con-naming, rekon-navigation, rekon-skillify, rekon-agents-maintenance-skillify]
sources:
  - id: current
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution
  - id: naming
    resource: /constitution/naming/exploration/draft3.sol56x.md
    title: Rekon core draft 3
  - id: discussion
    resource: /constitution/naming/exploration/draft2-discussion0.sol56x.md
    title: Draft 2 grammar and core-scope discussion
  - id: hierarchy
    resource: /constitution/README-heirarchy1.glm53m.md
    title: Topic-first hierarchy revision 1
  - id: skills
    resource: /design/skillify/README1.sol56x.md
    title: Skillification revision 1
  - id: modularization
    resource: /design/rekon/agents-modularization0.sol56x.md
    title: Modularizing Rekon's global agent context
  - id: navigation
    resource: file:///home/rektide/src/opencode-watchman/.design/watchman/nav0-syn0.gpt56sol.md
    title: Watchman assurance navigation synthesis
  - id: reconstruction
    resource: /code/task-reconstruction/philosophy.md
    title: Reconstruct From The Present
  - id: preservation
    resource: /code/task-reconstruction/README.md
    title: Reconstruction with docs preservation
  - id: assembly
    resource: /constitution/agents-assembly.md
    title: AGENTS assembly contract and migration status
  - id: work
    resource: /.beads/issues.jsonl
    title: Tracked work snapshot; live decision state inspected with bd show
  - id: okf
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
    title: Open Knowledge Format v0.2, local archive copy consulted
---

<a id="situation"></a>

# Coming Up For Air

The request is not another refinement of naming punctuation. It is to look
around after an intensive period of work and produce a constitution that can
stand for the larger activity, without making one document contain everything.
The starting evidence is the [draft2 discussion](/constitution/naming/exploration/draft2-discussion0.sol56x.md)
at `416f2ec4c84f651e589488d308e66a45a9cf0f85` and
[draft3](/constitution/naming/exploration/draft3.sol56x.md) at
`b25511218aaba992ba92513050d1ba053264236e`.

This reassessment inspected Rekon's recent ancestor history, concentrating on
2026-08-28 through 2026-09-04, and followed the Navigation work into the local
Watchman corpus. It is a bounded cross-topic reading, not a survey of every
workspace project or a fresh verification of every command in the cited guides.
The author read the sources used below directly. This is synthesis, not an
independent multi-model review or an acceptance record.

The subsequent direction keeps the center explicitly on Rekon's constitution:
use preceding project constitutions to identify missing contracts, treat the
broader reference corpus mainly as material for domain skills, and use
**hypothetical tickets** to expose relationships without creating tracker work.
The [constitutional index](/constitution/index.md) and
[work map](/constitution/work-map0.gpt6a.md) make that distinction navigable.

**Working thesis:** Rekon helps people and agents make technical work cumulative:
understanding can become usable capability, action can produce inspectable
evidence, and new evidence can change the course without losing the learning.
Naming makes those relationships addressable. It does not supply their purpose.

A useful continuation prompt is: *Which shared obligations let this learning
and action cycle work across independently owned topics, and which mechanisms
should the constitution deliberately leave to those topics?*

<a id="recenter"></a>

## What Draft 3 Gets Right, And Where Its Center Still Pulls

Draft3's strongest contribution is separating native owners, aliases, channels,
exact evidence, and guarded identity change. The explanation of an accepted tip
as a named mutable selection rather than the identity of every selected artifact
is especially useful. Preserve that.

But its shape still begins with an entity census and a long naming glossary,
then expands naming's machinery until it describes a core. That answers **how
things retain identity**, not yet **why this workspace brings them together**.
It gives knowledge, computation, and publication places in a table without
carrying the activation, navigation, completion, and feedback contracts already
developed elsewhere.

The new README should lead with the work people are trying to do. Introduce terms
where they become necessary. Give naming one substantial place in the compact,
not the responsibility to explain every neighboring concern.

<a id="map"></a>

## The Broader Map

| Recent line of work | What it adds to the shared constitution | What remains with the domain |
|---|---|---|
| [Knowledge modules](/constitution/README.md#doc-constitution-module) | A maintained public account can reconcile evidence without becoming its replacement history. | Authoring modes, detailed adoption procedures, and corpus-specific research protocols. |
| [Topic-first hierarchy](/constitution/README-heirarchy1.glm53m.md#doc-constitution-hierarchy1-shape) | Organize by subject; distinguish where something lives from what a reader may rely on. | Docroot declarations, layout choices, inventories, and individual moves. |
| [Naming draft3](/constitution/naming/exploration/draft3.sol56x.md#planes) | Keep native owners; make cross-domain relations explicit; separate current names from exact evidence. | Full grammar, actor conventions, membership profiles, anchor tooling, and compatibility representation. |
| [Navigation's founding synthesis](file:///home/rektide/src/opencode-watchman/.design/watchman/nav0-syn0.gpt56sol.md) | A possibility map and a chosen course are different artifacts. Direction needs a known starting position, observable arrival, and reasons to stop or reroute. | Navigation's reusable type contract and the Watchman-specific route, profiles, and state machines. |
| [Skillification](/design/skillify/README1.sol56x.md#skillify-contract) | Knowledge becomes capability through activation, scope, process, completion, evidence, and maintenance. | Skill forms, routing scenarios, host profiles, and branch-specific procedures. |
| [AGENTS modularization](/design/rekon/agents-modularization0.sol56x.md#agents-maintenance-skillify-two-transformations) | Ownership decomposition and attention reduction are different changes. Preserve behavior before changing delivery. | Parity ledgers, fragment inventory, production cutover, and per-domain tiering. |
| [Assembly](/constitution/agents-assembly.md#agents-assembly-policy) | Derived delivery must retain provenance and make drift visible without taking editorial authority. | Manifest syntax, path rules, command options, and output mechanics. |
| [Reconstruction](/code/task-reconstruction/philosophy.md#reconstruct-from-top) and [docs preservation](/code/task-reconstruction/README.md) | Preserve understanding while reopening mechanisms against present intent. Exact recovery and clean redesign are different jobs. | Snapshot, replay, split, carrier, and tool-version-specific procedures. |
| [jj reference](/vcs/jj/README.md), [JavaScript](/js/README.md), and [ATProto lookups](/atproto/README.md) | The architecture already has practical consumers: job-oriented references, activated recipes, and explicit limits on what a signal proves. | Command semantics, toolchain choices, API recipes, and their verification. |

This is not a stack in which each row must be completed before the next starts.
It is a set of cooperating responsibilities. A small task can cross several
without producing a document for each.

<a id="connections"></a>

## Three Connections Worth Carrying Into The Draft

**Preservation makes redesign possible.** The current constitution says
promotion is synthesis, not movement (lines 314–327). Reconstruction says history
is evidence, not the default architecture (lines 42–57, 187–237). The preservation
procedure keeps the documentation across a code restart (lines 39–69). Together
they support a stronger position than either conservatism or clean-slate zeal:
retain the reasons and obligations so the next implementation need not retain
every old mechanism. This README revision should practice that same distinction.

**Attention is part of correctness.** Skillification's activation flow can fail
before the body is read or after its procedure starts (lines 99–125).
Modularization therefore separates ownership from activation tiering (lines
69–118). A link or skill file that nobody reaches does not discharge a
costly-to-miss obligation. Conversely, loading everything is not successful
integration. Human recall, descriptions, common bodies, and branch loads all
spend attention; AGENTS byte count measures only part of it.

**Signals must not inherit larger promises.** The Watchman synthesis names
false beacons: real signals that prove less than their names suggest (lines
246–310). The same mistake appears in knowledge work: acceptance is not factual
verification; successful assembly is not useful activation; a passing local test
is not end-to-end assurance; a search miss is not proof of absence. The
[ATProto caveat](/atproto/README.md) at lines 68–84 and the
[jj verification correction](/vcs/jj/README.md) at lines 274–281 make this concrete
outside the naming discussion. These are conceptual connections, not fresh audits
of the external systems.

<a id="corrections"></a>

## Reconciliation Without Erasing Disagreement

| Tension | Proposed treatment |
|---|---|
| “Location is identity” versus “disposition replaces location” | Keep both after removing the category error: path participates in a knowledge name; path does not certify truth, maturity, or acceptance. Upstream layout can be valid while a move within it still changes identity. |
| Hierarchy1 ties acceptance to a human `verified` event (lines 244–255) | Reject the automatic implication. OKF verification means checking against sources/resource, not merely selecting a policy. Record acceptance separately; do not retroactively relabel every source wave. |
| Draft3 changes the ruling's flattened, suffixless spelling | Recommend its structured exact-path grammar, but keep the departure explicit and pending human acceptance. Do not treat the naming ticket's open status as erasing the earlier recorded human ruling. |
| Shared ticket/anchor namespace in the current README versus native work ownership | Propose explicit relations and useful aliases rather than a one-to-one identity equation. Ticket status cannot stand in for knowledge status. |
| Draft3 proposes `/core/README.md` | Do not add that umbrella by default. This request identifies `/constitution/README.md` as the shared compact. Naming can retain a specialized home; its eventual placement is a separate decision. |
| Topic-first future versus existing `design/`, `.design/`, and upstream roots | Prefer domain grouping and honest disposition prospectively. Preserve current paths and accepted carry-island practice; do not use the proposal as migration permission. |
| Explicit declarations versus previously accepted tacit deviations | Declare enough context where a resolver or publication promise actually needs it; do not demand declarations for every obvious local layout choice. [Adoption revision 1](/design/doc-constitution/adoption1.glm53m.md#doc-constitution-adoption-revision-deviations) explicitly rejected that bureaucracy. |
| Continuity versus unbounded process | Account for names and evidence that matter to later reliance. Do not require a ticket, actor register, navigation, skill, and migration apparatus for every note. |

<a id="position"></a>

## What Exists, What Is Decided, What Is Still Proposed

- **Current pilot account:** `/constitution/README.md` remains the maintained
  constitutional entry, itself draft. Initial authoring/torch/adoption work is
  closed in Beads; the umbrella epic remains open.
- **Recorded human direction:** the [naming override](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling)
  affirms location identity, deliberate rename cost, hats, and cross-modal
  aliases. Draft3's further grammar and home choices are proposals, not a new
  human ruling.
- **Implemented but not cut over:** the assembler exists in
  [`src/agents/assemble.ts`](/src/agents/assemble.ts), with tests beside it.
  The [status account](/constitution/agents-assembly.md#agents-assembly-status),
  absence of `constitution/agents.manifest.json`, and current `AGENTS.md` agree
  that production assembly has not replaced the monolith. No tests were rerun
  as part of this source reading.
- **Open design:** `rekon-doc-constitution-hierarchy`, `rekon-con-naming`, and
  `rekon-skillify` remain open. The five Navigation children are open; the
  founding Watchman synthesis is evidence for a reusable type, not that type's
  accepted implementation.
- **Partial practice:** the JavaScript README explicitly defers to AGENTS where
  they overlap. JavaScript and ATProto READMEs have no OKF frontmatter. The jj
  exposure is named `SKILLS.md`. These are reasons to test admission and host
  discovery on real modules, not reasons to deny that the documents are useful
  knowledge or to repair them opportunistically here.

The ticket observations came from live `bd show` on 2026-09-04. The committed
JSONL is an inspection/export surface, not a replacement for Beads' live owner.

<a id="boundary"></a>

## The README Boundary

The planned `README-draft0.gpt6a.md` candidate should own the purpose,
shared obligations, relationships, and a short reading map. It should let a
reader answer **what we are doing, what must not be conflated, where to go next,
and what is not yet settled** without learning the whole naming history.

It should not absorb the grammar ledger, every domain glossary, API examples,
an assembler specification, a navigation schema, a migration schedule, or the
entire AGENTS manual. A useful rule for later amendments is: *put the invariant
here, the procedure with its owner, and the proof beside the work it checks.*

The conversation itself also supplies a correction: delegation must not crowd
out deliberation. The user asked for incremental discussion and the primary
author's own follow-through, not reflexive agent dispatch. That preference
informs the candidate's collaboration clause; it is not a claim that all tasks
should be serial or all hosts share one execution limit.

<a id="questions"></a>

## Discussion Points That Remain Live

1. **Center:** is “making technical work cumulative” the right account of Rekon,
   or should the emphasis fall more strongly on a knowledge commons or an
   activatable workspace? The draft uses the first while retaining the other two
   as properties, not rival architectures.
2. **Constitutional force:** which shared obligations are durable enough for the
   compact, and which should remain pilot guidance? General Navigation deserves
   recognition without prematurely freezing its schema.
3. **Naming boundary:** can the constitutional identity invariant be accepted
   separately from exact grammar, export profiles, and the specialized module's
   eventual home? The draft exposes rather than hides those choices.
4. **Next evidence:** a bounded walkthrough of an ordinary task should show that
   someone can find the current account, select useful depth, identify the next
   action, inspect its result, and feed a correction back to the right owner.
   More synthesis alone cannot establish that.

<a id="crossrefs"></a>

## Cross-references

- The [current constitution's lineage](/constitution/README.md#doc-constitution-lineage)
  preserves the is-tree, dolt-rs, and other antecedents. This pass does not claim
  a fresh audit of those earlier corpora; it carries their already-explained
  relevance through the current account.
- [Draft2 discussion](/constitution/naming/exploration/draft2-discussion0.sol56x.md#disagreements)
  remains the detailed naming disagreement ledger. This discussion changes the
  organizing question rather than replacing that evidence.
- [Skillification's disclosure ladder](/design/skillify/README1.sol56x.md#skillify-information-levels)
  supplies the anti-monolith delivery model; [modularization](/design/rekon/agents-modularization0.sol56x.md#agents-maintenance-skillify-ownership)
  applies it without silently deleting ambient obligations.
- [Watchman navigation synthesis](file:///home/rektide/src/opencode-watchman/.design/watchman/nav0-syn0.gpt56sol.md)
  supplies the map/route distinction and bounded assurance example. This is an
  explicitly local cross-project reference, not a portable publication claim.
- [OKF v0.2](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)
  constrains metadata and verification semantics. Its local archive copy was
  consulted; using its fields does not make the whole Rekon repository a
  conforming knowledge bundle.
