---
type: Handoff
title: Finish Rust combine with standard Markdown and element-based directives
description: Implementation handoff after restoring ambient guidance; prioritize the actual combiner and explore standard HTML elements or anchors instead of bespoke comment markers.
resource: /design/toolchain/handoff0.gpt6a.md
status: draft
generated: { by: "model:openai/gpt-6-astra", at: 2026-10-11 }
sources:
  - { resource: "urn:opencode:session:ses_f908bad7fffeW0pr4FLFa4VFyn", title: Recovery work, correction toward combine, and the new element/anchor preference }
  - { resource: /design/toolchain/draft0.glm53m.md, title: Accepted Rust-first toolchain direction, especially the October 8 addendum }
  - { resource: /design/context-compilation/repair0.gpt6a.md, title: Landed recovery and coverage audit }
---

# Handoff prompt

**Later direction:** [enriched whole-document assembly](/design/toolchain/enriched-assembly0.gpt6a.md)
records the October 11 tidy-lagoon commission: whole-file composition, frontmatter
emitted as scoped MDX metadata, semantic definitions, and a final ToC pass. Read
that contract before treating the implementation/output instructions below as
settled. This handoff remains recovery and migration background.

You are continuing work in `/home/rektide/src/rekon`. **Finish the Rust
`rekon combine` command: the actual working combiner is the priority.** The
previous session restored missing operating guidance and built a coverage audit,
but over-weighted that safeguard instead of finishing the combiner. Do not
repeat that detour or spend another session only proposing the implementation.

## Latest human direction

The user requested this handoff with one change to the current approach:

> "i want to try to use the markdown + xml element more and less comments
> maybe for our directives. or just anchor tags directly. it's weird having
> our own system. i forget that md + html/xml thing what it's called"

The term may be **MDX** (Markdown plus JSX), but that is not confirmed. Ordinary
Markdown already supports embedded HTML, including `<a id="…"></a>` anchors.
Treat the substantive preference as **standard, legible Markdown/element/anchor
conventions over a home-grown comment directive language**, not as a mandate to
introduce a JSX runtime or an XML dialect. Briefly explain the distinction when
checking in. Show a small concrete syntax example before committing to a broad
format change; implementation of the settled combiner mechanics can proceed.

## Current working state

- Production assembly is **TypeScript/Effect**:
  `node tool/agents.ts`, with `--check`, `--manifest`, and `--output` options.
  It works and currently generates the live root `AGENTS.md`.
- [`agents.manifest.json`](/agents.manifest.json) selects **12 fragments**.
  [`AGENTS.old.md`](/AGENTS.old.md) is the preserved legacy manuscript.
  The earlier five-fragment cutover omitted beads and much other operational
  guidance. Recovery has landed; retain it during the producer migration.
