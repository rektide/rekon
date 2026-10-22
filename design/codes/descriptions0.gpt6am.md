---
type: Reference
title: Codes glossary description pitches
description: Seven concise glossary notes emphasizing code meanings, usage cues, and likely confusions.
resource: /design/codes/descriptions0.gpt6am.md
status: draft
generated: { by: "model:openai/gpt-6-astra#medium", at: 2026-10-22 }
tags: [codes, glossary, naming]
sources:
  - { resource: "opencode://ses_e9bc39334ffetsgyIAj4L0kp0A", title: Commissioned independent pitch of codes glossary descriptions }
  - { resource: "opencode://ses_e9b4539e1ffeFQd1jPJsHlZRzY", title: Drafted independent meaning-first descriptions against the current registry and naming practice }
  - { resource: /codes/GLOSSARY.md, title: Current generated descriptions and code targets }
  - { resource: /constitution/naming/README.md#short-codes, title: Reference vocabulary and scoped code usage }
  - { resource: /CONTEXT.md, title: Plain definitions and targeted disambiguation precedent }
---

# Codes glossary description pitches

These candidate notes give meaning and usage alongside the existing target column.
The [codes glossary](/codes/GLOSSARY.md) supplies the current wording; the
[short-codes practice](/constitution/naming/README.md#short-codes) grounds their
use in conversation, headings, and tickets, while the
[working vocabulary](/CONTEXT.md) supplies the definition-and-disambiguation tone.

## `cf`

```text
Compfuzor, when referring to the project or its work; not Cloudflare or scholarly “cf.” (compare).
```

Current: The compfuzor project (`rektide/compfuzor`). Not scholarly “cf.” (compare) and not Cloudflare.

Rationale: Keeps both real ambiguities visible and makes the project-reference use explicit without repeating repository coordinates.

## `con`

```text
Constitution: shared principles for naming, knowledge, and work; also a topic prefix in `rekon-con-naming` and `con-structure`.
```

Current: Earlier cross-modal precedent: `rekon-con-naming` names the native Beads work; `con-structure` refers to the structure contract.

Rationale: Defines the concern before showing its established use across ticket and document references.

## `exp`

```text
Exploration within the named topic, as in `con-naming-exp-…`; the topic supplies the scope, not a directory named `exp`.
```

Current: Scoped: `exploration` relative to the named topic, as in `con-naming-exp-…`; no fixed target document and no root `exp/` directory.

Rationale: Retains the concrete example while explaining how to interpret it without turning the code into a path.

## `g`

```text
Global guidance: the shared ambient context supplied before task-specific reading; use for guidance meant to travel across topics.
```

Current: Human-provided shortcode for global guidance; delivery and ownership recorded at the target.

Rationale: Replaces provenance and destination commentary with what ambient guidance does and when this code is useful.

## `p`

```text
Praxis: how we work and learn from doing; `p0` names the proposed opening “what we are / how we work” account, still being shaped.
```

Current: Human-provided shortcode. `p0` is the suggested opening “what we are / how we work” account, still being shaped.

Rationale: Gives the base code a meaning while preserving the specific, provisional meaning of `p0`.

## `res`

```text
Research within the named topic; use for that topic’s research material, with scope supplied by context rather than a fixed directory.
```

Current: Scoped: `research` relative to the named topic; the containing topic supplies the concrete path.

Rationale: Makes the reference use explicit and distinguishes contextual scope from a globally fixed research location.

## `t`

```text
Shared Rust-first toolkit: use for the `rekon` CLI and its document assembly, glossary generation, checking, and inspection capabilities.
```

Current: Human-provided shortcode; the physical toolkit directory is `tool/`, with entrypoints such as `tool/agents.ts`.

Rationale: Replaces directory and legacy entrypoint details with the toolkit’s current identity and practical jobs.
