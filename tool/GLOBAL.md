---
id: tool
order: 450
source: /tool/README.md
status: draft
type: ContextFragment
title: Tools — check knowledge documents
description: Quick rkt recipes for Markdown checking and generated-output freshness, plus the checker's profiles, link resolution, and freshness-vs-coverage distinctions.
resource: /tool/GLOBAL.md
generated: { by: "model:openai/gpt-6-astra#high", at: 2026-10-22 }
sources:
  - { resource: "opencode://ses_e9d711bbeffenO3yHUyxP2Bv1G", title: "Human requests Rust integration of the consumer-document checker; parent completes parser corrections, documentation and generated guidance after G reaches its usage limit" }
  - { resource: "opencode://ses_e9d67cd87ffe5fumUOB6NXo3Zn", title: "G develops the initial Rust check engine and CLI integration" }
  - { resource: /tool/check/README.md, title: "Document checker commands, profiles and limits" }
  - { resource: "opencode://ses_e9bc39334ffetsgyIAj4L0kp0A", title: "Human direction to document the toolkit's offerings with quick usage examples under the rkt code" }
---

# Tools — check knowledge documents

**`rkt` — quick recipes from any checkout.** Verify Markdown you just wrote,
or check generated output, in seconds:

```sh
R="cargo run --quiet --manifest-path ~/src/rekon/tool/rust/Cargo.toml"
$R check FILE...                      # links, anchors, fences, whitespace
$R check --okf DOC.md                 # + frontmatter and session provenance
$R combine --check                    # AGENTS.md fresh?
$R codes --check                      # codes glossary fresh?
$R toc FILE --check                   # nav region fresh?
$R audit-context                      # declared coverage of the bundle
```

Checks are read-only; `--check` never writes. Use `rkt` for Markdown
maintenance in any repo, not just rekon's.

- After writing or revising Markdown, check the explicit owned files with
  `check FILE...` (from any checkout via the `$R` alias above). It verifies
  local link/image targets, heading/explicit-ID anchors, fenced-code closure
  and trailing whitespace outside code, without editing files.
- Add `--okf` for knowledge documents: require the selected frontmatter fields,
  a native OpenCode session source and resolvable local `sources[].resource` paths.
  Ordinary Markdown does not need this profile. The command recognizes session
  authorities; the author must still supply the actual contributing session.
- `/...` links resolve at the nearest `.jj`/`.git` ancestor of each checked file;
  use `--root DIR` to select another bundle root. `file:///...` links retain their
  absolute checkout meaning. Remote URLs and semantic claims are not verified.
- For generated `AGENTS.md`, edit its source fragments and run `combine`, then
  `combine --check`; document checking does not establish assembly freshness.
  Use [the checker guide](/tool/check/README.md) for parser/profile limits and
  [the toolkit map](/tool/README.md) for ToC, scoped OKF extraction, the codes
  glossary, and coverage tools.
