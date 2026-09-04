---
type: NamingOverrideResponse
title: Location identity needs boundaries and tombstones
description: An independent extension of the human ruling, grounding location identity in a declared bundle authority, making compatibility artifacts honest under OKF, and pricing the contemplated naming-topic move against the live corpus.
resource: /constitution/naming/exploration/draft1.sol56x.md
tags: [naming, identity, location, okf, migration, federation, compatibility, aliases, hats]
status: draft
generated: { by: "model:openai/gpt-5.6-sol#max", at: 2026-09-03T22:05:50-04:00 }
stale_after: 2026-12-03T00:00:00-05:00
extensions:
  ticket: rekon-con-naming
  wave: naming override response draft 1
  model_suffix: sol56x
  position: extend
  independence: No other draft1 artifact was consulted.
  actor_claim:
    name: surveyor/gpt-5.6-sol-max
    status: self-proposed
    scope: rekon-con-naming
    hats: [respondent, migration-auditor]
  responds_to:
    - /constitution/naming/exploration/proposal4.glm53m.md
    - /constitution/naming/exploration/proposal4.sol56x.md
    - /constitution/naming/exploration/ask-audit0.glm53m.md
sources:
  - id: ruling
    resource: /constitution/naming/exploration/proposal4.glm53m.md
    title: Naming for rekon revision 4, the human override
    author: steward/glm-5.3-max
  - id: override-companion
    resource: /constitution/naming/exploration/proposal4.sol56x.md
    title: Naming for rekon revision 4 companion override, including portability addendum
    author: model:openai/gpt-5.6-sol#max
  - id: ask-audit
    resource: /constitution/naming/exploration/ask-audit0.glm53m.md
    title: Naming ask audit revision 0
    author: steward/glm-5.3-max
  - id: weaver-synthesis
    resource: /constitution/naming/exploration/proposal3.glm53m.md
    title: Naming synthesis revision 3, including override response
    author: weaver/glm-5.3-max
  - id: concordance-synthesis
    resource: /constitution/naming/exploration/proposal3.sol56x.md
    title: Naming synthesis revision 3, including override response
    author: rekon-naming-concordance
  - id: codex-synthesis
    resource: /constitution/naming/exploration/proposal3b.glm53m.md
    title: Standalone naming synthesis revision 3b, including override response
    author: codex/glm-5.3-max
  - id: fm-response
    resource: /constitution/naming/exploration/draft0.glm53fm.md
    title: Naming exploration draft by FM, including override response
    author: model:glm-5.3-flash-max
  - id: proposal2
    resource: /constitution/naming/exploration/proposal2.glm53m.md
    title: Naming and the new core exploration proposal revision 2
    author: model:glm-5.3-max
  - id: canonical-constitution
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution
    author: project:rekon
  - id: hierarchy1
    resource: /constitution/README-heirarchy1.glm53m.md
    title: Topic-first hierarchy with migration vocabulary
    author: model:glm-5.3-max
  - id: okf-spec
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
    title: Open Knowledge Format version 0.2
    author: project:knowledge-catalog
    last_modified: 2026-06-30
  - id: naming-work
    resource: /.beads/issues.jsonl
    title: Rekon work graph containing rekon-con-naming
    author: project:rekon
---

<a id="location-identity-needs-boundaries-and-tombstones"></a>

# Location Identity Needs Boundaries And Tombstones

<a id="position"></a>

## Position

I **extend** the ruling. Location-implied naming is the right identity model for
tree-resident knowledge in Rekon, and the three respondent addenda correctly
concede that their move-stable stems were a preference rather than an
architectural necessity. The ruling nevertheless needs two additional pieces of
law before it can carry the successor compact: the bundle boundary that makes a
path meaningful, and the identity status of the compatibility artifact left at
a retired path.

This is not a fourth vote on whether location is identity. The authority for
that choice is the human ruling in
[`proposal4.glm53m.md`](/constitution/naming/exploration/proposal4.glm53m.md).
The work here begins after the choice and asks whether its accounting closes.

The answer is mostly yes. `naming-name-promise`,
`naming-reference-registers`, `naming-slug-at-mint`, the speech-act census,
plane federation, the promise/currency split, the deferred tool seam, and the
supersede ambition all survive. The open edge is not a hidden semantic stem. It
is the declared frame within which a location is a location.

<a id="the-ruling-changes-the-source-of-knowledge-names"></a>

# The Ruling Changes The Source Of Knowledge Names

Before the ruling, the syntheses treated a tree path as a locator for a
separately minted semantic owner. After the ruling, a steward mints a
tree-resident knowledge name by placement. The identity is composed directly
from native facts:

```text
document identity = authority + bundle-relative path without .md
section identity  = document identity + pinned anchor
```

The `+` signs describe composition, not a proposed wire syntax. The local OKF
Concept ID is the path component. Rekon's section extension adds the fragment.
`naming-federated-authority` supplies authority qualification when the name
crosses a project or bundle boundary.

This changes several concrete classifications:

- A file placement is the document minting act, as
  `naming-placement-at-mint` now records.
- A file move is a rename of one document and all section names composed from
  its path.
