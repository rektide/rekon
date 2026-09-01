---
type: ConstitutionVision
title: Workspace document constitution vision
description: A self-explaining knowledge flow from exploration through stable documentation, on-demand skills, and selected ambient context, coordinated by durable links and tickets.
resource: /design/doc-constitution/draft0.sol56m.md
tags: [rekon, documentation, knowledge-modules, skills, agents, beads, hypermedia]
status: draft
generated: { by: "model:gpt-5.6-sol-medium", at: 2026-09-01T16:50:40-04:00 }
extensions:
  ticket: rekon-doc-constitution-authoring
  local_knowledge_id: doc-constitution
  qualified_knowledge_id: rekon-doc-constitution
  independence: No same-wave constitution-draft0 peer artifact was consulted.
sources:
  - id: constitution-brief
    resource: file:///home/rektide/src/rekon/.test-agent/doc-constitution/brief-for-sol.md
    title: "Brief for sol: the document constitution dialogue"
    author: project:rekon
  - id: constitution-lineage-map
    resource: file:///home/rektide/src/rekon/.test-agent/doc-constitution/README.md
    title: doc-constitution scratch lineage and decisions
    author: project:rekon
  - id: constitution-tickets
    resource: file:///home/rektide/src/rekon/.beads/issues.jsonl
    title: rekon document-constitution and AGENTS-maintenance tickets
    author: human:rektide
  - id: is-tree-constitution-v1
    resource: file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md
    title: System-state research document constitution
    author: model:gpt-5.6-terra
  - id: is-tree-constitution-v2
    resource: file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md
    title: System-state research constitution reconciliation and synthesis amendment
    author: model:gpt-5.6-terra
  - id: dolt-rs-constitution
    resource: file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md
    title: Dependency-Ordered Research Wiki Constitution
    author: model:openai-gpt-5.6-terra
  - id: first-compute-constitution-v1
    resource: file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md
    title: Goal-directed compute research document constitution
    author: model:gpt-5.6-terra
  - id: first-compute-constitution-v2
    resource: file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md
    title: Foundation-wave sealing and fair-synthesis amendment
    author: model:gpt-5.6-terra
  - id: execsnoop-constitution
    resource: file:///home/rektide/src/execsnoop-rc/.design/constitution/constitution.glm53.md
    title: execsnoop-rc topic-document constitution
    author: model:glm-5.3
  - id: opencode-otel-constitution
    resource: file:///home/rektide/src/opencode-otel/.design/otel/research/topic-document-constitution/topic-document-constitution.gpt56t.md
    title: OpenCode OTEL planning research constitution
    author: model:gpt-5.6-terra
  - id: robustness-constitution
    resource: file:///home/rektide/src/chrome-agent-platform-fox/docs/CONSTITUTION.md
    title: Chrome Agent Platform Robustness Constitution
    author: project:chrome-agent-platform-fox
---

<a id="doc-constitution"></a>
# Workspace Document Constitution Vision

This is an independent draft for dialogue, not the accepted constitution. It asks what
the project constitutions were reaching toward when viewed from the scale of the whole
workspace.

The answer is not a larger research template. It is a **self-explaining knowledge
flow**. A question can be explored, argued over, stabilized, loaded as an on-demand
skill, and selectively made ambient without being rewritten into four unrelated forms.
Markdown documents remain the durable substrate throughout. Tickets coordinate the
work around them. Links preserve the explanation across time.

The workspace should make this path legible:

```text
question or pressure
        |
        v
design/<topic>/     building, exploring, learning, comparing
        |
        | synthesis or acceptance, with provenance retained
        v
doc/<topic>/        stable, user-facing current knowledge
        |
        +------ SKILL.md -> README.md ------> on-demand context
        |
        +------ GLOBAL.md ------------------> deterministic AGENTS.md

tickets surround the flow: scope, dependency, lifecycle, ownership, acceptance
links cross the flow: identity, evidence, inheritance, contrast, and history
```

`design/` and `doc/` are not merely stages in a publishing pipeline. Either may become
a knowledge module, expose itself as a skill, and contribute a small global fragment.
The distinction is the posture of the knowledge: `design/` is allowed to investigate;
`doc/` promises a maintained, user-facing account.

<a id="doc-constitution-principles"></a>
## Governing Principles

