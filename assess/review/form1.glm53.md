---
type: ReviewProtocol
title: Architectural and close review — form 1
description: Reusable shared instructions for architectural assessment and close review, revised from form 0 using the Lastact cohort's five reflections.
resource: /assess/review/form1.glm53.md
tags: [review, architecture, correctness, evidence, protocol]
status: draft
generated: { by: "model:glm-5.3", at: 2026-09-06 }
sources:
  - { resource: /assess/review/form0.gpt6a.md, title: Form 0 — frozen predecessor and Lastact shared protocol }
  - { resource: /assess/case-studies/lastact/wave0.gpt6a.md, title: Lastact wave assignment and gate }
  - { resource: /assess/case-studies/lastact/review0.gpt6a.md, title: Astra reflection (§7) }
  - { resource: /assess/case-studies/lastact/review0.glm53.md, title: GLM reflection (§7) }
  - { resource: /assess/case-studies/lastact/review0.gpt56s.md, title: Sol reflection (§7) }
  - { resource: /assess/case-studies/lastact/review0.gpt56l.md, title: Luna reflection (§7) }
  - { resource: /assess/case-studies/lastact/review0-parent.gpt6a.md, title: Parent reflection (§7) }
  - { resource: /AGENTS.md, title: Architectural stewardship }
---

# Architectural and close review — form 1

## Changes from form 0

[Form 0](/assess/review/form0.gpt6a.md) stays frozen as the protocol the
Lastact cohort actually used. Form 1 revises it from the five §7 reflections:

- **Host-basis matrix and resolution** (Astra, GLM, Sol, Luna, parent): the
  assignment must supply dependency pins, source-checkout revisions, and live
  runtime identity up front; §2 must record them in a table and state, per
  load-bearing seam, which artifact proves the behavior and which generation
  is authoritative when they disagree. Cross-generation differences are
  compatibility findings, not baseline defects.
- **Structural evidence tables** (GLM, parent, Sol): §5 carries a required
  claim-to-evidence table with testimony/test/source provenance, including
  load-bearing documentation promises; §4 may compress lower-consequence
  findings into a compact table instead of repeated prose.
- **Seam map before lenses** (parent): identify where tests substitute for the
  real boundary early, then spend close-inspection effort there.
- **Harness probes** (GLM): import-only scratch diagnostics that start no host
  process and install nothing are a named allowed category, so seam checks are
  not orphaned between "existing tests" and forbidden experiments.
- **Recovery and admission lenses** (Astra, Luna): one cold/restart trace for
  observational or stateful systems; for each durable side effect, whether it
  lands before, during, or after the host's commit, with a
  failure-after-hook counterexample where applicable; when a finding turns on
  event or message discrimination, name the discriminant fields inspected.
- **Mechanical connection** (Sol): a claimed compatibility guard must be shown
  mechanically connected to upstream, not merely copied from it.
- **Budgets and variance** (parent): the assignment declares a review budget;
  the coordinator records variance in budgets, host bases, and word counts
  rather than reading one run as intrinsic model quality.

## Commission

Review **the system as it stands**, unless the assignment explicitly supplies a
change range. The user believes it fulfills its purpose and works well. Treat
that as valuable operational testimony, not proof of every behavior. Determine
whether the implementation fits its intent, what earns confidence, what limits
it, and which small next decisions would matter. Do not manufacture a redesign.

The assignment must name the target repository and immutable baseline, scope
(whole system or exact diff), intent sources, artifact destination, allowed
checks, independence/completion gate, and a review budget (effort and word
bounds). It must also supply the **host basis** as inputs, not leave reviewers
to do the archaeology: dependency pins, the source checkouts they resolve to,
the live runtime/build identity actually running, and which historical
comparisons are permitted. Record missing inputs rather than quietly filling
them with invented requirements.

## Conduct

1. Read the intent and explain the system in your own terms. Then inspect the
   implementation, tests, and load-bearing upstream contracts directly.
2. Build a **seam map** before walking lenses: where do tests substitute for
   real boundaries (mocks, fakes, harness shims), and which seams have no
   substitute at all? Spend close-inspection effort there; a generic lens list
   is less useful than this map.
3. Follow important paths across boundaries: inputs/configuration, state and
   ownership, external operations, output, restart/recovery, and disposal.
   This is a lens set, not a demand to invent a finding per lens. Two lenses
   earn explicit work: for stateful or observational systems, reconstruct one
   **cold-start/restart trace** showing observable state before and after loss
   of process state; for durable side effects, decide whether each lands
   before, during, or after the host's commit, and reconstruct one
   failure-after-hook counterexample where applicable.
4. Resolve the host basis per seam. Distinguish published docs, installed
   declarations, source checkouts, and live runtime observation. For each
   load-bearing seam, name which artifact proves the behavior; when
   generations disagree, say which is authoritative for the pinned baseline
   and treat the difference as a compatibility question. A type assertion or
   mock cannot prove a host behavior by itself.
5. Verify that each claimed compatibility guard is **mechanically connected**
   to the upstream it guards against, not merely copied from its source
   snapshot.
