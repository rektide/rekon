---
type: PracticeBrief
title: Remake — reconstruct the landing, assess the growth
description: The commissioning question and scope for a Rekon practice that derives a defensible patch sequence and architectural assessments from a developed project.
resource: /remake/design/brief0.gpt6a.md
tags: [remake, assessment, architecture, sequencing, provenance]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
extensions:
  ticket: rekon-phoenix
sources:
  - { resource: "urn:opencode:session:ses_f86f757c0ffe5G5aHU26e2evOL", title: Human commission and redirection from NAN to the Rekon practice }
  - { resource: /assess/README.md, title: Assessment practice }
  - { resource: /code/task-reconstruction/philosophy.md, title: Reconstruct from the present }
---

# Remake commissioning brief

## Situation and the human question

The human began in `~/src/nan-rs`, reflecting on a recently developed project
that had grown heavily through successive burns. The priority was first to
ask what the **right build order would be if patches landed in sequence**, then
to ask **how each step should be assessed against architecture and high-level
concerns**. The desired reusable home was named `remake/` in Rekon.

After initial reading, the human redirected the session: do the Rekon practice
now, in Rekon itself, and leave its project application for later. Session
`ses_f86f757c0ffe5G5aHU26e2evOL` moved from `/home/rektide/src/nan-rs` to
`/home/rektide/src/rekon` on 2026-09-06. The native session identity did not
change. No NAN code, documents, or references were changed before the move.

This is a **practice-definition draft**, not a completed NAN assessment, a
reconstructed patch stack, or acceptance of a new product architecture.

## Research question

> Given a developed system, its present intent, and its accumulated evidence,
> what sequence of coherent changes would make its construction and landing
> defensible? At each consequential step, what architectural judgment and
> evidence would warrant proceeding, without assuming future patches already
> exist?

The ordering question leads. Assessment should justify and improve that order,
not bury it under a universal review checklist.

## Work boundary

Included:

- Define the practice and distinguish it from review, planning, clean rework,
  and version-control replay.
- Describe the expected sequence proposal and its stage-specific assessments.
- Preserve uncertainty about whether a proposed intermediate tree will build
  until it has actually been materialized and checked.
- Connect the new home to existing Rekon knowledge without relocating sources
  or replacing existing practices.

Excluded:

- Assessing NAN or Lastact as products in this run.
- Reordering, squashing, replaying, implementing, publishing, or pushing code.
- Changing ambient `AGENTS.md`, installing a skill, or building automation.
- Creating a second project lifecycle in tickets merely to represent the method.

## Evidence work and release condition

The parent owns the method's architectural choices and has read the existing
assessment and reconstruction practices. One complementary background
investigation, Sol high session `ses_f86f5471bffemEW3v8DYYMIY3p`, owns
[`precedents0.gpt56s.md`](/remake/research/precedents0.gpt56s.md): primary
documentation relationships, conflicts, and absences. It does not assess a
product or independently propose a rival method.

Before finalizing the draft, the parent must read that complete artifact and
verify the load-bearing source relationships. This is an evidence dependency,
not a multi-model vote. No code action is authorized when that gate opens.

## Candidate distinctions to test

- Historical discovery order, recommended construction order, and maintainer
  landing order need not coincide.
- A defensible order is conditional on the desired outcome and review audience;
  a dependency graph can admit several equally valid sequences.
- Knowing how the final system works does not prove every proposed prefix works.
- Late fixes may belong with their owning capability in the proposed series,
  while the actual corrective history remains preserved and cited.
- Architectural assessment asks what a stage makes possible, where ownership
  settles, and which future changes become easier or harder—not only whether
  its tests pass.
- A retrospective exercise can use hindsight without claiming the builders
  should have possessed that knowledge earlier.

## Cross-references

- [Assessment](/assess/README.md) **owns judgment and evidence posture**;
  remake adds a sequence of states to assess rather than another defect rubric.
- [Reconstruction philosophy](/code/task-reconstruction/philosophy.md)
  **provides prior art and a boundary**: history is evidence, not architecture;
  actual redesign and implementation require their own chosen work mode.
- [Evidence gates](/assess/gates/README.md) **provides the release discipline**
  for the bounded source investigation above.

# Addendum — broader direction and work anchor, 2026-09-06

The human subsequently broadened this commission to define `digest/`, explain
`evidence/` as referentiality rather than necessarily a new artifact kind, and
update Rekon's root README around the growing family of action practices.
The human also supplied Chad Fowler's Phoenix Architecture series and asked
how Rekon itself might be reborn. These directions are preserved in the
[NAN–Lastact case story](/evidence/case-stories/nan-lastact/genesis0.gpt6a.md)
and [Phoenix reading](/design/rekon/phoenix0.gpt6a.md).

At explicit human request, **`rekon-phoenix` is now the work's Beads anchor in
Rekon**. This supersedes the initial no-ticket scope only for the authorized
Rekon work; it does not authorize NAN tickets or product changes. Use
`bd show rekon-phoenix` for current work state and notes, including the session,
artifact, and commit receipts. The initial brief remains as the record of the
earlier scope rather than being rewritten to predict the later request.

The required precedent report returned as `c71028b1` and was read in full
before the parent finalized the first remake definition. Its relevant
reconstruction and assessment claims were checked against their source
documents. That satisfied the drafting dependency, not an implementation gate.
