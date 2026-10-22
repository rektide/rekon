---
type: Reference
title: Codes registry
description: Short codes mapped to document-and-fragment targets (file#tag strings); workspace reference vocabulary for conversation, headings, and tickets.
resource: /codes.md
tags: [codes, naming, aliases, registry]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-10-22 }
extensions:
  related_work: [rekon-con-naming]
sources:
  - { resource: "opencode://ses_e9bc39334ffetsgyIAj4L0kp0A", title: Commission the codes registry and declare cf for compfuzor }
  - { resource: /constitution/naming/README.md, title: Short codes in use — the prior maintained table }
---

# Codes registry

<a id="codes"></a>

Short codes are reference vocabulary, not directory names. Each entry maps a
code to its target as a `file#tag` string: the document that owns the code's
meaning, with the fragment when a section carries it. Scoped codes (`exp`,
`res`) have no fixed target document; they resolve to the
[practice](/constitution/naming/README.md#short-codes) that defines their
scoped use. Practice, rulings, and history live with
[naming](/constitution/naming/README.md#short-codes); this file is the data.

| Code | Target (`file#tag`) | Note |
|---|---|---|
| `g` | [`/constitution/structure.md#global`](/constitution/structure.md#global) | Human-provided shortcode for global guidance; delivery and ownership recorded there. |
| `p` | [`/praxis/README.md`](/praxis/README.md) | Human-provided shortcode. `p0` is the suggested opening “what we are / how we work” account, still being shaped. |
| `t` | [`/tool/README.md`](/tool/README.md) | Human-provided shortcode; the physical toolkit directory is `tool/`, with entrypoints such as `tool/agents.ts`. |
| `con` | [`/constitution/README.md`](/constitution/README.md) | Earlier cross-modal precedent: `rekon-con-naming` names the native Beads work. `con-structure` refers to the [structure contract](/constitution/structure.md). |
| `exp` | [`/constitution/naming/README.md#short-codes`](/constitution/naming/README.md#short-codes) | Scoped: `exploration` relative to the named topic, as in `con-naming-exp-…`. No fixed target document; implies no root `exp/` directory. |
| `res` | [`/constitution/naming/README.md#short-codes`](/constitution/naming/README.md#short-codes) | Scoped: `research` relative to the named topic; the containing topic supplies the concrete path. |
| `cf` | [`file:///home/rektide/src/compfuzor/README.md`](file:///home/rektide/src/compfuzor/README.md) | The compfuzor project (`rektide/compfuzor`, canonical [`README.md`](https://github.com/rektide/compfuzor/blob/main/README.md)). Not scholarly “cf.” (compare) and not Cloudflare. |