| Principle | Consequence |
|---|---|
| One body, several exposure surfaces | `README.md` is canonical; skill and global exposure must not create competing prose authorities. |
| Durable identity outranks current layout | Semantic anchors survive heading renumbering, prose edits, and reasonable reorganization. |
| Current understanding and historical evidence are both valuable | Canonical READMEs stay coherent while model-suffixed drafts and accepted-source links preserve how the conclusion formed. |
| Context is a budget | Only short, frequent, costly-to-miss rules become ambient; depth remains one skill load away. |
| Tickets coordinate; documents explain | Work state, dependencies, and acceptance live in tickets. Concepts, rationale, evidence, and durable guidance live in Markdown. |
| Process is proportional | Strong wave, seal, numbering, and validation patterns are available without burdening every note or edit. |
| Claims and relationships are named | Sources establish evidence; links explain inheritance, constraint, contrast, extension, or another material relationship. |
| Local differences remain local | A workspace constitution supplies common mechanics without turning one repository's layout or dispatch policy into universal law. |

The constitution should make the cheap, general rules easy to obey and make the more
elaborate patterns easy to discover. It should not demand research-program ceremony
from a two-paragraph user guide.

<a id="doc-constitution-module"></a>
# The Knowledge Module

The primary unit is a domain-grouped directory, not an isolated Markdown file. A module
has one canonical human and agent-readable body and may expose selected projections of
that body to different consumers.

```text
design/<topic>/ or doc/<topic>/
  README.md                 canonical current body and landing page
  SKILL.md -> README.md     optional relative symlink for skill loading
  GLOBAL.md                 optional concise ambient contribution
  log.md                    optional update history
  <supporting artifacts>    evidence, waves, experiments, fixtures, decisions
```

A directory becomes a knowledge module when its `README.md` deliberately identifies
the topic, audience, posture, sources, and navigation. `SKILL.md` and `GLOBAL.md` add
exposure capabilities; they are not required merely to make a directory legitimate.
When present, however, their meanings are fixed by this architecture.

<a id="doc-constitution-module-readme"></a>
## `README.md`: Canonical Knowledge

`README.md` is the maintained answer to “what is this topic, what should I understand
or do now, and where does the detail live?” It is simultaneously:

- the human landing page;
- the stable current tip for ordinary links;
- the body loaded when the module is used as a skill;
- the authority from which any ambient fragment is selected;
- the map to exact historical, evidentiary, and experimental artifacts.

Canonical does not mean exhaustive or immutable. It means that conflicting guidance is
resolved here rather than split between a README and a separately maintained skill
body. Supporting files can own detailed evidence or a specific historical argument.
The README should link to them instead of absorbing the whole corpus.

A knowledge-module README should carry trustworthy OKF-style frontmatter: identity,
one-line scope, honest status, authorship or generation, and sources actually consulted.
It must not claim `verified` merely because its author reread it. If it is exposed as a
skill, the same frontmatter must also provide enough routing information for a skill
loader without making the human opening read like tool metadata.

<a id="doc-constitution-module-skill"></a>
## `SKILL.md`: Exact On-Demand Exposure

`SKILL.md` is a **relative symlink** to `README.md`. It is not a copy, wrapper, summary,
or second editorial surface.

```text
SKILL.md -> README.md
```

The symlink makes a strong promise: humans arriving through conventional repository
navigation and agents arriving through skill routing receive the same knowledge. A
module that cannot serve both audiences from one body should improve its progressive
disclosure and supporting links rather than fork its authority.

Skill metadata should answer when the module should be loaded and what questions it can
answer. The body should answer those questions. Supporting research and longer guides
remain ordinary linked Markdown under the module.

<a id="doc-constitution-module-global"></a>
## `GLOBAL.md`: Selected Ambient Contribution

`GLOBAL.md` is the small fragment a module contributes to the deterministically
assembled workspace `AGENTS.md`. It is deliberately not a symlink to the README. Global
context has a different cost model: every agent pays for every byte whether or not the
topic is relevant.

A global fragment should contain only rules that are:

- frequently needed across the workspace;
- expensive to miss before an agent knows to load a skill;
- concise and immediately actionable;
- broadly applicable rather than repository-specific by accident;
- stable enough not to make ambient behavior oscillate;
- traceable to a canonical README section for rationale and exceptions.

The fragment should stand alone operationally but link back for depth. It should not
carry long history, extensive examples, specialist troubleshooting, or every option.
Those belong in the canonical README reached through `SKILL.md`.

