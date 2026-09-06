---
name: code
type: Guide
title: "Coding practice — commits, review, planning, posture"
description: Use when writing code, making commits, responding to review comments, or writing plans and designs: commit discipline with jj, hunk-level selection, review protocol, planning rules, and coding posture. The common cases live here; nitty-gritty lives in the support docs.
resource: /code/README.md
tags: [code, commits, review, planning, jj, jj-hunk, practice]
status: draft
generated: { by: model:zai-coding-plan/glm-5.3-max, at: 2026-09-04 }
stale_after: 2027-03-04
sources:
  - id: agents-operating-manual
    resource: /AGENTS.md
    title: "The coding-practice sections of the workspace operating manual this module promotes (human-codified, evolved since 2026-02)"
    author: human:rektide
    last_modified: 2026-09-04
  - id: vcs-jj
    resource: /vcs/jj/README.md
    title: "The jj+git knowledge home — owns history spelunking and rewrite mechanics"
  - id: js-precedent
    resource: /js/README.md
    title: "The promoted-module precedent this collection follows"
---

<a id="code"></a>

# Coding practice

This is the home for everyday coding-work practice: how we commit, how we
respond to review, how plans and designs are written, and the posture rules
that keep code honest. It carries the **common weight inline** — the rules
most agents need most sessions — while the support docs hold the
nitty-gritty for specific situations.

This is the promoted home for coding-practice guidance that previously lived
only in the global `AGENTS.md`. Until this skill is fully accepted, the
global file remains authoritative where they overlap; reconcile differences
here rather than diverging silently.

<a id="code-inventory"></a>

## The collection

| Doc | Load when |
| --- | --- |
| [`commits.md`](/code/commits.md) | Committing work: mechanics, message discipline, jj guardrails beyond the basics below. |
| [`jj-hunk.md`](/code/jj-hunk.md) | Doing hunk-level work — intermingled changes inside files, no-TTY clean commits. |
| [`review-handling.md`](/code/review-handling.md) | Review comments have arrived and corrections are due. |
| [`planning.md`](/code/planning.md) | Writing a plan or design document. |
| [`coding-advice.md`](/code/coding-advice.md) | Deciding error handling or cleanup behavior. |
| [`task-reconstruction/`](/code/task-reconstruction/README.md) | Restarting a task line while preserving its docs: the jj replay procedure, its philosophy, and measured evidence. |

Neighbors, not members: [`vcs/jj/`](/vcs/jj/README.md) owns jj/git history
spelunking and rewrite mechanics; [`js/`](/js/README.md) owns the
JavaScript/TypeScript toolchain; [`constitution/`](/constitution/README.md)
owns the documentation module system itself.

<a id="code-committing"></a>

## Committing

- Commit each coherent stage as it becomes available—code, prompts, research,
  drafts, and guidance. Do not bank stages until the task or wave ends.
- Before a review-driven revision or revamp, commit the current owned state;
  then commit the revision separately and explain its delta and destination.
  A checkpoint preserves an argument; it does not declare acceptance.
- `jj commit -m "<description>" <files...>` — explicitly provide the paths
  that belong to the commit; everything else stays in the working copy.
- The empty working-copy commit `jj commit` leaves behind is expected
  (jj auto-stages everything and starts a new empty commit). Not a bug.
- If you used `jj commit`, the work is saved. Do not verify; move on.
- Do not run `jj status` first — assume you are the only actor in the
  directory.
- Message discipline: describe the work. Relevant ticket IDs go in the
  **body**, never the title. No "phases" in titles — that is short-term
  planning, not long-term meaning.
- Never `jj edit`. Never `squash` unless explicitly instructed — humans
  review all changes before squash.
- Review your diff with `jj diff --git` or `jj diff --stat`; plain `jj
  diff` is misleading without color.
- When logical changes intermingle **inside** files, reach for
  [`jj-hunk`](/code/jj-hunk.md).

Depth: [`commits.md`](/code/commits.md).

<a id="code-review-handling"></a>

## Responding to review

- When comments arrive, **commit the present owned state before revising it**
  and discuss the direction with the human. Saving a predecessor and checking
  in conversationally are separate obligations; do not silently rework.
- Break corrections up into individual, well-argued, cohesive commits:
  one logical correction per commit, with a commit message that argues
  the change.

Depth: [`review-handling.md`](/code/review-handling.md).

<a id="code-planning"></a>

## Planning

- Never write time-frames like "days" or "weeks" into plans. Estimates
  are crude and will not match delivery; leave them out.
- Breaking changes are fine while a project is `<1.0`.
- Never a flat project layout: domain-grouped structure, always. When
  asked for options, offer several plausible domain groupings.
- Mermaid diagrams are excellent visual references — use meaningful node
  identifiers when creating them.

Depth: [`planning.md`](/code/planning.md).

<a id="code-coding-advice"></a>

## Coding posture

- **Let It Fail**, when it makes sense: do not create a try-catch just to
  print an error — the stack trace shows what is wrong.
- Pruning stale or undeclared artifacts is usually out of scope. Do not
  add automatic prune/reconciliation behavior unless explicitly requested:
  create and update declared state, leave cleanup deliberate.

Depth: [`coding-advice.md`](/code/coding-advice.md).

<a id="code-cross-references"></a>

## Cross-references

- [`/AGENTS.md`](/AGENTS.md) — the operating manual this module promotes
  from; authoritative until this collection is accepted.
- [`/vcs/jj/README.md`](/vcs/jj/README.md) — jj+git reading and rewrite
  reference: revsets, templates, traps, forensics.
- [`/js/README.md`](/js/README.md) — same promotion pattern, JavaScript
  domain.
- [`/constitution/README.md`](/constitution/README.md) — the module
  contract (README canonical, `SKILL.md` symlink, `GLOBAL.md` fragment)
  this collection follows.
