---
type: RekonMigrationDesign
title: Modularizing rekon's global agent context
description: A one-time migration design for decomposing rekon's monolithic global guidance into domain-owned modules while preserving ambient behavior before selectively activating depth on demand.
resource: /design/rekon/agents-modularization0.sol56x.md
tags: [rekon, migration, skills, agents, context, documentation, assembly]
status: draft
generated: { by: "model:gpt-5.6-sol-max", at: 2026-09-02T02:47:26-04:00 }
extensions:
  ticket: rekon-agents-maintenance-skillify
  local_knowledge_id: agents-maintenance-skillify
  independence: Written without reading design/skillify/README.md or any same-wave peer artifact.
sources:
  - id: human-direction
    resource: urn:opencode:session:ses_fa19cd46bffeBW6dPrBzttf4Gv
    title: GLOBAL fragments, README-led skills, selective calving, and independent skillify design direction
    author: human:rektide
  - id: document-constitution
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution
    author: project:rekon
  - id: agents-assembly
    resource: /constitution/agents-assembly.md
    title: Assembling AGENTS.md from GLOBAL.md fragments
    author: project:rekon
  - id: global-guidance
    resource: /AGENTS.md
    title: Workspace agent guidance
    author: human:rektide
  - id: skill-writing-reference
    resource: file:///home/rektide/.agents/skills/writing-great-skills/SKILL.md
    title: Writing great skills
    author: project:agent-skills
  - id: jj-reference
    resource: file:///home/rektide/archive/doc/jj.md
    title: jj/git branch spelunking cheat sheet
    author: project:archive-doc
  - id: skillify-ticket
    resource: /.beads/issues.jsonl
    title: rekon-agents-maintenance-skillify
    author: project:rekon
---

<a id="agents-maintenance-skillify"></a>
# Modularizing Rekon's Global Agent Context

This is the rekon-specific movement plan for turning the current AGENTS.md
manuscript into assembled, domain-owned context. It applies skillification
ideas to one repository and one migration; it is not the general definition of
skillification.

The target profile is general: a module has canonical `README.md`, exact
`SKILL.md -> README.md` activation, and an optional `GLOBAL.md` contribution
that is programmatically assembled into a parent scope's `AGENTS.md`. Rekon's
specific concerns are the movement into that profile: preserving a mature
monolith, its global symlink chain, ordering, parity, and costly-to-miss rules
while source ownership changes underneath it.

The movement is not principally about making AGENTS.md shorter. It gives each
domain a local owner, durable body, activation path, and explicit claim on
ambient attention.

The current monolith successfully makes important guidance hard to miss. Its
weakness is not merely token count: unrelated domains share one editing
surface, one history, one blame stream, and one undifferentiated context cost.
An eager extraction into skills would cure locality by creating a worse
failure: agents would stop seeing rules before they knew which skill to load.

The governing sequence is therefore:

> **Decompose ownership first. Change context behavior second.**

At first, almost everything may remain globally assembled. The first win is
that beads rules live beside beads knowledge, jj rules beside jj knowledge,
and documentation rules beside the document constitution. Selective calving
comes only after the assembled output proves that no behavior was lost and the
activation boundary has evidence behind it.

<a id="agents-maintenance-skillify-two-transformations"></a>
# Two Independent Transformations

<a id="agents-maintenance-skillify-ownership"></a>
## 1. Ownership Decomposition

Ownership decomposition moves source text out of the AGENTS.md manuscript into
domain-owned `GLOBAL.md` fragments. The assembler links those fragments back
into one global runtime image.

This transformation SHOULD initially preserve context behavior:

- the same operating rules remain ambient;
- their order remains deliberate;
- each rule gains fragment identity, canonical source, and local history;
- AGENTS.md becomes generated output rather than an editorial surface;
- every old line has a declared owner or an explicit disposition.

Ownership decomposition is worthwhile even if the assembled file remains the
same size. Data locality, provenance, review boundaries, and independent
evolution are already substantial gains.

<a id="agents-maintenance-skillify-activation"></a>
## 2. Activation Tiering

Activation tiering decides which owned knowledge remains ambient and which
depth becomes loadable through `SKILL.md` or support documents.

This transformation changes agent context and therefore requires stronger
evidence. A rule can leave GLOBAL only when:

1. an agent can reliably recognize the trigger before needing the rule;
2. the skill description routes that trigger without excessive global cost;
3. missing the rule before activation is unlikely to corrupt work;
4. the skill body or support document preserves the full operating meaning;
5. representative tasks show the new path is actually loaded and followed.

