---
type: GuidanceInventory
title: Ambient guidance inventory for human-steered extraction
description: A line-complete parity ledger for the current hand-maintained AGENTS.md, locating likely domain owners, overlaps, review candidates, and bounded coarse extraction groupings without changing runtime guidance.
resource: /design/rekon/context-weave/inventory0.gpt56s.md
tags: [rekon, agents, ambient-context, inventory, modularization, parity-ledger]
status: draft
generated: { by: "model:openai/gpt-5.6-sol-high", at: 2026-09-06 }
stale_after: 2026-12-06
extensions:
  ticket: rekon-phoenix
  related_owner: rekon-agents-maintenance
  baseline_commit: 83dadd4045822be9e9224b605290df93e0c211de
  inspected_working_copy_commit: c00eb954d82e90a30a8de9f3bbaf6d9c9ce8343d
  inspected_agents_sha256: 4bf6260ebf26db5e011affca5c8a1cffae4a9a1270db702b253d6a8f93d7f1bf
  coverage: 368/368 lines in 32 non-overlapping rows
sources:
  - id: ambient-guidance
    resource: /AGENTS.md
    title: Hand-maintained workspace agent guidance, inspected working bytes
    author: human:rektide
    last_modified: 2026-09-06
  - id: modularization-design
    resource: /design/rekon/agents-modularization0.sol56x.md
    title: Modularizing Rekon's Global Agent Context
    author: model:gpt-5.6-sol-max
  - id: assembly-contract
    resource: /constitution/agents-assembly.md
    title: Assembling AGENTS.md from GLOBAL.md fragments
    author: project:rekon
  - id: constitution
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution
    author: project:rekon
  - id: code-module
    resource: /code/README.md
    title: Coding practice
    author: project:rekon
  - id: js-module
    resource: /js/README.md
    title: JavaScript / TypeScript development skill
    author: project:rekon
  - id: jj-module
    resource: /vcs/jj/README.md
    title: jj + git working guide
    author: project:rekon
---

<a id="ambient-guidance-inventory"></a>
# Ambient Guidance Inventory

## Scope and source identity

This report inventories the live, hand-maintained [`/AGENTS.md`](/AGENTS.md) as
source material for a human-steered extraction. It is an inventory, not an
accepted module architecture, production manifest, activation decision, or
cleanup authorization. In particular, “candidate review” below never means
“junk,” and apparent age never establishes removability.

The exact committed baseline beneath the working copy was
`83dadd4045822be9e9224b605290df93e0c211de`. At inspection time, jj represented
the working copy as commit
`c00eb954d82e90a30a8de9f3bbaf6d9c9ce8343d`. The inspected `AGENTS.md` bytes had
SHA-256:

```text
4bf6260ebf26db5e011affca5c8a1cffae4a9a1270db702b253d6a8f93d7f1bf
```

Those bytes differ from the parent version (SHA-256
`92f8ad8492dde609f0a026d20c5f8241f673d5f6b067c87a5c368c3747c53bce`) by one
known operator edit at line 238: the push rule now permits pushing when
explicitly directed by the operator. This report treats the working bytes—not
the parent rendering—as the inventory source and does not touch that edit.

The governing migration constraint is **ownership before activation**. A
“relocate” entry proposes a source owner; it does not imply removing the rule
from ambient context at first cutover. The columns deliberately separate:

- **Relocate / owner:** where canonical ownership could live.
- **Initial ambient retention:** what should remain globally visible while
  ownership is decomposed.
- **Refinement or removal review:** later human decisions, never actions implied
  by relocation.

## Complete non-overlapping parity ledger

Line ranges include their heading and trailing blank lines up to the next row.
Fenced code ranges are called out explicitly so comments beginning with `#`
inside shell examples are not mistaken for document sections.