Assembly should be reproducible from declared fragment identity and order. The same set
of fragments must produce the same `AGENTS.md`; duplicate identities, unstable ordering,
or unmarked hand edits should fail validation rather than produce context that depends
on filesystem accident. Generated sections should retain source provenance so a reader
can find the owning module.

<a id="doc-constitution-module-support"></a>
## Supporting Artifacts And Indexes

The canonical README is not required to contain every proof, alternative, or run log.
Supporting artifacts can include model-suffixed wave files, source research, ADRs,
fixtures, diagrams, experiments, and maintenance notes. Their names should reveal their
role, and durable claims in the README should link to the exact artifact and section
that supports them.

`index.md` remains available as a directory listing for progressive disclosure when a
module has enough artifacts to need one. It does not displace the README as the module's
conceptual landing page and current account. `log.md` records concise update history; it
does not become another concept document.

<a id="doc-constitution-lifecycle"></a>
# Design And Documentation Lifecycle

The `design/` to `doc/` distinction describes epistemic and audience posture, not file
quality.

| Location | Primary posture | Reader promise |
|---|---|---|
| `design/<topic>/` | Build, explore, learn, compare, falsify, and decide | The material exposes the live argument, alternatives, and uncertainty. |
| `doc/<topic>/` | Explain stable current knowledge to users and maintainers | The material is a supported entry point and distinguishes remaining uncertainty from accepted guidance. |

Both are durable and citable. `design/` is not a trash directory, and `doc/` is not a
claim of eternal finality.

<a id="doc-constitution-lifecycle-design"></a>
## Design Is The Learning Surface

A design module may begin with only a README that frames the problem and links evidence.
As needed, it can grow independent drafts, adversarial reviews, experiments, synthesis,
and decision records. Its canonical README should explain the current state of the
inquiry rather than pretending all supporting artifacts agree.

Design work may expose a skill when agents need the active method or research corpus on
demand. It may contribute a global fragment when a still-evolving practice is already
frequent and costly to miss, but volatility raises the threshold: ambient context should
not become a live transcript of an unsettled design.

<a id="doc-constitution-lifecycle-doc"></a>
## Documentation Is The Stable User Surface

A documentation module owns the maintained explanation a user or implementer should
trust first. It should state scope, current contract, examples or operational guidance,
known limits, and material relationships. It should not erase the design record that
made the guidance credible.

Stable documentation can still change. The promise is not immobility; it is that changes
are integrated coherently, source relationships remain navigable, and old semantic
anchors are not silently reassigned.

<a id="doc-constitution-lifecycle-promotion"></a>
## Promotion Is Synthesis, Not A File Move

Knowledge is ready for `doc/` when there is a useful maintained account for a stable
audience, not merely when a design file has aged. Promotion should:

1. Identify the accepted claims, operating guidance, and intended readers.
2. Preserve unresolved questions and confidence boundaries honestly.
3. Synthesize competing design artifacts rather than copy one draft by default.
4. Establish durable semantic anchors and explained cross-references.
5. Link exact design sources so the reasoning history remains inspectable.
6. Update the design README to point to the stable account when that improves navigation.
7. Decide skill exposure and global contribution separately from promotion.

The normal result is a new or updated `doc/<topic>/README.md` that cites the design
corpus. The source artifacts stay where they were. Existing exact links therefore keep
their historical meaning, while ordinary readers gain a stable current tip.

The flow is intentionally non-mandatory. Straightforward stable knowledge may originate
directly in `doc/`. Some investigations may remain valuable in `design/` indefinitely.
Promotion is a change in knowledge contract, not a graduation ritual.

<a id="doc-constitution-references"></a>
# Durable Markdown Referentiability

The workspace should behave like a durable, plain-Markdown wiki. A reader should be able
to quote a section, follow its sources, discover why another document links to it, and
return years later without depending on one renderer's generated heading slugs.

<a id="doc-constitution-references-anchors"></a>
## Semantic Anchors

Substantive, linkable sections use an explicit HTML anchor immediately before the
heading:

```markdown
<a id="example-recovery-policy"></a>
## Recovery Policy
```

Canonical anchor IDs should:

- use lowercase ASCII words, digits, and hyphens;
- describe durable meaning rather than a heading number;
- begin with the module's local root ID, then refine it semantically;
- remain stable when display text, numbering, or nearby structure changes;
- have one semantic owner across canonical project surfaces;
- never be silently moved to an unrelated idea.

