---
type: ResearchReport
title: Hyalo ToC lineage recovered
description: cotail --since 30d recovery of the actual hyalo line-numbered ToC work (~/ado bin/hyalo-toc.ts), its session lineage, current hyalo 0.24.1 ToC surface, and relevance to Rekon's new tool/ toolkit home.
resource: /design/rekon/hyalo-toc0.glm53m.md
tags: [rekon, hyalo, toc, cotail, ado, t-toolkit]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-10-06 }

sources:
  - { resource: "urn:opencode:session:ses_f160b81daffeQqQLu97GUaN9IR", title: ToC request and build session (2026-09-29), author: "session:stellar-planet" }
  - { resource: "urn:opencode:session:ses_f064908a", title: Later regeneration use (2026-10-01), author: "session:witty-rocket" }
  - { resource: "file:///home/rektide/ado/bin/hyalo-toc.ts", title: The recovered implementation, author: "session:stellar-planet" }
  - { resource: "hyalo 0.24.1 installed binary --help (madr/okf/find/read)", title: Current capability surface, author: "host:rektide" }
  - { resource: /assess/rekon/tooling0.glm53.md, title: Prior hyalo capability map, author: "model:glm-5.3-max" }
---

# Hyalo ToC Lineage Recovered

<a id="hyalo-toc0-scope"></a>

## Search scope and method

Ran the real `cotail` CLI (v0.1.0, profile `opencode-local` →
`~/.local/share/opencode/opencode-local.db`): `cotail search hyalo --since 30d`
(top-level and `--subagent`), plus queries for `toc` and `table of contents`
(`--title-only` and content). The `--since 30d` filter **is available** and was
used as-is; the queries ran 2026-10-07 00:30–01:00 **UTC** = 2026-10-06
20:30–21:00 **EDT** host-local (receipt: this session, `ses_eec3b15f…`, started
2026-10-07 00:30:34 UTC per the DB; DB timestamps render UTC), so the 30d
content cutoff is ≈ **2026-09-07 00:30 UTC / 2026-09-06 20:30 EDT**. (Without
`--since`, cotail bounds content search to the last 31d anyway — the explicit
30d is slightly tighter; no broadening was needed or done.)
Message-level detail was read via read-only SQLite against the same DB
(`mode=ro`), because cotail's `tail` cannot yet filter to one arbitrary session
ID — a documented CLI limit, worked around without widening time bounds.

**Everything in the lineage (2026-09-28/29 request+build, 2026-10-01 reuse) is
inside the window.** The one older lead,
[`/assess/rekon/tooling0.glm53.md`](/assess/rekon/tooling0.glm53.md), was
**not among the query's results** — found by repo inventory and inspected
separately. Its exact jj commit time (2026-09-06 22:36:58 UTC / 18:36:58 EDT,
commit `953258d2`) independently proves it ~2h outside the 30d cutoff.

<a id="hyalo-toc0-request"></a>

## The request, verbatim

Session `ses_f160b81daffeQqQLu97GUaN9IR` ("Parallel repo updates for
opencode-args", dir `~/src/opencode-working`, glm-5.3#max, seq 1114,
2026-09-29 18:18):

> can you figure out how to use hyalo to build a ToC? i think it supports that.
> ideally i'd love some kind of line number hidden in the ToC to help llm's
> figure out what to read

<a id="hyalo-toc0-done"></a>

## What was done — intended vs actual