| # | `AGENTS.md` lines | Content and dependencies | Likely existing or focused home | Relocate / owner proposal | Initial ambient retention | Refinement or removal review |
|---:|---:|---|---|---|---|---|
| 1 | 1–11 | **Subagents:** background execution, deliberative returns, model availability override, priming with research, agent-owned artifacts and commits, no-overwrite posture, references, continuation identity. Depends on harness vocabulary and the architectural-stewardship row. | `agents/` is present but only contains [`followup.md`](/agents/followup.md.disabled); [`assess/gates/README.md`](/assess/gates/README.md) covers waiting and receipt discipline, not dispatch as a whole. | A focused collaboration/agent-operation module, with the gate material referenced rather than copied. | Preserve the costly-to-miss dispatch, overwrite, artifact, return-reference, and continuation rules during ownership cutover. | Review “task_id” against the active harness’s `sessionID` vocabulary; clarify rather than delete. Model-selection wording is environment-specific and may belong in a host overlay. |
| 2 | 12–20 | **Architectural stewardship:** delegation boundary, consequential checkpoints, trust-but-review, checkpoint return shape, cooperative annealing and future-sensitive follow-up. Depends on row 1 and overlaps evidence readiness. | [`assess/README.md`](/assess/README.md) and [`assess/gates/README.md`](/assess/gates/README.md) are substantial nearby homes; neither currently claims all architectural stewardship. | Assessment/stewardship practice, or the same collaboration module as row 1 with links into `assess/`. | Preserve globally at first: these rules prevent premature delegation of architectural responsibility before any skill is routed. | Human decision: whether stewardship is a universal operating invariant or an assess-on-demand praxis. Do not collapse “trust checks ran” into “trust correctness.” |
| 3 | 21–26 | **Experimenting:** prefer `rg`/`fd`, define variant workspaces, and keep domain-grouped `.test-agent` scratch with a README. Depends on workspace-path conventions in row 9. | The constitution covers scratch lifecycle; no workspace/experimentation module exists. | A small workspace/experimentation owner, or constitution for scratch semantics plus a workspace owner for tools/variants. | Retain the scratch-location rule; retain `rg`/`fd` only if these tools are reliably present throughout the target host scope. | Review whether every disposable scratch subtree needs a README; distinguish a helpful default from an unconditional ceremony. |
| 4 | 27–33 | **Documentation/reference basics:** canonical links, scratch/design/doc placement, addenda, and a WIP `summarizer` note. | [`constitution/README.md`](/constitution/README.md) already owns module links, design/doc posture, supporting artifacts, and addenda; [`constitution/GLOBAL.md`](/constitution/GLOBAL.md) projects part of it ambiently. | Constitution documentation module. | Preserve canonical-link and safe-destination guidance needed before skill discovery; initially retain inherited prose until normalized parity is reviewed. | Review line 31’s unclear “use pandoc to write OKF tags for the title” against the current full-frontmatter contract. Re-check the WIP `summarizer` status; the note may need replacement by an honest route, not age-based deletion. |
| 5 | 34–56 | **Wave method and naming:** wave types, revision/synthesis suffixes, standard names, accepted-tip symlinks, stage-setting, mandatory model suffix, true model identity, non-overwrite behavior, iteration, indexes, and same-wave isolation. Depends on documentation lifecycle and subagent collaboration. | [`constitution/README.md`](/constitution/README.md#doc-constitution-patterns) now owns independent waves, model suffixes, isolation, addenda, and accepted-tip semantics at a higher level. | Constitution README plus a focused wave support document if the complete operational recipe is still wanted. | Retain non-overwrite, true-model suffix, and same-wave isolation globally until reliable routing exists; preserve all inherited text during first ownership move. | Reconcile “always suffix 0” and “prefix-less symlink” terminology with current constitutional wording. Decide whether index maintenance encouragement is part of waves or corpus maintenance. |
| 6 | 57–70 | **Doc-pass:** seven-step concept search, relationship verification, cross-links, reciprocal links, index maintenance, link checking, and optional subagent search return shape. | Constitution names corpus validation/doc-pass as opt-in but does not yet carry this complete recipe. | Constitution support document, potentially exposed through the constitution skill. | Preserve the trigger and costly-to-miss “verify substantive relationship” / “explain why” rules; move procedural depth only after routing is proven. | Clarify “recommended after most waves of significant weight” into a recognizable threshold. No deletion merely because it is procedural. |
| 7 | 71–75 | **Prompts/discussion:** multiple-choice preference and discussion before questions. | No direct module. Collaboration/human-steering is the nearest domain; the constitution is not a natural owner of interaction style. | Collaboration/human-steering owner, potentially combined with rows 1–2 within ticket constraints. | Likely ambient because it changes interaction before routing. | Review “prefer multiple choice” as a preference, not a prohibition on necessary free-form questions. |
| 8 | 76–104 | **Writing and OKF:** Markdown style, trustworthy frontmatter, full example, bundle-root links, and reserved `index.md`/`log.md`. Fenced Markdown example is exactly lines **81–99**. | [`constitution/README.md`](/constitution/README.md#doc-constitution-module-readme) and [`constitution/GLOBAL.md`](/constitution/GLOBAL.md) already own this domain. | Constitution module. | Keep concise Markdown/link/frontmatter invariants ambient initially; complete example is a strong later depth candidate. | Reconcile the sample’s fields with current OKF and constitution language; distinguish example comments from required schema. |
| 9 | 105–117 | **Known directories/tools:** archive-first source lookup, doc/wiki/src/ff/opt/srv locations, `llms.txt`, `dl`, scratch placement, and executable scripts. Depends on experimenting and language-specific script rules. | No general workspace map exists. [`atproto/README.md`](/atproto/README.md) repeats archive/wiki conventions only for ATProto; [`upstream/README.md`](/upstream/README.md) repeats archive-first provenance for mirrors. | Workspace/source-discovery module. Keep domain-specific inventories in their domains. | Preserve archive-first, first-party checkout, wiki, `dl`, and scratch rules before routing; these determine where agents search and write. | Review the self-negating `llms.txt` sentence and define root/availability semantics. Move the script/shebang rule to engineering/JS if it is not truly workspace-wide. Validate whether `/opt`, `/srv`, and `~/ff` remain common enough for ambient load. |
| 10 | 118–124 | **Project planning:** no delivery timeframes, meaningful Mermaid identifiers, pre-1.0 breakage, and mandatory domain-grouped layouts/options. | Fully duplicated and expanded in [`code/planning.md`](/code/planning.md), summarized in [`code/README.md`](/code/README.md#code-planning). | Existing `code/` module. | Retain compact planning invariants ambient initially; the existing [`code/GLOBAL.md`](/code/GLOBAL.md) currently retains only no-time-estimates and is not yet assembler-ready. | Review whether “breaking changes are fine” needs explicit project/user constraints; do not interpret `<1.0` as permission beyond the requested scope. |
| 11 | 125–132 | **Coding advice and checkpoint discipline:** Let It Fail, no automatic pruning, commit coherent stages, preserve predecessors before revision, and separate human check-in from VCS checkpoint. | Fully duplicated/expanded across [`code/coding-advice.md`](/code/coding-advice.md), [`code/commits.md`](/code/commits.md#commits-checkpoints), and [`code/review-handling.md`](/code/review-handling.md). | Existing `code/` module. | Preserve error/cleanup posture and checkpoint-before-revision globally at first cutover. | Line 128 names Compfuzor while the surrounding file presents workspace rules; establish intended scope. “Let It Fail” already has an “when it makes sense” qualifier that must survive compression. |
| 12 | 133–136 | **Rust publication:** `cargo bump`, default patch bump, commit, then publish. | No Rust module exists. | Focused Rust engineering module or a language/toolchains module shared with JS at ticket grain. | Initially preserve because release ordering is costly to miss when Rust publication is already underway. | Verify `cargo bump` availability and the intended crate/workspace release process before promoting it as canonical; publication itself remains explicitly requested work. |
| 13 | 137–147 | **Preferred Rust modules/checks:** figment2, clap/clap_complete, bon, nextest, Criterion extensions, jiff, clippy, tracing. | No Rust module exists. | Rust engineering module, possibly support depth under a broader toolchains pass. | Preserve inherited preferences during ownership decomposition; later language recognition may make most skill-only. | Fix the malformed backtick around clippy only in a reviewed editorial pass. Library preferences are choices, not timeless facts; attach review cadence and project-local exceptions. |
| 14 | 148–151 | **Clap + Figment2 integration:** derive set, serialized defaults, and precedence. | No Rust module exists. | Rust module support section. | Preserve initially; likely on-demand depth once a Rust/config trigger is demonstrably reliable. | Verify exact figment2 API/version and whether absent values preserve intended precedence. Do not remove based only on specificity. |
| 15 | 152–155 | **TypeScript imports:** explicit `.ts` extension for sibling project code. | Fully duplicated in [`js/README.md`](/js/README.md#running-typescript). | Existing `js/` module. | Retain as a cheap, common convention when TypeScript work is detected; ownership move should not change activation yet. | Clarify package/build exceptions if required by target tooling. |
| 16 | 156–167 | **Preferred npm modules/package management:** node prefix, vitest, gunshi, oxfmt/oxlint, tsdown, tsgo, config libraries, pnpm install and no hand-editing dependency fields. | Duplicated and updated in [`js/README.md`](/js/README.md#package-management). | Existing `js/` module. | Preserve inherited preferences initially; later reduce to package-manager and critical typecheck/build distinctions if routing proves reliable. | Line 163 is an incomplete sentence. Reconcile standalone oxfmt/oxlint advice with the README’s newer vite-plus bundling guidance. Keep “install with `@latest`” distinct from lockfile reproducibility and existing-project constraints. |
| 17 | 168–173 | **Async discipline:** avoid sync/callback APIs where async/promise APIs exist, use `node:fs/promises`, defer awaits until values are needed. | Fully duplicated in [`js/README.md`](/js/README.md#async-discipline). | Existing `js/` module. | Preserve initially; likely a compact JS GLOBAL candidate. | “Avoid callback” and deferred-await advice need correctness/lifetime exceptions; avoid turning concurrency preference into floating promises. |
| 18 | 174–181 | **Build concerns:** distribution-only output, pnpm, TypeScript-first/direct execution, noEmit/import extensions/isolated declarations. | Duplicated and expanded in [`js/README.md`](/js/README.md#running-typescript) and its script/build sections. | Existing `js/` module. | Preserve direct-source development and distribution-only build posture initially. | Review Node version assumptions behind type stripping. Correct typo and clarify that `isolatedDeclarations` applies when declarations are emitted. |
| 19 | 182–185 | **TypeScript classes:** make members public while not guaranteeing implementation stability. | Duplicated in [`js/README.md`](/js/README.md#typescript-style). | Existing `js/` module. | Preserve inherited behavior at first cutover; likely skill depth later. | Clarify whether this rejects language-level `private` universally or expresses an API philosophy with exceptions. |
| 20 | 186–214 | **Concurrently script taxonomy:** build/check/fix namespaces, aggregate globs, examples, labels, and dev target. Fenced text example is exactly lines **190–194**. | Duplicated and evolved in [`js/README.md`](/js/README.md#npm-script-structure-with-concurrently). | Existing `js/` module. | Preserve all at ownership cutover; this recipe is a strong later skill-depth candidate. | Reconcile `build:type` versus `check:type`, vite-plus commands, and project-specific `wxt`/`vici` examples. Verify whether concurrently remains the default for new projects rather than copying an old stack globally. |
| 21 | 215–221 | **CLI main-module pattern:** simple file CLIs, sibling `foo.main`, shebang/executable, realpath main detection, nonblocking `.then`, and direct `.ts` usage examples. | Duplicated and corrected with TypeScript examples in [`js/README.md`](/js/README.md#cli-patterns). | Existing `js/` module. | Preserve main-detection safety and direct-source usage initially; recipe depth can later activate on CLI work. | Reconcile `.js` wording with TypeScript-first convention. Verify exact Node runtime semantics and decide whether dynamic import should invoke an export or rely on side effects. |
| 22 | 222–244 | **jj/git everyday policy:** dated bookmarks, explicit-path commits, expected empty working copy, no redundant verification, no edit/squash, diff modes, DSL help/operators/templates, compact log, push policy, message rules, actor assumption, and jj-hunk route. | Almost entirely duplicated/expanded in [`code/commits.md`](/code/commits.md); deeper read/forensics material belongs to [`vcs/jj/README.md`](/vcs/jj/README.md). | Existing `code/` owns commit policy; `vcs/jj/` owns tool forensics. Avoid creating a competing jj policy owner. | Preserve commit safety, push line 238 exactly as inspected, message rules, explicit paths, and DSL non-guessing globally. | Clarify “do not verify” as avoiding ritual VCS rechecks, not skipping result/test verification. Review dated-bookmark universality and the sole-actor assumption around pre-existing dirty work. The operator-edited push exception must not regress. |
| 23 | 245–252 | **jj-hunk:** purpose, full-ID spec, dry-run, wholesale rewrite method, and route to full docs. | Fully duplicated/expanded in stable [`code/jj-hunk.md`](/code/jj-hunk.md). | Existing `code/` module. | Keep the trigger, full-ID hazard, and mandatory dry-run ambient; move examples/recovery only after routing. | Add owned-change safeguards around restoring a wholesale rewrite; do not turn the recipe into permission to overwrite unrelated edits. |
| 24 | 253–262 | **Beads basics:** working directory, stable namespaced IDs, epic/child roles, append-only notes, blocker direction, Dolt/export, and prohibition on `bd prime`. | Only generic upstream-style [`.beads/README.md`](/.beads/README.md) exists; it does not own these workspace conventions. Constitution discusses tickets conceptually, not CLI operations. | New focused Beads/work-tracking module. | Preserve all basics globally, especially stable IDs, dependency direction, history, and `bd prime` prohibition. | Re-verify `export.auto` and Dolt behavior against the installed Beads version. Separate workspace policy from tool implementation facts. |
| 25 | 263–271 | **Beads minimal command pattern.** Fenced shell block is exactly lines **267–270**. | No focused home. | Beads module routine-work section. | Preserve a minimal create form only if agents commonly create tickets without loading the module; otherwise retain during ownership first and evaluate later. | Verify whether `--title` is required/current. Keep stable-ID policy even if command syntax evolves. |
| 26 | 272–296 | **Beads linked work:** prefix lookup, epic/feature create, parent-child/blocker edges, show, ready-list. Fenced shell block is exactly lines **274–295**. | No focused home. | Beads module routine graph section. | Preserve dependency direction and verification globally; full command examples can become depth after routing. | Ticket type/label/priority examples are policy-laden; confirm defaults and current CLI syntax before canonization. |
| 27 | 297–312 | **Beads investigate/close:** search, inspect, claim with notes/labels, close with reason. Fenced shell block is exactly lines **299–311**. | No focused home. | Beads module lifecycle section. | Preserve search-before-create, inspect, append-notes, and evidence-backed closure globally. | Clarify when claiming is expected and whether all completed work requires closure by the acting agent. |
| 28 | 313–331 | **Beads supersession:** create replacement, supersedes edge, close old, cycle check, inspect replacement. Fenced shell block is exactly lines **315–330**. | No focused home. | Beads module deeper lifecycle/recovery section. | Preserve “prefer update/rename over duplicate” via row 29; retain this inherited block initially, then consider skill depth. | Full supersession is uncommon and reliably triggerable; likely depth, but only after the new owner and route exist. Verify edge semantics. |
| 29 | 332–340 | **Beads rename:** prefer rename/update, dependency rewrite, simple command. Fenced shell block is exactly lines **336–339**. | No focused home. | Beads module identity-repair section. | Preserve the preference to rename/update instead of duplicating; command can remain ambient initially. | Verify `bd rename` behavior and limits against current version. |
| 30 | 341–361 | **Beads broad recovery:** deliberate JSONL export/edit, database removal, import, list/show verification. Fenced shell block is exactly lines **343–360**. | No focused home. | Beads module recovery support document. | Preserve the warning that database removal discards state and requires deliberate export; inherited text stays ambient through ownership cutover. | This is destructive, specialized depth and a strong later removal-from-ambient candidate—not a removal-from-knowledge candidate. Require explicit authorization and current-version verification before execution. |
| 31 | 362–365 | **Web components:** wc-toolkit CEM generation and JSDoc. | Fully duplicated in [`js/README.md`](/js/README.md#web-components). | Existing `js/` module. | Preserve initially; likely skill-only once JS/web-component routing is shown reliable. | Verify current wc-toolkit invocation/integration; the rule says what outcome is required but not the exact command. |
| 32 | 366–368 | **Shell usage:** environment variables do not persist across shell executions and must be reset each time. | No shell/workspace module exists. | Minimal workspace/runtime owner, possibly combined with rows 3 and 9. | Preserve globally: it changes commands before a shell skill could be loaded and prevents immediate failures. | Scope it explicitly to this agent harness’s separate shell invocations; it is not a universal shell fact. |

### Coverage receipt

- **Source lines:** 368.
- **Ledger rows:** 32.
- **Covered union:** lines 1–368, contiguous.
- **Gaps:** 0.
- **Overlaps:** 0.
- **Explicit fenced ranges:** 8 (`81–99`, `190–194`, `267–270`,
  `274–295`, `299–311`, `315–330`, `336–339`, `343–360`).
- **True Markdown sections represented:** 25 headings outside fences (11 H1,
  12 H2, 2 H3). Shell comments inside fences are not counted as headings.

## Existing homes and duplication map

The repository already contains several plausible owners. Their presence is
evidence for reuse, not proof that their draft GLOBALs are accepted or that
runtime text may yet be removed.

| Existing home | `AGENTS.md` coverage already represented | Inventory implication |
|---|---|---|
| [`constitution/README.md`](/constitution/README.md), [`constitution/GLOBAL.md`](/constitution/GLOBAL.md) | Much of 27–104: knowledge flow, README/SKILL/GLOBAL contract, design/doc posture, waves, model suffixes, same-wave isolation, addenda, accepted tips, OKF-style labels, index/log roles. | This is the strongest existing owner for documentation practice. The full wave naming and doc-pass recipes remain more detailed than the current README. `constitution/GLOBAL.md` is the only fragment with assembler metadata and remains draft. |
| [`code/README.md`](/code/README.md), [`code/commits.md`](/code/commits.md), [`code/planning.md`](/code/planning.md), [`code/coding-advice.md`](/code/coding-advice.md), [`code/review-handling.md`](/code/review-handling.md), [`code/jj-hunk.md`](/code/jj-hunk.md) | Nearly complete duplication/expansion of 118–132 and 222–252. | Reuse this owner instead of creating planning, coding-advice, commits, and jj-hunk tickets. [`code/GLOBAL.md`](/code/GLOBAL.md) is a draft projection but lacks the frontmatter required by the implemented assembler and intentionally contains only a subset. |
| [`vcs/jj/README.md`](/vcs/jj/README.md) | Tool depth overlapping the DSL/log part of 222–244; stable forensics, reads, rewrites, and traps beyond AGENTS. | Keep everyday commit policy in `code/`; route tool forensics here. Avoid two canonical owners for the push/commit rules. |
| [`js/README.md`](/js/README.md) and `js/SKILL.md -> README.md` | Nearly complete duplication and editorial evolution of 152–221 and 362–365. | Strong existing owner. It has no GLOBAL and no OKF frontmatter, so ownership/activation cutover is not complete. Reconcile evolved vite-plus and TypeScript examples rather than blindly copying either version. |
| [`assess/README.md`](/assess/README.md), [`assess/gates/README.md`](/assess/gates/README.md) | Partial conceptual overlap with 3–20: evidence gates, return receipts, parent responsibility, readiness before dependent judgment. | Possible depth for stewardship, but not a replacement for all dispatch/collaboration rules. The parent’s manual workshop should decide the seam. |
| [`agents/followup.md`](/agents/followup.md.disabled) | Concrete subagent behavior related to source lookup, verification, preserving changes, and concise receipts. | A harness agent definition is not automatically a canonical collaboration knowledge module; reference it as implementation evidence, not a wholesale owner. |
| [`atproto/README.md`](/atproto/README.md) | Repeats archive/wiki/source lookup conventions in an ATProto-specific inventory. | Keep the domain inventory there; move generic archive-first rules to a workspace owner rather than treating ATProto as their source. |
| [`upstream/README.md`](/upstream/README.md) | Overlaps archive-first source preference, provenance, commit separation, and no-pruning posture. | This is a specialized practice, not the owner of all workspace discovery or commit behavior. Preserve links to shared owners instead of copying ambient rules into it. |

No focused existing home was found for the Rust guidance (133–151), Beads CLI
operations (253–361), or harness shell persistence (366–368). No existing
GLOBAL was found for JS, jj, Rust, Beads, collaboration, or workspace discovery.

## Candidate stale, ambiguous, or misleading instructions

These are review candidates, not removal decisions. Each remains covered in the
ledger and should survive ownership decomposition unless a human explicitly
chooses a disposition.

1. **Harness vocabulary and scope:** line 10 says `task_id`; current available
   delegation uses a session ID. Line 368 describes this harness’s isolated
   shell executions, not shell behavior in general. Lines 5 and 238 also encode
   host/operator policy and may belong in a host overlay even when their effects
   stay ambient.
2. **Documentation wording drift:** line 31’s pandoc/OKF-title instruction is
   unclear beside the constitution’s current frontmatter contract. Line 32’s
   WIP `summarizer` note needs a current capability check. Line 45’s
   “prefix-less symlink” and line 43’s universal `0` suffix should be reconciled
   with the newer constitution rather than silently discarded.
3. **Self-negating or malformed prose:** line 111 dismisses its own `llms.txt`
   rule; line 145 has malformed code quoting; line 163 and line 180 are sentence
   fragments/typos. These are editorial defects, not evidence against the
   underlying guidance.
4. **Overbroad policy wording:** line 122 can read as blanket permission for
   breaking changes; line 184 can read as a universal ban on private members;
   line 229 can read as “never verify outcomes”; line 242’s sole-actor
   assumption can obscure pre-existing dirty work. Preserve the intended
   guardrail while narrowing the claim.
5. **Scope leakage:** line 128 names Compfuzor inside a workspace-wide coding
   section. Lines 116 and 362–365 are language-specific content outside the JS
   section. Line 224’s dated-bookmark rule may be workflow-specific rather than
   universal.
6. **Version-sensitive tool facts:** Rust APIs and `cargo bump` (133–150),
   vite-plus/npm tooling (156–220), jj commands (222–252), and Beads/Dolt syntax
   (253–361) need version-aware verification when promoted. Existing detailed
   JS and jj homes already contain newer or better-qualified accounts.
7. **Destructive depth:** lines 341–361 deliberately describe database removal
   and rebuild. Their warning should remain visible, but the recipe is a prime
   candidate for routed recovery depth once a Beads module exists.

## Coarse grouping options within the existing epic budget

The owner epic `rekon-agents-maintenance` already has the **closed assembler**
child and the **in-progress skillify** child. To stay at five total records
(parent + four children), only **two additional children** fit. Each option below
therefore treats `skillify` as one of at most three active domain/coordinating
passes; it does not propose a second migration program or ticket-per-section.

### Option A — established homes, then missing homes

1. **Existing `skillify`: documentation/collaboration ownership and parity
   coordination** — constitution/waves/doc-pass plus the manual stewardship
   workshop, with the assembler remaining the already-closed implementation
   prerequisite.
2. **Additional pass: engineering modules** — reuse `code/`, `js/`, and
   `vcs/jj/`; create a Rust home within the same coherent engineering pass.
3. **Additional pass: workspace operations** — Beads plus workspace discovery,
   experimentation, and shell persistence.

Pressure: the final pass combines issue tracking with environment discovery,
which share operational cadence but not a vocabulary.

### Option B — human/work context versus implementation/tool context

1. **Existing `skillify`: migration contract, parity ledger, and cutover
   coordination** — no independent domain extraction beyond maintaining the
   ownership-before-activation rules.
2. **Additional pass: knowledge and collaboration** — subagents/stewardship,
   prompts, documentation, waves, doc-pass, OKF, scratch lifecycle.
3. **Additional pass: engineering and workspace operations** — code/planning,
   Rust, JS, jj, Beads, paths, experiments, web components, shell behavior.

Pressure: the third pass is broad, but it can reuse multiple already-written
modules and reserve new writing mainly for Rust, Beads, and workspace gaps.

### Option C — action lifecycle versus technology conventions

1. **Existing `skillify`: knowledge lifecycle** — documentation, waves,
   doc-pass, OKF, source locations, scratch, and parity/cutover coordination.
2. **Additional pass: work lifecycle** — subagents/stewardship, prompts,
   planning, checkpoints, jj commit policy, jj-hunk, Beads, and shell execution.
3. **Additional pass: implementation conventions** — coding posture, Rust,
   JS/TypeScript, npm scripts/CLI, and web components.

Pressure: the work-lifecycle pass crosses several established directories, but
it groups by one human-visible journey: decide, delegate, record, commit, and
track.

All three options keep the child count bounded: closed assembler + active
skillify + two possible passes = four children, five records including the
owner epic. They are inventory groupings for the parent’s decision, not ticket
requests.

## Unresolved decisions for the human workshop

1. Does architectural stewardship canonically belong with agent collaboration
   or assessment praxis, and what minimum invariant must stay ambient before
   either route activates?
2. Is generic workspace/source discovery a real domain module, a small host
   overlay, or a collection of local adaptations? This decides rows 3, 9, and
   32 without requiring three tickets.
3. Should Rust receive a durable language home parallel to `js/`, or should
   language conventions share a broader engineering/toolchains module while
   retaining domain-grouped support documents?
4. Is `code/commits.md` the sole owner of day-to-day jj policy while `vcs/jj/`
   owns forensic/tool depth? Existing prose strongly suggests this seam, but it
   should be explicitly accepted before relocation.
5. Which Beads commands must remain available before routing, versus which
   recovery branches can leave ambient context only after positive/boundary
   activation tests?
6. Does first production assembly preserve normalized operating prose and
   heading order exactly, or allow reviewed editorial fixes during the same
   cutover? Mixing those changes makes parity harder to audit.
7. How should host-specific rules—available subagent models, continuation
   identity, push authorization, shell process isolation—compose with portable
   domain modules without losing their always-loaded effect?

## Cross-references

- [Modularizing Rekon's global context](/design/rekon/agents-modularization0.sol56x.md)
  **supplies the governing sequence**: assign every old passage an owner before
  changing activation or ambient cost.
- [AGENTS assembly](/constitution/agents-assembly.md) **defines the eventual
  mechanical boundary**: complete fragment coverage, deterministic order,
  provenance, link safety, and no production manifest before migration parity.
- [Documentation constitution](/constitution/README.md) **already owns much of
  the documentation corpus** and distinguishes canonical README, exact SKILL
  exposure, ambient GLOBAL projection, evidence-preserving waves, and optional
  strong research mechanics.
- [Code practice](/code/README.md) **already owns and expands the planning,
  coding, review, commit, and jj-hunk passages**; its draft GLOBAL is a proposed
  projection, not yet a production fragment.
- [JavaScript/TypeScript](/js/README.md) **already owns and updates the JS and
  web-component guidance**, exposing the body through a SKILL symlink but not a
  GLOBAL fragment.
- [jj + git guide](/vcs/jj/README.md) **owns deep tool forensics** and helps keep
  tool knowledge distinct from the everyday commit policy in `code/`.
- [Assessment gates](/assess/gates/README.md) **provides nearby practice for
  deliberative returns and parent-owned readiness**, while leaving the broader
  collaboration/stewardship ownership seam open for the human workshop.
- [Upstream reference mirrors](/upstream/README.md) **shows related archive,
  provenance, no-pruning, and commit-separation behavior in a specialized
  workflow**, not a replacement owner for those cross-cutting rules.

## Limits

- No active instruction, GLOBAL, README, SKILL, manifest, symlink, or ticket was
  edited, deleted, created, or activated by this work.
- This report did not resolve naming/constitution archaeology or perform the
  parent-owned glossary-first workshop.
- No other new `design/rekon/context-weave/` report was read while composing
  this independent inventory.
- Tool/API freshness was assessed only from the requested local documents and
  visible repository state; version-sensitive candidates are flagged rather
  than claimed obsolete.
- The grouping options optimize for the stated record budget. They are not a
  final architecture and do not authorize the two possible additional tickets.