When a section moves, preserve an alias or short tombstone at the old target when it
still has meaningful inbound links. A model-suffixed historical artifact remains
citable by exact file and anchor even after the canonical README advances.

Display coordinates such as `D4.2` remain useful for formal research programs, printed
discussion, or mechanically checked corpora. They are an available pattern, not the
workspace-wide identity system. Semantic anchors and links carry durable identity.

<a id="doc-constitution-references-links"></a>
## Links Carry Relationships

Use standard Markdown links. Prefer bundle-root-relative links within a repository and
canonical repository resources for cross-project published material. For local-only or
uncommitted evidence, an absolute `file://` reference is acceptable when explicitly
labeled as local WIP.

A useful link says why the target matters. The lineage's relationship vocabulary is a
good editorial aid:

| Relationship | Meaning |
|---|---|
| inherits | Uses the target's contract without redefining it. |
| implements | Supplies a concrete realization of the target. |
| consumes | Calls, queries, renders, or interprets the target. |
| motivates | Explains why this mechanism or decision exists. |
| constrains | Correctness depends on an invariant established by the target. |
| contrasts | Intentionally differs in identity, lifetime, or semantics. |
| extends | Adds behavior or state above the target. |
| evidences | Supplies source findings, tests, traces, or measurements. |

These words sharpen prose; they need not become mandatory edge metadata. Inventing a
relationship to make the corpus look connected is worse than recording that no material
edge was found.

<a id="doc-constitution-references-evidence"></a>
## Source References And Knowledge References

Source references and knowledge references do different work:

- A source reference grounds a claim in code, tests, primary documentation, a command,
  or another inspected artifact.
- A knowledge reference connects one maintained argument or contract to another.
- A ticket reference connects knowledge to work state, dependencies, and acceptance.

Documents should distinguish **Observed**, **Derived**, **Inference**,
**Recommendation**, and **Hypothesis** where confusing them would change a decision.
Not every sentence needs a badge. Important claims need enough local context and exact
source identity that another reader can reproduce the distinction.

Trustworthy frontmatter is an identity and orientation surface, not claim-level proof.
Listing a source means it was consulted; it does not imply that every claim derives from
every source. A ticket records intent and acceptance, not implementation evidence.

<a id="doc-constitution-namespace"></a>
# The Shared Ticket And Anchor Namespace

Beads tickets and canonical semantic anchors should share one project knowledge
namespace. This lets a ticket forward-declare knowledge, a section grow into tracked
work without renaming, and a reader move between explanation and lifecycle without an
external mapping table.

The mapping is:

```text
project prefix:       rekon
local knowledge ID:   doc-constitution
Beads ticket ID:      rekon-doc-constitution
HTML anchor ID:       doc-constitution
qualified ID:         rekon-doc-constitution
resolvable location:  /doc/README.md#doc-constitution
```

The **local form** omits the project prefix because the repository already supplies that
context. The **globally qualified cross-project form** prepends the registered project
prefix with a hyphen and is therefore the same shape as a full Beads ticket ID:

```text
<project-prefix>-<local-knowledge-id>
```

For globally resolvable citations, pair that qualified ID with the canonical repository
URL and exact path-plus-anchor. Do not infer the project/local boundary by splitting on
the first hyphen; resolve it against the known project-prefix registry.

<a id="doc-constitution-namespace-induction"></a>
## Inductive Refinement

A child section may refine an existing local ID by appending a semantic segment:

```text
foo-bar                 existing ticket or section
foo-bar-baz             child section
acme-foo-bar-baz        optional globally qualified ticket/knowledge ID
```

For this module:

```text
doc-constitution
doc-constitution-module
doc-constitution-module-global
```

This is an inductive naming affordance, not a requirement that every intermediate ID
have a ticket or even a heading. If `foo-bar-baz` later needs lifecycle coordination,
the project can create ticket `<prefix>-foo-bar-baz` without changing the anchor.

Lexical ancestry also does not replace Beads dependency edges. If the child ticket is
actually part of, blocked by, or supersedes another ticket, record that relationship in
Beads. The name makes the relationship legible; the ticket graph makes it operational.

<a id="doc-constitution-namespace-ownership"></a>
## Ownership, Aliases, And Historical Drafts

