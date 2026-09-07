---
type: Workshop
title: Context weave — shared meaning, focused ownership
description: The manual glossary-first context workshop and bounded, human-steered inventory for extracting Rekon's accumulated AGENTS guidance into domain homes.
resource: /design/rekon/context-weave/README.md
tags: [context, constitution, naming, global, inventory, honing]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
extensions:
  ticket: rekon-agents-maintenance
  related_work: [rekon-phoenix, rekon-con-naming, rekon-agents-maintenance-skillify]
sources:
  - { resource: /design/rekon/context-weave/north-star0.gpt6a.md, title: Prior naming and constitutional basis }
  - { resource: /design/rekon/context-weave/inventory0.gpt56s.md, title: Complete source inventory }
  - { resource: /constitution/README.md#doc-constitution-honing, title: Human-authorized honing posture }
---

# Context weave

**Read the whole context before deciding what to excise. Give its parts homes
without confusing relocation, simplification, and removal from ambient use.**

The work is anchored by `rekon-agents-maintenance`, related to `rekon-phoenix`
and `rekon-con-naming`. The existing epic is reused, not replaced by another
cleanup program. Current scope is a manual workshop and steering inventory;
the human has not authorized automatic deletion of the old guidance.

## Read in this order

1. [North-star basis](/design/rekon/context-weave/north-star0.gpt6a.md) — what
   the pre-today naming/constitution rounds establish, what remains contested,
   and how the current request extends them.
2. [Shared glossary](/CONTEXT.md) — working terms, not a new name registry or
   a decision to replace the earlier naming grammar.
3. [Manual assembled view](/design/rekon/context-weave/assembly0.gpt56s.md) —
   root purpose and action map, candidate GLOBALs, then the retained AGENTS
   manuscript. Read its receipt for source basis and any inherited limitations;
   the view is not live guidance.
4. [Complete inventory](/design/rekon/context-weave/inventory0.gpt56s.md) —
   every source line, likely homes, overlaps, candidate stale wording, and
   alternate coarse grouping options.

The [root README](/README.md#rekon-sense-map),
[constitution GLOBAL](/constitution/GLOBAL.md), and
[code GLOBAL](/code/GLOBAL.md) intentionally repeat important terms and
operating meaning while honing. The existing README/SKILL symlinks remain;
no production manifest or active AGENTS replacement was created.

## Five tickets, including the existing epic

The human asked for fewer than six tickets. The chosen staging uses **the
existing epic plus four children**, one of which is already completed tooling.
Only two tickets were added. Current lifecycle lives in Beads, not this table.

| Ticket | Role and source coverage | Likely homes to discuss |
|---|---|---|
| `rekon-agents-maintenance` | Owns the complete migration/steering contract and the small work map. | This workshop and the existing migration design. |
| `rekon-agents-maintenance-skillify` | Existing coordinator plus **knowledge/collaboration**: lines 1–20 and 27–104; **98 lines**. | `constitution/`, naming, `assess/`, and a human-selected collaboration/agent home. |
| `rekon-agents-maintenance-engineering` | **Engineering**: lines 118–252 and 362–365; **139 lines**. | Existing `code/`, `js/`, `vcs/jj/`; proposed focused Rust home. |
| `rekon-agents-maintenance-workspace` | **Workspace and work tracking**: lines 21–26, 105–117, 253–361, 366–368; **131 lines**. | A workspace/runtime home plus focused Beads depth; not one undifferentiated operations file. |
| `rekon-agents-maintenance-assembler` | Existing completed deterministic assembler capability. Not another extraction lane. | [Assembly reference](/constitution/agents-assembly.md) and its implementation. |

The three source lanes cover **368/368 lines with no overlap** at the inventory's
AGENTS SHA-256 `4bf6260ebf26db5e011affca5c8a1cffae4a9a1270db702b253d6a8f93d7f1bf`.
All eight fenced source examples are included. Ticket grain is intentionally
larger than directory grain: the engineering ticket is not a proposal to merge
Rust, JavaScript, and VCS into one flat directory.

This selects [inventory option A](/design/rekon/context-weave/inventory0.gpt56s.md)
for staging: use existing homes, then fill missing homes. Its weakness is that
the workspace lane joins environment discovery and Beads, two distinct vocabularies.
They may be steered separately inside that ticket. The inventory also offers a
pure coordinator plus two larger domain passes, or knowledge/work/implementation
lifecycle groupings. The human can change the grouping without losing the ledger.

## The passes within a chosen lane

The human chooses a lane and subsection. Then:

1. **Owner pass:** compare old guidance with its candidate home. Decide whether
   the home owns the concept or merely happens to repeat it.
2. **Meaning pass:** identify current intent, duplication, contradictions,
   version-sensitive claims, and examples whose scope has been lost.
3. **Disposition pass:** propose each passage as retained ambient, rehomed but
   still ambient, routed depth, rewritten with a reason, or deliberately retired.
   “Old” and “already repeated elsewhere” are not removal decisions by themselves.
4. **Human checkpoint:** select a coherent edit and its intended behavioral
   effect. The ticket records the choice and unresolved pieces; no new ticket
   is required for each heading.
5. **Selected change and proof:** preserve the predecessor, make only the chosen
   move or correction, check incoming references and content parity, and test
   routing before claiming an obligation can safely leave ambient context.

The especially plausible first candidates include already-owned JS recipes,
duplicated code/commit prose, and the specialized Beads recovery recipes. These
are choices to bring to the human, not an instruction to remove all three now.
Keep costly-to-miss guards such as scope protection, explicit push authorization,
stable work IDs, and `bd prime` prohibition visible while routing is unsettled.

## Source-time changes worth knowing

The inventory faithfully describes its inspected snapshot. Since that reading,
the parent preserved the glossary at `cab80d7e` and revised candidate exposures
at `198c302d`: `code/GLOBAL.md` now has fragment metadata, and both current
GLOBALs begin with vocabulary. Thus the inventory's earlier “code fragment lacks
metadata” observation is historical, not a new defect in the current tree.
Its **AGENTS line ledger remains applicable** because this session has not edited
the live manuscript.

The inspected AGENTS body includes the operator's pre-existing uncommitted
push-policy change. It was neither reverted nor committed as this session's
own work. A retained copy in the workshop does not promote the manuscript's
other ambiguous passages into new policy.

## How we will know this helped

First: can the human see the whole context, recognize the earlier north star,
and select a bounded extraction without losing track of the rest?

Later: does an agent reach the right depth before needing it, retain the intended
guardrails, and spend less effort reconstructing vocabulary? A smaller byte
count or a valid manifest does not establish those outcomes. Preserve surprises
as [case stories](/evidence/README.md#evidence-case-stories).

For a separate project trial, [NAN restart briefs](/evidence/case-stories/nan-lastact/resume0.gpt6a.md)
distinguish the already-tested Burn B code from the not-yet-tried digest/remake
practices and the future hwsim work.
