---
type: Research
title: GLOBAL research — the refs module and a KISS index profile
description: Source-cited evidence on the GLOBAL.md fragment contract, existing fragment shapes, assembler constraints, and where guidance invites bloat; prepared for the parent's refs/{README,skills,GLOBAL}.md integration pass.
resource: /design/refs/global-research0.glm53h.md
tags: [global, refs, skills, assembly, context, kiss, skillify]
status: draft
generated: { by: "model:zai-coding-plan/glm-5.3#high", at: 2026-10-07T00:45:00Z }
verified: { by: none, at: never }
stale_after: 2026-11-06
extensions:
  ticket: rekon-skill-management
  posture: research-only; no refs/, constitutional, or code files edited by this change
  scope_note: read snapshot taken 2026-10-06/07 while constitutional structure material was being edited concurrently
sources:
  - { resource: /constitution/README.md, title: GLOBAL contract, honing exception, ambient cut, module shape }
  - { resource: /constitution/agents-assembly.md, title: Implemented fragment metadata, manifest, and command contract }
  - { resource: /src/agents/assemble.ts, title: Assembler implementation actually enforced today }
  - { resource: /constitution/GLOBAL.md, title: Heaviest existing fragment, honing posture }
  - { resource: /code/GLOBAL.md, title: Second fragment, glossary-first honing shape }
  - { resource: /doc/GLOBAL.md, title: Fragment retired mid-session by a concurrent actor }
  - { resource: /praxis/ticket-pointers/GLOBAL.md, title: Leanest pre-existing fragment precedent }
  - { resource: /refs/README.md, title: Parent's new references landing page }
  - { resource: /refs/skills.md, title: Parent's new skill-process support document }
  - { resource: /refs/GLOBAL.md, title: Parent's new ambient index fragment }
  - { resource: /constitution/structure.md, title: refs/ declared being established; doc/ retirement; t/ tool home }
  - { resource: /constitution/naming/README.md, title: Short codes and the glm53h suffix precedent }
  - { resource: /design/skillify/README1.sol56x.md, title: One body three exposures, default-zero GLOBAL, router risk }
  - { resource: /design/rekon/context-weave/README.md, title: Honing workshop frame and the intentionally-not-compact first assembly }
  - { resource: /CONTEXT.md, title: Shared vocabulary (global contribution, exposure, honing) }
  - { resource: /README.md, title: Root sense-map and starting points }
  - { resource: /code/README.md, title: Module landing page with Doc/Load-when routing table precedent }
  - { resource: /doc/README.md, title: Doc landing page as of the retiring-bucket era }
  - { resource: /t/README.md, title: New tool home and t-agents rebuild standing }
  - { resource: /upstream/README.md, title: Upstream reference mirrors — refs' nearest neighbor practice }
  - { resource: /src/command/project-files.ts, title: Existing ~/archive and ~/wiki lookup convention }
  - { resource: /AGENTS.md, title: Live manuscript, archive-first and well-known-directories sections }
  - { resource: ".beads/issues.jsonl", title: "rekon-skill-management epic: exposure direction and documentation-home question" }
  - { resource: "file:///home/rektide/archive/kitlangton/motel/skills/motel-debug/SKILL.md", title: Archive-backed skill entry, verified present }
  - { resource: "file:///home/rektide/archive/polars-inc/skills/polars/SKILL.md", title: Archive-backed skill entry, verified present }
---

<a id="global-research"></a>

# GLOBAL research — the refs module and a KISS index profile

<a id="global-research-situation"></a>

## Situation

The human direction for `rekon-skill-management` (2026-10-06) asks for
`refs/{README,skills,GLOBAL}.md`: README as a brief introduction to references
at large, `skills.md` explaining our skill process, and GLOBAL holding a core
knowledge index that is **chiefly actual skill links** — with an explicit
**KISS** requirement on GLOBAL. The parent authored those three files during
this research and owns their revision; this report is the requested evidence
dump. It classifies what is *current rule*, *older honing exception*,
*implementation constraint*, and *new direction*, then names concrete edit
opportunities with exact paths and lines.

The epic itself frames GLOBAL's job: "generate a compact skill/tool index for
global AGENTS.md with when-to-load cues and concrete archive citations"
(`.beads/issues.jsonl`, `rekon-skill-management`, Exposure bullet). The
refs/GLOBAL the parent wrote is exactly that shape.

**Snapshot qualification:** constitutional structure material changed
concurrently during this research. `AGENTS.md` was revised twice (doc/ bucket
retiring in Rekon; tools to `t/<tool>/`), `/constitution/structure.md` now
declares ``refs/` (being established) `` at line 40, and `doc/GLOBAL.md`
changed from `status: draft` to `status: deprecated` mid-session. Line numbers
below were accurate at read time (2026-10-07T00:37Z); re-verify before editing
anything in `constitution/` or `doc/`.

<a id="global-research-classification"></a>

## What kind of instruction each source is

| Source | Kind | What it actually says |
|---|---|---|
| [`/constitution/README.md` §GLOBAL](/constitution/README.md#doc-constitution-global) (lines 242–258) | **Current rule** | GLOBAL is "the concise fragment a module contributes to an assembled AGENTS.md… a curated operational projection, not a README summary and not a symlink"; every line pays repeated context cost; SHOULD point to its README; contributions MUST stay traceable. |
| [`/constitution/README.md` §Honing](/constitution/README.md#doc-constitution-honing) (lines 260–290) | **Human-authorized exception, dated 2026-09-06** | During the rekon-phoenix workshop: start GLOBALs with "terms in use", accept deliberate redundancy, keep SKILL symlinks intact, distinguish candidate/promoted/selected/loaded. Explicitly "not acceptance of every candidate GLOBAL"; the compact profile "should emerge from reading and using the combined context". |
| [`/constitution/README.md` §ambient cut](/constitution/README.md#doc-constitution-context-ambient) (lines 729–743) | **Current rule (test)** | A rule earns global load when its cost-of-absence exceeds its context cost: corruption, cheapness, universality. |
| [`/constitution/agents-assembly.md`](/constitution/agents-assembly.md) + [`/src/agents/assemble.ts`](/src/agents/assemble.ts) | **Implementation constraint** | What the tool actually enforces: metadata, one-H1 shape, link-destination rules, manifest, provenance, byte costs. Detailed below. |
| [`/design/skillify/README1.sol56x.md`](/design/skillify/README1.sol56x.md) (lines 417–465) | **Accepted design direction** | "One body, three exposures"; GLOBAL assembles into a parent AGENTS via manifest; **new modules default to no GLOBAL** — ambient cost must be demonstrated. |
| KISS direction for refs (2026-10-06, human, via parent) | **Newest user direction** | GLOBAL must stay simple for refs. This is the compact-profile posture the honing section anticipated, applied from birth rather than honed down. It does not revoke the honing exception for the existing phoenix fragments — those remain human-steered per [`context-weave`](/design/rekon/context-weave/README.md) — but it governs new refs work. |

The important reading: **the honing workshop's deliberate redundancy was an
exception with a stated exit** ("The normal compact profile repeats what must
stand alone before skill discovery", constitution line 253–254). The KISS
direction for refs is not a contradiction of the constitution — it is the
normal profile being applied. The parent should not import honing-era shapes
(terms tables, repetition disclaimers) into refs/GLOBAL.

<a id="global-research-inventory"></a>

## Existing fragments, measured

Prose bytes measured with the implemented parser (`parseFragment`,
[`assemble.ts:348`](/src/agents/assemble.ts)) on 2026-10-07:

| Fragment | id / order | prose bytes | Shape |
|---|---|---|---|
| [`/constitution/GLOBAL.md`](/constitution/GLOBAL.md) | doc-constitution / 100 | **4148** | Honing artifact: 7-row terms table, glossary-first, repetition rationale, closing disclaimers (79 lines). |
| [`/code/GLOBAL.md`](/code/GLOBAL.md) | code / 200 | **1675** | Terms list + 5 dense operating bullets, each ending in a `/code/...` depth pointer. |
| [`/doc/GLOBAL.md`](/doc/GLOBAL.md) | doc / 300 | — | **Retired mid-session** by a concurrent actor: `status: deprecated`, body now a redirect to the structure contract. No longer an assembly candidate. |
| [`/praxis/ticket-pointers/GLOBAL.md`](/praxis/ticket-pointers/GLOBAL.md) | praxis-ticket-pointers / 400 | **614** | Leanest pre-existing precedent: plain cue bullets + one "Deeper practice" link. No terms section, no body banner. |
| [`/refs/GLOBAL.md`](/refs/GLOBAL.md) | refs / 500 | **381** | Parent's new fragment: one instruction line + three cue links. Passes assembler validation (verified). |

The ladder itself is evidence: fragment weight has fallen steadily
(4148 → 1675 → 614 → 381) as the corpus moved from honing toward compact
profile. `praxis/ticket-pointers` (2026-09-23) already shows the KISS shape
the human is now asking to make explicit; refs/GLOBAL continues it.

<a id="global-research-contract"></a>

## Assembler constraints that shape refs (verified against the implementation)

From [`/src/agents/assemble.ts`](/src/agents/assemble.ts) and
[`/constitution/agents-assembly.md`](/constitution/agents-assembly.md):

1. **Metadata is exactly four enforced fields** — `id` (kebab-case, unique),
   `order` (unique integer), `source` (bundle-root-relative canonical README),
   `status` (`draft` \| `stable`) —
   [`fragmentMeta`, assemble.ts:317–336](/src/agents/assemble.ts). All other
   frontmatter (type, title, description, extensions, sources) is ignored by
   the tool. refs/GLOBAL.md carries only the four: minimal metadata is
   contract-compliant KISS. The richer OKF identity lives in refs/README.md
   and refs/skills.md, which is the right division.
2. **Exactly one top-level heading, in leading position, with content beyond
   it** — [`validateProse`, assemble.ts:271–305](/src/agents/assemble.ts);
   agents-assembly.md lines 256–259. refs/GLOBAL.md satisfies this (`#
   References`).
3. **Link destinations must be location-independent**
   ([assemble.ts:239–243](/src/agents/assemble.ts); agents-assembly.md lines
   107–123): anchors, `/bundle-root` paths, and protocol URIs (`https:`,
   `mailto:`, `file:`). Anything else — including a `~/archive/...` tilde
   path in link position — is rejected as fragment-relative. The refs files
   already split this correctly: `file:///home/rektide/archive/...` links
   ([refs/skills.md:82,92](/refs/skills.md); [refs/GLOBAL.md:12–13](/refs/GLOBAL.md))
   with tilde paths kept in prose
   ([refs/README.md:20](/refs/README.md); [refs/skills.md:34](/refs/skills.md)).
   **Keep that split; a tilde link will hard-fail assembly.**
4. **Frontmatter is stripped and the provenance comment omits `status`** —
   [`renderDocument`, assemble.ts:454–460](/src/agents/assemble.ts) emits
   `id/order/path/source` only. So in assembled output, `status: draft` is
   invisible; the only assembly-visible posture signal is body text. The
   honing-era "Honing candidate" body banners exist partly for this reason.
   Consequence for KISS: if a posture must survive assembly, it must be one
   body line — or the assembler should grow `status` in its provenance
   comment (a small change worth considering for the `t-agents` rebuild,
   which is being designed concurrently per
   [`/t/README.md:29`](/t/README.md)).
5. **No auto-discovery; manifest-driven** (agents-assembly.md lines 128–131;
   [assemble.ts:14](/src/agents/assemble.ts) — no production manifest
   exists). A GLOBAL file's existence asserts nothing about ambient load —
   matching the glossary's "global contribution… is not proof it is loaded"
   ([CONTEXT.md:147–150](/CONTEXT.md)).
6. **Context cost is real UTF-8 bytes, reported per fragment**
   (agents-assembly.md lines 216–223). The 381-byte refs fragment is the
   corpus's cheapest; the constitution fragment costs ~10.9× more per
   assembly.

**Verified contract check (read-only probe, 2026-10-07):**
`parseFragment('refs/GLOBAL.md')` → VALID, `{id: refs, order: 500, source:
/refs/README.md, status: draft}`, 381 prose bytes. Order 500 is the next gap
after 100/200/300/400 — consistent with the "leave gaps" guidance
(agents-assembly.md line 100). `source: /refs/README.md` correctly names the
module landing page per the metadata table (agents-assembly.md line 101), even
though the index's rationale lives in skills.md — README is the canonical
module body; skills.md is a support document. One precedent deviates:
`praxis/ticket-pointers/GLOBAL.md` points `source` at a design file
(`/praxis/ticket-pointers/ticket-pointers0.glm53fx.md`) rather than a module
README — a small contract drift worth normalizing whenever that module gains
a README.

**Found during the same probe:** `doc/GLOBAL.md` now **fails**
`fragmentMeta` — its retired `status: deprecated` is outside `draft|stable`
(assemble.ts:330–333). Since assembly is manifest-driven with no
auto-discovery, the failure only fires if someone declares it — which acts as
a tripwire against accidental re-inclusion, plausibly intentional. The parent
should pick one story: either bless "fails validation = retired" (say so in
the retirement note), or extend the status vocabulary in the `t-agents`
rebuild. No edit is required today.

<a id="global-research-links"></a>

## Link semantics for archive skill citations

- **`file://` is the assembler-legal form for archive citations.** Protocol
  URIs are location-independent
  ([assemble.ts:242](/src/agents/assemble.ts)); the constitution permits
  absolute `file://` links for explicitly local material
  ([constitution/README.md:643–646](/constitution/README.md)). An ambient
  index whose whole job is "reach the local checkout now" is exactly the
  explicitly-local case.
- **Machine-locality is the known trade-off.** Skillify open question 6 asks
  how capabilities cite knowledge "that may not exist at the same filesystem
  path on another machine"
  ([README1.sol56x.md:770–772](/design/skillify/README1.sol56x.md)). The
  refs files already mitigate it the right way: canonical upstream URLs and
  revisions live in `refs/skills.md` frontmatter and per-skill "Upstream:"
  lines ([skills.md:84,94](/refs/skills.md)), while GLOBAL carries only the
  local `file://` links; skills.md:52–54 states the re-resolution rule for
  another machine. Keep that division: **provenance in skills.md, reachability
  in GLOBAL.**
- **Both archive entries are real.** Verified present:
  `~/archive/kitlangton/motel/skills/motel-debug/SKILL.md` (with
  `references/`, `scripts/`) and `~/archive/polars-inc/skills/polars/SKILL.md`
  (with `references/`). The epic recorded them as absent at its inspection on
  2026-10-06; they have since been fetched — the intended citation targets now
  resolve.
- **The lookup convention already exists in tooling.**
  [`src/command/project-files.ts:68–108`](/src/command/project-files.ts)
  walks `~/archive` and `~/wiki` — the same roots refs/README documents. When
  the skill-manager tool is designed, `project-files` is the in-repo precedent
  for archive resolution (the epic already cites it).

<a id="global-research-relationship"></a>

## README / skills.md / GLOBAL in refs — the division holds

Assessed against the constitution's module shape
([constitution/README.md:177–189](/constitution/README.md)) and the skillify
host profile ([README1.sol56x.md:387–415](/design/skillify/README1.sol56x.md)):

- **`refs/README.md` is the required canonical landing page** — brief intro
  to references at large. It correctly routes to skills.md, GLOBAL, and the
  nearest neighbor practice `upstream/` (mirrors are about *code baselines*;
  refs are about *selected resources* — the cross-link at
  [refs/README.md:26](/refs/README.md) is the right relationship edge).
- **`skills.md` is a support document** explaining the process — the same
  pattern as `/code/`'s support docs with their "Doc | Load when" routing
  table ([/code/README.md:43–52](/code/README.md)). Its §"Keep GLOBAL simple"
  ([skills.md:70–76](/refs/skills.md)) is the KISS rule written down: "A
  useful cue plus a direct link is the default… Add a stand-alone instruction
  only when it must act before the linked material is loaded." That sentence
  is the cleanest expression of the ambient-cut corruption test in the corpus;
  it deserves to survive revision.
- **Lowercase `skills.md` is deliberately not `SKILL.md`.** A `SKILL.md`
  exposure must be a symlink to the *canonical README*
  ([constitution/README.md:226–240](/constitution/README.md)) — it would
  expose the references landing page, not the process doc. The parent chose
  the right filename for a process explanation. If on-demand exposure of the
  curation practice is ever wanted, add `refs/SKILL.md -> README.md` (five
  in-repo precedents: atproto, code, constitution, js, vcs/jj) — do not
  rename skills.md.
- **GLOBAL is a router, and routers have a named failure mode.** Skillify's
  router-skill risk: "becoming a second catalog that repeats every child
  description" ([README1.sol56x.md:306–313](/design/skillify/README1.sol56x.md)).
  The current three-line index is the counterexample. The main KISS risk as
  the catalog grows is cue-line inflation; the epic's open question "What
  belongs in the global index versus domain-grouped, on-demand indexes"
  stands, and refs/skills.md's publish-step guidance (one short task cue per
  entry) is the right guard.

<a id="global-research-bloat"></a>

## Where current docs invite GLOBAL bloat or blur index vs body

Concrete spots, with the distinction they blur:

1. **The honing "terms in use" opening** —
   [constitution/README.md:273–277](/constitution/README.md) ("Start a GLOBAL
   with the small terms in use…"). Read alone, this invites every new
   fragment to open with a glossary. For an *index* fragment the corruption
   test fails: no missing term damages work before a skill loads, because the
   cue lines are self-describing. `praxis/ticket-pointers` and `refs/GLOBAL`
   both skip terms and are better for it. *Opportunity:* when the honing
   section is next revised (concurrent actor permitting), add one sentence
   scoping terms-in-use to fragments whose *instructions* lean on contested
   vocabulary — not to routing indexes. A pointer to `/CONTEXT.md` suffices
   where a term needs defining.
2. **Posture stated twice** — the honing-era fragments carry `posture:` in
   frontmatter extensions *and* a body banner ("Honing candidate, not a
   promoted GLOBAL", [constitution/GLOBAL.md:24](/constitution/GLOBAL.md),
   [code/GLOBAL.md:23–24](/code/GLOBAL.md)). Since frontmatter is stripped at
   assembly and provenance omits status (above), the body line is the only
   assembly-visible marker — but a paragraph is not needed. *Opportunity:*
   standardize on **one short body line** where a posture must survive
   assembly; keep the rest in frontmatter. refs/GLOBAL currently has no
   posture line at all, which is acceptable *while no production manifest
   exists* (nothing assembles it); revisit at cutover.
3. **`doc/GLOBAL.md`'s old self-description** (superseded within this
   session): it "deliberately repeats the ruling already carried by the
   constitution fragment" — redundancy the honing exception authorized. The
   concurrent actor has already retired it; the lesson to carry into refs is
   the general one: **an index fragment that restates another fragment's rule
   pays double context cost for one behavior.** Point, don't restate.
4. **The constitution fragment's closing navigation block**
   ([constitution/GLOBAL.md:76–79](/constitution/GLOBAL.md)) — "load the
   constitution body… load its reading map… does not replace the live AGENTS
   manuscript." Three sentences of workshop meta-commentary in ambient space.
   KISS profile: at most one "depth:" pointer line (the `code/GLOBAL.md:43`
   pattern: `module entry point: /code/README.md`). This is the clearest
   single place the existing corpus invites trimming when the human starts
   the compact-profile pass.
5. **Index vs body confusion risk in skills.md itself:** §"Initial sources"
   ([skills.md:78–98](/refs/skills.md)) correctly holds the per-skill
   prose (support bases, upstream preference caveats). The guard to keep:
   anything longer than a cue line belongs here, never in GLOBAL. The
   publish step ([skills.md:43–47](/refs/skills.md)) already says this;
   future entries just need to obey it.
6. **AGENTS.md overlap:** the live manuscript's "well known directories"
   section (archive-first, `~/wiki`, `dl`, `/llms.txt`; AGENTS.md lines
   ~107–117) covers the same landscape refs/README now introduces. Under the
   migration policy ([agents-assembly.md:58–74](/constitution/agents-assembly.md))
   the monolith stays hand-maintained until every section has a fragment, so
   the KISS move is *not* deletion but a one-line pointer from that section to
   `/refs/README.md` when the parent next revises AGENTS.md — the section
   then has a named fragment destination for the eventual cutover.
7. **Volatile-status tension:** metadata guidance says ambient context
   "should not broadcast unsettled design" (agents-assembly.md line 102), and
   archive-backed entries are the most volatile things in the system (refresh
   moves paths; the epic's update-behavior question). The refs answer is
   right: GLOBAL indexes *reachability* (file:// to the current checkout)
   while skills.md keeps findings revision-bound ([skills.md:100–106](/refs/skills.md)).
   Worth one added sentence in skills.md eventually: the *index* changes when
   the checkout moves; *conclusions* about a skill stay revision-bound.

<a id="global-research-recommendations"></a>

## Concrete recommendations for the parent's integration pass

Ordered by leverage; all are parent-owned edits, none performed here:

- **R1 — Ratify the refs shape as the KISS template.** refs/GLOBAL.md as
  written (4-field frontmatter, one H1, one stand-alone instruction, cue-line
  index, process pointer) is the compact profile the constitution anticipated.
  When revising skills.md or the constitution's GLOBAL section, name it as
  the pattern for new index-class fragments.
- **R2 — Make posture assembly-visible cheaply.** Either add `status` to the
  provenance comment in the `t-agents` Effect rebuild (small, aligns with
  [`/t/README.md:29`](/t/README.md)), or adopt the one-body-line convention
  for fragments that need it. Don't do both per fragment.
- **R3 — Decide the retired-fragment story.** `doc/GLOBAL.md` now fails
  `fragmentMeta` on `status: deprecated`. Bless it as a tripwire (note it in
  the retirement note) or extend the vocabulary in the rebuild. Also normalize
  `praxis/ticket-pointers`'s `source` to a module README when one exists.
- **R4 — Scope the honing terms-in-use guidance** to instruction-bearing
  fragments (one sentence, constitution §honing or §GLOBAL, coordinate with
  the concurrent structural edits).
- **R5 — Trim the constitution fragment's closing block** (its lines 59–79:
  repetition rationale + navigation + workshop disclaimer) toward the
  `depth:` one-liner pattern when the human begins the compact-profile pass;
  it is 40% of the heaviest fragment's weight and mostly workshop
  meta-commentary. Human decision, not ours — the honing exception is still
  standing policy for the phoenix fragments.
- **R6 — Add the AGENTS.md pointer** from "well known directories" to
  `/refs/README.md` on the next AGENTS revision.
- **R7 — Record curation scope in skills.md:** which skill classes earn
  ambient cues (currently: archive-backed selections), which are reachable
  via their own domains (the five in-repo SKILL modules), and which stay
  harness-discovered (`~/.agents/skills/`, 42 entries incl. the matt-pocock
  suite). The epic's "we decide which capabilities agents see" needs that
  boundary written down before the catalog grows.
- **R8 — Keep the tilde/file:// split and the provenance/reachability
  division** as documented above; they are load-bearing for assembly
  validity and portability respectively.

<a id="global-research-uncertainties"></a>

## Uncertainties

- **Concurrency:** `constitution/structure.md`, `AGENTS.md`, and
  `doc/GLOBAL.md` all changed during this research; another actor continues
  in that area. Every constitutional line citation above carries read-time
  risk. Re-read before editing.
- **Whether KISS becomes constitutional policy or stays module practice:**
  the honing section reserved compact-profile emergence for human steering;
  recording it as general rule is a human decision this report only
  recommends toward.
- **`t-agents` rebuild surface** (provenance status field, status vocabulary)
  is being designed concurrently; my tooling suggestions may collide with
  that design's own choices.
- **The `.agents/skills` relationship** (native harness discovery vs our
  catalog as "single entry point") is unresolved in the epic; R7 is a
  placeholder boundary, not a settled one.
- **Not read:** sibling same-wave docs (none existed in `design/refs/` at
  start — the directory was created for this report), and the context-weave
  sub-artifacts (north-star, inventory, assembly) beyond their README
  summary.
- Verification I ran and trust: assembler `parseFragment` probes over all
  five fragments (results above), archive-path existence checks, symlink
  inventory, byte measurements. Verification I did not run: full `rekon.ts
  agents` assembly (no manifest exists; would be a no-op) and link-target
  resolution inside the archive skills' own bodies.

<a id="global-research-references"></a>

## References

- [constitution README — GLOBAL contract](/constitution/README.md#doc-constitution-global),
  [honing exception](/constitution/README.md#doc-constitution-honing),
  [ambient cut](/constitution/README.md#doc-constitution-context-ambient),
  [module shape](/constitution/README.md#doc-constitution-module), [links
  policy](/constitution/README.md#doc-constitution-references-links) — the
  normative layer.
- [agents assembly reference](/constitution/agents-assembly.md#agents-assembly-metadata)
  and [assemble.ts](/src/agents/assemble.ts) — the enforced layer.
- [structure contract](/constitution/structure.md#homes) — refs/ declared
  line 40; doc/ retirement lines 67–70; tool shape lines 75–100.
- [skillify revision 1](/design/skillify/README1.sol56x.md#skillify-host-global)
  — default-zero GLOBAL, router risk, open questions 6 and 9.
- [context weave](/design/rekon/context-weave/README.md) — the honing
  workshop frame; its "intentionally not compact" 42,267-byte first assembly
  (line 52–54) is the contrast case for KISS.
- [refs README](/refs/README.md), [refs skills](/refs/skills.md),
  [refs GLOBAL](/refs/GLOBAL.md) — the parent's new module, assessed above.
- [praxis/ticket-pointers GLOBAL](/praxis/ticket-pointers/GLOBAL.md) — the
  lean precedent.
- [rekon-skill-management epic](.beads/issues.jsonl) — exposure direction,
  worked examples, documentation-home question this module answers.
- [upstream mirrors](/upstream/README.md) and
  [project-files.ts](/src/command/project-files.ts) — neighboring practices
  and the existing archive-lookup convention.