- An anchor change is a section rename even when the file does not move.
- A heading wording change with the pinned anchor retained is not a rename.
- A checkout path, mirror URL, session route, or task handle remains a locator,
  not the knowledge name.
- A frontmatter `resource` equal to the current file path is a checkable
  self-locator, not a second identity authority. If it disagrees with placement,
  placement wins and the field is stale.
- Historical `local_knowledge_id` and `qualified_knowledge_id` fields remain
  usable resolution forms. They cannot override the path that owns the current
  OKF Concept ID.

The last two points matter in this corpus. Many artifacts state both their
location and a stem in frontmatter. Location identity does not leave an
election between them. The tree is current authority; redundant fields are
projections that can be checked and repaired.

<a id="what-survives-and-how-it-changes"></a>

## What Survives And How It Changes

| Surviving structure | Consequence under location identity |
|---|---|
| `naming-name-promise` | The steward promises to maintain a location name or disclose its retirement and successor. Durability means kept repair, not immobility. |
| `naming-one-name-locality` | One current regular path owns one knowledge identity in one authority. Additional forms resolve to it; historical names retain explicit disposition. |
| `naming-reference-registers` | The bundle path occupies the knowledge Name register and may also function as a locator. Citation tokens and evidence bindings remain distinct. |
| `naming-derived-vs-declared` and `naming-slug-at-mint` | The heading derives the anchor at mint, the explicit anchor pins the section-local segment, and the enclosing path composes the full name. |
| `naming-speech-acts` | A knowledge name is literally an address in its native plane. A work name remains a commitment and an actor name remains a vow. |
| `naming-native-authority` and `naming-plane-federation` | Filesystem placement names knowledge; Beads names work; actors declare themselves; version control names exact evidence. Shared spelling does not collapse the planes. |
| `naming-promise-currency-split` | A self-name or hat is the actor's unilateral promise; an authority supplies shared currency. Knowledge placement is instead an authority-governed mint in the tree it owns. |
| `naming-rename-covenant` and `naming-convergence-repair` | They become the ordinary payment instruments for moves: predecessor, successor, authority, compatibility disposition, controlled rewrites, and repeated repair. |
| `naming-home-not-layout` | It survives only as sequencing. The compact can be accepted before a move; a later move is still identity change. |
| `naming-successor` and `naming-supersede-proof` | The maintained home can be a new synthesis at its intended final location. Historical evidence need not be moved merely because the current account is promoted. |
| `naming-tool-seam` | The invariants are now more concrete, but tool behavior remains deliberately deferred. Counting and checking here demonstrate the seam without specifying its implementation. |

`naming-location-separation` does not survive as general knowledge law. Its
valid actor-side observation is already named more precisely by
`naming-address-vs-identity`: moving a session or changing a routing address
does not rename the actor. It should not be quietly redefined and allowed to
compete with the ruling.

The FM response addendum landed while this artifact's migration count was being
verified. Its `naming-location-fixed-point` names a real benefit: current
identity bookkeeping and the referent become the same inspectable tree, so a
shadow stem-to-path ledger cannot drift silently. I adopt it with a boundary the
addendum itself supplies through `naming-promise-jurisdiction`: the fixed point
is current and authority-local. Aliases, retired paths, federation, and
authority death still require declared mappings and cannot be repaired by
placement alone. FM is also right to demote `naming-resolver` from the canonical
core to scale changes, history, and cross-authority resolution; that is where
the remaining maps actually live.

<a id="naming-bundle-boundary"></a>

# Naming Bundle Boundary

`naming-bundle-boundary` is the first new law this response proposes:

> **A location-derived knowledge identity is relative to one declared bundle
> authority and bundle root. The authority and root MUST be stable enough to
> interpret every local Concept ID. Changing either is an identity migration,
> not a change of process working directory. A physical checkout path is never
> part of the local knowledge identity.**

This is not optional polish. OKF section 2 defines a Concept ID as the path of a
file **within the bundle**, and section 3 defines a bundle as a hierarchical
directory tree and unit of distribution. Without the bundle boundary, the same
file can be called any of these depending on where a consumer starts walking:

```text
constitution/naming/exploration/proposal4.sol56x
naming/exploration/proposal4.sol56x
exploration/proposal4.sol56x
proposal4.sol56x
```

All are paths. Only one is the path within the naming authority that made the
promise.

Hierarchy1's **docroot** does not automatically solve this. A docroot is the
declared top of a project's preferred knowledge hierarchy. Rekon also has root
topics such as `constitution/`, and OKF permits a bundle to be a whole
repository or a subdirectory. Bundle root and docroot may coincide, but they
answer different questions and MUST NOT become synonyms by accident.

The current corpus strongly suggests the repository root as Rekon's bundle
root: its canonical links and frontmatter resources are root-relative from
there, and root topics are intentional. That is evidence for the eventual
human choice, not an authority claim by this draft. Exact declaration syntax,
discovery, and serialization remain at `naming-tool-seam`; the semantic
requirement to have one does not.

<a id="the-location-identity-tuple"></a>

## The Location Identity Tuple

The useful mental model is a structured coordinate, not a flattened byte
string:

