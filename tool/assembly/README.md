---
type: ToolContract
title: GLOBAL-fragment assembly
description: The independently owned Effect implementation of Rekon's manifest-selected GLOBAL.md assembler.
resource: /tool/assembly/README.md
tags: [tool, agents, assembly, effect]
status: draft
generated: { by: "model:gpt-6.1-sol-xhigh", at: 2026-10-06 }
verified: { by: "model:gpt-6.1-sol-xhigh", at: 2026-10-06 }
sources:
  - {
      resource: /constitution/agents-assembly.md,
      title: Legacy assembler contract,
      author: "project:rekon",
    }
  - {
      resource: /design/rekon/agents-effect0.sol61xh.md,
      title: Verified Effect generation and corrected toolkit placement,
      author: "model:gpt-6.1-sol-xhigh",
    }
---

# GLOBAL-fragment assembly

This is the new Effect assembler's domain home inside the shared `tool/`
toolkit. **It is independent of the legacy Gunshi implementation**, which remains
available through `node rekon.ts agents`. `t` is the toolkit's shortcode, not a
physical directory name.

**Current standing:** the independently owned Effect command is runnable at
[`tool/agents.ts`](/tool/agents.ts). Fragment/manifest validation, ordered
assembly, atomic write/check, and CLI behavior have colocated tests. Since
2026-10-07 it is the production assembler: the root manifest
[`agents.manifest.json`](/agents.manifest.json) selects five fragments
(glossary, constitution, code, praxis-ticket-pointers, refs) into
[`AGENTS.md`](/AGENTS.md), and the hand-maintained predecessor is preserved
at [`AGENTS.old.md`](/AGENTS.old.md).

Tool work must not regenerate or retarget that output as a side effect: edit
fragments or the manifest deliberately, then re-run the command. No automatic
discovery, pruning, or content migration is authorized here.

## Run the new command

Execute TypeScript directly; no distribution build is needed:

```sh
node tool/agents.ts --help
node tool/agents.ts --manifest path/to/manifest.json
node tool/agents.ts -m path/to/manifest.json --check
node tool/agents.ts -m path/to/manifest.json --output .test-agent/agents-effect/preview.md
```

The live document is regenerated with a bare `node tool/agents.ts` (the root
`agents.manifest.json` is the default manifest); `--check` verifies it against
the declared fragments.

| Option           | Default                             | Meaning                                                                                         |
| ---------------- | ----------------------------------- | ----------------------------------------------------------------------------------------------- |
| `-m, --manifest` | `agents.manifest.json` | Manifest path, resolved from the invocation cwd. The default is the production root manifest. |
| `-o, --output`   | Manifest's `output`                 | Optional destination override, resolved from the invocation cwd.                                |
| `-c, --check`    | Off                                 | Compare bytes without creating directories, staging artifacts, or output.                       |

Write mode exits `0` and reports the output, total UTF-8 bytes and each ordered
fragment's order, ID, path and prose-byte cost. Fresh check exits `0` with a
`fresh:` report. Missing or differing output exits `1` with `stale:` detail;
differences include the first differing UTF-8 byte and both byte lengths.
Validation, input-read and nonmissing output I/O failures also exit `1`, retaining
their path/error context rather than being mislabeled stale. Help exits `0`;
unknown flags exit `1`. Imports do not execute the command, including imports
without `argv[1]`; direct launch through a symlink is supported.

These commands execute the **new Effect** implementation. The separate
`node rekon.ts agents ...` command still executes the unchanged legacy Gunshi
implementation; neither command forwards to the other.

## Declaration and fragment contract

The [legacy contract](/constitution/agents-assembly.md) supplies the compatibility
baseline. A JSON manifest explicitly declares nonempty `fragments` and `output`:

```json
{
  "fragments": ["constitution/GLOBAL.md", "code/GLOBAL.md"],
  "output": "AGENTS.md"
}
```

Paths in this example assume the manifest lives at the bundle root. Fragment
paths are normalized, relative, and have no `.`/`..` segments; they resolve from
the manifest's directory, not the invocation cwd. Output is also normalized and
manifest-relative, but deliberate parent destinations such as `../AGENTS.md`
are allowed. A CLI output override will instead resolve from the invocation cwd.
Neither destination may overwrite the manifest or a selected fragment.
These are lexical Node path guarantees, not symlink confinement.

Directory names such as `..notes/` are accepted: `..notes` is not the `..`
traversal segment. This corrects the legacy guard's overbroad rejection of any
name beginning `..`; actual `.`/`..` segments remain rejected.

Each fragment has frontmatter `id` (lowercase kebab-case), integer `order`,
bundle-root-relative `source`, and `status` (`draft` or `stable`). Assembly
rejects duplicate identity/order and sorts by metadata order, not declaration
order. Provenance fields reject `--` and control characters to keep HTML
comments intact.

