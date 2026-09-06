---
type: Praxis
title: Evidence gates — wait until well equipped
description: Declare the evidence needed for a dependent decision and keep acquisition, synthesis, and action distinct.
resource: /assess/gates/README.md
tags: [gates, praxis, subagents, waves, readiness]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
sources:
  - { resource: /AGENTS.md, title: Deliberative subagents and architectural checkpoints }
  - { resource: /constitution/README.md, title: Dependency-gated dispatch and independent waves }
  - { resource: "urn:opencode:session:ses_f872bf58affeS5IBlC38cA6iYX", title: Human direction to gate analysis on all returning }
---

<a id="gates"></a>
# Wait until well equipped

The point of a gate is not waiting. It is **avoiding a dependent judgment with
an impoverished evidence set**. Fast partial results can anchor the comparison,
cause avoidable rework, or contaminate supposedly independent authors.

## Declare before dispatch

Name the decision the gate protects, required inputs, named participants,
baseline, owned artifacts, allowed independent work, and the release condition.
Distinguish two common gates:

| Gate | What it protects | Release condition |
|---|---|---|
| Complete wave | Independent comparison needs the whole declared cohort. | All required participants have terminal dispositions and every required successful artifact is available; exceptions are explicitly recorded and authorized. |
| Dependency readiness | Later work needs particular facts/contracts, not every concurrent task. | Those specific prerequisites are available, read, and sufficient for the named decision. |

For a requested **all-results wave**, do not quietly use a quorum, early
consensus, or “enough similar answers” instead. A runnable subset must identify
unavailable requested participants; it is not the complete requested cohort.

## While closed

- Authors can reason within their own assignments. The coordinator can do its
  own independent review or genuinely unrelated work. The gate does not prohibit
  thinking; it prohibits *dependent* analysis and action before its inputs exist.
- Do not read peer findings into an independent review, summarize early results
  as the emerging conclusion, steer other reviewers with them, or start repairs.
- Ask returns to carry receipts, not findings: session, artifact, checks, and
  blockers. Record arrival administratively without interpreting the results.
- Use completion notifications. Do not busy-wait, sleep/poll, or repeatedly query
  agents for progress. If no independent work remains, yield the turn explicitly
  with the gate closed; do not invent useful-looking work to fill the interval.

## Release deliberately

There are two steps: **arrival** (inputs exist) and **readiness** (the coordinator
has read them and can explain what they establish and leave unresolved). Only
then begin comparison, reaction, or a change wave. Reading the entire corpus
before comparing is not an incremental synthesis of the first result.

Record participants and artifacts, failures/unavailable slots, scope changes,
and the decision to release. A failure is not an empty successful review. Retry
or replace within the authorized scope; otherwise request a waiver or retain the
blocked scope. Do not claim an unfilled requested model was assessed. Late
evidence after an authorized partial release requires an explicit addendum or
new synthesis, not a silent rewrite of the earlier evidence boundary.

Urgent safety/operational intervention can interrupt a wave. Say why, preserve
the existing artifacts, and re-pin affected inputs before resuming. A gate
cannot compel ignoring a real hazard; ordinary impatience is not such a hazard.

## Proportionality and ownership

A two-agent check may need only a short declared gate. A comparative research
wave benefits from an artifact manifest. Gates should make the next decision
legible, not create a universal scheduler or a mandatory document for every
task. If acquisition uncovers a new prerequisite, explicitly extend the gate;
do not pretend it was always part of the original experiment.

The coordinator owns sufficiency and next-slice authorization. Agent completion,
passing tests, model agreement, and human acceptance are different signals.

## Cross-references

- [Review form](/assess/review/form1.glm53.md) **uses** a full-wave gate to protect
  independent assessment and subsequent comparative reading.
- [Constitution's opt-in patterns](/constitution/README.md#doc-constitution-patterns)
  **provides prior art**: assignment manifests, wave seals, dependency gates,
  and proportional adoption already belong to this knowledge system.
- [Architectural stewardship](/AGENTS.md#architectural-stewardship) **owns**
  consequential decision checkpoints after evidence becomes available.
