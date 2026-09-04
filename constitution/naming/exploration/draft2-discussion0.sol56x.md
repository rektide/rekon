---
type: DesignDiscussion
title: "Draft 2 discussion: grammar, anchors, core scope, and accepted tips"
description: Compares the naming grammars, develops explicit-anchor tooling and short glossary IDs, explains accepted-tip identity, and records every remaining draft2 disagreement.
tags: [rekon-core, naming, grammar, anchors, glossary, channels, migration, discussion]
status: draft
generated: { by: "integrator/gpt-5.6-sol-max", at: 2026-09-04T10:26:27-04:00 }
stale_after: 2026-12-04T00:00:00-05:00
extensions:
  ticket: rekon-con-naming
  wave: naming draft 2 discussion 0
  model_suffix: sol56x
  routing_address: "openai/gpt-5.6-sol#max"
  session: urn:opencode:session:ses_f9369369bffe6dqU53mp5C6HKv
  discusses:
    - /constitution/naming/exploration/draft2.glm53m.md
    - /constitution/naming/exploration/draft2.sol56x.md
    - /constitution/naming/exploration/draft2-review0.sol56x.md
  human_direction: >-
    Prefer visually explicit anchors with tooling that can generate, update,
    and deduplicate them; compare canonical grammars; use well-known glossary
    terms with short IDs; explain accepted-tip identity; treat the subject as
    Rekon's broad core; produce draft3 and end this discussion with the full
    disagreement list.
sources:
  - id: glm-draft2
    resource: /constitution/naming/exploration/draft2.glm53m.md
    title: Naming draft 2, the successor compact
    author: curator/glm-5.3-max
  - id: sol-draft2
    resource: /constitution/naming/exploration/draft2.sol56x.md
    title: "Naming draft 2: names at home, continuity in relations"
    author: integrator/gpt-5.6-sol-max
  - id: sol-review
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
  - id: okf
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
    title: Open Knowledge Format v0.2
    author: project:knowledge-catalog
---

<a id="discussion"></a>

# Draft 2 Discussion

The draft2 pair agrees on the architecture more than it first appears. Both
make location identity-bearing for knowledge, keep work and actors in native
authorities, distinguish current knowledge from exact evidence, accept explicit
aliases and hats, and require deliberate repair when identities change. Their
largest disagreement is not whether paths matter. It is **which path-shaped
value is canonical, how authority qualifies it, and which alternate forms are
names versus aliases**.

The new direction also clarifies the subject. This program began with naming,
but its scope now includes knowledge, work, actors, evidence, publications,
relations, change, and tools. Naming is the identity kernel of a broader
**Rekon core**. A maintained destination called only `naming` would now
understate the referent.

This discussion makes the choices legible before draft3 adopts them. It uses
short explicit anchors and cites them actively, so the document demonstrates
the proposed practice rather than merely describing it.

<a id="grammar"></a>

# Canonical Grammar

A canonical grammar answers seven independent questions:

| Dimension | Question |
|---|---|
| Authority | Which project or bundle owns the name? |
| Root | Relative to which stable boundary is the path interpreted? |
| Membership | Which paths name artifacts, and which are admitted knowledge? |
| Path | Is the exact suffix retained? |
| Structure | Are `/`, `.`, and `#` retained or flattened? |
| Section | Is the fragment explicit, generated, or both? |
| Revision | Does the reference mean current knowledge, a mutable channel, or exact evidence? |

Treating those as one punctuation question caused much of the draft2
confusion. They should be decided separately.

<a id="forms"></a>

## Compared Grammar Forms

Assume a future maintained section named `authority` in `/core/README.md`.

