---
type: ToolContract
title: Enriched whole-document assembly
description: Rust combine assembles complete GLOBAL contributions with scoped frontmatter, semantic sections, a final ToC, and atomic write/check behavior.
resource: /tool/assembly/README.md
tags: [tool, agents, assembly, rust, mdx, okf]
status: draft
generated: { by: "model:openai/gpt-6-astra", at: 2026-10-11 }
sources:
  - { resource: "opencode://ses_ed31715c4ffex2ROd2G2C617AF", title: Rust producer integration, cross-engine review, and concise section identities }
  - { resource: "urn:opencode:session:ses_ed31715c4ffex2ROd2G2C617AF", title: Enriched whole-file assembly commission, Rust integration, and producer cutover }
  - { resource: /design/toolchain/enriched-assembly0.gpt6a.md, title: Current integration contract }
  - { resource: /design/toolchain/draft0.glm53m.md, title: Earlier Rust-first and combine-first direction }
  - { resource: "opencode://ses_e9bc39334ffetsgyIAj4L0kp0A", title: Human names globbing; contract updated with pattern directives and the section-selection follow-on }
  - { resource: "opencode://ses_e5fb7f311ffeXrWNZ49Ok9Rh2N", title: Adopt wildcard selection with a retired-fragment exclusion and align coverage audit }
  - { resource: /constitution/agents-assembly.md, title: Legacy assembler contract and operational guarantees }
  - { resource: /design/rekon/agents-effect0.sol61xh.md, title: Prior Effect producer and validation evidence }
  - { resource: /design/context-compilation/repair0.gpt6a.md, title: Guidance recovery and independent coverage audit }
---

# Enriched whole-document assembly

**`rekon combine` assembles complete selected files into `AGENTS.md`.** The
Rust producer preserves frontmatter as scoped `<Okf data={…} />` metadata,
builds a section hierarchy, and runs a final ToC pass before writing or checking.
Edit source contributions, then regenerate; the output is owned by the tool.

## Run

From the repository root:

```sh
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- combine
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- combine --check
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- combine --output .test-agent/enriched-assembly/AGENTS.md
pnpm check
```

An installed Rust binary uses the shorter `rekon combine` spelling. The existing
npm/Gunshi executable also has the name `rekon`; the Cargo invocation above
unambiguously selects this producer.

| Option | Default | Meaning |
|---|---|---|
| `-m, --manifest` | `agents.manifest.json` | Selection declaration, resolved from invocation cwd. |
| `-o, --output` | Manifest `output` | Override resolved from invocation cwd. |
| `-c, --check` | Off | Compare complete final bytes without writing. |

Exit 0 means success/freshness; stale or missing output exits 1 with details.
Validation and I/O errors also exit 1, retaining their context. Checks create no
output or staging directories. Help exits 0; invalid CLI arguments exit 1.

## Selection and ordering

The [root manifest](/agents.manifest.json) declares whole inputs and patterns:

```json
{
  "fragments": ["CONTEXT.md", "**/GLOSSARY.md", "**/GLOBAL.md"],
  "exclude": ["doc/GLOBAL.md"],
  "output": "AGENTS.md"
}
```

Every selected body's content is included. There is no subsection selection;
patterns deliberately select matching files. Fragment paths resolve from the manifest directory and
must be normalized, relative, and free of `.`/`..` segments. Manifest output may
deliberately use parent segments. The destination must not overwrite an input.

### Globbing

An entry containing a pattern metacharacter (`*`, `?`, `[`) is a **glob
directive**: the walk finds matching files under the manifest directory and
selects them in deterministic sorted order. The pattern is itself the
declaration — expansion is deliberate selection, not unconstrained discovery:

- every match is revalidated against the fragment path rules and never escapes
  the manifest directory;
- dotfiles and dot-directories are skipped, directories are never selected;
- glob matches through symlinked files or directories are skipped, avoiding alias
  duplication and selection outside the bundle; literal entries remain explicit;
- a pattern matching no files is a hard error, so stale directives fail loudly;
- optional `exclude` entries are unique, normalized literal paths, removed after
  expansion; an exclusion that selected nothing is an error. The root manifest
  excludes the retired `doc/GLOBAL.md` rather than activating it;
- duplicate detection and output-overwrite protection run against the expanded
  set (an explicit entry plus a glob matching the same file is rejected);
- ordering is unchanged: numeric frontmatter `order` still composes the
  document, and each matched file must carry its own full fragment frontmatter
  — identity and provenance stay with owners.

The coverage audit uses the same expanded, filtered selection. A new matching
file becomes ambient on the next successful assembly; review its metadata and
content deliberately. Legacy TypeScript assemblers do not implement this manifest
extension; the Rust producer is the live implementation.

Section selection (`file#tag` entries addressing one section of a larger
document, resolved through the heading/anchor indexing the ToC pass already
builds) is a designed follow-on; it needs an entry grammar that supplies
fragment identity for content whose file does not declare it.

