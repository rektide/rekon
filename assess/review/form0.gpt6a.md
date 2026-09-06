---
type: ReviewProtocol
title: Architectural and close review — form 0
description: Frozen shared instructions for the Lastact independent-review experiment and a reusable starting point for future assessments.
resource: /assess/review/form0.gpt6a.md
tags: [review, architecture, correctness, evidence, experiment]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
sources:
  - { resource: /AGENTS.md, title: Architectural stewardship }
  - { resource: /constitution/README.md, title: Evidence and proportional process }
  - { resource: "file:///home/rektide/.agents/skills/code-review/SKILL.md", title: Standards and spec as distinct review axes }
---

# Architectural and close review — form 0

## Commission

Review **the system as it stands**, unless the assignment explicitly supplies a
change range. The user believes it fulfills its purpose and works well. Treat
that as valuable operational testimony, not proof of every behavior. Determine
whether the implementation fits its intent, what earns confidence, what limits
it, and which small next decisions would matter. Do not manufacture a redesign.

The assignment must name the target repository and immutable baseline, scope
(whole system or exact diff), intent sources, relevant host/dependency versions,
artifact destination, allowed checks, and independence/completion gate. Record
missing inputs rather than quietly filling them with invented requirements.

## Conduct

1. Read the intent and explain the system in your own terms. Then inspect the
   implementation, tests, and load-bearing upstream contracts directly.
2. Follow important paths across boundaries: inputs/configuration, state and
   ownership, external operations, output, restart/recovery, and disposal.
   Investigate timing, concurrency, failure, compatibility, and resource behavior
   where relevant. This is a lens set, not a demand to invent a finding per lens.
3. Check the exact installed/source host contract. Distinguish published docs,
   installed types, local upstream source, and live runtime observation. A type
   assertion or mock cannot prove a host behavior by itself.
4. Tests and tooling support judgment; report commands, results, and what they
   do not cover. Do not reinstall dependencies, mutate production state, run a
   live-session experiment, or fix source during review. Put allowed diagnostic
   artifacts in your own ignored scratch directory.
5. Keep hard spec failures, documented-standard violations, architectural
   judgments, and hypothetical risks distinguishable. Cite a rule before calling
   something a rule violation. A smell is a heuristic, not a verdict. Do not
   waste the report restating stylistic failures already covered by tooling.
6. Preserve independence: no peer reports, peer summaries, or synthesis until
   the declared wave gate opens. Return only an artifact/session/check receipt
   to the coordinator; put findings in the artifact.

## Markdown deliverable

Aim for a focused report of roughly 1,200–2,200 words; use more only when the
evidence earns it. Use these headings so reports can be compared without forcing
identical conclusions:

### 1. Assessment in brief

State the overall architectural judgment and the most important threads. Say
what should be preserved, what needs attention, and how confident you are.

### 2. Basis and boundaries

Record repository, revision, reviewed scope, intent sources, host/dependency
basis, actual model/session identity (or uncertainty), checks, and exclusions.
Whole-system review needs no artificial diff base. Distinguish user testimony
from tests and your own source reasoning.

### 3. Architectural account

Explain purpose and non-goals, main domain boundaries, data/control flow, state
owners and lifetimes, and the seams that couple the implementation to its host.
A small diagram or ownership table is useful when it makes the argument clearer.
Evaluate whether the shape fits the problem and where future changes would land.
Include strengths with evidence; do not merely list the directory tree.

### 4. Close findings

For each consequential finding provide:

- a stable local ID and type: **defect**, **contract/documentation mismatch**,
  **architectural pressure**, **coverage gap**, or **open question**;
- impact/priority and confidence, separately (high consequence need not mean
  high confidence);
- exact source links with symbol/line references and the relevant requirement;
- trigger or counterexample, observable consequence, and whether reproduced,
  source-derived, or hypothetical;
- the smallest sensible response and a verification that would establish it.

Group by consequence, not file order. Related symptoms may share one finding.
If there are no substantiated defects, say so. Keep optional improvements and
future ideas out of the defect count.

### 5. Evidence and coverage

Explain which claims the checks establish and which remain source-only. Name
missing integration scenarios or uncertainty that could change the assessment.
Do not treat passing mocks as an end-to-end certificate.

### 6. Direction, not a rewrite mandate

Separate **preserve**, **fix/verify next**, **watch**, and **deliberately defer**.
Tie each proposal to a real pressure or declared possible future. Provide small
coherent slices and arrival evidence, not time estimates or a new architecture
because a refactor is possible. The parent retains architectural responsibility;
findings are not implementation authorization.

### 7. Reflection on this review form

What did the form make you inspect that you might otherwise miss? What was
ambiguous, costly, repetitive, or missing? Propose up to three concrete changes.
Evaluate the protocol from its use, not by writing a rival generic checklist.

## Citation and return contract

Use durable Markdown links: canonical repository URLs when resolvable, and
explicit `file://` local evidence for workspace-only sources. Include revision
and line/symbol context; a moving file link alone is not an immutable citation.
Use honest frontmatter and a model-suffixed filename. The parent and an Astra
child are distinct authors/runs even if they share a model family.

Commit only your assigned report with `jj commit -m "…" <exact-path>`, in the
artifact-owning repository. Do not commit peer reports, shared files, or target
code. No pushes, `bd prime`, or issue mutations. Return: your session ID, artifact
path, commit receipt, checks and limitations — **not findings**, so completion
notifications do not contaminate the parent's independent review.

## Comparison after the gate

The coordinator reads every report, checks load-bearing disagreements against
source, and compares coverage, evidence fidelity, architectural judgment,
actionability, calibration, and reading cost. Preserve unique useful findings
and disagreements. Do not vote truth into existence or infer general model
quality from one run. Publish both the product synthesis and the protocol
lessons, citing all attempts and explicit missing slots.
