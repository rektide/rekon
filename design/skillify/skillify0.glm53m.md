---
name: skillify
description: Calve knowledge out of a monolithic AGENTS.md or long doc into self-explaining modules. Use when splitting global context into topics, deciding what earns ambient GLOBAL.md load, writing skill descriptions, or adding SKILL.md symlinks.
type: Pattern
title: Skillify — calving knowledge into modules
resource: /design/skillify/skillify0.glm53m.md
tags: [documentation, skills, agents, context, modules, global-fragments]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-09-02T02:43:31-04:00 }
extensions:
  ticket: rekon-agents-maintenance-skillify
  local_knowledge_id: skillify
  qualified_knowledge_id: rekon-skillify
  skill: none yet — README + SKILL.md symlink come at promotion, on acceptance
  global_fragment: none (deliberate; default-zero for new modules)
sources:
  - id: constitution
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution
    author: model:gpt-5.6-terra with human direction
  - id: synthesis
    resource: /design/doc-constitution/syn0.gpt56t.md
    title: Constitution synthesis and deferred tool questions
    author: model:gpt-5.6-terra
  - id: assembler-doc
    resource: /constitution/agents-assembly.md
    title: AGENTS.md assembly manifest and CLI
    author: agent:flash with review fixes
  - id: writing-great-skills
    resource: file:///home/rektide/.agents/skills/writing-great-skills/SKILL.md
    title: Skill-writing vocabulary (context load, routing, progressive disclosure)
    author: human:matt-pocock
  - id: monolith
    resource: /AGENTS.md
    title: The global agents monolith being carved
    author: human:rektide
---

<a id="skillify"></a>
# Skillify

**Skillifying** is calving knowledge out of a monolith — usually the global
`AGENTS.md` — into **self-explaining knowledge modules**: directories that own a
canonical `README.md`, may expose it on demand as a skill, and may contribute a
small ambient fragment to assembled global context.

