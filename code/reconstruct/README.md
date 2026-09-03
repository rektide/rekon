---
type: Guide
title: Reconstruct From The Present
description: Guidance for rebuilding software from current goals and constraints without letting historical implementation paths dictate the new design.
resource: /code/reconstruct/README.md
tags: [code, reconstruction, rework, redesign, migration, carriers, agents]
status: draft
generated: { by: model:openai-gpt-5.6-sol, at: 2026-09-03 }
stale_after: 2027-03-03
sources:
  - id: human-direction
    title: Break historical path-dependence when reconstructing and distinguish clean rework from carry optimization
    author: human:rektide
    last_modified: 2026-09-03
  - id: documentation-constitution
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution
    author: project:rekon
  - id: phase-a-recovery
    resource: /doc/opencode-session.md
    title: Phase A recovery guide
    author: project:rekon
  - id: phase3-calibration
    resource: /doc/phase3.md
    title: Phase 3 calibration against original intent
    author: project:rekon
  - id: graph-caution
    resource: /doc/fancy-graph.md
    title: Explicit stepping stone before general graph machinery
    author: project:rekon
  - id: watchman-carrier
    resource: file:///home/rektide/src/opencode-watchman/.design/watchman/carrier0.gpt56s.md
    title: Watchman clean-slate replacement carrier
    author: model:openai-gpt-5.6-sol
    last_modified: 2026-09-03
---

<a id="reconstruct"></a>

# Reconstruct From The Present

Reconstruction is not replay.

When a user asks to reconstruct, rebuild, recreate, consolidate, carry, or
rework old software, the historical implementation is evidence. It is not the
default architecture, the default sequence, or the default definition of the
job. The first responsibility is to recover the present ask and design the
best destination from the present baseline.

The past can establish requirements, incidents, compatibility obligations,
and hard-won negative knowledge. It does not give every old mechanism a right
to survive.

The compact rule is:

> **Recover the intent. Recheck the constraints. Redesign the destination.
> Use history as evidence, not gravity.**

<a id="reconstruct-scope"></a>

## Scope

Use this guide when historical code, commits, sessions, plans, or architecture
are inputs to substantial implementation work. It applies when an agent must
decide what the software should become, not merely recover bytes.

It does not turn routine requests to compile, regenerate artifacts, restore a
known file, rerun a build, or reproduce an exact state into architecture
exercises. Exact restoration is a valid reconstruction mode; it is simply a
different job from clean rework.

<a id="reconstruct-problem"></a>

## The Path-Dependence Failure

Agents commonly turn a reconstruction request into a historical compression
exercise:

1. Read the old commits in order.
2. Treat every mature-looking mechanism as a settled requirement.
3. Map old commits into fewer new commits.
4. Preserve temporary interfaces because later work depended on them.
5. Rebuild the historical solution more neatly.

This may produce a cleaner patch stack while preserving the wrong design. It
answers "how can the old path be replayed more tidily?" before answering "what
should exist now?"

A carrier is one delivery strategy. Carriability is one optimization axis. It
must not silently replace maintainability, simplicity, correctness, module
depth, operational clarity, or the user's actual goal.

<a id="reconstruct-job"></a>

## Determine The Actual Job

The word "reconstruct" is ambiguous. Classify the requested outcome before
planning implementation.

| Mode                     | Primary objective                                                            | Authority of history                                           | Normal baseline          | Typical result                                    |
| ------------------------ | ---------------------------------------------------------------------------- | -------------------------------------------------------------- | ------------------------ | ------------------------------------------------- |
| Exact recovery           | Restore a lost or damaged state                                              | High, including accidental details when fidelity requires them | Historical state         | Faithful reconstruction                           |
| Behavior-preserving port | Preserve an observable contract on a newer platform or upstream              | High for behavior, low for mechanisms                          | Current target platform  | Adapted implementation                            |
| Carrier rebuild          | Preserve selected downstream behavior while reducing future carry cost       | High for accepted behavior and ownership constraints           | Current upstream         | Compact, separable patch stack                    |
| Clean rework             | Produce a clearer, deeper, more maintainable solution to the current problem | Evidence only; every mechanism is reopened                     | Current codebase         | Redesigned implementation                         |
| Replacement              | Satisfy the underlying need by a different approach                          | Low except for required outcomes and compatibility             | Current system and needs | New solution, possibly deleting the old subsystem |

