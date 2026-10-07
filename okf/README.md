---
type: Reference
title: Open Knowledge Format in Rekon
description: The format behind our self-describing knowledge documents, with honest provenance, verification, lifecycle, and links to the upstream specification.
tags: [okf, knowledge, frontmatter, provenance, verification]
status: draft
generated: { by: "rekon/gpt-6-astra", at: 2026-10-06T20:40:40-04:00 }
sources:
  - id: writing-guidance
    resource: /AGENTS.md
    title: Existing OKF Writing guidance, full frontmatter example, and index/log conventions
  - id: okf-spec
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/86df01601733f93c0f243a727927d03ebf6d7e1d/okf/SPEC.md
    title: Open Knowledge Format v0.2
  - { resource: /constitution/README.md, title: Knowledge modules and trustworthy frontmatter }
  - { resource: /constitution/reassessment0.gpt6a.md, title: Acceptance and verification are different claims }
  - { resource: "urn:opencode:session:ses_eec6a0bd5ffe45v03zJ7px5vgb", title: Human direction for an OKF home and source-citation practice }
---

<a id="okf"></a>
# Open Knowledge Format

**Make knowledge explain its own grounds.** OKF supplies a small, readable
format for that practice: Markdown bodies with YAML frontmatter, organized in
directories. The label tells a reader what the document describes, what informed
it, who produced or checked it, and when its claims need another look. The body
does the explaining.

This home formalizes the existing
[Writing guidance in the retired monolith](/AGENTS.old.md#writing) and the
[constitutional frontmatter contract](/constitution/README.md#doc-constitution-module-readme).
That preserved manual supplies the full “Active customers” example, including
`sources:`, and the `index.md` / `log.md` conventions. Here we explain
those commitments and connect their supporting practices.

The authoritative format definition is
[GoogleCloudPlatform/knowledge-catalog — OKF specification](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md),
also available in our
[archive](file:///home/rektide/archive/GoogleCloudPlatform/knowledge-catalog/okf/SPEC.md).
The account below is grounded in v0.2 at the revision in `sources`.

<a id="frontmatter"></a>
## A trustworthy label

```yaml
---
type: Practice
title: Curating skills
description: Select and compose skills while retaining their source ownership.
tags: [skills, references]
status: draft
generated: { by: "rekon/gpt-6-astra", at: 2026-10-06T20:40:40-04:00 }
sources:
  - id: direction
    resource: urn:opencode:session:ses_eec6a0bd5ffe45v03zJ7px5vgb
    title: Human direction for archive-backed skill curation
---
```

For ordinary concepts, `type` is the only always-required field. Use optional
fields when they convey real information; specialized concept types can impose
additional requirements.[^okf-spec]

| Field | What it says |
|---|---|
| `type`, `title`, `description` | Kind of knowledge, display name, and concise orientation. Types are open-ended. |
| `resource` | Canonical URI of an underlying asset the concept describes, when there is one. It is not a substitute for the concept's file identity. |
| `sources` | Material the content derives from. Every entry needs `resource`; stable `id` keys support per-claim footnotes. See [our source practice](/refs/sources.md). |
| `generated` | Who produced the current content and when it meaningfully changed. |
| `verified` | Actual checks against sources or the described resource, each with `by` and `at`. Omit it when unverified. |
| `status` | `draft`, `stable`, or `deprecated`; upstream defaults an absent value to `stable`, so label drafts explicitly. |
| `stale_after` | The instant on or after which review is due. |

Timestamp fields use ISO 8601 datetimes with an explicit offset, including
`generated.at`, `verified.at`, `last_modified`, and `stale_after`. Actor spellings
are `<producer>/<version>` for agents/tools, `human:<id>` for people, and
`process:<id>` for automated processes. Write a verification event only for a
check that actually happened; human acceptance of a proposal does not itself
verify its factual claims.

One normalization from the existing AGENTS example: its date-only
`stale_after` and `last_modified` should use full offset datetimes when producing
v0.2-conformant metadata. Date-only headings remain correct in `log.md`.

<a id="structure"></a>
## Bodies, links, and bundles

- A concept is a Markdown document. Within a declared bundle, its Concept ID is
  the bundle-relative file path with `.md` removed.
- Explain relationships with ordinary Markdown links and surrounding prose.
  Root-relative links are recommended within the bundle.
- `index.md` is reserved for directory listings and progressive disclosure;
  `log.md` is reserved for dated update history. Neither is a concept document.
- Producer extensions are allowed. Preserve unknown fields when moving material
  between tools; name local conventions honestly.

Rekon uses OKF-informed knowledge labels. Full bundle conformance is a separate
claim: our `GLOBAL.md` fragments, for example, obey the
[assembler's metadata contract](/constitution/agents-assembly.md#agents-assembly-metadata)
and are not automatically OKF concept documents. That metadata is stripped
during assembly; it serves a different interface.

<a id="practice"></a>
## Where the practice lives

- [Sources and native references](/refs/sources.md) — how we cite sessions,
  specifications, code, and observed results, including the OpenCode URN convention.
- [Evidence](/evidence/README.md) — what a source supports, challenges, or leaves
  uncertain. A format field is a label, not the evidentiary argument.
- [Constitution](/constitution/README.md#doc-constitution-module-readme) — shared
  knowledge ownership and the document contract.
- [Earlier reassessment](/constitution/reassessment0.gpt6a.md) — why acceptance,
  verification, and deployment must stay distinct when our knowledge evolves.

[^okf-spec]: Open Knowledge Format v0.2, particularly sections 2–9 and 11; see the pinned source above.
