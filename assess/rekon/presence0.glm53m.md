---
type: Assessment
title: Idea presence map — where recent additions live, with what reaches AGENTS.md
description: Per-idea breakdown of which parts landed in which homes for the Nov 2–3 session-day and (lighter) Oct 26–31, quoting the literal assembled AGENTS.md content, plus a reusable relative-path column template for reassessment passes.
resource: /assess/rekon/presence0.glm53m.md
status: draft
tags: [rekon, assessment, presence, agents-assembly, coverage, glossary, reassessment]
generated: { by: "model:zai-coding-plan/glm-5.3#max", at: "2026-11-03" }
sources:
  - { resource: "opencode://ses_e60122a28ffem0Q4yFFw0qqZEM", title: Commission the presence breakdown, literal AGENTS.md quotes, and shared relative-path table for a coming reassessment }
  - { resource: /agents.manifest.json, title: Assembly declaration of AGENTS.md inputs }
  - { resource: /AGENTS.md, title: Assembled output quoted as of working copy 6f1e87c2 plus uncommitted edits }
  - { resource: /CONTEXT.md, title: Glossary source of the magnitude entry }
  - { resource: /praxis/coverage/README.md, title: Coverage practice home }
  - { resource: /praxis/coverage/GLOBAL.md, title: Coverage ambient fragment }
  - { resource: /code/workspace/README.md, title: Known-places directory home }
  - { resource: /code/workspace/GLOBAL.md, title: Known-places ambient fragment }
  - { resource: /constitution/practice/GLOBAL.md, title: Durable-writing fragment carrying the decision-rationale bullet }
  - { resource: /praxis/candidates/README.md, title: Make-the-lean-inspectable section behind the ambient bullet }
  - { resource: /codes/ado.md, title: ado code row source }
  - { resource: /praxis/flights/random-combo.md, title: Random-combo flight model }
  - { resource: /tool/random-combo/README.md, title: Random-combo generator contract }
  - { resource: /agents/models/README.md, title: Dated model and variant inventory }
  - { resource: /agents/GLOBAL.md, title: Collaboration fragment, including the uncommitted variant table }
---

# What landed where — a presence map for reassessment

## Situation and ask

The human wants to reassess the recent session-day's work (and, lighter, the
past week). Before judging the ideas, we need an inventory of **which parts of
each idea are present in which homes** — and for everything that reaches the
assembled [`AGENTS.md`](/AGENTS.md), the **literal contents**, since quotes are
the ideal currency for that review. This document maps idea → home → verbatim
ambient presence, and defines the shared relative-path table used throughout.
It is an account of placement, not an evaluation; reassessment itself follows.