Do not infer "carrier rebuild" merely because branches, commits, or upstream
drift appear in the evidence. Do not infer "exact recovery" merely because the
user supplied a detailed historical plan.

Treat even a polished prior plan as a source document unless the user
explicitly asks for literal execution. Its detail records prior reasoning; it
does not settle the present reconstruction mode or exempt its mechanisms from
review.

When the mode is unclear and the distinction would materially change the work,
ask a short multiple-choice question:

> Which outcomes matter here: exact historical recovery, preservation of
> behavior on the current base, minimum future carry cost, a cleaner redesign,
> or freedom to replace the approach entirely?

Several answers may apply. State which objective wins when they conflict.

<a id="reconstruct-default"></a>

## Default Posture

Unless exact recovery or compatibility preservation is explicit, use this
posture:

- Start from the current intended baseline, usually current upstream.
- Describe the desired outcome without naming historical mechanisms.
- Revalidate old claims against current code and current dependencies.
- Preserve only constraints with present evidence or explicit human authority.
- Design the destination before designing its commit sequence.
- Build one production implementation.
- Verify outcomes through current interfaces rather than textual parity with
  the old tree.

If persistent data, shipped behavior, external consumers, wire compatibility,
or an active migration creates a real compatibility obligation, name it. Do
not invent compatibility code from general caution.

<a id="reconstruct-from-top"></a>

# Reassess From The Top

<a id="reconstruct-present-brief"></a>

## 1. Write A Present-Tense Brief

Before discussing old modules or commits, write a short account containing:

- The user-visible or system-visible outcome.
- The current failure, friction, or missing capability.
- The actors and consumers that matter now.
- The correctness and operational properties that must hold.
- Known compatibility, deployment, and ownership constraints.
- Evidence still needed.

Test the brief by removing historical proper nouns. If it cannot explain the
job without names such as an old manager, fallback, cursor, bridge, phase, or
commit ID, it is probably describing the path rather than the ask.

<a id="reconstruct-current-baseline"></a>

## 2. Inspect The Current Baseline First

Read current upstream or the current target code before deciding what to port.
Establish:

- Which problem still exists.
- Which old problem upstream has already solved.
- Which seams and consumers exist now.
- Which old assumptions no longer hold.
- Whether a competing or deeper current design has appeared.
- Which tests and tooling are authoritative today.

Pin the exact baseline used for design and implementation. A moving bookmark
is not a reproducible base. Refresh deliberately after the first complete,
green implementation rather than chasing upstream continuously.

<a id="reconstruct-mine-history"></a>

## 3. Mine History By Claim, Not Chronology

Read old commits, branches, incidents, reviews, and tests as a quarry. Extract
claims into a small ledger instead of retelling the commit sequence.

| Claim            | Evidence                                                          | Present status                                            | Consequence                        |
| ---------------- | ----------------------------------------------------------------- | --------------------------------------------------------- | ---------------------------------- |
| Required outcome | Human decision, external contract, or accepted specification      | Accepted, rejected, or unresolved                         | What the new system must do        |
| Failure evidence | Reproduction, incident, trace, or focused test                    | Still reproducible or obsolete                            | What must be prevented or observed |
| Constraint       | Protocol, platform, persisted data, deployment, or ownership fact | Current or superseded                                     | What limits the design space       |
| Mechanism        | Old module, state, callback, flag, queue, retry, or workaround    | Rejustify or discard                                      | Candidate implementation only      |
| Test             | Assertion from the old line                                       | Contract, regression evidence, or implementation coupling | Port, rewrite, or delete           |

Chronology explains why people made a choice. It does not prove the choice is
still appropriate.

<a id="reconstruct-authority"></a>

## Evidence Has Unequal Authority

Usually preserve:

- Explicit current human decisions.
- Shipped or externally consumed behavior.
- Persisted-data and wire-format obligations.
- Reproduced incidents and failure boundaries.
- Measured platform or dependency behavior.
- Tests that state observable outcomes through a stable interface.
- Negative knowledge about approaches already shown unsafe.

