---
type: ToolContract
title: Markdown document checks
description: Read-only Rust checking of explicitly selected Markdown documents, local destinations and anchors, with an optional OKF knowledge-frontmatter profile.
resource: /tool/check/README.md
status: implemented
generated: { by: "model:openai/gpt-6-astra#high", at: 2026-10-22 }
sources:
  - { resource: "opencode://ses_e9d711bbeffenO3yHUyxP2Bv1G", title: "Prototype checker for diff-dirs consumer writing; commission G, then complete Rust parser boundaries, tests, docs and assembly integration after its usage limit" }
  - { resource: "opencode://ses_e9d67cd87ffe5fumUOB6NXo3Zn", title: "G integrates the initial Rust document-check library and command" }
  - { resource: /tool/rust/src/check.rs, title: "Parser-based checks and in-memory filesystem tests" }
  - { resource: /tool/rust/src/cli/check.rs, title: "Explicit file CLI, bundle-root discovery, diagnostics and exit status" }
  - { resource: /tool/rust/tests/check_cli.rs, title: "Public cross-repository and read-only failure test" }
  - { resource: /tool/rust/crates/rekon-markup/src/lib.rs, title: "Shared YAML frontmatter parsing" }
  - { resource: /tool/rust/crates/rekon-toc/src/slug.rs, title: "Shared Rekon heading slug convention" }
  - { resource: /refs/sources.md, title: "Knowledge provenance and native session-source practice" }
verification:
  method: Workspace nextest, doctests, strict all-target clippy, scoped rustfmt, document checks, assembly and ToC freshness
  result: 119 tests and one doctest passed; clippy, changed checker-file formatting, knowledge-document checks and generated output freshness passed
  limits: Workspace-wide formatting has existing differences; the coverage audit reports an existing Project Planning excerpt mismatch for domain-grouped project layouts in code/GLOBAL.md
---

# Markdown document checks

**`rekon check FILE...` checks selected documents without modifying them.** It is
usable for ordinary Markdown in any checkout. `--okf` adds Rekon's selected
knowledge-frontmatter contract; it is opt-in rather than a demand that every
README or upstream document carry knowledge metadata.

## Run

From Rekon:

```sh
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- check tool/README.md
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- check --okf tool/GLOBAL.md tool/check/README.md
```

From another project, point at Rekon's Rust toolchain while naming that project's
files normally:

```sh
cargo run --quiet --manifest-path ~/src/rekon/tool/rust/Cargo.toml -- check --okf doc/consumer/diff-dirs.md
```

An installed **Rust** `rekon` supports the shorter spelling. The legacy npm CLI
also uses that name; the Cargo invocation unambiguously selects this implementation.

| Argument | Meaning |
| --- | --- |
| `FILE...` | One or more explicit files. Shell glob expansion works; no recursive discovery or rewriting occurs. |
| `--root DIR` | Bundle root for `/...` destinations in every selected file; resolved against invocation cwd. |
| No `--root` | Discover the nearest `.jj` or `.git` ancestor of each canonical checked file; fall back to invocation cwd. |
| `--okf` | Also check required knowledge frontmatter, native session provenance and local source resources. |

Diagnostics identify the file and, for whitespace/fence problems, the line.
Counts summarize documents, link occurrences/definitions and selected metadata
sources. A used reference definition can contribute both a link occurrence and
a definition; the count is not a unique-URL census. Exit 0 means the selected
checks passed; a reported problem or unreadable input exits 1. Remaining explicit
inputs are still checked after one unreadable file.

## Checks and interpretation

### Ordinary Markdown

- Leading YAML, when present, uses the existing `rekon-markup` parser: malformed
  or non-object frontmatter is reported even without `--okf`.
- Markdown links, images, autolinks, effective reference definitions (including
  unused definitions), and literal HTML `href`/`src` attributes supply destinations.
  The parser distinguishes code examples and HTML blocks from real Markdown
  definitions, including multiline and nested definitions.
