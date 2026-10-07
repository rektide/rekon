---
type: Toolkit
title: Rekon tools
description: The t namespace for executable tools, with each tool's entrypoint, documentation, and domain-grouped source together.
resource: /t/README.md
tags: [rekon, tools, cli, structure]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-10-06 }
sources:
  - { resource: "urn:opencode:session:ses_eec5afbf4ffeIhL6wxmv8NF3gn", title: Human direction for the t tool home }
  - { resource: /constitution/structure.md#tools, title: Co-integrated tool layout }
  - { resource: /rekon.ts, title: Existing Gunshi command registry }
  - { resource: /constitution/agents-assembly.md, title: Existing GLOBAL-fragment assembler contract }
---

<a id="tools"></a>
# Tools — `t`

**Tools make Rekon's practices easier to perform.** This is their executable
home: a tool's explanation, entrypoint, and domain-grouped implementation live
together. The [structure contract](/constitution/structure.md#tools) describes
the layout and the [naming home](/constitution/naming/README.md#short-codes)
records the `t` prefix.

## First homes

| Tool | Job | Implementation standing |
|---|---|---|
| `t-agents` | Assemble explicitly selected `GLOBAL.md` fragments into a provenance-marked `AGENTS.md`; check for drift | Effect CLI implementation being built at `t/agents/agents.ts`. The [Gunshi assembler](/constitution/agents-assembly.md) remains runnable. |
| `t-interpolate` | Expand prompt-template arguments | Existing [Gunshi command](/src/command/interpolate.ts); guide being brought into this home. |
| `t-install-commands` | Install Markdown prompts as command links | Existing [Gunshi command](/src/command/install-commands.ts); historical concurrency proposal being brought into this home. |

Run the existing tools from source with `node rekon.ts <command>`. Each new
tool documents its own `node t/<tool>/<tool>.ts` entrypoint when it is runnable.
The original CLI also owns `combine`, `project-files`, and `ln`; their source
remains in [`src/command/`](/src/command/) pending deliberate adoption here.

## Markdown and knowledge tooling

The GLOBAL assembler is one tool in a broader repertoire: Markdown structure,
ToCs, anchors, links, and navigable views are tool concerns too. The current
Hyalo investigation is recovering the actual ToC implementation and its native
owner before choosing an integration. A `hyalo/` source seam should follow a
real consumer and API, rather than name an empty future subsystem.

## Keeping a tool understandable

- Keep its entrypoint and README at the top of its directory.
- Put substantive source and tests under named responsibilities, such as
  `cmd/`, `md/`, or the tool's own domain; omit unused layers.
- Explain inputs, outputs, failure behavior, and source ownership.
- Distinguish a runnable tool from selecting its output for live use. The
  assembler's existence does not itself cut over the hand-maintained AGENTS.
- Link to [praxis](/praxis/README.md) for the working method and
  [constitution](/constitution/README.md) for the shared obligations it serves.