Usually reopen:

- Module and file boundaries.
- Internal interfaces and callback shapes.
- State-machine decomposition.
- Retry ownership and lifecycle placement.
- Names inherited from superseded models.
- Configuration knobs and diagnostic surfaces.
- Commit count, ordering, and titles.
- Temporary proof scaffolding.
- Tests coupled to private state or incidental event counts.

Usually omit unless newly justified:

- Transitional compatibility paths that were never shipped.
- Add-then-delete history.
- Rejected fallback or migration machinery.
- Historical metrics retained only because they already exist.
- Whole-file transplants into changed upstream-owned files.
- Exact tree parity as a substitute for behavioral proof.
- Documentation that duplicates history already preserved elsewhere.

<a id="reconstruct-counterfactuals"></a>

## 4. Break Historical Gravity With Counterfactuals

Ask these questions before accepting an inherited mechanism:

1. If the old implementation vanished but its verified requirements remained,
   would we invent this mechanism today?
2. Which present consumer requires this interface?
3. Which current constraint makes this state or option necessary?
4. If this module were deleted, would complexity disappear or merely spread
   into callers?
5. Does this test protect an outcome, or only the old implementation shape?
6. Are we retaining two implementations because differential execution proves
   something, or because the old line feels authoritative?
7. Does the proposed final state already contain a workaround scheduled for a
   later migration?
8. Would the proposed commit ladder still make sense if the old commit history
   were unavailable?

"It existed before" is provenance, not justification.

<a id="reconstruct-destination"></a>

# Design The Destination

<a id="reconstruct-invariants"></a>

## Separate Invariants From Tactics

Freeze outcomes only as strongly as their evidence warrants.

An invariant might say:

> After continuity loss, consumers are explicitly told that current subtree
> state is unknown and can converge by rescanning.

A tactic might say:

> Deliver a fake update at the watched root through a private ready callback.

The first constrains correctness. The second is one historical encoding and
must compete with cleaner alternatives.

The recurring posture is **same goal, different mechanism**: preserve the
validated outcome while allowing the current architecture to supply a better
way to achieve it.

Label plan statements as one of:

- **Required outcome**: acceptance depends on it.
- **Current constraint**: evidence presently limits alternatives.
- **Design decision**: selected after comparing current options.
- **Implementation hypothesis**: expected to evolve during construction.
- **Historical evidence**: informative but not normative.

Do not call tactics "frozen end state" merely because reviews converged on
them under an older baseline.

<a id="reconstruct-design-again"></a>

## Design Again Before Mapping Old Code

For each consequential seam, produce at least one design from current
requirements without consulting the old implementation shape. For uncertain or
high-risk seams, design it twice and compare:

- Interface depth and caller burden.
- Ownership of state, failure, retries, and cleanup.
- Locality of future changes.
- Testability through the same interface callers use.
- Compatibility obligations.
- Operational observability.
- Carry cost, if carry cost is actually a goal.
- Deletion of historical machinery.

Only after selecting the destination should old code be mapped onto it. Reuse
algorithms, protocol knowledge, fixtures, and proven edge cases where they fit.
Do not let reusable code select the architecture.

Freedom from accidental historical structure is not permission for a wholesale
or speculative rewrite. Prefer the smallest deep design that satisfies the
present ask. Add general machinery only when current variation or current
consumers justify it.

<a id="reconstruct-one-implementation"></a>

## Prefer One Production Implementation

The default reconstruction has one production line built from the current
baseline. Keep the old line immutable and inspect it read-only.

Use a second implementation only when running both implementations through the
same black-box harness is itself necessary evidence. Merely implementing the
same design twice and comparing final files is weak proof: both can be wrong,
and each integration can fail differently.

Temporary diagnostics can live in a disposable child or overlay on the actual
new implementation. Do not rebuild the old architecture solely to retain
instrumentation that the final design rejects.

<a id="reconstruct-pitch"></a>

# Pitch The Rework