Do not combine these transformations into one giant rewrite. If context
behavior changes while ownership changes, a missing rule is difficult to
distinguish from a routing failure, assembly error, or editorial omission.

<a id="agents-maintenance-skillify-module"></a>
# The Skillified Knowledge Module

Skillification inherits the document constitution rather than inventing a
parallel prompt structure:

```text
<topic>/
  README.md                 canonical body and human landing page
  SKILL.md -> README.md     exact on-demand exposure
  GLOBAL.md                 selected ambient operating fragment
  <support documents>       branch-specific depth and evidence
```

The three primary files answer different questions:

| Surface | Question | Content posture |
|---|---|---|
| `README.md` | What is this domain, what do I do, and where is deeper material? | Canonical orientation, workflow, rules, exceptions, maintenance, and context pointers. |
| `SKILL.md` | How does an agent load that exact body? | A relative symlink, never a second maintained prompt. |
| `GLOBAL.md` | What must an agent know before it has loaded anything? | Cheap, universal, costly-to-miss actions plus a route to depth. |
| Support doc | What does only this branch or investigation need? | Detailed reference, examples, recovery, research, or maintenance behind a conditional pointer. |

Skillification does not require all three projections. A new module gets
`GLOBAL.md` only when it earns ambient cost. During decomposition of existing
AGENTS guidance, however, inherited ambient text begins in GLOBAL because its
cost already exists; reducing that cost is a later activation decision.

<a id="agents-maintenance-skillify-description"></a>
## Description Is Routing, Not A Quick Reference

A model-invoked skill description is always-loaded catalog context. It should
state when to load the module and name each genuinely distinct branch once. It
should not summarize the body's conclusions or carry commands that belong in
GLOBAL.

The division is:

```text
description  -> recognition and routing
GLOBAL.md    -> immediate ambient action
README.md    -> canonical method and understanding
support docs -> conditional depth
```

This resolves the apparent desire for a skill description that can stand alone:
it stands alone as a **router**, not as an operating manual. If an action must
work without loading the skill, it belongs in GLOBAL rather than an increasingly
large description.

<a id="agents-maintenance-skillify-invocation"></a>
## Invocation Is A Budget Choice

Use model invocation when the agent must discover the skill autonomously or
another skill must reach it. The description then spends global context on
every turn, so independent reach must earn that cost.

Use user invocation when the human will deliberately select the workflow. Set
`disable-model-invocation: true`; its description becomes a short human label.
If too many user-invoked skills become hard to remember, introduce a small
router skill rather than making every skill model-invoked.

Split by invocation only when a branch has its own recognizable trigger. Split
by sequence only when seeing later steps causes premature completion of the
current step. Domain grouping alone does not justify a new skill catalog entry.

<a id="agents-maintenance-skillify-disclosure"></a>
## Progressive Disclosure Follows Branches

Keep steps every invocation needs in the README. Move reference behind a
context pointer when only one branch needs it. The pointer's wording MUST state
when and why to load the target; a bare “see also” link does not route context.

Definitions, rules, caveats, and examples for one concept stay co-located. Do
not scatter a concept across GLOBAL, README, and several support files merely
to make each file short. Deliberate ambient duplication is acceptable, but one
canonical section must own the meaning.

<a id="agents-maintenance-skillify-placement"></a>
# Placement Decision

Every candidate rule or passage is classified independently. Whole AGENTS
sections are convenient migration units, not proof that all their contents
belong on one tier.

<a id="agents-maintenance-skillify-placement-tests"></a>
## Placement Tests

| Test | Question |
|---|---|
| Corruption | Can missing this before routing damage work, provenance, safety, or collaboration? |
| Frequency | How often does it change an ordinary workspace decision? |
| Universality | Does it apply across projects, languages, and task types? |
| Cheapness | Can it change behavior in a few actionable lines? |
| Routability | Can a description reliably recognize the need before the rule is required? |
| Depth | Does correct use require examples, branches, recovery, or substantial reference? |
| Volatility | Is it stable enough for ambient behavior, or still under active design? |
| Locality | Is it workspace-wide, project-local, upstream-native, or specific to one tool/domain? |

<a id="agents-maintenance-skillify-placement-outcomes"></a>
## Placement Outcomes

| Outcome | Use when |
|---|---|
| GLOBAL only | The rule is cheap and costly to miss, while the model already knows enough depth. |
| GLOBAL plus skill | A quick operating rule is costly to miss, but correct depth needs a workflow or reference. |
| Skill only | The trigger is reliable, absence before routing is safe, and depth is too large or specialized for ambient context. |
| Skill plus support docs | Distinct branches need substantial reference that would obscure the common workflow. |
| External reference via module | Another corpus owns the canonical depth; a local README owns routing and workspace-specific adaptation without copying it. |
| Project-local guidance | The rule follows a project or upstream convention rather than workspace law. |
| No instruction | The line is a no-op, duplicate, obsolete, or unsupported recommendation, after deliberate review. |

