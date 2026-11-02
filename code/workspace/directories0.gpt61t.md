---
type: Proposal
title: Common workspace directories index
description: A proposed discoverable home and explicit path notation for local workspace directories and aliases.
resource: /code/workspace/directories0.gpt61t.md
status: draft
generated: { by: "model:openai/gpt-6.1-terra#high", at: 2026-11-03 }
sources:
  - { resource: /code/workspace/GLOBAL.md, title: Current ambient workspace locations }
  - { resource: /code/README.md, title: Coding practice and workspace fragment inventory }
  - { resource: /constitution/naming/README.md, title: Scoped aliases and shortcodes }
  - { resource: /vcs/jj/waltz.md, title: OpenCode donor alias and preparation boundary }
  - { resource: /design/rekon/ado-recovery0.glm53m.md, title: Earlier investigation of the OpenCode notes home }
  - { resource: "file:///home/rektide/ado/README.md", title: OpenCode notes index }
  - { resource: "file:///home/rektide/ado/patches.md", title: OpenCode maintenance procedures and path examples }
  - { resource: "opencode://ses_e6143e1e5ffeewf1KkLGRgz75C", title: Investigate common directory references and propose a maintained index }
---

# A common directories index

**Proposal, not an adopted convention.** The immediate ambiguity is real:
`ado/patches.md` looks like a path relative to the reader's current directory,
not a home-directory path. The OpenCode notes index calls it `patches.md` in
its own context; elsewhere `~/ado/patches.md` gives the missing root.

## Home and shape

Create a maintained `code/workspace/README.md` as the workspace locations home,
next to the existing [ambient fragment](/code/workspace/GLOBAL.md), rather
than turning [shortcodes](/constitution/naming/README.md#short-codes) into a
directory registry. `code/README.md` already points to the workspace fragment;
if adopted, add the README there and a short pointer from the fragment. The
index should navigate to authoritative project/topic documents, not take over
their live operating procedures. Distinguish a filesystem alias from a concern
code, a checkout from its repository, and a current local observation from a
portable guarantee.

Use a compact table: **Reference / resolves to or placement / purpose / owner
or caveat**. Group shared roots first and project-specific shortcuts second;
label observed symlinks as local and date checked. Do not imply each variant
must exist. A path's meaning or existence can change independently of the index.

## Notation to consider

- Write `~/ado/patches.md` when the intent is the operator's home, never bare
  `ado/patches.md` as an implicit synonym. Explain `~` as the invoking user's
  home in shell contexts; this inventory describes **this operator's** home,
  not a promise about other accounts or remote hosts.
- For Markdown navigation use a real link target such as
  [`~/ado/patches.md`](file:///home/rektide/ado/patches.md), since Markdown
  does not expand `~`. `file://` is machine-specific; prefer repository URLs
  for portable external citations where available. `/foo` in Rekon's own
  Markdown denotes its bundle root, not the host filesystem; explicit absolute
  shell paths and file URLs mean something different.
- A bare relative path is relative to an explicitly identified document,
  repository, or command working directory; don't let a glossary silently
  change shell or Markdown resolution. Introduce any conversational alias
  alongside its full home path; no resolver or new code is implied.

## Candidate first rows

These are scoped local observations checked 2026-11-03, with meanings grounded
in the named documents; they are seeds for review, not an exhaustive inventory.

| Reference | Resolution or placement | Purpose and boundary |
| --- | --- | --- |
| `~/src/` | `/home/rektide/src/` | First-party checkouts generally; variants often use `<repo>-<variant>` ([workspace guidance](/code/workspace/GLOBAL.md)). |
| `~/archive/<org>/<repo>` | Under `/home/rektide/archive/` | Third-party source lookup before web research; not a claim that every entry is third-party. |
| `~/wiki/<org>/<repo>` | `~/wiki` is locally a symlink to `~/deepwiki.com` | May contain generated architectural notes; existence and freshness vary. |
| `~/archive/doc/` and `~/src/doc/` | The latter resolves to the former | Broad-topic notes; a shared notes repository, not each project's own `doc/`. |
| `~/ado/` | Resolves to `~/archive/doc/opencode/` | [OpenCode notes index](file:///home/rektide/ado/README.md); `~/ado/patches.md` is its maintenance and ideas record. The directory is not the OpenCode source checkout. |
| `~/a/a/opencode/` | Resolves to `~/archive/anomalyco/opencode/` | OpenCode primary donor location; [waltz guidance](/vcs/jj/waltz.md#jj-waltz-source) does not guarantee it is prepared. |
| `~/oc/working` | Resolves to `~/src/opencode-working` | Local OpenCode compose workspace alias; consult [patches](file:///home/rektide/ado/patches.md) for its current gates and lifecycle, not this table. |
| `~/ff/` | Resolves to `~/src/future-fuze/` | Future-fuze Kubernetes resources, per workspace guidance. |
| `/tmp/opencode/` | Absolute host path, not a home alias | Harness-approved temporary location for this environment; ephemeral, not a durable evidence home. |

## Minimal ambient exposure and decisions

If accepted, keep existing archive-first and scratch rules in `GLOBAL.md`;
add only: “For common local directories, aliases and their scope, see
`/code/workspace/README.md`; spell home paths with `~/`.” Make that a real
bundle-root link. Generate `AGENTS.md` only as part of a separately authorized
integration.

Open choices: whether the index should cover only frequently used paths or
also project-specific aliases; whether the proposed workspace README should
own broader shell guidance; and how portable links should be for notes without
a public repository URL. Confirm ownership and current targets with the
operator before promoting the list. This inquiry changes no existing guidance.