The intent was fully met and exceeded, **in `~/ado`** (not in
`opencode-working`; the session cd'd there to commit):

1. Found `hyalo` installed at `/usr/local/bin/hyalo` (session seq 1126).
2. Mapped the native surface: `okf index` regenerates file-level ToCs from
   frontmatter into managed markers (no line numbers, by design);
   `find --file X --fields sections` returns a heading map that **already
   carries `line` per heading, always computed live**; `read --section` /
   `read --lines` are the consumers (seq 1141–1163).
3. Built `bin/hyalo-toc` (shell prototype) — a generator using hyalo as data
   source, emitting entries like `- On deck <!-- L663-2013 -->`: HTML comments
   hide ranges from rendered view while LLMs can jump *and budget* reads. Hit
   the self-referential bug (the ToC's own insertion shifts the headings it
   indexes → stale on arrival, caught by its own `--check`) and fixed it with
   **fixed-point regeneration** (≤3 passes). Commit **`2a5a593acc95`**
   "ado: line-numbered ToCs via hyalo section data" (`patches.md`,
   `patches-log.md`, `plugins.md`, `bin/hyalo-toc`).
4. Rewrote as TypeScript per house JS conventions and retired the shell
   version; added usage mentions in `patches.md` § *See also* and `plugins.md`
   § *Cross-references`; regenerated all ToCs. Commit **`6c48b3f35aa0`**
   "ado: hyalo-toc in TypeScript; mention in patches/plugins docs".
5. **Living discipline since:** session `ses_f064908a` (2026-10-01,
   opencode-selfuse) ran `node bin/hyalo-toc.ts patches.md plugins.md && …
   --check && jj commit` as a routine part of doc edits.

**Verified current state (2026-10-07):** all three files exist in `~/ado` with
live `<!-- hyalo-toc:begin/end -->` regions; `./bin/hyalo-toc.ts --check
patches.md patches-log.md plugins.md` exits **0** (drift-free); ranges have
grown since creation (`On deck` now `L663-2427` vs `L663-2013` at birth),
proving regeneration kept pace; ground truth re-checked live —
`hyalo find --file patches.md --fields sections` reports `line` per heading,
and `hyalo read --file patches.md --lines 663:663` returns exactly
`## On deck`.

<a id="hyalo-toc0-impl"></a>

## The implementation

[`~/ado/bin/hyalo-toc.ts`](file:///home/rektide/ado/bin/hyalo-toc.ts) — 121
lines, node-ran `.ts` (shebang, executable, type-stripping), async
`node:child_process`/`node:fs/promises`, let-it-fail posture,
`realpath(process.argv[1]).then(...)` main-module convention. Semantics:

- Data: `hyalo find --file <f> --fields sections,lines --format json` → level
  ≤2 headings, each `{heading, level, line}`; entry range = heading line →
  next listed heading −1 (else EOF `lines`).
- Emit: `- <heading> <!-- LSTART-END -->` inside
  `<!-- hyalo-toc:begin … -->` / `<!-- hyalo-toc:end -->`; first run inserts
  after the H1; region-only splice leaves prose untouched.
- Fixed point: regenerate from post-write state until stable (block size is
  constant per pass, converges by pass 2, capped at 3).
- `--check`: exit 1 on drift — same discipline as `okf index`'s
  `changed == 0` CI gate.
- Line numbers are body-relative for frontmatter files, matching
  `hyalo read --lines`.

No `~/ado`-specific coupling beyond the `hyalo` binary on PATH — the script is
directly portable.

<a id="hyalo-toc0-surface"></a>

## hyalo 0.24.1 ToC-relevant surface (source-verified via installed `--help`)

| Capability | What | Line numbers |
|---|---|---|
| `hyalo okf index` / `okf log` | regenerate `index.md` (file-level ToC from frontmatter, `<!-- okf:index:begin/end -->`) and dated `log.md`; dry-run default, CI gate on `.results.changed == 0` | no — file-level by design |
| `hyalo madr toc` | ADR ToC/status dashboard into `<adr-dir>/README.md` inside `<!-- madr:toc:begin/end -->`; `--dry-run` default, non-zero exit on drift | no — ADR number/title/status/date only |
| `hyalo find --file X --fields sections,lines` | heading map `{heading, level, line}` + total lines; always live, never a cache | **yes** |
| `hyalo read --section H` / `--lines A:B` | consumers; bodies >8 KiB get budgeting hints pointing at the two-step `find --fields sections` → `read --section` pattern | — |

**The gap `bin/hyalo-toc.ts` fills:** hyalo has no native generator that embeds
heading line numbers into a ToC. The ado script is the glue between `find`'s
live line data and `read`'s ranged consumption — exactly the "line number
hidden in the ToC" the user asked for, as ranges so an LLM can budget before
spending tokens.

<a id="hyalo-toc0-rekon"></a>

## Hyalo material in Rekon today

- [`/assess/rekon/tooling0.glm53.md`](/assess/rekon/tooling0.glm53.md)
  (committed 2026-09-06 22:36:58 UTC, `953258d2` — not a query result,
  recovered by repo inventory; provably ~2h outside the query's 30d cutoff):
  thorough capability map from the then-archive checkout (v0.20.0-13). Now
  stale on two points: it says hyalo "not installed" (0.24.1 is installed,
  build stamp 2026-09-23), and it predates both `madr toc` prominence and the
  whole ToC lineage. Its boundary contract (hyalo = deterministic Markdown/OKF
  lifecycle; summarizer = semantic production) still holds and is confirmed by
  this lineage.
- [`/assess/rekon/README.md`](/assess/rekon/README.md) hyalo row repeats the
  "was not installed here" claim — same staleness.
- [`/t/README.md`](/t/README.md) (2026-10-06, written into the then-`t/` home;
  the layout ruling below supersedes its directory scheme): already
  anticipates this report — "The current Hyalo investigation is recovering the
  actual ToC implementation and its native owner before choosing an
  integration. A `hyalo/` source seam should follow a real consumer and API,
  rather than name an empty future subsystem."
- [`/constitution/structure.md`](/constitution/structure.md) names "recover
  the Hyalo ToC lineage" as immediate work (done here) and blesses a
  tool-local `hyalo/` seam as a domain directory.

<a id="hyalo-toc0-next"></a>

## Relevance to `tool/` and next integration possibilities (parent's call)

> **Correction (2026-10-06, human ruling):** this report originally treated
> `t` as the tool *directory* — that guidance was wrong. **`t` is a short
> code** (g = global, p = praxis, t = tool), not a layout. The selected
> physical layout is one `tool/` toolkit: entrypoints such as
> `tool/agents.ts` at its root, with shared domain dirs `tool/cmd/`,
> `tool/md/`, `tool/hyalo/` as the work justifies. Actionable references
> below are corrected to that layout; observations of the superseded `t/`-era
> docs remain as history. Shortcode lineage:
> [`/design/rekon/shortcodes0.glm53m.md`](/design/rekon/shortcodes0.glm53m.md).

The recovered consumer+API is now concrete: `find --fields sections,lines` →
splice into `hyalo-toc` markers → gate with `--check`. Options, not decisions:

1. **Adopt into the `tool/` toolkit** (entrypoint `tool/hyalo-toc.ts` at the
   toolkit root, shared domain dirs per the layout above): the script ports
   nearly verbatim; its only dependency is the `hyalo` binary (Rust, installed
   0.24.1) — a new kind of external dependency for Rekon's node-only tool
   stack, worth stating explicitly in the tool's README.
2. **Point it at Rekon's own corpus first** (constitution/, assess/, prompt/
   long-form docs) as the real consumer, mirroring how ado dogfoods it on its
   three big docs — the same "self documents structure" motive that launched
   the tool home.
3. **Generalize deliberately or not at all**: ado's version takes explicit
   file args; multi-vault/glob/config support is speculative until a second
   consumer exists (the t/README's own "no empty future subsystem" rule).
4. **Upstream, longer shot**: the fixed-point + hidden-range ToC is a
  candidate `hyalo toc` subcommand upstream (`madr toc` proves the pattern:
  managed region, dry-run default, drift exit). Not started; ractive/hyalo
  archive checkout is at v0.20.0-13, behind the installed 0.24.1.

No overlap taken with the other G's `ado`/recovery comparison
([`ado-recovery0.glm53m.md`](/design/rekon/ado-recovery0.glm53m.md)), the
parent's naming/structure work (t/README, structure.md already landed), or
Sol's Effect CLI assembler
([`agents-effect0.sol61xh.md`](/design/rekon/agents-effect0.sol61xh.md)).

<a id="hyalo-toc0-uncertain"></a>

## Uncertain or unverified claims

- **hyalo install provenance**: `/usr/local/bin/hyalo --version` reports
  `0.24.1 (9d105bc10ade 2026-09-23)`; install date is inferred from that build
  stamp plus tooling0's "not found" on 2026-09-06 — no install log checked.
- The build session mentioned "a reference in `~/src/doc/src.md`"; that file
  no longer greps for hyalo — unverified what it said.
- `hyalo madr toc` was verified against the installed 0.24.1 `--help` only;
  not run against a real ADR directory here.
- Whether Rekon's own docs should carry embedded ToCs (vs the never-stale
  two-step `find`→`read --section` query path) is a judgment call the parent
  owns; ado's experience: embedded wins on cost, at the price of regeneration
  discipline (`--check` gate).
- cotail ID truncation (displays 8 hex chars) required prefix-matching in
  direct DB reads — full IDs cited above are exact.

<a id="hyalo-toc0-xref"></a>

## Cross-references

- [`/assess/rekon/tooling0.glm53.md`](/assess/rekon/tooling0.glm53.md) — prior
  hyalo capability map; this report supersedes its install status and adds the
  ToC lineage it predates.
- [`/t/README.md`](/t/README.md) — the tool-home README as it stood in the
  `t/`-directory era (superseded by the `tool/` ruling; parent owns the
  rewrite); it awaited exactly this recovered consumer/API before opening a
  `hyalo/` seam.
- [`/constitution/structure.md`](/constitution/structure.md) — named this
  recovery as immediate work; its `t/<tool>/` layout rows predate the human's
  `tool/` ruling and are the parent's to update.
- [`/design/rekon/ado-recovery0.glm53m.md`](/design/rekon/ado-recovery0.glm53m.md)
  — sibling investigation of `~/ado` itself; the ToC artifacts live there and
  its README/docs conventions govern how the script is referenced.
- External: [`~/ado/bin/hyalo-toc.ts`](file:///home/rektide/ado/bin/hyalo-toc.ts),
  `~/ado/patches.md` (§ *See also* mention), `~/ado/plugins.md`
  (§ *Cross-references* mention), and the
  [summarizer/hyalo juncture](file:///home/rektide/src/summarizer/doc/discovery/hyalo-juncture.md)
  boundary contract.