No-op or stale findings are proposals, not automatic cleanup. Ownership
decomposition does not prune. Removal requires a human-readable rationale and
parity ledger entry.

<a id="agents-maintenance-skillify-procedure"></a>
# Skillification Procedure

<a id="agents-maintenance-skillify-procedure-inventory"></a>
## 1. Inventory The Runtime Contract

Parse the current AGENTS.md into addressable passages. For each passage record:

- source heading and line range;
- rule or concept owner;
- current reason for being ambient;
- dependencies on neighboring passages;
- candidate module and tier;
- disposition: preserve, relocate, refine, or propose removal.

The inventory is a **parity ledger**. Every current operating instruction must
appear once. This prevents attractive new module prose from quietly dropping
awkward but important guardrails.

<a id="agents-maintenance-skillify-procedure-domains"></a>
## 2. Choose Domain Owners

Choose domains that can maintain their own meaning, not one directory per
heading. A good module owns a vocabulary, workflow, and change cadence. A bad
module is merely a place to put ten lines.

Proposed ownership for the current AGENTS surface:

| Current area | Candidate owner | Initial ambient posture |
|---|---|---|
| Subagent dispatch and return packets | `subagents/` | Preserve globally; collaboration can be corrupted before routing. |
| Scratch, variants, archive/wiki paths | `workspace/` or `rekon/` | Preserve globally; agents need these before knowing where to search or write. |
| Documentation, waves, doc-pass, OKF | `doc/` | Quick invariants global; full wave and doc-pass methods on demand. |
| Planning and coding defaults | `planning/` and `coding/` | Mostly global because short and broadly applicable. |
| Rust conventions and preferred libraries | `rust/` | Initially global; later test whether language detection routes reliably enough for depth. |
| JavaScript/TypeScript conventions | `typescript/` | Initially global; larger CLI/build recipes are candidates for skill depth. |
| jj/git and jj-hunk | `jj/` | Guardrails and commit behavior global; DSL, hunk surgery, and forensics on demand. |
| Beads | `beads/` | Stable-ID/dependency semantics and common commands global; recovery and graph rewrites on demand. |
| Web components | `webcomponents/` | Candidate skill unless frequent work shows ambient value. |
| Shell environment behavior | `shell/` | Preserve globally; missing process/environment facts cause immediate errors. |

These are hypotheses for synthesis, not a required directory list. Adjacent
areas may combine when they share invocation and ownership; a domain may split
when branches have genuinely different triggers.

<a id="agents-maintenance-skillify-procedure-modules"></a>
## 3. Build Canonical Modules

For each owner:

1. Write or identify its canonical README.
2. Preserve source history and cite the AGENTS passages it inherits.
3. Add skill-compatible routing metadata only if on-demand activation is useful.
4. Add `SKILL.md -> README.md` as a relative symlink when exposed.
5. Add support docs for branch-specific depth rather than copying external
   references into the body.
6. Start GLOBAL with the inherited ambient passage during decomposition; do
   not optimize it yet.

When working in upstream-owned repositories, follow upstream layout for
upstream-bound content. Keep workspace-only activation machinery in a carry
island or rekon-owned overlay.

<a id="agents-maintenance-skillify-procedure-assemble"></a>
## 4. Assemble A Shadow Runtime

Create an explicit preview manifest with every fragment. Use `rekon agents` to
assemble a non-production output; do not overwrite AGENTS.md yet.

The preview must pass:

- unique fragment identity and order;
- valid GLOBAL frontmatter and one leading H1;
- no fragment-relative links that would retarget;
- provenance and UTF-8 cost reporting;
- deterministic repeated assembly;
- complete parity-ledger coverage.

Generated markers and provenance comments will change bytes, so parity is not
raw file equality alone. Compare the normalized operating prose and heading
order, then review deliberate wording changes separately.

<a id="agents-maintenance-skillify-procedure-cutover"></a>
## 5. Cut Over Ownership In One Reviewed Change

Once every old passage has a source fragment, land the production manifest and
generated AGENTS.md together. The cutover commit marks AGENTS.md generated and
establishes `node rekon.ts agents --check` as the drift gate.

Avoid a prolonged dual-authority phase. Until cutover, AGENTS.md remains
canonical and preview fragments should not receive unrelated editorial changes.
After cutover, fragments are canonical and hand edits to output are stale.

