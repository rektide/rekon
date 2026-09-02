---
type: AdoptionProposal
title: Pilot adoption and upstream carry policy
description: An independent Sol Max proposal for shaking out the documentation constitution, proposing migrations in small projects, and containing workspace knowledge when working in upstream repositories.
resource: /design/doc-constitution/adoption0.sol56x.md
tags: [documentation, constitution, adoption, upstream, migration, carry]
status: draft
generated: { by: "model:gpt-5.6-sol-max", at: 2026-09-01T22:14:48-04:00 }
extensions:
  ticket: rekon-doc-constitution-adoption
  local_knowledge_id: doc-constitution-adoption
  independence: Written without reading the same-wave Flash Max proposal.
sources:
  - id: canonical-constitution
    resource: /doc/README.md
    title: Self-Explaining Documentation Constitution
    author: model:gpt-5.6-terra with human direction
  - id: constitution-synthesis
    resource: /design/doc-constitution/syn0.gpt56t.md
    title: Self-explaining documentation constitution synthesis
    author: model:gpt-5.6-terra
  - id: adoption-direction
    resource: urn:opencode:session:ses_fa19cd46bffeBW6dPrBzttf4Gv
    title: Human decisions on pilot force, small-project proposals, and upstream carry burden
    author: human:rektide
---

<a id="doc-constitution-adoption"></a>
# Pilot Adoption And Upstream Carry Policy

The constitution is ready to govern new work, but it is not yet evidence that
existing projects should be reorganized around it. Those are different claims.

The immediate posture should be **prospective pilot**:

- use the constitution for new knowledge modules unless local context argues
  otherwise;
- observe how README/SKILL/GLOBAL modules behave in real projects;
- propose small migrations where the whole documentation surface can still be
  understood cheaply;
- collect friction before recommending a workspace-wide migration program;
- preserve upstream fit and minimize the burden of carrying local knowledge
  across upstream changes.

This is not hesitation about the architecture. It is evidence discipline. The
module model is a recommendation; corpus migration remains a hypothesis until
several projects demonstrate better discovery, references, activation, global
context cost, and maintenance.

<a id="doc-constitution-adoption-two-changes"></a>
## Two Changes Must Not Be Confused

Adoption can change either knowledge semantics or repository layout:

1. **Knowledge-contract adoption** gives a topic a canonical current account,
   honest status, durable anchors, source lineage, and explicit exposure rules.
2. **Layout migration** moves or renames files into README-led module
   directories.

A project can adopt the first without the second. An upstream-native `docs/`
tree can gain canonical ownership, stable anchors, and skill routing without
being reshaped into `doc/<topic>/`. Conversely, moving files into a preferred
layout without resolving authority or links is not meaningful adoption.

The constitution should recommend knowledge contracts strongly and layout
migrations cautiously.

<a id="doc-constitution-adoption-intent"></a>
# Establish Work Intent Before Choosing Layout

Repository ownership alone does not decide where knowledge belongs. The
decisive question is what relationship this work is expected to have with
upstream.

| Intent | Directory posture | Skill and global posture |
|---|---|---|
| Known upstream contribution | Fit the upstream project's existing docs, naming, and review conventions. Add the smallest native change that carries the knowledge contract. | Do not introduce workspace-only `SKILL.md` or `GLOBAL.md` files unless they are acceptable upstream concepts. Keep local activation overlays outside the upstream patch. |
| Upstream intent uncertain | Keep exploratory and agent-specific work in one workspace-owned domain subdirectory. Avoid scattering it through upstream-owned roots. | Treat skill/global exposure as local overlay behavior until upstream intent becomes clear. |
| Known workspace-local extension | Use the normal domain modules under `design/<topic>/` and `doc/<topic>/`, or a project-documented equivalent. | README/SKILL/GLOBAL may use the full constitution contract. |
| First-party project we control | Use the constitution prospectively, while preserving any stronger established local convention. | Module exposure may be native project structure. |

"Adapt to upstream" is not an exception to the constitution. It follows the
principle that local differences stay local and that layout is not identity.

<a id="doc-constitution-adoption-carry-island"></a>
## The Carry Island

