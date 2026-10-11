//! Rust engines behind the `rekon` super-tool.
//!
//! [`assembly`] combines complete, explicitly selected fragments into enriched
//! Markdown/MDX. Frontmatter survives as scoped `Okf` data; document headings
//! become a nested section hierarchy. [`coverage`] independently checks declared
//! operating requirements against the preserved legacy baseline.
//!
//! The workspace's `rekon-markup` library owns reusable document/metadata
//! semantics, and `rekon-toc` owns navigation generation. CLI orchestration
//! applies the ToC pass before publishing or checking the assembled bytes.
//!
//! Current design: `/design/toolchain/enriched-assembly0.gpt6a.md`.
//! Maintained assembly guide: `/tool/assembly/README.md`.

pub mod assembly;
pub mod coverage;
pub mod manifest;
pub mod nodepath;

pub use manifest::{DEFAULT_MANIFEST, Manifest, ManifestError};
