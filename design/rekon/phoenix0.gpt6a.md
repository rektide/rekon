---
type: ResearchDiscussion
title: Rekon as Phoenix — preserve understanding, reopen its expressions
description: A selective reading of Chad Fowler's Phoenix Architecture and an exploratory direction for Rekon's action practices, evidence relations, and replaceable mechanisms.
resource: /design/rekon/phoenix0.gpt6a.md
tags: [rekon, phoenix, regeneration, evidence, provenance, intent, architecture]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
extensions:
  ticket: rekon-phoenix
  session: urn:opencode:session:ses_f86f757c0ffe5G5aHU26e2evOL
  basis: Eight essays and the series index read directly on 2026-09-06; conceptual synthesis, not validation of regenerative tooling
sources:
  - { resource: "https://chadfowler.com/regenerative-software/", title: Chad Fowler — The Phoenix Architecture }
  - { resource: /constitution/reassessment0.gpt6a.md, title: Coming up for air }
  - { resource: /code/task-reconstruction/philosophy.md, title: Reconstruct from the present }
  - { resource: /assess/review/prompt-assessment0.gpt6a.md, title: Did our review prompt serve the ask? }
  - { resource: /rekon.ts, title: Current CLI command surface }
---

<a id="rekon-phoenix"></a>
# What would let Rekon be reborn?

The human's question is larger than a rename: how might Rekon itself become
Phoenix-like? This follows the NAN-origin remake commission, the digest and
evidence discussion, and the request to sample Chad Fowler's writing for its
vocabulary and richness.

**Proposed direction:** Rekon could become a place where technical intent,
reasons, evidence, and evaluations survive while their prompts, workflows,
tools, and implementations can be reconsidered. Not an engine that burns code
on command—a way to make renewal possible without forgetting.

