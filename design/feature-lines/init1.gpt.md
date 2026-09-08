---
type: DirectionAndPassPlan
title: Regather, carry intent, and rebuild the shared context
description: "Revised starting point after the feature-line run-off and human steering: required GLOBAL glossaries, candidates praxis, tickets as attention memory, and candidate cross-topic documentation passes."
resource: /design/feature-lines/init1.gpt.md
tags: [rekon, constitution, praxis, candidates, intents, glossary, global, regather]
status: draft
generated: { by: "model:gpt (exact model identifier unavailable)", at: 2026-09-08 }
extensions:
  session: urn:opencode:session:ses_f953b45fbffe4sDfLun8QQ4HOZ
  work_anchors: [rekon-agents-maintenance-skillify, rekon-agents-maintenance-engineering, rekon-agents-maintenance-workspace]
  attention_reminders: [rekon-intents, rekon-phoenix-regather]
  posture: Recorded human direction plus proposed pass choices; no production cutover or blanket acceptance of source drafts.
sources:
  - { resource: "urn:opencode:session:ses_f953b45fbffe4sDfLun8QQ4HOZ", title: Human direction following the two documentation assessments }
  - { resource: /design/feature-lines/review0-parent.gpt.md, title: Parent session-informed assessment }
  - { resource: /design/feature-lines/review0.gpt6a.md, title: Independent Astra comparison and recovered earlier work }
  - { resource: /design/rekon/context-weave/README.md, title: Existing context workshop and coarse ownership lanes }
  - { resource: /CONTEXT.md, title: Shared working vocabulary }
  - { resource: /constitution/README.md, title: Maintained pilot and context contract }
  - { resource: /praxis/candidates/README.md, title: Candidates praxis commissioned in this conversation }
  - { resource: /constitution/naming/exploration/proposal4.glm53m.md, title: Recorded naming ruling and promise-language lineage }
---

# Regather, carry intent, and rebuild

## What changed

Rekon has grown from a largely single ambient manuscript into a much wider
home for ideas, knowledge, practices, and operational experience. That growth
is welcome. **Part of praxis is now to regather and focus: decide what to bring
upward into the shared core, then do the documentation work.** Expansion and
regathering are complementary movements, not good and bad alternatives.

The [first map](/design/feature-lines/init0.glm53f.md) was a poor starting point
in several respects. The [parent assessment](/design/feature-lines/review0-parent.gpt.md)
and [Astra comparison](/design/feature-lines/review0.gpt6a.md) preserved useful
corrections and interpretations. This `init1` is the new starting point after
human steering, not another general audit of the conversation's mistakes.
Its location retains the feature-line inquiry's history; its scope now includes
the shared core and the next cross-topic passes.

The human has limited attention and has not reviewed all the accumulated work.
**That fact is not an unresolved technical mystery.** More summaries, indexes,
or agent agreement do not substitute for the missing human review. Preparation
can be offloaded; acceptance cannot be inferred from the preparation.

## Direction on record, versus choices still open

| Human direction now established | Not established by it |
|---|---|
| A glossary is a **required element of `GLOBAL.md`**, grounded in constitution/core as well as praxis. | Every existing fragment is accepted, or all of `CONTEXT.md` must be injected everywhere. |
| Develop **candidates praxis**; too often the human is given only one thing. | Every task needs multiple models, every comparison needs one winner, or variants of a preselected answer are sufficient. |
| Tickets are the human's **todo/attention log for intents they do not want to lose**. | Every idea or captured prompt automatically becomes a task, promise, or implementation mandate. |
| Regathering and focusing belong in praxis; selected understanding should be brought upward into the core. | The whole corpus should be centralized, deduplicated, or reorganized before useful work can proceed. |
| Identify a bounded set of top-level topics; make passes through their READMEs and GLOBALs; culminate in a rebuild. | A new tooling platform, production AGENTS replacement, or blanket removal of old guidance is authorized now. |
| Openintent gets a separate README-driven idea and a plugin-index entry. | Rekon's documentation must wait for the plugin to exist. |

This document's `draft` status applies to its account and proposed execution
choices; it does not demote the explicit glossary requirement back to a mere
workshop possibility.

## Glossary-first is part of the core contract

Every GLOBAL should introduce the terms its operating guidance relies on.
The glossary belongs before the instructions that use it. Shared terms can
link to [the working glossary](/CONTEXT.md); local terms should explain their
scope and the distinctions needed here. A few meaningful definitions can be
enough. A link saying “see glossary elsewhere” is not the whole local element.