The proposal should lead with the destination, not the excavation diary.
When reassessment supports a cleaner rework, recommend it directly. Name the
departure from the historical plan, explain what it improves, and distinguish
required decisions from optional refinements. Do not silently smuggle in a
redesign, but do not hide the better destination behind deference to a detailed
old plan either.

<a id="reconstruct-pitch-order"></a>

## Recommended Pitch Order

1. **Present ask.** State the outcome, current baseline, and primary
   optimization objective.
2. **Proposed destination.** Explain the final modules, interfaces, ownership,
   and runtime behavior in present tense.
3. **Why this destination.** Compare the important alternatives and name the
   tradeoffs deliberately accepted.
4. **What history contributes.** Cite verified requirements, incidents,
   constraints, fixtures, and negative knowledge.
5. **What does not survive.** Name obsolete mechanisms, temporary interfaces,
   and accidental behavior being dropped.
6. **Landing structure.** Present dependency-ordered, green slices derived from
   the destination architecture.
7. **Proof.** Map required outcomes and failure boundaries to executable tests,
   live gates, and operational evidence.
8. **Open decisions.** Expose unresolved choices that can materially change the
   architecture or scope.
9. **Historical appendix.** Put commit folds, lineage details, and archaeology
   after the proposal rather than making them its spine.

<a id="reconstruct-pitch-language"></a>

## Prefer Destination Language

Path-dependent pitch:

> Replay the old stack, preserve its metrics while replacing recovery, then
> rebuild the verified tree into twelve commits corresponding to the old work.

From-the-top pitch:

> Build one supervised watcher from current upstream. It emits exact changes
> during continuity and typed invalidations after continuity loss. Historical
> incidents define the failure matrix; fallback, cursor, and temporary metrics
> mechanisms are not part of the new design.

The second pitch can still use historical evidence. It does not make history
the architecture.

<a id="reconstruct-landing"></a>

# Plan The Landing

<a id="reconstruct-commit-slices"></a>

## Derive Commits From The New Dependency Graph

Commit boundaries should follow the final modules and their dependencies, not
the order in which the old system happened to evolve.

Good reconstruction commits are:

- Green and behaviorally coherent.
- Final-form for the module they introduce.
- Reviewable through an explicit interface and focused tests.
- Separable where ownership, upstreamability, or rollback differs.
- Free of scaffolding introduced only so a later commit can delete it.
- Flexible in count and title as current code reveals better boundaries.

Separate reusable or upstreamable foundations from downstream-only adapters and
thin composition changes when that separation is real. Do not manufacture
seams solely to make the history look tidy.

<a id="reconstruct-branching"></a>

## Keep Branch Mechanics Subordinate

For a clean rework:

1. Preserve the old tip under an immutable reference.
2. Pin the full current baseline revision.
3. Create a physically clean workspace for the new implementation.
4. Run baseline installation, checks, and tests before editing.
5. Keep construction references separate from accepted feature references.
6. Move the accepted bookmark once, after verification.
7. Freshen onto newer upstream deliberately after the implementation works.

Do not clean, rebase, or replay the old branch unless the selected work mode
actually requires it. Historical contamination does not need surgery when the
new line does not descend from it.

<a id="reconstruct-validation"></a>

## Prove Outcomes, Not Resemblance

Validation should answer whether the new system satisfies the present ask.

Prefer:

- Black-box scenarios shared across old and new systems where behavior should
  remain equivalent.
- Contract tests at the new module interface.
- Deterministic failure and concurrency schedules.
- Live tests for external process, filesystem, protocol, and deployment facts.
- State-convergence assertions rather than incidental event counts.
- Negative checks proving rejected mechanisms and options are absent.
- Full current-package and integration checks before promotion.

Treat historical tests individually. Port contract tests, rewrite useful
regressions against the new interface, and delete tests whose only purpose was
to protect discarded implementation detail.

Textual parity with an independently rewritten proof tree is not acceptance.

<a id="reconstruct-stop"></a>

# Stop And Reassess When

Pause implementation and return to the user or design when:

