---
type: RekonCoreDraft
title: "Rekon core draft 3: explicit names, native planes, maintained relations"
description: A candidate core for Rekon using structured location identity, short explicit anchors, native authorities, typed continuity, and a root-level maintained home.
tags: [rekon-core, naming, identity, anchors, authority, work, actors, evidence, relations]
status: draft
generated: { by: "integrator/gpt-5.6-sol-max", at: 2026-09-04T10:30:31-04:00 }
stale_after: 2026-12-04T00:00:00-05:00
extensions:
  ticket: rekon-con-naming
  wave: rekon core draft 3
  model_suffix: sol56x
  routing_address: "openai/gpt-5.6-sol#max"
  session: urn:opencode:session:ses_f9369369bffe6dqU53mp5C6HKv
  actor_claim:
    name: integrator/gpt-5.6-sol-max
    status: self-proposed
    scope: rekon-con-naming
    hats: [drafter, lexicographer]
  revises: /constitution/naming/exploration/draft2.sol56x.md
  discussion: /constitution/naming/exploration/draft2-discussion0.sol56x.md
  human_direction: >-
    Favor visually explicit anchors backed by generation, update, and
    deduplication tooling; compare canonical grammars; name the system's aspects
    with well-known terms and short IDs; explain accepted tips; treat the work
    as Rekon's broad core and allow its maintained account to move upward.
sources:
  - id: discussion
    resource: /constitution/naming/exploration/draft2-discussion0.sol56x.md
    title: Draft 2 grammar, anchor, core-scope, and accepted-tip discussion
    author: integrator/gpt-5.6-sol-max
  - id: sol-draft2
    resource: /constitution/naming/exploration/draft2.sol56x.md
    title: "Naming draft 2: names at home, continuity in relations"
    author: integrator/gpt-5.6-sol-max
  - id: glm-draft2
    resource: /constitution/naming/exploration/draft2.glm53m.md
    title: Naming draft 2, the successor compact
    author: curator/glm-5.3-max
  - id: adversarial-review
    resource: /constitution/naming/exploration/draft2-review0.sol56x.md
    title: Naming draft 2 adversarial review
    author: openai/gpt-5.6-sol-max
  - id: ruling
    resource: /constitution/naming/exploration/proposal4.glm53m.md
    title: Naming revision 4 human override
    author: steward/glm-5.3-max
  - id: ask-audit
    resource: /constitution/naming/exploration/ask-audit0.glm53m.md
    title: Naming ask audit revision 0
    author: steward/glm-5.3-max
  - id: constitution
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution
    author: model:gpt-5.6-terra
  - id: beads-study
    resource: /constitution/naming/exploration/beads-offerings0.sol56x.md
    title: Beads system offerings for Rekon's naming core
    author: model:gpt-5.6-sol-xhigh
  - id: okf
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
    title: Open Knowledge Format v0.2
    author: project:knowledge-catalog
---

<a id="core"></a>

# Rekon Core Draft 3

Rekon is a **federated named workspace**. It joins knowledge, work, actors,
evidence, computation, publication, and tools without forcing them into one
database or one punctuation scheme. Each native system keeps the identities and
lifecycle it can maintain. Rekon supplies the shared terms, explicit relations,
and guarded operations that make those entities one navigable system.

The compact promise is:

> **Name at the source. Keep one current owner. Cite explicit anchors. Preserve
> exact evidence. Relate every change. Derive the views.**

