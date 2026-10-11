---
type: ToolContract
title: Operating-context coverage
description: Review the legacy-to-fragment mapping and check instructional coverage independently of generated-file freshness.
resource: /tool/assembly/coverage.md
status: draft
generated: { by: "model:openai/gpt-6-astra", at: 2026-10-11 }
sources:
  - { resource: "opencode://ses_ed31715c4ffex2ROd2G2C617AF", title: Integrate independent operating coverage with the Rust producer }
  - { resource: "urn:opencode:session:ses_ed31715c4ffex2ROd2G2C617AF", title: Rust producer integration and preservation of the independent coverage regression }
  - { resource: /design/context-compilation/repair0.gpt6a.md, title: Missing ambient guidance diagnosis }
  - { resource: /AGENTS.old.md, title: Reviewed legacy baseline }
  - { resource: /tool/assembly/README.md, title: Production byte-assembly contract }
---

# Operating-context coverage

**Fresh output can still be incomplete.** The assembler verifies that output
matches selected inputs. The Rust `audit-context` command separately checks the
reviewed operating requirements declared in [coverage.json](/tool/assembly/coverage.json).
Use both after changing selection or guidance:

```sh
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- audit-context
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- combine --check
```

`pnpm check` runs both. The audit reads files and reports problems; it never
rewrites guidance, creates output, migrates documents, or updates its own ledger.
Rust `combine` produces the enriched `AGENTS.md`; the audit remains independent
of its metadata, heading, and navigation transformations.

## What is checked

- The declaration pins the legacy file's SHA-256, so changed evidence requires
  deliberate re-review. Every real top-level heading must have one coverage entry;
  headings inside code, quotes, or lists are not source sections.
- Each entry identifies required instructional excerpts in selected fragments.
  A file merely existing outside the manifest cannot satisfy coverage.
- Required excerpts must occur in the fragment's visible Markdown body, ignoring
  whitespace wrapping. Frontmatter and HTML comments cannot satisfy them.
- Duplicate/unknown headings, empty assertions, unavailable sources, omitted
  fragments, and absent excerpts fail with actionable diagnostics. All coverage
  findings are reported together; malformed input and I/O failures retain context.
- `--manifest` and `--coverage` are invocation-relative paths. Paths declared by
  the coverage file resolve from the manifest directory, like fragment paths.
  Paths are lexical; this is not a filesystem sandbox.

Exit 0 means the **declared checks** passed. It does not prove semantic equivalence,
complete per-sentence migration, or editorial quality. The ledger is a review
surface, not a substitute for reading the combined document. Never weaken a
required excerpt just to make the check green; explain a genuine policy change
in its section's `note` and update the owned prose together.

Implementation: [`coverage.rs`](/tool/rust/src/coverage.rs) parses real Markdown
headings and visible bodies with pulldown-cmark; the existing Rust manifest
planner supplies selection and path validation. Its CLI lives in
[`main.rs`](/tool/rust/src/main.rs). Run `cargo nextest run --manifest-path
tool/rust/Cargo.toml` and `cargo clippy --manifest-path tool/rust/Cargo.toml
--workspace --all-targets -- -D warnings`. The CLI regression test invokes the
Rust assembler in temporary bundles. It proves fresh output can still fail
coverage and that the audit does not write the output.

## Reviewed composition, October 11

| Legacy area | Current exposure | Treatment |
| --- | --- | --- |
| Subagents, stewardship, discussing | [agents/GLOBAL.md](/agents/GLOBAL.md) | Restore actionable delegation and parent responsibility; correct obsolete `task_id` wording to the returned harness identifier. |
| Experimenting, locations, shell | [workspace fragment](/code/workspace/GLOBAL.md) | Restore archive-first lookup, scratch, variants, and environment lifetime. |
| Documentation, waves, doc-pass, writing | [constitution practice](/constitution/practice/GLOBAL.md) | Restore operational wave rules, provenance, and corpus linking beside the current constitution. Unfinished summarizer stays identified as unfinished. |
| Planning, coding, checkpoints | [code fragment](/code/GLOBAL.md) | Keep the newer checkpoint language; restore domain layouts, error handling, and deliberate cleanup. |
| Rust | [Rust fragment](/code/rust/GLOBAL.md) | Restore tools/publishing; clarify that one Figment defaults provider cannot establish CLI/env precedence. |
| JS/TS and Web Components | [JS fragment](/js/GLOBAL.md) | Restore common conventions; full recipes remain in the existing JS README. Respect the later Rekon Rust-first tool decision. |
| jj/git and jj-hunk | [jj fragment](/vcs/jj/GLOBAL.md), [code fragment](/code/GLOBAL.md) | Restore guardrails, messages, date cues; keep hunk depth in its existing guide. |
| Beads | [beads fragment](/praxis/beads/GLOBAL.md), [beads practice](/praxis/beads/README.md) | Restore explicit ID/namespace, graph direction and the command loop. Current CLI help distinguishes exports from backups; retire the SQLite-delete/import recipe for Dolt recovery. |

This is a reviewed, section-level recovery with selected load-bearing assertions,
not a verbatim legacy concatenation. The current glossary and ticket-pointer
practice remain selected. The old manuscript remains unchanged evidence.

## Next useful iterations

Review the actual combined context for clarity and operating cost, including how
much glossary should be ambient. Extend the assertions when a real missed rule
is found. Reuse the coverage declaration in future Rust `combine` preflight;
keep byte freshness, declared coverage, and human judgment distinct. A later
producer change should demonstrate both parity and these negative cases, rather
than treating a different implementation language as the fix for missing advice.