| Form | Example | What it actually supplies |
|---|---|---|
| OKF Concept ID | `core/README` | Suffixless document identity inside one declared OKF bundle; OKF does not define the section fragment. |
| Ruling-style implied name | `core-readme-authority` | Readable flat projection of path plus anchor; loses segment boundaries without another mapping. |
| GLM draft2 local identity | `core/README#authority` | Suffixless path plus explicit anchor, aligned with OKF at document grain. |
| GLM draft2 qualified fallback | `rekon-core-readme-authority` | Project prefix plus lossy flattening; a declared Beads alias may override it. |
| Sol draft2 native identity | `(https://github.com/rektide/rekon, /core/README.md, authority)` | Canonical authority URI, exact stored path, and explicit anchor with boundaries preserved. |
| Local Markdown citation | `/core/README.md#authority` | A directly followable route when project authority and current line are already contextual. |
| Exact evidence citation | `authority URI + commit + /core/README.md#authority` | Immutable observation of the path and fragment at one commit. |

<a id="tradeoffs"></a>

## Grammar Tradeoffs

**Suffixless paths** align directly with OKF and shorten Markdown concept
names. They also assume Markdown as the serialization and require an actual
bundle boundary. A broad core that names MDX, RST, code, generated views, and
other artifacts needs additional per-format rules.

**Exact paths with suffixes** are reversible, directly inspectable in Git, and
uniform across artifact formats. They deliberately diverge from OKF Concept IDs,
which then become typed export aliases rather than Rekon's canonical native
names. Changing `.md` to `.mdx` is an identity migration, which is consistent
with location identity.

**Flattened names** travel into Beads IDs, YAML keys, Mermaid nodes, and speech,
but flattening is not injective: `a/b-c.md` and `a-b/c.md` collide. A flat form
can be a useful declared alias. It is not a safe canonical encoding unless it
carries escaping or a segment map.

**Structured names** retain authority, path, suffix, and fragment boundaries.
They are longer but do not need a reverse-engineered glossary merely to find
their owner. Short local anchors and ordinary Markdown links carry the common
case.

<a id="choice"></a>

## Recommended Grammar

Draft3 should keep the structured exact-path grammar:

```text
canonical current knowledge name =
  (canonical authority URI, exact namespace-relative path, optional explicit anchor)
```

The reference family around that one name is:

| Role | Form |
|---|---|
| Local shorthand | `#authority` inside the document |
| Local route | `/core/README.md#authority` |
| Cross-project name | canonical authority URI plus exact path and anchor |
| OKF alias | suffixless path inside a declared exported bundle |
| Cross-modal alias | declared flat token such as `rekon-core-authority` |
| Exact evidence | authority URI plus immutable commit plus path and anchor |

This choice openly revises the ruling's flattened suffixless spelling. The
reason is the expanded subject: Rekon's core names more than Markdown Concepts,
and the canonical form should be reversible without a hidden registry. It still
honors the ruling's deeper decision that logical location participates in
identity and a move is a rename.

The authority URI is mandatory across projects. `rekon:` remains a local
elision, not a globally unique authority identifier. The namespace root is the
repository root; knowledge admission remains explicit rather than pretending
the entire repository is an OKF bundle.

<a id="anchors"></a>

# Explicit Anchors

The preference for source-visible explicitness is compatible with short IDs and
with unique rendered output. The recommended source form is:

```markdown
<a id="authority"></a>

## Naming Authority
```

The canonical section component is the short word `authority`. The visible
heading says enough to orient a reader. Pandoc's generated heading ID is
`naming-authority`, so this representation does not emit two elements with the
same `id`.

The prose should then use the ID:

```markdown
Every [name](#name) is scoped by an [authority](#authority) and a
[namespace root](#root).
```

That is the active demonstration missing from much of draft2. A list of anchors
at the end is not usage. Internal links, glossary references, change tables, and
cross-document citations should exercise the names readers and tools are being
asked to trust.

<a id="anchor-tool"></a>

## Anchor Tool Contract

The tooling should manage explicit anchors as identity-bearing source, not as
decorative lint.

