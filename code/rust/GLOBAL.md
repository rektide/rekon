---
id: rust
order: 260
source: /code/rust/GLOBAL.md
status: draft
type: ContextFragment
title: Rust conventions
description: Rust tool preferences, configuration precedence, and publication mechanics.
resource: /code/rust/GLOBAL.md
generated: { by: "model:openai/gpt-6-astra#high", at: 2026-10-22 }
sources:
  - { resource: /AGENTS.old.md, title: Legacy Rust preferences }
  - { resource: "opencode://ses_e9a41f6dfffexEmfMeIjFhXDko", title: Clarify publication version selection alongside the preferred pre-alpha API-marker policy }
---

# Rust conventions

- Preferred libraries: figment2 for config; clap v4 derive and clap_complete for
  CLI/completions; bon for builders; jiff for time rather than chrono; tracing
  with useful spans for diagnostics.
- Use nextest for tests and clippy for checking. Benchmark with criterion and
  cargo-criterion; use criterion-perf-events or criterion-cycles-per-byte when
  the relevant hardware/per-byte measurements matter.
- For Clap/Figment2 integration, derive Parser, Serialize/Deserialize, and Builder
  where useful. Preserve CLI > environment > defaults precedence. Do not let
  clap's default values accidentally override environment configuration; distinguish
  explicit CLI values when layering. `Serialized::defaults(CliArgs::parse())`
  alone does not establish that precedence.
- Publishing: choose the version under the project's policy, using `cargo bump`
  or its coordinated workspace bump tool; then make a jj commit and run
  `cargo publish`. Patch is the fallback for ordinary release bumps; the
  [preferred pre-alpha strategy](/code/versioning.md) uses minor API-change markers.

Recovered from [legacy Rust preferences](/AGENTS.old.md#rust-instructions).
The precedence clarification preserves its intended ordering without treating
the old single-provider example as a complete layering implementation.