6. Tests and tooling support judgment; report commands, results, and what they
   do not cover. Allowed diagnostics include **harness probes**: import-only
   scratch runs in your own ignored directory that never start a host process
   or install dependencies. Do not reinstall dependencies, mutate production
   state, run a live-session experiment, or fix source during review.
7. Keep hard spec failures, documented-standard violations, architectural
   judgments, and hypothetical risks distinguishable, and mark each finding's
   **applicability**: baseline defect (reproduced or derived against the
   pinned generation) or compatibility finding (arising against a different
   generation). Cite a rule before calling something a rule violation. A smell
   is a heuristic, not a verdict. Do not waste the report restating stylistic
   failures already covered by tooling.
8. Preserve independence: no peer reports, peer summaries, or synthesis until
   the declared wave gate opens. Return only an artifact/session/commit
   receipt to the coordinator; put findings in the artifact.

## Markdown deliverable

Aim for a focused report of roughly 1,200–2,200 words; use more only when the
evidence earns it. Use these headings so reports can be compared without forcing
identical conclusions:

### 1. Assessment in brief

State the overall architectural judgment and the most important threads. Say
what should be preserved, what needs attention, and how confident you are.

### 2. Basis and boundaries

Record repository, revision, reviewed scope, intent sources, actual
model/session identity (or uncertainty), checks, and exclusions. Include a
**host-basis table** with (at least) installed declaration, source checkout,
and live runtime columns, and a **runtime-under-review** line naming what was
actually running — distinct from dependency pins. State the per-seam
authority resolution for seams where generations disagree. Whole-system review
needs no artificial diff base. Distinguish user testimony from tests and your
own source reasoning.

### 3. Architectural account

Explain purpose and non-goals, main domain boundaries, data/control flow, state
owners and lifetimes, and the seams that couple the implementation to its host.
A small diagram or ownership table is useful when it makes the argument
clearer. Evaluate whether the shape fits the problem and where future changes
would land. Include strengths with evidence; do not merely list the directory
tree.

### 4. Close findings

For each consequential finding provide:

- a stable local ID and type: **defect**, **contract/documentation mismatch**,
  **architectural pressure**, **coverage gap**, or **open question**;
- **applicability**: baseline defect or compatibility finding, and against
  which generation;
- impact/priority and confidence, separately (high consequence need not mean
  high confidence);
- exact source links with symbol/line references and the relevant requirement;
  when the finding turns on event or message discrimination, name the
  discriminant fields inspected, not only the event type;
- trigger or counterexample, observable consequence, and whether reproduced,
  source-derived, or hypothetical;
- the smallest sensible response and a verification that would establish it.

Group by consequence, not file order. Related symptoms may share one finding.
Lower-consequence items may be compressed into a compact table (ID, type,
applicability, impact/confidence, link) instead of repeated prose. If there
are no substantiated defects, say so. Keep optional improvements and future
ideas out of the defect count.

### 5. Evidence and coverage

Carry a **claim-to-evidence table** for load-bearing claims — including
promises made in documentation and names that promise behavior ("drift
catcher" deserves auditing as much as a config flag) — with three provenance
columns: testimony, test, source. Explain which claims the checks establish
and which remain source-only. Name missing integration scenarios or
uncertainty that could change the assessment. Do not treat passing mocks as an
end-to-end certificate.

### 6. Direction, not a rewrite mandate

Separate **preserve**, **fix/verify next**, **watch**, and **deliberately
defer**. Tie each proposal to a real pressure or declared possible future.
Provide small coherent slices and arrival evidence, not time estimates or a
new architecture because a refactor is possible. The parent retains
architectural responsibility; findings are not implementation authorization.

### 7. Reflection on this review form

What did the form make you inspect that you might otherwise miss? What was
ambiguous, costly, repetitive, or missing? Propose up to three concrete
changes, and say whether form 1's additions (host-basis table, seam map,
harness probes, applicability) earned their cost in this run. Evaluate the
protocol from its use, not by writing a rival generic checklist.

## Citation and return contract

Use durable Markdown links: canonical repository URLs when resolvable, and
explicit `file://` local evidence for workspace-only sources. Include revision
and line/symbol context; a moving file link alone is not an immutable citation.
Use honest frontmatter and a model-suffixed filename. Distinct agents are
distinct authors/runs even when they share a model family.

Commit only your assigned report with `jj commit -m "…" <exact-path>`, in the
artifact-owning repository. Do not commit peer reports, shared files, or target
code. No pushes, `bd prime`, or issue mutations. Return: your session ID,
artifact path, commit receipt, checks and limitations — **not findings**, so
completion notifications do not contaminate the parent's independent review.

## Comparison after the gate

The coordinator reads every report, checks load-bearing disagreements against
source, and compares coverage, evidence fidelity, architectural judgment,
actionability, calibration, and reading cost. Record variance in review
budgets, host bases actually used, and word counts alongside the comparison;
reviewer-selected differences confound a run-off even under an identical
prompt, and one run per reviewer does not measure intrinsic model quality.
Preserve unique useful findings and disagreements. Do not vote truth into
existence. Publish both the product synthesis and the protocol lessons, citing
all attempts and explicit missing slots.