It is the operational companion to the
[constitution](/constitution/README.md#doc-constitution). The constitution says what a
module *is*; this pattern says when and how to *make* one.

> One body, three exposures: **read** the README, **load** the SKILL,
> **assembly** carries the GLOBAL.

<a id="skillify-when"></a>
# When To Skillify

Skillify a section when it has become a *topic*: a cohesive domain a reader or
agent could name in two words (`jj`, `beads`, `waves`, `rust-deps`). Apply the
tiering tests from the constitution's
[ambient cut](/constitution/README.md#doc-constitution-context-ambient):

| Question | If yes |
|---|---|
| Is it needed in most sessions, and does violating it corrupt work before any skill loads? | A few lines belong in a `GLOBAL.md` fragment. |
| Is it deep, stable, and needed *when the topic comes up*, not always? | The body belongs in a module README behind a `SKILL.md`. |
| Is it scratch, one-off, or transcript-like? | Leave it in `.test-agent/`; no machinery. |

Do not skillify to shrink the monolith for its own sake. A section that is
cheap, universal, and dangerous to miss (`jj commit` discipline, never push)
stays ambient. Skillifying moves knowledge *closer to its topic*, not out of
sight.

<a id="skillify-anatomy"></a>
# Module Anatomy

```text
<topic>/                     # design/<topic>/ while developing, doc/<topic>/ when stable
  README.md                  # canonical: the one current authority
  SKILL.md -> README.md      # optional: on-demand exposure (semi-uniform, see below)
  GLOBAL.md                  # optional: earned ambient contribution (default: absent)
  <support docs>             # evidence, guides, long write-ups
  log.md / index.md          # when history or size demand them
```

Only `README.md` is required. `SKILL.md` and `GLOBAL.md` are capabilities a
module *earns*; neither follows automatically from creating the directory.

<a id="skillify-skill"></a>
# The Semi-Uniform SKILL.md Pattern

`SKILL.md` is **uniform in form, semi in presence**:

- **Form is fixed.** Wherever it exists it is exactly a *relative* symlink to
  the canonical README — `ln -s README.md SKILL.md` — never a wrapper, copy, or
  second body. Humans and routed agents read the same bytes.
- **Presence is earned.** Add it when on-demand routing value is concrete: an
  agent working an unrelated topic should be able to skip this knowledge, and an
  agent entering the topic should be able to load it. Skip it for indexes,
  superseded evidence, and modules too small to route.

A skill-exposed README must carry routing metadata in its frontmatter — `name`,
a `description` written *for routing* (see below), and an honest `status`
(`draft` modules are loadable but flagged unstable). `doc/` already practices
this: [`/constitution/SKILL.md`](/constitution/SKILL.md) → `/constitution/README.md`.

<a id="skillify-descriptions"></a>
# Descriptions Route; Bodies Do

The old open question — "should skill descriptions carry a quick intro that can
standalone?" — resolves as a division of labor:

| Surface | Job | Failure mode |
|---|---|---|
| `description` frontmatter | **Routing**: when to load, what's inside. Front-load the topic's leading word. | A miniature doc: bloats the catalog without improving routing. |
| `GLOBAL.md` fragment | **Doing**: the few lines an agent must obey *before* anything loads. | Restating the README; ambient creep. |
| `README.md` body | **Depth**: method, examples, tradeoffs, maintenance. | Duplicating what a support doc should own. |

Write the description as triggers ("Use when splitting global context...",
"Use when adding SKILL.md symlinks..."), not as a summary of conclusions.

<a id="skillify-global"></a>
# GLOBAL Fragments

A `GLOBAL.md` is a claim on every agent's attention in every session. It must
pass the constitution's corruption / cheapness / universality tests, and it
carries assembler metadata (stripped from output):

```yaml
---
id: jj-workflow        # unique, kebab-case
order: 200             # unique integer; gap-numbered (100s, 200s, ...)
source: doc/jj/README.md
status: draft          # draft | stable
---
```

Defaults that keep the budget honest:

- **Default-zero.** A new module contributes no `GLOBAL.md` until ambient need
  is demonstrated. This module itself deliberately has none.
- **Point, rarely restate.** A pointer plus one or two guardrails beats a
  compressed copy of the README.
- **Fragments assemble; they are not the monolith relocated.** See
  [agents-assembly](/constitution/agents-assembly.md) for the manifest, provenance
  markers, and `rekon agents --check`.

<a id="skillify-procedure"></a>
# Procedure

1. **Name the topic.** Two or three words; this becomes the module id and the
   anchor root (`jj` → `jj`, `doc-constitution` → `doc-constitution`).
2. **Test the tier.** When/ambient/load/scratch — the table above. Write the
   verdict down; it decides everything after.
3. **Distill, don't move.** Write a canonical README that synthesizes the
   section's current truth (promotion-is-synthesis). Cite the monolith section
   and any prior design material as sources. Fix links to the new home.
4. **Decide exposure.** Add `SKILL.md` if routing value is concrete. Add
   `GLOBAL.md` only if the ambient tests pass — usually *not* on day one.
5. **Trim the monolith.** Replace the carved section with, at most, a pointer.
   The monolith shrinks last, after the module exists and links.
6. **Ticket if lifecycle-worthy.** Multi-session carve programs get a beads
   epic; a single clean extraction needs none.
7. **Validate.** Pandoc parses; anchors unique and adjacent; local links
   resolve; `git diff --check` clean; `rekon agents --check` if a manifest
   covers it. Commit the module and the monolith trim together.

<a id="skillify-examples"></a>
# Worked Examples

- **`doc/` (documentation topic).** The constitution module:
  `README.md` canonical, `SKILL.md` symlink, `GLOBAL.md` at order 100 carrying
  only the universal rules. The exemplar.
- **beads (planned).** Command patterns stay ambient (violating them corrupts
  tickets); epics/supersede/rename-recovery depth moves to a module skill.
- **jj (planned).** Workflow norms (`commit as you go`, never push, no squash)
  stay ambient; revset/template depth already exists on demand at
  [`~/archive/doc/jj.md`](file:///home/rektide/archive/doc/jj.md) and can be
  wrapped by a thin module rather than rewritten.

<a id="skillify-anti"></a>
# Anti-Patterns

- **Description-as-quick-ref** — rebuilding the monolith inside the skill
  catalog.
- **Forked bodies** — a `SKILL.md` that is a copy, summary, or "agent version"
  of the README. The symlink is the whole point.
- **Ambient creep** — every module feeling entitled to a GLOBAL fragment.
  Absence is the default and usually correct.
- **Ticket-per-section** — prose does not need lifecycle coordination.
- **Big-bang migration** — carve one section at a time, monolith stays honest
  and assembled output stays trustworthy.

<a id="skillify-status"></a>
# Open Questions

- Which harnesses discover project-directory skills, and how? (Symlink states
  the contract; discovery is external.)
- Should assembled `AGENTS.md` keep the monolith's familiar section headings as
  anchors for inbound references?
- When does a `design/` module's skill earn promotion to `doc/`?
- Does the carve want a manifest per repository or one workspace manifest?

<a id="skillify-cross-references"></a>
# Cross-References

- [/constitution/README.md](/constitution/README.md#doc-constitution) **governs** this pattern:
  module contract, namespace, ambient tests, promotion.
- [/constitution/agents-assembly.md](/constitution/agents-assembly.md) **implements** the
  assembly half: manifests, provenance, `rekon agents`.
- [syn0.gpt56t.md](/design/doc-constitution/syn0.gpt56t.md) **defers** the tool
  questions this pattern inherits.
- [writing-great-skills](file:///home/rektide/.agents/skills/writing-great-skills/SKILL.md)
  **supplies** the vocabulary: context load, routing, progressive disclosure,
  duplication, sediment.
- [/AGENTS.md](/AGENTS.md) **is the carve target**; each carved section should
  cite its new module home.
