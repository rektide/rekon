---
type: ResearchOrientation
title: Rekon praxis and tooling inquiry — reading the results
description: Status-aware entry to the lineage and tooling reports, with their verification qualifications kept visible.
resource: /assess/rekon/README.md
tags: [rekon, praxis, lineage, tooling, verification]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
sources:
  - { resource: /assess/rekon/lineage0.gpt6a.md, title: Astra lineage inquiry }
  - { resource: /assess/rekon/tooling0.glm53.md, title: GLM tooling inquiry }
  - { resource: /assess/rekon/verification0.glm53.md, title: Targeted verification and qualifications }
---

# Read the results with their qualifications

This inquiry accompanies the [assessment-practice workshop](/assess/README.md).
It is not an adopted architecture, a new work graph, or authorization to install
tools. **No tickets were mutated by these assignments.**

## Lineage

[Astra's lineage report](/assess/rekon/lineage0.gpt6a.md) locates the earlier
September 6 stewardship conversation in **DL**, not Rekon:
`ses_f876a85afffeeKH82rtZMoWGsx`. User corrections there produced the stewardship,
implementation-review, and cooperative-exploration commits `80bcfdc6`,
`852ee56a`, and `cf036c5a`. This session,
`ses_f872bf58affeS5IBlC38cA6iYX`, began in Lastact and moved to Rekon.

The report's useful interpretation is **cumulative work without compulsory
inheritance of old mechanisms**. These conversations are a connected episode,
not one session or an invented chain of blocking dependencies. Exact native
messages, commits, and earlier documents make that interpretation inspectable.

## Tooling

Read [the capability map](/assess/rekon/tooling0.glm53.md) **with**
[its verification report](/assess/rekon/verification0.glm53.md). Do not carry
the original's unqualified negative/completeness claims into plans:

| Question | Supported account after verification |
|---|---|
| ACP `--server` | A real attachment gap in the inspected V2 handler and installed help: ACP starts a private server instead of accepting the normal server-selection flags. |
| Private process and storage | Private process does **not** prove isolated data/config. Source uses common resolution under matching channel/environment. Live concurrent behavior was not verified. |
| Agent session self-knowledge | ACP replies return the native ID; V2 also puts it in model context. A subprocess `$OPENCODE_SESSION_ID` is a separate plugin-provided capability. |
| Ticket history | `bd history` provides committed issue-row snapshots, not the complete history of comments, relationships, labels, or uncommitted work. A fuller changelog needs those distinctions. |
| hyalo | Substantial deterministic Markdown/OKF maintenance already exists upstream, including index/log generation; it is not semantic summarization and was not installed here. |
| Tooling home | `summarizer` has the ACP spike and larger design; `sum-pub`/`pubsum` has a working EPUB/session loop. The relationship is real; consolidation remains a decision. |

The existing [summarizer/hyalo juncture](file:///home/rektide/src/summarizer/doc/discovery/hyalo-juncture.md)
already explains the semantic-production versus deterministic-maintenance
boundary. It is stronger prior art than inventing that division again here.
Rekon's [assembler](/constitution/agents-assembly.md) also exists: the broader
tooling future is incomplete, not tooling-free.

## How this bears on prompting

[The prompt self-assessment addendum](/assess/review/prompt-assessment0.gpt6a.md#addendum--verification-tests-a-different-prompt)
explains the methodological result: a generic demand for evidence did not
prevent overbroad inferences. Targeted verification of specific implications
did. Keep the original artifacts as evidence and this qualification map as the
route through them; do not silently rewrite the first report into an appearance
of having been right initially.

The checker also found twelve host-absolute `/home/…` Markdown targets in the
original tooling report. They refer to real local files but are not portable
bundle links; use the verified report's `file://` references for local follow-up.
No corpus-wide link migration was performed.