The point is to provide a shared understanding of the work, not a dictionary
as an appendix. Examples and counterexamples belong where a short synonym
would hide the consequential difference. `Prime` versus forge default is one
case; candidate versus selected work, intent versus commitment, and outcome
versus permission are others.

The upcoming core pass must incorporate this required element into the
constitution's GLOBAL contract, not leave it solely in a praxis note or this
design file. Candidate GLOBALs remain candidates until reviewed; assembly and
actual loading remain separately identifiable events. The existing README-led
homes and SKILL symlinks need not be moved to make this work.

<a id="intents"></a>
## Intents: tickets keep attention available

The human's statement is the center:

> writing tickets for intent is the best practice in general for things i dont
> want to lose attention on: we should capture with a ticket. that is my todo log.

This is more than a work-tracker command recipe. A ticket can preserve a wanted
possibility, an unfinished question, or work to return to before its exact
implementation is known. It should retain enough of the human's intent to be
recognizable later, with links to its native discussion and candidates. It can
be deliberately deferred without becoming rejected or forgotten.

The next workspace/core pass should express that practice without making ticket
capture burdensome. Keep native work ownership and the existing append-notes
discipline; do not maintain a second lifecycle in a Markdown task list. Distinguish
“do not lose this” from “build this now.”

An `intents/` topic is a serious candidate. The human connects it to the much-loved
Mark Burgess body of thought, already important in the naming corpus. A broader
account should connect intention, voluntary promise, request, obligation, choice,
and fulfillment without equating them—or reducing them to JavaScript Promises.
That research and final placement are **tabled**, recorded in `rekon-intents`.
The practical ticket-attention meaning need not wait for a comprehensive theory.

## Candidates and regathering

[Candidates praxis](/praxis/candidates/README.md) is the companion new document
from this commission. It develops meaningful differences, gives them a fair
hearing, supports independent or cooperative comparisons, and preserves options
without requiring a single winner. Recommendations are useful; manufacturing
three labels around one assumed solution is not.

Regathering asks what these explorations now equip us to understand or do. It
does not invalidate the expansion. It lets us focus by preserving alternatives
and selecting a few consequential changes for the core and maintained homes.

Rekon also deserves a situated note on its own accumulated work, but that is
not a new general failure investigation to perform now. `rekon-phoenix-regather`
records the deferred, offloadable preparation: gather a human-selected set,
surface candidate focuses, and leave the actual review decision with the human.

## Candidate organizations for the next passes

These are real program alternatives, not an already-dispatched wave. They use
the existing maintenance work rather than establish another umbrella epic.

| Candidate | How the work proceeds | Why it is attractive | Cost or tension |
|---|---|---|---|
| **Shared core, then domain passes** | First develop common language and the glossary contract; then revise selected practice and operational homes; rebuild the combined view. | Cross-topic terms have a usable center before several writers rely on them. | The core can become an abstract bottleneck if it tries to settle every domain first. |
| **Domain candidates, then regather** | Several top-level homes produce README/GLOBAL candidates from their concrete experience; compare what deserves shared standing, then rebuild. | Keeps the core grounded in actual practices and preserves distinctive voices. | Requires a deliberate comparative pass; otherwise it repeats the uncontrolled accumulation. |
| **Encounters across the homes** | Follow selected encounters—keeping an intent, choosing among candidates, resuming a feature line—through all relevant READMEs/GLOBALs, then rebuild. | Tests useful understanding while changing the docs, rather than merely their organization. | Can leave untouched areas uneven; needs explicit coverage accounting rather than pretending to be a whole-corpus pass. |

My recommendation is **a light shared-core pass followed by domain candidates**:
record the settled distinctions, not an exhaustive ontology; then give the
domains room to produce different good expressions before regathering. The
encounter-led candidate remains useful as a way to read the resulting view.
The human has not selected this sequence merely by inviting candidates.

## Proposed topic set and existing work homes

