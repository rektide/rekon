---
type: Design
title: Enriched whole-document assembly
description: Implementation boundary for Rust combine, scoped OKF emission, semantic definitions, and subsequent ToC and extraction tools.
status: draft
generated: { by: "model:openai/gpt-6-astra", at: 2026-10-11 }
sources:
  - { resource: "opencode://ses_ed31715c4ffex2ROd2G2C617AF", title: Enriched assembly integration, ToC review, and October 12 section-ID simplification }
  - { resource: "urn:opencode:session:ses_ed31715c4ffex2ROd2G2C617AF", title: Whole-file assembly, enriched output first, semantic definitions, and parallel Rust tooling commission }
  - { resource: /design/toolchain/handoff0.gpt6a.md, title: Earlier combiner handoff, superseded on output and scope }
  - { resource: /okf/README.md, title: Existing OKF conventions and file-level scope }
  - { resource: /tool/assembly/README.md, title: Current production assembly guarantees }
  - { resource: /design/rekon/hyalo-toc0.glm53m.md, title: ToC lineage and post-insertion line accounting }
  - { resource: "https://mdxjs.com/docs/what-is-mdx/", title: MDX syntax and Markdown interoperability differences }
  - { resource: "https://mdxjs.com/guides/frontmatter/", title: Frontmatter is an MDX extension, not built-in metadata semantics }
  - { resource: "https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dfn", title: Definition semantics and linkable terms }
  - { resource: "https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dl", title: Term-description groups }
---

# Enriched whole-document assembly

## Commission and standing

The human selected whole-file inclusion, enriched assembly first, subsequent
reader/editor tools, semantic definition elements, and Rust implementation.
The earlier handoff's exact-output/parity framing does not settle this design.
The implementation contract below is the parent's initial integration choice;
it is open to refinement through working output. Web and TUI components are
future consumers, not prerequisites.

## Initial wire contract

An assembled document has a single H1. Each input document becomes a section,
with its leading H1 shifted to H2 and its descendants shifted consistently.
Actual headings, not heading-like code, define nested section boundaries.
All selected body content is included; there is no subsection selection.

```mdx
<section id="code" data-source="/code/GLOBAL.md">

<Okf data={{"type":"Practice","title":"Code","status":"draft"}} />

## Code

<section id="code-terms">

### Terms

<dl>
  <dt><dfn id="checkpoint">Checkpoint</dfn></dt>
  <dd>A preserved state against which later work can be compared.</dd>
</dl>

</section>
</section>
```

`Okf` is a proposed component with one `data` prop holding a JSON-compatible
literal object. The emitter uses JSON serialization inside JSX braces, rather
than inventing attribute names for arbitrary YAML keys. Complete frontmatter,
including unknown fields and nested values, survives. Unsupported YAML values
must fail with context instead of silently losing data. This is a static data
interface: extraction parses literals and never evaluates JavaScript.

The nearest enclosing section/article owns an `Okf` record; a record outside
one belongs to the document. Descendants do not inherit verification or other
metadata implicitly. Input frontmatter is emitted once on its document section.
Subsections can already carry their own explicit records; headings alone do not
invent metadata. HTML sections and `data-source` identify assembly structure
and origin separately from OKF's `resource` field.

`dfn`, `dl`, `dt`, and `dd` carry vocabulary semantics. Vocabulary remains with
guidance in GLOBAL contributions. CONTEXT.md is the initial semantic conversion
case; preserve its existing path/anchors while the broader home migration is
considered. No separate GLOSSARY file format is introduced.

**October 12 naming correction:** the human found that `fragment-` carried no
useful meaning in IDs. Document sections now use the declared identity directly
(`code`), with children such as `code-terms`. The human also rejected doubled
hyphens: hierarchy and heading slugs use single hyphens throughout. The renderer checks actual IDs
against source anchors across the whole selection, replacing the old blanket
prefix reservation. The examples above reflect this correction.

## Rust boundaries

Keep `tool/rust/Cargo.toml` as the workspace root and existing `rekon` package.

- `crates/rekon-markup`: reusable Markdown/MDX structure, frontmatter and static
  OKF metadata reading/emission. Used by assembly, extraction, and ToC as useful.
- `crates/rekon-toc`: ToC generation and managed-region replacement, with a pure
  render/update API independent of the CLI. Initially it uses the same maintained
  `markdown` parser directly: its document reader also needs navigation
  boundaries, explicit anchor bindings, and IDs that the metadata library's
  public result does not expose. Static metadata reading remains in markup.
- Existing `src/`: manifest/path planning, coverage audit, assembly orchestration,
  publication/checking, and thin CLI commands.

This chooses shared document semantics over a crate per command. Keeping all
domains as modules was the smaller alternative; the independently usable ToC
and metadata consumers justify these two library boundaries. Rendering adapters
can depend on their data contract later without making the engine depend on UI.

## Assembly and ToC

Keep existing explicit manifest selection and metadata order for this landing.
Preserve deterministic output, input protection, validation-before-publication,
atomic writing, and read-only byte freshness checks. Output gains readable
generated ownership, scoped metadata, section nesting, and a final ToC pass.
ToC must count the final document if it emits line ranges, and updating it twice
must be idempotent. A semantic `<nav>` region is preferred to comment directives.

Composition must handle link targets, explicit IDs, reference labels, and heading
depth deliberately. Do not silently clamp deep headings or let duplicate IDs
retarget references. Preserve prose source where possible. The output remains
agent-readable text; support for a complete interactive MDX runtime is not needed
to assemble it. Honest syntax limitations must be documented and tested.

## Proof and relationships

The first proof is actual enriched assembly of the live manifest, followed by
freshness and operating-coverage checks. Extraction and ToC have focused tests;
an integration case must extract a source's metadata from combined output.

The semantic vocabulary conversion exposed a real MDX authoring constraint:
multiline description bodies put `<dd>` on its own line, then their prose,
then `</dd>`. Keeping the opening tag and initial prose on one line while closing
in another block fails the JSX inline/flow boundary. CONTEXT's conversion was
corrected against the shared parser before integration.

### Landed implementation checkpoint — October 11

Rust `combine` now produces the live `AGENTS.md`: 12 complete fragments,
43,325 UTF-8 bytes, scoped frontmatter, nested sections, semantic definitions,
and 27 navigation entries with final physical line ranges. `pnpm check` now
selects the Rust producer and passes freshness plus 108 declared coverage
excerpts across 13 legacy sections. Standalone ToC freshness also passes.

Workspace nextest passed all 101 tests; strict workspace clippy, Rust formatting,
and the changed package manifest's formatting passed. Cross-engine tests recover
both whole-document and authored subsection metadata, retain HTML-looking JSON
strings, exclude commented-out records, and exercise read-only checking and
symlink-target ToC publication. Review replaced unsafe ToC boundary searches with
parser-confirmed isolated elements and removed the superseded HTML scanner.

The component interface remains static data transport. Web/TUI presentation and
broader source-document relocation remain the subsequent directions discussed
in the commission; this checkpoint demonstrates working assembly and readers.

- [Assembly home](/tool/assembly/README.md): production behavior to replace and
  operational guarantees to retain.
- [OKF home](/okf/README.md): file-level upstream semantics; scoped elements are a
  Rekon representation, not an upstream OKF standard.
- [Prior handoff](/design/toolchain/handoff0.gpt6a.md): recovery evidence and
  implementation background; this commission reopens its markup assumptions.
- [ToC research](/design/rekon/hyalo-toc0.glm53m.md): useful line-range and
  idempotence precedent, with comment markers intentionally replaced here.
