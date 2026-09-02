---
type: ConstitutionSynthesis
title: Self-explaining documentation constitution synthesis
description: Cross-comparison and decisions used to turn the independent Sol and GLM Max visions into the first canonical workspace constitution.
resource: /design/doc-constitution/syn0.gpt56t.md
tags: [rekon, documentation, constitution, synthesis]
status: draft
generated: { by: "model:gpt-5.6-terra", at: 2026-09-01T17:32:46-04:00 }
sources:
  - id: sol-draft
    resource: /design/doc-constitution/draft0.sol56m.md
    title: Workspace document constitution vision
    author: model:gpt-5.6-sol-medium
  - id: glm-draft
    resource: /design/doc-constitution/draft0.glm53m.md
    title: Workspace knowledge-module constitution
    author: model:glm-5.3-max
  - id: human-direction
    resource: urn:opencode:session:ses_fa19cd46bffeBW6dPrBzttf4Gv
    title: README, SKILL, GLOBAL, ticket-anchor, and knowledge-flow direction
    author: human:rektide
---

<a id="doc-constitution-synthesis"></a>
# Constitution Synthesis

Both independent drafts found the same center: documentation is not a pile of
files or an AGENTS.md manuscript. It is a context-delivery system built from
self-explaining knowledge modules. Inquiry becomes maintained knowledge;
maintained knowledge can be activated on demand; selected operating rules can
be made ambient; tickets coordinate the work; anchors and links preserve its
identity.

The convergence is strong enough to write a canonical draft. The differences
are useful policy choices rather than competing architectures.

<a id="doc-constitution-synthesis-common"></a>
## Common Ground

Both drafts establish:

1. `README.md` is the canonical body and human landing page.
2. `SKILL.md` is a relative symlink, not a second maintained body.
3. `GLOBAL.md` is a selected ambient projection, not a README summary.
4. `design/` and `doc/` name epistemic postures, not quality grades.
5. Promotion is synthesis into stable user-facing knowledge, not moving or
   deleting evidence.
6. Semantic anchors carry durable identity; display numbering is optional.
7. Ticket and anchor names share a project namespace without requiring
   ticket-per-section parity.
8. Canonical tips remain coherent and mutable while model-suffixed evidence
   retains exact historical meaning.
9. Heavy research mechanics remain discoverable opt-in patterns.

<a id="doc-constitution-synthesis-choices"></a>
## Choices Made

### One namespace, no new separator

Sol uses `doc-constitution` locally and `rekon-doc-constitution` globally.
GLM proposes `rekon:doc-constitution` for sections while retaining the hyphen
form for tickets. The canonical draft chooses Sol's form because the human
asked for the omitted beads project prefix to become implied qualification.
Context distinguishes a ticket record from a document coordinate; a canonical
Markdown URL resolves the latter. No new punctuation language is needed.

### Mutable README, durable evidence

The canonical `constitution/README.md` is a real maintained file, not a symlink to one
wave. Design drafts remain exact, immutable evidence. This preserves both an
usable current account and honest history.

### Ambient admission uses three tests

The canonical draft adopts GLM's memorable corruption, cheapness, and
universality tests, framed by Sol's expected-cost test. No hard byte cap is set
before the assembler exists; fragment size and total assembled cost must be
made reviewable by that tooling.

### Skills may exist in either tissue

Both `design/<topic>/` and `doc/<topic>/` may expose skills. Draft status must
be honest and design-origin global fragments face a higher stability bar.

### Prospective workspace default

The constitution governs new workspace work by default. Existing files are
grandfathered; projects may document local deviations. It is not a mandate to
mass-rewrite the corpus.

<a id="doc-constitution-synthesis-strengths"></a>
## Cross-Comparison

The Sol draft is strongest as a balanced constitution. It clearly separates
knowledge posture from file quality, explains promotion as synthesis, gives a
simple one-namespace mapping, and protects proportional process. Its ticketing
table and mutable-tip model are especially suitable for ambient guidance.

The GLM draft is strongest as an activating manifesto and implementation
provocation. "Edit locally, assemble globally, cite durably, coordinate by
tickets" compresses the system well. Its ambient admission test, warning about
scratch mortality, and concrete fragment sketch make costs visible.

Sol's main weakness is breadth: at 686 lines it can become the heavy machinery
the constitution argues should be optional. GLM's main weakness is added
syntax and stronger ceremony: the colon-qualified section form and universal
append-only language create avoidable distinction and friction. The canonical
draft keeps Sol's structure and restraint while using GLM's strongest compact
concepts and tests.

<a id="doc-constitution-synthesis-open"></a>
## Tool Questions Deliberately Deferred

The later AGENTS assembler must still decide:

- where the declared fragment manifest lives and how domain order is stated;
- how assembled output preserves source provenance and source-relative links;
- whether size is warned, budgeted, or capped;
- how duplicate fragment identities and canonical anchors are validated;
- how skill-shaped project directories are discovered or installed;
- how symlink-unaware packaging is detected and repaired.

These are tickets under `rekon-agents-maintenance`, not reasons to postpone the
knowledge-module constitution.
