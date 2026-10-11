---
type: Reference
title: Static scoped OKF extraction
description: Rust library contract for preserving frontmatter and reading explicitly scoped metadata from Markdown and static MDX JSX.
status: draft
generated: { by: "rekon/gpt-6.1-sol", at: "2026-10-11T18:11:59-04:00" }
sources:
  - { resource: /design/toolchain/enriched-assembly0.gpt6a.md, title: Enriched assembly interface and static metadata commission }
  - { resource: /okf/README.md, title: Rekon OKF conventions and unknown-field preservation }
  - { resource: /refs/sources.md, title: Native provenance and honest verification }
  - { resource: "https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/86df01601733f93c0f243a727927d03ebf6d7e1d/okf/SPEC.md", title: OKF v0.2, consulted in the local archive }
  - { resource: "https://docs.rs/markdown/1.0.0/markdown/", title: Maintained CommonMark/GFM and MDX AST parser }
  - { resource: "urn:opencode:session:ses_ed2ffcc34ffeinaoX0ouV2gDCy", title: Sol Medium static extractor implementation, API reasoning, and focused verification }
  - { resource: "urn:opencode:session:ses_ed31715c4ffex2ROd2G2C617AF", title: Parent commission for enriched Rust assembly, scoped extraction, and shared markup interfaces }
---

# Static scoped OKF extraction

[`rekon-markup`](/tool/rust/crates/rekon-markup/src/lib.rs) reads YAML frontmatter
and static `<Okf>` records without executing JavaScript. It preserves unknown
fields and nested JSON-compatible values. This is metadata transport and
structural extraction, **not** full OKF bundle validation, verification, or a
trust-tier policy.

## Wire representation and ownership

```mdx
<section id="fragment-code" data-source="/code/GLOBAL.md">

<Okf data={{"type":"Practice","unknown":{"nested":[true,null,12]}}} />

## Code

<article id="local-detail">

<Okf data={{"type":"Reference"}} />

</article>
</section>
```

The nearest enclosing `section` or `article` owns each record, even through
other intervening elements. Outside those elements, the document owns it.
Leading source YAML also belongs to the document. Descendants do not inherit
metadata, `data-source`, verification, or other claims. Multiple records are
returned in source order and are not merged.

`owner.id` is the explicit structural ID, and `owner.data_source` describes
assembly origin. Neither is OKF `metadata.resource`. Missing owner IDs remain
`null`; the library invents no identity and does not enforce global uniqueness.
Scoped JSX is a **Rekon extension**, not upstream OKF v0.2 syntax.

## Library API

```rust
use rekon_markup::{extract, parse_document, parse_frontmatter, render_okf};

let source = "---\ntype: Practice\nextension: {nested: [true, null]}\n---\n# Body\n";
let frontmatter = parse_frontmatter(source)?;
let jsx = render_okf(frontmatter.metadata.as_ref().unwrap())?;
let records = extract(&jsx)?;
assert_eq!(records[0].metadata, frontmatter.metadata.unwrap());
let parsed = parse_document(source)?;
assert_eq!(parsed.headings[0].text, "Body");
Ok::<(), rekon_markup::Error>(())
```

- `parse_frontmatter(&str) -> Result<Frontmatter<'_>>`: optional
  `serde_json::Value` metadata, exact borrowed `body`, `body_offset`, and optional
  frontmatter `span`. Only `---` on the first line starts metadata; a closing
  `---` is mandatory. LF and CRLF are supported.
- `render_okf(&Value) -> Result<String>`: a compact object serialized inside one
  JSX expression (`data={` + JSON + `}`). Non-object roots fail.
- `extract(&str) -> Result<Vec<ScopedMetadata>>`: metadata, owner, origin, and
  source range per record.
- `parse_document(&str) -> Result<ParsedDocument>`: the same records plus
  Markdown headings (`depth`, readable `text`, `span`). ATX and setext headings
  come from the maintained parser, including within JSX sections. Fenced and
  indented code and inline code do not introduce records or headings.

`SourceRange { start, end }` always means a half-open **UTF-8 byte range in the
original complete source**, including its frontmatter. Owner ranges cover the
whole enclosing element, including opening and closing tags; document ownership
covers the whole source. Metadata ranges cover the complete frontmatter block
or `Okf` element. The types implement `Serialize` for CLI JSON/YAML adapters.

## Static syntax boundary

The library uses `markdown` 1.0's CommonMark/GFM and MDX JSX AST, not a custom
Markdown scanner. HTML comments recognized as standalone Markdown HTML nodes
are blanked with byte-preserving spaces before JSX parsing, so generated
ownership comments do not become records or corrupt offsets.

YAML roots must be mappings; recursively supported values are string-keyed
objects, arrays, strings, booleans, null, and JSON-representable numbers. Tags,
non-string mapping keys, non-finite numbers, duplicate YAML keys, malformed YAML,
and missing closing delimiters fail explicitly. Unknown field names are fine.
No OKF field is inserted, normalized, or silently dropped; a bare `verified`
mapping stays a mapping in transport rather than being rewritten as a list.

`Okf` has exactly one `data` prop, containing a literal **JSON object**, and no
body content. JavaScript object syntax (unquoted keys or single-quoted strings),
spread props, identifiers, function calls, and arrays/null as the data root fail.
The parser's expression callback uses a real JSON parser: unmatched braces
inside JSON strings round-trip correctly. Structural `id` and `data-source`
accept quoted JSX strings or JSON-string expressions; dynamic ownership fails.
Other JSX expression attributes also must be literal JSON. Body braces are
ordinary prose, not executable MDX expressions; imports/exports are not run.

This deliberately supports a static Markdown/JSX subset, not every legacy HTML
or runnable MDX document. JSX elements must be balanced (HTML void elements
must use `<br />`, for example). Comments embedded inside a larger raw HTML
block are not guaranteed to be recognized. A multiline container such as `dd`
must put its opening tag on its own line (`<dd>`, then prose on the next line),
or keep its contents and closing tag on one line; legacy `<dd>prose` followed by
multiline continuation is not valid flow JSX. These syntax limits surface as
errors rather than producing partial metadata. Frontmatter-only consumers can
use `parse_frontmatter` without parsing the body.

## Checks and connections

Run `cargo nextest run -p rekon-markup --manifest-path tool/rust/Cargo.toml`
(or `cargo test -p rekon-markup --manifest-path tool/rust/Cargo.toml`).
[Focused tests](/tool/rust/crates/rekon-markup/tests/metadata.rs) exercise nested
unknown-field round-trips, nearest ownership without inheritance, source byte
ranges, code/comment exclusion, headings, Unicode/CRLF, malformed/dynamic
attributes, and unsupported YAML.

- [OKF home](/okf/README.md): canonical file-level knowledge conventions; this
  tool extends their transport without declaring a new upstream standard.
- [Enriched assembly contract](/design/toolchain/enriched-assembly0.gpt6a.md):
  explains why records attach to explicit document sections, not heading names.
- [Source practice](/refs/sources.md): provenance fields retain native identity;
  successful extraction is not verification of their claims.
- [Assembly home](/tool/assembly/README.md): the producer can lift a source's
  frontmatter into the static scoped representation and check extraction parity.
