---
type: ConstitutionRevision
title: Topic-first hierarchy with README presence and frontmatter disposition
description: Revision of hierarchy0 that retires the design/ posture directory, makes the topic README the public presence, and adds terms of art including docroot and disposition.
resource: /constitution/README-heirarchy1.glm53m.md
tags: [documentation, hierarchy, glossary, okf, disposition, promotion, migration]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-09-02T15:50:42-04:00 }
extensions:
  ticket: rekon-doc-constitution-hierarchy
  local_knowledge_id: doc-constitution-hierarchy1
  revision_of: /constitution/README-heirarchy0.sol56x.md
  precedence: none until accepted
sources:
  - id: hierarchy0
    resource: /constitution/README-heirarchy0.sol56x.md
    title: Topic-first documentation hierarchy (revision 0)
    author: model:gpt-5.6-sol-max
  - id: canonical-constitution
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution
    author: project:rekon
  - id: okf-spec
    resource: file:///home/rektide/archive/GoogleCloudPlatform/knowledge-catalog/okf/SPEC.md
    title: Open Knowledge Format v0.2
    author: project:knowledge-catalog
    last_modified: 2026-06-30
  - id: direction-session
    resource: urn:opencode:session:ses_f9cf5c4bbffep3k7RE7wv2frLM
    title: Glossary, docroot, and design-directory dissolution direction
    author: human:rektide
  - id: workspace-guidance
    resource: /AGENTS.md
    title: Workspace agent guidance (waves, doc-pass, scratch)
    author: human:rektide
---

<a id="doc-constitution-hierarchy1"></a>
# Topic-First Hierarchy, Revised: README Presence And Frontmatter Disposition

This is revision 1 of the topic-first hierarchy proposal, revising
[`README-heirarchy0.sol56x.md`](/constitution/README-heirarchy0.sol56x.md).
The filename keeps the historical `heirarchy` spelling so the revision chain
stays greppable beside the beads forward anchor that cites revision 0. It does
not yet modify the canonical constitution or authorize moving existing files.

Stage setting: revision 0 proposed topic-first containment but kept the two
posture directories, `research/` and `design/`, inside each topic. Since then
the direction has sharpened. The `design/` directory is now understood as a
mid-refactor artifact that can be dissolved entirely: each topic's README is
the public presence, sundry details live in other files and subdirs, and the
reader-trust meaning that directories used to carry moves into per-file OKF
frontmatter. What was missing was vocabulary — for the root document
directory, for legacy structures, for disposition, for migration moves. This
revision adds that terms-of-art layer and re-grounds the hierarchy on it.

The three changes that matter:

1. **The `design/` posture directory is retired.** Intention-work's destiny is
   absorption into the topic's README and support docs; it does not need a
   home directory. `research/` survives only as an optional noise container,
   not as half of a posture pair.
