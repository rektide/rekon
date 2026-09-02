---
name: agents-assembly
description: Deterministic AGENTS.md assembly from declared GLOBAL.md fragments - fragment metadata, manifest format, and the rekon agents command.
type: Reference
title: Assembling AGENTS.md from GLOBAL.md fragments
resource: /doc/agents-assembly.md
tags: [documentation, agents, tooling]
status: draft
generated: { by: "model:glm-5.3-flash", at: 2026-09-02T02:00:00Z }
verified: { by: "model:glm-5.3-flash", at: 2026-09-02T02:45:00Z }
stale_after: 2026-12-01
extensions:
  ticket: rekon-agents-maintenance-assembler
  fragment_ticket: rekon-doc-constitution-global
sources:
  - id: constitution
    resource: /doc/README.md
    title: Self-Explaining Documentation Constitution
    author: model:gpt-5.6-terra
  - id: global-fragment
    resource: /doc/GLOBAL.md
    title: Constitution ambient fragment
    author: project:rekon
  - id: marked
    resource: https://github.com/markedjs/marked
    title: marked Markdown parser (token/lexer validation)
    author: project:markedjs
  - id: gunshi
    resource: https://github.com/kazupon/gunshi
    title: gunshi CLI framework (command definition)
    author: project:kazupon
---

<a id="agents-assembly"></a>

# Assembling `AGENTS.md` from `GLOBAL.md` Fragments

`AGENTS.md` is becoming deterministic build output. Knowledge modules own small
ambient fragments in their local `GLOBAL.md`; an explicit manifest declares
which fragments assemble and where output lands; `rekon agents` assembles the
document deterministically with per-fragment provenance. Contributors edit
their local fragment, not a monolith.

