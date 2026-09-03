---
type: ExperimentLog
title: "jj date preservation and split mechanics — measured on jj 0.40.0"
description: Verified behavior of duplicate/split/rebase/git-import on author and committer timestamps, the split -A descendant-drag incident from the real replay, and its op-log forensics and recovery.
resource: /code/reconstruction/experiments0.glm53h.md
tags: [jj, reconstruction, dates, split, op-log, forensics]
status: stable
generated: { by: model:zai-coding-plan/glm-5.3, at: 2026-09-03 }
verified: { by: agent-verified (paste-tested in scratch repos + the live watchwoman repo), at: 2026-09-03 }
stale_after: 2027-03-03
sources:
  - id: jj-cheatsheet
    resource: file:///home/rektide/archive/doc/jj.md
    title: jj/git branch spelunking cheat sheet
  - id: replay-script
    resource: /code/reconstruction/replay-docs.sh
    title: The replay procedure this log validates
  - id: subject-repo
    resource: file:///home/rektide/src/watchwoman-systemd
    title: Watchwoman repo where the full replay ran
---

# jj date preservation and split mechanics — measured on jj 0.40.0

Supporting evidence for [`README.md`](/code/reconstruction/README.md). Every
claim below was paste-tested either in a scratch repo under
`~/src/rekon/.test-agent/reconstruction/` or against the live watchwoman repo
(jj 0.40.0, `jj --version` = `0.40.0-cc1aa776f6f0eb1f78da33f690fd5ef869ba3e79`).

## Timestamp behavior of rewrite operations

Reading timestamps:

```sh
jj log -r <rev> --no-graph \
  -T '"A:" ++ author.timestamp().format("%Y-%m-%d %H:%M:%S") ++ " C:" ++ committer.timestamp().format("%Y-%m-%d %H:%M:%S") ++ "\n"'
```

jj 0.40 has **no CLI flag and no config key to set either timestamp** — checked
`commit`, `describe`, `new`, `split`, `duplicate`, `bookmark` help pages and
`jj config list`. Preservation is the only mechanism jj offers:

| operation | author date | committer date |
| --- | --- | --- |
| `jj duplicate X` | preserved from X | set to now |
| `jj split` (plain, both halves) | preserved from X | set to now |
| `jj rebase -r X -d Y` (and descendants) | preserved | set to now |
| set a date explicitly | **impossible in jj 0.40** | **impossible** |
| git `commit --amend --date` + `GIT_COMMITTER_DATE`, then `jj git import` | preserved from git | preserved from git |
| any jj rewrite after that import | preserved | reset to now |
| `jj git push` to a remote | exported as stored | exported as stored |

Consequences:

- If "preserve commit dates" means the dates `git log` and GitHub show by
  default (author dates), jj alone satisfies the hard requirement end-to-end.
- Committer dates cannot be preserved across jj rewrites. If they must be
  pinned, do all DAG surgery in jj, then do a final pass in a **colocated**
  repo with git plumbing (below) — and never rewrite through jj again.
- `jj git push` materializes exactly what jj stores: a test commit whose
  author was pinned to 2020-01-01 and committer to the rewrite time arrived
  in a local bare remote as `ai:2020-01-01 00:00:00 +0000
  ci:<rewrite time>`.

## The git-colocation escape hatch (E5)

In a colocated repo, git plumbing sets dates and jj imports them faithfully:

```sh
git checkout -b datetest <jj-commit-sha>
GIT_COMMITTER_DATE="2020-01-01T00:00:00+00:00" \
  git commit --amend --allow-empty --no-edit --date="2020-01-01T00:00:00+00:00"
jj git import
jj log -r <new-sha> --no-graph -T '... author/committer timestamps ...'
# -> both 2020-01-01
```

Traps measured while doing this:

- `git commit --amend` without `--allow-empty` refuses when the amended
  commit would become empty (jj had already made that commit empty relative
  to its parent during earlier experiments).
- After the amend, the bookmark that pointed at the pre-amend jj commit
  showed as **(divergent)** until cleaned up — the git-side branch and the
  jj-side bookmark resolve to two commits with the same change id.
- A later `jj rebase` of the imported commit reset the committer timestamp
  to now while keeping the 2020 author date. Lock dates last, in git, or
  accept committer drift.

## Split mechanics (E2, E7)

Plain `jj split -r <rev> -m "<msg>" <paths>`:

- Selected paths stay in the **original change** (same change id, new commit
  id) with the `-m` description; the remainder becomes a **child** commit
  keeping the original description.
- With `-A/--insert-after <dest>` (and `-B`/`-o`), the **selected** part is
  extracted at `<dest>` and the **remainder** keeps the original change id —
  the change-id assignment inverts between the two forms. Do not parse the
  output text for ids; resolve from the repo.
- Author timestamps are preserved on both halves; committer timestamps
  update.
- **Bookmark forwarding**: splitting a commit that carries a bookmark moves
  the bookmark to the child/remaining part. Never split a commit holding a
  dated snapshot bookmark — duplicate first, split the duplicate.
- Non-interactive use requires BOTH pathsets and `-m`; without `-m` jj opens
  an editor (and dies under a harness with "Failed to edit description").

## The `split -A` descendant-drag incident (real run, recovered)

The first live replay of the watchwoman docs line used
`jj split -r <dup> -A <tip> <docs paths>` for mixed commits. `-A` does not
merely extract: to keep the DAG connected it **rebases `<dest>`'s descendants
on top of the extracted commit**. Because `<dest>` (the growing docs tip, and
`main@origin` for the first commit) is an ancestor of the entire source line,
every split dragged the whole source line — including the `systemd` and dated
`systemd-20260903` snapshot bookmarks — onto the docs line, four times over.

Detection and forensics:

```sh
# walk ops backward; print whenever the bookmark's target changed
jj op log --limit 90 --no-graph -T 'id.short() ++ "\n"' | tac | while read -r op; do
  CUR=$(jj --at-op "$op" log -r 'systemd-20260903' --no-graph -T 'commit_id ++ "\n"')
  ...
done
# -> bookmark moved at each "split commit <sha>" op

jj op show <op-id>      # "Changed commits:" exposed the rebase of the
                        # bookmarked tip and the working-copy commit
```

The damage was metadata-only: `jj diff --from 4eb1ecb7 --to 8cea6582` was
`0 files changed` — same trees, spliced ancestry.

Recovery:

```sh
jj bookmark set --allow-backwards systemd-20260903 -r <original-tip>
jj bookmark set --allow-backwards systemd            -r <original-tip>
jj abandon <shadow-rewrites>                          # clears (divergent)
jj new <original-tip>                                 # re-seat the empty @
```

- `jj bookmark set` refuses to move a bookmark "backwards or sideways"
  without `--allow-backwards`; restoring a dragged snapshot is the legitimate
  use.
- Unhiding the original tip left the shadow rewrite of the same change
  visible as divergent until the shadow was explicitly abandoned; abandoning
  the shadow tip hid its whole ancestry.

After recovery the docs line itself re-verified clean (23 commits, docs-only
diff, author dates intact, docs trees byte-identical to the source line) —
the extracted parts were never wrong, only the collateral rebases were. The
procedure in [`replay-docs.sh`](/code/reconstruction/replay-docs.sh) was then
changed to plain split → abandon remainder → rebase, re-rehearsed in scratch
(bookmarks provably stationary), and re-run for the final line.

## Smaller traps (each one bit during this work)

1. `jj duplicate` prints `Duplicated <old> as <change> <commit> <desc>` on
   **stderr** — a stdout pipe sees nothing. Capture with `2>&1`.
2. Split output columns are not stable: `Selected changes :` (colon separate)
   vs `Remaining changes:` (colon attached). Never parse them for ids;
   resolve via change-id or `children(<tip>)` instead.
3. `jj rebase -r X -d Y` **changes X's commit id**; track the result through
   the stable change-id (`jj log -r <change-id> -T 'commit_id'`). Following
   the pre-rebase sha silently builds on a hidden commit — visible in
   rehearsal as duplicated nodes.
4. An empty-but-described commit (0 files) classifies as "code-only" in a
   file-shape audit; preserve it deliberately if its description is docs.
5. `jj op show <id>` and `jj --at-op <id>` are the ground truth for "what
   moved my bookmark" — use them before theorizing.
6. Two jj commands racing in one shared repo triggered "Concurrent
   modification detected, resolving automatically". Serialize jj work in a
   repo (also a recorded lesson in the opencode flow's manifest).
7. zsh does not word-split unquoted `$var` — the first docs/code audit
   misclassified commits until the classifier moved into awk.