| Pass area | README/GLOBAL surfaces to examine together | Concrete result | Existing work home |
|---|---|---|---|
| **Shared meaning and core** | Root orientation, `CONTEXT.md`, `constitution/`, its naming sources, and `praxis/` | Required GLOBAL glossary contract; clear roles for candidates, intents, and regathering; an intelligible common opening. Preserve recorded naming decisions. | `rekon-agents-maintenance-skillify` |
| **Ways of working and judging** | `praxis/` including candidates, `assess/`, `digest/`, `remake/`, `evidence/` | Purposeful README accounts and glossary-bearing GLOBAL candidates, with different practice promises still visible; not five restatements of one procedure. | `rekon-agents-maintenance-skillify`, related to `rekon-phoenix` |
| **Operational continuity** | `code/`, `vcs/jj/`, and the selected language/tool references in `js/` and `atproto/` | Align instructions with their domain explanations and examples; integrate feature-line distinctions without reviving the default-branch conflation. | `rekon-agents-maintenance-engineering` |
| **Attention and work environment** | Work-tracking/source-location guidance, `upstream/`, and relevant `forge/` material; consider an `intents/` home through candidates rather than assume it | Tickets as intentional attention memory; distinguish evidence, desired work, and authorized action; route actual mechanics to their homes. | `rekon-agents-maintenance-workspace`; broader `rekon-intents` |

This is a proposed starting set, not “all top-level directories.” Each selected
home needs an explicit README/GLOBAL disposition. A topic with no ambient
contribution should say so rather than grow an empty GLOBAL; every GLOBAL that
is written or retained needs its glossary. The goal is useful coverage, not a
file-count target.

For each pass: read the existing account and load-bearing sources; identify
the meaning to retain or change; write meaningful candidates where the shape
is open; compare; make a proposed maintained revision; check links and term
relationships; commit coherent stages. Capture unresolved pursuits in the
native ticket when the human wants to keep attention on them. Do not keep
rewriting the intent indefinitely instead of producing the candidate passages.

## Culmination: rebuild a view we can read

Regather the selected README/GLOBAL work into a new shared view with a short
receipt: which sources/versions were used, what stayed ambient, what changed,
which disagreements remain, and what still awaits review. Read it as a whole
for purpose, language, repetition, and behavior—not just as concatenated files.

The work is **prose-first and can be manual**. Existing assembly experiments
are not a reason to start a tooling project, reopen completed assembler work,
or make current plugin capabilities a prerequisite. Rebuilding the workshop
view and replacing a live `AGENTS.md` are different actions. The latter still
requires an explicit cutover decision with coverage and activation consequences
visible. Untouched parts of the live manuscript must not disappear.

## Openintent remains a separate aside

[Openintent's README](file:///home/rektide/src/openintent/README.md), committed
as `5443708f`, captures the plugin idea: prompts in sessions and to subagents
become potential promises that can be composed in JavaScript; a gate can be
attached after dispatch but before protected release. Outcome completion and
permission to flow are separate. `Promise.all()` needs substantive conditions,
not just two arrivals. Three integration candidates preserve the difference
between explicit consumers and native host-wide delivery control.

The idea is logged in [the plugin index](file:///home/rektide/archive/doc/opencode/plugins.md)
as on-deck, README-first, unimplemented and uninstalled. Its relationship to
candidates/gates/intents is useful; its implementation is not the current Rekon
work and must not take over this pass program.

## What stays tabled

- Whole-corpus archaeology, general behavioral benchmarks, and the backlog of
  human review: prepare a selected slice later under `rekon-phoenix-regather`.
- A broad Burgess/intent-theory investigation and final `intents/` architecture:
  preserve under `rekon-intents`, while using the practical ticket principle now.
- Directory-wide migration, a constitution/praxis merger, new registries,
  automated gate enforcement, and live context installation.
- Resolving every criticism in the earlier assessments before starting useful
  candidate passages. The assessments are inputs, not another compulsory backlog.

The next work decision is which candidate pass organization and first topic set
to take up. The present delivery is this revised direction, the candidates
praxis, the separate Openintent README/index entry, and durable attention
records—not a claim that the broad passes or rebuild have happened.

## Cross-references and corrections

- [Astra's map corrections](/design/feature-lines/review0.gpt6a.md#map-corrections)
  **limits use of `init0`**: prime was conflated with forge default, installation
  overstated, and earlier feature-line work missed. Preserve the historical map
  but use the corrected basis for new work.
- [Existing context workshop](/design/rekon/context-weave/README.md) **owns the
  preparatory migration and three coarse lanes**. This direction deepens that
  work; it does not replace the program or its unfinished human decisions.
- [Earlier bookmark proposal](/.test-agent/dated-variant-bookmarks/README.md)
  **is recovered local-only prior art**, still discussion-gated. Its existence
  disproves the old absence claim; it does not make its proposals accepted.
- [Naming override](/constitution/naming/exploration/proposal4.glm53m.md)
  **preserves human authority and location identity** while new core vocabulary
  develops. No `/core/` relocation or hidden identifier scheme is implied.
- [Candidates praxis](/praxis/candidates/README.md) **puts the new preference
  into a usable draft now**, rather than leaving it only as another future task.
