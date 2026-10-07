---
type: Research
title: Prior structure, tools, and paths decisions bearing on the AGENTS.md cutover
description: Fill-in on settled rulings, the AGENTS assembly story, open/deferred questions, tensions, and a reading order — assembled from repo documents and beads for the context-compilation cutover session.
resource: /design/context-compilation/research0.glm53fx.md
tags: [rekon, agents, assembly, glossary, structure, tools, hyalo, research]
status: draft
generated: { by: "model:glm-5.3-flash-max", at: 2026-10-07 }
sources:
  - { resource: /constitution/agents-assembly.md, title: Assembler contract (legacy reference doc) }
  - { resource: /tool/assembly/README.md, title: Effect assembler domain guide }
  - { resource: /tool/README.md, title: Tool toolkit map }
  - { resource: /constitution/structure.md, title: Placement contract }
  - { resource: /constitution/naming/README.md, title: Short codes in use }
  - { resource: /design/rekon/context-weave/README.md, title: Context weave workshop }
  - { resource: /design/rekon/context-weave/north-star0.gpt6a.md, title: Glossary-first basis }
  - { resource: /design/rekon/context-weave/inventory0.gpt56s.md, title: AGENTS line inventory }
  - { resource: /design/rekon/context-weave/assembly0.gpt56s.md, title: Manual combined view }
  - { resource: /design/feature-lines/init1.gpt.md, title: Glossary requirement direction }
  - { resource: /design/refs/global-research0.glm53h.md, title: Fragment survey and KISS direction }
  - { resource: /design/rekon/agents-modularization0.sol56x.md, title: Two-transformations migration design }
  - { resource: /design/rekon/hyalo-toc0.glm53m.md, title: Hyalo ToC lineage }
  - { resource: /design/rekon/shortcodes0.glm53m.md, title: Shortcode lineage }
  - { resource: /design/rekon/doc-cull0.glm53m.md, title: doc/ cull record }
  - { resource: /.transcript/PLAN-combine.md, title: combine command plan }
  - { resource: "bd:rekon-agents-maintenance", title: AGENTS maintenance epic (beads) }
---

# Prior structure / tools / paths decisions — research fill-in

Commissioned by the parent session executing the AGENTS.md cutover (move the
monolith to `AGENTS.old.md`, assemble a live `AGENTS.md` from `GLOBAL.md`
fragments plus the working glossary, and revive `node tool/agents.ts`), with
hyalo adoption an open question. Everything below is recorded intent with
citations; nothing here is a new decision.

Current live state verified 2026-10-07: `AGENTS.md` is 371 lines, SHA-256
`c474204a…` (grown past the 368-line `4bf6260e…` snapshot the inventory
accounts for — a `## doc-pass` section and other edits landed since), and
`constitution/agents.manifest.json` still does not exist.

## 1. Settled rulings

