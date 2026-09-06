---
type: Praxis
title: Assessment — become well equipped to judge and act
description: A pilot home for architectural review and evidence-gated deliberation, exercised against opencode-lastact.
resource: /assess/README.md
tags: [assessment, review, praxis, waves, evidence]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
sources:
  - { resource: "urn:opencode:session:ses_f872bf58affeS5IBlC38cA6iYX", title: Human request for assessment, completion gates, lineage, and a ticket-backed wiki }
  - { resource: /AGENTS.md, title: Architectural stewardship and independent waves }
  - { resource: /constitution/README.md, title: Documentation constitution pilot }
  - { resource: /constitution/reassessment0.gpt6a.md, title: Coming up for air }
---

<a id="assess"></a>
# Assessment

**Become well equipped for the next decision, then make it deliberately.**
Assessment is not synonymous with fault-finding, checklist compliance, or a
license to rewrite. It connects what a thing is trying to accomplish with how
it is built, what evidence supports it, and what should happen next. A system
that works well deserves an explanation of why and what to preserve.

A **praxis** is a repeatable posture joining purpose, procedure, evidence, and
reflection. Rules constrain individual moves; a praxis helps choose and relate
the moves. This is a working definition for this pilot, not an adopted universal
taxonomy for Rekon.

## Two initial practices

- [Review form 1](/assess/review/form1.glm53.md) — the current reusable brief
  for both high-level architecture assessment and close code review, revised
  from the Lastact cohort's reflections. Load it when a review should explain
  the system, not merely inspect a diff. [Form 0](/assess/review/form0.gpt6a.md)
  stays frozen as the protocol that experiment actually used.
- [Evidence gates](/assess/gates/README.md) — decide in advance what must become
  available before comparison, judgment, or action. Load it when work depends
  on multiple agents, investigations, or unresolved contracts.

The [Lastact case study](/assess/case-studies/lastact/wave0.gpt6a.md) tests both:
independent reviewers use one form and one baseline, the parent writes its own
review before reading theirs, and comparison waits for the runnable cohort to
resolve. An unserved Terra request remains explicit.

## Results and workshop

The latest direction is [high/low passes](/assess/strategies/high-low0.gpt6a.md):
different agents should pursue different aim points, including expansive
possibilities and close evidence, with effort allocated to high-interest
fronts. This challenges the predecessor experiment's overly uniform restraint;
it is a strategy draft, not another dispatched wave.

Start with [Did our review prompt serve the ask?](/assess/review/prompt-assessment0.gpt6a.md):
the complete-cohort parent self-assessment of prompting, alignment, and
orchestration. The reviews were useful, but the prompt underweighted its own
evaluation. The [ambient candidate](/assess/review/ambient-candidate0.gpt6a.md)
keeps suggested AGENTS language outside the active instruction file while that
discussion remains open. No issue or product mutations were made by this wave.

The [Rekon research results](/assess/rekon/README.md) connect the two September 6
sessions and route through the tooling map **with its verification corrections**.
They also supply a second prompting lesson: explicit evidence labels do not by
themselves prevent an overbroad capability or absence claim.

## Working boundaries

`assess/` owns the method and its evaluation, not every project's findings or
every kind of work coordination. Case studies may live here when the method
itself is being tested. Product owners retain code, accepted product guidance,
and product tickets. Source evidence is not moved merely to fit this layout.

The [Rekon inquiry brief](/assess/rekon/brief0.gpt6a.md) holds the broader
situation: this session follows another stewardship session, and asks how
documentation, capabilities, session provenance, and work history can become a
navigable whole. It is a research boundary, not an authorization to build that
whole.

## Cross-references

- [Constitution](/constitution/README.md#doc-constitution-references-evidence)
  **constrains** evidence honesty: source inspection, inference, recommendation,
  and acceptance are distinct.
- [Coming up for air](/constitution/reassessment0.gpt6a.md#connections)
  **motivates** assessment: attention is part of correctness and signals must
  not inherit promises they cannot establish.
- [Modularization](/design/rekon/agents-modularization0.sol56x.md#agents-maintenance-skillify-two-transformations)
  **constrains** adoption: a local practice home is not a production AGENTS
  assembly cutover or permission to remove existing ambient obligations.
- [Review handling](/code/review-handling.md) **owns** the next responsibility:
  responding to feedback and committing corrections. Assessment does not silently
  enter that mode while evidence is still arriving.
