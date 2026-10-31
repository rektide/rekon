---
type: Praxis
title: Flights — compose agent work with explicit spawn and join
description: A reusable model for coordinating a series of agent promises, their dependencies, evidence joins, and human decisions.
resource: /praxis/flights/README.md
status: draft
generated: { by: "model:openai/gpt-6.1-terra#xhigh", at: 2026-10-31 }
tags: [praxis, flights, agents, coordination, evidence, synthesis]
sources:
  - { resource: "opencode://ses_e6c900557ffex5YjEpZrQCueNC", title: Connect and update the Kinefolk firmware case through observed flight completion }
  - { resource: "opencode://ses_e6ccb4e5effe6EWqmToxh2oLHO", title: Human commission for flight vocabulary, full-cycle practice and model reference }
  - { resource: "opencode://ses_e6cc6c15bffe34JypBfFJ1p6L1", title: Rekon entrypoint investigation informing the proposed practice home }
  - { resource: "opencode://ses_e6cc34212ffelIc5sLrW1xqvi7", title: Develop flight dependency model and optional full-cycle example }
  - { resource: /praxis/parallax/README.md, title: Independent viewpoints, synthesis and parent architectural responsibility }
  - { resource: /assess/gates/README.md, title: Evidence gates and partial-result release conditions }
  - { resource: /constitution/practice/GLOBAL.md, title: Independent waves and durable provenance }
  - { resource: /praxis/flights/kinefolk-firmware0.gpt6a.md, title: Completed firmware research flight and observed learning }
---

# Flights

A **flight** is a reusable coordination model for a series of agent promises:
explicitly spawn bounded work, join the results required for a dependent step,
and carry the evidence and decisions forward. A particular flight is an actual
instantiation with its own participants, inputs, joins, and outcomes. This is
human-requested draft practice; recording the term does not require a further
approval, and the practice does not require a flight for every task.

A promise here is a conceptual commitment to a result or a terminal disposition,
not a language-level Promise, a guarantee of success, or a claim that a scheduler
exists. A join states which promises matter to a subsequent decision and what
releases it. It is not necessarily a join of every task in the flight. The parent
coordinates using the actual harness and owns interpretation and architecture;
the human retains decisions and authorizations reserved to them.

## Shape a flight

1. **Frame:** state the question, intended decision, shared baseline, constraints,
   and what the human has authorized. Choose the smallest useful cohort and joins.
2. **Spawn:** assign each agent a bounded question, owned artifact, relevant
   sources/revisions, permitted and forbidden peer reading, and return receipts.
   Preserve independence where comparison needs it; shared context is not a leak.
3. **Join:** name the dependent action, required artifacts and participants,
   release condition, and independent work permitted while waiting. Record
   arrivals without prematurely interpreting results when an independent cohort
   is still acquiring evidence. A terminal failure is not a successful artifact.
4. **Read and carry forward:** after the required arrival, inspect the actual
   evidence for sufficiency, compare positions, and record disagreement, missing
   evidence, assumptions, and the next decision. Spawn subsequent work with
   explicit inputs and its own independence boundary.
5. **Close or continue:** preserve artifacts and provenance, report what was learned
   and what remains blocked, and distinguish a proposed next action from one the
   human has authorized. A checkpoint or agent agreement is not acceptance.

The [evidence-gate practice](/assess/gates/README.md) owns detailed arrival,
readiness, and release rules. Do not busy-wait: use completion notifications and,
if no independent work remains, yield with the gate closed. A join can be
proportional to a small dependency or protect a whole explicitly requested cohort.

## Example: a full research-to-design cycle

This is a selectable example, not a universal pipeline or a mandate to invoke
these models. It assumes the human has commissioned the indicated participants
and comparative work; actual availability, costs, and useful cohort size matter.

1. **Independent research:** spawn a research cohort against a common question
   and baseline, with unique outputs and no reading same-wave findings. Join all
   required successful research artifacts before the comparative step.
2. **Independent syntheses:** after the research join, spawn independent **T, A,
   and G** syntheses over the same identified research corpus. Each may read that
   corpus, but not its sibling syntheses until their join. Ask each to distinguish
   shared findings, distinctive insights, conflicts, and evidence gaps.
3. **Resynthesis:** after the synthesis join, compare the independent syntheses
   and their underlying research; reconcile where evidence permits, preserve live
   alternatives and disagreements, and record the exact inputs and resulting brief.
   This is not an automatic averaging or a decision by model vote.
4. **Independent designs:** spawn **A max, T xhigh, and G** designs against the
   same resynthesis and constraints; keep sibling designs independent until their
   own join. Compare the designs against original evidence and human intent, with
   architectural responsibility remaining with the parent. Seek human direction
   when selection or implementation crosses the authorized boundary.

Here T means Terra, A Astra, and G GLM; their configured agents and exact requested
variants are in the [model lookup](/agents/models/README.md). Names in this example do not
authorize a model override or establish that those agents are always available.
Later evidence or a changed brief may call for another join, a focused experiment,
or an addendum rather than silently changing a prior comparison.

## Motivating case: firmware research

The [Kinefolk firmware flight](/praxis/flights/kinefolk-firmware0.gpt6a.md)
maps a commissioned sequence from two orientations through a shared QMK/ZMK
study, three independent Kinefolk-fit inquiries, five syntheses, and a returning
agent's reconsideration. Its all-of joins make the differing cohorts and
continuation identities visible. The completed case records actual corrections,
parent learning and limits; Kinefolk's brief maintains the dispatch and research.
Completion does not establish runtime conformance or general practice success.

## Failure and partial results

Record each requested slot's disposition, exact artifact and revision when present,
and the consequence of a failure, interruption, or provider limit. Retry or replace
only within the authorized scope; never describe a missing participant as assessed.
If a required successful result is absent, the join stays closed unless the human
authorizes a partial release or a changed cohort. Label the narrower evidence base
and preserve resumable inputs and independence constraints. A late result prompts
an explicit addendum or new synthesis; it does not retroactively make an earlier
partial comparison complete. Independent work can proceed where its prerequisites
really are met, without laundering an assumption into a completed join.

## Relationships and learning

- [Parallax](/praxis/parallax/README.md) explains why independent and complementary
  views reveal different evidence; flights make their sequencing and joins explicit.
- [Evidence gates](/assess/gates/README.md) protect particular decisions; a flight
  can contain several gates without turning them into a universal scheduler.
- [Independent waves](/constitution/practice/GLOBAL.md#waves) preserve authorship,
  artifacts, and comparison boundaries; a wave can be one stage of a flight.
- [Case stories](/evidence/README.md#evidence-case-stories) capture whether the
  arrangement exposed consequential evidence or merely produced more artifacts.
