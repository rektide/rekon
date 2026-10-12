---
type: ToolContract
title: Table of contents for Rekon Markdown/MDX
description: rekon-toc renders a heading ToC into a managed <nav id="rekon-toc"> region with real link targets, optional final-document line ranges, and idempotent replacement.
resource: /tool/toc/README.md
tags: [tool, toc, rust, markdown, mdx]
status: draft
generated: { by: "model:zai-coding-plan/glm-5.3#max", at: 2026-10-11 }
sources:
  - { resource: "opencode://ses_ed31715c4ffex2ROd2G2C617AF", title: CLI integration, source-preservation review, and parser-profile clarification }
  - { resource: "urn:opencode:session:ses_ed2ff8ca0fferqEbe9uxlNoiOv", title: rekon-toc crate commission, implementation, and review correction (slug allocation, id-conflict rejection, nav-region safety, MDX opacity) }
  - { resource: "urn:opencode:session:ses_ed31715c4ffex2ROd2G2C617AF", title: Parent commission and review findings for enriched assembly and Rust tool boundaries }
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

## Command

```sh
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- toc guide.md --line-ranges
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- toc AGENTS.md --line-ranges --check
```

`rekon toc FILE… [--check] [--min-depth N] [--max-depth N] [--line-ranges]`
prepares every input before updating any file. Writes atomically replace each
target while preserving its permissions; symlinks remain links and their targets
are updated. `--check` writes nothing and exits 1 on drift. `combine` always
enables line ranges, so include `--line-ranges` when checking its navigation with
the standalone command.

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
| `line_ranges` | false | Emit `data-line-start`/`data-line-end` on entry anchors, 1-based whole-file line ranges of the final document. `TocOptions::with_line_ranges()` matches the combine pass. |
| `aria_label` | "Table of contents" | Accessible label on the nav element. |

## Region shape

```md
<nav id="rekon-toc" aria-label="Table of contents">

- [Code](#code)
  - [Terms](#code-terms)

</nav>
```

With line ranges, each entry becomes an HTML anchor inside the same list:
`- <a href="#code" data-line-start="12" data-line-end="40">Code</a>`.
The first run inserts the region after the document's first H1 (or at the top
when there is none); later runs replace the region wholesale and never touch
prose outside it. A document with no listable headings keeps an empty region
in place (no pruning); when no region exists yet and nothing is listable, the
input is returned unchanged.

## Semantics

- **Parser.** markdown-rs (the maintained `markdown` crate, same dependency
  as [`rekon-markup`](/tool/rust/crates/rekon-markup/)) with the **strict
  JSX reader profile** — GFM + frontmatter + MDX expressions + JSX, raw-HTML
  constructs off, HTML comments masked first (byte-preserving, exactly the
  way rekon-markup's reader does it). Both require balanced JSX structure;
  their expression policies differ. A custom expression validator extends the default brace
  counter to respect JSON strings and block comments, so metadata like
  `<Okf data={{"note":"brace } inside"}} />` parses.
- **Static MDX is opaque.** `<Okf data={…} />` is one element; HTML-looking
  strings inside its JSON (`"<nav id='rekon-toc'>"`,
  `"<section id=\"x\">paired</section>"`) are string content — they never
  define a region, a section scope, or a reserved id, and the metadata bytes
  survive update unchanged. `{/* … */}` comments are equally opaque: fake
  headings or navs inside them are comment text.
- **Managed region.** The parser-confirmed `<nav id="rekon-toc">` element
  span, nothing else. A `</nav>` inside fenced code cannot close it; an
  unclosed nav fails the parse; a second managed nav is an error rather than
  a silent choice; a one-line `<nav id="rekon-toc"></nav>` is handled.
  Content outside the element is never touched. The nav must **own its
  lines**: one embedded in prose (`Before <nav…></nav> after`) or under a
  container prefix (blockquote `> `, list `- `) is rejected with
  `UnisolatedNav`, because wholesale region replacement would drop that
  content — and an embedded nav must not silently become a second region.
- **Ignored headings.** Anything inside code fences or indented code, YAML
  frontmatter, JSX expression comments, and the managed region itself. ATX
  and setext headings both count; frontmatter lines count in line
  coordinates (whole-file, physical, 1-based).
- **Link targets.** Renderer slugs are allocated across *every* real heading
  in document order — unlisted H1s and explicitly-targeted headings consume
  theirs too, so `# Same` + `## Same` links the H2 to `#same-1`. Listed
  headings then pick, once each: a bound empty-anchor id
  (`<a id="…"></a>` separated from the heading only by blank lines), the
  closest enclosing `<section id="…">`, then the renderer slug.
- **Id conflicts are rejected, not worked around.** If a heading's renderer
  slug equals an unrelated explicit id (a `<dfn>` term, another section) or
  the managed nav id, the rendered document would carry duplicate ids and a
  silently shifted slug would point at a nonexistent anchor — so `update_toc`
  fails with `ConflictingIds` asking for source repair. Two coincidences are
  valid: the slug may equal the heading's *own* bound anchor id or its
  *enclosing section* id; both anchor the same spot and the explicit target
  is preferred.
- **Line ranges.** Computed on the final post-splice layout: an entry runs
  from its heading line to one before the next listed heading, or end of
  file. Inline numbers never change the region's line count, so one pass
  reaches the fixed point — updating an updated document returns identical
  bytes (verified by test on every fixture, and `check_toc` doubles as the
  drift gate, the `--check` discipline the hyalo lineage established).
- **Labels stay literal.** Flattened heading text is escaped, never
  re-interpreted: literal braces from inline code (`## Configure \`{name}\``)
  render as `\{`/`\}` in Markdown labels and `&#123;`/`&#125;` numeric
  entities in HTML anchors, so navigation output cannot turn source text into
  a runtime MDX expression.

## Limits

- **Balanced JSX.** Unbalanced HTML elements and multiline containers that put
  initial prose on the opening-tag line (`<dd>text…`) but close in a later block
  are errors. Put multiline element content on following lines —
  [CONTEXT.md](/CONTEXT.md) shows the compatible `<dd>` shape. HTML comments
  must be blank-line-separated blocks (masking recognizes them as blocks).
- **Expression profile.** ToC reads balanced MDX expressions without evaluating
  them; literal prose braces should be escaped or put in code. The metadata
  extractor instead treats body braces as prose and requires literal JSON in
  expression attributes. Neither is a JavaScript runtime or a complete MDX compiler.
- The managed region must be a real element: an open `<nav
  id="rekon-toc">` without a closing `</nav>` fails the parse; a fenced fake
  closing tag does not rescue it and cannot define a region.
- Generated navigation uses `\n`; existing prose outside the region is retained.
  IDs must be static literals (`id={expr}` is not a
  usable target).
- Duplicate explicit ids are reported through `analyze`'s `duplicate_ids`
  (a document problem), not rewritten. Empty-title headings render without
  anchors and are skipped.