At canonical tips, one qualified knowledge ID has one semantic owner. Two current
READMEs must not use the same project-local anchor for different concepts. A moved alias
may repeat the ID only to route explicitly to that owner.

Independent wave artifacts are a deliberate exception in representation, not meaning.
They may propose matching anchors while exploring the same topic, but citations to them
must include the exact model-suffixed file. Acceptance assigns canonical ownership in a
README; it does not retroactively make one draft the input another author read.

An unticketed anchor still occupies the knowledge namespace. Creating a same-named
ticket later attaches work to that meaning; it must not reuse the ID for different work.

<a id="doc-constitution-namespace-forward"></a>
## Forward Anchors

A ticket may reserve and name a path-plus-anchor before the document exists:

```text
Forward anchor: /doc/example/README.md#example-recovery
```

Forward anchors are promises about intended knowledge, not evidence that the knowledge
already exists. They should be labeled as forward references in ticket prose. The
ticket's acceptance criteria should say what makes the target real and useful. Once the
document lands, the same ID becomes an ordinary durable reference.

Do not create empty documents solely to eliminate a deliberate forward reference. The
open ticket already reveals that the surface is planned and unfinished.

<a id="doc-constitution-tickets"></a>
# Balanced Ticket Coordination

Tickets reveal dimensions that prose alone handles poorly: work surface, dependency,
ownership, lifecycle, priority, blockers, and acceptance. Documents reveal dimensions
tickets handle poorly: concepts, evidence, rationale, examples, contracts, and durable
navigation. The constitution should preserve this complement rather than forcing either
system to imitate the other.

There is no one-to-one cardinality rule:

- one ticket may produce or update several sections;
- one section may support several tickets;
- a document may have no ticket when it is straightforward stable knowledge;
- a ticket may close without a durable document when the work creates no lasting
  explanation;
- a local anchor may exist before, after, or without a corresponding ticket.

<a id="doc-constitution-tickets-criteria"></a>
## When To Ticket Document Work

Create or use a ticket when one or more of these are material:

| Ticketing is useful when | Ticketing is usually noise when |
|---|---|
| Work spans sessions, agents, or review boundaries. | A local edit is obvious, bounded, and immediately verifiable. |
| Dependencies or blockers affect sequencing. | A subsection exists only to make an already-owned explanation readable. |
| Acceptance criteria distinguish done from merely drafted. | Every heading would receive a mechanically mirrored ticket. |
| A planned document or section needs a forward anchor. | A typo, link repair, or small clarification has no independent lifecycle. |
| Promotion changes the supported user-facing contract. | Stable reference material has no active work state to coordinate. |
| Ownership, risk, or disagreement should remain visible after chat context disappears. | Ticket creation would cost more interpretation than the change itself. |

The test is not “is this documentation?” It is “does explicit lifecycle coordination
make this knowledge work safer, more discoverable, or more finishable?”

When a ticket materially shapes a section, link them reciprocally when practical. The
ticket should point to the durable path or forward anchor; the document may link the
ticket for history and acceptance context. Closed tickets remain useful records, but the
stable document should not require a ticket database merely to explain itself.

<a id="doc-constitution-context"></a>
# Ambient And On-Demand Knowledge

The workspace has two different delivery economics:

| Surface | Cost | Best content |
|---|---|---|
| Assembled `AGENTS.md` | Paid by every agent on every task | Short operational defaults and costly-to-miss guardrails. |
| Skill-loaded `README.md` | Paid only when routing selects the topic | Rationale, exceptions, examples, advanced patterns, maintenance, and specialist workflows. |

The existence of a skill is not an argument for removing its topic from global context.
The existence of a global quick reference is not an argument for copying the whole skill
into `AGENTS.md`. A module can contribute to both at the appropriate depth.

<a id="doc-constitution-context-ambient"></a>
## The Ambient Cut

A rule earns global load when its expected cost of absence exceeds its repeated context
cost. Consider frequency, workspace reach, consequence if missed, actionability before
skill discovery, size, and volatility.

Good ambient material tells an agent what to do now or what not to do at a dangerous
decision point. It can name an available deeper pattern and link its module. It should
not attempt to teach the whole pattern.

The assembled `AGENTS.md` is an interface produced from module-owned fragments. Its
determinism and provenance matter as much as concision: agents must receive one stable
ordering, and maintainers must know which README owns each rule.

<a id="doc-constitution-context-ondemand"></a>
## The On-Demand Body

