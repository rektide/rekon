---
type: PromptAssessment
title: Did our review prompt serve the ask?
description: Parent self-assessment of the review prompt and its orchestration, grounded in all five available review attempts and the human's priority correction.
resource: /assess/review/prompt-assessment0.gpt6a.md
tags: [prompting, assessment, praxis, experiment, alignment, self-review]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
extensions:
  session: urn:opencode:session:ses_f872bf58affeS5IBlC38cA6iYX
  cohort: Parent plus Astra, GLM, Sol, Luna; Terra unavailable, no result claimed.
  posture: Self-assessment, not independent verification of the prompt author.
sources:
  - { resource: /assess/review/form0.gpt6a.md, title: Frozen prompt under assessment }
  - { resource: /assess/case-studies/lastact/wave0.gpt6a.md, title: Shared inputs and gate }
  - { resource: /assess/case-studies/lastact/review0-parent.gpt6a.md, title: Parent independent use }
  - { resource: /assess/case-studies/lastact/review0.gpt6a.md, title: Astra use and reflection }
  - { resource: /assess/case-studies/lastact/review0.glm53.md, title: GLM use and reflection }
  - { resource: /assess/case-studies/lastact/review0.gpt56s.md, title: Sol use and reflection }
  - { resource: /assess/case-studies/lastact/review0.gpt56l.md, title: Luna use and reflection }
---

# Outcome: useful review prompt, misweighted experiment

**The prompt produced useful architectural and close reviews. It did not give
evaluation of the prompt itself the prominence the human wanted.** This is the
most important assessment, not a footnote beneath the plugin findings.

The original request joined two purposes: understand a successful plugin, and
workshop a reusable assessment praxis through multiple independent attempts.
The later clarification explicitly prioritizes the second. I made “review the
plugin” the dominant assignment and appended a short reflection section. The
results follow that allocation remarkably closely. That is successful
instruction-following in service of an imperfectly prioritized commission.

This report was written after **all four dispatched reviewers returned** and
all reports were read. The parent's own review was committed before peer
reading. Terra was unavailable; this evaluates five supplied artifacts, not a
completed six-reviewer request or general model benchmark.

## How the ask performed

| Human purpose | What I asked the agents to do | Observed result | Assessment |
|---|---|---|---|
| High-level architecture **and** close review | Explain purpose, boundaries, ownership and host seams; supply concrete findings | Every report covers both; several trace contracts beyond the repo | Good direction and useful evidence. |
| Respect that the plugin works well | Treat success as testimony; preserve strengths; no manufactured rewrite | All preserve the core shape and identify bounded pressures | Appropriate posture, though the shared framing may also encourage consensus. |
| Workshop the review form | Final section, at most three proposed improvements | Concrete suggestions, but little space devoted to testing prompt quality | Underweighted relative to the clarified priority. |
| Independent run-off | Same form/baseline, separate artifacts, no peer reading | Four distinct reports and a separately saved parent report | Strong procedural alignment; correlated sources and author roles remain. |
| Gate comparison on all returning | Receipt-only completion notifications; read all before synthesis | All four peer returns arrived before comparative reading | The clearest successful praxis experiment. |
| Discussion before rules/action | Draft a practice and try it | I began provisional AGENTS edits before bringing the method back for discussion | Parent pacing/authorization error; not caused by a reviewer. |
| Beads literally last | Read-only tracker access, no mutations | Agents cited old records; no new records or naming contention | Mutation gate held, but narration failed to make existing versus new work obvious. |

## What the prompt reliably encouraged

**Architecture as an argument, not a directory tour.** Reviewers explain why
the TUI observation model and durable server recorder are different lifetimes,
why a shared universal tracker would be unnecessary, and why additive
session-bound surfaces fit the plugin. These are useful judgments, not just
headings filled with module names.

