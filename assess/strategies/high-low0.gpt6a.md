---
type: AssessmentStrategy
title: High/low passes — different agents, different aim points
description: Revises the same-prompt review experiment toward complementary conceptual exploration and close evidence, with agent effort allocated to promising idea fronts.
resource: /assess/strategies/high-low0.gpt6a.md
tags: [assessment, high-low, exploration, prompting, agents, praxis]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
extensions:
  session: urn:opencode:session:ses_f872bf58affeS5IBlC38cA6iYX
  predecessor_checkpoint: 30354e74
  disposition: Human-directed strategy exploration; not a launched wave or accepted complete prompt replacement.
sources:
  - { resource: /assess/review/form0.gpt6a.md, title: Same-form predecessor }
  - { resource: /assess/review/prompt-assessment0.gpt6a.md, title: Parent assessment being challenged }
  - { resource: /assess/gates/README.md, title: Evidence gates retained across differentiated passes }
  - { resource: /code/commits.md#commits-checkpoints, title: Capture stages before revamping }
  - { resource: "urn:opencode:session:ses_f872bf58affeS5IBlC38cA6iYX", title: Human correction toward high/low aim points and interest-based agent allocation, author: "human:rektide" }
---

# From repeated review to complementary inquiry

The preceding experiment gave Astra, GLM, Sol, Luna, and the parent essentially
the same bounded review commission. It produced useful close findings and a
comparable set of reports. It also constrained the possibility space. The
parent's claim that discouraging gratuitous rewriting was simply a success
missed this cost.

The human now proposes a **high/low strategy**: generally seek at least an
expansive and a grounded pass, steer different agents toward different aim
points, and map several promising idea fronts in parallel. Allocate agent
effort according to those fronts' possible interest and leverage—not merely
to repeat one question with more models.

The distinction to preserve is:

> **Constrain unauthorized action, not the scale of thought.**

No source changes during review does not mean no large proposals. Preserving
what works is a legitimate position, not a prerequisite imposed on every
reviewer. Radical alternatives can be useful without being commitments.

## What changes, what remains, where we are headed

| Predecessor | Revision | Destination |
|---|---|---|
| Same prompt, repeated across models | Shared situation and evidence contract; differentiated questions | Complementary discovery, not only corroboration |
| Prefer the smallest response, avoid redesign | Make both incremental and structural possibilities discussable; state costs and assumptions | A richer map of what could become possible |
| Prompt reflection appended to product review | Assess the steering itself: what each aim point surfaced or suppressed | Learn how to commission work, not just collect reports |
| More agents largely mean more coverage of one scope | Allocate effort among fronts by interest, uncertainty, and leverage | Deliberate use of stronger reasoning where it can change direction |
| Full-wave independence gate | Retain, but declare which unlike inputs the synthesis needs | Well-equipped comparison without pretending unlike reports are votes |

The original form and reports remain intact. Checkpoint `30354e74` preserves the
parent's prior assessment plus verification addendum before this change in
direction. This new file records the delta; it does not rewrite the earlier
stance into an appearance of having always endorsed it.

## Two axes, not one ladder

**High/low describes the aim point, not the model's worth or size.**

- **High pass:** purpose, possibilities, alternative framings, design axes,
  relationships, latent capabilities, and larger transformations.
- **Low pass:** actual behavior, contracts, state transitions, counterexamples,
  evidence quality, operational constraints, and concrete mechanisms.

Either can require substantial reasoning. A difficult concurrency or host
contract question may deserve Astra or Sol. A bounded conceptual inventory
may be well served by GLM or Flash. A medium-sized model's alternate reading
can be valuable precisely because it approaches the question differently.

Both passes should be able to report observations that belong at the other
altitude. Their assignments emphasize a question; they do not censor discoveries.

## Example shared envelope and distinct prompts

The **shared envelope** names the human situation, references and baselines,
active uncertainties, artifact ownership, permitted diagnostics, gate, and
action authority. It explicitly welcomes exploration while keeping product
edits and ticket mutations behind their own decisions. Both returns include a
substantive evaluation of how their prompt shaped the work.

### High: open the possibility space

> Assess the larger possibilities around this working system. What is it
> becoming, what could it enable, and which alternative framings or designs
> would materially change its usefulness? Do not let the current implementation
> or an instruction to minimize changes determine the size of your thinking.
> Map the consequential design axes and several promising fronts. Ground
> starting facts; label assumptions and proposals; explain opportunities, costs,
> and tensions. Give the no-change position a fair hearing without making it the
> default answer. Identify which fronts deserve deeper investigation and why.
> Proposals are not implementation authorization. Evaluate what this prompt
> helped you see and what it may have steered you away from.

### Low: establish what the system can support

> Trace the actual system and its load-bearing boundaries. What works, what
> does each signal prove, which invariants matter, and where do concrete
> counterexamples challenge the stated contract? Distinguish supported behavior,
> compatibility conditions, absent evidence, and actual failures. Use focused
> probes where permitted. Explain which mechanisms make changes easy or hard,
> including where a structural change—not merely a patch—could be justified.
> Do not mistake current limitations for permanent limits on possible design.
> Proposals are not implementation authorization. Evaluate what this prompt
> helped you see and what it may have steered you away from.

These are candidate prompts for workshopping, not a newly frozen run. The
high pass is not “be vague”; the low pass is not “only fix bugs.”

## Allocate interest before allocating agents

For each front, state the **question, possible payoff, uncertainty, required
depth, and useful return**. The possible payoff can be exploratory: discovering
a promising new direction is valuable even if it does not produce an immediate
patch. Do not turn speculative interest into a false quantitative score.

The human names Astra, Sol, GLM, GLM-Flash, and sometimes Luna as a useful range
of agent allocations. Use those as operator-configurable choices, not a rigid
model-to-role taxonomy. Kimi and Minimax are part of the earlier practice but
are not currently available dispatch choices in this session. Never manufacture
availability or silently substitute a requested participant.

Possible allocation patterns:

- **High/low pair:** one opens the design space, one establishes the concrete
  ground. Appropriate when both breadth and depth matter.
- **One deep front plus several bounded scouts:** invest a stronger reasoner in
  a high-interest, consequential question; let smaller assignments map neighboring
  possibilities and evidence. The scouts can reveal where to invest next.
- **Competing high passes plus a low pass:** explore genuinely different
  architectural directions while independently checking current capabilities.
- **Same-prompt replicas:** still useful when robustness or comparative prompting
  is itself the question. Deliberately choose this experiment rather than making
  it the default shape of all parallel work.

The first two assignments can run independently from the same envelope.
Follow-ups that specifically test a high-pass proposal need that proposal as an
input: put them behind a dependency gate instead of smuggling evolving conclusions
into the original independent low pass.

## Synthesize across aim points

Wait for the declared evidence set, then relate results by question and claim:

- What did each pass uniquely make visible?
- Which low-level facts enable, constrain, or redirect a high-level possibility?
- Which high-level possibilities reveal that the low pass used too narrow a
  definition of the problem?
- What is contradicted, what is merely a different aim point, and what remains
  untested? Do not call complementary reports independent votes.
- Which front now deserves stronger reasoning, an experiment, a design pass,
  or an implementation decision—and why?

Assess the prompts as well as the outputs: **did our steering create useful
differences, or merely assign different headings?** Did the allocation uncover
high-interest ideas, and did the evidence make them more actionable without
prematurely shrinking them?

Commit every coherent artifact when it exists. Preserve the completed wave
before changing its aims. Explain the next destination rather than silently
rewriting the earlier attempt. The gate protects interpretation; checkpoints
preserve how the interpretation became possible.