- **Shortcodes `g`/`p`/`t` are reference vocabulary, not directories.** Human
  correction 2026-10-06: "t is a prefix not a dir", "g is the global
  shortcode, p praxis" ([`/design/rekon/shortcodes0.glm53m.md`](/design/rekon/shortcodes0.glm53m.md) lines 47–70,
  quoting the ruling session). The maintained table is
  [`/constitution/naming/README.md`](/constitution/naming/README.md) lines 15–35 ("Shortcodes are
  reference vocabulary, not directory names", line 31); `con`/`exp`/`res`
  scoped aliases and the earlier human naming ruling
  (location-is-identity, deliberate rename cost, hats, cross-modal aliases)
  are recorded at
  [`/constitution/naming/exploration/proposal4.glm53m.md`](/constitution/naming/exploration/proposal4.glm53m.md) line 77+
  ("The Override Stated As Ruling"), with the glossary-alias restoration
  quoted at [`shortcodes0`](/design/rekon/shortcodes0.glm53m.md) lines 97–104.
  There is no `g0`/`t0` usage anywhere; `p0` ("what we are") is a proposed
  praxis opening account only ([`shortcodes0`](/design/rekon/shortcodes0.glm53m.md) lines 142–156).
- **One shared `tool/` toolkit, domain-grouped, no `src/`/`lib/` wrapper.**
  [`/constitution/structure.md`](/constitution/structure.md) lines 74–104: entrypoints at the top of
  `tool/`, responsibilities as named subdirectories (`cmd/`, `assembly/`,
  `md/`, `hyalo/` when adopted), tests beside behavior, direct TS execution
  in development. Same content operationalized in
  [`/tool/README.md`](/tool/README.md) lines 16–37 and 62–73. The overall placement
  contract is [`structure.md`](/constitution/structure.md) itself (status: draft, generated
  2026-10-06 from the human ruling session, sources at lines 9–15).
- **`doc/` bucket retired in Rekon.** "New Rekon explanations no longer
  default to `doc/<topic>/`" — [`structure.md`](/constitution/structure.md) lines 69–73. The first
  cull (31→5 files, all dl-owned removals byte-verified) is recorded in
  [`/design/rekon/doc-cull0.glm53m.md`](/design/rekon/doc-cull0.glm53m.md) lines 19–114; its two rekon-owned
  survivors later moved (`doc/interpolate.md` → `tool/cmd/interpolate.md`,
  `doc/plan-parallelize.md` → `tool/cmd/install-commands.md`, lines 94–108),
  and `doc/opencode-session.md` was deleted with pinned citations
  (lines 109–114; execution in
  [`/design/rekon/ado-recovery1.glm53m.md`](/design/rekon/ado-recovery1.glm53m.md)). `doc/GLOBAL.md` is a
  deprecated redirect, "not an assembly input" (frontmatter lines 1–20 and
  body of `/doc/GLOBAL.md`). Recent commit `uyrokrwwqpyz` "Expose the
  runnable toolkit and finish retiring the documentation bucket" completed
  this.
- **Assembler contract (binding on both implementations).**
  [`/constitution/agents-assembly.md`](/constitution/agents-assembly.md): four enforced fragment
  metadata fields — `id` (kebab-case, unique), `order` (unique integer,
  leave gaps), `source` (bundle-root-relative canonical README), `status`
  (`draft|stable`) — lines 87–115; KISS body guidance lines 117–131; link
  destinations must be location-independent (anchors, `/bundle-root`,
  protocol URIs) lines 133–149; JSON manifest, manifest-relative paths,
  duplicate rejection lines 151–179; command flags lines 181–213; generated
  header + per-fragment provenance comments lines 215–239; UTF-8 byte-cost
  reporting lines 241–249; provenance-injection safety lines 251–259;
  no-pruning generated-file policy lines 261–288.
- **New Effect CLI is the going-forward implementation.**
  [`/tool/assembly/README.md`](/tool/assembly/README.md): runnable at
  [`/tool/agents.ts`](/tool/agents.ts) (lines 30–33), invocation
  `node tool/agents.ts -m <manifest> [--check] [--output]` (lines 43–63),
  independent of the legacy Gunshi path `node rekon.ts agents` (lines 65–67),
  deliberate header difference (`<!-- GENERATED FILE … node tool/agents.ts`)
  so cross-tool outputs are not byte-identical (lines 106–112), implementation
  map `tool/{agents.ts,cmd/,assembly/,md/}` (lines 115–152), Effect 4.0.1 +
  integrated `effect/cli` verified upstream (lines 154–166, evidence in
  [`/design/rekon/agents-effect0.sol61xh.md`](/design/rekon/agents-effect0.sol61xh.md) lines 53–84).
  Verification commands: `tool/assembly/README.md` lines 168–176 (`pnpm exec
  vitest run tool`, `tsgo --noEmit`, `oxlint tool`, `oxfmt --check`,
  effect-language-service diagnostics). Legacy contract doc still describes
  `src/agents/` paths at [`agents-assembly.md`](/constitution/agents-assembly.md) lines 209–213 — the
  new tool docs supersede those pointers.
- **The `t`-directory episode is closed.** The provisional `t/README.md`
  placement was corrected the same evening it propagated
  ([`hyalo-toc0`](/design/rekon/hyalo-toc0.glm53m.md) correction note lines 166–174;
  [`structure.md`](/constitution/structure.md) lines 113–114 "The initial draft mistakenly turned `t`
  into a directory").

## 2. The AGENTS.md assembly story

- **Intent:** "AGENTS.md is becoming deterministic build output. Knowledge
  modules own small ambient fragments in their local `GLOBAL.md`; an explicit
  manifest declares which fragments assemble…; contributors edit their local
  fragment, not a monolith" ([`agents-assembly.md`](/constitution/agents-assembly.md) lines 47–51).
  The governing sequence from the migration design:
  "**Decompose ownership first. Change context behavior second.**" — two
  independent transformations (ownership decomposition, then evidence-gated
  activation tiering), most guidance staying ambient at first cutover
  ([`/design/rekon/agents-modularization0.sol56x.md`](/design/rekon/agents-modularization0.sol56x.md) lines 69–118,
  483–496).
- **Production assembly deliberately deferred:** "No production manifest
  exists… A production manifest that would overwrite the monolith must not be
  created until every section it contains has a declared `GLOBAL.md` source
  fragment; otherwise assembly would silently discard unpromoted operating
  rules" ([`agents-assembly.md`](/constitution/agents-assembly.md) lines 65–83). The cutover pattern is
  fixed: monolith → `AGENTS.old.md`, manifest + generated output land in one
  change ([`agents-modularization0.sol56x.md`](/design/rekon/agents-modularization0.sol56x.md) lines 318–328 "Cut
  Over Ownership In One Reviewed Change"; avoid prolonged dual authority,
  failure-mode table line 478). The human's current direction authorizes this
  step; the guard that remains binding is *every assembled section needs a
  fragment source*.
- **The context-weave workshop (2026-09-06)** hand-assembled a reader view:
  root README purpose, then glossary-bearing constitution + code GLOBALs,
  then the complete retained AGENTS manuscript — 42,267 bytes "intentionally
  not compact" ([`/design/rekon/context-weave/README.md`](/design/rekon/context-weave/README.md) lines 43–57;
  receipt in [`assembly0.gpt56s.md`](/design/rekon/context-weave/assembly0.gpt56s.md) lines 39–67). It mapped
  all 368 AGENTS lines into three disjoint extraction lanes under the
  existing epic plus four children ([`context-weave/README.md`](/design/rekon/context-weave/README.md)
  lines 59–84; beads `rekon-agents-maintenance` notes confirm lane state:
  assembler ✓ closed, skillify ◐, engineering ○, workspace ○).
- **What "glossary-first" means:** a GLOBAL leads with a small "Terms in
  use" section before the instructions that rely on those terms, linking the
  shared glossary instead of copying it. Established as a **required element
  of `GLOBAL.md`** by human direction recorded in
  [`/design/feature-lines/init1.gpt.md`](/design/feature-lines/init1.gpt.md) lines 50–56 and 63–81
  ("The glossary belongs before the instructions that use it… A link saying
  'see glossary elsewhere' is not the whole local element"; "The upcoming
  core pass must incorporate this required element into the constitution's
  GLOBAL contract"). Earlier, as workshop posture:
  [`north-star0.gpt6a.md`](/design/rekon/context-weave/north-star0.gpt6a.md) lines 75–81 ("'Always
  glossary-first' is a workshop posture to try, not an already validated
  universal requirement"), and in-fragment:
  [`/constitution/GLOBAL.md`](/constitution/GLOBAL.md) lines 27–41 (terms table) and 60–64
  ("A tiny local glossary belongs before instructions that rely on its
  terms; it need not repeat the entire shared glossary");
  [`/code/GLOBAL.md`](/code/GLOBAL.md) lines 26–35.
- **The shared glossary is [`/CONTEXT.md`](/CONTEXT.md)** (frontmatter
  `type: Glossary`, lines 1–20; 202 lines of cross-topic terms). Its role is
  "a cross-topic reader aid, not a successor naming constitution or an
  identifier registry" ([`north-star0`](/design/rekon/context-weave/north-star0.gpt6a.md) lines 75–81;
  [`CONTEXT.md`](/CONTEXT.md) lines 24–26).
- **Compiling the glossary in with the fragments was left open — and one
  formulation was explicitly *not* established.** [`init1.gpt.md`](/design/feature-lines/init1.gpt.md)
  line 52: the required-glossary direction does **not** establish "Every
  existing fragment is accepted, **or all of `CONTEXT.md` must be injected
  everywhere**." Nothing in the repo declares `CONTEXT.md` an assembly input:
  it carries no fragment metadata (`id`/`order`/`source`/`status`), and the
  manual combined view deliberately used the two GLOBALs' local glossaries
  rather than injecting `CONTEXT.md` ([`assembly0.gpt56s.md`](/design/rekon/context-weave/assembly0.gpt56s.md)
  lines 47–53; repeated-`Terms in use` heading collision documented lines
  55–61). Assembler constraints to note if it is promoted into the manifest:
  the provenance comment omits `status` and frontmatter is stripped, so any
  posture signal must be body text ([`global-research0.glm53h.md`](/design/refs/global-research0.glm53h.md)
  lines 148–157); `CONTEXT.md` has one H1 (# Rekon working vocabulary, line
  22) so it would satisfy the one-H1 shape check.
- **"GLOSSARY.md" as a filename has no prior record in the repo** — `rg -i
  GLOSSARY` matches only "glossary" prose and `type: Glossary` frontmatter on
  `CONTEXT.md`. The human's name for it in the current session is new
  direction, not a previously settled filename. Prior human practice is
  short glossaries "usually within the readme" (naming-wave session seq 189,
  quoted [`shortcodes0`](/design/rekon/shortcodes0.glm53m.md) lines 78–85).

## 3. Open / deferred questions

- **Hyalo integration (unresolved, concrete candidate).** Hyalo is a Rust
  CLI (installed 0.24.1) for deterministic Markdown/OKF work:
  `okf index`, `madr toc`, `find --fields sections,lines` (live heading→line
  map), `read --section/--lines` — surface table at
  [`/design/rekon/hyalo-toc0.glm53m.md`](/design/rekon/hyalo-toc0.glm53m.md) lines 120–135. A working
  consumer exists outside the repo: `~/ado/bin/hyalo-toc.ts` (121 lines,
  line-numbered hidden ToCs with fixed-point regeneration and `--check`
  drift gate; implementation lines 95–118, provenance lines 45–93).
  Four adoption options, "Options, not decisions": adopt into `tool/`
  (`tool/hyalo-toc.ts`, new external-binary dependency kind for a node-only
  stack), point it at rekon's own corpus first, generalize only with a second
  consumer, or upstream a `hyalo toc` subcommand ([`hyalo-toc0`](/design/rekon/hyalo-toc0.glm53m.md)
  lines 177–194). [`structure.md`](/constitution/structure.md) lines 86–92 pre-blesses a `tool/hyalo/`
  domain seam "when adopted"; [`tool/README.md`](/tool/README.md) lines 53–60 records the
  candidate. Boundary contract with the summarizer (hyalo = deterministic
  lifecycle, summarizer = semantic production) holds ([`hyalo-toc0`](/design/rekon/hyalo-toc0.glm53m.md)
  lines 146–149; boundary doc `~/src/summarizer/doc/discovery/hyalo-juncture.md`).
  Prior capability map [`/assess/rekon/tooling0.glm53.md`](/assess/rekon/tooling0.glm53.md) is stale on
  install status and predates the ToC lineage ([`hyalo-toc0`](/design/rekon/hyalo-toc0.glm53m.md)
  lines 139–152).
- **Remaining legacy `rekon.ts` commands.** `combine`, `project-files`, `ln`
  have no new-home guides yet — "their source remains in
  [`src/command/`](/src/command/) pending deliberate adoption here"
  ([`tool/README.md`](/tool/README.md) lines 34–37). `interpolate` and
  `install-commands` have guides under `tool/cmd/` that explicitly document
  the legacy Gunshi source ([`tool/cmd/interpolate.md`](/tool/cmd/interpolate.md) lines 1–20;
  [`tool/cmd/install-commands.md`](/tool/cmd/install-commands.md) lines 20–31). Beads still carries:
  `rekon-combine` epic + `rekon-combine-glob` (P2, open), `rekon-interp-exec`
  ("add interpolation-execution mode to interpolate", P2 open), plus the two
  un-run extraction lanes `rekon-agents-maintenance-engineering` and
  `-workspace` (bd list 2026-10-07). Note the tension: `combine` merges
  `prompt/*.md` into `COMBINED.md` ([`/.transcript/PLAN-combine.md`](/.transcript/PLAN-combine.md)
  lines 1–56), but `prompt/` now contains only `.archive/` — "there is no
  active prompt material" ([`/prompt/.archive/README.md`](/prompt/.archive/README.md) lines 1–6), so its
  input corpus is retired.
- **doc/ rehoming is deliberate, not finished-by-fiat:** remaining files
  "are being rehomed deliberately, with citation repair" ([`structure.md`](/constitution/structure.md)
  lines 69–73).
- **"The tool running again soon":** nothing is broken — `node tool/agents.ts`
  is runnable and tested ([`tool/assembly/README.md`](/tool/assembly/README.md) lines 30–33). What does
  not exist is the production manifest (deliberately deferred,
  [`agents-assembly.md`](/constitution/agents-assembly.md) lines 65–83) and an assembled `AGENTS.md`; the
  human's cutover direction is what changes that.
- **doc/GLOBAL validation tripwire:** its `status: deprecated` fails
  `fragmentMeta` (`draft|stable` only) — harmless while manifest-driven
  ("acts as a tripwire against accidental re-inclusion"); the parent was to
  pick a story: bless fail=retired or extend the status vocabulary
  ([`global-research0.glm53h.md`](/design/refs/global-research0.glm53h.md) lines 181–188). Relevant again the
  moment a manifest is written.

## 4. Tensions and alternatives recorded but not resolved

- **Monolith sections with no fragment source.** Current `AGENTS.md`
  headings (rg `^#{1,2}` on the live file): subagents + architectural
  stewardship (lines 1–21), Experimenting (22–27), Documentation/wave/
  doc-pass/prompts/Writing (28–106), well known directories (107–119),
  Project Planning (120–126), Coding advice (127–134), Rust (135–153),
  JavaScript/npm (154–223), jj + jj-hunk (224–255), beads (256–364),
  webcomponents (365–368), shell usage (369–371). Only four fragments exist:
  `doc-constitution`/100, `code`/200, `praxis-ticket-pointers`/400,
  `refs`/500 (metadata frontmatters in each `GLOBAL.md`). The inventory's
  finding still binds: "No focused existing home was found for the Rust
  guidance (133–151), Beads CLI operations (253–361), or harness shell
  persistence (366–368). No existing GLOBAL was found for JS, jj, Rust,
  Beads, collaboration, or workspace discovery"
  ([`inventory0.gpt56s.md`](/design/rekon/context-weave/inventory0.gpt56s.md) lines 155–157). Every cutover
  plan must answer where these live: extract first (per the contract) or
  assemble a partial document (which contradicts the recorded contract).
- **Global vs local glossary homes.** Shared [`CONTEXT.md`](/CONTEXT.md) at the
  root vs each fragment carrying only the terms its instructions need
  ([`constitution/GLOBAL.md`](/constitution/GLOBAL.md) lines 60–64; [`north-star0`](/design/rekon/context-weave/north-star0.gpt6a.md)
  lines 75–81; [`init1`](/design/feature-lines/init1.gpt.md) lines 63–69). Injecting all of `CONTEXT.md`
  everywhere was named and *not* chosen ([`init1`](/design/feature-lines/init1.gpt.md) line 52).
- **KISS fragment bodies vs honing-era coverage.** The 2026-10-06 human
  direction makes "a useful cue plus a direct link" the GLOBAL default; the
  honing workshop's deliberate redundancy is an exception "not the default
  template for new fragments" ([`constitution/README.md`](/constitution/README.md) lines 260–287, quote
  at 279–281; analysis at [`global-research0`](/design/refs/global-research0.glm53h.md) lines 86–93).
  Meanwhile the workshop warns to keep "costly-to-miss guards… visible while
  routing is unsettled" ([`context-weave/README.md`](/design/rekon/context-weave/README.md) lines 104–108) —
  measured costs: constitution fragment 4,148 prose bytes vs refs 381
  ([`global-research0`](/design/refs/global-research0.glm53h.md) lines 104–108, 163–166).
- **Coverage vs deferred extraction.** The assembler contract requires full
  fragment coverage before overwrite ([`agents-assembly.md`](/constitution/agents-assembly.md) lines
  74–76); the migration design separately warns against combining ownership
  and activation into one giant rewrite and against extraction-equals-deletion
  ([`agents-modularization0.sol56x.md`](/design/rekon/agents-modularization0.sol56x.md) lines 116–118, 467–478).
  Any shortcut (e.g., wrapping whole AGENTS sections as temporary fragments)
  is a new decision the record has not endorsed; the parity-ledger discipline
  ([`inventory0`](/design/rekon/context-weave/inventory0.gpt56s.md) as ledger, [`agents-modularization0`](/design/rekon/agents-modularization0.sol56x.md)
  lines 240–254) is the recorded alternative.
- **Two CLI implementations coexist** with no cross-tool byte identity and a
  different generated header — a document generated by the old command is
  stale under the new until rewritten ([`tool/assembly/README.md`](/tool/assembly/README.md) lines
  65–67, 106–112). Cutover should name which implementation produces the
  live file (the Effect one, per [`tool/README.md`](/tool/README.md) lines 28–32).
- **Unresolved migration questions** carried forward verbatim from
  [`agents-modularization0.sol56x.md`](/design/rekon/agents-modularization0.sol56x.md) lines 499–514: owner for
  cross-cutting workspace paths (`workspace/` vs `rekon/`); heading-text
  normalization at first cutover; project-local skill discovery without
  committing workspace machinery upstream; machine-readable routing fixtures;
  portable addressing of machine-local canonical docs (`~/archive/doc/jj.md`);
  which AGENTS passages are genuinely no-ops; per-language skills vs shared
  router.

## 5. Reading order for a fresh session

1. [`/constitution/structure.md`](/constitution/structure.md) — the placement contract: homes,
   tool shape, doc/ retirement, shortcodes (2026-10-06 state).
2. [`/tool/assembly/README.md`](/tool/assembly/README.md) then
   [`/constitution/agents-assembly.md`](/constitution/agents-assembly.md) — the runnable tool, then the
   binding fragment/manifest contract and its deferral guard.
3. [`/design/rekon/context-weave/README.md`](/design/rekon/context-weave/README.md) — the workshop: lanes,
   passes, guards; [`/design/rekon/context-weave/inventory0.gpt56s.md`](/design/rekon/context-weave/inventory0.gpt56s.md)
   for the line ledger (skim; the AGENTS SHA it indexes is one revision old).
4. [`/design/feature-lines/init1.gpt.md`](/design/feature-lines/init1.gpt.md) — the glossary-first
   requirement, what it does and does not establish, candidates/regather.
5. [`/CONTEXT.md`](/CONTEXT.md) plus [`/constitution/GLOBAL.md`](/constitution/GLOBAL.md) and
   [`/code/GLOBAL.md`](/code/GLOBAL.md) — the working glossary and the two honing
   fragments as concrete shape.
6. [`/design/refs/global-research0.glm53h.md`](/design/refs/global-research0.glm53h.md) — fragment
   inventory, byte costs, KISS direction, assembler constraint details.
7. [`/design/rekon/agents-modularization0.sol56x.md`](/design/rekon/agents-modularization0.sol56x.md) — the
   two-transformations migration design and its open questions.
8. [`/design/rekon/hyalo-toc0.glm53m.md`](/design/rekon/hyalo-toc0.glm53m.md) with
   [`/tool/README.md`](/tool/README.md) — the hyalo candidate and toolkit conventions.

[`/design/rekon/shortcodes0.glm53m.md`](/design/rekon/shortcodes0.glm53m.md) is the escalation read if any
`g`/`p`/`t` naming question resurfaces.

## Cross-references

- [`/constitution/README.md`](/constitution/README.md) §GLOBAL (lines 260–287) **specifies** the
  GLOBAL writing contract (KISS default, traceability) that fragment bodies
  must satisfy at cutover; §Honing (lines 289–300) records the dated
  workshop exception.
- [`/design/rekon/doc-cull0.glm53m.md`](/design/rekon/doc-cull0.glm53m.md) **records** the doc/
  retirement evidence this session's `AGENTS.old.md` move rhymes with
  (retire with preservation, not deletion).
- [`/design/rekon/agents-effect0.sol61xh.md`](/design/rekon/agents-effect0.sol61xh.md) **evidences** the
  Effect 4 API verification behind the runnable tool.
- [`/design/rekon/phoenix0.gpt6a.md`](/design/rekon/phoenix0.gpt6a.md) **frames** why expressions
  (like AGENTS.md) may be regenerated while understanding is preserved —
  the deeper rationale the cutover serves.
- Beads `rekon-agents-maintenance` (epic, in_progress) **owns** the
  migration; its notes log every authorization boundary crossed so far
  (`bd show rekon-agents-maintenance`).