| Component | Who owns it | What changing it means |
|---|---|---|
| Authority | Project or bundle steward | Cross-authority transfer, fork, or succession |
| Bundle root | Authority declaration | Batch re-identification of paths if the root changes |
| Relative path without `.md` | Filesystem placement | Document rename when moved within the bundle |
| Pinned anchor, when present | Owning document | Section identity; anchor change is section rename |
| Revision, when present | Version history | Evidence binding, not part of the current semantic name |
| Reachable URL or checkout path | Host or transport | Locator; may multiply without renaming the local concept |

This tuple does not restore a hidden identity above location. It is location
decomposed so that federation does not mistake `/home/rektide/src/rekon` for a
semantic prefix or a GitHub mirror for a second project.

<a id="travel-is-not-one-operation"></a>

## Travel Is Not One Operation

The respondent addenda surfaced `naming-extraction-asymmetry`: location names
land exactly and travel with context; portable stems travel lightly and require
a map. The companion's portability addendum correctly separates travel cases.
`naming-identity-change` should adopt that distinction explicitly:

| Operation | Identity result |
|---|---|
| Intact bundle copied, archived, or cloned under the same authority | Local Concept IDs are unchanged. The new physical route is another locator. |
| Read-only mirror under the same authority | Local identities are unchanged. Divergent writable mirrors require a continuity decision. |
| Bundle fork under a new authority | Relative paths may match, but qualified identities differ. Provenance or equivalence relates them. |
| File copied into a second bundle | The destination mints a new artifact identity. Matching bytes do not create identity. |
| Canonical ownership transferred across bundles | The old identity retires and the new authority mints a successor. Only the old authority can preserve old-name resolution. |
| File moved inside one bundle | The path changes, so the document and its pinned sections are renamed. |
| Bundle root changed around an otherwise stationary tree | Every affected Concept ID changes. This is a namespace migration. |

This table prevents two opposite mistakes: treating every `git clone` as a
mass rename, and treating extraction into a new tree as though copied
frontmatter could carry the old authority with it.

<a id="standalone-documents-carry-origin-without-carrying-identity"></a>

## Standalone Documents Carry Origin Without Carrying Identity

A standalone document can travel without its original tree, but it cannot carry
an old authority merely by remembering an old stem. At a destination, three
facts must remain separate:

- **Current identity:** the destination authority and destination path if the
  document is installed as a concept there.
- **Origin:** a qualified source identity, ideally revision-bound when the
  claim is historical.
- **Equivalence claim:** an explicit statement of whether the destination is a
  mirror, derivation, translation, replacement, or merely discusses the same
  subject.

Frontmatter sources are well suited to carrying origin. They do not make the
copied file the original entity. A detached file that is only being transported
and not installed in a new bundle can continue to describe its source identity,
just as an attachment can describe where it came from; a consumer must not infer
a new canonical authority from a temporary download directory.

The minimum portable envelope is not settled here. It must at least preserve
the source authority, bundle-relative path, and citation intent. Whether that is
encoded in `resource`, a producer extension, a package manifest, or a canonical
URL belongs at the deferred seam.

<a id="cross-project-citation-retains-delimiters"></a>

## Cross Project Citation Retains Delimiters

Across projects, a citation needs the authority plus the structured local
identity. For current knowledge, use an authority-qualified canonical route to
the bundle path and optional anchor. For exact evidence, also pin the revision
and keep the path that was true at that revision. A later move should repair the
first kind and must not rewrite the second into an ahistorical claim.

The ruling's flattened example,
`constitution-naming-exploration-proposal4-grammar`, is a valid location-implied
form under a declared projection. It is not mechanically reversible in every
possible tree. For example, these two paths flatten identically if slash and
hyphen are both simply replaced by hyphens:

```text
a/b-c.md
a-b/c.md
```

This does not defeat `naming-cross-modal`. It explains why one-byte identity
across hostile media remains dead and why `naming-alias-collapse` still has
work. The slash-preserving path and fragment are the native knowledge form; a
flat Beads ID, Mermaid node, YAML key, or spoken abbreviation is a media-native
projection of the same owner when context or a declaration restores component
boundaries. If it cannot, the alias has collapsed.

No advance central permission is required to use the projection. Durable use
does create a promise to keep its mapping. `rekon-con-naming` remains the
precedent: it is one native work ID containing the cross-modal `con` form, not a
claim that the work item and this knowledge artifact are one entity.

<a id="section-identity-is-a-rekon-extension"></a>

# Section Identity Is A Rekon Extension

OKF's receipt is exact but narrower than some responses imply. In section 2, a
Concept is one Markdown document and its Concept ID is the file path without
`.md`. OKF does not say that every heading is another Concept. Rekon adds
section identity as a local extension:

```text
<OKF Concept ID>#<pinned Rekon section anchor>
```

The distinction should remain visible:

- The document is the OKF Concept and owns frontmatter disposition,
  provenance, and trust signals.
- A pinned section is a Rekon knowledge entity scoped by that document.
- A renderer-derived heading slug without a pin is a convenient locator, not a
  durable section promise.
- `naming-slug-at-mint` creates the durable segment by deriving it from the
  heading and declaring it explicitly.
- Moving the document renames every composed section even though the anchor
  bytes stay fixed.
- Editing the section's prose without moving or re-anchoring it preserves its
  identity but can still drift semantically. Identity is not verification.