<a id="agents-maintenance-skillify-procedure-tier"></a>
## 6. Calve Depth One Domain At A Time

Only after ownership cutover should a domain reduce GLOBAL. For each reduction:

1. identify the exact removed ambient passage;
2. show where its full meaning lives in README or support docs;
3. make the description recognize the required trigger;
4. retain any costly-to-miss quick action in GLOBAL;
5. run positive, negative, and boundary routing scenarios;
6. inspect assembler context-cost change;
7. record acceptance or rollback in the domain ticket.

This produces small, attributable behavior changes. If a task regresses, its
domain reduction can be reversed without undoing ownership decomposition.

<a id="agents-maintenance-skillify-examples"></a>
# Worked Placement Examples

<a id="agents-maintenance-skillify-example-beads"></a>
## Beads: Quick Work Graph, Deep Recovery

Beads illustrates GLOBAL plus skill:

**GLOBAL SHOULD retain:**

- stable human-readable IDs and project prefixes;
- child IDs namespaced beneath epics;
- dependency direction (`A` depends on `B`);
- search-before-create, claim, verify, and close behavior;
- the few common create/update/dep/show commands;
- the prohibition on `bd prime` and requirement to preserve history.

**README/SKILL SHOULD own:**

- linked-work and supersede procedures with full examples;
- graph audits and ambiguity handling;
- rename and broad JSONL/database recovery;
- export modes, Dolt internals, and unusual failure diagnosis;
- migration of documentation tickets and forward anchors.

An agent can create and update ordinary work from GLOBAL. It loads the beads
skill when designing a graph, repairing identity, or operating an uncommon
lifecycle branch.

<a id="agents-maintenance-skillify-example-jj"></a>
## jj: Ambient Guardrails, External Forensics

jj also illustrates GLOBAL plus skill plus external reference:

**GLOBAL SHOULD retain:**

- commit proactively in logical groups with explicit paths;
- never push;
- do not amend, edit, squash, or destructively reset without direction;
- preserve unrelated work;
- consult `jj help` before guessing revset or template syntax;
- the compact log command and core `jj commit` form.

**README/SKILL SHOULD own:**

