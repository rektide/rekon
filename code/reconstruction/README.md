---
type: Guide
title: "Reconstruction with docs preservation — the jj procedure"
description: Step-by-step procedure for restarting a code line on the upstream tip while preserving every docs commit and its author date; snapshot-first doctrine, mixed-commit unmixing, verification, and the author-date guarantee (the committer-date aside lives in vcs/jj/rewrites.md).
resource: /code/reconstruction/README.md
tags: [code, reconstruction, jj, docs, preservation, snapshot, split, replay]
status: draft
generated: { by: model:zai-coding-plan/glm-5.3, at: 2026-09-03 }
stale_after: 2027-03-03
sources:
  - id: human-direction
    title: Snapshot first, preserve all docs commits, unmix with jj split, preserve commit dates, prefer jj over git
    author: human:rektide
    last_modified: 2026-09-03
  - id: patches-flow
    resource: file:///home/rektide/archive/doc/opencode/patches.md
    title: The opencode working-stack flow this procedure generalizes
  - id: jj-cheatsheet
    resource: file:///home/rektide/archive/doc/jj.md
    title: jj/git spelunking cheat sheet
  - id: reconstruct-philosophy
    resource: /code/reconstruct/README.md
    title: Reconstruction is not replay — the philosophy this implements
  - id: experiments
    resource: /code/reconstruction/experiments0.glm53h.md
    title: Measured jj 0.40.0 date/split behavior backing every claim here
  - id: worked-example
    resource: file:///home/rektide/src/watchwoman-systemd
    title: Watchwoman repo where this procedure ran end-to-end 2026-09-03
  - id: jj-guide
    resource: /vcs/jj/README.md
    title: The promoted jj+git working guide (vcs/jj set) — rewrites.md there is the canonical home of the committer-date aside
    author: model:zai-coding-plan/glm-5.3-max
    last_modified: 2026-09-03
---

# Reconstruction with docs preservation — the jj procedure

[Reconstruction is not replay](/code/reconstruct/README.md): the old code is
evidence, not the plan. But one class of history must survive a restart
intact — **the docs**. Design waves, deployment notes, decision records
(`.design/` and friends) are the accumulated understanding that the rebuild
will need. This is the concrete procedure: snapshot the present, then rebuild
a line that is *upstream tip + every docs commit*, with each commit's author
date preserved.

