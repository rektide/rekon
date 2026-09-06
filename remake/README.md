---
type: Praxis
title: Remake — a defensible sequence of architectural growth
description: Reconsider how a developed system should be built and landed, then specify the architectural assessment each consequential intermediate state deserves.
resource: /remake/README.md
tags: [remake, architecture, sequencing, assessment, reconstruction]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
extensions:
  ticket: rekon-phoenix
sources:
  - { resource: /remake/design/brief0.gpt6a.md, title: Human commission and Rekon scope }
  - { resource: /remake/research/precedents0.gpt56s.md, title: Existing practice ownership and missing stage-relative assessment }
  - { resource: /code/task-reconstruction/philosophy.md, title: Reconstruct from the present }
  - { resource: /assess/review/form1.glm53.md, title: Architectural and close review }
---

<a id="remake"></a>
# Remake

**Given what we now understand, how should this system come into being—and
what should earn our confidence at each step?**

Remake turns a developed project's architecture, intent, and accumulated
learning into a proposed sequence of coherent changes. Its primary output is
a defensible **build-and-landing order**. Its companion is an assessment of
what each consequential intermediate state would establish, put at risk, or
leave deliberately unfinished.

It is not a tidier retelling of the commit log. Nor does a remake commission
automatically authorize a rewrite, history surgery, or a new architecture.
The default action here is **analysis and proposal**. A successful result may
preserve the existing architecture and most of its order.

Start with the [sequence and checkpoint form](/remake/sequence/form0.gpt6a.md).
The [bounded event-reader example](/remake/examples/event-reader0.gpt6a.md)
shows the distinction between a proposed gate and a verified patch prefix.
These are draft practice artifacts; no real project has yet been remade with
this form.

<a id="remake-order"></a>
## “Right order” is an architectural argument

Pin the system studied, the proposed landing base, the intended destination,
and the audience. State which goal leads: early feasibility, coherent consumer
capability, maintainer review, independent ownership, or some combination.
There is usually a partial order, not one uniquely correct numbered list.

Separate four things that history tends to mix:

| Order | Question |
|---|---|
| Historical | What actually happened, and what was known then? |
| Learning | Which uncertainty should we resolve before committing to a design? |
| Construction and landing | Which coherent capabilities can be introduced on which predecessors? |
| Assessment | Where do new responsibilities, promises, or consequences demand judgment? |

A host-contract probe may need to happen first without becoming a production
patch. A correction discovered late may belong with its capability in a
proposed new series. Several small commits may share one architectural
checkpoint. These are correspondences to explain, not graphs to force into
one shape.

Hindsight is useful here. It is not evidence that the original builders should
have possessed later knowledge. Preserve the actual history, including its
failed approaches and reasoning.

<a id="remake-method"></a>
## Derive the series

1. **Recover the purpose and current shape.** Explain consumers, boundaries,
   ownership, external contracts, strengths, and meaningful constraints. A
   [digest](/digest/README.md) may provide this account. Read consequential
   claims against their owning sources rather than treating recent plans as
   implemented truth.