On-demand knowledge is not secondary or less authoritative. It is the canonical depth
that would be wasteful to load universally. A good module README begins with enough
orientation to route a human or agent, then progressively discloses method, examples,
tradeoffs, validation, maintenance, and sources.

Because `SKILL.md` is the README, skill descriptions should optimize for routing while
the body optimizes for understanding and action. There is no separate “skill version”
whose simplifications can drift from the human documentation.

<a id="doc-constitution-patterns"></a>
# Core Rules And Available Patterns

The workspace-wide core should remain cheap:

1. Give a knowledge module one canonical `README.md` body.
2. Use a relative `SKILL.md -> README.md` symlink for on-demand exposure.
3. Contribute only selected, traceable rules through `GLOBAL.md`.
4. Distinguish exploratory `design/` posture from stable user-facing `doc/` posture.
5. Use explicit semantic anchors and explained standard Markdown links.
6. Keep source identity, claim maturity, and verification status honest.
7. Let tickets coordinate lifecycle without requiring ticket parity for documents.
8. Preserve coherent current guidance and navigable historical evidence.

The deeper constitution or skill should advertise these opt-in patterns:

| Pattern | Reach for it when |
|---|---|
| Independent model-suffixed waves | Useful disagreement or independent coverage matters. |
| Same-wave no-peer-reading | Independence is part of the evidence, not merely a scheduling convenience. |
| Append-only refinement addenda | The evolution of one agent's confidence is itself valuable evidence. |
| Synthesis and adversarial waves | Multiple drafts must be reconciled and stress-tested rather than averaged. |
| Assignment manifests and wave seals | A large parallel corpus needs reproducible input boundaries. |
| Dependency-gated dispatch | A higher-layer author must inherit settled lower-layer contracts. |
| Display coordinates and prefix registries | A formal corpus benefits from compact reading coordinates in addition to anchors. |
| Claim-class tables | Fact, derivation, inference, recommendation, and hypothesis are easy to conflate. |
| Inherited/value-add/stop-boundary analysis | A dependent document risks copying prerequisites or absorbing another module's scope. |
| Corpus validation and doc-pass | Repeated structure and cross-links justify mechanical checks and reciprocal navigation. |

These patterns should be loadable and composable. None should become a universal ritual
merely because it was effective in a complex research wave.

<a id="doc-constitution-history"></a>
# History And Evolution

The lineage demonstrates a progression in the unit being governed.

<a id="doc-constitution-history-lineage"></a>
## From Product Invariants To Knowledge Flow

1. The Chrome Agent Platform robustness constitution used “constitution” for durable
   product invariants and independent review gates. It governs the product, not document
   mechanics.
2. `is-tree` v1 established the root document constitution: one model-suffixed artifact
   per assignment, OKF frontmatter, numbered coordinates plus semantic anchors,
   relationship-bearing doc-refs, claim classes, independent waves, and append-only
   refinement.
3. `first-compute` expanded the root into a complete research program: terminology and
   anti-conflation rules, shared comparison axes, viability judgments, foundation,
   refinement, synthesis, and adversarial waves. Its amendment added assignment
   manifests, immutable wave barriers, inherited-premise tracking, and fair synthesis.
4. `dolt-rs` independently built the richest wiki-oriented sibling: dependency-ordered
   topology, relationship vocabulary, inherited/value-add/stop boundaries, agent input
   and return packets, durable source policy, and mutable tips over immutable evidence.
5. `is-tree` v2 reconciled those siblings prospectively, adopting `Derived` claims,
   relationship language, layer contracts, durable sources, return packets, synthesis,
   and structural validation. Its open question asks whether the projects should share
   a constitution while retaining project-specific dispatch.
6. `execsnoop-rc` and OpenCode OTEL showed that the root mechanics could be adapted or
   slimmed for engineering and planning domains rather than copied at maximum weight.

The workspace constitution answers the reconciliation question by moving one level up.
Its common unit is not the numbered research draft but the self-explaining knowledge
module. Research-wave mechanics remain available inside that module. Project-specific
topic registries, dependency graphs, comparison axes, and dispatch rules remain local.

<a id="doc-constitution-history-evolution"></a>
## Mutable Tips Over Durable Evidence

Canonical READMEs should remain readable current accounts. Requiring every correction
to live forever as an appended amendment would preserve chronology at the cost of making
ordinary guidance unusable. Conversely, silently rewriting every historical draft would
erase the evidence of how a decision formed.