This avoids claiming that OKF already standardized fragments while preserving
the human ruling that anchors imply the rest of the name.

<a id="promotion-and-extraction"></a>

## Promotion And Extraction

Turning a section into a standalone document is not a neutral locator move. The
new file receives an OKF Concept ID; the old section remains historical or
becomes a compatibility presence. `naming-identity-change` should classify the
operation by intent:

| Intent | Treatment |
|---|---|
| The old section remains the maintained owner and a new document elaborates it | Mint a new document and state `extends` or another truthful relation. |
| The new document becomes the maintained owner | Mint the document, retire the old section identity, and state supersession. |
| One section is split into several independent concepts | Mint each destination and state the split lineage; do not let one old fragment silently denote all outputs. |
| Text is copied for convenience | Two location identities exist; state derivation or remove accidental duplication deliberately. |

The same reasoning applies to promotion from a model-suffixed synthesis into a
maintained README. The accepted README is a new current account with an
explicit source and supersession relation. The evidence artifact keeps the
identity under which it was actually authored. This is stronger than pretending
that promotion must physically move history.

<a id="naming-retired-path-tombstone"></a>

# Naming Retired Path Tombstone

`naming-retired-path-tombstone` is the second new law this response proposes:

> **Under path identity, a regular Markdown compatibility file at an old path
> is the deprecated presence of the old location identity, not an identity-free
> alias and not the new concept. It MUST disclose its successor and MUST NOT
> present a competing current body. Resolution MAY follow that relation to the
> successor; exact historical evidence continues to denote the old identity at
> its cited revision.**

This follows directly from OKF rather than adding ceremony for its own sake.
OKF section 3 says all non-reserved `.md` files are concept documents. Section
5.4 says `deprecated` means kept for links and history, no longer current. A
regular `old.md` stub is therefore still the concept at `old`; its current body
is a retirement record. `new.md` is the successor at `new`. They are related,
not secretly one pathless object.

The distinction repairs a loose sentence repeated in the responses: "an old
path is only a retired resolution form, not another concept." That can be true
of an HTTP redirect or a resolver-table entry. It is not literally true of an
OKF-conformant Markdown stub. The stub is how the old concept keeps its final
promise without competing as current truth.

The minimal semantic shape, without prescribing exact extension syntax, is:

```yaml
type: Compatibility Notice
status: deprecated
resource: <old bundle-relative path>
extensions:
  superseded_by: <new authority-qualified path>
```

The body explains the relationship in ordinary prose and links to the
successor. `naming-rename-covenant` records who authorized the transition and
what compatibility is promised. No tool design is implied.

<a id="why-the-stub-is-an-entity"></a>

## Why The Stub Is An Entity

Calling the stub an entity preserves rather than weakens one-name:

- The old identity has one deprecated owner: the tombstone state at the old
  path.
- The new identity has one current owner: the maintained body at the new path.
- The supersession edge says why resolution may continue from old to new.
- The old name is never reassigned to an unrelated referent.
- A reader can distinguish "what was here" from "what should I read now."
- A crawler does not need an undocumented exception saying some Markdown files
  count as concepts and visually similar ones do not.

The rename is still a rename. A tombstone does not claim the old and new names
are simultaneous canonical identities of one current body. It records the
transition between two location identities.

<a id="deep-fragment-compatibility"></a>

## Deep Fragment Compatibility

A file-level tombstone does not automatically keep section promises. Browsers
do not carry an incoming `#old-section` through an ordinary Markdown link to
`new.md#new-section`. If a durable old fragment is promised, the compatibility
surface needs that old anchor and an explicit destination or a host mechanism
whose fragment behavior has been verified.

This produces three levels of compatibility that migration reviews must not
collapse:

| Level | Promise kept |
|---|---|
| Old file route opens | Document-level discovery only |
| Old explicit anchors still land on retirement notices | Historical section identities remain addressable |
| Old anchors lead to each intended successor section | Current semantic navigation is preserved fragment by fragment |

The appropriate level depends on durable inbound use. It is not necessary to
manufacture 408 fragment redirects merely because 408 anchors exist, but every
omitted externally cited fragment is a deliberate break, not coverage supplied
by the file stub.

<a id="symlinks-need-rekon-law"></a>

## Symlinks Need Rekon Law

OKF v0.2 does not define symlink semantics. A symlink may be preserved by Git,
dereferenced by an archive, ignored by a bundle scanner, or rendered by a forge
as though the target occupied both paths. Under location identity those
behaviors are semantically different.

Hierarchy1 can continue to permit a symlink as a compatibility mechanism, but
Rekon must declare what it means in each publication surface. A symlink is a
resolver exposure only when the authority promises that interpretation. If a
consumer walks both paths as concept files, the same bytes have two path
identities. If packaging dereferences the link, separate per-path disposition
cannot be expressed. For portable plain-Markdown compatibility, a regular
deprecated tombstone is the clearer default; symlinks remain a verified local
optimization.

This also applies to winner-symlink mode and `SKILL.md -> README.md`. The
one-body rule can still hold editorially, but the bundle consumer needs an
explicit exposure rule rather than an assumption imported from one filesystem.

<a id="the-rename-ripple-priced-in-this-corpus"></a>

# The Rename Ripple Priced In This Corpus