The body must lead with its sole top-level heading and contain additional
Markdown. Links/images/reference definitions, including nested or unused
definitions, must use portable destinations: anchors, empty self-links, a
single-root-relative `/...` path, or protocol URIs. Code is not treated as links.
Prose is never rendered back into Markdown: only CRLF-to-LF normalization and
boundary whitespace trimming change source text.

## Output and the intentional header difference

Output begins with a generated-file marker naming `node tool/agents.ts` and
retains the legacy `rekon-fragment` provenance format. This deliberate header
change means a document generated by the old command is stale under the new
tool until an explicit rewrite. The tools do not promise cross-tool byte identity.
Sections have one blank-line boundary and the file ends in a single newline.
Context costs and comparison offsets count UTF-8 bytes, not JS character units.

## Implementation map

- [`../agents.ts`](/tool/agents.ts) owns executable/main detection. It
  asynchronously realpaths the invocation before dynamically loading the command.
- [`../cmd/agents.ts`](/tool/cmd/agents.ts) owns Effect CLI flags, human reports
  and stale-check failure. Its embeddable `runAgents(args)` is an Effect; `main`
  supplies `NodeServices.layer` and runs it with the Node runtime.
- [`manifest.ts`](/tool/assembly/manifest.ts) owns manifest schemas, selected
  input paths, output planning, duplicate-path and overwrite guards. Planning
  takes the invocation cwd explicitly and performs no I/O.
- [`assemble.ts`](/tool/assembly/assemble.ts) owns filesystem orchestration,
  unique identity/order, sorted rendering, publication, and checking. It reads
  at most eight fragments concurrently while preserving declared results before
  sorting. All validation finishes before any output directory or file is created.
- [`../md/fragment.ts`](/tool/md/fragment.ts) owns assembler-specific fragment
  metadata/frontmatter, prose structure, and relocation safety. It returns
  named Effects with schema-tagged failures; it needs no runtime services.
- [`../md/document.ts`](/tool/md/document.ts) owns pure generated rendering and
  byte comparison. Callers supply validated, ordered fragments.
- Tests are colocated beside each boundary; Vitest discovers `tool/` as well as
  the unchanged legacy `src/` tests.

The filesystem functions accept an `AssemblyRequest` containing `manifestPath`,
explicit `cwd`, and `outputOverride: Option<string>`, and require upstream
`FileSystem`. `assemble` returns the ordered fragments, document, UTF-8 byte
cost and absolute output without writing. `writeAssembly` stages a complete
document in a uniquely owned directory under the output parent, then renames
it into place on the same filesystem. Scope cleanup removes that staging
directory on success, failure and cooperative interruption; it never reconciles
undeclared artifacts or removes the output parent. `checkAssembly` catches only
`PlatformError` with `reason._tag === "NotFound"` when reading the output.
Permission and directory/read errors remain platform failures.

Atomic rename is not a promise of crash-durable/fsynced publication. Cleanup
cannot run after forcible process termination (`SIGKILL`) or a host crash; an
owned staging artifact may then remain for deliberate cleanup. Lexical path
guards do not defend against symlink aliases or concurrent hostile filesystem
changes. Those stronger policies are not part of this compatibility contract.

## Effect tooling

The tested runtime generation is `effect@4.0.1` and
`@effect/platform-node@4.0.1`. CLI composition uses integrated `effect/cli`, not
the v3-only standalone `@effect/cli`. The new assembler has no artificial
business-service class: named Effects compose upstream platform services at
the application boundary.

The root tsconfig enables `@effect/language-service`. Editor/plugin support uses
workspace TypeScript 5.9.3 because TypeScript 7's native compiler cannot load this
plugin, and the existing distribution tool requires TS 5. `pnpm typecheck` still
uses the existing `tsgo`; adding editor support does not replace it. The actual
language-service resolution is 0.87.3 after `pnpm install ...@latest`.

Verification commands:

```sh
pnpm exec vitest run tool
pnpm exec tsgo --noEmit
pnpm exec oxlint tool
pnpm exec oxfmt --check tool/agents.ts tool/assembly tool/cmd tool/md
pnpm exec effect-language-service diagnostics --project tsconfig.json --format text --severity error,warning --strict
```

Runtime tests cover bounded reads, unique order/identity, validation-before-write,
exact snapshots around checks, parent/override paths, retained platform errors,
publication failure and interrupted staging. Subprocess tests exercise real
exit codes/reports, aliases, the default manifest, help/parser errors, import
safety and symlink launch. Review the
[design checkpoint](/design/rekon/agents-effect0.sol61xh.md) for upstream evidence
and the physical-layout correction.