Naming is the identity kernel of this [core](#core), not its entire subject.
The maintained account should therefore become a root-level
[`home`](#home) for how Rekon works.

This draft actively uses its short explicit IDs. For example,
[`authority`](#authority), [`root`](#root), [`anchor`](#anchor),
[`channel`](#channel), and [`move`](#move) are local forms. Their current full
names compose this document's exact path with those fragments.

<a id="terms"></a>

# Core Glossary

These are ordinary domain terms, not a parallel collection of poetic law names.
Each short ID is an explicit local anchor. The containing path and
[`authority`](#authority) supply the rest of its canonical name.

<a id="authority"></a>

**Naming authority (`authority`).** The project steward or native system
entitled to maintain a canonical name in one scope. Across projects, authority
is identified by a canonical URI; a short project word such as `rekon` is an
[`alias`](#alias).

<a id="root"></a>

**Namespace root (`root`).** The stable path base from which an authority
interprets native artifact paths. Rekon's native root is the repository root.
This does not make the repository an OKF bundle.

<a id="member"></a>

**Knowledge member (`member`).** A version-controlled artifact admitted as
knowledge by non-empty `type` frontmatter or an explicit grandfathered-path
declaration.

<a id="path"></a>

**Canonical path (`path`).** The exact namespace-relative stored path,
including its filename suffix.

<a id="anchor"></a>

**Explicit anchor (`anchor`).** A source-visible fragment naming one durable
section. Heading text presents the section; the anchor preserves its identity.

<a id="name"></a>

**Canonical name (`name`).** The one current native identity an
[`authority`](#authority) assigns to an entity. For knowledge it composes
authority URI, [`path`](#path), and optional [`anchor`](#anchor).

<a id="alias"></a>

**Alias (`alias`).** A declared alternate reference with one recoverable owner
and explicit scope. Flat Beads-compatible spellings, old fragments, and short
project labels are aliases when they map to a canonical name.

<a id="locator"></a>

**Locator (`locator`).** A route to a representation or execution, such as a
checkout, mirror URL, session ID, or task handle. A value can be both a locator
and a name, but the roles remain distinguishable.

<a id="revision"></a>

**Revision binding (`revision`).** An immutable commit ID, content digest,
receipt, or signature binding an exact observation. A rewritable jj change ID
is lineage, not an exact revision.

<a id="channel"></a>

**Publication channel (`channel`).** A mutable named entity selecting the
currently published target. A branch and an accepted-tip symlink are channels.

<a id="tip"></a>

**Accepted tip (`tip`).** A channel selecting the currently accepted artifact
in a design series. The tip and selected artifact are different entities.

<a id="plane"></a>

**Native plane (`plane`).** A domain whose own system maintains canonical
identifiers and lifecycle, such as knowledge files, Beads work, actor
declarations, or version-control evidence.

<a id="bridge"></a>

**Cross-plane bridge (`bridge`).** An explicit relation or declared alias
connecting entities owned by different native planes.

<a id="idea"></a>

**Durable idea (`idea`).** An abstract concept with an authority-selected
maintained knowledge presence. Evidence artifacts discussing it remain separate
and connect through typed relations.

<a id="actor"></a>

**Actor (`actor`).** A human, agent, or process making an identity, authorship,
or verification claim.

<a id="hat"></a>

**Hat (`hat`).** A scoped alias by which one actor is addressed for a concern.

<a id="role"></a>

**Role (`role`).** A typed historical responsibility relation between an actor,
a capability, and a scope. A role is not another actor name.

<a id="work"></a>

**Work item (`work`).** A Beads-owned commitment with acceptance, dependency,
assignment, and lifecycle state.

<a id="move"></a>

**Identity migration (`move`).** A canonical authority, root, path, or anchor
change. A move is human-gated and never hidden as locator maintenance.

<a id="successor"></a>

**Successor (`successor`).** The current entity explicitly related to a retired
identity after rename, replacement, split, merge, extraction, or transfer.

<a id="tombstone"></a>

**Tombstone (`tombstone`).** A deprecated old-path knowledge presence that
states retirement and points to a successor when one exists.

<a id="home"></a>

**Maintained home (`home`).** The current public account readers should trust
first. It synthesizes accepted claims while citing exact evidence.

<a id="tool"></a>

**Naming tool (`tool`).** Guarded machinery that generates, validates,
migrates, deduplicates, removes, and indexes names without becoming their
authority.

<a id="accept"></a>

**Acceptance (`accept`).** The human decision selecting policy. Acceptance does
not silently perform implementation or imply source verification.

<a id="identity"></a>

# Knowledge Identity

Every version-controlled path on an authority's canonical line has an artifact
identity. A regular artifact becomes a knowledge [`member`](#member) when it
carries parseable frontmatter with non-empty `type`, or when the project
explicitly declares it as grandfathered knowledge.

The canonical current knowledge [`name`](#name) is:

```text
(canonical authority URI, exact namespace-relative path, optional explicit anchor)
```

For a future core section:

```text
authority: https://github.com/rektide/rekon
path:      /core/README.md
anchor:    authority
```

The common local route is `/core/README.md#authority`. The short in-document
form is `#authority`. A cross-project reference MUST carry the canonical
authority URI. The short `rekon:` form is contextual elision.

This grammar deliberately retains the suffix and structural delimiters. It
revises the ruling's flattened suffixless example because the [core](#core)
names Markdown, MDX, RST, code, generated views, and other artifacts. The
canonical representation must be reversible without a hidden segment map.

Within a declared OKF export, `/core/README.md` projects to the suffixless
Concept ID `core/README`. That is a typed OKF [`alias`](#alias), not Rekon's
second canonical name. Flat strings such as `rekon-core-authority` are likewise
declared cross-modal aliases, never mechanically privileged fallbacks.

The logical path preserves exact case and components. Serialized URLs use
standard percent encoding. New Rekon-owned paths reserve `#` for fragments and
avoid `.`/`..`, repeated separators, control characters, and ambiguous Unicode
normalization. Upstream-native paths remain valid through encoding rather than
forced renaming.

<a id="anchors"></a>

# Explicit Anchor Practice

Durable sections MUST use source-visible explicit [`anchors`](#anchor). The
preferred form uses a short known term and a fuller visible heading:

```markdown
<a id="authority"></a>

## Naming Authority
```

This is both readable and mechanically safe: the explicit ID is `authority`,
while the generated heading ID is `naming-authority`. The source does not
contain two rendered elements with one ID.

Headers do not silently mint canonical section names. The explicit anchor does.
The heading supplies presentation and a semantic correspondence that tools can
check. The containing [`path`](#path) supplies context. This draft demonstrates
the convention by using short anchors in the glossary and citing them throughout
the normative prose.

An anchor remains stable when the heading is reworded but the referent remains.
If the referent or canonical term changes, that is an identity
[`move`](#move), not routine formatting.

<a id="references"></a>

# Reference Family

One canonical [`name`](#name) can have several typed references:

| Reference | Purpose | Rule |
|---|---|---|
| Local fragment | Concise reference inside one document | Context restores authority and path. |
| Local root-relative link | Current navigation inside one project | Exact path and explicit anchor are present. |
| Cross-project name | Durable federated reference | Canonical authority URI is mandatory. |
| Alias | Cross-modal or compatibility spelling | Mapping, scope, and owner are declared. |
| Locator | Reach a checkout, mirror, session, or service | Must not silently claim canonical ownership. |
| Channel | Follow the current selected publication | Mutable selection is explicit. |
| Revision-bound citation | Cite exact evidence | Immutable revision, historical path, and fragment are retained. |

The reference family permits practical shorthand without violating one-name.
Ambiguity is reported rather than guessed. Search may rank candidates;
canonical resolution must not fuzzy-bind an old or partial string to a merely
similar entity.

<a id="channels"></a>

# Channels And Accepted Tips

A suffixless accepted [`tip`](#tip) is a canonical name for a mutable
[`channel`](#channel), not another name for every artifact it selects.

```text
adoption.md -> adoption1.glm53m.md
```

`adoption1.glm53m.md` names one exact model-attributed artifact.
`adoption.md` names the current accepted-adoption channel. Retargeting the
channel changes its state; it does not rename either target.

Use the model-suffixed target for exact argument or historical evidence. Use the
accepted-tip channel when the intended referent is "the accepted winner now."
Use a real maintained README for supported current prose. A maintained README
is normally not a winner symlink.

The channel's carrier must be tested. A filesystem symlink can be preserved,
dereferenced, ignored, or rendered differently by Git, archives, forges,
package tools, and browsers. Channel identity does not manufacture portable
symlink behavior.

<a id="planes"></a>

# Native Planes And Bridges

| Plane | Canonical owner | Native identity |
|---|---|---|
| Knowledge | Project authority through admitted file and anchor declarations | Authority URI plus path and optional anchor |
| Work | Beads | Explicit stable issue ID |
| Actors | Actor declaration accepted in a scope | Actor name; model and session remain separate |
| Evidence | Version control or evidence system | Immutable revision plus native coordinate |
| Computation | Sanctioned definition owner | Native computation or service identifier |
| Publication | Maintainer or selection process | Maintained home or named channel |

Shared words do not merge planes. `rekon-con-naming` is a Beads-owned
[`work`](#work) name. A knowledge document may declare it as an
[`alias`](#alias) or link to it through a [`bridge`](#bridge); neither operation
makes ticket closure verify the document.

Indexes, graphs, glossaries, and resolvers are derived views. When a derived
view disagrees with native state, it is stale or making an independent claim.
It does not silently become the authority.

<a id="ideas"></a>

# Durable Ideas And Relations

Location names a claim-bearing presence. It does not prove that similar prose
in several drafts denotes one abstract [`idea`](#idea). When an idea needs an
independent lifecycle and cross-document citation, the project authority MUST
select or create one maintained document or section as its canonical presence.

Other artifacts retain their own names and use familiar relationship words:

| Relation | Meaning |
|---|---|
| supports | Supplies evidence or argument for the idea |
| contrasts | Deliberately presents a different account |
| derives from | Uses the source to create a new presence |
| extends | Adds scope without replacing the source |
| supersedes | Becomes the maintained successor for the declared scope |
| copies | Reproduces content without claiming identity |
| mirrors | Presents one authority's content through another route |

An equivalence claim is allowed when sourced and stewarded. Similar terms or
same-wave convergence are not enough to infer it.

<a id="actors"></a>

# Named Actors

The root session of a naming or core program MUST self-name. A child agent that
produces durable work SHOULD self-name in its first durable artifact. Ephemeral
executions MAY remain producer-and-session identified when no future operation
needs to address them.

The actor and machine address form the requested Docker-style pair:

| Strand | Example | Meaning |
|---|---|---|
| Actor name | `integrator/gpt-5.6-sol-max` | Chosen continuity and accountability claim |
| Producer/model | `openai/gpt-5.6-sol#max` | What generated this execution |
| Session locator | `ses_f936...` | Exact resumption or transcript address |
| Hat | `lexicographer` | Scoped alias for this concern |
| Role | author of draft3 | Historical responsibility relation |

An [`actor`](#actor) self-declares its name and [`hats`](#hat). A receiving
authority decides whether those claims have local currency. A [`role`](#role)
records responsibility beside hats, not instead of them. Separate executions
are separate actors unless an accepted handoff asserts continuity.

Human names are voluntary scoped presences. Under OKF, `human:` denotes a
person actor and affects trust-tier classification; the prefix is not
authentication and must never be fabricated.

<a id="works"></a>

# Beads Work

Beads owns [`work`](#work): commitments, acceptance criteria, dependency,
readiness, assignment, and closure. Documents own explanation and evidence.
Findings that need prose live at explicit anchors; findings that need action
may also receive a linked Beads ID.

Beads rename repairs Beads-owned references. A knowledge rename uses the
identity-migration contract below. Cross-plane shared spelling is an explicit
[`bridge`](#bridge), not evidence that the work item and document are one
entity.

Ordinary mutations remain in audit and journal history. An event receives a
durable name only when a later decision or claim must cite the occurrence
itself.

<a id="moves"></a>

# Identity Migration

A canonical authority, [`root`](#root), [`path`](#path), or
[`anchor`](#anchor) change is an identity [`move`](#move). It MUST be human
approved and MUST use a sealed, immutable-revision-bound inbound inventory.

The migration record states:

1. old and new canonical names;
2. relation type, including non-bijective split or merge;
3. approving human and enactment commit;
4. controlled live references and exact historical references;
5. forward anchors, aliases, channels, generated views, and deep fragments;
6. successor, compatibility, or explicit loud-break decision; and
7. verification on every promised publication carrier.

Content and heading edits preserve identity when path and anchor remain and the
referent is recognizable. Copy, extraction, fork, and authority transfer create
new identities with explicit relations. An intact clone under one authority
changes only the [`locator`](#locator).

Rename cost is a decision-time factor, not a monotonic constitutional law.
Citations and promises can accumulate, consolidate, or retire. Measure before
the move; do not infer that an inexpensive move is valuable.

<a id="tombstones"></a>

# Tombstones And Compatibility

A regular Markdown [`tombstone`](#tombstone) at an old path is the current
deprecated presence of that old path identity. It is not the
[`successor`](#successor) and must not contain a competing current account. It
states retirement, relation, and the level of fragment compatibility promised.

Compatibility carriers are chosen deliberately:

| Carrier | Use |
|---|---|
| Controlled rewrite | Move a live citation to the successor |
| Retained old anchor | Preserve a fragment alias in one document |
| Regular tombstone | Preserve an old knowledge path with explicit disposition |
| Symlink | Expose a verified local alias or channel |
| Redirect | Preserve a host-level route with declared fragment behavior |
| Loud break | State that no compatibility promise remains |

The top-level OKF `resource` field names an underlying asset, not the
tombstone's own path. Self-location derives from the canonical path. Historical
evidence citations remain pinned to the old revision and are not rewritten to
pretend they originated at the successor.

No migration automatically prunes compatibility artifacts. Retirement and
removal remain deliberate stewarded changes.

<a id="tools"></a>

# Guarded Naming Tools

The first [`tool`](#tool) surface is explicit-anchor management, grounded in
declared authority and membership.

| Operation | Contract |
|---|---|
| Generate | Add a short explicit anchor to a durable heading and show the full resulting name. |
| Validate | Check syntax, adjacency, source ownership, resolved links, and one rendered target on every supported carrier. |
| Update | Keep anchors through display edits; use a human-gated migration when referent or canonical term changes. |
| Deduplicate | Identify the owner, remove or rename accidental duplicates, and preserve declared compatibility aliases. |
| Remove | Require retirement or evidence that no durable promise remains. |
| Inventory | Resolve Markdown, HTML, frontmatter, symlink, channel, generated-view, and Beads references at an immutable revision. |
| Index | Derive names, headings, aliases, relations, inbound references, and source revision without becoming authority. |

The tool reports ambiguity, collision, retirement, missing membership, broken
links, and unsupported carriers. It never repairs a canonical reference through
fuzzy search.

<a id="homes"></a>

# Root-Level Maintained Home

The maintained successor should be **`/core/README.md`**. The subject is the
Rekon [core](#core), while naming is its identity kernel. `/naming/README.md`
would understate the scope; the repository root README should remain available
for project orientation rather than absorbing all constitutional depth.

Implementation should:

1. synthesize accepted policy into `/core/README.md`;
2. leave model-suffixed exploration evidence at authored paths;
3. turn `/constitution/naming/README.md` into historical navigation or a
   tombstone with an explicit successor;
4. repair controlled current links while preserving revision-bound evidence;
5. update the constitution and ambient projection to point to the new home; and
6. avoid moving the historical subtree merely to make the tree look uniform.

The maintained home is a new identity that earns succession by being the best
current account. Promotion changes reach and authority; it does not rewrite
history.

<a id="acceptance"></a>

# Acceptance And Implementation

Human [`acceptance`](#accept) selects policy. It should explicitly decide:

1. exact structured paths instead of the ruling's suffixless flattened form;
2. canonical authority URI and repository-root native namespace;
3. artifact identity and knowledge membership admission;
4. short explicit anchors and the one-target carrier invariant;
5. named actors, hats, roles, and the Docker-style exact/chosen pair;
6. durable idea presences and typed relations;
7. accepted-tip channels as entities distinct from selected artifacts;
8. mandatory human-gated identity migration; and
9. `/core/README.md` as the maintained successor home.

Acceptance updates the decision record and `rekon-con-naming`. It does not
silently verify source claims or perform the migration.

A separate reviewed implementation plan owns the new home, constitution and
`GLOBAL.md` amendments, project declarations, grandfathered projections,
anchor tooling, relation representation, one bounded pilot, reciprocal links,
and wiring without bulk migration or pruning.

<a id="demonstration"></a>

# Active Naming Demonstration

This source contains short explicit IDs and uses them as references. Selected
examples:

| Local ID | Current full draft identity | Used by |
|---|---|---|
| [`authority`](#authority) | `/constitution/naming/exploration/draft3.sol56x.md#authority` | identity grammar, planes, federation |
| [`root`](#root) | `/constitution/naming/exploration/draft3.sol56x.md#root` | membership, moves, maintained home |
| [`anchor`](#anchor) | `/constitution/naming/exploration/draft3.sol56x.md#anchor` | sections, migration, tools |
| [`channel`](#channel) | `/constitution/naming/exploration/draft3.sol56x.md#channel` | accepted tips and current publication |
| [`idea`](#idea) | `/constitution/naming/exploration/draft3.sol56x.md#idea` | canonical concepts and evidence relations |
| [`move`](#move) | `/constitution/naming/exploration/draft3.sol56x.md#move` | rename covenant and anchor updates |
| [`tool`](#tool) | `/constitution/naming/exploration/draft3.sol56x.md#tool` | generation, validation, and deduplication |

If this policy is accepted and synthesized at `/core/README.md`, the maintained
terms receive new full identities such as `/core/README.md#authority`. The
draft identities remain exact design evidence and relate to the maintained
sections as sources; they are not silently rewritten as if they had always
lived there.

<a id="questions"></a>

# Remaining Representation Questions

The policy can be accepted while these implementation details remain open:

- which source parser and publication renderers form the supported carrier set;
- how a canonical authority URI survives repository transfer or rename;
- where grandfathered knowledge paths are declared before all maintained
  knowledge carries `type` frontmatter;
- which relation representation is smallest while remaining checkable;
- when local actor declarations earn a maintained actor roll;
- how private topology is named without publishing sensitive paths; and
- which old public paths earn indefinite compatibility.

Each question has an observable trigger. None requires a universal registry or
automatic cleanup before the first bounded implementation.

<a id="verify"></a>

# Structural Verification

Strict Pandoc GFM rendering completed without warnings. All 44 explicit source
anchors appear exactly once, and all 62 rendered anchor and heading IDs are
unique. Forty-eight local fragment citations resolve to 23 named sections; six
root-relative document references resolve. The source is ASCII, and
`git diff --check` reports no whitespace errors. These checks verify structure,
not policy correctness.

<a id="crossrefs"></a>

# Cross References

- [`draft2-discussion0.sol56x.md`](/constitution/naming/exploration/draft2-discussion0.sol56x.md)
  **compares** the four grammar families, explains the accepted-tip distinction,
  records the anchor-tool lifecycle, and ends with the complete 50-item draft2
  disagreement ledger.
- [`draft2.sol56x.md`](/constitution/naming/exploration/draft2.sol56x.md)
  **supplies** native authority, membership, exact structured paths, broad
  entity census, typed continuity, and separate acceptance/implementation.
- [`draft2.glm53m.md`](/constitution/naming/exploration/draft2.glm53m.md)
  **supplies** concise citation intents, law-bearing actor naming, explicit
  aspect names, and the proposal to raise the subject to a root topic; its OKF
  bundle, `resource`, anchor-verification, equivalence, and migration claims are
  revised here.
- [`draft2-review0.sol56x.md`](/constitution/naming/exploration/draft2-review0.sol56x.md)
  **adversarially establishes** the membership, authority, actor, idea,
  rename-gate, source, OKF, evidence, and sequencing corrections inherited here.
- [`proposal4.glm53m.md`](/constitution/naming/exploration/proposal4.glm53m.md)
  **establishes** location identity, deliberate rename liability, hats, and
  cross-modal aliases. This draft preserves those decisions while openly asking
  the human to revise its concrete flattened suffixless grammar.
- [`/constitution/README.md`](/constitution/README.md) **remains current law**
  until acceptance and a separate implementation plan integrate this core.
- [OKF v0.2](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)
  **defines** the interoperable Markdown Concept profile, source semantics,
  actor convention, lifecycle fields, and conformance boundary used here
  without being mistaken for Rekon's whole native namespace.
