---
type: Practice
title: Sources and native references
description: Record the material behind our knowledge in OKF sources, including native session identifiers, and explain which claims it supports.
tags: [sources, provenance, references, okf, sessions]
status: draft
generated: { by: "rekon/gpt-6-astra", at: 2026-10-11T18:11:00-04:00 }
sources:
  - resource: "opencode://session/ses_ed2db3069ffeQV7v6JC7p69Vte"
    title: Adopt canonical OpenCode URI citations, source qualification, and legacy compatibility in Rekon guidance
  - resource: "opencode://session/ses_ed2eda746ffefem0MPL9M5MYX7"
    title: Human direction in opencoattails to adopt OpenCode URI references and align Rekon provenance guidance
  - resource: "urn:opencode:session:ses_ed2fb6875ffe2HmO6fhnDZensz"
    title: Human direction in lucky-eagle to emphasize current-session provenance as vital to OKF writing
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

This elaborates the existing [Writing guidance in AGENTS](/AGENTS.md#durable-writing):
frontmatter is a trustworthy label, including where claims came from; the body
explains them. Native session references extend the `sources:` practice already
shown there.

<a id="session-provenance"></a>
## Always record the writing session

**When creating or substantively revising a knowledge document, include your
current native session ID in its OKF `sources:`.** This is a required Rekon
writing practice: the session preserves the human direction, reasoning, and
choices that a finished document often compresses away. Without that reference,
the next reader can lose the way back to why the work took its shape.

- Use `resource: "opencode://session/<actual-session-id>"`, replacing the
  placeholder with the current ID supplied by the harness. Never copy an
  example's ID as your own. If the ID is unavailable, recover it from session
  tooling or ask; do not invent one.
- Give the source a title describing what this session contributed. A petname
  can help recognition but does not replace the native ID.
- On revision, retain earlier session sources and add the current session.
  When another session supplies direction or evidence, cite that session too
  with its own role.
- Each session maintains its own source entry as its work develops. Gradually
  update the title to reflect the contribution actually made: an initial request
  may grow into investigation, decisions, implementation, or review. The title
  need not be rewritten at every turn; bring it into alignment at meaningful
  updates. Keep the native session ID stable and update the existing entry
  rather than adding a duplicate for each change in the session's role.
- Check the session source before saving the document. `generated.by` identifies
  the producer; it does not supply the missing conversation provenance.

## The small pattern

```yaml
sources:
  - id: skill-direction
    resource: opencode://session/ses_eec6a0bd5ffe45v03zJ7px5vgb
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

Use the canonical long form for new citations:

```text
opencode://session/<session-id>[/message/<message-id>[/part/<part-id>]]
```

Brackets describe optional coordinates, not literal URI characters. A session
reference names the native container; a message reference narrows it to an
utterance. A part reference identifies the actual native tool call ID belonging
to that message. Do not construct a part target under a session alone: resolving
it requires the message coordinate as well.

Cotail accepts the shorthand
`opencode://<session-id>[/<message-id>[/<part-id>]]`, but maintained examples and
new `sources:` entries use the long form. Session petnames such as
`brave-mountain` are useful labels; the native ID remains the lookup key.

Any of these targets can include `?source=<durable-registered-source-id>`.
The source ID identifies an entry in Cotail's source registry, **not** a channel,
profile, project path, or database path. Qualification selects the registered
store as well as the native target; without it, resolution requires a selected
source. Ordinary session-only knowledge citations may omit `source`. Portable
cross-store exports include it, so a reader does not silently resolve the same
coordinates against a different selected store. Use the actual registered ID;
do not invent one from a path. Qualification does not grant access or freeze
the store's contents.

This is our citation convention, supported by Cotail's URI parse/read/view
interface, not a claim of an upstream OpenCode standard or an installed OS URI
handler. Use tooling with access to the named or selected source. If a reader
needs an exported transcript, cite that artifact too and retain the native URI
as its origin. A session is an evolving container, not a frozen revision or
evidence that every message agrees; use message coordinates, timestamps, or a
bounded quote when the exact utterance matters.

The earlier `urn:opencode:session:<session-id>` spelling remains accepted as a
legacy session reference. Preserve historical source entries; adopting URIs is
not authorization to mass-rewrite prior documents.

For the related distinction between a durable mark and the session it targets,
see [bookmarking](/praxis/bookmarking/README.md). A citation records provenance;
it does not itself create a bookmark or establish acceptance of the cited work.

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
