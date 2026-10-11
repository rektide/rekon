---
type: Toolkit
title: Rekon tools
description: The shared tool toolkit, shortcode t, with root-level entrypoints and domain-grouped source and documentation.
resource: /tool/README.md
tags: [rekon, tools, cli, structure]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-10-06 }
sources:
  - { resource: "opencode://ses_ed31715c4ffex2ROd2G2C617AF", title: Integrate the enriched assembly and metadata toolchain }
  - { resource: "urn:opencode:session:ses_ed31715c4ffex2ROd2G2C617AF", title: Enriched Rust assembly, semantic ToC, scoped OKF extraction, and workspace integration }
  - { resource: "urn:opencode:session:ses_eec5afbf4ffeIhL6wxmv8NF3gn", title: Human distinction between shortcodes and directories and selection of one tool toolkit }
  - { resource: /constitution/structure.md#tools, title: Co-integrated tool layout }
  - { resource: /rekon.ts, title: Existing Gunshi command registry }
  - { resource: /constitution/agents-assembly.md, title: Existing GLOBAL-fragment assembler contract }
---

<a id="tools"></a>
# Tool toolkit — shortcode `t`

**Tools make Rekon's practices easier to perform.** This is their executable
home: entrypoints live at the top of `tool/`, with shared implementation and
explanation grouped underneath by concern. The [structure contract](/constitution/structure.md#tools) describes
the layout and the [naming home](/constitution/naming/README.md#short-codes)
records `t` (tool), `p` (praxis), and `g` (global). These are shortcodes,
not abbreviated directory names.

## First homes

| Tool | Job | Implementation standing |
|---|---|---|
| [`t-agents`](/constitution/agents-assembly.md) (legacy) | Earlier comment-based fragment assembly | Effect at [`tool/agents.ts`](/tool/agents.ts), Gunshi via `node rekon.ts agents`. Retained as migration evidence; use Rust `combine` for live output. |
| [`audit-context`](/tool/assembly/coverage.md) (rust) | Check the reviewed legacy-section mapping, selected fragments, and required instructional excerpts | Runnable `rekon audit-context` under `tool/rust/`; included in `pnpm check` beside byte freshness. Read-only; detects the fresh-but-incomplete composition failure. |
| [`combine`](/tool/assembly/README.md) (Rust) | Regenerate `/AGENTS.md` from whole fragments, preserving scoped OKF metadata and semantic hierarchy | Rust workspace at [`tool/rust/`](/tool/rust/), binary `rekon`; final ToC pass and read-only byte check. **Production producer after the October 11 enriched-output cutover.** |
| [`toc`](/tool/toc/README.md) (Rust) | Generate/check a semantic navigation region over Markdown/MDX headings | Reusable `rekon-toc` library and `rekon toc FILE...`; final physical line ranges are optional. |
| [`okf extract`](/tool/okf/README.md) (Rust) | Read file frontmatter and section-owned static OKF data | Reusable `rekon-markup` library; JSON/YAML output, optional owner-ID selection, no JavaScript evaluation. |
| [`t-interpolate`](/tool/cmd/interpolate.md) | Expand prompt-template arguments | Existing [Gunshi command](/src/command/interpolate.ts), with its guide now under `tool/cmd/`. Adaptation deferred pending a value call. |
| [`t-install-commands`](/tool/cmd/install-commands.md) | Legacy prompt-command installer and its concurrency proposal | [Gunshi source](/src/command/install-commands.ts) remains; the prompt collection is [retired](/prompt/.archive/README.md). |

Run legacy tools from source with `node rekon.ts <command>`. New Rust commands
run with `cargo run --manifest-path tool/rust/Cargo.toml -- <command>`.
The original CLI also owns `combine`, `project-files`, and `ln`; their source
remains in [`src/command/`](/src/command/) pending deliberate adoption here.

The current assembly, navigation, and inspection commands are:

```sh
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- combine
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- combine --check
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- toc AGENTS.md --line-ranges --check
cargo run --quiet --manifest-path tool/rust/Cargo.toml -- okf extract AGENTS.md --scope fragment-code
```

The [assembly guide](/tool/assembly/README.md) owns the format, output behavior,
and implementation map. The shared markup and ToC libraries live under
`tool/rust/crates/`; the root package owns assembly, the coverage audit, and CLI
orchestration. Web/TUI metadata components are deferred consumers of the same
static `Okf` data interface.

## Markdown and knowledge tooling

The GLOBAL assembler is one tool in a broader repertoire: Markdown structure,
ToCs, anchors, links, and navigable views are tool concerns too. The
[Hyalo ToC investigation](/design/rekon/hyalo-toc0.glm53m.md) recovered the
working script at [`~/ado/bin/hyalo-toc.ts`](file:///home/rektide/ado/bin/hyalo-toc.ts).
It uses `hyalo find --fields sections,lines` to put hidden line ranges in a
managed ToC, then converges after insertion shifts the indexed headings.
Its `--check` mode detects drift. The script remains a historical reference;
the [Rust ToC](/tool/toc/README.md) implements navigation over the enriched
document and is the final pass used by `combine`. The
[current design](/design/toolchain/enriched-assembly0.gpt6a.md) records the
semantic-element and scoped metadata direction.

## Keeping a tool understandable

- Keep executable entrypoints at the top of `tool/`, next to this toolkit map.
- Put substantive source, detailed explanation, and tests under named
  responsibilities, such as `cmd/`, `md/`, `assembly/`, or `hyalo/`; omit
  unused layers. Share a domain implementation when the behavior genuinely is
  shared, without building a separate mini-package for each entrypoint.
- Explain inputs, outputs, failure behavior, and source ownership.
- Distinguish a runnable tool from selecting its output for live use. The
  assembler's existence does not itself cut over the hand-maintained AGENTS.
- Link to [praxis](/praxis/README.md) for the working method and
  [constitution](/constitution/README.md) for the shared obligations it serves.
