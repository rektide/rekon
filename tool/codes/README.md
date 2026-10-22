---
type: ToolContract
title: Codes glossary generation
description: Rust codes assembles the short-codes glossary from self-describing code files under codes/, emitting a fragment that bundles into ambient context by default.
resource: /tool/codes/README.md
tags: [tool, codes, naming, glossary, rust]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-10-22 }
sources:
  - { resource: "opencode://ses_e9bc39334ffetsgyIAj4L0kp0A", title: "Commission the codes generator, then direct its glossary into ambient context" }
  - { resource: /codes/GLOSSARY.md, title: The generated glossary, bundled by the root manifest }
  - { resource: /constitution/naming/README.md, title: Short codes practice that owns code vocabulary }
---

<a id="codes-tool"></a>
# Codes glossary generation

**`rekon codes` renders [`codes/GLOSSARY.md`](/codes/GLOSSARY.md) from the code
files under [`codes/`](/codes).** Each code is one self-describing Markdown
file; the glossary is deterministic generated output — and a valid assembly
fragment that the [root manifest](/agents.manifest.json) bundles into
`AGENTS.md`, so the codes table is ambient by default.

```sh
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- codes
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- codes --check
```

## Code files

The file name stem is the code: `codes/cf.md` declares `code: cf`. Frontmatter
carries three row fields — `code`, `target` (a `file#tag` string:
bundle-root-relative `/…`, `file://…`, or `http(s)://…`), and `description`
(the glossary row's note, pipe-free) — beside the OKF knowledge fields and the
fragment contract fields (`id`, `order`, `source`, `status`). The fragment
fields let a code contribute itself to assembly the way GLOBAL fragments do;
generation uses only the frontmatter, and the body's single H1 plus prose is
for readers and any future per-code assembly.

## Generated glossary

`codes/GLOSSARY.md` is reserved generated output: the tool never parses it as a
code file. It carries its own fragment identity (`id: codes`, `order: 55`), so
it assembles without a wrapper. Order 55 sits inside the code files' dormant
51–57 span on purpose: glob-assembling `codes/*.md` together with the glossary
then fails loudly on a duplicate order instead of silently assembling the table
twice. The per-code files remain the rich, non-ambient form.

## Behavior

- Rows sort by code; output bytes are stable for the same inputs.
- A declared code must match its file stem, so duplicate codes are impossible.
- Zero code files, missing fields, unsafe targets, pipes in row fields, and an
  output aliasing a code file are all loud errors.
- Publication stages a complete file and renames it into place; `--check`
  compares bytes without writing.

The [naming practice](/constitution/naming/README.md#short-codes) owns code
vocabulary; this tool owns generation. Implementation:
[`tool/rust/src/codes.rs`](/tool/rust/src/codes.rs), adapter
[`tool/rust/src/cli/codes.rs`](/tool/rust/src/cli/codes.rs).
