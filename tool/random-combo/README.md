---
type: ToolContract
title: Random-combo Node generator
description: Reproducible weighted triples with replacement and immediate persistent 0.8 decay, supplied as one inline-weighted option per line.
resource: /tool/random-combo/README.md
status: draft
generated: { by: "model:openai/gpt-6.1-sol#high", at: 2026-11-02 }
tags: [tool, random-combo, flights, node, weighted-sampling]
sources:
  - { resource: "opencode://ses_e614c6702ffe3Ev02jA4WnHnBF", title: Add replayable one-triple-per-line row output }
  - { resource: "opencode://ses_e6172cb39ffe5Q3YzyRjA9H32n", title: Implement and test the commissioned Node generator and reusable random-combo flight }
  - { resource: /tool/README.md, title: Top-level executable entrypoints and domain-grouped implementation conventions }
  - { resource: /constitution/structure.md#tools, title: Source-run Node tool layout and explicit implementation standing }
  - { resource: /praxis/flights/README.md, title: Flight coordination and authorization boundaries }
---

# Random-combo generator

Pick **three weighted options per game, with replacement**. Immediately after
each win, multiply that option's **current** weight by **0.8**. Updated weights
carry through subsequent slots and games in this process. No resetting between
games, no deduplication, and no rejection of doubles or triples.

This is the expressly commissioned **Node.js** tool, not a Rust subcommand of
`rekon`, a scheduler, or an agent launcher. It implements the draw stage of the
⇄ [random-combo flight](/praxis/flights/random-combo.md).

## Run

Use Node.js **22.18+** (native TypeScript stripping); tested here on Node 26.6.0.
Only Node builtins are used: no install, build, or package-manager command needed.
From the Rekon checkout:

```sh
node tool/random-combo.ts --help
node tool/random-combo.ts --games 4 --seed example tool/random-combo/example.txt
node tool/random-combo.ts --games 4 --seed example --jsonl tool/random-combo/example.txt
node tool/random-combo.ts --rows 8 tool/random-combo/example.txt > rows.jsonl 2> seed.txt
printf 'explore w=1\nsmall experiment w=0.1\n' | node tool/random-combo.ts --games 2 --seed demo
```

`FILE` is optional: omitted or `-` reads UTF-8 stdin to EOF. One positional file
is allowed; use `-- ./-unusual-name.txt` for a filename starting with a dash.
`--games N` defaults to **1** and must be a positive integer whose triple is also
a safe JavaScript integer. The number of draws per game and the decay are fixed,
not configurable knobs. To run from elsewhere, use the entrypoint's absolute
path; relative input files resolve from your current working directory.
`--rows N` instead selects N games in compact row mode, subject to the same
integer limit; it conflicts with both `--games` and `--jsonl`.

## Input grammar

One option per line, with **exactly one whitespace-delimited `w=NUMBER` token**
anywhere in each entry. A supplied number is mandatory; there is no implicit
weight and no separate weight array. For example:

```text
# Full-line comments and blank lines are allowed.
careful exploration w=1
w=0.1 unusual perspective
small w=0.5 experiment
not in this run w=0
```

- Remaining tokens form the arbitrary option label. Whitespace is normalized to
  single spaces; UTF-8 labels, CRLF, and a leading BOM are supported. Labels must
  be nonempty and unique after normalization. Option IDs start at 1 in input order.
- Numbers are unsigned decimal literals, optionally fractional or exponential:
  `1`, `0.1`, `.1`, `1.`, `1e-3`. Whitespace around the equals sign is not allowed.
  Negative weights, `NaN`, `Infinity`, hex, empty weights, trailing junk,
  overflow, and positive literals that round to zero are errors.
- Zero weights remain visible but **never win**. At least one option must have a
  positive weight. A single positive option is valid: every game is its triple,
  with three consecutive decays.
- There are no quotes, escapes, CSV fields, or inline comments. `#` is literal
  within a label, but a trimmed line beginning with `#` is a comment. A label
  cannot contain another whitespace-delimited token starting with `w=`.

Duplicate **input labels** are errors to prevent ambiguous identity; repeated
**draws** of a valid option are intentional and must remain in the result.

## Draw and replay contract

Before each slot, choose option `i` with probability proportional to its current
weight. In implementation, divide all current weights by their maximum, sum
those scaled values, and walk their intervals in input order using `u * total`.
Intervals are left-closed/right-open; zero-weight intervals are skipped. The
last positive interval handles a floating-point boundary rounding upward.
Scaling avoids overflow even when two individually finite weights sum to infinity.

Then perform **one** `current[i] *= 0.8` and keep every other weight unchanged.
For a weight starting at 1, three wins produce approximately
`0.8`, `0.64`, `0.512`; a fourth win, even in the next game, produces `0.4096`.
There is no renormalization of stored weights, persistence between processes,
or mutation of the input file. Restarting the command starts from the input weights.

`--seed TEXT` is exact UTF-8 text, including an empty string. Without it, the tool
generates and prints a 128-bit random hex seed. Record the seed, **ordered input**,
games, algorithm, and code revision for replay; the seed alone does not identify
the input. The algorithm name is `sha256-u32le-mulberry32-v1`: SHA-256 the seed,
read the first 4 digest bytes as a little-endian unsigned integer, initialize
Mulberry32, and divide each unsigned 32-bit output by `2^32` for `u ∈ [0,1)`.
This is deterministic exploration sampling, **not cryptographic randomness** or
a fair lottery. Seed hashing has only a 32-bit PRNG state; collisions are possible.

JavaScript IEEE-754 arithmetic is visible, not prettified into false exactness:
`0.6400000000000001` is normal. Very small relative probabilities can fall below
floating-point or PRNG resolution. An extreme run can reach subnormal weights
that no longer decrease under multiplication; the tool stops with a numerical
decay-limit error rather than silently freezing or zeroing the winner. Shorten
the run or increase initial weights; no automatic rescale changes recorded state.

## Audit output and failure

`--rows N` writes exactly N lines on successful completion: each line is a JSON
array of three ordered labels, e.g. `["first choice","other","first choice"]`.
JSON quoting makes spaces and punctuation unambiguous, and repeated picks remain
present. The seed (as a JSON string) and algorithm name go to stderr, not stdout.
Capture both streams as in the example above; retain the ordered input, row count,
and code revision too. Replay with `--rows N --seed TEXT` and the same input;
decode the JSON seed string if necessary before passing its exact value. To inspect
the full draw audit for those rows, use `--games N --seed TEXT --jsonl` with the
same input and revision. Both modes use the same continuous draw sequence.

Default text shows the algorithm and seed, **all initial weights**, each draw's
game, slot, option ID and label, raw `u`, scaling factor, scaled total, winning
weight before/after decay, the ordered game triple, all weights after each game,
and final weights. A repeat remains visible in every occupied slot.

`--jsonl` provides the same audit as **one JSON object per line**, not one JSON
array. Event order is `initial`, then three `draw` and one `game` per game, then
`final`. The initial event has `seed`, `algorithm`, `games`, `drawsPerGame`,
`decay`, and `weights`; draw events have `game`, `slot`, `optionId`, `label`,
`random`, `scale`, `scaledTotal`, `weightBefore`, `weightAfter`; game events have
`game`, ordered `picks`, and `weights`; final has `weights`. Each weight entry is
`{ id, label, weight }`, each pick `{ optionId, label }`.

Successful completion exits 0; expected usage/input/numerical errors exit 2 and
explain the problem on stderr, including line numbers for malformed entries.
Input and arguments are validated before any transcript is emitted. A numerical
limit, interruption, or output failure can leave a **partial** transcript: require
a successful process exit and, for audit JSONL, a `final` event; for rows, check
the requested line count as well.
Unexpected failures retain their normal Node stack. Output is streamed with
backpressure; no generated file is silently written or overwritten.

## Verification and implementation

```sh
node --test tool/random-combo/test.ts
```

The dependency-free Node test suite covers grammar/errors, zero weights,
doubles and triples, decay across game boundaries, changed selection intervals,
input immutability, huge weights, numerical limits, a pinned PRNG vector,
seed replay, file/stdin parity, CLI errors/help, import safety, and symlink entry.
These are implementation checks, not evidence that a particular flight is useful.

- [Entrypoint](/tool/random-combo.ts): import-safe invocation detection.
- [Generator](/tool/random-combo/generate.ts): parser, seeded PRNG, lazy audit events.
- [CLI](/tool/random-combo/cli.ts): arguments, stdin/file input, text/JSONL/rows, errors.
- [Tests](/tool/random-combo/test.ts) and [example input](/tool/random-combo/example.txt).

The [toolkit map](/tool/README.md) locates this tool among Rekon's executable
capabilities; the [flight](/praxis/flights/random-combo.md) owns agent assignments,
joins, and interpretation rather than making them CLI side effects.
