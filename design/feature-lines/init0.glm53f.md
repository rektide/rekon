---
type: Research
title: Feature lines and the global glossary — space paged in, no conclusions
description: Descriptive map of where feature-line/vcs vocabulary and glossary machinery currently live in rekon, as groundwork for placing prime/tip/feature-line terms; deliberately draws no conclusions.
resource: /design/feature-lines/init0.glm53f.md
tags: [glossary, context-weave, vcs, jj, bookmarks, prime, feature-lines]
status: draft
generated: { by: model:zai-coding-plan/glm-5.3-flash, at: 2026-09-08 }
sources:
  - { resource: /CONTEXT.md, title: Rekon working vocabulary (root glossary) }
  - { resource: /constitution/GLOBAL.md, title: Constitution shared sense-map fragment }
  - { resource: /code/GLOBAL.md, title: Code stage-and-change fragment }
  - { resource: /constitution/index.md, title: Constitutional reading map }
  - { resource: /praxis/README.md, title: Praxis home }
  - { resource: /design/rekon/context-weave/inventory0.gpt56s.md, title: Ambient guidance inventory }
  - { resource: /design/rekon/context-weave/README.md, title: Context weave workshop }
  - { resource: /code/commits.md, title: Commit discipline with jj }
  - { resource: /vcs/jj/README.md, title: jj + git working guide }
  - { resource: /vcs/jj/rewrites.md, title: jj rewrite surgery }
  - { resource: /AGENTS.md, title: Live manuscript (dated-bookmark rule) }
---

# Feature lines and the global glossary — space paged in

Commission: research rekon scoped to "feature line stuff"; the open question
is where a **global glossary** lives (praxis-level vibe, maybe
constitution-level) and what `prime` / active-tip vocabulary should be
recorded. This pass pages material in and maps what exists. **No conclusions
are drawn here** — placement decisions are a separate, human-steered step.

Triggering context (2026-09-08 session): tangled repos were rebookmarked to
dated feature names (`bookmarklet-20260214`, `bookmarklet-20260807`,
`part-of-mark-20260908`) and `main` deleted from the personal tangled repos;
tangled calls its default branch the **prime**, exposed via the
`sh.tangled.repo.setDefaultBranch` XRPC procedure (lexicon mirrored at
`~/src/tangled-cli-create/lexicons/sh/tangled/repo/defaultBranch.json`). The
knot does **not** block deleting the branch prime points at; both repos now
show "This branch is empty" until prime is re-pointed.

## 1. Glossary surfaces that exist today

- **`/CONTEXT.md`** — the root **working glossary** (OKF `type: Glossary`,
  draft, generated 2026-09-06 by gpt-6-astra, ticket `rekon-phoenix`). Four
  term groups, each term with an `<a id>` anchor:
  - *Names, homes, relations:* name, authority, reference, alias, hat, role,
    home, channel, revision, sense-map
  - *Knowledge, work, learning:* effort, intent-front, work-anchor, claim,
    evidence, provenance, case-story
  - *Action and delivery:* action, practice, capability, skill, exposure,
    ambient, global-contribution, context-pointer
  - *Honing and commitment:* honing, checkpoint, acceptance, promotion,
    activation, verification
  - **No vcs/bookmark/branch terms** in any group.
- **`/constitution/GLOBAL.md`** — order-100 `ContextFragment`, "glossary-first
  candidate contribution"; Terms-in-use table; posture **honing, not
  promoted**. States the governing pattern verbatim: *"A tiny local glossary
  belongs before instructions that rely on its terms; it need not repeat the
  entire shared glossary."*
- **`/code/GLOBAL.md`** — order-200 fragment, same pattern. Local terms:
  stage/checkpoint, correction, scope, hunk; explicitly defers
  acceptance/promotion/activation distinctions to CONTEXT.md.
- **SKILL.md symlinks** (README-body exposures): atproto, code,
  constitution, js, vcs/jj. GLOBAL.md exists only for constitution and code.
- **`/README.md`** carries the "Terms above the fold" sense-map table and
  points at CONTEXT.md as the glossary.

So a two-tier glossary mechanism (shared root glossary + tiny per-domain
glossary fragments before term-reliant instructions) is already installed and
honing. vcs is a SKILL-symlinked domain with **no GLOBAL fragment**.

## 2. Feature-line / vcs vocabulary state