The portability addendum in `proposal4.sol56x.md` correctly said the actual move
had not yet been counted. This response prices the contemplated promotion of
`constitution/naming/` to root topic `naming/` against the live pre-`draft1`
substrate.

<a id="the-counting-snapshot"></a>

## The Counting Snapshot

The snapshot was taken on 2026-09-03 after all three synthesis respondent
addenda, the companion portability addendum, and FM's round-0 response landed.
FM's response arrived during measurement and raised the floor by eight section
identities and nine path mentions; that live change is itself evidence for
sealing an inbound inventory before migration. Every `draft1.*` file was
excluded so this independent response did not inspect peer work. The counts are
therefore a lower bound, not the final bill after this wave grows.

| Surface | Observed count | Consequence of a full subtree move |
|---|---:|---|
| Markdown concept files excluding `draft1.*` | 16 | 16 OKF document identities change |
| Explicit pinned anchors in those files | 416 | 416 composed Rekon section identities change |
| Document plus section identities | **432** | The minimum identity migration set before this wave |
| Literal `constitution/naming` path occurrences in those files | 295 | Each occurrence needs classification, not blind replacement |
| Markdown link destination occurrences under `/constitution/naming` | 147 | Live links need repair or intentional compatibility |
| Distinct Markdown destinations among those links | 30 | The controlled target set is smaller than the mention count |
| Deep-link occurrences carrying a fragment | 21 | File-only compatibility is insufficient for these citations |
| Distinct deep-link destinations | 13 | At least these fragment mappings are visible in controlled prose |
| Source files containing deep links | 4 | Fragment pressure is concentrated but real |
| Frontmatter path occurrences under `/constitution/naming` | 117 | Structured identity, lineage, and provenance fields need disposition |
| Top-level self `resource` values among those | 16 | Every moved file's redundant self-locator must change |
| Source-entry `resource` values among those | 71 | Historical evidence and live source navigation must be distinguished |
| Scalar relation paths among those | 11 | `revision_of`, `companion_to`, ruling, research, and exploration edges need review |
| List-valued relation paths among those | 18 | Multi-target lineage and response declarations need review |
| Free-text frontmatter path occurrences among those | 1 | One wrapped mandate/addendum statement also embeds the old path |
| Flattened `constitution-naming` or `rekon-constitution-naming` forms | 29 across 9 files | Each needs keep, retire, or alias disposition; not every form is a literal path link |
| Symlinks currently targeting the subtree | 0 | No existing symlink compatibility surface can be assumed |
| Beads records outside the subtree containing the path | 1 | `rekon-con-naming` carries one live forward anchor and one historical audit mention |
| Literal path mentions in that Beads record | 2 | The forward anchor should move if its target moves; the historical note should not be rewritten as if history differed |

The counting method used `fd` to enumerate Markdown files while excluding
`draft1.*`, `rg '^<a id="'` for explicit anchors, and targeted `rg -o`
patterns for links, fragments, frontmatter paths, and flattened forms. A scan of
tracked and normally visible Markdown and JSONL under `~/src`, excluding the
subtree and unread wave, found no other local project citation; the Beads record
is the only controlled reference outside the topic. That is not evidence that
remote repositories, offline checkouts, copied prose, or human memory contain
none.

The categories overlap. The 147 Markdown targets and 117 frontmatter values are
subsets of the 295 literal path occurrences, not numbers to add to it. The 432
identity count is a different axis: it measures what gets renamed even if no
string currently cites it.

<a id="what-the-counts-mean"></a>

## What The Counts Mean

The largest bill is not textual replacement. It is the identity decision for
416 sections. A bulk `sed` could rewrite many strings; it cannot decide whether
a historical source path should remain historical, whether an old fragment has
a durable external consumer, or whether a model-suffixed evidence artifact
should move at all.

The Beads record demonstrates the required judgment in one line. Its forward
anchor is a current promise to a future or maintained knowledge target. If that
target is renamed, the work authority should update it through a guarded
operation. Its later note says where the ask audit landed as an event in
history. Rewriting that note would make the audit trail falsely describe the
past. `naming-reference-registers` and the evidence-binding register exist for
exactly this distinction.

The 71 frontmatter source paths carry the same split. A source entry that means
"follow the current account" may migrate. A source entry that means "this
author consulted these bytes at this path" should become revision-bound or keep
the historical path. Location identity makes citation intent more important,
not less.

<a id="move-less-than-you-promote"></a>

## Move Less Than You Promote

The rename ripple is an accepted cost. Accepted does not mean gratuitous. The
corpus's own `naming-successor` ambition and the constitution's promotion model
suggest a smaller, more honest operation than moving the entire exploration
history:

1. Write the accepted maintained synthesis at the desired current home, such as
   a future `/naming/README.md`, once the human chooses that home.
2. Treat that placement as a new current document identity that explicitly
   supersedes the prior public account.
3. Leave model-suffixed proposals, drafts, audits, and system research at the
   exact paths under which they were authored and cited.
4. Convert the old public presence into a
   `naming-retired-path-tombstone` or a historical navigation page according to
   the chosen succession relation.
5. Update live indexes, current-account links, and the Beads forward anchor.
6. Preserve exact historical citations rather than rewriting evidence to look
   newly born.