| Operation | Required behavior |
|---|---|
| Generate | Detect a durable heading without an explicit adjacent anchor, propose a short glossary term, insert the anchor, and show the resulting full path-plus-fragment name. |
| Validate source | Require valid syntax, adjacency, one canonical owner per fragment, and no accidental duplicate declarations. |
| Validate render | Render every supported publication carrier and require one unambiguous target per promised fragment. |
| Heading edit | Keep the anchor when the referent remains the same; report misleading heading/anchor divergence for review. |
| Identity rename | Require a human-gated old-to-new map, update controlled citations, and retain the old fragment as a compatibility alias where promised. |
| Deduplicate | Remove or rename accidental duplicate declarations after identifying the owner; never delete an intentional compatibility alias merely because two routes reach one referent. |
| Remove | Permit removal only when the entity is retired or no durable citation promise remains, with explicit disposition. |
| Generate index | Derive the path, heading, anchor, aliases, and inbound references without becoming their authority. |

"Update as changing" therefore does not mean rewriting an anchor every time a
heading changes. That would turn display prose back into identity. The tool
updates an anchor when the **referent or canonical term** changes, and otherwise
keeps the pin stable.

The supported-carrier set must be declared. At minimum it includes the source
Markdown parser used by validation and the publication renderers on which links
are promised. Pandoc-only slug comparison is insufficient, and equality between
two source sequences is not rendered-ID verification.

<a id="terms"></a>

# Glossary Terms And Short IDs

Short does not mean cryptic. Each ID below is a familiar term whose heading and
definition remove ambiguity.

| ID | Canonical term | Meaning |
|---|---|---|
| `core` | Rekon core | The named system of knowledge, work, actors, evidence, relations, publication, and tools. |
| `authority` | Naming authority | The project steward or native system entitled to maintain a name in one scope. |
| `root` | Namespace root | The stable path base from which native artifact names are interpreted. |
| `member` | Knowledge member | An artifact explicitly admitted as knowledge by type frontmatter or a grandfathered declaration. |
| `path` | Canonical path | The exact namespace-relative stored path, including suffix. |
| `anchor` | Explicit anchor | The source-visible fragment naming one durable section. |
| `name` | Canonical name | The one current identity assigned by a native authority. |
| `alias` | Alias | A declared alternate reference to one owner. |
| `locator` | Locator | A route to a representation or execution that need not be identity. |
| `revision` | Revision binding | An immutable coordinate that binds exact evidence. |
| `channel` | Publication channel | A mutable name selecting the currently published target. |
| `tip` | Accepted tip | A channel selecting the current accepted artifact in a design series. |
| `plane` | Native plane | A domain whose own system maintains its canonical identifiers and lifecycle. |
| `bridge` | Cross-plane bridge | An explicit relation or alias connecting entities from different native planes. |
| `idea` | Durable idea | An abstract concept with an authority-selected maintained knowledge presence. |
| `actor` | Actor | A human, agent, or process making an identity or authorship claim. |
| `hat` | Hat | A scoped alias by which one actor is addressed for a concern. |
| `role` | Role | A typed, historical responsibility relation, not another actor. |
| `work` | Work item | A Beads-owned commitment with lifecycle and dependency state. |
| `move` | Identity migration | A canonical authority, root, path, or anchor change. |
| `successor` | Successor | The current entity explicitly related to a retired identity. |
| `tombstone` | Tombstone | A deprecated old-path knowledge presence describing retirement and successor. |
| `home` | Maintained home | The current public account readers should trust first. |
| `tool` | Naming tool | Guarded machinery that generates, validates, migrates, and indexes names. |
| `accept` | Acceptance | The human decision selecting policy without silently performing implementation. |

Draft3 should use these anchors in its own prose. The full identity remains the
document path plus short anchor; the table is a glossary for meaning, not a
second identifier registry.

<a id="scope"></a>

# Rekon Core Scope

Calling the maintained result `naming` now under-describes it. The system says:

- what kinds of durable entities Rekon recognizes;
- which native systems own knowledge, work, actors, and evidence;
- how those entities are named and related;
- how current publications differ from exact history;
- how identity changes are approved and repaired;
- how tools derive indexes without taking authority; and
- how the constitution and tickets govern acceptance and implementation.

That is a compact account of what Rekon is and how it works, not merely a style
guide for identifiers.

<a id="home"></a>

## Maintained Core Home

Three plausible homes have different claims:

