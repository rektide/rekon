---
type: AdoptionRevision
title: Adoption revision 1 - per-repository counting, design/ spelling, tacit deviations
description: Records the three human decisions that close the synthesis open questions and the amended canonical clauses integrated into the constitution.
resource: /design/doc-constitution/adoption1.glm53m.md
tags: [documentation, constitution, adoption, upstream, carry, revision]
status: stable
generated: { by: "model:glm-5.3-max", at: 2026-09-02T02:00:44-04:00 }
extensions:
  ticket: rekon-doc-constitution-adoption
  local_knowledge_id: doc-constitution-adoption-revision
sources:
  - id: adoption-synthesis
    resource: /design/doc-constitution/adoption-syn0.sol56x.md
    title: Pilot adoption and upstream carry synthesis
    author: model:gpt-5.6-sol-max
  - id: sol-proposal
    resource: /design/doc-constitution/adoption0.sol56x.md
    title: Pilot adoption and upstream carry policy
    author: model:gpt-5.6-sol-max
  - id: flash-proposal
    resource: /design/doc-constitution/adoption0.glm53fm.md
    title: Adoption, migration restraint, and upstream intent
    author: model:glm-5.3-flash-max
  - id: canonical-constitution
    resource: /doc/README.md
    title: Self-Explaining Documentation Constitution
    author: model:gpt-5.6-terra
  - id: human-decisions
    resource: urn:opencode:session:ses_f9f646c2cffe1hZGwZx4AhinVI
    title: Alignment decisions on counting scope, carry-island spelling, and tacit deviations
    author: human:rektide
---

<a id="doc-constitution-adoption-revision"></a>
# Adoption Revision 1

The [synthesis](adoption-syn0.sol56x.md) left the adoption policy sound but with
three deliberately unsettled points. The human settled all three on 2026-09-02.
This revision records those decisions and the amended clauses now integrated
into [`/doc/README.md`](/doc/README.md#doc-constitution-adoption). The wave-0
proposals and synthesis remain untouched history.

<a id="doc-constitution-adoption-revision-count"></a>
## Decision 1: Count Per Repository

**Decision.** The `<16` threshold counts the **repository as a whole**.
Documentation roots inside a monorepo are not counted separately.

**Why.** The heuristic asks whether one background agent can hold the entire
documentation surface in one pass. Applying it per documentation root would let
a large monorepo qualify three times over while exceeding what one pass can
honestly cover. Per-repository counting routes monorepos to deliberate
human-led planning, which is the conservative outcome a trial wants.

**What changed.** Canonical wording moves from "a project with fewer than 16
maintained documentation files" to "a repository with fewer than 16 maintained
documentation files," states the whole-repository scope explicitly, and refers
to repositories rather than projects in the counting and borderline rules.

<a id="doc-constitution-adoption-revision-spelling"></a>
## Decision 2: `design/` Everywhere

**Decision.** The default carry-island root is the undotted
`design/<topic>/`, including in upstream-owned repositories. One spelling.

**Why.** The workspace was inconsistent: all five lineage constitutions use
`.design/`, while rekon and the global AGENTS.md say `design/`. A single
undotted spelling matches the global guidance, stays visible in listings, and
ends the ambiguity. If `design/` collides with a real upstream convention,
another subtree is chosen — see Decision 3 for how that choice lives.

**Grandfathering.** Existing `.design/` trees (is-tree, dolt-rs, execsnoop-rc,
opencode-otel, first-compute) remain exactly as they are. This is a
prospective default, not a migration; renaming those trees would break durable
anchors for zero benefit.

<a id="doc-constitution-adoption-revision-deviations"></a>
## Decision 3: Deviations Stay Tacit

**Decision.** Local deviations and island-root choices are implicit, tacit
knowledge. The constitution does not require stating them.

**Why.** The phrase "explicitly documented local equivalent" was an agent
invention from the wave-0 drafting, not a human requirement. In practice the
choice is obvious from the repository and the work; demanding a statement
would add noise and one more thing to maintain. Permission to document a
deviation remains (a MAY in the scope section), but no requirement exists, and
the canonical adoption clauses no longer use "documented."

<a id="doc-constitution-adoption-revision-amended"></a>
## Amended Canonical Clauses

The following replacements are integrated into
[`/doc/README.md`](/doc/README.md#doc-constitution-adoption):

```markdown
| Upstream intent absent or uncertain | Use a domain-grouped **carry island**,
commonly `design/<topic>/`. If the name collides with an upstream convention,
choose another subtree. | Skill and global projections may live in the island
or a rekon-owned overlay. |
```

```markdown
A carry island contains workspace-specific design, research, agent guidance,
and support artifacts under one domain subtree. The default root is
`design/<topic>/`; older `.design/` trees remain valid where they already
exist. It remains README-led and domain-grouped rather than becoming a flat
dump.
```

```markdown
When adoption is under consideration for a repository with **fewer than 16
maintained documentation files**, a background Flash Max agent SHOULD inspect
the complete surface and propose a migration. The threshold counts the
repository as a whole: documentation roots inside a monorepo are not counted
separately. [...]

Count first-party, human-maintained documents across the text formats the
repository actually uses, [...]. At 16 or more, the lightweight path stops;
deliberate human-led planning is still available. Borderline repositories must
not manipulate exclusions to qualify.
```

<a id="doc-constitution-adoption-revision-crossrefs"></a>
## Cross-References

- [`adoption-syn0.sol56x.md`](adoption-syn0.sol56x.md) **posed** the open
  points this revision closes; its integrated decisions are otherwise
  unchanged.
- [`adoption0.sol56x.md`](adoption0.sol56x.md) **listed** the spelling and
  count-scope questions among its learn-in-pilot items; the human chose to
  settle them now.
- [`adoption0.glm53fm.md`](adoption0.glm53fm.md) **raised** per-repo versus
  per-root counting and the deviation-documentation question in its
  ambiguities section.
- [`/doc/README.md#doc-constitution-adoption`](/doc/README.md) **carries** the
  amended clauses as the canonical account.
- The grandfathered `.design/` constitutions (is-tree, dolt-rs, execsnoop-rc,
  opencode-otel, first-compute) **precede** this spelling decision and keep
  their layout.
