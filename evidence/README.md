---
type: PracticeConcept
title: Evidence — references that bear on a judgment
description: A relational home for evidence, provenance, and case stories without relocating owner-native artifacts or treating naming as proof.
resource: /evidence/README.md
tags: [evidence, reference, provenance, case-stories, learning]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
extensions:
  ticket: rekon-phoenix
sources:
  - { resource: "urn:opencode:session:ses_f86f757c0ffe5G5aHU26e2evOL", title: Human direction on evidence as referentiality and on learning from action case stories }
  - { resource: /constitution/README.md, title: Documentation and evidence contracts }
  - { resource: /constitution/reassessment0.gpt6a.md, title: Signals must not inherit larger promises }
  - { resource: /assess/review/prompt-assessment0.gpt6a.md, title: Lastact prompt assessment }
  - { resource: "https://chadfowler.com/regenerative-software/3mtgs36dnq22o/", title: Chad Fowler — The Specification Is Not a Document }
---

<a id="evidence"></a>
# Evidence

**Evidence names a relationship: something bears on a claim or a judgment.**
It need not be a new species of file, a universal schema, or a central store.
A test, conversation, decision, trace, implementation, or failed attempt may
be evidence in one inquiry and simply background in another.

This home makes that role easier to express and find. It does not require
moving evidence out of the projects, sessions, repositories, or systems that
own it. Nor does placing a file here make its contents more authoritative.

<a id="evidence-reference"></a>
## Naming is the beginning of the relation

Reference makes something addressable. Provenance explains where it came from.
Calling it evidence adds an argument about **what it supports, challenges, or
leaves unresolved, under which conditions**. These are useful distinctions
within referentiality, not a claim that each deserves a separate platform.

Prefer a small, inspectable statement:

> This observation, produced by this action against this basis, supports or
> challenges this claim for this reason. It does not establish that larger
> claim. Here is where to inspect it.

Record just enough to answer:

| Question | Why it matters |
|---|---|
| What claim or decision is this relevant to? | A citation without an argument does not say what follows. |
| What is the source, scope, revision, and observation context? | A real result can be inapplicable to a different version or environment. |
| Is this observed, reported by someone, source-derived, or inferred? | These offer different kinds of confidence; none should impersonate another. |
| What does it not prove, and what could defeat the inference? | Passing checks and plausible stories both have ceilings. |
| Who can decide whether this becomes an obligation? | Evidence can inform intent; it does not automatically rewrite it. |

Not every link needs a table. A sentence can carry this relationship well.
Preserve contrary evidence and scoped disagreements instead of forcing one
apparently consistent story. A later source may supersede an earlier claim;
it does not erase the earlier observation.

<a id="evidence-case-stories"></a>
## Case stories: did the action help?

A case story connects **desired help → commissioned action → actual conduct →
result → reflection**. It is not only a success anecdote, an incident report,
or a transcript digest. It should let a later person understand how an action
did or did not align with what was wanted.

For a consequential attempt, capture:

1. **Setting and expected help.** The human's question, priority, and what a
   useful result would have enabled. Say when that expectation was clarified.
2. **The actual commission.** Prompt/form version, participants, scope, allowed
   actions, and native session or work references. Preserve directory moves
   and cross-session associations without inventing causal dependencies.
3. **Conduct and artifacts.** What actually happened, including corrections,
   missing participants, and departures from scope. Link the owner records.
4. **Expected versus observed.** What was produced, what was used, what the
   human said, and what remains unobserved. Reported usefulness is not measured
   downstream impact; artifact completion is not successful shipping.
5. **Interpretation and next trial.** Plausible reasons for alignment or mismatch,
   alternatives to that explanation, and the smallest practice change worth
   trying. Do not infer general model quality from one attempt.

This is a kaizen loop: accumulated cases should help us revise the work, not
only enlarge the archive. If a practice is the experimental object, allocate
real attention to its usefulness in the original commission. Do not make its
evaluation an afterthought beneath a large product review.

<a id="evidence-stories"></a>
## Initial association

- [NAN and Lastact: genesis of a family of actions](/evidence/case-stories/nan-lastact/genesis0.gpt6a.md)
  distinguishes a **new commission and deferred product application** in NAN
  from Lastact's **completed review experiment and author self-assessment**.
- [Did our review prompt serve the ask?](/assess/review/prompt-assessment0.gpt6a.md)
  remains the owner-native Lastact account. Its useful reviews, underweighted
  method evaluation, and corrected guidance-activation mistake are evidence
  about the practice, not merely plugin findings.

<a id="evidence-boundary"></a>
## What this does not claim

There is no implemented evidence graph, ingestion service, truth registry, or
automatic promotion process here. The first useful machinery is named sources,
explained links, honest scope, and cases that can be revisited. A queryable
projection may later help; it should retain source ownership and not require
hand-maintaining a competing history.

Some conversations or runtime records cannot safely be published. Preserve a
stable reference and an access limitation where necessary rather than copying
private material into a public case story.

## Cross-references

- [Constitution](/constitution/README.md#doc-constitution-references-evidence)
  **constrains provenance and claim honesty**; this home does not replace it.
- [Digest](/digest/README.md) **finds and interprets current evidence** for
  steering; [assessment](/assess/README.md) **judges its sufficiency**.
- [Remake](/remake/README.md) **attaches it to proposed intermediate states**;
  a tested final tree is not evidence that an unbuilt patch prefix passes.
- [Phoenix reading](/design/rekon/phoenix0.gpt6a.md) **explores the larger
  possibility** of preserving understanding while replacing its implementations.
