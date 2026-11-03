---
type: Flight
title: Random-combo — weighted draws for independent exploration
description: A reusable coordination model that draws an ordered set of options per game with replacement — three slots by default, `--slots N` for other counts — preserves persistent weight decay, and joins independent game results before comparison.
resource: /praxis/flights/random-combo.md
status: draft
generated: { by: "model:openai/gpt-6.1-sol#high", at: 2026-11-02 }
tags: [praxis, flights, random-combo, exploration, independence]
sources:
  - { resource: "opencode://ses_e60122a28ffem0Q4yFFw0qqZEM", title: Generalize the draw to N slots with the triple as default and add the generator flag }
  - { resource: "opencode://ses_e6172cb39ffe5Q3YzyRjA9H32n", title: Implement the commissioned reusable random-combo flight and its reproducible Node draw stage }
  - { resource: /praxis/flights/README.md, title: Frame, bounded spawns, explicit joins, independence, and terminal dispositions }
  - { resource: /praxis/parallax/README.md, title: Complementary assignments are not independent replications or a model benchmark }
  - { resource: /assess/gates/README.md, title: Distinguish full-cohort arrival from evidence readiness before comparison }
  - { resource: /tool/random-combo/README.md, title: Inline-weighted input grammar, reproducible draws, immediate persistent decay, and audit events }
---

# Random-combo flight

**Purpose:** explore a question through unusual combinations of arbitrary
options, with a reproducible draw record and independent game responses before
comparison. Options may be lenses, methods, constraints, themes, or other
human-supplied ingredients; this flight does not prescribe a domain vocabulary.
Every **game** receives an ordered draw from the same pool: three slots by
default — the usual triple — with the generator's `--slots N` opening other
counts when the frame calls for them.

This is a reusable **coordination model**, not an implementation of a scheduler
or permission to delegate. The parent coordinates actual promises in the harness;
models, participants, spending, and implementation still require their proper
authorization. Random selection does not choose what is true or accepted.
The [flights home](/praxis/flights/README.md) owns the general practice.

## Frame

Record the common question, intended decision, baseline sources/revisions,
constraints, permitted action, number of games, and authorized participants.
Supply one arbitrary labeled option per line with an explicit inline weight:

```text
careful exploration w=1
unexpected perspective w=0.1
small experiment w=0.5
```

Use the ⇄ [Node generator contract](/tool/random-combo/README.md) for the full
grammar, zero/invalid weight policy, seed and replay, output, and numeric limits.
State what applying a repeated option means in this task: usually extra emphasis
or distinct applications of the same lens, **not** a demand to invent a different
option. Preserve slot identity even when labels repeat.

Choose a seed or keep the printed generated seed. Preserve ordered input and
the generator revision alongside the transcript; name where game artifacts and
the final comparison will live. A frame can authorize only drawing combinations,
only equipping a choice, or a wider exploration. Do not infer the wider authority.

## Spawn and join map

| Promise | Bounded assignment and return | Dependencies / release |
|---|---|---|
| **Draw record** | Parent or authorized draw keeper validates the pool and runs all games as **one sequential generator run**; returns input, seed, command/revision, full audit, exit status, and ordered triples. This need not spawn another agent. | Requires the frame. The draw join releases only after successful exit and a complete transcript with the final event (or its text equivalent). |
| **Game response `g`** | An authorized participant applies game `g`'s exact ordered triple to the common question. Returns a uniquely owned artifact: slot mapping including repeats, approach, result, evidence, assumptions, limitations, and a concrete candidate next action. | Spawn after the draw join. All games share the same baseline; sibling game artifacts remain unread until the game join. Different games can execute concurrently. |
| **Comparison** | Parent or separately authorized synthesizer reads all required game artifacts and the audit, compares useful results and tensions, traces claims to sources, and returns options for the framed decision. | The all-of game join requires every framed game to have a successful, sufficient artifact. An arrival is not automatically sufficient; inspect before releasing. |
| **Decision / continuation** | Human or scoped owner selects, rejects, asks for more evidence, or authorizes a next action. Preserve disagreement, provenance, and the actual outcome. | Requires comparison unless the human explicitly changes the frame. Completion and a compelling combination are not acceptance. |