- The requested reconstruction mode remains ambiguous.
- A compatibility requirement may involve persisted data or external users.
- Current upstream has introduced a competing solution or moved the seam.
- A supposedly required mechanism has no present consumer or evidence.
- The plan requires implementing rejected machinery before deleting it.
- A supporting feature introduces a second unresolved event or ownership
  architecture.
- An external deployment gate has no known owner or may be unnecessary under
  the selected design.
- The implementation cannot explain its destination without replaying the old
  chronology.

Do not force progress through a semantic collision merely because a detailed
historical plan exists.

<a id="reconstruct-agent-output"></a>

# Agent Output Template

Use this compact structure when proposing a reconstruction:

```markdown
# <Present-tense outcome>

## Actual ask

<What should be true, for whom, on which current baseline?>

## Reconstruction mode

<Exact recovery | behavior-preserving port | carrier | clean rework | replacement>
<Primary optimization and secondary constraints>

## Current system

<Relevant current modules, consumers, and upstream changes>

## Evidence retained

| Claim | Source | Current consequence |

## Destination design

<Modules, interfaces, ownership, lifecycle, and failure behavior>

## Deliberate omissions

<Historical mechanisms and accidental behavior not carried>

## Alternatives

<Meaningful options and why this one wins>

## Landing graph

<Dependency-ordered green slices; no fixed count unless the count matters>

## Verification

<Outcome-to-test and failure-to-test mapping, including live gates>

## Open decisions

<Only decisions that can still alter scope or architecture>

## Historical appendix

<Commit map and archaeology, if useful>
```

<a id="reconstruct-checklist"></a>

# Reconstruction Checklist

Before presenting a plan, verify:

- [ ] The reconstruction mode is explicit.
- [ ] The primary optimization objective came from the user rather than the
      available artifacts.
- [ ] The ask is stated without relying on historical mechanism names.
- [ ] Current upstream or the intended target baseline was inspected first.
- [ ] Outcomes, constraints, mechanisms, and historical evidence are labeled
      separately.
- [ ] Every retained mechanism has a present justification.
- [ ] Compatibility code is tied to persisted data, shipped behavior, external
      consumers, or an explicit requirement.
- [ ] At least one destination design was produced independently of the old
      file and commit layout.
- [ ] The proposed interfaces reduce caller knowledge rather than preserving
      historical plumbing.
- [ ] One production implementation is sufficient, or the need for two is
      proved by the validation strategy.
- [ ] Commits follow the new dependency graph and remain green.
- [ ] The plan does not introduce machinery solely for later deletion.
- [ ] Historical tests were classified rather than copied wholesale.
- [ ] Acceptance tests prove outcomes and failure boundaries, not tree parity.
- [ ] Branch surgery is limited to work the selected mode truly requires.
- [ ] The pitch leads with the destination and leaves chronology in supporting
      material.

<a id="reconstruct-cross-references"></a>

# Cross-References

- [`/constitution/README.md`](/constitution/README.md#L318-L331) explains why
  promotion is synthesis rather than movement: current guidance should
  reconcile design evidence instead of copying one historical artifact by
  default.
- [`/doc/opencode-session.md`](/doc/opencode-session.md#L589-L591) contains the
  direct precursor to this guide: understand what was done and why, then
  implement it fresh against the current codebase. Its [later reconstruction
  procedure](/doc/opencode-session.md#L1326-L1357) ends with the concise rule
  "adapt, don't copy."
- [`/doc/phase3.md`](/doc/phase3.md#L18-L32) is a worked example of calibrating
  current reality against original intent and accepting "same goal, different
  mechanism." Its [rejected legacy adapter](/doc/phase3.md#L66-L87) also shows
  that a migration can fail by preserving the bridge it was meant to
  eliminate.
- [`/doc/fancy-graph.md`](/doc/fancy-graph.md#L79-L105) supplies the balancing
  caution: escaping old structure does not justify premature general machinery
  when a smaller explicit design can prove the needed architecture.
- [Watchman clean-slate replacement carrier](file:///home/rektide/src/opencode-watchman/.design/watchman/carrier0.gpt56s.md)
  is the motivating example. Its requirements and failure research are strong;
  its historical compression, fixed commit ladder, and dual implementation
  strategy illustrate the path-dependence this guide is intended to break.