Each source declares `id` (lowercase kebab-case), integer `order`, root-relative
`source`, and `status` (`draft` or `stable`) in YAML frontmatter. IDs and orders
must be unique; numeric metadata order determines composition. Additional
frontmatter fields are preserved rather than discarded.

The initial path validators retain the previous producer's rejection of control
characters and `--` in fragment paths and `source` values. Source fragments must
start with their single Markdown H1 and include content beyond it.

## Enriched output

The output has one document H1 and a semantic navigation region. Each source
becomes a `<section>` carrying an assembly ID and `data-source` path. Its
frontmatter is emitted as one static JSON-compatible object:

```mdx
<section id="code" data-source="/code/GLOBAL.md">

<Okf data={{"id":"code","order":200,"source":"/code/README.md","status":"draft"}} />

## Code

<section id="code-terms">

### Terms

Content remains here, including semantic definition elements.

</section>
</section>
```

The document section uses its source's frontmatter `id` directly (`code`);
nested sections extend it with heading slugs (`code-terms`). Separators are
single hyphens, including within heading slugs; doubled hyphens are collapsed.
There is no
generic `fragment-` prefix. Generated IDs are checked against all selected
source anchors and section IDs; `rekon-toc` remains reserved for navigation.

The source's leading H1 becomes H2; descendants shift consistently and form
nested sections. Hashes inside code remain code. Definitions using `dl`, `dt`,
`dd`, and `dfn` remain semantic content, including their linkable identities.

`Okf` is Rekon's proposed component interface, not an upstream OKF element. Its
`data` prop contains the source metadata, including nested and unknown fields.
The enclosing section owns that record; descendant sections do not implicitly
inherit verification. `data-source` records the source file, independently of
the metadata's `resource` field. Web and TUI renderers can use this same data
interface later. [OKF extraction](/tool/okf/README.md) reads it without evaluating
JavaScript; the [OKF home](/okf/README.md) explains the upstream YAML format.

The final [ToC pass](/tool/toc/README.md) uses the assembled heading hierarchy
and final physical line ranges. Running the same combination again produces the
same bytes. This deliberately differs from the former comment-based output;
byte identity with the TypeScript assembler is not promised.

## Guarantees and boundaries

- Validate all inputs before publication. Missing files, malformed metadata,
  duplicate identities/orders, invalid structure, and unsafe destinations fail.
- Keep root-relative/protocol links portable. Assembly must not silently
  retarget fragment-relative links or duplicate anchor identities.
- Preserve body content while applying structural and MDX-syntax transforms;
  frontmatter survives semantically, not necessarily with original YAML formatting.
- Publish atomically through same-directory staging. This is not a claim of
  crash-durable/fsynced output. Cleanup after forcible termination is deliberate.
- Freshness compares UTF-8 bytes. Missing output is stale; other read failures
  remain errors. No automatic pruning or undeclared-file reconciliation.

Generated sections put blank lines around their element boundaries. The ToC
reader and metadata extractor use the maintained `markdown` parser with GFM and
balanced JSX, keeping metadata strings and comment bodies opaque to document
structure. Metadata extraction additionally requires literal JSON attributes.
Full interactive MDX execution is not part of either tool.

Freshness does not establish completeness or editorial quality. `pnpm check`
also runs the independent [operating coverage audit](/tool/assembly/coverage.md).
That audit checks selected source guidance against its reviewed legacy ledger.

## Implementation and verification

- [`tool/rust/src/assembly/`](/tool/rust/src/assembly/) owns fragment validation,
  composition, and publication/checking.
- [`manifest.rs`](/tool/rust/src/manifest.rs) and
  [`nodepath.rs`](/tool/rust/src/nodepath.rs) own explicit selection/path planning.
- [`rekon-markup`](/tool/rust/crates/rekon-markup/) owns reusable metadata parsing
  and the scoped data interface; [`rekon-toc`](/tool/rust/crates/rekon-toc/) owns
  navigation transforms.
- [`cli/`](/tool/rust/src/cli/) composes engines and reports command results.

```sh
cargo nextest run --manifest-path tool/rust/Cargo.toml --workspace
cargo clippy --manifest-path tool/rust/Cargo.toml --workspace --all-targets -- -D warnings
cargo fmt --manifest-path tool/rust/Cargo.toml --all --check
pnpm check
```

## Earlier producers

`node tool/agents.ts` is the previous Effect assembler. `node rekon.ts agents`
is the older Gunshi implementation. Their code and fixture tests remain as
migration evidence; both emit legacy comment-based output. Use the Rust command
for the live `AGENTS.md`. The [legacy reference](/constitution/agents-assembly.md)
and [earlier Effect account](/design/rekon/agents-effect0.sol61xh.md) explain their
interfaces and historical differences.
