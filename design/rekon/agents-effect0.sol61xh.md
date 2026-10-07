---
type: ImplementationDesign
title: An independently owned Effect agents assembler
description: Verified Effect 4 APIs, domain boundaries, and compatibility commitments for the agents assembler in Rekon's co-integrated tool/ toolkit.
resource: /design/rekon/agents-effect0.sol61xh.md
tags: [rekon, tools, agents, assembly, effect, cli]
status: draft
generated: { by: "model:gpt-6.1-sol-xhigh", at: 2026-10-06T20:33:11-04:00 }
revised: { by: "model:gpt-6.1-sol-xhigh", at: 2026-10-06T20:37:58-04:00 }
stale_after: 2026-11-06
sources:
  - id: task
    resource: urn:opencode:session:ses_eec3a18b8ffeOUokLEAa8Fbqjk
    title: Parent-directed implementation and architectural checkpoint
    author: agent:parent
  - id: human-layout
    resource: urn:opencode:session:ses_eec5afbf4ffeIhL6wxmv8NF3gn
    title: Human clarification of shortcodes and selection of one tool/ toolkit
    author: human:rektide
  - id: contract
    resource: /constitution/agents-assembly.md
    title: Existing GLOBAL.md assembly contract
    author: project:rekon
  - id: legacy
    resource: /src/agents/assemble.ts
    title: Legacy Gunshi assembler behavior
    author: project:rekon
  - id: effect-published
    resource: https://unpkg.com/effect@4.0.1/package.json
    title: Published Effect 4.0.1 package exports
    author: project:Effect-TS
  - id: node-published
    resource: https://unpkg.com/@effect/platform-node@4.0.1/package.json
    title: Published compatible Node platform package
    author: project:Effect-TS
---

# Situation and scope

Rekon needs a new Effect CLI assembler, not a migration of the existing
Gunshi command. The human chose one co-integrated `tool/` toolkit; `t` is only
its shortcode namespace. The parent's initial `t/agents/` placement proposal
conflated shortcodes with physical layout and is superseded. This implementation will not import the old assembler,
alter its tests, create a production manifest, or regenerate the handwritten
[`AGENTS.md`](/AGENTS.md). The parent owns shared tools/placement documentation.

The behavior to preserve is the
[`assembly contract`](/constitution/agents-assembly.md): explicit selection,
manifest-relative input paths, metadata order, validated Markdown, source-text
preservation, provenance, UTF-8 costs, atomic writes, and non-writing checks.
The old command remains separately executable through `node rekon.ts agents`.

# Verified package generation

**Use Effect 4, not the standalone v3 CLI package.** Registry inspection on
2026-10-06 reports `effect@latest = 4.0.1`,
`@effect/platform-node@latest = 4.0.1`, and
`@effect/language-service@latest = 0.87.4`. In contrast,
`@effect/cli@latest = 0.77.2` requires `effect ^3.22.2` and the v3 platform.

The local archive was consulted first. Its Effect checkout currently labels
the core package `4.0.0-rc.117`, so its source is useful evidence but the
published 4.0.1 exports/declarations were checked independently:

| Capability | Verified API and evidence |
| --- | --- |
| CLI | `Command.make`, `Command.runWith`, `Flag.String`, `Flag.Boolean`, `Flag.withAlias`, `Flag.withDefault`, `Flag.optional` from [`effect/cli`](https://unpkg.com/effect@4.0.1/dist/cli/Command.d.ts). Flags accept only the argument tail; `runWith` does not expect Node executable/module arguments. |
| Platform | [`NodeServices.layer`](https://unpkg.com/@effect/platform-node@4.0.1/dist/NodeServices.d.ts) provides filesystem, paths, terminal, stdio, child process and crypto services. [`NodeRuntime.runMain`](https://github.com/Effect-TS/effect/blob/main/packages/platform/node/src/NodeRuntime.ts) owns interruption and process exit. |
| Files and ownership | [`FileSystem`](https://github.com/Effect-TS/effect/blob/main/packages/effect/src/FileSystem.ts) offers `readFileString`, byte `readFile`, `makeDirectory`, `makeTempDirectoryScoped`, and `rename`. A scoped staging directory under the output parent keeps rename on the same filesystem. |
| Expected failures | [`Schema.TaggedError`](https://github.com/Effect-TS/effect/blob/main/packages/effect/src/Schema.ts) remains available. Platform I/O failures use `PlatformError` with `reason._tag`; only `NotFound` means missing output. |
| Schema and effects | `Schema.decodeUnknownEffect`, `Schema.Struct`, schema checks/brands, `Effect.fn`, `Effect.forEach`, `Effect.catchTag`, `Option`, and `Layer` are present in the archive's v4 source. Installed declarations will be the final type authority. |
| Editor tooling | [`language-service README`](https://github.com/Effect-TS/language-service/blob/main/README.md) documents a root tsconfig plugin and workspace TypeScript, and explicitly supports v4 diagnostics. Install the language service plus `typescript@latest` for editor use; keep the existing `tsgo` check unchanged. |

**Skill adaptation:** the loaded Effect skill describes v3 `Effect.Service`
with automatic accessors/default layers. That API is absent in this v4 source;
v4 has `Context.Service` instead. This stateless assembler does not need a
business-service class merely to wrap functions. It will use injected upstream
filesystem/path services and an application-root layer, named effect functions,
schema-tagged domain errors and `Option` for the optional override. If a genuine
owned service emerges, use v4 `Context.Service` with an explicit layer, not a
pretend v3 compatibility abstraction.

The public `effect/cli` export is nevertheless annotated **unstable**. The lockfile
pins the tested resolution; future package updates require CLI regression tests.

# Domain layout and interface

```text
tool/
  agents.ts                  executable, import-safe entrypoint
  README.md                  toolkit landing page (parent-owned)
  cmd/
    agents.ts                typed flags, command handler, reports
    agents.test.ts           actual source-executed CLI tests
  assembly/
    README.md                assembler contract and implementation map
    manifest.ts              declaration schema, path planning, collision guards
    assemble.ts              read/parse/order/render, write/check effects
    assemble.test.ts          filesystem workflows and no-write guarantees
  md/
    fragment.ts              frontmatter/metadata and Markdown relocation checks
    document.ts              pure rendering, provenance, UTF-8 bytes/differences
    fragment.test.ts          targeted text/metadata safety cases
```

These are behavioral seams: `md/` cannot perform filesystem I/O; `assembly/`
does not parse CLI tokens or report to a terminal; `cmd/` cannot reimplement
ordering, validation, or byte comparisons. Domain types/errors stay beside the
behavior that owns them, not in a generic `lib/` or shared catch-all file.

The application-facing operations are `assemble`, `writeAssembly`, and
`checkAssembly`: named Effects accepting a manifest path and an
`Option<string>` output override. Assembly returns ordered fragments, content,
total UTF-8 bytes, and the resolved output path. Check returns a `fresh | stale`
result with stale detail; stale is a CLI failure, not a filesystem error.
Expected manifest/fragment/identity/output-collision failures have specific
schema-tagged errors with paths and messages. Output I/O failures remain
upstream platform errors instead of being mislabeled staleness.

The source command is `node tool/agents.ts [-m PATH] [-o PATH] [-c]`, with
the existing defaults and aliases. The entrypoint asynchronously realpaths
`process.argv[1]` using `.then`, handles its absence, and launches the Node
runtime only when the canonical file URL matches `import.meta.url`. Imports
must neither parse argv nor perform assembly. All local imports include `.ts`.

# Compatibility and limits

- Preserve the manifest/fragment format, ascending unique order, provenance
  comment shape, and meaningful command reports. Use `gray-matter` and the
  existing `marked` lexer as parsing dependencies, not a Markdown renderer.
- Preserve body bytes except CRLF-to-LF and boundary whitespace trimming.
  Fragment paths/source metadata reject comment-breaking text and controls;
  nested links/images/reference definitions are checked, while code is ignored.
- The generated header intentionally names `node tool/agents.ts`, not
  `rekon agents`. Thus legacy output is stale under the new tool until explicitly
  rewritten; do not claim cross-tool byte identity.
- Preserve lexical filesystem path semantics. Fragment declarations have no
  `.`/`..` segments; deliberate parent output is allowed. Relative output overrides
  use the invocation cwd. This is not a symlink sandbox or realpath-based
  confinement policy; strengthening that policy would require separate design.
- Never write until all inputs validate. Check cannot create even an output
  directory. Atomic write owns only its temporary staging artifact; scoped
  cleanup is not pruning/reconciliation of undeclared files.

# Proposed first implementation slice and checkpoint

After parent approval, install `effect@latest` and
`@effect/platform-node@latest`, add the editor language-service packages/plugin,
extend Vitest discovery to `tool/`, and implement the `md/` and manifest/planning
seams with focused tests. Commit that coherent slice before adding filesystem
write/check orchestration and the CLI. The next checkpoint will show real types,
error boundaries and rendering/path tests before the runnable command lands.

Final verification includes new domain and subprocess CLI tests, all legacy
assembler tests unchanged, `tsgo`, targeted `oxlint`/`oxfmt`, and separate
reporting of pre-existing repo failures. CLI tests cover help/defaults/aliases,
fresh/stale/missing outputs, no writes, read errors, relative overrides,
provenance/order/cost reports, import safety and symlink invocation.

# Cross-references

- [`constitution/agents-assembly.md`](/constitution/agents-assembly.md) supplies
  the exact compatibility contract and the prohibition on premature production
  migration; the parent may move this reference and will coordinate link updates.
- [`legacy command`](/src/command/agents.ts) and
  [`legacy tests`](/src/agents/assemble.test.ts) establish CLI options and observable
  edge cases; they remain unmodified, not dependencies of the new implementation.
- [`ownership decomposition design`](/design/rekon/agents-modularization0.sol56x.md)
  distinguishes implementation ownership from changing ambient context behavior.
  This work adds a tool; it does not authorize either content migration or activation tiering.

# Layout correction checkpoint

The predecessor in commit `2d8e8717` records the parent's original placement
assumption. The human then clarified that **`t` means tool, `p` praxis, and `g`
global as shortcodes, not directories**, and selected the spelled-out `tool/`
home. This correction preserves that design's verified Effect generation and
domain split while replacing the mistaken physical placement. Command and
implementation directories are shared toolkit domains, not nested beneath an
`agents/` tool directory. The parent approved this corrected first slice before
dependency installation or implementation began.
