---
type: Praxis
title: Post-build — gather the possibilities a working system reveals
description: Use a completed vertical to find architectural leverage, strengthen shared foundations, and choose promising new capabilities without turning exploration into automatic implementation.
resource: /praxis/post-build/README.md
tags: [praxis, post-build, architecture, possibilities, stewardship]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
sources:
  - { resource: /praxis/post-build/colorlu/brief0.gpt6a.md, title: Human-commissioned first application of this practice }
  - { resource: /assess/strategies/high-low0.gpt6a.md, title: Expansive possibilities grounded in close evidence }
  - { resource: /digest/README.md, title: Present-state and intent reading }
  - { resource: /AGENTS.md#architectural-stewardship, title: Parent judgment and consequential checkpoints }
---

# Post-build: what can we do from here?

**Having just done the thing, use the new vantage point.** A working vertical
exposes real paths through a system: which abstractions paid off, what callers
still had to coordinate, where promises weakened between modules, and which
larger capabilities have become plausible. Gather those possibilities before
automatically implementing the next feature or adding another library module.

The architectural win is not necessarily less code. It is **more useful behavior
under stronger, more local guarantees**, with less knowledge required of the
next caller—and a clearer route to capabilities that were previously awkward.

This is a draft, readable practice, not an installed skill, CLI command, or
mandatory gate after every change. Its first case is the
[colorlu / luf-settings / luffect inquiry](/praxis/post-build/colorlu/brief0.gpt6a.md).
The human explicitly requested the home `~/src/rekon/praxis/`; this directory
does not relocate or supersede Rekon's other practice homes.

## The posture

Hold three questions together:

1. **What now works, and why?** Preserve real strengths and distinguish the
   scope of the demonstration from the scope of the public promise.
2. **Where can one investment help systematically?** Find repeated caller
   obligations, missing shared guarantees, and foundations that make several
   workflows simpler or safer.
3. **What new offering is now within reach?** Explore alternatives and futures
   generously; state their assumptions and costs. Do not reduce the inquiry to
   either defect repair or immediately demanded methods.

Constrain unauthorized action, not the scale of thought. A possibility map is
not an accepted roadmap. A working prototype is not proof of deployability.
An upstream filename is not proof of matching semantics.

## When this is useful

- A first application has exercised a previously library-only stack.
- A cross-repository feature works, but its glue is acquiring special rules.
- Several implementation waves have landed and the next useful direction is
  less obvious than the next available task.
- A port or platform adaptation needs to reconcile upstream fidelity with
  concrete host experience.

Do not wait for perfection. Known limitations can be part of the evidence, so
long as the report does not relabel partial operation as successful cutover.

## A way to proceed

### 1. Capture the vantage point

Record the human's direction, the completed outcome, source/dependency revisions,
what was tested, and what remains unproved. Read the build checkpoint, then read
the consequential code. Treat author-reported checks as reports; rerun only to
answer a particular unresolved question.

Separate build arrival from readiness to judge. Where another agent is still
building the relevant slice, wait for that evidence rather than assess its
intended implementation as if it already existed.

### 2. Follow obligations, not directory names

Trace one successful path and its important counterpaths: failure, interruption,
replacement, stale completion, invalid input, repeated execution, and host exit.
Ask what a caller must know to get each right.

For each obligation, name:

| question | useful answer |
|---|---|
| Who holds the authority or resource? | The owner, not merely the last function called |
| When does a promise become true? | Admission, completion, verification, commit, or acknowledgment |
| What can invalidate it? | A new request, failed validation, disposal, external edit, or other explicit condition |
| What must survive invalidation? | Already-completed facts, acquired-resource cleanup, or a deliberately entered commit |
| Where can we vary the behavior? | The actual seam and its adapters, not hypothetical configurability |

These distinctions often matter more than missing helper functions. Cancellation
is not rollback; a queue is not a host-safety guarantee; semantic equality is not
byte equality; atomic replacement is not a transaction.

### 3. Gather several kinds of architectural opportunity

- **Concentrate a guarantee:** move an obligation to the lowest owner that can
  actually uphold it. A storage integrity obligation needs no second caller to
  justify repair.
- **Deepen a module:** hide recurring coordination behind a small interface
  without erasing differences callers need to express.
- **Separate meanings:** undo an abstraction that conflates requests with
  resolved state, events with snapshots, or general path operations with
  project identity policy.
- **Complete an existing foundation:** improve a real semantic substrate rather
  than append isolated surface methods that each reinvent its machinery.
- **Enable a larger offering:** articulate what becomes possible once those
  guarantees compose—another host, another consumer, inspectable changes,
  previews, exports, or stronger automation.

Paving a cowpath is not copying every workaround into a shared library. Some
paths exist only because a lower module fails its promise. Some encode domain
policy and should stay local. Some suggest a different road entirely.

### 4. Check the proposed substrate and its alternatives

For upstream-inspired work, trace **public symbol → implementation → dependency
machinery → tests** at a pinned revision. Follow a type-only interface to its
implementors before declaring there is no runtime capability. Distinguish:

- semantic fidelity;
- interface omissions or adaptations;
- host-specific adapters;
- product policy;
- accidental drift.

Compare extracting today's mechanism, restoring an upstream contract, designing
a better seam, and deliberately leaving a local solution alone. Preserving an
option is different from implementing it. Preserve important policy differences
within shared machinery rather than standardizing them away.

### 5. Make picks, not a compulsory inventory

For each leading front, explain **the payoff, owner, caller obligations removed,
credible future applications, cost, evidence, and next proof**. Choose a small
number to recommend in an order that explains its dependencies. Include at least
one broader possibility worth keeping open, and say what would change the order.

The next proof might be a counterexample, a paired-adapter contract, an upstream
semantic fixture, a host-turn trace, a packaging experiment, or a user decision.
Avoid invented time estimates and numerical rankings that merely hide judgment.

### 6. Preserve the learning and return the steering

Commit the inquiry, evidence, and assessment as coherent stages. Keep previous
claims visible when new evidence corrects them. Link significant discoveries
to their native code/design owners; do not move product ownership into Rekon.

End with the few architectural wins now worth attention, the unresolved tensions,
and what decision the assessment equips. Implementation, live cutover, ticket
creation, and publication each retain their own authorization.

## Agent use and evidence

The parent retains architectural responsibility. Delegate bounded investigations
or verification where they can sharpen judgment, not the entire destination.
A high pass can open possibilities while a close pass establishes their ground;
these are complementary questions, not independent votes.

Respect the operator's requested serial or parallel rhythm. In a serial inquiry,
one evidence assignment can finish before synthesis chooses the next. Passing
the previous suite repeatedly is not a substitute for examining whether it
tests the newly claimed contract.

## How to know the practice helped

The report should make a consequential next choice easier. Did it expose a
better shared guarantee, recover an overlooked upstream capability, distinguish
two meanings an abstraction had conflated, or identify a compelling new use?
Record the reader's response and subsequent evidence when available.

Writing a report alone does not establish that a proposed architecture was
accepted, that a shared module removed the expected work, or that a future
offering proved valuable.

## Cross-references

- [Digest](/digest/README.md) provides the living state and intent fronts. This
  practice asks what that state makes possible after a consequential build.
- [Assessment](/assess/README.md) owns broader judgment about fitness and evidence;
  [high/low passes](/assess/strategies/high-low0.gpt6a.md) prevents possibility
  gathering from shrinking into a defect checklist.
- [Remake](/remake/README.md) asks how the work should have been built and landed;
  post-build asks which foundations and offerings to pursue from the position
  actually reached. It need not reconstruct a hypothetical past sequence.
- [Evidence gates](/assess/gates/README.md) separates prerequisite arrival,
  sufficient reading, synthesis, and authorization.
- [Evidence and case stories](/evidence/README.md) supplies the distinction between
  what help was expected, what happened, and what the practice learned.
