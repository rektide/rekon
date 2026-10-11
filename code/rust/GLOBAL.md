---
id: rust
order: 260
source: /code/rust/GLOBAL.md
status: draft
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
- Publishing: run `cargo bump` with major/minor/patch (patch by default), then
  make a jj commit, then `cargo publish`.

Recovered from [legacy Rust preferences](/AGENTS.old.md#rust-instructions).
The precedence clarification preserves its intended ordering without treating
the old single-provider example as a complete layering implementation.