It generalizes the bookmark/freshen/compose flow documented for opencode in
[`patches.md`](file:///home/rektide/archive/doc/opencode/patches.md): same
dated-bookmark convention, same duplicate-then-rebase never-rewrite doctrine,
extended with the docs-preservation constraint and a verified answer to "can
jj keep my dates?"

## Requirements this procedure enforces

1. **Snapshot first.** Before any reconstruction work, the current state gets
   an enduring dated bookmark. This should be a standing requirement of any
   reconstruction: work is never lost to an experiment, and the snapshot is
   addressable in tool-call history.
2. **Every docs commit survives** — including docs-only commits, the docs
   half of mixed commits, and empty-but-described docs commits.
3. **Author dates are preserved** — exactly, and natively: jj keeps author
   dates across `duplicate`/`split`/`rebase`, and author date is what
   `git log` and GitHub display. (Committer dates cannot be preserved across
   jj rewrites; that case and its git fallback are an aside in
   [`/vcs/jj/rewrites.md`](/vcs/jj/rewrites.md), not part of this
   procedure.)
4. **jj is the primary tool**; git is a fallback for date surgery only.
5. Originals and snapshots are **never rewritten**: all work happens on
   duplicates.

## The procedure

### 0. Commit the working copy, then snapshot

If the working copy holds real (even mixed) work, commit it as one honest
commit first — do not leave work outside history before a reconstruction.
Then place the floating and dated bookmarks in **separate, explicit tool
calls** (deliberately auditable):

```sh
jj commit -m "feat: ...; docs sync" <all-changed-paths>
jj bookmark set <feature> -r @-          # fast-forward floating bookmark
jj bookmark create <feature>-YYYYMMDD -r @-   # immutable dated snapshot
```

The dated bookmark is the point-in-time snapshot you can always come back to.
Never move it later; if a rewrite ever drags it, restore with
`jj bookmark set --allow-backwards` (see
[experiments](/code/reconstruction/experiments0.glm53h.md)).

### 1. Audit the line

Classify every commit on the line by the files it touches, using one docs
predicate everywhere (here: `.design/`, `docs/`, `doc/`, `design/` subtrees
plus any `*.md`/`*.mdx`):

```sh
jj log -r 'main@origin..<line>' --no-graph -T 'commit_id ++ "\n"' | tac \
| while read -r rev; do
    jj diff -r "$rev" --summary | awk -v rev="$rev" '
      { f = $2
        doc = (f ~ /^\.design\// || f ~ /^docs?\// || f ~ /^design\// \
               || f ~ /\.mdx?$/)
        if (doc) d = 1; else c = 1 }
      END { print rev, (d && c) ? "MIXED" : (d ? "docs" : (c ? "code" : "EMPTY")) }'
  done
```

Expect four classes: `docs`, `code`, `MIXED`, and occasionally `EMPTY`
(described but contentless — usually a docs edit that landed in the previous
commit; preserve them, their description is the record). Two zsh traps: put
the classifier in awk (unquoted `$var` does not word-split), and remember
`jj diff --summary` prints `R old new` for renames.

### 2. Replay onto the upstream tip

Per commit, oldest first (see [`replay-docs.sh`](/code/reconstruction/replay-docs.sh)
for the executable version):

- **docs** → duplicate, rebase onto the growing tip.
- **MIXED** → duplicate, `jj split` the duplicate with the docs paths
  (selected half keeps the duplicate's change id and a `docs: ...`-prefixed
  description citing the source commit), abandon the code remainder, rebase
  the docs half onto the tip.
- **code** → skip (it stays on the source line).
- **EMPTY** → duplicate and rebase like docs.

```sh
DUP=$(jj duplicate "$REV" 2>&1 | awk '/^Duplicated/ {print $5}')   # stderr!
jj split -r "$DUP" -m "docs: <orig subject> (docs portion of <short-sha>)" \
  <docs-paths>
SEL=$(jj log -r "$DUP_CHANGE" -T 'commit_id')   # selected keeps change id
jj abandon "$(jj log -r "children($SEL)" -T 'commit_id')"   # code remainder
jj rebase -r "$SEL" -d "$TIP"
TIP=$(jj log -r "$DUP_CHANGE" -T 'commit_id')   # rebase changed the id!
```

**Do not use `jj split -A <dest>` here.** `-A` rebases `<dest>`'s
descendants — your entire source line, bookmarks included — on top of the
extracted commit. In the watchwoman run this dragged the dated snapshot
bookmark four times before it was caught and restored. Plain split of a leaf
duplicate touches nothing else. Full incident and forensics in
[experiments0](/code/reconstruction/experiments0.glm53h.md).

Because docs paths and code paths are disjoint, these replays are
conflict-free by construction; treat any conflict as a bail-out signal, not
something to resolve creatively (same guardrails as a freshen).

### 3. Verify before bookmarking

```sh
# only docs paths changed, vs upstream
jj diff --from main@origin --to "$TIP" --summary \
  | awk '{print $2}' | grep -vE '^(\.design/|docs?/|design/)|\.md$'   # expect nothing
# every replayed commit kept its author date
jj log -r 'main@origin..'"$TIP" --no-graph -T 'author.timestamp() ++ " " ++ description.first_line() ++ "\n"'
# docs trees identical to the source line's docs
jj diff --stat --from "$TIP" --to <feature>-YYYYMMDD -- .design design docs README.md CHANGELOG.md
# snapshot unmoved
jj log -r <feature>-YYYYMMDD -T 'commit_id ++ " " ++ bookmarks ++ "\n"'
```

### 4. Bookmark the reconstruction

```sh
jj bookmark create docs -r "$TIP"                 # floating
jj bookmark create docs-YYYYMMDD -r "$TIP"        # dated snapshot of this rebuild
```

## Unmixing docs from code

Three situations, three tools:

1. **Before committing** (best): commit in two steps —
   `jj commit -m "docs: ..." <doc-paths>` then `jj commit -m "feat: ..."`
   for the rest. `jj commit <paths>` keeps the named paths in the described
   commit and moves the rest to a new working-copy commit on top.
2. **Already committed on a live line**: `jj split -r <rev> -m "<docs msg>"
   <doc-paths>` splits it in place. Note the bookmark-forwarding behavior:
   a bookmark on the split commit moves to the child (remainder) half, so if
   the commit is snapshotted, duplicate-then-split instead (the replay above
   is exactly this).
3. **Docs and code changes share one file** (doc comments in source, a
   README hunk inside a code commit): file-level split is not enough. Bring
   the file's mixed state into a fresh working-copy commit and use
   `jj-hunk` to commit only the doc hunks:

   ```sh
   jj new <parent-of-mixed>
   jj restore --from <mixed> -- <shared-file>
   jj-hunk list --files                                  # discover hunk ids
   jj-hunk commit '<spec-json>' "docs: <subject>"        # doc hunks only
   jj commit -m "feat: <subject>"                        # remainder
   ```

   Then `jj rebase` the pair into place, or use them as the replay source.

## Dates: the author-date guarantee

**jj preserves author dates across `duplicate`, `split`, and `rebase` —
natively, zero extra machinery.** The whole replay above keeps them without
any date work. Author date is what `git log` and GitHub display, so **jj
alone meets the hard requirement** (measured on jj 0.40.0:
[experiments0](/code/reconstruction/experiments0.glm53h.md)).

One-line verification — run it over the replayed line before bookmarking:

```sh
jj log -r 'main@origin..'"$TIP" --no-graph -T 'author.timestamp() ++ " " ++ description.first_line() ++ "\n"'
```

Committer dates reset to now on every rewrite, and jj cannot *set* either
timestamp; if committer dates genuinely matter, that case — the
git-colocation pinning recipe, its `(divergent)` bookmark trap, and the
lock-dates-last rule — is an aside kept in
[`/vcs/jj/rewrites.md`](/vcs/jj/rewrites.md).

## Worked example — watchwoman, 2026-09-03

Source line: 37 commits above `main@origin` (upstream tip `d074ebac`) on the
`systemd` floating bookmark.

| step | result |
| --- | --- |
| snapshot | `systemd` fast-forwarded, `systemd-20260903` → `4eb1ecb7` |
| audit | 15 docs (incl. 1 empty-described), 8 mixed, 14 code |
| replay | 23 docs commits onto `main@origin`, zero conflicts |
| verification | docs-only diff; 23/23 author dates byte-equal; docs trees identical to source line |
| result | `docs` + `docs-20260903` → `2e13d504` |

("v2@origin+docs" in the request maps to this repo's `main@origin` — the
upstream default branch; the phrase carries over from the opencode flow.)

One incident during the run: the first pass used `split -A` and dragged the
source line's bookmarks (content untouched, ancestry spliced). Detected via
`jj op log` / `jj --at-op` walk (deep dive:
[`/vcs/jj/oplog-forensics.md`](/vcs/jj/oplog-forensics.md)), restored with
`--allow-backwards` bookmark moves and shadow abandons, procedure fixed,
replay re-run clean. The final line comes from the fixed procedure.

## What this gives the rebuild

The docs line is the **carrier** across the restart (see the
[carrier pattern](/code/reconstruct/README.md)): a small, dated, verifiable
line holding every decision record, pinned to the upstream tip, with dates
that still tell the truth about when understanding was written down. The code
halves stay on the snapshotted source line as evidence. New work starts from
`main@origin` (or the docs line, when the rebuild wants the docs in-tree) —
never from the old code path by default.

## Cross-references

- [`code/reconstruct/README.md`](/code/reconstruct/README.md) — the
  philosophy: reconstruct from present intent, history as evidence. This
  document is its executable counterpart for the docs-preservation
  constraint.
- [`experiments0.glm53h.md`](/code/reconstruction/experiments0.glm53h.md) —
  measured jj behavior and the incident log backing every claim above.
- [`replay-docs.sh`](/code/reconstruction/replay-docs.sh) — the replay as a
  parameterized, rehearsed script (repo, base, line).
- [`patches.md`](file:///home/rektide/archive/doc/opencode/patches.md) — the
  opencode working-stack flow whose bookmark conventions and
  duplicate-then-rebase doctrine this generalizes.
- [`/vcs/jj/README.md`](/vcs/jj/README.md) — the promoted jj+git working
  guide: command equivalences, traps, and the op-log forensics used here
  (provenance: the paste-tested
  [`jj.md`](file:///home/rektide/archive/doc/jj.md) cheat sheet it grew
  from, 2026-08-29/30).