This is not a covert return to move-stable identity. The new maintained home has
a new location identity, and the old public identity receives explicit
disposition. It simply classifies the operation as synthesis and supersession
rather than pretending every evidence artifact must relocate with the current
account.

If the human instead wants the full subtree at root, the 432-identity baseline
is the price before this wave, and hierarchy1's migration map, inbound
inventory, human gate, controlled rewrite, and compatibility review are the
right machinery. The ruling makes that move permissible. The supersede ambition
does not make it necessary.

<a id="actors-hats-and-cross-modal-aliases"></a>

# Actors Hats And Cross Modal Aliases

The ruling on actors is also correct. A declared hat passes the same family test
as a citation token: it maps a concern-scoped reference to one canonical owner.
`naming-role-is-relation` survives after losing exclusivity. A hat answers how
an actor may be addressed here; a role edge records what the actor undertook,
under what scope and history. `naming-actor-braid` needs both rows.

Six actors preceded this response by self-naming, including the root session as
`steward/glm-5.3-max`. This response makes a seventh self-proposal:
**`surveyor/gpt-5.6-sol-max`**, chosen because this artifact surveys a namespace
boundary and measures a migration. The actual generating model remains the
separate, environment-supplied `model:openai/gpt-5.6-sol#max` in
`generated.by`.

Within `rekon-con-naming`, the actor declares two hats:

| Hat | Concern | Maps to |
|---|---|---|
| `respondent` | Taking a fresh position on the human ruling | `surveyor/gpt-5.6-sol-max` |
| `migration-auditor` | Counting and interpreting the contemplated path move | `surveyor/gpt-5.6-sol-max` |

The hats are not peer actors. The self-name and aliases are unilateral promises;
their shared currency remains for the receiving authority to accept. If no roll
accepts the actor, the claim still remains honest artifact provenance, not a
globally allocated identity.

<a id="one-name-still-means-one-owner"></a>

## One Name Still Means One Owner

Restored hats and aliases do not weaken `naming-one-name-locality`. The invariant
is ownership and precedence, not conversational vocabulary:

- One current regular location owns a knowledge identity in one bundle
  authority.
- One canonical actor claim owns the concern-scoped hats it declares.
- One Beads record owns `rekon-con-naming` in the work plane.
- A token, hat, abbreviation, symlink exposure, or retired path needs a route to
  its owner or successor.
- Matching semantic components across planes establish sympathy, not sameness.

The duplicate-drift finding therefore survives untouched. A second live body
at another path is a second artifact. A second actor name without an alias map
is a competing promise. A tombstone and a successor do not compete because
their dispositions and relation are explicit.

<a id="aliases-can-cross-without-erasing-boundaries"></a>

## Aliases Can Cross Without Erasing Boundaries

`naming-cross-modal` and `naming-aliases` are restored as ordinary practice.
The right constraint is not advance permission but recoverability:

- `con` may abbreviate `constitution` in a local glossary or native work ID.
- A flat spelling may project a slash path into a medium that rejects slashes.
- A hat may address one actor by concern.
- A citation token may compress one section inside a carried table.
- A retired path may lead to a successor through an explicit compatibility
  relation.

These forms differ operationally, but `naming-alias-collapse` gives them one
failure test: if context or declaration cannot recover one intended owner, the
form has become ambiguous or competitive. Governance is then earned by actual
pressure. Before then, a nearby declaration is enough.

<a id="new-law-and-existing-law"></a>

# New Law And Existing Law

<a id="law-to-add-now"></a>

## Law To Add Now

Two additions are constitutional rather than tooling details:

1. **`naming-bundle-boundary`:** a path identity has meaning only inside a
   declared bundle authority and root; changing that frame is identity change,
   while moving an intact checkout is not.
2. **`naming-retired-path-tombstone`:** a regular Markdown stub at an old path
   is the deprecated presence of the old concept, with an explicit successor,
   not an identity-free alias or a competing current body.

Without the first, Concept IDs depend on the consumer's current directory.
Without the second, the compatibility machinery contradicts OKF's statement
that non-reserved Markdown files are concepts and leaves symlink behavior to
accident.

<a id="law-to-extend-without-new-names"></a>

## Law To Extend Without New Names

Everything else can be carried by identifiers already minted:

- Extend `naming-identity-change` with intact-bundle transport, fork, copy,
  cross-authority transfer, bundle-root change, and section extraction.
- Extend `naming-reference-registers` so a native path can occupy both Name and
  Locator roles, while a revision-pinned path remains an evidence binding.
- Extend `naming-rename-covenant` to record old and new authority, bundle root,
  path, affected durable fragments, and compatibility disposition.
- Bound `naming-location-fixed-point` with `naming-promise-jurisdiction`: the
  current tree fuses name and referent inside one living authority, while
  federation and retired-name mappings remain promises that authority must keep.
- Keep `naming-resolver` demoted from canonical identity enforcement to aliases,
  scale changes, history, collision reporting, and cross-authority lookup.
- Use `naming-convergence-repair` for controlled rewrites and repeated checks,
  never as a claim of global atomicity.
- Keep `naming-coordinate-context`: the tree names and locates, but directory
  ancestry does not prove semantic containment, causation, or supersession.