| Home | Strength | Weakness |
|---|---|---|
| `/naming/README.md` | Direct continuity with the exploration and human instruction to move naming upward | Understates work, evidence, actors, publication, and relations |
| `/core/README.md` | Names the broadened subject and remains a domain-grouped root topic | Requires an explicit successor relation from the naming topic |
| `/README.md` | Makes the project explain itself at entry | Mixes project introduction, software usage, and constitutional depth too early |

The best maintained destination is **`/core/README.md`**. Naming remains its
identity kernel and a major section. The existing exploration evidence should
remain at its authored paths. Implementation should turn
`/constitution/naming/README.md` into a concise historical navigation or
tombstone pointing to the new maintained [home](#home), update controlled live
references, and leave revision-bound evidence citations unchanged.

This is the useful sense in which naming moves up a level: the current account
moves from a child of the constitution to a root-level core concern. It does
not require pretending every historical draft was authored at the new path.

<a id="tip"></a>

# Accepted Tip Semantics

The disagreement numbered seven in the earlier comparison was whether a
suffixless symlink becomes the canonical identity of the accepted artifact.
The answer is that the symlink and target name **different referents**.

Using the existing specimen:

```text
design/doc-constitution/adoption.md -> adoption1.glm53m.md
```

| Path | Referent | Citation promise |
|---|---|---|
| `adoption1.glm53m.md` | One exact model-attributed artifact | These authored bytes and this evidence identity |
| `adoption.md` | The mutable accepted-adoption channel | Whichever artifact is currently selected as accepted |

The suffixless path has a canonical identity, but it is the identity of the
**channel**, not a second identity for every target it selects. Retargeting the
symlink changes channel state. It does not rename either old or new target.

This yields a simple citation rule:

| Intent | Cite |
|---|---|
| Exact argument or historical evidence | Model-suffixed target plus immutable revision when needed |
| Current accepted winner in a design series | Accepted-tip channel |
| Maintained user-facing truth | Real maintained `README.md`, normally not a winner symlink |

The GLM draft is right that corpus practice treats the suffixless path as a
meaningful accepted route. It goes too far when it calls that route the accepted
artifact's canonical identity. The Sol draft was too terse when it called it
only a channel without emphasizing that channels are themselves named entities.

Symlink transport remains a separate question. Git, archives, forges, package
tools, and browsers may expose the channel differently. A publication promise
must test the actual carrier, and exact evidence must never depend on mutable
retargeting.

<a id="draft3"></a>

# Consequences For Draft 3

Draft3 should:

1. call the subject Rekon core and propose `/core/README.md` as its maintained
   destination;
2. use the structured exact-path grammar while labeling the departure from the
   ruling and OKF explicitly;
3. define canonical authority URI, namespace root, artifact identity, and
   knowledge admission before name resolution;
4. use short, explicit glossary anchors and cite them throughout;
5. require an anchor tool that generates, validates, migrates, deduplicates, and
   indexes explicit source anchors;
6. define an accepted tip as a named mutable channel whose target keeps a
   separate exact artifact identity;
7. preserve native planes for Beads work, actors, evidence, and computations;
8. give durable ideas authority-selected maintained concept presences;
9. keep human-gated identity migration and carrier-specific compatibility;
10. move constitutional edits, the new core home, and any path migration into a
    separate post-acceptance implementation plan.

<a id="references"></a>

# Cross References

- [`draft2.glm53m.md`](/constitution/naming/exploration/draft2.glm53m.md)
  **provides** the suffixless OKF-aligned grammar, explicit named laws, citation
  table, actor threshold, and root-level promotion proposal assessed here.
- [`draft2.sol56x.md`](/constitution/naming/exploration/draft2.sol56x.md)
  **provides** the exact structured grammar, namespace admission, canonical URI,
  broad entity census, channel distinction, and post-acceptance implementation
  split carried into draft3.
- [`draft2-review0.sol56x.md`](/constitution/naming/exploration/draft2-review0.sol56x.md)
  **identifies** the grammar, membership, authority, actor, idea, anchor,
  evidence, OKF, and acceptance defects corrected before this discussion.
- [`proposal4.glm53m.md`](/constitution/naming/exploration/proposal4.glm53m.md)
  **records** the location-identity, rename-cost, hat, and cross-modal-alias
  ruling. Draft3 preserves its identity principle while openly revising its
  flattened suffixless spelling.
- [OKF v0.2](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)
  **defines** the suffixless Markdown Concept ID against which the broader
  multi-format Rekon grammar must declare its typed alias rather than claim
  conformance accidentally.
- [`/constitution/README.md`](/constitution/README.md) **remains current law**
  until a human accepts the core direction and a separate implementation plan
  integrates it.

<a id="disagreements"></a>

# Full Draft 2 Disagreement List

This is the complete disagreement ledger after the new direction. "Draft3"
means the resolution proposed by this discussion, not accepted law.

| # | Question | GLM draft2 | Corrected Sol draft2 | Draft3 direction |
|---:|---|---|---|---|
| 1 | Subject | Naming compact | Naming as native identity architecture | Rekon core, with naming as the identity kernel |
| 2 | Maintained home | Move to `/naming/README.md` | Keep naming presence unless separately decided | Promote current account to `/core/README.md`; preserve evidence paths |
| 3 | Canonical document path | Bundle path without `.md` | Exact project path including suffix | Exact path including suffix |
| 4 | Structural delimiters | Native slashes locally, flattened qualified fallback | Preserve `/`, `.`, and `#` | Preserve structure canonically; flat forms are aliases |
| 5 | Authority identifier | Project/bundle label plus optional repository URL | Canonical repository URI mandatory across projects | Canonical authority URI; `rekon` is local elision |
| 6 | Namespace root | Repository working copy as OKF bundle root | Repository root as native namespace, no OKF conformance claim | Repository root as native namespace root |
| 7 | Namespace membership | Unspecified exclusions from an alleged bundle | Every tracked path is an artifact; typed or declared subset is knowledge | Keep explicit artifact and knowledge admission classes |
| 8 | Bundle root versus boundary | One equivalence class, boundary preferred | Different concepts: path base versus authority/membership frame | Use standard `root` and `authority`; no forced alias |
| 9 | Docroot | Separate from bundle root | Navigation/layout root, not identity root | Retain as navigation vocabulary only |
| 10 | Module and README | Directory module and README are one identity | Directory and README may be separate entities; elision is an alias | README is maintained account; directory route is contextual resolution |
| 11 | Knowledge versus artifact | Mostly one path-based concept class | Distinct artifact identity and admitted knowledge identity | Keep the distinction |
| 12 | Durable abstract idea | Equivalence class of similarly named sections | Authority-selected maintained concept presence | Canonical maintained presence plus typed evidence relations |
| 13 | Anchor source | Explicit anchor equal to heading-derived slug | Explicit source anchor, representation may differ from heading slug | Explicit short glossary anchor |
| 14 | Anchor semantic correspondence | Byte equality at mint | Semantic correspondence separated from rendering | Short term must remain visibly related to heading and glossary definition |
| 15 | Rendered duplicate IDs | Claimed verified by matching two sequences | Detected as a Pandoc defect | Require one rendered target per carrier; generate a non-colliding representation |
| 16 | Anchor prefix | Optional trade for ticket ancestry | Useful for readability and avoiding generated-ID collision | Prefer short well-known IDs; expand the visible heading instead |
| 17 | Anchor tooling | Anchor checker first, focused on derived equality | Broader validator and migration tooling | Generate, validate, migrate, deduplicate, remove, and index explicit anchors |
| 18 | Same terminal anchor across files | Harmless under path identity | Harmless under structured full identity | Retain |
| 19 | One-name discipline | Preferred-string equivalence classes in maintained register | One native canonical owner plus typed relations | Canonical presence plus aliases; never infer equivalence from wording |
| 20 | Qualified knowledge form | Flattened derivation unless a Beads-owned alias overrides | Canonical URI plus structured exact path | Structured canonical form; Beads IDs are cross-plane aliases or relations |
| 21 | `local_knowledge_id` | Retire | Grandfather as alias/projection when useful | Retire from new work; migrate only when materially touched |
| 22 | `qualified_knowledge_id` | Keep as declared Beads-backed alias slot | Grandfather as alias, never canonical | Optional explicit cross-modal alias, not a required field |
| 23 | Top-level `resource` | Own path or absent | Underlying asset URI per OKF | Follow OKF; derive self-location from canonical path |
| 24 | Current versus exact | Three citation intents | Current, exact, channel, and relation intents | Keep four typed intents |
| 25 | Accepted tip | Suffixless symlink is accepted artifact identity | Suffixless path is a channel | Channel is a named entity distinct from every selected artifact |
| 26 | Maintained README | Can be winner symlink under acceptance placement | Normally real maintained prose | Real `/core/README.md`; accepted-tip symlinks remain design channels |
| 27 | Actor naming | Law-bearing documents owe a named voice | Root must self-name; durable children normally self-name | Retain Sol rule and demonstrate it |
| 28 | Numerical plus chosen actor name | Implicit model/session distinction | Explicit Docker-style pair | Exact session locator plus chosen actor name |
| 29 | Hats and roles | Hats aliases; roles relations | Same | Retain |
| 30 | Actor roll | Maintained topic README register at acceptance | Defer registry until pressure | Begin with local declarations and derive an index; promote to roll when collisions or continuity earn it |
| 31 | Source key identity | Not central | `sources[].id` is scoped token; resource reaches source | Retain Sol correction |
| 32 | Rename gate | Human gate and sealed inventory | Mandatory human gate and immutable enactment revision | Retain |
| 33 | Tombstone meaning | Old identity in deprecated state | Distinct old-path deprecated presence related to successor | Old path's current deprecated presence; never the successor's identity |
| 34 | Tombstone `resource` | Uses old self-path | Reject self-path use | Omit unless the notice describes an underlying asset; successor relation carries navigation |
| 35 | Symlink compatibility | Tombstone default, symlink verified optimization | Carrier-specific choice | Retain carrier-specific rule |
| 36 | Escape window | Monotonic law, narrowed to live citations | Heuristic only; promises can retire | Decision-time cost factor, not constitutional law |
| 37 | Promotion | Move whole naming subtree at acceptance | Synthesize maintained account, leave evidence | Create `/core/README.md`; old topic becomes historical navigation/tombstone |
| 38 | Historical wave evidence | Move unmodified while rewriting its links | Leave at authored paths | Leave in place and cite exactly |
| 39 | Inbound-link claim | Zero, based on corpus measurement | Literal isolation only, not complete graph | Require parser-resolved sealed inventory before implementation |
| 40 | External citations | Claims zero | Unknown outside controlled surfaces | Treat as unknown |
| 41 | Acceptance timing | Amend constitution and move topic in acceptance change | Acceptance selects policy only | Separate acceptance record from implementation plan |
| 42 | Constitutional amendments | A1-A5 applied at acceptance | Surgery matrix implemented later | Reuse corrected A1-A5 organization in separate plan |
| 43 | Beads role | Native work IDs plus declared qualified aliases | Native work authority linked to knowledge | Retain native work authority and explicit bridge |
| 44 | Federation | Authority-qualified path plus paired URL | Canonical URI required | Canonical URI is identity component across projects |
| 45 | Portable origin | Mandatory conceptual envelope | Required context, schema deferred | Retain, expressed through ordinary provenance and relation terms first |
| 46 | OKF status | Repository treated as bundle with exclusions | Repository explicitly not conformant | Use OKF as export/interchange profile, not native namespace authority |
| 47 | `stale_after` | Date only | Offset-bearing instant | Offset-bearing instant |
| 48 | Measurement authority | Counts used to establish law and migration timing | Counts are revision-bound evidence with limited entailment | Keep measurements out of canonical grammar; use them in implementation decisions |
| 49 | Convergence | Five extensions treated as settlement | Shared reading is evidence, not authority | Human acceptance decides |
| 50 | Tool priority | Anchor equality checker first | Membership, anchors, links, aliases, and migrations form one boundary | Explicit-anchor operations first, but grounded in declared membership and authority |