That extends the existing [cumulative-work thesis](/constitution/reassessment0.gpt6a.md#situation).
It does not establish a new constitution, authorize migration, or declare the
current implementation disposable.

**The work anchor is `rekon-phoenix` in Rekon Beads** (`bd show rekon-phoenix`).
It holds scope, work state, evolving notes, and native session/commit references.
This document owns the source reading and architectural argument, not a second
lifecycle. The ticket was created at the human's explicit direction after the
first draft had been preserved.

<a id="rekon-phoenix-reading"></a>
## The sample, and why the later writing matters

I read the index and these eight essays directly on 2026-09-06. This is a
purposeful sample, not a survey or endorsement of the entire series. The pages
are author-published, date-labeled sources; no immutable web snapshot or
independent verification of the author's architectural claims was produced.

| Essay | Vocabulary and contribution to this inquiry |
|---|---|
| [Conceptual Mass and the Compaction Discipline](https://chadfowler.com/regenerative-software/3mbhnolyzds2d/) — Jan 2 | **Conceptual mass** is the burden of concepts, invariants, interfaces, dependencies, and exceptions—not line count. **Compaction** challenges whether distinctions earn their keep rather than merely rearranging them. |
| [The Deletion Test](https://chadfowler.com/regenerative-software/3md5ftetaes2e/) — Jan 24 | A diagnostic question about what would survive implementation loss. It explicitly is not a recommendation to delete. Required behavior and evaluation must not exist only in the artifact being replaced. |
| [The Regenerative Grain](https://chadfowler.com/regenerative-software/3mfai4nqg6224/) — Feb 19 | A replaceable unit is bounded by coherent invariants, mutation ownership, and verifiable contracts. Too coarse hides coupling; too fine fragments meaning. Well-bounded units still need composition evidence. |
| [The Conversation Is the Commit](https://chadfowler.com/regenerative-software/3mhxvpam4z22z/) — Mar 26 | **Provenance debt**, reasons and rejected alternatives, and conversation plus execution context. Strong motivation for preserving the path to a decision; its agent-only authorship prescription is not adopted here. |
| [The Phoenix Primitives](https://chadfowler.com/regenerative-software/3mjfruwwuck2d/) — Apr 13 | **Specification, evaluation, context boundary, provenance**; the **conservation layer** holds contracts that cannot change through one component's isolated regeneration. |
| [Production Is a Compiler Input](https://chadfowler.com/regenerative-software/3mjx4erlboc2l/) — Apr 20 | **Evidence decay**, **technical drift**, and **selective invalidation**: a changing world can undermine a claim even when its implementation does not change. |
| [The Implementation Remembers](https://chadfowler.com/regenerative-software/3mobohx4fq22x/) — Jun 14 | Strange behavior may be **scar tissue**, preserving lessons absent from documents. Cleaning up the artifact can destroy the memory. |
| [The Specification Is Not a Document](https://chadfowler.com/regenerative-software/3mtgs36dnq22o/) — Aug 19 | Heterogeneous connected knowledge, **plans and reasons**, scoped authority, explicit contradiction, and documents as projections. **Regeneration is an epistemic test** of whether enough knowledge escaped the implementation. |

The last essay substantially enriches a slogan-sized reading of the earlier
primitives. “Keep the spec” does not mean “put everything in `SPEC.md`.” It
means keeping connected knowledge in forms appropriate to it: executable
evaluations, schemas, prose, observations, constraints, disagreements, and
decisions. No one universal representation is promised.

Two short formulations are especially useful:

> “The durable model should be strict about authority and loose about vocabulary.”

> “The LLM is an interface to memory, not the memory itself.”

Both are from [The Specification Is Not a Document](https://chadfowler.com/regenerative-software/3mtgs36dnq22o/).
They fit the human's qualification that evidence may be a category of naming
and referentiality. Useful distinctions should remain possible without
requiring a committee, a new storage service, or a universal ontology.

<a id="rekon-phoenix-direction"></a>
## A possible shape for Rekon

The interesting unit is not necessarily a code module. An action practice can
also have a regenerative grain: a defined purpose, a bounded commission,
observable helpfulness criteria, authority limits, and a history of attempts.
Its exact prompt or orchestration can change without discarding those things.

This is **an analogy to test**, not proof that an action agent is as mechanically
evaluable as a deterministic component. Human usefulness, novel situations,
and model variance complicate that evaluation.

| Rekon responsibility | What should endure | What might change |
|---|---|---|
| Digest | Questions about present state and intent; source fidelity; the reader's steering need | Search strategy, report shape, number of agents, recency heuristics |
| Assessment | Architectural and behavioral concerns; evidence limits; authority to decide | Review prompt, investigative lenses, diagnostics, cohort composition |
| Remake | Desired outcomes, boundary obligations, negative knowledge, prefix acceptance | Proposed order, chosen implementation mechanisms, checkpoint granularity |
| Evidence and case stories | Native source identity, what was claimed and observed, disagreements, expected versus actual help | Indexes, summaries, query views, relationship vocabulary |
| Tooling and delivery | Verified behavior, ownership rules, permissions, observable activation | CLI implementation, assembler, harness adapters, particular prompt files |

These are cooperating responsibilities, not a mandatory pipeline. Digest can
change what we choose to assess. Assessment can reveal that no remake is
needed. A remake can uncover missing evidence. A case story can change the
practice's commission rather than its implementation.

The conservation layer in Rekon might therefore consist of **promises and
reasons**, not a frozen directory tree: who owns what, what an action may do,
which changes need acceptance, what evaluations protect, and where the actual
sources live. Even those boundaries can change, but not by silently rewriting
one prompt while every consumer assumes the old contract.

<a id="rekon-phoenix-evidence"></a>
## Evidence as a role, not a warehouse

An artifact does not become evidence merely by receiving the label. The
relationship is something like:

```text
source observation ── supports / challenges, within scope ──> claim
claim + reasons + authority ── may inform ──> decision
decision ── constrains ──> future action or implementation
evaluation ── tests a stated consequence ──> observed result
case story ── compares expected and actual help ──> practice revision
```

This is conceptual notation, not a schema being installed. Explained Markdown
links can already express much of it. A source may support several claims,
and a claim may have conflicting sources. Naming makes those joins possible;
it does not settle the disagreement or confer authority.

Fowler's August essay explicitly warns against maintaining a giant secondary
model by hand. Rekon should take that seriously. Capture important relations
as work happens and preserve native histories. A derived projection should
not be recursively treated as a fresh independent observation of its own
source, nor should it manufacture a second ticket lifecycle.

<a id="rekon-phoenix-tensions"></a>
## What I would borrow, and what I would not

**Borrow the relocation of rigor, not a claim that generation guarantees
fidelity.** The April primitives essay says derivation prevents spec drift.
A generator and its evaluations can share a misunderstanding; a passing suite
can miss a real obligation. The June and August essays sharpen exactly that
danger. Independent boundary evidence and surviving operational knowledge are
still necessary. Regeneration exposes some gaps; it does not prove completeness.

**Borrow provenance, not compulsory agent-only authorship.** The conversation
essay argues that direct human edits necessarily break the chain. That is too
strong for Rekon. Humans can record reasons and tests; agents can fabricate
reasons, miss constraints, or exceed authorization. Require inspectable
justification and outcomes, not a privileged kind of author. A transcript also
contains abandoned ideas: retaining it does not ratify every statement in it.

**Borrow replaceable interiors, not the abandonment of comprehension.** The
primitives essay deemphasizes navigation for generated interiors. Rekon's
experience says attention and navigability remain correctness concerns for
the people and agents selecting, verifying, and operating those interiors.
The grain essay itself retains comprehension as a diagnostic. Regeneration
should reduce the need for archaeology, not excuse opacity.

**Borrow compaction without erasing negative knowledge.** New homes such as
`digest/`, `remake/`, and `evidence/` must earn their distinction by helping
someone act. They should not become three parallel copies of the same story.
Equally, removing odd-looking code or prose before understanding its lesson
fails the implementation-memory test. Protocol and safety-sensitive components
deserve the extra care Fowler explicitly names in the grain essay.

**Borrow selective reassessment before automatic regeneration.** A changed
host contract or failed evaluation should reopen the affected claim and action
plan. It should not silently trigger deletion, rewriting, deployment, or new
policy. Evidence does not derive an “ought” from an “is.”

<a id="rekon-phoenix-next"></a>
## Three compatible directions, not a platform mandate

1. **Practice-centered renewal.** Make the action homes usable, run bounded
   cases, record expected versus actual help, and improve prompts from those
   cases. This is the smallest current direction and the one these documents
   begin—not yet an evaluated success.
2. **Queryable connected memory.** Build question-shaped views over existing
   sources and explicit relations. First establish a few valuable queries,
   such as “which current constraints came from a failed attempt?” Preserve
   source links and competing claims. No new graph database is selected.
3. **An evaluated regeneration workbench.** Eventually replace one bounded
   tool or practice using retained contracts and independent evaluations.
   Compare outcomes and missing knowledge before considering any promotion.
   This requires its own scope and authorization; it is not happening now.

These can coexist. They differ in what must be proved next. More documentation
alone cannot establish query reliability or replacement safety.

For a first real trial, preserve a practice's current form and compare a revised
form on a fresh, bounded case against declared usefulness criteria. Lastact
already teaches why: [its author self-assessment](/assess/review/prompt-assessment0.gpt6a.md)
found useful reviews but misallocated attention to the method itself. That is
a concrete learning target, not a reason to build a universal evaluator.

**The Phoenix question for Rekon:** if one of our prompts, tools, or workflows
vanished, could we recover what useful action it was meant to perform, what it
must not do, why those boundaries exist, and how to judge its replacement?
Where the answer is no, we have found work worth doing before rebirth.

## Cross-references

- [Root orientation](/README.md) **presents the action family**, while this
  document keeps the broader direction exploratory.
- [Coming up for air](/constitution/reassessment0.gpt6a.md#connections)
  **is the local conceptual precursor**: preservation enables redesign,
  attention matters, and signals must not inherit larger promises.
- [Reconstruction](/code/task-reconstruction/philosophy.md#reconstruct-mine-history)
  **already treats implementation as evidence** and preserves learned outcomes
  while reopening mechanisms.
- [NAN–Lastact genesis](/evidence/case-stories/nan-lastact/genesis0.gpt6a.md)
  **keeps this inquiry's actual provenance and limits**.
- [Evidence](/evidence/README.md), [digest](/digest/README.md), and
  [remake](/remake/README.md) **are bounded expressions of the direction**, not
  proof that Rekon has become a regenerative platform.
