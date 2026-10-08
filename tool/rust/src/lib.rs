//! `rekon` — the rust super-tool crate.
//!
//! Rekon's accepted toolchain end-state ([design/toolchain/draft0] addendum,
//! 2026-10-08) is rust-first for new code: one crate under `tool/rust/`, binary
//! `rekon`, whose subcommands are the super-tool surface. There is no TS
//! composition layer.
//!
//! The first subcommand is `combine`, the AGENTS.md combiner. Its behavior
//! contract is binding and pre-dates this crate:
//!
//! - [`constitution/agents-assembly.md`][assembly-contract] — fragment
//!   metadata, manifest format, output shape, provenance, generated-file
//!   policy.
//! - [`tool/assembly/README.md`][assembly-readme] — the production Effect
//!   tool's invocation, exit codes, reports, and the `..notes/` correction.
//! - The TypeScript implementation in `tool/assembly/` and `tool/md/` (with
//!   its colocated tests) is the executable form of that contract; this crate
//!   ports observable behavior, not redesigns it.
//!
//! The crate deliberately does **not** regenerate `/AGENTS.md`'s header with
//! the TypeScript tool's identity: `rekon combine` writes its own
//! generated-file header. A producer flip away from `node tool/agents.ts` is a
//! separate deliberate change.
//!
//! Module boundaries mirror the TypeScript layout: [`manifest`] ~
//! `tool/assembly/manifest.ts`, and later `fragment`/`markdown` ~
//! `tool/md/fragment.ts`, `document` ~ `tool/md/document.ts`, `combine` ~
//! `tool/assembly/assemble.ts` plus the command wiring in `tool/cmd/agents.ts`.
//!
//! [design/toolchain/draft0]: /design/toolchain/draft0.glm53m.md
//! [assembly-contract]: /constitution/agents-assembly.md
//! [assembly-readme]: /tool/assembly/README.md

pub mod manifest;
pub mod nodepath;

pub use manifest::{DEFAULT_MANIFEST, Manifest, ManifestError};
