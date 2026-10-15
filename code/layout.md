---
type: Guide
title: "Project layout"
description: How repositories are scaffolded and restructured here: domain-grouped top-level directories, never a flat dump, no package-prefix repetition in the tree, registry prefixes carried by package names instead of directories, and worked good/bad trees for multi-crate Rust and multi-package pnpm workspaces.
resource: /code/layout.md
tags: [code, layout, structure, workspace, crates, packages, scaffolding]
status: draft
generated: { by: model:zai-coding-plan/glm-5.3-max, at: 2026-10-16 }
stale_after: 2027-04-16
sources:
  - id: agents-operating-manual
    resource: /AGENTS.md
    title: "The layout rule of the workspace operating manual this guide extends (human-codified)"
    author: human:rektide
    last_modified: 2026-09-04
  - { resource: "opencode://ses_eb9709a91ffeKsq7fEUI1ptzt2", title: Human direction to make layout its own practice — no package prefixes in the tree, bucket policy, registry names keep the full prefixed name }
---

<a id="layout"></a>

# Project layout

<a id="layout-domain-grouped"></a>

## Domain-grouped top-level directories

A repository's top-level directories are **domains or roles** —
`parser/`, `storage/`, `cli/`, `net/` — never a flat dump of files, and
never a single generic bucket as the only structure.

A single `crates/` or `packages/` bucket is acceptable **only while the
repository is single-product** and small. The moment a second product or
cross-cutting concern arrives, split the bucket into domain directories at
the top level. Growth rule: when a domain gains a second member, it becomes
a directory holding both.

When asked to propose a layout, offer **multiple** plausible domain
groupings — the grouping is a design decision the human should get to
weigh, not a detail to bury.

<a id="layout-no-prefixes"></a>

## No package prefixes in the tree

Workspace members never repeat the project prefix in their directory
names. The repository name already says it once.

```text
bad:   ~/src/<pkg>/crates/<pkg>-parser
       ~/src/<pkg>/crates/<pkg>-storage
good:  ~/src/<pkg>/crates/parser        (single-product, one bucket)
       ~/src/<pkg>/parser/              (grown: domain dirs at top)
```

<a id="layout-registry-names"></a>

## Registry names keep the prefix

Published names are the exception: when a registry needs unique names, the
package **name** keeps the full specified name with the project prefix —
while the directory stays prefix-free.

- Rust: `crates/parser/Cargo.toml` declares `name = "<pkg>-parser"`.
- pnpm/npm: `packages/cli/package.json` declares `"name": "@<pkg>/cli"`.

The tree is for the humans navigating it; the manifest is for the registry.
Do not leak registry-naming pressure into directory structure.

<a id="layout-examples"></a>

## Worked trees

```text
bad — prefix echo, bucket as junk drawer:
  <pkg>/
    crates/
      <pkg>-core/
      <pkg>-parser/
      <pkg>-parser-net/
      <pkg>-cli/

good — grown multi-domain Rust workspace:
  <pkg>/
    parser/
      syntax/            # <pkg>-parser-syntax
      stream/            # <pkg>-parser-stream
    storage/
      page/              # <pkg>-storage-page
    cli/                 # <pkg>-cli
    xtask/               # build tooling role

good — single-product pnpm monorepo (bucket still fine):
  <pkg>/
    packages/
      core/              # @<pkg>/core
      cli/               # @<pkg>/cli
    web/                 # role, not package
```

<a id="layout-cross-references"></a>

## Cross-references

- [`/code/README.md#code-layout`](/code/README.md#code-layout) — the
  common-weight summary.
- [`/code/planning.md#planning-layout`](/code/planning.md#planning-layout) —
  plans propose layouts; this guide owns the tree-shape rules they follow.
- [`/code/rust/GLOBAL.md`](/code/rust/GLOBAL.md) and
  [`/js/README.md`](/js/README.md) — the toolchains these layouts host.
