---
type: ResearchNote
title: "Recent practical work informing the broader constitution — context and practice scan 0"
description: What the 2026-09-03/04 rekon modules (code/reconstruct, code/reconstruction, vcs/jj, js, atproto) actually establish, which relationships a broader root constitution should express, and what must stay domain-owned.
resource: /design/doc-constitution/context-practice0.glm53f.md
tags: [doc-constitution, evidence, skillification, jj, navigation, naming]
status: draft
generated: { by: "model:zai-coding-plan/glm-5.3-flash", at: 2026-09-04 }
stale_after: 2026-12-04
sources:
  - id: constitution
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution (current law, draft)
  - id: naming-draft3
    resource: /constitution/naming/exploration/draft3.sol56x.md
    title: Rekon core draft 3 (draft policy, not implemented)
    author: integrator/gpt-5.6-sol-max
  - id: reconstruct
    resource: /code/reconstruct/README.md
    title: Reconstruct From The Present
  - id: reconstruction
    resource: /code/reconstruction/README.md
    title: Reconstruction with docs preservation — the jj procedure
  - id: experiments0
    resource: /code/reconstruction/experiments0.glm53h.md
    title: jj date preservation and split mechanics (measured evidence)
  - id: jj-set
    resource: /vcs/jj/README.md
    title: jj + git working guide set
  - id: js-skill
    resource: /js/README.md
    title: JavaScript / TypeScript development skill
  - id: atproto-skill
    resource: /atproto/README.md
    title: ATProto lookup skill
  - id: dolt-rs
    resource: file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md
    title: Dependency-Ordered Research Wiki Constitution (older antecedent)
  - id: is-tree-v2
    resource: file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md
    title: is-tree constitution v2 (older antecedent)
---

<a id="scope"></a>

# Scope, Window, Limits

Evidence gathering for a **broader root constitution/README.md candidate**; the parent session
owns constitution, naming, hierarchy, skillification, navigation, and assembly. This note does
not author that document.

- **Window:** work dated 2026-09-03/04 — [`code/reconstruct/`](/code/reconstruct/README.md),
  [`code/reconstruction/`](/code/reconstruction/README.md), [`vcs/jj/`](/vcs/jj/README.md),
  [`js/`](/js/README.md), [`atproto/`](/atproto/README.md) — read from local files and
  `jj log` (draft3 = `b2551121`, draft2-discussion0 = `416f2ec4` confirmed as named).