- Keep `naming-work-knowledge-seam`: `rekon-con-naming` coordinates this
  document; shared words do not merge their speech acts.
- Keep `naming-reference-pressure` and `naming-ceremony-budget` to decide which
  old fragments earn compatibility and which aliases earn stronger governance.
- Keep `naming-tool-seam` deferred. It now has clearer invariants to derive
  behavior from, not permission to hide unsettled policy in code.
- Keep `naming-successor` and `naming-supersede-proof`: the new home should earn
  authority as a maintained synthesis, not by moving every source beside it.

`naming-preference-priced-as-law` remains an important caution from Weaver. It
also cuts the other way now: location identity is settled policy, but it should
not be inflated into a demand that every promotion physically move its evidence.
Choose rename, extraction, copy, and supersession by what happened.

<a id="order"></a>

# Order

The response treats the following as settled enough for successor language:

- Location is identity for tree-resident knowledge.
- The identity is bundle-relative, not checkout-absolute.
- A pinned anchor extends the document identity at section grain.
- Placement and anchoring are stewarded mint acts.
- File, anchor, and bundle-root moves are identity changes.
- Trust and disposition remain orthogonal to location.
- Work, actors, computations, and evidence retain native authorities.
- Hats are declared actor aliases; roles are complementary relations.
- Cross-modal aliases need no advance central permission.
- One-byte spelling across media is not required.
- Historical evidence citations and live semantic citations have different
  repair behavior.
- Regular compatibility Markdown remains an identity-bearing deprecated
  presence.
- The rename ripple is accepted and must be priced before movement.
- Tool behavior remains deferred until derived from accepted law.

<a id="nebula"></a>

# Nebula

The following material should remain visibly unsettled rather than be guessed
into the compact:

- Whether Rekon's accepted bundle root is the repository root, one or more
  declared subtrees, or a packaged composition. This response recommends the
  repository root from current evidence but cannot enact it.
- The canonical authority identifier and discovery mechanism used across
  projects.
- The serialization that preserves path-segment boundaries in flat media
  without demanding one byte string everywhere.
- The minimum portable envelope for a detached standalone document.
- The relation vocabulary for mirror, fork, translation, extraction, and
  canonical ownership transfer.
- How long a public old path must remain and what evidence permits deliberate
  breakage for third-party citers.
- What survives an authority's death, the limit named by
  `naming-promise-jurisdiction`, and whether any successor may inherit its
  compatibility promises without pretending to be the old authority.
- Which publication surfaces preserve symlinks and deep fragments, and whether
  any are acceptable as canonical compatibility mechanisms.
- Whether every durable old section needs a tombstone or only sections with
  observed reference pressure.
- How a resolver presents the old concept, its tombstone state, and its current
  successor without silently collapsing their identities.
- When a nearby alias declaration has grown enough to earn a project glossary
  or checked registry.
- When a concern expressed as a hat has developed enough independent agency to
  become an actor.
- Whether the accepted successor should live at `/naming/README.md`, remain
  under `/constitution/naming/`, or take another domain-grouped home.

These questions are not evidence against the ruling. They are the nebula around
the order it creates.

<a id="identifier-ledger"></a>

# Identifier Ledger

<a id="minted-here"></a>

## Minted Here

Two semantic identifiers are minted and used as proposed laws:

| Identifier | Sole referent in this response |
|---|---|
| `naming-bundle-boundary` | The authority and bundle-root frame required to interpret a location-derived Concept ID |
| `naming-retired-path-tombstone` | The deprecated identity-bearing Markdown presence left at a retired path and linked to its successor |

One actor name is self-proposed: `surveyor/gpt-5.6-sol-max`. Two scoped hats,
`respondent` and `migration-auditor`, are declared as aliases to that actor in
the `rekon-con-naming` scope. They do not mint additional actors.

The section-local anchor identifiers minted by this artifact are:

`location-identity-needs-boundaries-and-tombstones`, `position`,
`the-ruling-changes-the-source-of-knowledge-names`,
`what-survives-and-how-it-changes`, `naming-bundle-boundary`,
`the-location-identity-tuple`, `travel-is-not-one-operation`,
`standalone-documents-carry-origin-without-carrying-identity`,
`cross-project-citation-retains-delimiters`,
`section-identity-is-a-rekon-extension`, `promotion-and-extraction`,
`naming-retired-path-tombstone`, `why-the-stub-is-an-entity`,
`deep-fragment-compatibility`, `symlinks-need-rekon-law`,
`the-rename-ripple-priced-in-this-corpus`, `the-counting-snapshot`,
`what-the-counts-mean`, `move-less-than-you-promote`,
`actors-hats-and-cross-modal-aliases`, `one-name-still-means-one-owner`,
`aliases-can-cross-without-erasing-boundaries`, `new-law-and-existing-law`,
`law-to-add-now`, `law-to-extend-without-new-names`, `order`, `nebula`,
`identifier-ledger`, `minted-here`, `adopted-as-is`,
`mechanical-anchor-verification`, and `cross-references`.

The frontmatter source keys `ruling`, `override-companion`, `ask-audit`,
`weaver-synthesis`, `concordance-synthesis`, `codex-synthesis`, `fm-response`,
`proposal2`, `canonical-constitution`, `hierarchy1`, `okf-spec`, and
`naming-work` are scoped provenance identifiers, not semantic owners.