2. **Disposition replaces location.** What a reader should trust is stated in
   each artifact's OKF frontmatter (`status`, `generated`, `verified`,
   `stale_after`), uplifted from the [Open Knowledge Format
   v0.2](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)
   (local copy consulted:
   [SPEC.md](file:///home/rektide/archive/GoogleCloudPlatform/knowledge-catalog/okf/SPEC.md)),
   rather than by which storage class the file sits in.
3. **A terms-of-art layer is added** so migrations and agent instructions can
   say precisely what they mean: `docroot`, `docroot declaration`, `root
   topic`, `legacy doc structures`, `sundry artifacts`, `disposition`,
   `docroot migration`, `compatibility stub`, `doc-pass`, and the rest
   defined below.

<a id="doc-constitution-hierarchy1-terms"></a>
# Terms Of Art

These terms are the vocabulary this hierarchy rallies behind. Groups marked
**(new)** are coined or formalized by this revision; the rest are inherited
from the canonical constitution or revision 0 with their meaning tightened.

<a id="doc-constitution-hierarchy1-terms-layout"></a>
## Project Layout **(new)**

| Term | Definition |
|---|---|
| **docroot** | The declared top of a project's knowledge hierarchy. `doc/` by default; upstream projects commonly use `docs/`; carry contexts historically used `design/` or `.design/`. Makes migration sentences precise: "is-tree's docroot is `.design/`; the proposal migrates it to `doc/`." |
| **docroot declaration** | A one-line, machine-findable statement of the docroot — and any grandfathered legacy roots — in root README frontmatter or `AGENTS.md`. Absence means the default `doc/`. See [Docroot Declaration](#doc-constitution-hierarchy1-docroot). |
| **root topic** | A topic directory at repository root instead of under the docroot (`constitution/`, `prompt/` in rekon). A documented composition choice, not an anomaly. |
| **legacy doc structures** | The semi-formal, often informal predecessors of this hierarchy: the `design/`-vs-`doc/` storage classes, `.design/` trees, upstream `docs/`, `doc/discovery/`, `doc/plan/`. Grandfathered and interpreted through the [legacy mapping](#doc-constitution-hierarchy1-legacy), never treated as errors requiring immediate movement. |
| **reserved names** | Names with fixed meaning inside a topic: `README.md`, `SKILL.md`, `GLOBAL.md`, `index.md`, `log.md`, and the optional `research/` and `archive/` directories. |

<a id="doc-constitution-hierarchy1-terms-topic"></a>
## Topic And Presence

| Term | Definition |
|---|---|
| **topic** | One domain-grouped directory under the docroot: `doc/<topic>/`. The unit of navigation, noise containment, and filtering. |
| **public presence** | The topic README: the account a reader or agent trusts first. Replaces revision 0's "real space." |
| **sundry artifacts** **(new)** | Everything beside the README: detail, evidence, waves, guides, experiments. Their trustworthiness is stated by frontmatter disposition, not by location. |
| **knowledge module** | A topic operating the README/SKILL/GLOBAL contract inherited from the canonical constitution. |
| **exposures** | The channels one body serves: README (navigational), `SKILL.md` (on-demand), `GLOBAL.md` (ambient). One body, several exposures. |

<a id="doc-constitution-hierarchy1-terms-disposition"></a>
## Disposition **(new, OKF uplift)**

| Term | Definition |
|---|---|
| **disposition** | The reader-trust state of an artifact, carried in its frontmatter rather than its directory. What `design/` versus `doc/` used to say by storage class. |
| **status** | `draft` (working material, possibly incomplete), `stable` (supported, ready for consumption), `deprecated` (kept for links and history, no longer current). Absent means `stable`, so drafts MUST say so. |
| **generated / verified** | Who wrote the content versus who confirmed it against its sources. Kept distinct because the writer is not the checker. |
| **trust tier** | Derived from `verified`: unverified (no key), machine-confirmed (non-`human:` actors only), human-reviewed (a `human:` actor). Advisory signal, not access control. |
| **stale_after** | An absolute instant on or after which the content is due for review. Absolute, not a relative TTL, so staleness is a plain comparison. |
| **supersession** | Marking a superseded artifact `deprecated` and pointing at its successor. A local `superseded_by` frontmatter key is a conformant OKF producer extension. |

<a id="doc-constitution-hierarchy1-terms-waves"></a>
## Waves And Naming

| Term | Definition |
|---|---|
| **wave** | One round of independently authored artifacts on one question; same-wave authors do not read peers when independence is the evidence. |
| **wave role** | The filename's leading role label: `init`, `draft`, `syn`, `adversarial`, and similar. Orthogonal to disposition. |
| **revision number** | The trailing counter distinguishing rounds of the same role (`draft2`, `feature3-syn2`). |
| **model suffix** | The `.<model>.md` tail naming the actual authoring model, never the harness. Every wave file carries one. |
| **posture prefix** | The `research-`/`design-` filename fallback, used only when an artifact must live outside its topic. With posture directories gone this fallback, not a parent directory, is what carries the claim when location cannot. |

<a id="doc-constitution-hierarchy1-terms-promotion"></a>
## Promotion

| Term | Definition |
|---|---|
| **promotion** | Absorbing accepted working material into the public presence. Changes reach, not history: sources cite the exact artifacts, which stay where they are. |
| **maintained-README mode** | The default: the README is a real, continuously edited file that integrates accepted conclusions coherently. |
| **winner-symlink mode** | The exception for topics that are essentially one artifact: README symlinks to the exact accepted file, which stays immutable and model-attributed. |
| **accepted tip** | A suffix-less symlink naming the current winner, so stable citations do not encode model identity. Inherited from the canonical constitution's design-program practice. |

<a id="doc-constitution-hierarchy1-terms-migration"></a>
## Migration And Adoption

| Term | Definition |
|---|---|
| **grandfathering** | Legacy doc structures remain valid until a topic is materially reorganized. Location does not retroactively certify anything. |
| **prospective adoption** | New topics use the hierarchy; existing trees are migrated only deliberately, topic by topic, never in bulk. |
| **contract adoption vs layout migration** | A project can adopt honest identity, status, anchors, and sources without moving a single file. The two changes are separable. |
| **upstream-native layout** | Upstream-bound work follows the upstream project's own documentation conventions; layout is not identity. |
| **carry island** | A workspace-owned subtree carrying local design, research, and guidance across upstream rebases. Inherited; default root was `design/<topic>/`, which legacy mapping now folds into docroot topics. |
| **docroot migration** **(new)** | Deliberately adopting or renaming a project's docroot, e.g. `.design/` → `doc/`. A layout migration whose scope is the root itself. |
| **migration map** **(new)** | The per-topic old-path → new-path plan a migration follows, with explicit non-actions. |
| **inbound link inventory** **(new)** | The exact set of links, anchors, frontmatter `resource`s, symlinks, and ticket forward anchors targeting pre-move paths. Assembled before moving anything. |
| **compatibility stub** **(new)** | A small file or symlink left at an old path to preserve its old meaning while one canonical owner takes over. The retired OpenCode `plugin.md` demonstrates the pattern. |

<a id="doc-constitution-hierarchy1-terms-lifecycle"></a>
## Lifecycle And Process

| Term | Definition |
|---|---|
| **scratch / pre-history** | `.test-agent/<topic>/`, git-ignored. Evidence that stays load-bearing at acceptance must be absorbed into the public presence or promoted to a committed location. |
| **doc-pass** **(new here, practice established)** | The post-writing pass that connects new work into the corpus: cross-references with stated relationships, reciprocal links to older documents, and index/README maintenance. Promoted from `AGENTS.md` practice to a term of art. |

<a id="doc-constitution-hierarchy1-shape"></a>
# The Topic Shape

```text
doc/<topic>/
  README.md                 public presence — what a reader trusts first
  SKILL.md -> README.md     optional on-demand exposure
  GLOBAL.md                 optional ambient contribution
  <sundry artifacts>        waves, evidence, guides; disposition in frontmatter
  research/                 optional container for noisy understanding work
  archive/                  optional deliberately-retired material
  index.md / log.md         when artifact count or history earns them
```

The topic README is the public presence. Everything else is sundry: useful,
citable, durable, but trusted or distrusted according to its own frontmatter,
not its address. This is what eliminates the need for a `design/` directory
entirely — the README absorbs the intention-work, the waves that produced it
stay beside it as evidence, and neither needs a posture peer to be
interpretable.

`research/` is demoted from posture to option. A topic whose
understanding-work would drown the README's neighborhood MAY keep it in
`research/`; a quiet topic simply does not create the directory. There is no
`design/` sibling because there is no longer a posture pair — only noise
containment.

The current `constitution/` module itself demonstrates the shape acceptably:
maintained README, `SKILL.md` and `GLOBAL.md`, and revision waves sitting
beside the README as sundry artifacts.

<a id="doc-constitution-hierarchy1-disposition"></a>
# Disposition Over Location

Revision 0 kept the storage-class instinct one level down: posture
directories inside topics. This revision completes the move that revision 0
started. Trust is a property of an artifact's claims and checking, so it is
stated where the claims live — in OKF frontmatter — rather than in the tree.

The uplifted machinery, following OKF v0.2 §5 and §7:

```yaml
---
type: DesignProposal
title: Promotion flow
status: draft                      # draft | stable | deprecated
generated: { by: model:glm-5.3-max, at: 2026-09-02T15:50:42-04:00 }
# verified appears only when someone actually checked against sources:
verified: { by: human:rektide, at: 2026-09-02T16:00:00-04:00 }
stale_after: 2026-12-01            # optional review deadline
---
```

The reader contract this buys:

- the topic README is the first account to trust;
- a sundry artifact says what it is: `status: draft` working material,
  `stable` support docs, `deprecated` kept-for-history;
- `generated` names the writer, `verified` names an actual checker — a
  missing `verified` is honestly unverified, never silently trusted;
- the `human:` actor prefix is what separates human review from
  machine confirmation;
- `stale_after` makes review debt visible instead of latent.

Two cautions. First, `verified` MUST mean checked against sources or the
resource, not reread by its author; the trust tier is only as good as that
honesty. Second, small documents stay proportional — OKF conformant means
`type` plus whatever the audience justifies, and deliberate notes need not
grow bureaucracy.

The research-versus-design claim distinction survives as content, not
location. Revision 0's principle "what appears true and what we intend to
make true remain distinguishable" still holds; a design proposal is labeled
by `type` and prose, marked `draft`, and cites its research. See
[tensions](#doc-constitution-hierarchy1-tensions).

<a id="doc-constitution-hierarchy1-promotion"></a>
# Promotion: Absorbing The Core

Promotion is the act this hierarchy exists to make honest: getting accepted
working material into the public presence without pretending the evidence
away. It changes reach, not history.

The sequence, in maintained-README mode (the default):

1. **Waves happen** in the topic: model-suffixed files, `status: draft`,
   unverified or machine-confirmed.
2. **Acceptance** is a human decision. The accepted synthesis gains
   `verified: { by: human:<id>, at: ... }` — this is the moment a trust tier
   changes.
3. **The README integrates** the accepted claims as coherent current prose,
   frontmatter `sources` citing the exact wave files that were reconciled.
4. **Superseded waves are marked**: `status: deprecated` with
   `superseded_by` pointing at the README or successor artifact. The files
   stay in place; exact citations keep meaning what they meant.
5. **Navigation updates**: README context pointers, `index.md` when present,
   and `log.md` entries; a doc-pass connects the change to the rest of the
   corpus.

Winner-symlink mode remains available for topics that are essentially one
artifact — a selected synthesis the topic simply exposes:

```text
doc/skillify/
  README.md -> skillify3.sol56x.md
  SKILL.md -> README.md
  skillify0.glm53m.md            superseded evidence
  README.sol56x.md               superseded evidence
  README1.sol56x.md              superseded evidence
  skillify3.sol56x.md            winner; immutable, model-attributed
```

Choosing and changing modes stays deliberate, exactly as revision 0 defined:
symlink → maintained means synthesizing the winner into a real README with
honest sources; maintained → symlink means preserving the old README as a
cited historical artifact first. A symlink chain `SKILL.md -> README.md ->
winner` is acceptable when hosts follow it; packaging tooling must validate
that behavior.

<a id="doc-constitution-hierarchy1-naming"></a>
# Naming Within A Topic

With posture directories gone, filenames carry wave identity and frontmatter
carries disposition. The wave grammar is unchanged:

```text
<role><revision>.<model>.md          draft0.sol56x.md, feature3-syn2.glm53m.md
```

The posture prefix survives as a fallback for artifacts that must live
outside their topic:

```text
research-<topic-or-question>.<model>.md
design-<topic-or-proposal>.<model>.md
```

Do not require `research/research-foo.md`; a prefix that merely restates its
parent directory is noise. Every wave file still ends in its actual concise
model suffix, never the harness name.

<a id="doc-constitution-hierarchy1-docroot"></a>
# Docroot Declaration

Agents and migrations should not have to guess where a project's knowledge
hierarchy starts. A project SHOULD declare its docroot when it differs from
the default, in a place a machine can find — root README frontmatter or
`AGENTS.md`:

```yaml
docroot: doc/                # default when absent
docroot_legacy:              # optional, grandfathered, still populated
  - design/
  - .design/
```

Rules:

- absence means `doc/`;
- `docroot_legacy` lists legacy roots still carrying material; it is a
  grandfathering statement, not a migration plan;
- upstream-conforming projects record the upstream convention
  (`docroot: docs/`) rather than renaming it;
- root topics (topics at repository root) may be noted the same way when a
  project uses them.

The declaration is a routing fact, not a new file format: one key, read by
grep.

<a id="doc-constitution-hierarchy1-legacy"></a>
# Legacy Doc Structures

Legacy doc structures are the semi-formal, often informal arrangements this
hierarchy succeeds. They are grandfathered facts about a corpus, interpreted
through this mapping — kept near the convention so older prompts and
repositories stay easy to read:

| Legacy form | New preferred home | Notes |
|---|---|---|
| `design/<topic>/` | `doc/<topic>/` | Wave files sit beside the topic README; disposition moves into frontmatter. No `design/` level is recreated. |
| `.design/<topic>/` | Grandfather in existing projects; new local work uses the docroot. | Earlier hidden design-root convention; no mass migration implied. |
| `doc/discovery/<topic>/` | `doc/<topic>/research/` or `doc/<topic>/` | `discovery` was the earlier term of art for source-grounded understanding work. |
| `doc/plan/<topic>.md` | `doc/<topic>/` support docs | Plans are intention-work absorbed into the topic. |
| Stable `doc/<topic>.md` | `doc/<topic>/README.md` | Maintained public knowledge moves behind a topic landing page. |
| Model-suffixed waves under any older root | Same exact artifact under its topic when deliberately migrated | Model identity, anchors, and historical interpretation unchanged. |
| Upstream `docs/`, `website/docs/`, etc. | Unchanged | Upstream-native layout is not an error; the docroot declaration records it. |
| `.test-agent/<topic>/` | Unchanged | Disposable pre-history stays outside the committed hierarchy. |

The mapping is for navigation and prospective naming. It does not authorize
bulk movement, link rewriting, or automatic cleanup.

<a id="doc-constitution-hierarchy1-migration"></a>
# Migration Vocabulary And Restraint

A layout migration is described with four named moves:

1. **Docroot migration** — adopt or rename the root itself
   (`.design/` → `doc/`, or declaring an upstream `docs/` as the docroot).
2. **Migration map** — the per-topic old-path → new-path plan, with explicit
   non-actions for material that deliberately stays.
3. **Inbound link inventory** — every link, anchor, frontmatter `resource`,
   symlink, and ticket forward anchor targeting pre-move paths, assembled
   before anything moves.
4. **Compatibility stubs** — files or symlinks left at old paths with durable
   consumers, each pointing at the new canonical owner.

The restraint revision 0 established carries forward unchanged: prospective
adoption, topic-by-topic deliberation, no pruning of stale or undeclared
artifacts, contract adoption separable from layout migration, and
upstream-bound work following upstream-native layout.

<a id="doc-constitution-hierarchy1-tensions"></a>
# Tensions To Test

1. **Frontmatter discipline is the new gate.** Disposition only works if
   drafts actually carry `status: draft` and model identity. A forgotten
   frontmatter block is unmarked sundry noise — worse than a wrong
   directory, because nothing in the tree hints at it. Validators and agent
   habit have to carry this.
2. **Claim distinction without location.** With no `design/` directory,
   nothing structural separates "what appears true" from "what we intend to
   make true." `type`, prose, and posture prefixes must carry it. Does that
   hold up under fatigue?
3. **Sundry crowding.** Busy topics could drown the README's neighborhood.
   `research/`, `archive/`, and `index.md` are the relief valves; when do
   they stop being enough and the topic splits?
4. **Proportionality of OKF.** How much frontmatter is justified for small
   support docs before the ceremony exceeds the audience? `type`-only
   minimums remain conformant; practice must not inflate.
5. **Trust-tier honesty.** `verified` must mean checked against sources.
   Model-authored documents rereading themselves must never write a
   `human:` verification event.
6. **Symlink portability.** Do every forge, package, and skill loader follow
   the `README -> winner` and `SKILL -> README` chains correctly? (Kept
   from revision 0.)
7. **Declaration discoverability.** Will agents actually read the docroot
   declaration, or guess anyway? The default-on-absence rule bounds the
   damage.
8. **Winner ambiguity.** Mid-lifecycle topics may look like both a
   maintained README and a winner-in-waiting. Mode choice guidance may need
   a decision procedure revision 0 did not supply.

<a id="doc-constitution-hierarchy1-amendment"></a>
# Proposed Canonical Amendment

If accepted, replace the canonical constitution's `design/` versus `doc/`
storage-class model with this rule:

> Documentation is topic-first under the project's **docroot** (`doc/` by
> default, declared when it differs). Each topic's README is its **public
> presence**; sundry artifacts live beside it and state their **disposition**
> in OKF frontmatter — `status`, `generated`, `verified`, `stale_after`. The
> `design/` storage class is retired: intention-work is absorbed into topic
> READMEs through promotion that cites its exact sources and marks what it
> supersedes. `research/` remains an optional container for noisy
> understanding work and succeeds the historical `discovery` term. Legacy doc
> structures are grandfathered and mapped, never treated as errors.

The module's README/SKILL/GLOBAL contract, durable anchor rules, evidence
discipline, ticket balance, parent AGENTS assembly, and prospective adoption
policy otherwise continue unchanged.

<a id="doc-constitution-hierarchy1-cross-references"></a>
# Cross-References

- [`README-heirarchy0.sol56x.md`](/constitution/README-heirarchy0.sol56x.md)
  **is revised by** this document. Its topic-first containment, promotion
  modes, historical mapping, and restraint survive; its posture-directory
  pair and "real space" vocabulary are superseded.
- The [canonical constitution](/constitution/README.md) **is amended by**
  this proposal only after acceptance; its storage-class model remains
  authoritative meanwhile, and its promotion-as-synthesis section already
  anticipates absorption over movement.
- The [OKF v0.2 SPEC](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)
  (local copy:
  [SPEC.md](file:///home/rektide/archive/GoogleCloudPlatform/knowledge-catalog/okf/SPEC.md))
  **defines** the disposition family uplifted here: `status` §5.4,
  `generated`/`verified` and trust tiers §5.2–5.3, `stale_after` §5.5, the
  actor convention §7, and producer-extension permission §4.1.
- [`AGENTS.md`](/AGENTS.md) **establishes** the wave, scratch, and doc-pass
  practice this revision promotes to terms of art, and **will carry** the
  rekon docroot declaration.
- The [historical discovery workflow](/prompt/.archive/workflows/discovery.md)
  **evidences** `discovery` as an actual workspace term of art that
  `research/` succeeds.
- [`README1.sol56x.md`](/design/skillify/README1.sol56x.md) **demonstrates**
  a winner-symlink candidate once skillify adopts the topic shape.
- Beads ticket `rekon-doc-constitution-hierarchy` **decides** acceptance of
  the hierarchy line; its forward anchor cites revision 0.