When work is not known to be upstream-bound, prefer a **carry island**: one
declared, domain-grouped subtree that owns our workspace-specific design,
research, agent instructions, and supporting artifacts.

Common forms are:

```text
design/<topic>/
.design/<topic>/
doc/<topic>/
```

Choose one that does not collide with an upstream convention and state its
local meaning. The exact spelling matters less than containment.

Containment reduces carry burden because the subtree can be:

- included or excluded with one path filter;
- reviewed separately from upstream-compatible changes;
- moved between workspaces without reconstructing scattered files;
- rebased across upstream updates with fewer overlapping paths;
- removed deliberately if the local experiment is abandoned;
- promoted selectively when part of the work becomes upstream-worthy.

The carry island is not permission to create a flat dumping ground. It remains
domain-grouped, README-led, and referentiable. It is a boundary around ownership
and transport, not around quality.

<a id="doc-constitution-adoption-overlays"></a>
## Skills And Global Context In Upstream Trees

README/SKILL/GLOBAL is a knowledge-module contract, but every projection does
not need to be committed into every upstream repository.

<a id="doc-constitution-adoption-overlays-readme"></a>
### README

Prefer the upstream's canonical landing-page convention. If it uses
`docs/index.md`, package-level READMEs, or generated reference pages, map the
knowledge contract onto those surfaces instead of adding a parallel root.

<a id="doc-constitution-adoption-overlays-skill"></a>
### SKILL

If `SKILL.md -> README.md` would be foreign to the upstream project, expose the
upstream document through a workspace-owned overlay or installer. The overlay
must point at the canonical upstream body; it must not fork a private summary.

<a id="doc-constitution-adoption-overlays-global"></a>
### GLOBAL

Workspace-global agent instructions normally belong in a rekon-owned module,
not in an unrelated upstream repository. A local GLOBAL fragment can cite an
upstream canonical source while remaining outside the upstream patch. Commit a
GLOBAL fragment upstream only when the upstream project itself adopts that
ambient-agent contract.

This separation keeps our context runtime composable without asking upstream
maintainers to accept our entire delivery mechanism.

<a id="doc-constitution-adoption-small"></a>
# Small-Project Proposal Pilot

Projects with fewer than 16 maintained documentation files are tentative
migration candidates because one background agent can usually understand the
whole surface without lossy sampling. The threshold is a dispatch heuristic,
not a claim that the sixteenth file changes architecture.

The default action is to **propose**, never migrate.

<a id="doc-constitution-adoption-count"></a>
## Count Transparently

The proposing agent should count first-party, human-maintained documentation
that participates in project navigation or explanation, including:

- Markdown and MDX documents;
- reStructuredText or AsciiDoc when they serve the same role;
- root and package READMEs, contribution guides, ADRs, design notes, and user
  documentation;
- symlinked canonical documents counted once by resolved identity.

It should exclude:

- vendored dependencies and mirrors;
- generated API/reference output;
- build, cache, coverage, fixture, and package-manager directories;
- issue exports, transcripts, and machine logs unless the project treats them
  as maintained documentation;
- archived snapshots that are not part of current navigation.

The proposal must report the search roots, exclusions, file list, and final
count. Borderline counts should not be gamed: if exclusions hide a meaningful
corpus, the agent should say the project is not actually small.

<a id="doc-constitution-adoption-agent"></a>
## Background Flash Max Procedure

For a qualifying project, launch Flash Max in the background with read-only
instructions:

1. Read project-level agent guidance and identify upstream remotes, ownership,
   and evidence of intended contribution.
2. Inventory every counted document and the links among them.
3. Identify current canonical tips, duplicate authorities, replacement claims,
   dead ends, and renderer-dependent anchors.
4. Classify topics and audiences without forcing the inventory into a
   preselected directory tree.
5. Decide whether the existing layout can express the constitution without
   moving files.
6. If change helps, propose domain module boundaries, canonical READMEs,
   compatibility aliases, support documents, and promotion provenance.
7. Separately propose skill exposure and global contribution. Neither follows
   automatically from module creation.
8. State upstream intent and whether each proposed file belongs in an upstream
   patch, a carry island, or a rekon-owned overlay.
9. Return validation steps, risks, unresolved choices, and explicit human
   acceptance criteria.