- when to use normal commits versus `jj-hunk`;
- the exact hunk discovery, dry-run, and commit procedure;
- common revset/template vocabulary and troubleshooting;
- a context pointer to the stable
  [`jj.md`](file:///home/rektide/archive/doc/jj.md) for branch forensics,
  verified git equivalents, absent jj capabilities, and failure traps.

The module README owns routing and workspace policy. The archive reference
continues to own the detailed verified cheat sheet; skillification does not
copy it into a second authority.

<a id="agents-maintenance-skillify-validation"></a>
# Validation And Evaluation

<a id="agents-maintenance-skillify-validation-static"></a>
## Static Contract

Validate every module for:

- README existence and honest routing/status frontmatter;
- relative SKILL symlink target and no duplicate body;
- GLOBAL identity, order, source, status, Markdown structure, and link safety;
- support-doc context pointers with branch conditions;
- local and canonical cross-reference targets;
- one semantic owner for each instruction;
- assembler determinism and no hand-edited output.

<a id="agents-maintenance-skillify-validation-routing"></a>
## Routing Scenarios

For every model-invoked skill, define:

- positive prompts for each distinct branch;
- neighboring negative prompts that should not load it;
- boundary prompts where GLOBAL must suffice before routing;
- tasks that require following a support-doc pointer;
- failure cases where missing activation would corrupt work.

The completion criterion is behavioral: the skill loads when needed, remains
absent when irrelevant, and changes the agent's process in the intended way.
A description that reads well but does not route is not done.

<a id="agents-maintenance-skillify-validation-budget"></a>
## Context Budget

Use assembler per-fragment and total UTF-8 costs to make changes visible. Do
not set an arbitrary hard cap during the pilot. A reduction is valuable only
if routing remains reliable; a globally retained line is justified when its
absence costs more than its repeated load.

Descriptions also spend global catalog context even though they are not in
AGENTS.md. Count new model-invoked descriptions when evaluating net savings.
Skillification that moves 200 bytes out of AGENTS and adds 500 bytes of catalog
triggers is a context regression.

<a id="agents-maintenance-skillify-tickets"></a>
# Ticket Coordination

Skillification is coordinated work when it crosses sessions, changes ambient
behavior, adds a production fragment, or needs routing acceptance. Use an epic
for the domain and child tickets for independently acceptable tracer slices.

Useful forward anchors include:

```text
/beads/README.md#beads-recovery
/jj/GLOBAL.md
/typescript/README.md#typescript-cli
```

Do not create a ticket per AGENTS paragraph or README heading. The parity ledger
can map many passages to one module ticket. Tickets carry lifecycle,
dependencies, and acceptance; documents carry the operating knowledge.

<a id="agents-maintenance-skillify-failures"></a>
# Failure Modes

| Failure | Symptom | Response |
|---|---|---|
| Extraction equals deletion | AGENTS shrinks before routing works. | Restore inherited GLOBAL text; separate ownership from activation. |
| Distributed monolith | GLOBAL fragments are local but total context keeps growing invisibly. | Review assembler costs and require each ambient line to pass placement tests. |
| Description as manual | Catalog text accumulates commands and conclusions. | Return action to GLOBAL and depth to README; keep routing branches only. |
| Skill shadow body | SKILL.md copies or wraps README. | Replace it with a relative symlink and one canonical body. |
| Hidden depth | README says “see file” without when/why. | Write a conditional context pointer tied to a branch. |
| Trigger confetti | One skill lists many synonyms for the same branch. | Collapse to one leading word and one trigger per branch. |
| Premature fragmentation | Tiny skills have no independent invocation or ownership. | Recombine domains; split only by invocation or sequence. |
| Upstream imposition | Workspace files scatter through an upstream-owned tree. | Use upstream-native content and workspace overlays/carry islands. |
| Automatic cleanup | Migration removes stale-looking material. | Report it; leave disposition to an explicit human decision. |
| Dual authority | Humans edit AGENTS and GLOBAL during a long transition. | Shorten cutover; declare one current source and check generated output. |

<a id="agents-maintenance-skillify-decisions"></a>
# Proposed Decisions

1. Skillification is a two-stage program: ownership decomposition, then
   evidence-based activation tiering.
2. Most current guidance remains ambient at the first cutover.
3. README is canonical, SKILL is its relative symlink, and GLOBAL is the
   selected operational projection.
4. Skill descriptions route; they do not replace GLOBAL quick references.
5. New modules default to no GLOBAL, while extracted ambient rules retain their
   current global status until deliberately tiered.
6. Beads and jj are the first worked pilots because they exercise the
   quick-reference/deep-reference boundary differently.
7. A production manifest lands only with complete parity-ledger coverage and a
   generated AGENTS cutover.
8. Context reduction is measured across AGENTS fragments and model-invoked
   descriptions, not AGENTS bytes alone.

<a id="agents-maintenance-skillify-open"></a>
# Questions For Synthesis

1. Which top-level domain owns cross-cutting workspace paths and experimentation:
   `workspace/`, `rekon/`, or a smaller set of modules?
2. Should the first production cutover preserve current heading text exactly,
   or allow domain-normalized headings in the same reviewed change?
3. How are project-local module skills discovered without committing
   workspace-only machinery into upstream repositories?
4. Should routing scenarios become machine-readable fixtures, or remain a
   documented human/model review protocol during the pilot?
5. How should externally canonical docs such as `~/archive/doc/jj.md` be
   installed or addressed portably across machines?
6. Which current AGENTS passages are genuinely no-ops or sediment, and which
   merely look verbose because their failure cost is rare but severe?
7. Does each language deserve a model-invoked skill, or can code/file context
   activate language depth through a shared router without catalog sprawl?

<a id="agents-maintenance-skillify-cross-references"></a>
# Cross-References

- The [document constitution](/constitution/README.md) **defines** README/SKILL/GLOBAL,
  ambient admission, design/doc lifecycle, ticket anchors, and upstream carry
  boundaries inherited here.
- The [AGENTS assembler](/constitution/agents-assembly.md) **implements** deterministic
  fragment identity, ordering, provenance, validation, and context-cost
  reporting required for ownership cutover.
- [`README.sol56x.md`](/design/skillify/README.sol56x.md) **defines** the
  general skillification practice and the Markdown README/SKILL/GLOBAL-to-parent
  host profile that this one-time migration applies.
- [`writing-great-skills`](file:///home/rektide/.agents/skills/writing-great-skills/SKILL.md)
  **supplies** invocation load, branches, progressive disclosure, co-location,
  leading words, completion criteria, and skill failure vocabulary.
- [`jj.md`](file:///home/rektide/archive/doc/jj.md) **evidences** an external
  stable reference whose verified depth should activate on demand rather than
  be copied or loaded globally.
- Beads ticket `rekon-agents-maintenance-skillify` **tracks** the eventual
  accepted pattern and declares the beads/jj examples as acceptance criteria.