2. **Name the destination being sequenced.** Is this the present accepted
   capability set, a behavior-preserving port, or a separately proposed design?
   Use [reconstruction's work modes](/code/task-reconstruction/philosophy.md#reconstruct-job)
   when relevant. If a better architecture emerges, expose the departure and
   seek its decision; do not smuggle redesign into patch ordering.
3. **Choose coherent capability increments.** Each should introduce something
   understandable and assessable through a boundary. Avoid both an enormous
   foundation dump and artificial file-sized steps. Tests and documentation
   belong with the capability they establish, not in a final credibility patch.
4. **Draw and explain the dependencies.** Distinguish code/contract prerequisites,
   evidence needed to select a design, integration/release gates, and review
   preferences. Only actual prerequisites constrain the landing graph; an
   attractive reading order is not automatically a runtime dependency.
5. **Compare meaningful alternatives.** For example, an early consumer tracer
   may reveal a bad interface sooner, while owner-aligned foundations may be
   easier to land upstream. Prefer the order that best serves the declared
   goal without violating contracts. Explain what would change that preference.
6. **Map the old work onto the proposed series.** A many-to-many source map
   records retained behavior, regrouped corrections, historical scaffolding,
   deliberate omissions, and genuinely new proposals. Account for every
   in-scope capability; do not silently discard a public promise because it
   complicates the new story.

If the graph cycles, investigate the coupled responsibilities or the chosen
granularity. Do not invent an abstraction merely to make the diagram acyclic.
For multiple repositories, keep owner-specific series and name the compatible
revision rendezvous rather than pretending there is one atomic commit train.

<a id="remake-assessment"></a>
## Assess each state, not just the final tree

Each checkpoint asks two questions:

1. **Is this state sound for the capability it claims now?**
2. **Does it move coherently toward the selected destination?**

The first prevents lending it future code or future evidence. The second
prevents locally tidy patches from accumulating the wrong architecture.
Deliberately absent future capabilities are not defects; obligations activated
by the current patch cannot be deferred merely because the system is unfinished.
For example, accepting untrusted bytes activates input-safety concerns; owning
a queue activates resource and overflow concerns; introducing a background task
activates lifecycle and shutdown concerns.

Attach a few dominant concerns to the stage where they first matter. Explain:

- what becomes possible, and which promises remain unavailable;
- which interfaces and state owners settle, and why that placement is good;
- what future changes become easier or harder, including the cost of preserved
  options and any temporary coupling;
- which invariant is now active, its counterexample, and the evidence needed;
- what subsequent boundary change would reopen the judgment.

Use [assessment's close-review lenses](/assess/review/form1.glm53.md) when the
consequence warrants them. Do not commission a repetitive whole-system review
for every tiny patch. Keep the series-level concerns visible at later integration
points: individually good boundaries do not prove good composition.

<a id="remake-evidence"></a>
## A proposed prefix has not passed its tests

Label candidate states **proposed** until materialized. Existing tests on the
studied tip can support a design argument; they do not certify a differently
ordered intermediate tree. Record actual checked revisions, commands, host
bases, and results separately from planned checks and source-derived reasoning.

The proposed acceptance bar is: every landed prefix builds and meets its own
declared contract; invariants hold from their activation onward; the final
composition satisfies the destination. If that has not been exercised, say so.
Explicitly limited internal foundations can be valid prefixes; misleading
public facades and unsafe supposedly temporary behavior are not made valid by
their place in a plan.

Implementation, if later authorized, proceeds on a preserved, separately named
line with these checkpoints. [Reconstruction mechanics](/code/task-reconstruction/README.md)
and [VCS guidance](/vcs/jj/README.md) own the actual preservation and history
operations. This practice does not grant permission for them.

<a id="remake-learning"></a>
## Judge the remake too

Did the proposed order reduce uncertainty or review burden? Did it recover a
forgotten obligation? Did its neat sequence hide an integration dependency?
Did it help anyone land work—or only generate another plan?

Capture expected and actual usefulness in an [evidence case story](/evidence/README.md#evidence-case-stories).
Until a series is tried, report analytical benefits as analytical benefits.
Do not manufacture a success story from an unexecuted proposal.

## Cross-references

- [Reconstruct from the present](/code/task-reconstruction/philosophy.md#reconstruct-landing)
  **provides the foundation**: destination-derived, coherent slices and history
  as evidence. Remake extends it with explicit intermediate-state assessment.
- [Assessment](/assess/README.md) **owns judgment and evidence discipline**;
  [review handling](/code/review-handling.md) owns responses to accepted feedback.
- [Precedent research](/remake/research/precedents0.gpt56s.md) **documents the
  boundary analysis** read before this definition was finalized.
- [NAN–Lastact association](/evidence/case-stories/nan-lastact/genesis0.gpt6a.md)
  **records the genesis and limits** of this practice-definition run.
- [Phoenix inquiry](/design/rekon/phoenix0.gpt6a.md) **opens a broader question**:
  what understanding must survive replacing an implementation? Remake itself
  does not establish that replacement is safe.
