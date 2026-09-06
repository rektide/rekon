---
type: Reference
title: "Commit discipline with jj"
description: The mechanics and guardrails of day-to-day committing: file-scoped jj commit, the auto-staged working-copy commit, message discipline, diff review, the never-edit/never-squash rules, and where the jj DSL must not be guessed.
resource: /code/commits.md
tags: [code, commits, jj, commit-messages, diffs]
status: draft
generated: { by: model:zai-coding-plan/glm-5.3-max, at: 2026-09-04 }
stale_after: 2027-03-04
sources:
  - id: agents-operating-manual
    resource: /AGENTS.md
    title: "The jj/git and commit-message rules of the workspace operating manual (human-codified)"
    author: human:rektide
    last_modified: 2026-09-04
  - id: checkpoint-direction
    resource: urn:opencode:session:ses_f872bf58affeS5IBlC38cA6iYX
    title: Capture each stage before review-driven revamp; track the delta and destination
    author: human:rektide
    last_modified: 2026-09-06
---

<a id="commits"></a>

# Commit discipline with jj

The everyday loop: do a logical unit of work, commit it with the files that
belong to it, describe it honestly, move on. Depth behind the
[README committing section](/code/README.md#code-committing).

<a id="commits-checkpoints"></a>

## Capture the stage before changing direction

**Commit a coherent stage when it exists.** Do not accumulate several stages
until a feature, conversation, or wave is finished. This applies equally to
code, prompts, research, assessments, design drafts, and agent guidance. A draft
can be worth preserving while its conclusions are still open.

The loop is **capture → revise → compare**:

1. Before applying review feedback, revamping, replacing, or removing current
   work, commit the present **owned** changes with explicit paths or hunks.
   If that state is already committed, use its existing identity; do not invent
   a redundant checkpoint merely to perform a ritual.
2. State the new direction: what pressure or feedback motivates it, what should
   remain, and what destination the revision heads toward.
3. Revise in a distinct logical change. Commit that change separately and
   explain the delta against its predecessor. For independent or substantially
   different documents, prefer a new artifact and cite the earlier one.

Useful save points include a first usable prompt, an independent report, a
source-grounded finding, a completed synthesis, or a proposal about to be
reworked. These are coherent states, not a demand to commit every keystroke.
Do not combine a draft, its criticism, and its replacement into a single
history entry that erases the intermediate argument.

**A checkpoint is not acceptance, completion, publication, or permission to
act.** An evidence gate may postpone comparison, but each author should still
commit their result when it is available. Waiting for the slowest participant
is not a reason to leave completed artifacts uncommitted.

An uncommitted `AGENTS.md` edit is still an active instruction in hosts that
reload it. Stage unsettled policy in an ordinary proposal document, not in the
live instruction file. Preserve existing user and peer changes outside your
commit; use [hunk selection](/code/jj-hunk.md) when ownership shares a file.

<a id="commits-mechanics"></a>

## Mechanics

```sh
jj commit -m "feat: <description>" <files...>
```

- `jj commit <paths>` keeps the named paths in the described commit and
  moves everything else into a new working-copy commit on top. You do not
  need to stage anything ahead of time.
- **Empty commits are fine.** `jj commit` commits all current changes and
  starts a new empty commit — that empty `@` afterwards is the expected
  automatic staging state, not an error to fix.
- **Do not verify the commit happened.** If you ran `jj commit`, the work
  is saved. Move on.
- You may assume you are the only actor modifying a directory (unless told
  otherwise), so skip the `jj status` pre-flight. There may be unchecked-in
  changes when you start; after that, changes are your own.
- Committing docs and code that are both ready? Commit in two steps —
  `jj commit -m "docs: ..." <doc-paths>` first, then the code — rather than
  mixing and splitting later (the full three-situation unmixing table lives
  in [`/vcs/jj/rewrites.md`](/vcs/jj/rewrites.md)).

<a id="commits-messages"></a>

## Message discipline

- The title **describes the work**. "Closing ticket \_\_\_" does not.
- Relevant ticket IDs go in the **body** of the message, never the title.
- No "phases" of work in the title — short-term planning information is
  not long-term meaning.
- Commit messages are the review surface: a reader should understand what
  changed and why from the message plus the diff, without the session that
  produced it.

<a id="commits-diff-review"></a>

## Reviewing your own diff

```sh
jj diff --git     # diff style, the honest view
jj diff --stat    # summary
jj log --limit 10 -r '::@' -T 'change_id.short() ++ " " ++ description.first_line() ++ "\n"'
```

Plain `jj diff` without color is misleading — prefer `--git` or `--stat`.
Useful log template columns: `author.timestamp().ago()`,
`author.name()`, `commit_id.short()`, `bookmarks`, `tags`, `divergent`,
`immutable`, `mine()`, `parents.map(|c| c.commit_id().short())`.

<a id="commits-guardrails"></a>

## Guardrails

- **Never `jj edit`.**
- **Never `squash` unless explicitly instructed** — humans must review all
  changes before squash.
- Do not push work; the human pushes when they want to.
- When changes intermingle within a file, [`jj-hunk`](/code/jj-hunk.md)
  selects hunks programmatically; for wholesale-rewritten files use the
  restore-then-recommit dance it documents.

<a id="commits-dsl"></a>

## Do not guess the jj DSL

jj revsets and templates are their own languages. Before writing `-r` or
`-T` arguments, consult the help: `jj help -k revsets` and
`jj help -k templates`.

- Revset DAG operators: `::@` ancestors of `@`, `@::` descendants, `::`
  all visible, `x..y` range. There is no `::~`.
- Template fields are bare keywords, not object properties:
  `change_id.short()`, `description.first_line()` — never
  `commit.change_id.short()`.
- String concatenation in templates uses `++`.

The full cheat sheet — reading repos, exit codes, traps — is
[`/vcs/jj/README.md`](/vcs/jj/README.md).

<a id="commits-cross-references"></a>

## Cross-references

- [`/code/README.md#code-committing`](/code/README.md#code-committing) —
  the common-weight summary this doc deepens.
- [`/code/jj-hunk.md`](/code/jj-hunk.md) — hunk-level selection when file
  boundaries do not align with logical commits.
- [`/vcs/jj/README.md`](/vcs/jj/README.md) — jj/git reading, revset and
  template cheats, traps.
- [`/vcs/jj/rewrites.md`](/vcs/jj/rewrites.md) — split mechanics,
  timestamp preservation, unmixing docs from code.