The agent writes one model-suffixed proposal under a gitignored topic scratch
directory, commonly:

```text
.test-agent/doc-adoption/proposal0.<model>.md
```

It does not move, rename, rewrite, symlink, ticket, or prune project artifacts.
It does not treat the threshold as authorization. Its completion criterion is
a reviewable proposal with an exact inventory and a credible no-change option.

<a id="doc-constitution-adoption-return"></a>
## Required Proposal Return

The return packet should include:

| Item | Why it matters |
|---|---|
| Counted corpus and exclusions | Makes the `<16` dispatch decision reproducible. |
| Current authority map | Shows which documents are canonical, historical, duplicated, or ambiguous. |
| Upstream-intent assessment | Determines native layout, carry island, or overlay. |
| Proposed module map | Shows README ownership and support-document relationships. |
| Path-by-path change table | Makes moves, aliases, unchanged files, and link repairs reviewable. |
| Skill/global decisions | Prevents module creation from silently increasing context exposure. |
| Link and anchor impact | Preserves durable references and defines compatibility stubs. |
| Ticket recommendation | Identifies only work whose lifecycle benefits from tracking. |
| Validation plan | Defines link, frontmatter, symlink, build, and project-specific checks. |
| No-change alternative | Forces the proposal to prove migration is better than current structure. |

<a id="doc-constitution-adoption-acceptance"></a>
## Human Gate

After the background proposal returns, the human may:

- accept all or part of it;
- ask for another independent proposal or synthesis;
- adopt knowledge contracts without moving files;
- retain the existing layout and record why;
- create tickets for accepted multi-step work;
- decline the migration without creating cleanup work.

Only an accepted proposal authorizes implementation. Automatic reconciliation,
bulk renaming, and deletion of undeclared files remain out of scope.

<a id="doc-constitution-adoption-assessment"></a>
# What The Pilot Must Learn

Before recommending general migrations, assess several real modules against
observable questions:

1. Do humans and agents find the canonical current account faster?
2. Does skill routing load useful depth without needing duplicated bodies?
3. Do GLOBAL fragments reduce monolith history while keeping ambient guidance
   coherent and affordable?
4. Do ticket-derived anchors improve navigation without creating naming or
   ticket bureaucracy?
5. Do design-to-doc promotions preserve provenance while yielding readable
   user guidance?
6. Do carry islands reduce path conflicts and filtering work during upstream
   updates?
7. Do upstream-intended contributions still look native to their projects?
8. Which rules produce repeated friction, exceptions, or validation needs?

Record findings as design evidence. Promote migration from tentative
recommendation to target only after the evidence shows repeatable benefit and
the assembler/validator can support the contract without hidden manual work.

<a id="doc-constitution-adoption-clause"></a>
# Proposed Canonical Clause

The following is the concise policy I would integrate into the constitution:

> This constitution is a prospective workspace default under active pilot. It
> governs new knowledge modules but does not yet mandate migration of existing
> documentation. For a project with fewer than 16 maintained documentation
> files, a read-only background Flash Max agent SHOULD inventory the complete
> surface and propose a migration, including a credible no-change option;
> implementation requires explicit human acceptance. When work is intended for
> upstream, preserve the upstream project's native directories and conventions
> and map the knowledge contract onto them. When upstream intent is absent or
> uncertain, contain workspace-specific knowledge in one documented,
> domain-grouped subtree so it remains easy to filter, carry across upstream
> updates, or remove deliberately. Skill and global projections MAY live in
> workspace-owned overlays rather than being imposed on upstream trees.

<a id="doc-constitution-adoption-open"></a>
# Questions To Learn Rather Than Pre-Decide

The pilot should reveal, rather than guess:

- whether `<16` should count only current navigable docs or all maintained
  historical design evidence;
- whether `.design/` or `design/` is the better default carry-island root in
  upstream-owned repositories;
- how overlay skills point durably at upstream documents across checkout paths;
- whether GLOBAL fragments should name the source repository, module, or both;
- how much migration tooling can validate without turning proposals into
  automatic reconciliation;
- when a successful local carry island should be upstreamed, retained, or
  retired.

These are assessment outputs, not blockers to the pilot.
