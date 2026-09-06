---
type: AssessmentExperiment
title: Lastact review wave 0 — fixed inputs and completion gate
description: Assignment and evidence boundary for comparing independent reviews, not a second issue tracker.
resource: /assess/case-studies/lastact/wave0.gpt6a.md
tags: [lastact, review, wave, experiment]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
---

# Fixed inputs

- **Question:** Does the successfully used Lastact plugin have an architecture
  that fits its purpose, and what does close inspection establish or challenge?
  How well does the shared assessment form elicit useful answers?
- **Target:** `/home/rektide/src/opencode-lastact`, whole current system; clean
  working copy at launch. Immutable baseline commit `43515611` (unique Git
  commit prefix), parent of empty working copy `d0db0f7a` at initial inspection.
- **Shared form:** [form0](/assess/review/form0.gpt6a.md), frozen before dispatch.
- **Intent:** target `README.md`, target `.design/settled-clock/` prior design,
  existing target Beads records if useful (read-only), and user testimony that
  the plugin works as desired. No new feature requirement is part of this
  baseline review.
- **Host basis:** target package pins `@opencode-ai/plugin` and schema to
  `0.0.0-dev-18695`; inspect installed contracts and distinguish them from local
  OpenCode checkouts and current V2 documentation. Archive-first source root:
  `/home/rektide/archive/anomalyco/opencode`; first-party host checkout:
  `/home/rektide/src/opencode` and variants where evidence warrants.
- **Allowed verification:** read-only source/history inspection and existing
  `pnpm test` / `pnpm run check:type`; isolated diagnostics only in an owned
  `.test-agent/assess-<reviewer>/` directory. No product changes, installs, live
  prompt injection, issue mutation, or pushes.

## Cohort and artifact ownership

| Requested participant | Dispatch | Artifact under this directory |
|---|---|---|
| Parent independent review | Current session, user-selected Astra label | `review0-parent.gpt6a.md` |
| Astra | `astra-xhigh-gpt` | `review0.gpt6a.md` |
| GLM | `glm-max-z` | `review0.glm53.md` |
| Sol | `sol-xhigh-gpt` | `review0.gpt56s.md` |
| Luna | `luna-xhigh-gpt` | `review0.gpt56l.md` |
| Terra | **Unavailable in supplied agent catalog; not dispatched or substituted** | No result claimed |

Model names are dispatch/user labels, not independently attested identities.
If an agent knows a more accurate underlying identity it must say so in its
report. One run per reviewer is an experiment in this form, not a general model
benchmark. The parent authored the protocol; that author-role difference is a
comparison limitation even before any peer reports are read.

## Gate

The parent saves its independent review before opening any peer artifact.
Agents return receipts only, not findings. All four dispatched reviewers must
resolve successfully and supply their committed reports before peer reading;
if one fails, retain the gate while retry/replacement or an explicit exception
is decided. Then read **all** supplied reports before comparative analysis or
reaction. Do not start product corrections while collecting the wave.

This is a complete gate over the **runnable cohort**, not a claim to fulfill the
unavailable Terra slot. The broader requested six-reviewer comparison remains
qualified unless that slot becomes available or the human waives it. The
coordinator may publish a clearly labeled five-review assessment (including
parent), preserving the Terra gap rather than fabricating an attempt.

The separately commissioned Rekon lineage/tooling inquiry has its own two-input
gate. Product assessment does not await unrelated research; broader Rekon
synthesis does. Product feature work and the final architectural ticket pass
remain behind the relevant assessment decisions.

## Comparison to publish

Use all attempts. Compare architectural explanation, close coverage, evidence
accuracy, confidence calibration, actionability, and reading cost. Reconcile
specific disagreements against source; list source-derived but untested risks
honestly. Report unique contributions and weaknesses without selecting truth by
vote. Preserve this frozen form; write a new version if the experiment changes
the method. The case study should feed back into the small ambient guidance,
not install its entire report as AGENTS context.
