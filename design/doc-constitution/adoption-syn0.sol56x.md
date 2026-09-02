---
type: AdoptionSynthesis
title: Pilot adoption and upstream carry synthesis
description: Reconciliation of the independent Sol Max and Flash Max proposals for trial adoption, small-project migration proposals, and upstream-owned repositories.
resource: /design/doc-constitution/adoption-syn0.sol56x.md
tags: [documentation, constitution, adoption, upstream, migration, synthesis]
status: draft
generated: { by: "model:gpt-5.6-sol-max", at: 2026-09-02T01:12:23-04:00 }
extensions:
  ticket: rekon-doc-constitution-adoption
  local_knowledge_id: doc-constitution-adoption
sources:
  - id: sol-proposal
    resource: /design/doc-constitution/adoption0.sol56x.md
    title: Pilot adoption and upstream carry policy
    author: model:gpt-5.6-sol-max
  - id: flash-proposal
    resource: /design/doc-constitution/adoption0.glm53fm.md
    title: Adoption, migration restraint, and upstream intent
    author: model:glm-5.3-flash-max
  - id: human-decisions
    resource: urn:opencode:session:ses_fa19cd46bffeBW6dPrBzttf4Gv
    title: Prospective pilot, small-project proposal, and upstream carry decisions
    author: human:rektide
---

<a id="doc-constitution-adoption-synthesis"></a>
# Adoption And Upstream Carry Synthesis

Both independent proposals agree on the policy center:

1. The constitution governs new work prospectively while its broader migration
   value remains under assessment.
2. Fewer than 16 maintained documentation files enables a lightweight,
   read-only Flash Max proposal, not an automatic migration.
3. Human acceptance is the implementation gate.
4. Upstream-bound work follows upstream layout and tone.
5. Workspace-only work stays path-local so it is easy to filter and carry.
6. Skill and global machinery need not enter upstream-owned trees.
7. Adoption does not prune stale or undeclared artifacts.

<a id="doc-constitution-adoption-synthesis-sol"></a>
## Sol Max Strengths

[`adoption0.sol56x.md`](adoption0.sol56x.md) provides the stronger governing
model:

- It distinguishes **knowledge-contract adoption** from **layout migration**.
  A project can gain canonical ownership, honest status, anchors, and lineage
  without moving files.
- It names the **carry island**: one domain-grouped subtree that acts as an
  ownership and transport boundary for workspace-only work.
- It treats upstream README conventions as valid canonical surfaces and places
  private skill/global projections in workspace-owned overlays.
- It counts maintained documentation by role rather than one extension alone.
- It defines what the pilot must learn before migration becomes a target.

Its relative weakness is operational looseness: without a trigger and an exact
return contract, an agent could run proposals too eagerly or count the corpus
inconsistently.

<a id="doc-constitution-adoption-synthesis-flash"></a>
## Flash Max Strengths

[`adoption0.glm53fm.md`](adoption0.glm53fm.md) provides the stronger execution
guardrails:

- Classify upstream intent before counting or proposing.
- Dispatch only when adoption is under consideration, not whenever a small
  repository is opened.
- Record exact count commands, roots, exclusions, and raw results.
- Make the target repository read-only: no edits, moves, deletes, commits,
  symlinks, config changes, or pruning.
- Start migration proposals with zero new GLOBAL contribution; ambient context
  must be separately earned.
- Give read-only mirrors a valid refusal outcome: notes belong in a depending
  project or workspace knowledge home.
- Return inventory, authority map, path table, explicit non-actions, upstream
  plan, validation, and an acceptance checklist.

Its relative weakness is treating tracked `.md` as the primary definition of
documentation and treating upstream README/SKILL/GLOBAL semantics too
absolutely. A repository may use MDX, RST, AsciiDoc, package READMEs, or a
different canonical index. Our knowledge contract can map onto those surfaces
without imposing our filenames.

<a id="doc-constitution-adoption-synthesis-decisions"></a>
## Integrated Decisions

### Pilot force

The constitution is a prospective default for new modules and an active pilot
for existing corpora. Migration remains a hypothesis. Existing projects are
not compliance targets.

### Threshold and count

`<16` is a dispatch heuristic for the lightweight proposal path. Count
first-party, human-maintained documentation across relevant text formats,
resolve symlinked canonical documents once, exclude generated/vendor/scratch
trees, and report the exact inventory. At 16 or more, stop the lightweight path;
deliberate human-led planning remains available.

### Trigger and mutation boundary

Run Flash Max in the background when a human or coordinating session is
considering adoption for a qualifying project. The proposal may write one
model-suffixed scratch artifact but treats the target corpus as read-only. No
proposal authorizes implementation.

### Upstream fit

Map knowledge contracts onto upstream-native files when contributing upstream.
Keep local activation and ambient projections in overlays unless upstream
adopts them. A read-only mirror receives no local migration proposal.

### Carry island

When upstream intent is absent or uncertain, contain workspace work in one
declared domain subtree, commonly `design/<topic>/`, `.design/<topic>/`, or
`doc/<topic>/`. Containment makes path filtering, upstream rebasing, transport,
and deliberate removal tractable without turning the subtree into a flat dump.

### Ambient and cleanup defaults

A migration proposal begins with no GLOBAL fragment. Skill and ambient exposure
are separate decisions. Adoption never prunes; stale findings are reported for
human disposition.

### Decision memory

Not every exploratory proposal needs a ticket. When a proposal is requested for
review, crosses sessions, or produces an acceptance/rejection decision, a beads
ticket is the durable handle. Because scratch is ignored, consequential results
must be summarized in the ticket or promoted into committed design evidence.

<a id="doc-constitution-adoption-synthesis-return"></a>
## Minimal Proposal Contract

A qualifying proposal returns:

1. exact counted inventory, roots, exclusions, and commands;
2. upstream-intent classification;
3. current authority and link map;
4. proposed module boundaries and a credible no-change option;
5. path-by-path changes and explicit non-actions;
6. separate skill and global decisions, with GLOBAL defaulting to none;
7. compatibility, carry, and upstream interaction plan;
8. ticket recommendation, validation plan, risks, and human acceptance choices.

This is enough to make migration reviewable without turning assessment into
automatic reconciliation.