- The Rust crate is [`tool/rust/`](/tool/rust/), binary `rekon`.
  `combine` and `toc` in [`main.rs`](/tool/rust/src/main.rs) are still stubs.
  Rust-first new code and `combine` before ToC were accepted in the
  [toolchain design's October 8 addendum](/design/toolchain/draft0.glm53m.md).
- Existing Rust [`manifest.rs`](/tool/rust/src/manifest.rs) and
  [`nodepath.rs`](/tool/rust/src/nodepath.rs) implement manifest parsing and
  lexical path planning. Reuse their verified behavior where appropriate.
- Rust `audit-context` now works. It checks a pinned legacy baseline, mapping
  of 13 actual H1 sections, selected fragments, and 108 required visible text
  excerpts. This is **declared literal coverage**, not semantic equivalence and
  not a combiner. Its [contract](/tool/assembly/coverage.md) and
  [ledger](/tool/assembly/coverage.json) explain its limits.
- `pnpm check` runs the production assembler's freshness check plus the Rust
  coverage audit. Keep them useful through the cutover; avoid creating another
  competing producer or misleading check script.

## Inspect first

1. [`tool/assembly/README.md`](/tool/assembly/README.md) — production behavior,
   options, errors, atomic writing, and known path/IO boundaries.
2. [`tool/assembly/assemble.ts`](/tool/assembly/assemble.ts) and its tests — load,
   validate, order, render, write/check. [`tool/cmd/agents.ts`](/tool/cmd/agents.ts)
   and tests define actual CLI behavior.
3. [`tool/md/fragment.ts`](/tool/md/fragment.ts),
   [`tool/md/document.ts`](/tool/md/document.ts), and their tests — metadata,
   Markdown structure, link portability, provenance rendering, byte comparison.
4. [`tool/rust/src/`](/tool/rust/src/) and
   [`tool/rust/tests/context_audit.rs`](/tool/rust/tests/context_audit.rs) — current
   Rust boundaries and a regression test that invokes the production TS assembler.
5. [Recovery assessment](/design/context-compilation/repair0.gpt6a.md) — what was
   lost, restored, refined, and left for editorial work.

Two **pre-existing uncommitted files** were left untouched by the recovery:
`praxis/ticket-pointers/ticket-pointers0.glm53fx.md` and
`tool/rust/src/markdown.rs`. Inspect the current working copy before editing.
The latter is an unfinished ~1,000-line handwritten scanner, not wired into
`lib.rs`; its existence is not evidence that it compiles or is suitable.
Investigate its provenance and utility before adopting, replacing, or deleting
it. Do not silently commit someone else's work.

## Implement the combiner and refine the markup

Build the real load → validate → order → render → publish/check path in Rust.
Preserve explicit selection, deterministic ordering, source provenance, input
protection, useful diagnostics, UTF-8 byte checks, and validation before writes.
Use maintained parsers instead of growing a custom Markdown scanner to match
incidental implementation details. `pulldown-cmark` is already a dependency;
inspect its facilities and any necessary frontmatter parser before deciding.

The current generated output uses a large generated-file HTML comment and
`<!-- rekon-fragment: id=… order=… path=… source=… -->` comments. **These comments
annotate provenance; selection comes from the JSON manifest and fragment
frontmatter.** Establish which annotations/directives genuinely need machine
reading before designing a replacement.

Try ordinary anchors plus readable source links first. Consider standard element
attributes only where they carry useful structured data. Prefer that to inventing
new `<rekon-…>` syntax just to rename the custom system. Keep source identity
recoverable and generated-file ownership obvious. An exact byte copy of the old
comment syntax is not the goal when the human has asked to change it.

Verify CommonMark/GFM behavior: raw HTML wrappers can stop enclosed Markdown
from rendering as expected; anchor-only nodes, blank lines, escaping, duplicate
IDs, and validator assumptions about the first heading all matter. Existing
validation requires the first meaningful token to be H1, so a leading anchor
needs deliberate treatment. Changes should preserve prose bytes where possible,
not round-trip all Markdown through a renderer. Do not assume arbitrary XML
elements are portable Markdown semantics, or that MDX is required for HTML.

## Proof, cutover, and completion

- Demonstrate real Rust write and `--check` on fixture bundles and the production
  manifest. Exercise invalid metadata/Markdown, duplicate IDs/orders, unsafe
  paths, omitted inputs, stale/missing output, output-overwrites-input, and IO
  errors. Checks must be read-only; failed validation must not publish output.
- Compare Rust with the established assembler's semantic behavior and fixture
  tests. Document intentional markup/header differences explicitly rather than
  calling incompatible bytes "parity" or weakening unrelated guarantees.
- Once Rust is ready, deliberately update production invocation, generated
  ownership instructions, package checks, and maintained tool docs together.
  Regenerate `AGENTS.md` through the chosen producer and show fresh output plus
  passing operating coverage. Keep the old producer's status unambiguous.
- The acceptance demonstration is **`rekon combine` regenerates the complete
  operating context and its check catches drift**, with the agreed simpler
  markup. A successful audit alone does not complete this task.

Last verified before handoff: 19 Rust tests, 78 TS assembly/Markdown/CLI tests,
clippy, tsgo, formatting, and production freshness/coverage passed. Commands:

```sh
cargo nextest run --manifest-path tool/rust/Cargo.toml
cargo clippy --manifest-path tool/rust/Cargo.toml --all-targets -- -D warnings
pnpm exec vitest run tool/assembly tool/md tool/cmd/agents.test.ts
pnpm exec tsgo --noEmit
pnpm check
```

Commit coherent stages with explicit owned paths. Follow current instructions;
do not push unless the human directs it. Existing work history is
`rekon-agents-maintenance`, with toolchain direction under `rekon-tools-endstate`;
inspect those before adding any tracking. No ticket-per-file explosion.
Check in at consequential choices and keep the actual combiner central.

Recovery commits: `492cb7ff` diagnosis, `00cc45a2` guidance restoration,
`3e99a5ad` Rust audit and checks, `56a19bab` maintenance-ticket note.