This doubles as a coverage-style pass over the session-days in question: every
commit in range is accounted for below, including the uncommitted working-copy
state. Idea rows carry the [presentation lenses](/praxis/coverage/README.md#present-the-nature-of-a-thread)
(`#net-new`, `#iterative`, …) where they help; the lenses are non-exclusive cues,
not standing.

## What AGENTS.md is actually made of

Answering "is it CONTEXT or GLOSSARY?": **both, plus the GLOBALs**.
[`agents.manifest.json`](/agents.manifest.json) declares it exhaustively:

- **`CONTEXT.md`** — the root *Rekon working vocabulary* glossary. Its body is
  copied verbatim into AGENTS.md (section `#glossary`).
- **`codes/GLOSSARY.md`** — the codes glossary, itself **generated** by
  `rekon codes` from [`codes/`](/codes) files and [`agents/`](/agents) frontmatter.
  Rows are assembled, not hand-written; edit a code's own file, then run `codes`.
- **Sixteen `GLOBAL.md` fragments** — `constitution/GLOBAL.md`,
  `constitution/practice/GLOBAL.md`, `agents/GLOBAL.md`, `code/workspace/GLOBAL.md`,
  `code/GLOBAL.md`, `vcs/jj/GLOBAL.md`, `forge/GLOBAL.md`, `code/js/GLOBAL.md`,
  `code/rust/GLOBAL.md`, `atproto/GLOBAL.md`, `praxis/beads/GLOBAL.md`,
  `praxis/ticket-pointers/GLOBAL.md`, `praxis/coverage/GLOBAL.md`,
  `praxis/consumer-accounts/GLOBAL.md`, `tool/GLOBAL.md`, `refs/GLOBAL.md`.

Two consequences that matter for reassessment:

1. **Quoting AGENTS.md is quoting a source file.** Fragment bodies are copied
   unchanged, so every quote below is simultaneously a quote of the source
   (`CONTEXT.md` body, `codes/*.md`/`agents/*.md` frontmatter rows, or the
   `GLOBAL.md` body). Cite by section id/anchor, not ToC line number — regens
   shift line numbers.
2. **A GLOBAL is a deliberate condensation.** Each fragment's frontmatter names
   its `source:` home (e.g. `/praxis/coverage/GLOBAL.md` → `source:
   /praxis/coverage/README.md`). An idea usually lands twice: the full account
   in the README home and a *chosen subset* in the GLOBAL. Reassessment can
   treat the condensation itself as an artifact: is the ambient part still
   faithful to the home?

## The shared table

Preset columns, written as relative patterns so the same header serves any
directory being reassessed. `<dir>` is the concern's domain; `<dir>/<sub>` its
subdomain. If the concern *is* the `<dir>`, sub columns stay empty unless parts
landed in a subdomain; if the concern is a subdomain, the `<dir>` columns record
what the parent home carries (typically an index row or a fragment mention).
Leave any other inapplicable cell empty.

| Idea | `/CONTEXT.md` ⋆ | `codes/` ⋆ | manifest | `<dir>/README.md` | `<dir>/GLOBAL.md` ⋆ | `<dir>/<sub>/README.md` | `<dir>/<sub>/GLOBAL.md` ⋆ | `<dir>/<sub>/<doc>.md` | `design/`, waves | `tool/` |
|---|---|---|---|---|---|---|---|---|---|---|
| *columns* | glossary term | code row (from `codes/*.md` / agent frontmatter) | fragment activation | domain home | domain ambient cue | subdomain home | subdomain ambient cue | case/wave docs under the concern | root or local inquiry waves | implementations |

⋆ = assembled verbatim into `AGENTS.md` (given a manifest entry). Everything
else is maintained knowledge that AGENTS.md at most *links* to.

Commits are cited by jj change ID; author dates in this window are unreliable
(stale working-copy dates — see [the jj guide](/vcs/jj/README.md)), so ranges
below come from `jj op log` sequencing.

## Today — session-day Nov 2 evening → Nov 3 early morning

| Idea | `/CONTEXT.md` ⋆ | `codes/` ⋆ | manifest | `<dir>/README.md` | `<dir>/GLOBAL.md` ⋆ | `<dir>/<sub>/README.md` | `<dir>/<sub>/GLOBAL.md` ⋆ | `<dir>/<sub>/<doc>.md` | `design/`, waves | `tool/` |
|---|---|---|---|---|---|---|---|---|---|---|
| Coverage question → praxis (`#net-new`) | — | — | **added** `praxis/coverage/GLOBAL.md` | `praxis/README.md`: related-practice row | — | `praxis/coverage/README.md`: full practice | `praxis/coverage/GLOBAL.md`: cue + lens table | — | — | — |
| Magnitude measure (`#net-new`) | term `Magnitude` + calibrations | — | — | — | — | — | — | — | — | — |
| `ado/p` short (`#iterative`) | — | `ado` row gains the short | — | — | — | `code/workspace/README.md`: directory row | `code/workspace/GLOBAL.md`: known-places row | — | — | — |
| Known places + `~/r` home + alias codes (`#iterative`) | — | new `a ad d doc ff r s w wiki working` rows | — | `code/README.md` links it | known-places table rewritten | `code/workspace/README.md`: new home | known-places ambient | — | `code/workspace/directories0.gpt61t.md` | `tool/rust/src/codes.rs` (code-file support) |
| Inspectable decision rationale (`#net-new`) | — | — | — | — | `constitution/practice/GLOBAL.md`: durable-writing bullet | `praxis/candidates/README.md`: §Make the lean inspectable | — | — | — | — |
| Terra default → `#hi` (`#iterative`) | — | `t` row updated | — | — | — | — | — | — | — | `tool/rust/src/codes.rs` |
| Random-combo generator + flight (`#net-new`) | — | — | — | — | — | `praxis/flights/random-combo.md` (flight doc); flights README mentions | — | — | — | `tool/random-combo/*` + root entrypoint |
| Condensed variant-support table (`#iterative`) — **uncommitted** | — | — | — | — | `agents/GLOBAL.md`: new table | — | — | — | — | — |
| Ticket housekeeping — **uncommitted** | — | — | — | — | — | — | — | — | — | — (`bd` closures in `.beads/`) |

### Coverage question as a praxis — `48261554` → `0c44f307` → `33f205d2`

Three stages in one concern-home: record the practice, activate it in ambient
assembly, then add presentation lenses. Its parts:

- `praxis/README.md` — index row (quoted whole, it is two lines):
  > [Coverage](/praxis/coverage/README.md) collects the threads in a conversation or pass, says what happened to each, and names important topics left out without converting all of them into commitments.
- [`praxis/coverage/README.md`](/praxis/coverage/README.md) — the home: the
  four-step pass, the fuller lens table, relation to fronts.
- [`agents.manifest.json`](/agents.manifest.json) — `"praxis/coverage/GLOBAL.md"`
  added between ticket-pointers and consumer-accounts. **This line is the
  activation**; without it the fragment exists but never reaches AGENTS.md.
- [`praxis/coverage/GLOBAL.md`](/praxis/coverage/GLOBAL.md) — the ambient part,
  verbatim in AGENTS.md as section `#praxis-coverage` (order 405):

  > When the human asks **“coverage?”**, gather the relevant prompt or conversation's threads, re-present their dispositions, and name important adjacent topics not dealt with in this pass. Offer an unasked-for coverage check when it would help, including semi-periodic check-ins during longer work; use judgment about context and frequency, not every turn. State the examined scope and uncertainty; distinguish suggestions from decisions and results. Do not turn threads into tickets or commitments. Deeper practice: [coverage](/praxis/coverage/README.md).
  >
  > | Cue | Meaning |
  > |---|---|
  > | `#background` | Existing context or current ground. |
  > | `#iterative` | Extension or revision of existing work; not necessarily easy. |
  > | `#net-new` | A capability or approach new to the stated scope. |
  > | `#forward-looking` | A possibility or direction beyond established ground. |
  >
  > These lenses can combine: future-looking work can extend existing mechanisms, and net-new implementation can be ready now. They do not establish proposed, recommended, accepted, commissioned, implemented, or verified standing; say that separately when relevant. A recommendation tag is not authorization. They are not ticket or bookmark tags, runtime classification, or mandatory on every answer.

Note the condensation at work: the README's lens table carries fuller wording
("not a promise of trivial effort", "say what it is new relative to") that the
GLOBAL drops. That difference is itself reassessable.

### Magnitude as a measure of ideas — `651a691a` (+ regen `84692bbc`)

A pure glossary idea: everything lives in [`CONTEXT.md`](/CONTEXT.md), first
entry under *Knowledge, work, and learning*, verbatim in AGENTS.md:

> **Magnitude**
> An informal measure of how strongly an idea shapes and supports its project, top to bottom. High-magnitude ideas can be axes for architecture, gating or vocabulary; mid-magnitude ideas can be rich, valuable subsystems without restructuring the whole project. Human calibration: **8** for bolderfs equivalence-ungate, an unbuilt idea to leave retained readers ungated across a view swap when their reachable bytes are proven identical, while controlling admission for new handles and readers of changed bytes; **6 for now** for Watchment, a rich filesystem observation and state service. These are contextual judgments, not fixed grades. *Do not conflate with:* priority, urgency, effort size or acceptance.

(Links to the two external calibration anchors omitted here; they are
`file:///home/rektide/src/...` citations in the source.) The two calibration
sessions are recorded in CONTEXT.md frontmatter sources.

### `ado/p` conversational short — `3a2c1b91`

The idea touches four places and one symlink outside the repo (`~/ado/p.md`):

- [`codes/ado.md`](/codes/ado.md) — row source; its `description:` frontmatter
  *is* the glossary row. Generated output in AGENTS.md:
  > | `ado` | `/code/workspace/README.md#directory` | ~/ado → ~/r/opencode, the OpenCode notes directory; ado/p is the conversational short for ~/ado/patches.md (p.md symlink). |
- `code/workspace/README.md` — directory row mentions the symlink and short.
- `code/workspace/GLOBAL.md` — known-places row carries it into AGENTS.md:
  > | `~/ado/` | `~/r/opencode/`: OpenCode notes, including `~/ado/patches.md` (conversational short `ado/p`). |
- `codes/GLOSSARY.md` — regenerated (not hand-edited).

### Known places, `~/r` reference home, directory alias codes — `7b15dede` → `c5569c99` → `f4079b48`

The biggest placement fan-out of the day. Parts:

- `code/workspace/directories0.gpt61t.md` — Terra's proposal wave (evidence).
- [`code/workspace/README.md`](/code/workspace/README.md) — **new concern-home**:
  path notation, the verified directory table, and the 2026-11-03 relocation
  account (`~/archive/doc/` → `~/r/`, inode-preserving rename, alias repairs).
- [`code/workspace/GLOBAL.md`](/code/workspace/GLOBAL.md) — ambient rewrite; the
  known-places table now in AGENTS.md section `#workspace-known-places`:
  > **Spell home paths with `~/:** bare `ado/` or `doc/` looks relative to the current directory. `~/r/` is the operator's global reference/wiki space; **rk means Rekon**, the separate `~/src/rekon/` project.
  >
  > | Path / alias | Place and purpose |
  > |---|---|
  > | `~/r/` | Global reference space: broad-topic notes, research and a growing personal wiki. |
  > | `~/ad/`, `~/archive/doc/`, `~/src/doc/` | Compatibility aliases to `~/r/`; use `~/r/` for new references. |
  > | `~/w/` | Resolves to `~/src/opencode-working/`, the OpenCode compose workspace. |
- `codes/` — eleven new code files (`a`, `ad`, `d`, `doc`, `ff`, `r`, `s`, `w`,
  `wiki`, `working`, plus earlier `ado`), each generating a glossary row.
- `constitution/naming/README.md` — rulings home updated (codes practice).
- `tool/rust/src/codes.rs` — generator taught the code-file shape.
- Filesystem (outside the repo): the real `~/r/` move and its compatibility
  symlinks — recorded, not contained, by the above.

### Inspectable decision rationale — `23faa614` → `63b1ee14` → `c925ff33` → `255a1f9a`

- [`praxis/candidates/README.md`](/praxis/candidates/README.md) §*Make the lean
  inspectable* — the full practice (alternatives, tradeoffs, assumptions, lean,
  what would change it; explicitly not chain-of-thought).
- [`constitution/practice/GLOBAL.md`](/constitution/practice/GLOBAL.md) — the
  ambient condensation, verbatim in AGENTS.md under
  `#documentation-practice-durable-writing`:
  > **ChatGPT/OpenAI agents:** when discussion or a durable assessment calls for judgment, make the rationale inspectable: meaningful alternatives, material reasons and tradeoffs, assumptions, your tentative lean and confidence or uncertainty, and evidence or preferences that would change it. Distinguish a recommendation from an accepted decision or commissioned work. Give a concise decision rationale, not private chain-of-thought, hidden scratch reasoning, or routine tool narration; scale detail to the choice. No compulsory research wave or separate document. [Candidates practice](/praxis/candidates/README.md#make-the-lean-inspectable).

No manifest change was needed — the fragment was already active; only its body
grew. AGENTS.md regen `255a1f9a` published it.

### Terra default raised to `#hi` — `b41809c5` + `fa1cb977`

- `agents/t.md` — definition; its frontmatter carries the glossary row now in
  AGENTS.md:
  > | `t` | `/agents/t.md` | GPT-6.1 Terra, default #hi — general coding, analysis, and implementation |
- `agents/models/README.md` — the dated inventory's `t` row.
- `fa1cb977` records the change's provenance (codes frontmatter +
  `tool/rust/src/codes.rs` row-shape follow-through).

### Random-combo generator and reusable flight — `8e4ac0c6` + `3e6677cf`

The one idea of the day with **no ambient presence at all** — deliberate: a
flight is invoked, not ambient. Parts:

- [`praxis/flights/random-combo.md`](/praxis/flights/random-combo.md) — the
  reusable coordination model (frame, bounded spawns, explicit joins,
  independence, terminal dispositions).
- [`tool/random-combo/`](/tool/random-combo/README.md) — Node generator:
  reproducible weighted triples with replacement, 0.8 decay, one-triple-per-line
  rows (`3e6677cf`), tests and example.
- `praxis/flights/README.md` — links the case; `tool/README.md` — toolkit map
  row; root `tool/random-combo.ts` entrypoint.

### Condensed variant-support table — **uncommitted working copy**

`agents/GLOBAL.md` (+17), `agents/models/README.md` (variant table reworked to
rows-are-variants), AGENTS.md regen in the working tree. The committed tip
(`84692bbc`) deliberately excludes it so the tree reproduces from committed
sources; the ambient add, not yet in any commit, is:

> Variant support, condensed (catalog checked 2026-10-31). Rows are variants; **supported by** names providers — chatgpt is the `openai` models (`t`, `ast`, `sol`, `l`), `zai-coding-plan` is `g` and `f` — and **except** lists odd models without that variant. Overrides use full variant names; abbreviations and exact strings: [model lookup](/agents/models/README.md).
>
> | Variant | Supported by | Except |
> | --- | --- | --- |
> | `∅` | chatgpt | astra, sol |
> | `lo` | chatgpt, `zai-coding-plan` | |
> | `med` | chatgpt | |
> | `hi` | chatgpt, `zai-coding-plan` | |
> | `xh` | chatgpt | |
> | `max` | chatgpt, `zai-coding-plan` | |

### Ticket housekeeping — uncommitted `.beads/` export

Closures recorded in `.beads/issues.jsonl` for the Oct 31 fallback ticket
(`rekon-agent-rate-limit-fallback`) and the Oct 22 `rekon-code-versioning`,
`rekon-consumer-accounts` anchors — an export refresh, not new ideas.

## Past week — Oct 26–31, lighter

| Idea (commits) | Presence, compactly |
|---|---|
| Agent flights + model dispatch references (`f859d61a`) | **`/CONTEXT.md` ⋆**: `Flight` glossary term ("A reusable coordination model for a series of agent promises, with explicit spawn and join points…"); **`agents/GLOBAL.md` ⋆**: dispatch bullets gained flight/model-lookup references; new `agents/models/README.md`; homes `praxis/flights/README.md`, rows in `praxis/README.md`, `assess/gates/README.md`, `praxis/parallax/README.md` |
| Rate-limit fallback procedure (`abf56151`, `448d1202`, `589f0146`) | **`agents/GLOBAL.md` ⋆**: "When an agent hits a rate limit or quota exhaustion, use its stated fallback for the task; try the original agent again on each subsequent dispatch so it resumes when quota returns. Fallback is not a permanent switch."; per-agent fallbacks live in `agents/f.md`, `agents/g.md` descriptions; `agents/models/README.md` points to them |
| Variant abbreviations + matrix (`1681b7b3`, `5f996e20`) | **`codes/GLOSSARY.md` ⋆** header: "Agent rows name their configured model as `default #variant` using the standard variant abbreviations: `lo` low, `med` medium, `hi` high, `xh` xhigh, `max` max, `∅` no suffix"; rows across `agents/*.md`; generator `tool/rust/src/codes.rs`; `agents/models/README.md` table (since reworked twice) |
| Trimmed agent descriptions (`5042854e`) | `agents/*.md` bodies only; depth delegated to model lookup — no ambient delta beyond shorter rows |
| Dispatch field visibility (`d05976c7`), subagent catalog + continuation precedence (`96281396`) | `agents/models/README.md` only — inventory knowledge, never ambient |
| Forge publication + ATProto CLI (`13e41ba9`) | **two new fragments** ⋆ via manifest: `forge/GLOBAL.md` ("Default to public repositories, `main`, GitHub `rektide`, and SSH…") and `atproto/GLOBAL.md` ("Check version and identity with `atgc --version` and `atgc auth status`…"); homes `forge/publication.md`, `atproto/cli.md`; wave `design/atproto/cli0.glm53fx.md` |
| Kinefolk firmware flight case (`6eaba8fe`, `7a7a19e9`) | `praxis/flights/kinefolk-firmware0.gpt6a.md` + observed-learning addendum; indexed from `praxis/README.md` and flights README; **no ambient presence** (case, not cue) |
| Terra subagent added (`ec8e4cc4`) | `agents/t.md` definition + `codes/GLOSSARY.md` ⋆ row; AGENTS.md regen |
| Soft 300k delegated-context target (`e1685683`) | **`agents/GLOBAL.md` ⋆**: "Aim to keep delegated sessions under **~300k context tokens**. Modest overshoot to 330–350k is acceptable near a coherent finish…" |
| Followup agent dropped (`77bddf4a`) | `agents/followup.md` → `.md.disabled` (dormant convention); codes row removed; `assess/rekon/lineage0.gpt6a.md` and two design docs updated |
| Beads snapshot + closure interactions (`67225b26`) | `.beads/` exports only |
| Ticket-coordinator ↔ session-mementos relation (`ed25e379`) | note appended in `praxis/ticket-pointers/ticket-pointers0.glm53fx.md` |
| Checkpoint-based eventual integration proposal (`f1921232`) | wave `constitution/practice/eventual-integration0.gpt6a.md` + pointer in `constitution/README.md`; **proposal only** |
| Model routing fix (`1a52411d`) | `agents/{ast,f,g,l,sol,t}.md` + evidence wave `design/agents/model-routing0.glm53m.md`; bug fix, not an idea placement |

Oct 22 (naming rekey, js fold, consumer accounts, versioning) sits just outside
the week window and is already summarized in the codes glossary and fragments.

## Notes for the reassessment

- **Regen/check recipes**: `cargo run --quiet --manifest-path tool/rust/Cargo.toml
  -- combine` and `-- codes` regenerate; both take `--check`. Current committed
  AGENTS.md is fresh against committed sources; the working tree carries the
  uncommitted variant-table regen.
- **Two quotable layers**: AGENTS.md content == fragment body == hand-written
  condensation of a README home. Reassessing "what did we put in ambient" means
  reading GLOBALs; reassessing "what did we decide" means reading the homes.
- **Activation is a manifest line** — coverage needed one; forge/atproto needed
  two; the uncommitted variant table needed none. A fragment without a manifest
  entry is exposure-shaped but inert.
- **Rows are generated**: all `codes/GLOSSARY.md` rows trace to `codes/*.md` or
  `agents/*.md` frontmatter; never edit the glossary in place.
- **Author dates lie in this window**; sequence from `jj op log` (the guide's
  date-surgery note applies).
- Ideas that deliberately have **no ambient part** (random-combo, Kinefolk case,
  model-routing fix, eventual-integration proposal) are no less landed — their
  home is the practice/tool/design layer, and presence there is the correct
  measure for them.

Cross-references: [coverage practice](/praxis/coverage/README.md) (this
map is itself a coverage pass), [assess/rekon orientation](/assess/rekon/README.md),
[assembly contract](/constitution/agents-assembly.md) for the combine mechanism.

## Addendum — same-day follow-ups

Human review of this map redirected three rows, all landed this session:
the inspectable-rationale bullet was reframed as **ambient reviewable thinking
records** (not a judgment-moment trigger); the draft variant-support table was
replaced by a **dispatch translation table** (alias → full variant name →
`model` override base, no lookup needed) in `agents/GLOBAL.md` and
`agents/models/README.md`; and flights gained ambient presence via
`praxis/flights/GLOBAL.md`, with random-combo's generator gaining a `--slots N`
knob (default 3) across code, tests, and contracts.

Assembly itself changed mid-pass (concurrent session
`ses_e6011f662ffehNx613Rkve982p`): the manifest now selects `**/GLOBAL.md` and
`**/GLOSSARY.md` by wildcard with an explicit `exclude` (`doc/GLOBAL.md`) and
`omit_metadata_fields` (`sources`, `refs`), so new GLOBALs are discovered
without manifest edits and the emitted context carries no source metadata
blocks. Bookmarking and the flights depth pair (parallax, evidence gates) were
activated under that mechanism the same day.

## Second addendum — no-GLOBAL audit

Review of items with no ambient exposure, with leans (2026-11-03):

| Item | State | Lean |
|---|---|---|
| `praxis/fronts` | three CONTEXT terms link to it | no GLOBAL — vocabulary is its ambient form |
| `praxis/candidates` | practice-GLOBAL bullet (thinking records) | no GLOBAL — the bullet is its cue |
| `praxis/parallax`, `assess/gates` | named in the flights cue | no GLOBAL — depth pair, reached via flights |
| `digest`, `assess`, `remake`, `evidence` | sense-map sentence / glossary pointers | no GLOBAL — adequate |
| waves and case stories (Kinefolk, model-routing, eventual-integration) | none | correct — evidence, not ambient material |
| `terminal/` | topic home + `js.md` depth, no GLOBAL | no GLOBAL — on-demand subject home; a cross-ref from `code/js` could aid discovery when TUI work starts |
| `provider/` | one model-config JSON | no GLOBAL — config artifact, not knowledge |
| `prompt/` | empty directory | nothing to expose; cleanup is a separate human decision |
| `upstream/` | practice home (reference mirrors), no GLOBAL | no GLOBAL — situation-triggered; a one-line pointer from `code/GLOBAL.md` is the candidate if porting work recurs |
| `web/` | platform-first guide, no GLOBAL | no GLOBAL — on-demand guide; revisit when web work next happens |

Sharp edge of wildcard selection worth watching: any future `GLOSSARY.md`
anywhere in the bundle now auto-assembles (only `codes/GLOSSARY.md` exists
today), and a stray `GLOBAL.md` in a wave or vendor tree would go ambient
unintended. `doc/GLOBAL.md` is the one deliberate exclusion.