- **"feature line"**: zero occurrences anywhere in rekon.
- **"prime" (tangled default-branch sense)**: absent. Existing `prime` hits
  are the `bd prime` beads prohibition (AGENTS.md and several review templates)
  and prose uses ("prime material", "naming-priming-docs").
- **Dated-bookmark rule**: lives only in the live manuscript `/AGENTS.md`
  ("name dated feature bookmarks `<feature>-<YYYYMMDD>`… recover the local
  calendar date… from `jj op log`"), together with the author-date staleness
  and git-colocation backdating notes.
- **context-weave inventory0 ledger row 22** maps AGENTS lines 222–244
  (jj/git everyday policy incl. dated bookmarks) → `code/commits.md` owns
  day-to-day policy; `vcs/jj/` owns tool forensics; warns against a competing
  jj-policy owner. Its unresolved-flags section notes *"Line 224's
  dated-bookmark rule may be workflow-specific rather than universal"* and
  asks *"Is `code/commits.md` the sole owner of day-to-day jj policy while
  `vcs/jj/` [owns deeper material]"* as an open workshop decision.
- **`/vcs/jj/` contents**: README (spelunking guide, stable, verified),
  `rewrites.md` (timestamp preservation, backdating via git colocation, split
  mechanics, bookmark forwarding), `oplog-forensics.md` (what-moved-my-bookmark
  forensics; documents the dated `systemd-20260903` snapshot bookmarks),
  `git-only.md` (`ls-remote` vs last-fetched refs). No GLOBAL, no glossary
  terms, no default-branch/prime concept anywhere.
- **`/code/commits.md`**: capture/checkpoint discipline; "bookmark" appears
  only in the compact-log template column list.

Related vocabulary in live use but unrecorded in any glossary: *tip*, *active
tip*, *feature name*, *datestamp*, *prime branch*, *dangling prime*,
*knot/tangled remote*.

## 3. praxis/ and constitution/ as they stand

- **`/praxis/`** — cross-cutting posture home (chase-future, post-build,
  readme-driven, workflow-diagrams), generated 2026-09-06. Self-describes as
  connecting existing practices, *"not a new process above every task"*;
  defers navigation/course questions to constitution's work-map0.
- **`/constitution/`** — maintained pilot account (README, draft), reading
  map (`index.md`), recorded naming ruling
  (`naming/exploration/proposal4.glm53m.md`), work-map0, reassessment0,
  agents-assembly.md, threads0; `design/doc-constitution/` holds its design
  and adoption history.
- **Reconciliation status today**: constitution/index.md indexes praxis as a
  single entry ("a 2026-09-06 draft praxis contribution"). No document
  relates the two structurally. The frame voiced in commission — *praxis as
  bill of rights to constitution's declaration of independence* — appears
  nowhere in the corpus yet.
- Positioning contrast as written: constitution holds ownership/rulings/
  promotion; praxis holds posture/action learning with explicit non-process
  disclaimers.

## 4. Where the glossary machinery was designed

`/design/rekon/context-weave/`: north-star0 (basis), inventory0 (complete
AGENTS ledger: homes, duplication, stale candidates, three coarse grouping
options A/B/C), assembly0 (manual assembled view — not live guidance),
README. Posture: manual, human-steered workshop under `rekon-agents-maintenance`;
no automatic excision; GLOBALs intentionally repeat terms while honing;
nothing promoted or activated.

## 5. Zones not paged in this pass

digest/, assess/, remake/, evidence/, doc/ (adr/plan/discovery/transcript),
forge/, agents/, prompt/, provider/, build/, upstream/, js/, atproto/,
`.beads/`. Not read here; their relation to the glossary question is unknown
from this pass.

## Where we are (session state at paging time)

- margin (personal tangled repo): `bookmarklet-20260214` and
  `bookmarklet-20260807` pushed; undated `bookmarklet` and remote `main`
  deleted; loose review-wave/synthesis commits landed; two empty
  workspace-artifact commits abandoned. Local `main` retained, tracking the
  margin.at org repo (upstream, keeps main).
- parts-of-mark: `part-of-mark-20260908` pushed; tngl `main` deleted; local
  `main` retained (github origin line).
- **Both personal tangled repos now have a dangling prime** ("This branch is
  empty" + populated-branch hint on the repo page). Re-pointing requires
  `sh.tangled.repo.setDefaultBranch` with tangled atproto auth (web repo
  settings, or the `tang` CLI auth flow in `~/src/tangled-cli-create`).
- newpub: pushed to private GitHub `rektide/newpub` (`main`); refs.md and
  part-of-mark README references committed along the way.
