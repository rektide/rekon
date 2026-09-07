---
type: ProjectOverview
title: Rekon — understanding that can act and learn
description: A workspace for cumulative technical understanding, action practices, and the tools that deliver them.
resource: /README.md
tags: [rekon, practices, agents, knowledge, evidence, phoenix]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
extensions:
  ticket: rekon-phoenix
sources:
  - { resource: /constitution/reassessment0.gpt6a.md, title: Coming up for air }
  - { resource: /design/rekon/phoenix0.gpt6a.md, title: Phoenix inquiry and primary-source reading }
  - { resource: /evidence/case-stories/nan-lastact/genesis0.gpt6a.md, title: NAN and Lastact action-practice lineage }
  - { resource: /rekon.ts, title: Existing CLI command registry }
---

<a id="rekon"></a>
# Rekon

**Make technical work cumulative: understanding becomes useful action, action
leaves inspectable evidence, and what we learn can change the course.**

Rekon holds practices, research, references, and tools for working with people
and agents. It is becoming a home not just for instructions about writing code,
but for actions around an effort: finding its present state, understanding its
architecture, revisiting its direction, preparing coherent changes, and learning
whether those actions helped.

The emerging Phoenix inquiry asks what should survive when we replace a prompt,
workflow, tool, or implementation: intent, promises, evaluations, reasons,
boundaries, and provenance. **The work anchor is the Rekon Beads epic
`rekon-phoenix`** (`bd show rekon-phoenix`). Its notes retain the session,
artifact, and commit lineage; these documents explain the work, not a second
ticket lifecycle. The direction remains exploratory—not a project rename,
accepted new constitution, or implemented regeneration platform.

<a id="rekon-actions"></a>
## Actions around an effort

These names identify useful kinds of action, not a fixed taxonomy of agents or
a pipeline every task must traverse. One person or agent may perform several;
one action may use a bounded group of agents. A practice defines what useful
help looks like before prescribing a particular prompt or orchestration.

| When the need is… | Practice | What it should make possible |
|---|---|---|
| “Where are we now, and what is this becoming?” | [Digest](/digest/README.md) | A recency-aware architectural account, practical differences, and active intent fronts that equip steering or renewed shipping. |
| “Does this fit its purpose, and what earns confidence?” | [Assess](/assess/README.md) | Architectural and close judgment, strengths to preserve, scoped findings, and evidence sufficient for the next decision. |
| “How should this have been built and landed, knowing what we know now?” | [Remake](/remake/README.md) | A defensible sequence of coherent changes, with assessment of each consequential intermediate state. |
| “What should a historically informed implementation become now?” | [Reconstruct](/code/task-reconstruction/philosophy.md) | A chosen work mode and destination, retaining verified outcomes without inheriting accidental mechanisms. Actual implementation needs authorization. |

These are the kinds of tasks and **“en-shippenings”** we have been developing:
actions that help an effort regain direction, reach a useful result, or learn
why the previous action missed. New action homes are **draft guidance**, not
new CLI subcommands or automatically installed skills.

<a id="rekon-evidence"></a>
## Evidence and case stories

[Evidence](/evidence/README.md) is a role a reference plays in a judgment. Naming
makes a source addressable; provenance helps locate its origin; the evidentiary
argument says what it supports or challenges, under which conditions. The same
artifact can play several roles. Its location does not certify its truth.

We want **case stories**, not only polished success reports: what help was
expected, how an action was commissioned, what actually happened, where it
aligned or missed, and what we should try differently. This is how the practices
can improve through use rather than merely accumulate rules.

The [NAN–Lastact association](/evidence/case-stories/nan-lastact/genesis0.gpt6a.md)
is an initial thread:

- **Lastact** exercised architectural assessment and revealed, in the author's
  [prompt self-assessment](/assess/review/prompt-assessment0.gpt6a.md), useful
  reviews but insufficient emphasis on evaluating the practice itself.
- **NAN** supplied this session's question about a mature effort's landing
  sequence. The session moved from `nan-rs` to Rekon to define the practice;
  applying it to NAN remains deferred. This is Rekon-owned work.

The sources stay with their owners. `evidence/` connects and explains; it is
not a mandate to relocate every report, conversation, test, or ticket.

<a id="rekon-phoenix-direction"></a>
## Toward Phoenix

[Rekon as Phoenix](/design/rekon/phoenix0.gpt6a.md) samples eight essays from
[Chad Fowler's Phoenix Architecture](https://chadfowler.com/regenerative-software/).
The useful vocabulary includes regenerative grain, conservation layers,
conceptual mass, provenance debt, and evidence decay. The later writing treats
durable knowledge as connected and heterogeneous—not one master specification.

For Rekon, the question is whether we can preserve enough understanding to
change our expressions safely. Practices and tools should be replaceable;
their lessons should not vanish with them. A graph store, automated evaluator,
or regeneration engine is neither required by this first step nor implemented
by these documents.

<a id="rekon-map"></a>
## Other useful starting points

- [Constitution](/constitution/README.md) and its [reading map](/constitution/index.md)
  — knowledge ownership, identity, references, and current versus proposed guidance.
- [Coding practice](/code/README.md) — commits, review handling, planning, and posture.
- [Upstream reference mirrors](/upstream/README.md) — inactive vendor snapshots,
  provenance, and deliberate refresh of independently maintained adaptations.
- [Skillification](/design/skillify/README1.sol56x.md) — the proposal for turning
  knowledge into scoped, activatable capability; not a universal deployed contract.
- [AGENTS assembly](/constitution/agents-assembly.md) — implemented fragment
  assembly and its separately governed production cutover.
- [jj and Git](/vcs/jj/README.md), [JavaScript](/js/README.md), and
  [ATProto](/atproto/README.md) — domain references and operating practices.

<a id="rekon-tooling"></a>
## Existing tooling

The TypeScript CLI lives in [`rekon.ts`](/rekon.ts). Its registered commands
are `agents`, `combine`, `install-commands`, `interpolate`, `project-files`, and
`ln`; their implementations live in [`src/command/`](/src/command/). The
completion plugin also supplies `complete`.

During development, run the source directly:

```sh
node rekon.ts --help
```

The tooling and the knowledge/practice corpus are related but not identical.
A readable practice directory does not by itself install an agent or promise
that a harness can discover and execute it.

## Existing tooling TODOs

- speed-tune, adjusting model execution params
- pull in, expand scrape-variants from compfuzor/llama-cpp
