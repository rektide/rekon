---
type: Reference
title: Codes glossary description attempts
description: Three sizing passes (small, medium, large) at each codes glossary note, written independently of the commissioned astra pitch.
resource: /design/codes/descriptions0.glm53m.md
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-10-22 }
tags: [codes, glossary, naming]
sources:
  - { resource: "opencode://ses_e9bc39334ffetsgyIAj4L0kp0A", title: Independent three-size description passes beside the commissioned astra pitch }
  - { resource: /codes/GLOSSARY.md, title: Current generated glossary and its descriptions }
  - { resource: /constitution/naming/README.md, title: Short codes practice }
---

# Codes glossary description attempts

Three passes at each `description` — the glossary Note column. Written
independently of the astra pitch (`descriptions0.gpt6am.md`); compare, don't
merge blindly.

## What the Note column must do

The table already shows the code and its `file#tag` target, so the note never
restates *where*. It answers, for a reader who just met the code in a
conversation, heading, or ticket: what does it expand to, whose concern is it,
and what would I wrongly take it for? Sizing discipline:

- **Small** (≤ ~70 chars): expansion plus one decisive qualifier.
- **Medium** (≤ ~140): adds scope, ownership, or a usage trigger.
- **Large** (≤ ~220): adds a conflation guard or a decoded example; still one
  pipe-free table cell.

## Attempts

### `cf` — compfuzor (external target)

The only code whose target leaves this bundle, and the only one whose
disambiguation is load-bearing: agents meeting `cf` with Cloudflare skills
loaded will guess Cloudflare; scholars guess "compare". The note's first job
is to close both doors.

- Small: `Compfuzor (rektide/compfuzor) — never Cloudflare or “cf.” here`
- Medium: `The compfuzor project (~/src/compfuzor); in this workspace cf always means it, not Cloudflare or scholarly “cf.” (compare)`
- Large: `The compfuzor project (rektide/compfuzor, at ~/src/compfuzor). Workspace default: cf names this project, never Cloudflare or scholarly “cf.”; qualify explicitly when you do mean either`

### `con` — constitution, and prefix composition

The load-bearing fact is not "constitution" but that codes *compose*: `con-`
prefixes scoped work across modalities — tickets, doc anchors, discussion
threads. That precedent is why hyphenated references like `rekon-con-naming`
resolve without a resolver.

- Small: `Constitution; as a prefix it scopes work (con-naming, con-structure)`
- Medium: `The constitution, and a composing prefix: rekon-con-naming is the Beads naming epic, con-structure the structure contract`
- Large: `The constitution. As a prefix it composes scoped references across modalities — rekon-con-naming names the Beads naming epic, con-structure the structure contract — the recorded precedent for hyphenated cross-references`

### `exp` — scoped exploration

A suffix modifier, never a location. The guard matters: nothing implies a root
`exp/` directory; the named topic owns the path.

- Small: `exploration — a scoped suffix, not a directory`
- Medium: `Scoped suffix: exploration relative to the named topic (con-naming-exp-…); implies no root exp/ directory`
- Large: `A scoped suffix, not a location: exploration relative to the named topic, as in con-naming-exp-…. The containing topic supplies the concrete path; there is no root exploration/ directory`

### `g` — global guidance

Names a concern — the shared ambient layer every session loads — not a
directory. Worth keeping the glossary's own distinction nearby: a global
contribution is a domain's proposal, not global authority.

- Small: `Global guidance: the shared ambient layer every session loads`
- Medium: `Global guidance — the shared ambient layer assembled into AGENTS.md; delivery and ownership recorded in the structure contract`
- Large: `Global guidance: the shared ambient layer every session loads. Names the concern, not a directory — domains propose fragments, the root manifest selects them, and a contribution is a proposal, not global authority`

### `p` — praxis

The repeatable-practices domain; `p0` is its still-shaping opening account.
The twin fact with `g`/`t`: human-provided shortcode trio.

- Small: `Praxis: the repeatable-practices domain (bookmarking, beads, fronts)`
- Medium: `Praxis — repeatable ways of working (bookmarking, beads, fronts, ticket pointers); p0 is the shaping “what we are / how we work” account`
- Large: `Praxis: the practice domain — repeatable ways of joining purpose, procedure, and evidence (bookmarking, beads, fronts, ticket pointers). p0 names the still-being-shaped opening “what we are / how we work” account`

### `res` — scoped research

`exp`'s twin; same guards, sparser attestation. Keep them symmetric so the
pair reads as one rule with two words.

- Small: `research — a scoped suffix, not a directory`
- Medium: `Scoped suffix: research relative to the named topic; the topic supplies the concrete path`
- Large: `A scoped suffix like exp: research relative to the named topic. The containing topic supplies the concrete path; it never names a fixed directory of its own`

### `t` — tool

The shared toolkit: physically `tool/`, Rust-first `rekon` CLI with legacy
Gunshi beside it. The shortcode names the concern; the directory is `tool/`,
never `t/`.

- Small: `Tool: the shared Rust-first toolkit under tool/`
- Medium: `The shared toolkit (tool/, Rust-first): combine, codes, check, toc, okf, audit-context — entrypoints at the tree top`
- Large: `The shared toolkit: physical home tool/, Rust-first rekon CLI (combine, codes, check, toc, okf extract, audit-context) with legacy Gunshi commands retained. The shortcode names the toolkit; the directory is tool/, never t/`

## Choosing

Small suits the ambient table if token cost dominates; medium is the likely
default — it carries scope without spilling; large earns its bytes only where
a conflation is real (`cf`, `g`, `con`). A mixed adoption (medium everywhere,
large for `cf`) is defensible; per-item human selection is the acceptance step.
