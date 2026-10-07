---
type: Practice
title: Sources and native references
description: Record the material behind our knowledge in OKF sources, including native session identifiers, and explain which claims it supports.
tags: [sources, provenance, references, okf, sessions]
status: draft
generated: { by: "rekon/gpt-6-astra", at: 2026-10-06T20:40:40-04:00 }
sources:
  - id: writing-guidance
    resource: /AGENTS.md
    title: Existing Writing guidance and OKF sources example
  - id: source-direction
    resource: "urn:opencode:session:ses_eec6a0bd5ffe45v03zJ7px5vgb"
    title: Human direction to make native session provenance a refs practice
  - id: okf-spec
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/86df01601733f93c0f243a727927d03ebf6d7e1d/okf/SPEC.md
    title: Open Knowledge Format v0.2, sections 5.1 and 6.2
  - { resource: /evidence/README.md, title: Evidence as a source's role in a judgment }
  - { resource: /CONTEXT.md, title: References, provenance, and native ownership }
---

<a id="sources"></a>
# Sources and native references

**Leave the next reader a way back to what informed the work.** Record consulted
material in [OKF](/okf/README.md) `sources:` and explain in the body what it
supports. A specification, inspected code, a conversation containing human
direction, and a measured result can all be sources; their roles differ.

This elaborates the existing [Writing guidance in AGENTS](/AGENTS.md#writing):
frontmatter is a trustworthy label, including where claims came from; the body
explains them. Native session references extend the `sources:` practice already
shown there.

## The small pattern

```yaml
sources:
  - id: skill-direction
    resource: urn:opencode:session:ses_eec6a0bd5ffe45v03zJ7px5vgb
    title: Human direction for archive-backed skills and a compact global index
```

This records the conversation that commissioned the skill process.[^source-direction]
Use the actual native session ID. A title says why the source matters; an `id`
provides a stable key for claim-level attribution. Neither depends on the
source's position in the list.

For a particular claim, OKF joins a Markdown footnote label to the corresponding
`sources[].id`:[^okf-spec]

```markdown
Our skill index points directly into archive checkouts.[^skill-direction]

[^skill-direction]: Human direction for archive-backed skill curation.
```

## Our OpenCode session convention

`urn:opencode:session:<native-session-id>` is our source-identifier convention.
It names the session in the system that owns it, independent of the project
directory or a later Markdown export. Session petnames such as `brave-mountain`
are useful labels; the native ID remains the lookup key.

This spelling is a **Rekon convention**, not a claim of a registered URN
namespace, an OpenCode-defined link format, or an installed URI resolver. OKF
allows source descriptors as well as followable artifacts; the native owner
supplies the lookup and access, not the string alone.

To follow it, use the exact session ID with the session tooling that can access
that OpenCode instance. If the reader needs an exported transcript, cite that
artifact too and retain the session identifier as its origin. When the exact
utterance matters, record its native message ID, timestamp, or a bounded quote
in the supporting account. A session is an evolving container, not a frozen
revision or evidence that every message agrees.

## Choose the reference for the job

| Source | Useful reference | Keep clear |
|---|---|---|
| Human direction in a session | Native session identifier and descriptive title | What was requested, decided, or still open; the session's identity is not its author's identity. |
| Upstream code or specification | Canonical repository URL with commit, path, and useful lines/section | Which revision grounds the claim. |
| Material available locally | `file:///…` to the real archive path | Machine-local access; pair with upstream identity in the supporting account. |
| Our maintained explanation | Root-relative document path and anchor | Current guidance versus a revision-bound historical finding. |
| An observed run or result | Its retained report, trace, or receipt | Conditions and outcome actually observed. |

Use the native owner's identifiers rather than inventing a new central identity
for each reference. Prefer an ordinary retrievable URL when the source already
has one. This session convention is one concrete pattern, not a new grammar for
every kind of source.

## Keep provenance honest

- List material actually consulted. Suggested reading belongs in a reading map
  until it has informed the work.
- Distinguish what a source says from our inference or recommendation. Describe
  the relationship in prose; a source list alone cannot do that work.
- Record `author`, `last_modified`, and other source facts only when known.
  A source's modification time differs from this document's generation time.
- Keep `generated`, `verified`, acceptance, and deployment distinct. Omit
  `verified` until a real check against named sources or the described resource
  supports it; never use placeholder verification events.
- Preserve source identifiers through summaries and promotion. Cite the
  synthesis for its argument and its sources for the underlying evidence.

The broader [evidence practice](/evidence/README.md) explains how a source bears
on a judgment. [OKF](/okf/README.md) owns the frontmatter and format account;
[refs](/refs/README.md) owns source selection and access.

[^source-direction]: The originating brave-mountain session, recorded in this document's sources.
[^okf-spec]: Open Knowledge Format v0.2, provenance and path-valued fields.
