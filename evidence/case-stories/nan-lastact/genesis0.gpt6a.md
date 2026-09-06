---
type: CaseStory
title: From NAN and Lastact to actions that can learn
description: The provenance and alignment questions behind digest, remake, and evidence, associating NAN's commission with Lastact's assessment-practice experiment without claiming a completed NAN assessment.
resource: /evidence/case-stories/nan-lastact/genesis0.gpt6a.md
tags: [case-story, nan, lastact, digest, remake, assessment, provenance]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
extensions:
  session: urn:opencode:session:ses_f86f757c0ffe5G5aHU26e2evOL
  posture: Current-session observation and cited prior author self-assessment; not an independent outcome evaluation
sources:
  - { resource: "urn:opencode:session:ses_f86f757c0ffe5G5aHU26e2evOL", title: NAN-origin commission, directory move, and action-practice expansion }
  - { resource: /remake/design/brief0.gpt6a.md, title: Frozen initial practice brief }
  - { resource: /assess/case-studies/lastact/wave0.gpt6a.md, title: Lastact assignment and evidence boundary }
  - { resource: "https://github.com/rektide/rekon/blob/211fb0d41a57545774f9d151fbd2a95715a665a2/assess/review/prompt-assessment0.gpt6a.md", title: Lastact prompt author's self-assessment at its recorded commit }
  - { resource: /remake/research/precedents0.gpt56s.md, title: Source investigation for the remake definition }
---

<a id="nan-lastact-genesis"></a>
# From two efforts to a family of actions

NAN and Lastact are **associated cases, not equivalent experiments**. Lastact
provided an exercised architectural-review practice and a retrospective on its
prompting. NAN supplied the new question about remaking a heavily developed
project as a sequence of defensible landings. This session then moved into
Rekon to define the practice before applying it.

The association is useful because both ask a second-order question: **did the
action help in the way the human wanted?** A technically competent deliverable
can still spend its effort on the wrong object.

## Native lineage and the scope change

| Episode | Native record | What it establishes |
|---|---|---|
| Lastact review experiment | Session `ses_f872bf58affeS5IBlC38cA6iYX`; [wave 0](/assess/case-studies/lastact/wave0.gpt6a.md) | A frozen form, four dispatched peer reviewers, a separate parent review, and an explicitly unavailable Terra slot. |
| Lastact prompt assessment | [Author self-assessment](/assess/review/prompt-assessment0.gpt6a.md), recorded at `211fb0d41a57545774f9d151fbd2a95715a665a2` | The author's account of useful product reviews, misweighted method evaluation, and a corrected authorization mistake. This session has not independently replayed the experiment. |
| NAN-origin remake request | Current session `ses_f86f757c0ffe5G5aHU26e2evOL`, initially `/home/rektide/src/nan-rs` | Human prioritizes a defensible patch sequence, then architectural/high-level assessment at each step. Initial inspection identified `nan-rs` and the transport workspace as different owners. |
| Move to Rekon | Same session, `session_move` to `/home/rektide/src/rekon`, 2026-09-06 | Human explicitly requests that the reusable practice be done here and the project application later. No new session identity and no completed NAN assessment. |
| Broader action vocabulary | Subsequent messages in the same session | Human defines digest, asks for the root README to describe these actions, asks for expected-versus-actual case stories, and qualifies evidence as referentiality/naming rather than necessarily a separate artifact kind. |
| Phoenix inquiry | Human supplies [Fowler's series](https://chadfowler.com/regenerative-software/) and asks about Rekon's own rebirth | Permission to read, sample, and explore a direction—not a decision to rename Rekon or regenerate its implementation. |

The initial NAN reading used clean source tips `fc9e29fb` in `nan-rs` and
`25fa0c93` in `wl-nl80211-nan`, plus the existing design/burn records. Those
are orientation receipts only: no Rust checks, detailed product review, or
reconstruction wave was run. No NAN files or references were changed.

## Expected help versus observed behavior

### Lastact: useful work, imperfectly aimed

The [prompt assessment](/assess/review/prompt-assessment0.gpt6a.md) reports that
the reviews usefully explained architecture and investigated host seams, and
that all four dispatched reviewers returned before comparative reading. It
also reports that method reflection occupied only about 7–11% of each report,
despite the clarified human priority being to workshop the practice itself.

The important lesson is not “the reviewers failed.” The author's account is
that they followed a commission whose attention was misallocated. It also
records the parent putting provisional rules into the live `AGENTS.md` before
discussion, then removing only those additions and retaining an inactive
proposal. Uncommitted guidance was still active in that harness.

These are specific accounts of conduct and correction, not a general ranking
of models or proof that the revised form is better. A fresh trial is still
needed to evaluate the revision's effect.

### NAN to Rekon: keep the actual object visible

The parent initially announced a two-repository project investigation. Before
any product agent was dispatched, the human redirected the work into Rekon.
The session moved and only one bounded documentation-precedent investigation
was launched. That agent wrote its own report; the parent read the complete
artifact before finalizing the remake definition.

The observable alignment is **scope correction and ownership preservation**:
the requested practice is being written in Rekon, while the project exercise
remains deferred. The expected further benefit—using remake to improve a NAN
landing series—has not been observed. Likewise, the new digest and evidence
homes have not yet demonstrated that they make a future effort easier to steer.

## What this association should teach

- **Commission the right object.** If we are improving a practice through a
  project, say so before allocating most of the work to the project.
- **Preserve the human's priority changes.** A useful case story records when
  the question changed instead of rewriting the original ask as if it had
  always contained the final scope.
- **Name the owner.** A directory move can locate work correctly without
  breaking session lineage or relocating product evidence.
- **Record restraint as conduct, not completion.** Avoiding unauthorized code
  changes matters; it does not establish that the proposed practice works.
- **Keep the next trial small enough to learn from.** A real digest can test
  state recovery and steering usefulness; a paper remake can test sequencing
  clarity; a separately authorized landing can test intermediate correctness.
  These answer different questions.

No universal success metric or evaluator service has been implemented. The
next case should record the expected helpful result before the action, then
compare it with what the reader actually did or said afterward.

## Cross-references

- [Evidence](/evidence/README.md#evidence-case-stories) **owns this case-story
  shape**; the source artifacts retain their existing homes.
- [Digest](/digest/README.md) and [remake](/remake/README.md) **are the new
  practice definitions** commissioned here.
- [Assessment](/assess/README.md) **owns Lastact's existing experiment**;
  this association does not promote a new product conclusion.
- [Phoenix inquiry](/design/rekon/phoenix0.gpt6a.md) **extends the question**
  from improving a task to preserving understanding across replacement.