**Evidence at the seams.** The repeated high-value discoveries concern what
the host's timestamps actually prove, whether a copied interface can detect
host changes, what survives a TUI restart, and which operations happen before
successful user admission. The instruction to read load-bearing host contracts
did real work. Independent Astra also checked local-directory resolution rather
than assuming package exports satisfied the host's local loader.

**Restraint.** No reviewer rewrote the plugin, installed dependencies, launched
live prompt experiments, or filed tickets. The preserve/fix/watch/defer
distinction helped retain the successful architecture while questioning
specific guarantees. All reported 86 passing tests and typecheck success without
claiming that constituted complete integration acceptance—although some prose
still overstated individual checks.

**Uncontaminated returns.** Completion receipts carried artifact/session/check
identity instead of conclusions. This allowed the parent to remain independent
without needing to ignore findings already injected into its context. It is a
small procedural intervention with an observable effect.

## Where my prompting was weak

### 1. The experimental object was subordinate

Section 7 asks what the form helped inspect and requests up to three changes.
It does not explicitly ask: *Did this prompt serve the originating human ask?*
Nor does it require evidence for which instruction caused useful behavior,
what distorted priorities, or what a cheaper form would preserve.

A reproducible whitespace-word count, excluding frontmatter and trailing
reference definitions, shows the allocation:

| Run | Body words | Form-reflection words | Share |
|---|---:|---:|---:|
| Parent | 1,915 | 134 | 7.0% |
| Astra | 2,363 | 201 | 8.5% |
| GLM | 2,174 | 236 | 10.9% |
| Sol | 1,673 | 123 | 7.4% |
| Luna | 1,775 | 159 | 9.0% |

These are reading-allocation measurements, not token/cost or quality scores.
Across roughly 9,900 body words, prompt reflection occupies about 850. The
prompt itself—not agent reluctance—made meta-assessment secondary. Since the
human now values it more, the next brief should state that priority first and
give it its own substantial deliverable.

### 2. Identical prompts did not establish identical evidence conditions

All reviewers had to reconcile a pinned package, several source generations,
historical design notes, and the installed executable. The prompt said to
distinguish them but did not define the runtime being assessed or an evidence
matrix upfront. Considerable effort went into archaeology rather than the
intended comparative task.

This also affected calibration: Sol states a current-built-host failure more
confidently than source-to-binary provenance warrants; Astra more carefully
labels newer-source applicability. The same source argument can be useful
without establishing that the user's running plugin is broken. We needed a
per-finding **applicability** field, not just severity and confidence.

### 3. Asking for evidence did not prevent false assurance

GLM calls the local interface a strong drift catcher and says missed events
self-heal at the next tick. Parent, Astra and Sol identify why those claims are
too strong: the check compares local declarations, and settled-turn state
cannot recover a lost live-part timestamp. Luna's evidence section also says
typecheck establishes the handwritten host slice against installed declarations;
there is no mechanical connection doing that comparison.

These are not resolved by counting votes. The imported types, snapshot code,
and test setup settle the mechanism. The next form should explicitly ask which
**advertised safeguards are mechanically connected to what they claim to check**.

### 4. The existing-work permission needed sharper provenance

Read-only Beads access was useful for requirements and already-planned work,
but I did not require reviewers to distinguish a current live record from an
export snapshot. GLM consequently reported `oplast-time-shift-records` as
in progress from JSONL; a parent `bd list --all` read showed it **closed**.
GLM also reverses the empty-working-copy relationship in its baseline account.
These are concrete reporting mistakes, not ticket creation or a naming race.

The future prompt should say: **existing tickets may be evidence; report source
and observation date; do not infer current lifecycle from a stale export; no
ticket creation, reservation, renaming, claiming, notes, or closure in this wave.**

### 5. I conflated drafting guidance with staging it safely

I wrote provisional additions into `AGENTS.md` before presenting the comparison.
They were uncommitted, but that did not make them inactive: this harness reloads
the file. It was precisely the arrival/readiness/authorization distinction I
was trying to teach, violated by the coordinator's own editorial behavior.