There are two different orderings: **draws are sequential** because weights
persist; **game responses may be independent and concurrent** once the full draw
record is fixed. Do not generate each game in a new process, reset weights between
games, or draw the game's options independently from their original weights.

For example, from the Rekon checkout:

```sh
node tool/random-combo.ts --games 4 --seed inquiry-0 --jsonl tool/random-combo/example.txt
```

Capture stdout in the effort's deliberately chosen evidence location; no output
path is created automatically. This command only draws options: it never invokes
agents or authorizes an assignment. The example pool is illustrative, not a
recommended or required vocabulary.

## Sampling invariant

For each slot in each game:

1. Sample with probability proportional to **current** weights.
2. Keep the winner eligible: sampling is **with replacement**.
3. Immediately replace only the winning weight with `current weight × 0.8`.
4. Carry that updated pool into the next slot, and then the next game.

The same option may occupy several or all slots. One positive option is a
valid pool. Repeats are neither missing results nor a reason to redraw. Weights
express selection preference, not confidence, importance of evidence, or merit
of an eventual response. Decay spreads opportunity without guaranteeing diversity.

## Independence and useful waiting work

Each game participant may read the common frame, baseline, and assigned triple,
but must not read sibling same-wave responses before the join. Where practical,
send only its assigned triple rather than others' proposed approaches. Shared
source material and a repeated option are not independence violations. Use
distinct owned artifact paths; keep sources and actual session provenance.
Different triples are **complementary assignments**, not independent replications
or a model benchmark: [parallax](/praxis/parallax/README.md#two-useful-modes)
explains why the actual framing differences matter to comparison.

While games run, the parent can check transcript integrity, prepare comparison
criteria, and gather unrelated baseline evidence. It must not feed an early
response into remaining games or prematurely publish the comparative result.
Wait for the full cohort's arrival before reading its results for readiness;
[evidence gates](/assess/gates/README.md#release-deliberately) separates that
administrative arrival from a justified release to comparison.
If useful independent work is exhausted, yield and use completion notifications;
do not busy-wait or pretend that unfinished promises have arrived.

## Failure and changed frames

- **Invalid input or incomplete draw record:** no game dispatch. Correct the
  input or numeric/run limits, preserve the failed disposition, then record a
  new complete draw record. A changed pool or seed is a changed instance, not an
  invisible repair. A partial transcript does not satisfy the draw join.
- **Game failure, interruption, provider limit, or insufficient evidence:** record
  the exact game/slots, artifact revision if any, disposition, and consequence.
  The game join stays closed. Retry or replace only within authorized scope,
  preserving the same triple and baseline rather than redrawing for convenience.
- **Partial release:** only an explicit human/owner change can reduce the required
  cohort. Record the omitted games and narrowed claim; do not describe a partial
  comparison as complete. A late result needs an addendum or new comparison.
- **Another batch:** choose deliberately whether it is a fresh run from the
  original weights or a separately framed continuation using recorded final
  weights. The CLI does not automatically resume state or retain weights between
  processes. Do not silently mix reset and continuous-decay evidence.

## Carry forward and learn

Keep the frame, ordered weighted pool, seed/algorithm/revision, audit, dispatch
mapping, per-game terminal dispositions and artifact revisions, comparison,
decision authority, and next action linked from the effort's native home.
Report whether the combinations actually opened useful lines of thought,
whether repeats helped or constrained them, and where independence or weighting
misled the work. A technically correct draw is not proof of a useful flight.

The [flight practice](/praxis/flights/README.md#failure-and-partial-results)
explains terminal dispositions; the [generator](/tool/random-combo/README.md)
owns numerical and CLI behavior. Keep those responsibilities separate: the tool
can verify draw mechanics, while participants and owners must judge evidence,
scope, usefulness, and acceptance.