Use both layers:

- materially distinct proposals, independent drafts, and historical research remain
  new model-suffixed or otherwise identity-bearing artifacts;
- focused follow-up in the same authorship context may use an addendum when confidence
  evolution matters;
- accepted conclusions are integrated into the canonical README as coherent prose;
- `log.md`, version-control history, source links, and tickets record consequential
  changes and their rationale;
- semantic anchors remain stable, with explicit aliases or replacement notes when a
  concept moves;
- exact historical citations never silently advance merely because a canonical tip
  changed.

The constitution itself should follow this model. Significant contested amendments can
be developed under `design/`, reviewed, and then integrated into `doc/README.md`.
Evolution should be prospective unless correcting broken references, factual errors, or
materially misleading guidance. Existing documents need not be churned into compliance
with every new stylistic rule.

<a id="doc-constitution-tensions"></a>
# Tensions And Questions For Deliberation

This architecture resolves the broad direction but leaves important policy choices for
the human dialogue.

| Tension | Recommended lean | Remaining question |
|---|---|---|
| Canonical project-wide IDs vs duplicate independent drafts | Enforce unique ownership only at canonical tips; require exact file links for draft artifacts. | Should validation reserve every draft anchor or only canonical README anchors? |
| A coherent mutable README vs append-only constitutional history | Integrate accepted changes into the README; preserve proposals, logs, commits, and ticket links around it. | Which changes require an explicit design amendment before integration? |
| `GLOBAL.md` is selected prose but README is canonical | Require every ambient rule to identify its owning README anchor and validate provenance. | Should fragments be manually curated projections, generated excerpts, or support both forms? |
| Deterministic assembly vs local module autonomy | Give fragments declared identities and a stable assembly order; reject collisions. | Is order a central manifest, dependency graph, frontmatter key, or domain-grouped convention? |
| One README for humans and skill loaders | Put routing metadata in frontmatter and use progressive disclosure in the shared body. | What exact metadata contract keeps README rendering pleasant across repositories? |
| Full Beads-style qualification vs internet-global identity | Use `<project-prefix>-<local-id>` workspace-wide and pair it with canonical repository URL plus path/anchor externally. | Must project prefixes be registered workspace-wide to prevent collisions? |
| Ticket/anchor alignment vs bureaucratic parity | Treat names as interoperable reservations, not a one-ticket-per-section mandate. | Should tooling warn only on conflicting ownership, or also suggest missing reciprocal links? |
| Promotion vs physical relocation | Synthesize a stable README that cites design artifacts; do not move evidence merely for neatness. | When should stable documentation live beside code rather than under top-level `doc/`? |
| `README.md` as conceptual tip vs `index.md` as listing | Keep README explanatory and current; use `index.md` only when artifact enumeration needs its own surface. | Should large modules generate indexes mechanically? |
| Design modules may contribute ambient context | Permit it, but apply a higher volatility threshold and retain source status. | Should assembled global sections visibly mark design-origin fragments? |
| Prospective adoption vs corpus consistency | Do not mass-rewrite old documents; repair broken or misleading identity deliberately. | What minimum validator should new canonical modules pass before acceptance? |

Two questions are especially load-bearing:

1. **What is the assembly contract?** The three-file module architecture is strongest
   when `GLOBAL.md` identity, ordering, provenance, and validation are deterministic and
   simple enough to inspect without the assembler.
2. **What is the canonical-ID registry?** Beads already supplies project prefixes and
   ticket persistence. The constitution needs to decide whether scanning canonical
   READMEs is sufficient to enforce anchor ownership or whether a generated registry is
   worth maintaining.

<a id="doc-constitution-sources"></a>
# Exact Source References

These are local workspace sources inspected on 2026-09-01. They are cited with exact
paths and durable section anchors where available; line ranges identify the inspected
snapshot. They are local WIP references rather than claims about a published remote tip.

