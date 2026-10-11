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

The [naming and constitutional work](/constitution/index.md) is the north star
this grows from: wiki-native names, native owners, explicit relations, and
understanding that survives changes in its expression. Phoenix and the newer
action vocabulary extend that work; they do not replace its recorded rulings.

<a id="rekon-sense-map"></a>
## Terms above the fold

| Terms | Their relationship |
|---|---|
| **Name → reference → home** | A scoped identity can be reached through different references; its maintained home explains the concern and where its evidence lives. |
| **Actor → hat / role** | An actor may have concern-specific aliases and scoped responsibilities without becoming a new actor each time. |
| **Effort → intent front → action** | A continuing pursuit develops directions; a bounded action should help one of them. A practice supplies a repeatable way to do so. |
| **Work anchor ↔ knowledge** | The native ticket holds work scope and lifecycle; documents hold arguments and connect the work to its sources. |
| **Claim ↔ evidence ↔ provenance** | Evidence bears on a claim under stated conditions; provenance explains the source and how it arose. |
| **Home → exposure → activation** | A body of knowledge may be read, loaded, or supplied ambiently through different surfaces. Having a surface is not proof it was used. |
| **Honing → promotion** | We refine candidates before deciding their standing. Repetition may be useful while honing; actual activation is a separate fact. |

The [working glossary](/CONTEXT.md) gives these words short, linked definitions.
The [constitution's GLOBAL](/constitution/GLOBAL.md) deliberately
repeats their operating significance for a reader who has not loaded a skill.
The [manual weave](/design/rekon/context-weave/north-star0.gpt6a.md) informed the
current [manifest-selected assembly](/tool/assembly/README.md). `AGENTS.md` is now
generated; its predecessor lives at `AGENTS.old.md`. Run `pnpm check` to check
both output freshness and [operating coverage](/tool/assembly/coverage.md).
The October 11 repair restored advice lost at cutover, including
[beads IDs and commands](/praxis/beads/README.md).

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
| “Having built the vertical, which architectural wins and new possibilities can we pursue?” | [Post-build](/praxis/post-build/README.md) | Shared guarantees worth strengthening, promising new offerings, and a few evidence-grounded next investments. |
| “How should this have been built and landed, knowing what we know now?” | [Remake](/remake/README.md) | A defensible sequence of coherent changes, with assessment of each consequential intermediate state. |
| “What should a historically informed implementation become now?” | [Reconstruct](/code/task-reconstruction/philosophy.md) | A chosen work mode and destination, retaining verified outcomes without inheriting accidental mechanisms. Actual implementation needs authorization. |
| “What is our upstream actually about to do, and which of it matters to us?” | [Forge](/forge/README.md) | A provenance-marked corpus of the dependency's issues, PRs, discussions, and verified roles, triaged into real movement versus ideas, so downstream choices cite upstream evidence instead of rumor. |

These are the kinds of tasks and **“en-shippenings”** we have been developing:
actions that help an effort regain direction, reach a useful result, or learn
why the previous action missed. New action homes are **draft guidance**, not
new CLI subcommands or automatically installed skills.

[Chase the future](/praxis/README.md) connects these actions through a shared
praxis: build from current upstream possibilities, choose opportunity-opening
slices, and preserve alternate courses without turning them all into scope.

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

- [Rekon's structure](/constitution/structure.md) — where concerns live, how
  to place new explanations, and the [short codes](/constitution/naming/README.md#short-codes)
  used to name them. [Tool toolkit (`t`)](/tool/README.md) brings top-level
  executable entrypoints, documentation, and shared domain-grouped source into
  one physical `tool/` home. `g`, `p`, and `t` are shortcodes, not directories.
- [Constitution](/constitution/README.md) and its [reading map](/constitution/index.md)
  — knowledge ownership, identity, references, and current versus proposed guidance.
- [Coding practice](/code/README.md) — commits, review handling, planning, and posture.
- [References](/refs/README.md) — archive-backed knowledge and our curated skill
  index; [skill curation](/refs/skills.md) owns selection and composition.
- [Open Knowledge Format](/okf/README.md) — trustworthy knowledge labels;
  [source citations](/refs/sources.md) preserve their grounds and native origins.
- [Upstream reference mirrors](/upstream/README.md) — inactive vendor snapshots,
  provenance, and deliberate refresh of independently maintained adaptations.
- [Skillification](/design/skillify/README1.sol56x.md) — the proposal for turning
  knowledge into scoped, activatable capability; not a universal deployed contract.
- [Session mementos](/design/session-mementos/README.md) — multi-session terminal
  epilogues, explicit summary marks and agent marking tools; evolving capability
  tickets and source-cited designs, not a shipped bookmark system.
- [Ordered capability pipelines](/design/ordered-pipelines/README.md) — a memento
  of reusable rule assembly and termination authority, preserving the differences
  between policy, stream filtering and accumulating output.
- [AGENTS assembly](/tool/assembly/README.md) — the new Effect CLI tool,
  with its [legacy Gunshi reference](/constitution/agents-assembly.md) and
  separately governed production cutover.
- [jj and Git](/vcs/jj/README.md), [JavaScript](/js/README.md), and
  [ATProto](/atproto/README.md) — domain references and operating practices.

<a id="rekon-tooling"></a>
## Existing tooling

The shared [toolkit](/tool/README.md) has its first new Effect CLI entrypoint:

```sh
node tool/agents.ts --help
```

It assembles declared `GLOBAL.md` fragments and checks generated output; the
[assembly home](/tool/assembly/README.md) co-locates its contract with its
implementation. The original TypeScript CLI remains in [`rekon.ts`](/rekon.ts).
Its registered commands
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
