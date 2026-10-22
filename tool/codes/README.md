---
type: ToolContract
title: Codes registry generation
description: Rust codes assembles the workspace short-codes registry table from self-describing code files under codes/, with byte-exact check behavior.
resource: /tool/codes/README.md
tags: [tool, codes, naming, registry, rust]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-10-22 }
sources:
  - { resource: "opencode://ses_e9bc39334ffetsgyIAj4L0kp0A", title: "Commission codes/ code files and the Rust codes generator" }
  - { resource: /codes.md, title: The generated registry output }
  - { resource: /constitution/naming/README.md, title: Short codes practice that owns code vocabulary }
---

<a id="codes-tool"></a>
# Codes registry generation

**`rekon codes` renders [`/codes.md`](/codes.md) from the code files under
[`codes/`](/codes).** Each code is one self-describing Markdown file; the
registry is deterministic generated output, byte-checkable like `AGENTS.md`.

```sh
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- codes
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- codes --check
```

## Code files

The file name stem is the code: `codes/cf.md` declares `code: cf`. Frontmatter
carries three registry fields — `code`, `target` (a `file#tag` string:
bundle-root-relative `/…`, `file://…`, or `http(s)://…`), and `description`
(the registry row's note, pipe-free) — beside the OKF knowledge fields and the
fragment contract fields (`id`, `order`, `source`, `status`). The fragment
fields let a code contribute itself to assembly the same way GLOBAL fragments
do; the body's single H1 plus prose satisfies the fragment shape. Generation
uses only the frontmatter; the body is for readers and future assembly.

## Behavior

- Entries sort by code; output bytes are stable for the same inputs.
- A declared code must match its file stem; a stem collision is impossible by
  construction, so duplicate codes cannot occur.
- Zero code files, missing fields, unsafe targets, pipes in row fields, and an
  output aliasing a code file are all loud errors.
- Publication stages a complete file and renames it into place; `--check`
  compares bytes without writing.

The [naming practice](/constitution/naming/README.md#short-codes) owns code
vocabulary; this tool owns generation. Implementation:
[`tool/rust/src/codes.rs`](/tool/rust/src/codes.rs), adapter
[`tool/rust/src/cli/codes.rs`](/tool/rust/src/cli/codes.rs).
