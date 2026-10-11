---
type: ToolContract
title: Table of contents for Rekon Markdown/MDX
description: rekon-toc renders a heading ToC into a managed <nav id="rekon-toc"> region with real link targets, optional final-document line ranges, and idempotent replacement.
resource: /tool/toc/README.md
tags: [tool, toc, rust, markdown, mdx]
status: draft
generated: { by: "model:zai-coding-plan/glm-5.3#max", at: 2026-10-11 }
sources:
  - { resource: "urn:opencode:session:ses_ed2ff8ca0fferqEbe9uxlNoiOv", title: rekon-toc crate commission and implementation (library, tests, this contract) }
  - { resource: "urn:opencode:session:ses_ed31715c4ffex2ROd2G2C617AF", title: Parent commission for enriched assembly and Rust tool boundaries }
  - { resource: /design/toolchain/enriched-assembly0.gpt6a.md, title: Rust boundary placing rekon-toc beside rekon-markup }
  - { resource: /design/rekon/hyalo-toc0.glm53m.md, title: hyalo ToC lineage: line ranges, fixed-point regeneration, drift gates }
  - { resource: "file:///home/rektide/ado/bin/hyalo-toc.ts", title: Prior TypeScript implementation with comment markers }
---

# Table of contents for Rekon Markdown/MDX

**`rekon-toc` turns headings into a managed navigation region.** Given whole
document text it inserts or regenerates a semantic
`<nav id="rekon-toc" aria-label="…">` block: a plain Markdown list of links,
optionally carrying `data-line-start`/`data-line-end` attributes so agents can
jump and budget reads (`hyalo read`-style ranged consumption, without the HTML
comment markers the hyalo lineage used). The library is pure — text in, text
out — with `update_toc(text, options)` as the one-call combine hook.

- Crate: [`tool/rust/crates/rekon-toc/`](/tool/rust/crates/rekon-toc/) (library
  only; the `rekon toc` CLI lives in the workspace root package).
- Lineage: [`hyalo ToC research`](/design/rekon/hyalo-toc0.glm53m.md) — the
  line-range motive and drift-gate discipline come from there; comment markers
  are deliberately replaced by the semantic nav region per the
  [assembly contract](/design/toolchain/enriched-assembly0.gpt6a.md).

## API

```rust
use rekon_toc::{update_toc, check_toc, analyze, TocOptions};

let updated = update_toc(&text, &TocOptions::default())?;       // insert or regenerate
let drift   = check_toc(&text, &TocOptions::default())?;         // drift.up_to_date, drift.updated
let structure = analyze(&text, &TocOptions::default())?;         // entries, ids, region
```

| Option | Default | Meaning |
| --- | --- | --- |
| `min_depth` | 2 | Smallest heading level listed (document H1 stays out). |
| `max_depth` | 6 | Largest heading level listed. |
| `line_ranges` | false | Emit `data-line-start`/`data-line-end` on entry anchors, 1-based whole-file line ranges of the final document. `TocOptions::with_line_ranges()` is the planned CLI default set. |
| `aria_label` | "Table of contents" | Accessible label on the nav element. |

Planned CLI (owned by the root package): `rekon toc FILE… [--check]
[--min-depth N] [--max-depth N] [--line-ranges]`.

## Region shape

```md
<nav id="rekon-toc" aria-label="Table of contents">

- [Code](#fragment-code)
  - [Terms](#fragment-code--terms)

</nav>
```

With line ranges, each entry becomes an HTML anchor inside the same list:
`- <a href="#fragment-code" data-line-start="12" data-line-end="40">Code</a>`.
The first run inserts the region after the document's first H1 (or at the top
when there is none); later runs replace the region wholesale and never touch
prose outside it. A document with no listable headings keeps an empty region
in place (no pruning); when no region exists yet and nothing is listable, the
input is returned unchanged.

## Semantics

- **Parser.** markdown-rs (the maintained `markdown` crate, same dependency as
  [`rekon-markup`](/tool/rust/crates/rekon-markup/)), GFM + frontmatter. ToC
  reads tolerantly: MDX components such as `<Okf data={…} />` and prose braces
  are ordinary paragraph text here, so no document fails ToC for MDX
  strictness. Structural ids (`<section>`, `<nav>`, empty anchors, `<dfn>`)
  come from real AST nodes and raw HTML blocks the parser isolated. A shared
  `rekon_toc::parse_options()` exposes this profile.
- **Ignored headings.** Anything inside code fences or indented code, YAML
  frontmatter, and the managed nav region itself. ATX and setext headings both
  count; frontmatter lines count in line coordinates (whole-file, physical,
  1-based).
- **Link targets.** Preference order: an explicit empty anchor bound to the
  heading (`<a id="…"></a>` separated from it only by blank lines), then the
  closest enclosing `<section id="…">`, then a GitHub-style slug. A target is
  claimed once; later headings fall through to the next preference. Generated
  slugs avoid every literal id in the document (including `<dfn>` terms) and
  disambiguate deterministically: `same`, `same-1`, `same-2`.
- **Line ranges.** Computed on the final post-splice layout: an entry runs
  from its heading line to one before the next listed heading, or end of
  file. Inline numbers never change the region's line count, so one pass
  reaches the fixed point — updating an updated document returns identical
  bytes (verified by test on every fixture, and `check_toc` doubles as the
  drift gate, the `--check` discipline the hyalo lineage established).

## Limits

- HTML blocks follow CommonMark: a `<section>`/`<nav>` open tag ends at the
  next blank line, so emitted sections keep blank lines between the tag and
  nested Markdown (the assembly wire contract does).
- The managed region must be closed; an open `<nav id="rekon-toc">` without
  `</nav>` is an error, not a silent rewrite.
- Line endings are `\n`; ids inside raw HTML must be static literals.
- Duplicate explicit ids are reported through `analyze`'s `duplicate_ids`
  (a document problem), not rewritten.