<a id="adopted-as-is"></a>

## Adopted As Is

The response adopts and uses these existing identifiers without reminting them:

| Concern | Existing identifiers used |
|---|---|
| Human ruling and knowledge identity | `naming-implied-names`, `naming-placement-at-mint`, `naming-one-name-locality`, `naming-name-promise` |
| Reference family | `naming-reference-registers`, `naming-citation-token`, `naming-derived-vs-declared`, `naming-slug-at-mint`, `naming-alias-collapse` |
| Change and repair | `naming-identity-change`, `naming-rename-ripple`, `naming-rename-covenant`, `naming-convergence-repair`, `naming-preference-priced-as-law` |
| Context, travel, and federation | `naming-coordinate-context`, `naming-extraction-asymmetry`, `naming-location-fixed-point`, `naming-promise-jurisdiction`, `naming-native-authority`, `naming-federated-authority`, `naming-plane-federation` |
| Actors and aliases | `naming-address-vs-identity`, `naming-agents-hats`, `naming-role-is-relation`, `naming-actor-braid`, `naming-promise-currency-split`, `naming-cross-modal`, `naming-aliases` |
| Proportionality and planes | `naming-speech-acts`, `naming-work-knowledge-seam`, `naming-reference-pressure`, `naming-ceremony-budget` |
| Home and deferred machinery | `naming-home-not-layout`, `naming-successor`, `naming-supersede-proof`, `naming-resolver`, `naming-tool-seam` |
| Native work identity | `rekon-con-naming` |

`naming-location-separation` appears only for explicit disposition: it is
overruled for knowledge location and is not adopted as current law here.

<a id="mechanical-anchor-verification"></a>

# Mechanical Anchor Verification

Every heading was composed so that its Pandoc GFM-derived slug equals the
explicit anchor immediately before it. The complete file was rendered with
`pandoc --from=gfm --to=html`, and the declared anchor sequence was compared in
document order with the rendered heading ID sequence. Result: **thirty-two of
thirty-two anchors match**, with zero corrections required.

This verifies `naming-slug-at-mint` against Pandoc. It does not settle
cross-renderer behavior, symlink traversal, or fragment-aware compatibility;
those remain in the nebula at `naming-tool-seam`.

<a id="cross-references"></a>

# Cross References

- [`proposal4.glm53m.md`](/constitution/naming/exploration/proposal4.glm53m.md)
  **governs** this response. Its location-identity, accepted-ripple, hats, and
  cross-modal-alias rulings are premises, not questions reopened here.
- [`proposal4.sol56x.md`](/constitution/naming/exploration/proposal4.sol56x.md)
  **works through and then pressures** the ruling. Its portability addendum
  identifies actual-move pricing and bundle boundaries as debt; this response
  supplies the corpus count and promotes the boundary to proposed law.
- [`ask-audit0.glm53m.md`](/constitution/naming/exploration/ask-audit0.glm53m.md)
  **holds the accountability denominator**. It confirms that no original human
  ask may disappear when location identity is extended operationally.
- [`proposal3.glm53m.md`](/constitution/naming/exploration/proposal3.glm53m.md),
  [`proposal3.sol56x.md`](/constitution/naming/exploration/proposal3.sol56x.md),
  and
  [`proposal3b.glm53m.md`](/constitution/naming/exploration/proposal3b.glm53m.md)
  **supply the surviving architecture and respondent corrections**. Their
  addenda converge on revision, while Weaver names the externality, Concordance
  exposes bundle and fragment boundaries, and Codex names placement and
  extraction.
- [`proposal2.glm53m.md`](/constitution/naming/exploration/proposal2.glm53m.md)
  **supplies the restored wiki settlement**, the original rename ripple, hats,
  cross-modal sympathy, and the supersede ambition.
- [`draft0.glm53fm.md`](/constitution/naming/exploration/draft0.glm53fm.md#addendum-fm-responds-to-the-override-ruling)
  **extends the response set** with the location fixed point, resolver demotion,
  and promise jurisdiction. Its arrival during this audit also demonstrates why
  a migration inventory needs a sealed revision rather than a moving worktree.
- [OKF v0.2](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md#L71-L94)
  **defines the receipt**: one Concept is one Markdown document and its Concept
  ID is the path within the bundle without `.md`. Its bundle and conformance
  sections also motivate the boundary and tombstone corrections.
- [`README-heirarchy1.glm53m.md`](/constitution/README-heirarchy1.glm53m.md)
  **supplies the payment procedure**: migration map, inbound link inventory,
  human gate, and compatibility surfaces. This response refines what a regular
  Markdown compatibility stub means under path identity.
- [`constitution/README.md`](/constitution/README.md)
  **constrains promotion and evidence**. Promotion as synthesis, exact
  model-suffixed evidence, semantic anchors, and prospective adoption support
  creating a new maintained home without moving the whole historical wave.
- Beads work item [`rekon-con-naming`](/.beads/issues.jsonl)
  **coordinates** this knowledge and contains the only controlled path citation
  found outside the topic: a live forward anchor plus a historical path mention
  that should receive different migration treatment.
