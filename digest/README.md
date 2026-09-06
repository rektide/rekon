---
type: Praxis
title: Digest — find the living state of an effort
description: A recency-biased architectural reading of developments and intent, surfacing practical differences and deeper direction so an effort can be deliberately re-steered or prepared to ship again.
resource: /digest/README.md
tags: [digest, architecture, intent, assessment, steering]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
extensions:
  ticket: rekon-phoenix
sources:
  - { resource: "urn:opencode:session:ses_f86f757c0ffe5G5aHU26e2evOL", title: Human definition of digest and action practices }
  - { resource: /constitution/reassessment0.gpt6a.md, title: Coming up for air }
  - { resource: /assess/README.md, title: Assessment }
  - { resource: "https://chadfowler.com/regenerative-software/3mtgs36dnq22o/", title: Chad Fowler — The Specification Is Not a Document }
---

<a id="digest"></a>
# Digest

**Find out where things stand, what is changing, and what the effort is trying
to become.** Return an account that helps someone choose their next move.

A digest is an action, not just a shortened document. It often commissions one
or more architectural overviews, reads recent design work, and follows the
differences that matter. It asks both what changed practically and what the
changes say about the underlying purpose—the human's **“soul messages.”** It
can help re-steer an effort, resume it, or prepare it to ship again. None of
those follow-on actions is silently authorized by producing the digest.

This is a draft practice, not a registered CLI command or installed agent.

<a id="digest-attention"></a>
## Recency directs attention, not authority

Start with modern developments: recent commits, active design documents,
reviews, conversations, changed dependencies, and current owner decisions.
Declare the time window and repositories examined. Publication date,
observation date, and the date a claim became effective may differ.

Then follow the load-bearing references backward. Older decisions, shipped
promises, negative results, and unchanged invariants can outweigh a new draft.
An old document can remain current; a new document can describe obsolete code.
Never infer acceptance from freshness, confidence from length, or progress from
the number of documents produced.

The result should expose **intent fronts**: active lines along which the desired
system, its obligations, or its ownership are changing. These are not necessarily
features or tickets. “Keep the transport neutral,” “make recovery explicit,” and
“reduce the knowledge required of callers” are different fronts, even when one
patch touches all three.

<a id="digest-procedure"></a>
## How to do it

1. **Name the decision this reading should equip.** Resumption, architectural
   orientation, scope correction, and release preparation ask different things
   of the same corpus. Include the human's priorities, not just a search topic.
2. **Establish a present-state spine.** Read the maintained entry points, pin
   relevant code and dependency revisions, identify the recent window, and
   distinguish current work from proposed or historical material. Do not
   pretend an exhaustive review is required just to regain orientation.
3. **Read for consequential differences.** What became possible or impossible?
   Which boundary, state owner, dependency, public promise, or validation claim
   changed? A diff of ideas matters more than a list of newly created files.
4. **Recover direction as well as mechanics.** Explain the purpose behind the
   changes, what the effort is protecting, and which tensions recur. Cite the
   human's stated intent separately from your interpretation of it. A “soul
   message” is not permission to invent requirements on the human's behalf.
5. **Check the claims that would alter steering.** Trace them to their owning
   code, decision, experiment, or upstream contract. A document-only digest is
   legitimate when labeled; it must not call described capabilities verified.
6. **Offer a small set of next courses.** Explain benefits, costs, unresolved
   questions, and what would change the recommendation. “Continue as planned”
   and “leave this alone” are real outcomes, not failures to find work.

For a broad effort, delegate complementary overviews by domain or concern.
For independent competing interpretations, freeze the brief and keep peer
findings separate until the declared reading gate opens. Use
[assessment gates](/assess/gates/README.md) rather than treating the first
arrival as the emerging consensus. More agents are not inherently a better
digest; a single well-grounded reading may suffice.

<a id="digest-output"></a>
## What comes back

Lead with **the state now and the next decision**, not the excavation diary.
Use the following shape as a compact report, expanding only where earned:

- **Situation and basis:** question, owners, revisions, recency window, sources
  inspected, exclusions, and whether anything was executed.
- **Architectural account:** purpose, boundaries, state ownership, relevant
  dependencies, strengths to retain, and pressure on the current shape.
- **Practical differences:** a table of previous understanding → current
  evidence → consequence, including unchanged obligations where important.
- **Intent fronts:** the deeper directions, their sources, agreements and
  disagreements, and which interpretations still need human confirmation.
- **Next courses:** recommended steering and credible alternatives, with
  evidence or decisions still needed before action.
- **Limits and reflection:** what remains unknown; whether this digest made
  the intended decision easier or merely produced more reading.

An observation might be established; an interpretation might be persuasive;
a recommendation might be useful. Those are three different claims. Keep them
separable through [evidence relationships](/evidence/README.md).

<a id="digest-completion"></a>
## When it has helped

A useful digest lets its reader locate the present effort, see consequential
differences, understand its direction, and choose or explicitly defer a next
move. Capture the reader's response when available. Producing a lucid report
is not proof that it helped re-steer or ship anything.

Record important surprises as a [case story](/evidence/README.md#evidence-case-stories):
perhaps the reading recovered a lost constraint; perhaps it over-weighted recent
drafts, missed the real ask, or proposed work already completed. Preserve those
differences between expected and actual usefulness so the practice can improve.

<a id="digest-relations"></a>
## Neighbors and lineage

- [Assessment](/assess/README.md) **deepens judgment** where the digest finds a
  consequential seam. Digest emphasizes situated understanding; assessment
  emphasizes whether a claim, design, or action earns confidence. They overlap
  without needing a rigid sequence.
- [Remake](/remake/README.md) **uses a recovered state and intent** to propose
  a defensible build-and-landing order. A digest does not automatically enter
  that exercise.
- [Coming up for air](/constitution/reassessment0.gpt6a.md) **is a local
  antecedent**: it reads across recent work to recover purpose and distinguish
  cooperating responsibilities. It predates this practice name.
- [NAN–Lastact case-story association](/evidence/case-stories/nan-lastact/genesis0.gpt6a.md)
  **records this practice's commission**, including the move into Rekon.
- [Phoenix reading](/design/rekon/phoenix0.gpt6a.md) **offers an extension**:
  digest as a question-shaped view of durable knowledge, not a replacement
  source of truth.