I removed **only my additions**, preserving the pre-existing user push-policy
edit, and retained the proposed language in
[an ambient candidate](/assess/review/ambient-candidate0.gpt6a.md). This is the
right boundary for the workshop. The correction does not retroactively make
the initial choice good.

## Comparing the attempts as evidence about the prompt

| Run | Most useful contribution | Limitation exposed |
|---|---|---|
| Astra | Broadest close coverage; actual local-loader probe; distinguishes pinned contracts from newer-source failures; first-turn restart counterexample | Longest report, and substantial host archaeology. More evidence is not automatically a better use of the review budget. |
| Sol | Compact architectural account; concrete inventory-schema drift; subsequent-turn restart ambiguity | “Current built host” wording exceeds verified source-to-binary identity; source applicability should remain conditional. |
| Luna | Clear event-discriminant and failure-after-hook traces; good explanation of durable side effects | Calls admission atomicity a defect and proposes post-admission handling without resolving the requirement to appear *before* the prompt. Also overstates typecheck assurance. |
| GLM | Resists invented refactors; foregrounds availability coupling and snapshot coverage; explicitly distinguishes source generations | Too deferential to documentation's safeguards; misses counterexamples, understates live metadata refresh, reads stale ticket state. Suggested fallback-to-defaults can violate `enabled:false`. |
| Parent | Numeric coercion probe; evidence-provenance account; identifies existing resumption behavior and feature ambiguity | Missed the concrete local-loader and inventory-generation failures; protocol-author role is not comparable to a fresh reviewer; initially misprioritized the workshop itself. |

This is not an intrinsic ranking of models. There is one task, one attempt per
child, uneven investigative choices, shared prior documentation, and a parent
who authored the instructions. It shows **complementary behavior under this
prompt** and some recurring weaknesses the prompt did not prevent.

The disagreement over prompt admission is particularly informative. The source
supports a failed-attempt marker scenario. It does not alone settle whether the
product promises an attempt notice or an atomic successful-resumption notice.
A good review must distinguish discovering a mechanism from deciding its policy.
The next prompt should require a recommendation's **tradeoff or lost property**,
not only its proposed fix.

## The next prompt should lead differently

Proposed opening, **not yet a new frozen experiment**:

> We are evaluating a reusable assessment practice; Lastact is the case study.
> Produce a useful architectural and close review, then evaluate how this prompt
> shaped your work. The prompt assessment is at least as important as the product
> findings. Identify which instructions improved inquiry, which biased or wasted
> it, what was missed or left undecidable, and what the coordinator should change.
> Ground that account in your actual behavior and artifacts, not generic prompt
> advice. Do not treat useful product findings as proof that the prompt was optimal.

Keep the architectural/close lens, independent artifacts, receipts, and gate.
Add a compact host-basis matrix, per-finding applicability, explicit probe
permission, and a check that recommended fixes preserve the required behavior.
Use fewer repeated evidence disclaimers by giving each claim a clear evidence
home. Reserve substantive attention for the meta-assessment rather than a
three-bullet tail.

To learn whether this is *better*, use a later fresh case or fresh reviewers and
declare comparison criteria before dispatch. Re-prompting already informed
reviewers on the same code would test revision ability, not independent discovery.
Do not quietly initiate another large wave merely because a revised prompt can
be written.

## Workshop conclusion

The result is useful, appropriate in its product restraint, informative about
host seams, and directed well enough to produce complementary reviews. It is
**not yet well directed enough toward evaluating the practice itself**. The
next improvement is prioritization and sharper evidence obligations—not a
larger generic checklist.

The strongest new praxis distinction is:

> **Arrived does not mean understood; understood does not mean authorized.**

We demonstrated the first boundary through the completed wave and discovered a
coordinator mistake at the second. That is better material for improving Rekon
than pretending the first draft of the rules was already the finished practice.