- Relative paths resolve beside the checked file. `/...` paths use its selected
  bundle root. `file:///...` and `file://localhost/...` address absolute local
  paths; other file authorities are reported unsupported. Percent-encoded UTF-8
  paths are decoded. Network-path links (`//host/...`) and other URI schemes are
  external and skipped, without network access.
- Fragment-only links use the document's own anchors. Links to `.md` files also
  check their heading slugs and literal explicit `id` values. Repeated heading
  slugs receive collision-free numerical suffixes; duplicate explicit IDs in the
  checked document are reported. Target document frontmatter is excluded.
- Fenced blocks must have an explicit closing fence, even though CommonMark permits
  implicit EOF closure. Real nested fences are checked; fence-looking text in
  indented code or HTML is not reinterpreted as another Markdown fence.
- Trailing spaces/tabs outside code blocks are reported. Exactly two trailing
  spaces remain a legal Markdown hard break. Fenced and indented code preserve
  their own whitespace. These style checks concern the body, not YAML formatting.

### Knowledge profile

`--okf` requires non-null `type`, `title`, `description`, `resource`, `status` and
`sources` fields in leading YAML. `sources` must be a list with string `resource`
entries. Local source resources use the same destination/anchor checks as links.
At least one source must have a native `opencode://ses_...` authority; qualified
and message-specific sources are recognized by that authority too. Legacy URNs
can coexist with it and are not rewritten.

This is a focused presence/provenance profile, not a complete OKF schema validator.
It does not prove that a session exists, that it is the current contributing
session, or that a cited source supports a claim. Author/model/date and source
truth remain part of [writing practice](/constitution/practice/GLOBAL.md) and
[source practice](/refs/sources.md). Use `okf extract` for scoped static metadata
in enriched documents; generated `AGENTS.md` has scoped `Okf` elements rather than
the leading knowledge frontmatter required by this profile.

## Parser and evidence boundaries

The command uses pulldown-cmark with the assembly's GFM options, the existing
frontmatter parser, literal HTML-attribute extraction and `rekon-toc`'s slug
function. Its slug convention is approximately GitHub-style, with Rekon's
`section` fallback for punctuation-only headings. It does not promise identical
anchors on every host. `.rs#L...` source-line fragments and fragments on other
non-`.md` targets are not checked. Reference definitions shadowed under Markdown's
first-definition rule are not a second effective destination.

This is a static Markdown checker, not an MDX evaluator, HTML browser or JavaScript
runtime. Dynamic destinations, renderer-specific anchors and HTML entity handling
outside the Markdown parser need their own interpretation. It checks local path
existence and selected anchors, not remote availability, recursive link closure,
cross-store session resolution or semantic/architectural correctness. A source
that exists only at a cited historical revision is not automatically materialized.

The original experiment was a Python scratch checker used while writing Kubeless's
diff-dirs consumer account. This implementation uses maintained Rust parsers rather
than reproducing that experiment's regular expressions or depending on its files.
The library entry is `rekon::check::check_document` with supplied `DocumentFiles`;
CLI filesystem/root selection is separate.

## Verification and related tools

The tests cover native/reference/code boundaries, nested and mismatched fences,
heading collisions, local/file/network destinations, source provenance, external
repository root discovery and failure without writes. Relevant commands:

```sh
cargo nextest run --manifest-path tool/rust/Cargo.toml --workspace
cargo clippy --manifest-path tool/rust/Cargo.toml --workspace --all-targets -- -D warnings
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- combine --check
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- audit-context
```

- [Toolkit map](/tool/README.md): command inventory and domain owners.
- [Ambient tool guidance](/tool/GLOBAL.md): selected into generated operating context.
- [Assembly](/tool/assembly/README.md): `combine` and byte freshness; separate from
  document correctness.
- [Coverage audit](/tool/assembly/coverage.md): checks operating coverage even when
  assembly bytes are fresh.
- [Navigation](/tool/toc/README.md) and [OKF extraction](/tool/okf/README.md):
  related structural operations over the same toolchain.