| Source | Exact sections inspected and used | Contribution to this vision |
|---|---|---|
| [`rekon` brief](file:///home/rektide/src/rekon/.test-agent/doc-constitution/brief-for-sol.md) | lines 7-23, 25-39, 40-72, 75-101 | Workspace purpose, verified lineage, earlier two-tier decisions, target sequencing, and the convergence question. |
| [`rekon` scratch README](file:///home/rektide/src/rekon/.test-agent/doc-constitution/README.md) | lines 13-46, 48-54, 57-110 | Human decisions, ticket program, chronological lineage, and shared constitutional DNA. |
| [`rekon` Beads records](file:///home/rektide/src/rekon/.beads/issues.jsonl) | tickets `rekon-doc-constitution`, `rekon-doc-constitution-authoring`, `rekon-doc-constitution-global`, `rekon-doc-constitution-skill`, `rekon-doc-constitution-rekon-skill`, and `rekon-agents-maintenance` | Latest canonical README/SKILL/GLOBAL architecture, forward anchors, balanced ticketing, and sequenced AGENTS maintenance. |
| [`is-tree` v1 D1](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-purpose), [`D2`](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-document-unit), [`D3`](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-frontmatter) | lines 24-35, 44-117 | Document identity, independent model artifacts, and trustworthy OKF frontmatter. |
| [`is-tree` v1 D4](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-heading-numbering), [`D5`](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-doc-ref), [`D6`](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-evidence-discipline) | lines 119-211 | Number/identity split, semantic anchors, relationship-bearing doc-refs, source-link scope, and claim classes. |
| [`is-tree` v1 D8.1](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-append-only-refinement), [`D11`](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-constitution-evolution) | lines 257-271, 311-323 | Append-only refinement, prospective evolution, and preservation of historical interpretation. |
| [`dolt-rs` W1](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-purpose), [`W2`](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-topology) | lines 24-135 | Wiki purpose, governing principles, domain-grouped topology, dependency-gated dispatch, and prefix registry. |
| [`dolt-rs` W3](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-coordinates), [`W3.4`](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-coordinates-link-semantics) | lines 136-215 | Durable semantic identity, standard Markdown portability, and relationship vocabulary. |
| [`dolt-rs` W4.4](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-content-layering), [`W5.2`](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-agent-workflow-output), [`W6.1`](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-evolution-artifacts) | lines 264-273, 320-356, 358-386 | Inherited/value-add/stop boundaries, return packets and doc-pass, immutable evidence, mutable tips, and renumbering policy. |
| [`first-compute` v1 D1.2](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-program-posture), [`D7`](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-terminology-discipline), [`D8`](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-comparison-axes) | lines 57-72, 245-340 | Useful disagreement, anti-conflation, shared comparison axes, and evidence-based viability judgments. |
| [`first-compute` v1 D10](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-wave-protocol), [`D13`](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-constitution-evolution) | lines 375-405, 491-502 | Foundation/refinement/synthesis/adversarial waves, addenda, and revision identity. |
| [`first-compute` v2 D15.1](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-assignment-manifest), [`D15.2`](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-wave-seal), [`D15.3`](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-fair-synthesis), [`D15.8`](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-canonical-doc-refs) | lines 35-108, 163-183 | Assignment isolation, reproducible wave boundaries, fair synthesis, exact immutable doc-refs, accepted tips, and cross-project resource identity. |
| [`is-tree` v2 D22](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-adopted-practices), [`D25`](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-synthesis-protocol), [`D27`](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-open-questions) | lines 38-123, 164-224 | Reconciliation of the root and sibling, `Derived` claims, layer contracts, durable sources, return packets, synthesis, validation, and explicit cross-project convergence question. |
| [`execsnoop-rc` C1-C10](file:///home/rektide/src/execsnoop-rc/.design/constitution/constitution.glm53.md#c-purpose) | lines 20-240 | Adaptation beyond pure research, multi-letter prefixes, engineering topic layout, and prospective revision. |
| [OpenCode OTEL D1-D7](file:///home/rektide/src/opencode-otel/.design/otel/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-purpose) | lines 24-114 | Evidence that the constitution can be slimmed to a 121-line planning contract while retaining identity, anchors, evidence, and argument shape. |
| [Chrome robustness constitution](file:///home/rektide/src/chrome-agent-platform-fox/docs/CONSTITUTION.md) | lines 1-7, 97-119 | Earlier workspace use of “constitution” for durable invariants and independent review; a conceptual ancestor, not a document-governance source. |

The strongest inheritance is therefore selective: retain the lineage's durable identity,
evidence honesty, relationship-bearing links, independent work, synthesis, and
prospective evolution. Do not universalize its project-specific heading coordinates,
topic registries, wave barriers, dependency graphs, or required closing templates.