- **Antecedents (older, read only for contrast):** dolt-rs constitution and is-tree v2; the
  constitution already cites compute-fabric, which I only spot-checked (Reading Map,
  [`compute-fabric/README.md`](file:///home/rektide/archive/doc/compute-fabric/README.md) L655+).
- **Limits:** local primary documents only, no web. Naming/skillify design waves (`draft3`,
  siblings) treated as *unimplemented draft policy*; nothing below claims they are in force. I
  did not read new sibling drafts in this wave. Recent-vs-old provenance is stated per claim.

<a id="inventory"></a>

# What The Recent Work Actually Is

Four distinct postures landed in two days:

1. **Philosophy guide** — [`code/reconstruct/README.md`](/code/reconstruct/README.md):
   reconstruction modes table (L100-106), labeled claim classes (Required outcome / Current
   constraint / Design decision / Implementation hypothesis / Historical evidence, L287-294),
   stop-and-reassess triggers (L458-477). Sources cite a human directive plus four precursor
   docs with exact line links (L564-593).
2. **Executable procedure + measured evidence** —
   [`code/reconstruction/README.md`](/code/reconstruction/README.md) (snapshot-first, audit,
   unmix, replay, verify; author-date guarantee L198-216; worked watchwoman run + incident
   L218-239), [`experiments0.glm53h.md`](/code/reconstruction/experiments0.glm53h.md)
   (measured jj 0.40.0 tables L39-51; the `split -A` incident and recovery L109-157; trap log
   L159-177), and [`replay-docs.sh`](/code/reconstruction/replay-docs.sh).
3. **Promoted reference set** — [`vcs/jj/`](/vcs/jj/README.md) grew from the archive cheat
   sheet into README + `rewrites.md` + `oplog-forensics.md` + `git-only.md`, with a
   `verified:` frontmatter field (L9), a dated Provenance section (L274-281), job-organized
   "using this set" routing (L54-62), and a deliberately retained false-claim correction
   ("the famous correction", L281).
4. **First two skill extractions** — [`js/`](/js/README.md) (distilled from global AGENTS.md,
   commit `b0fc7117`; declared "promoted home … global file remains authoritative where they
   overlap", L9-11) and [`atproto/`](/atproto/README.md) (new lookup recipes, no monolith
   source; absence-is-not-proof caveats L70-74). Both are `SKILL.md -> README.md` symlinks;
   neither carries OKF frontmatter.

Older antecedents, for contrast only: is-tree v2 poses the exact convergence question this
broader constitution answers — "should this project and `dolt-rs` converge on a shared
constitution template while retaining project-specific dispatch rules?"
([`topic-document-constitution2.gpt56t.md`](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md)
L219-220). The dolt-rs constitution's **inherited / value-add / stop boundary** discipline
(L265-270, worked example L584) is the pre-existing law that the vcs/jj set then instantiates.

<a id="relationships"></a>

# Relationships A Broader Constitution Should Express

**R1. Philosophy ↔ procedure ↔ evidence ↔ tool is a reusable layer topology.**
`code/reconstruct` (why/modes) → `code/reconstruction` (how) → `experiments0` (measured proof)
→ `replay-docs.sh` (executable) is the constitution's promotion-as-synthesis and
sources-ground-claims principles realized as a *domain-internal shape* with reciprocal
cross-references on both ends (reconstruction L251-268; reconstruct L589-593). A broader
constitution can name this four-layer pattern once; each domain keeps its own layers.

**R2. Verification provenance needs an inherited-vs-rechecked distinction.**
`vcs/jj/README.md` L9 records verification as a compound claim (inherited paste-tests +
promotion-time re-spot-checks), and `oplog-forensics.md` honestly marks itself "not
independently re-run at promotion". The constitution's rule "MUST NOT claim verification merely
because its author reread it" ([`/constitution/README.md`](/constitution/README.md) L203-204)
is necessary but weaker than what practice now does. Root law should standardize the
`verified:` field's vocabulary (measured / paste-tested / inherited / re-spot-checked) — the
*methods* stay domain-owned.

**R3. Skillification passes through a declared dual-authority window.**
`js/README.md` L9-11 is the first live transition contract: content moved out of the AGENTS.md
monolith, precedence stays with the global file until acceptance, and divergence must be
reconciled rather than grown. The constitution describes the assembly target (L721-751) but has
no named interim state for "module promoted, authority not yet flipped". This window — and its
exit condition — is exactly what a root constitution owning skillification should specify.

**R4. Modules contain canonically-owned sub-documents; placement is a declared rule.**
`vcs/jj/rewrites.md` L38 states a *Placement rule*: the mainstream case lives in the README,
and rewrites.md is "the canonical home" of the committer-date aside;
[`code/reconstruction/README.md`](/code/reconstruction/README.md) respects it by pointing there
twice (L62-66, L212-216) instead of absorbing the material. This is dolt-rs's stop-boundary
law in daily use. Root law should require *a declared canonical home per shared aside*; which
home wins is a domain decision, not constitutional text.

**R5. Failure evidence has a pipeline: incident → forensics → generalizable guide → fixed tool.**
The `split -A` bookmark drag was diagnosed live, recorded exactly in `experiments0` (L109-157),
generalized into `oplog-forensics.md`, warned against in the procedure (L138-143), and fixed in
`replay-docs.sh` (plain split → abandon → rebase) with re-rehearsal. The constitution's "two
truths" (L757-761) and friction-triggers-rule-change (L809-811) describe this; a broader
constitution could name the pipeline as an expected motion, including that exact failure
records stay model-suffixed and revision-pinned while the lesson is promoted.

**R6. Exact evidence preservation has a working mechanism, not just a principle.**
Snapshot-first dated bookmarks, never-rewrite duplicate-then-rebase, and author-date
preservation (reconstruction L53-69, experiments0 L43-51) are how "exact evidence artifacts
preserve what an author argued" is actually *guaranteed* at the VCS layer. Draft3's "revision
binding" glossary entry anticipates this but is unimplemented policy; the practice itself is
real and citable today. Root law can require revision-pinned citations; the bookmark doctrine
stays vcs-owned.

<a id="domain-owned"></a>

# What Should Stay Domain-Owned

- **jj/git mechanics** — command tables, trap lists, exit-code semantics, the DSL warnings
  (`vcs/jj/`); useful law is only "traps are durable knowledge with mechanism".
- **Reconstruction mode taxonomy** — the five modes and counterfactual questions are
  code-work judgment; root law should link, not absorb.
- **Verification methods per domain** — paste-tests, scratch repos, live-repo replays,
  cross-check ladders (atproto's Constellation-vs-PDS checks, L76-84).
- **External-service facts and their staleness** — Constellation coverage numbers, AppView
  behaviors; they decay on their own schedule and belong in the atproto module.
- **Toolchain choices** — pnpm/vite-plus/tsdown versions and recipes in `js/`.
- **Placement decisions themselves** — which document canonically owns each aside.
- **All naming/core vocabulary** (draft3's glossary, channels, tombstones) — parent-owned
  draft; nothing in the recent practice depends on it being accepted.

<a id="absences"></a>

# Notable Absences And Frictions

- **Frontmatter split-brain:** `code/*` and `vcs/jj/*` carry full OKF frontmatter; `js/` and
  `atproto/` (the two newest modules) carry none. Two identity styles now coexist from the
  same week; the constitution's frontmatter SHOULD has no tooling or gate behind it.
- **Anchor practice regression:** `code/reconstruct` uses explicit `<a id>` anchors; `vcs/jj/`,
  `js/`, `atproto/` rely on heading slugs. The newest modules don't follow the constitution's
  anchor recommendation (L496-518).
- **Symlink naming drift:** `vcs/jj/SKILLS.md` (plural) vs `SKILL.md` in `js/`/`atproto/` and
  the constitution's MUST (L227-231). Unresolved which name harnesses actually honor.
- **No `GLOBAL.md` anywhere yet;** the js skill overlaps live AGENTS.md sections in both
  directions, making the declared window (R3) load-bearing right now.
- **No ticket linkage in the four practical modules** (no `extensions.ticket`), while the
  naming drafts carry them. Balanced ticketing may be fine with this — but it is untested
  against the constitution's namespace section.
- **`js/README.md` uses a `## Log` section** instead of the reserved `log.md` file (L186-189).
- **Worked examples cite `file:///home/rektide/src/watchwoman-systemd`** — a volatile local
  path carrying load-bearing evidence; no pattern exists for stable external worked-example
  citation.

<a id="recommendations"></a>

# Recommendations For The Broader Constitution Draft

1. Express R1-R6 as *contracts with local ownership*, not procedures: name the layer topology,
   the verified-field vocabulary, the dual-authority window, placement rules, the failure
   pipeline, and revision-pinned evidence — then stop.
2. Resolve the two live inconsistencies before they calcify: frontmatter-or-not for skills, and
   `SKILL.md` vs `SKILLS.md` symlink naming.
3. Treat `js/`'s promotion rule as the seed of the skillification lifecycle section; `atproto/`
   shows the no-monolith variant that needs no transition contract.
4. Keep the draft3 vocabulary out of root law until accepted; cite the *practices* (dated
   snapshots, model-suffixed evidence, accepted-tip symlinks like
   [`adoption.md`](/design/doc-constitution/adoption.md)) as the evidence layer those terms
   would name.

<a id="crossrefs"></a>

# Cross-References

- [`/constitution/README.md`](/constitution/README.md) — current law this note tests against;
  sections cited by line above.
- [`/constitution/naming/exploration/draft3.sol56x.md`](/constitution/naming/exploration/draft3.sol56x.md)
  — parent-owned draft whose glossary (revision, channel, move) *names* several practices
  documented here as already real; cited as design intent only.
- [`/code/reconstruction/README.md`](/code/reconstruction/README.md) **evidences** R1, R5, R6.
- [`/vcs/jj/README.md`](/vcs/jj/README.md) **evidences** R2, R4 and the absences (anchors,
  SKILLS.md naming).
- [`/js/README.md`](/js/README.md) **evidences** R3 and the dual-authority window.
- dolt-rs constitution (L265-270) **pre-establishes** the stop-boundary law R4 instantiates;
  is-tree v2 (L219-220) **poses** the convergence question this broader constitution answers.
