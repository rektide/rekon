---
type: Toolkit
title: Rekon tools
description: The shared tool toolkit, shortcode t, with root-level entrypoints and domain-grouped source and documentation.
resource: /tool/README.md
tags: [rekon, tools, cli, structure]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-10-06 }
sources:
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
| [`t-agents`](/tool/assembly/README.md) | Assemble explicitly selected `GLOBAL.md` fragments into a provenance-marked `AGENTS.md`; check for drift | Runnable Effect CLI at [`tool/agents.ts`](/tool/agents.ts), with `cmd/`, `assembly/`, and `md/` source. The [Gunshi assembler](/constitution/agents-assembly.md) remains runnable. **Production producer since 2026-10-07.** |
| [`audit-context`](/tool/assembly/coverage.md) (rust) | Check the reviewed legacy-section mapping, selected fragments, and required instructional excerpts | Runnable `rekon audit-context` under `tool/rust/`; included in `pnpm check` beside byte freshness. Read-only; detects the fresh-but-incomplete composition failure. |
| `t-combine` (rust) | Regenerate `/AGENTS.md` from the manifest as the `combine` subcommand of the rust super-tool | Crate at [`tool/rust/`](/tool/rust/) (bin `rekon`), accepted 2026-10-08 as the going-forward shape: rust-first new code, no TS composition layers. Parity-tested against the Effect tool before the producer flips. |
| [`t-interpolate`](/tool/cmd/interpolate.md) | Expand prompt-template arguments | Existing [Gunshi command](/src/command/interpolate.ts), with its guide now under `tool/cmd/`. Adaptation deferred pending a value call. |
| [`t-install-commands`](/tool/cmd/install-commands.md) | Legacy prompt-command installer and its concurrency proposal | [Gunshi source](/src/command/install-commands.ts) remains; the prompt collection is [retired](/prompt/.archive/README.md). |

Run the existing tools from source with `node rekon.ts <command>`. Each new
tool documents its own `node tool/<name>.ts` entrypoint when it is runnable.
The original CLI also owns `combine`, `project-files`, and `ln`; their source
remains in [`src/command/`](/src/command/) pending deliberate adoption here.

The first new command is ready to use with an explicitly selected manifest:

```sh
node tool/agents.ts --help
node tool/agents.ts --manifest path/to/manifest.json
node tool/agents.ts --manifest path/to/manifest.json --check
```

The [assembly guide](/tool/assembly/README.md) owns the format, output behavior,
and implementation map. Its command uses Effect 4's integrated CLI; the older
`node rekon.ts agents` still invokes Gunshi.

## Markdown and knowledge tooling

The GLOBAL assembler is one tool in a broader repertoire: Markdown structure,
ToCs, anchors, links, and navigable views are tool concerns too. The
[Hyalo ToC investigation](/design/rekon/hyalo-toc0.glm53m.md) recovered the
working script at [`~/ado/bin/hyalo-toc.ts`](file:///home/rektide/ado/bin/hyalo-toc.ts).
It uses `hyalo find --fields sections,lines` to put hidden line ranges in a
managed ToC, then converges after insertion shifts the indexed headings.
Its `--check` mode detects drift. **Direction since 2026-10-08** (accepted
[toolchain design](/design/toolchain/draft0.glm53m.md)): the TS script is a
*reference* adapted into the legacy `src/` tree; the production ToC arrives
as a rust subcommand of `rekon` after `combine`.

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