This is the implementation of
[Assembled `AGENTS.md`](/doc/README.md#doc-constitution-global-assembly) in the
[Self-Explaining Documentation Constitution](/doc/README.md). The module
contract lives there; this document owns the exact metadata, manifest, and
command syntax.

Implementation status: the assembler has been implemented and independently
standards/spec reviewed (2026-09-02); fixes from that review are applied and
covered by tests in [`src/agents/assemble.test.ts`](/src/agents/assemble.test.ts).

<a id="agents-assembly-status"></a>

## Migration Status: Deliberately Deferred

**No production manifest exists, and the current hand-maintained
[`/AGENTS.md`](/AGENTS.md) is untouched.** Only one fragment
([`/doc/GLOBAL.md`](/doc/GLOBAL.md)) has been extracted so far. A production
manifest that would overwrite the monolith must not be created until every
section it contains has a declared `GLOBAL.md` source fragment; otherwise
assembly would silently discard unpromoted operating rules.

Until then:

- `AGENTS.md` remains hand-maintained and is NOT generated output;
- new `GLOBAL.md` fragments can land beside it as declared future inputs;
- a production manifest lands in the same change that retires the monolith,
  once its last section has a fragment source.

<a id="agents-assembly-metadata"></a>

## Fragment Metadata

Each `GLOBAL.md` declares its assembly identity in fragment-local frontmatter.
Frontmatter is assembler metadata: it is consumed by the tool and omitted from
assembled prose.

```markdown
---
id: doc-constitution
order: 100
source: /doc/README.md
status: draft
---

# Documentation

...
```

| Field    | Requirement                                                                                                                                                                                                                       |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`     | Stable lowercase kebab-case identifier (`[a-z0-9]+(-[a-z0-9]+)*`). Unique across the manifest. Use the module's local knowledge ID, matching the [shared ticket and anchor namespace](/doc/README.md#doc-constitution-namespace). |
| `order`  | Unique integer. Assembly is ascending by `order`, regardless of manifest order. Leave gaps (100, 200, ...) so new fragments can slot between neighbors without renumbering.                                                       |
| `source` | The fragment's canonical source README as a **bundle-root-relative** path: leading `/`, normalized (no `.` or `..` segments), no control characters, no `--`. Recorded in output provenance.                                      |
| `status` | `draft` or `stable`. Ambient context should not broadcast unsettled design; volatile knowledge faces a higher stability bar.                                                                                                      |

Validation failures are errors, not warnings: missing or invalid metadata,
duplicate `id`, and duplicate `order` all stop assembly with a thrown error.

### Link destinations

Fragments assemble into documents that live somewhere else, so
location-ambiguous links would silently retarget. The assembler therefore
**rejects** any link, image, or reference-definition destination that resolves
against the fragment's own directory (marked resolves reference-style links
before the check). Permitted destinations are location-independent:

- anchors (`#section`) and empty self-references;
- bundle-root-relative paths (`/doc/README.md`) — a single leading `/`;
- protocol URIs (`https://…`, `mailto:…`, `file:…`).

Destinations inside code spans and code blocks are not links and are ignored.
When a fragment needs to point at module material, use the bundle-root form,
not a relative hop.

<a id="agents-assembly-manifest"></a>

## Manifest Format

The manifest is JSON. It is the explicit declaration of inputs and output;
there is no auto-discovery of `GLOBAL.md` files.

```json
{
  "fragments": ["doc/GLOBAL.md", "modules/other/GLOBAL.md"],
  "output": "AGENTS.md"
}
```

- Paths inside the manifest resolve **relative to the manifest file's
  directory**, so a manifest is movable with its declaration intact.
- `fragments` must be a non-empty array of path strings. Each entry must be
  relative, normalized (no `.`, `..`, or redundant separators), and contained
  under the manifest's directory — absolute paths and traversal escapes are
  rejected. The same file may not be declared twice.
- `output` is a path string, also relative to the manifest directory.
- Duplicate fragment `id`s and `order`s across the declared set are rejected.

The default manifest path is `doc/agents.manifest.json`. It does not exist yet
(see [migration status](#agents-assembly-status)).

<a id="agents-assembly-command"></a>

## Command

The assembler is the `agents` command of the
[`rekon.ts` CLI](/rekon.ts), run directly from TypeScript source:

```sh
# Assemble to the manifest's declared output
node rekon.ts agents --manifest doc/agents.manifest.json

# Verify without writing; exits nonzero and reports the first differing byte when stale
node rekon.ts agents --manifest doc/agents.manifest.json --check

# Write to an alternate path (resolved against the working directory)
node rekon.ts agents --manifest doc/agents.manifest.json --output /tmp/AGENTS.preview.md
```

| Option           | Default                    | Meaning                                                                          |
| ---------------- | -------------------------- | -------------------------------------------------------------------------------- |
| `-m, --manifest` | `doc/agents.manifest.json` | Manifest path.                                                                   |
| `-o, --output`   | manifest's `output`        | Output override, resolved against the working directory.                         |
| `-c, --check`    | off                        | Compare assembled bytes to the output; fail nonzero on mismatch without writing. |

`--check` is the CI/verification mode: it detects both drifted fragments and
unassembled hand edits to the generated file. Normal mode writes atomically
(temporary file, then rename) and creates the output directory if needed.

The implementation is deliberately small and deep:
[`src/agents/assemble.ts`](/src/agents/assemble.ts) owns loading, validation,
ordering, and rendering; [`src/command/agents.ts`](/src/command/agents.ts) is
thin command wiring. Tests live beside the module in
[`src/agents/assemble.test.ts`](/src/agents/assemble.test.ts).

<a id="agents-assembly-output"></a>

## Generated Output

Assembled output is normalized so identical inputs produce identical bytes:
fragment bodies keep their source prose byte-for-byte (no render round-trip),
with CRLF normalized to LF and boundary whitespace trimmed; sections join with
exactly one blank line; the file ends with a single newline.

Output starts with a do-not-edit marker, and each fragment is preceded by a
provenance comment:

```markdown
<!-- GENERATED FILE - DO NOT EDIT
Assembled by `rekon agents` from declared GLOBAL.md fragments.
Edit the fragment sources and re-run `node rekon.ts agents`;
`node rekon.ts agents --check` detects hand edits to this file.
-->

<!-- rekon-fragment: id=doc-constitution order=100 path=doc/GLOBAL.md source=/doc/README.md -->

# Documentation

...
```

<a id="agents-assembly-context-cost"></a>

### Context Cost

Context is a budget, so the assembler reports real UTF-8 byte costs, never JS
string lengths (which undercount multibyte text). `assemble` exposes
`proseBytes` per fragment and `totalBytes` for the assembled document; the
command prints both per fragment and in total, making each fragment's ambient
price visible next to its identity.

### Provenance Safety

Provenance fields are interpolated into HTML comments, so identity values are
guarded: `id` is restricted to kebab-case, `source` and manifest fragment
paths reject `--`/`-->` sequences and control characters, and `order` is an
integer. A hostile or careless metadata value cannot break out of the
provenance comment or inject markup into the generated file.

<a id="agents-assembly-policy"></a>

## Generated-File Policy

- **Edit fragments, not output.** The generated file carries the do-not-edit
  marker; `--check` treats any hand edit as staleness.
- **No pruning.** The assembler writes only the declared output path. It never
  removes or reconciles undeclared artifacts; cleanup stays deliberate, in
  line with workspace conventions.
- **Provenance is mandatory.** Every assembled fragment records its `id`,
  `order`, fragment path, and canonical `source` README, keeping the ambient
  contribution traceable to its module. Field validation (see
  [provenance safety](#agents-assembly-context-cost)) keeps those comments
  well-formed.
- **Failures surface.** Missing files, invalid manifests, bad metadata,
  duplicate identity, structurally invalid Markdown, and fragment-relative
  link destinations all throw with the offending path in the message; nothing
  is silently skipped. `--check` compares UTF-8 buffers and reports the true
  first differing byte; only a missing output counts as missing — permission
  and I/O errors surface.

Markdown structure is validated with the [marked](https://github.com/markedjs/marked)
lexer: a fragment must be meaningful Markdown with exactly one top-level (`#`)
heading, in leading position, with content beyond that heading, and no
fragment-relative link destinations (see [link destinations](#agents-assembly-metadata)).
Frontmatter is parsed with
[gray-matter](https://github.com/jonschlinkert/gray-matter); command wiring
uses [gunshi](https://github.com/kazupon/gunshi).

<a id="agents-assembly-cross-references"></a>

## Cross-References

- [`/doc/README.md#doc-constitution-global-assembly`](/doc/README.md#doc-constitution-global-assembly)
  **specifies** the assembler's obligations (identity, order, provenance,
  duplicate rejection, generated markers); this document implements them.
- [`/doc/GLOBAL.md`](/doc/GLOBAL.md) **is** the first declared fragment and
  the worked example of fragment metadata.
- [`/AGENTS.md`](/AGENTS.md) **is** the future assembly target, still
  hand-maintained until its sections each gain a fragment source.
- Beads ticket `rekon-agents-maintenance-assembler` **tracks** this
  implementation; `rekon-doc-constitution-global` **tracks** the fragment.
