---
type: Reference
title: "jj rewrite surgery — timestamps, split mechanics, bookmark forwarding"
description: Verified jj 0.40.0 behavior for duplicate/split/rebase on author and committer timestamps (the canonical home of the committer-date material), the plain-vs-A/B/o split forms, bookmark forwarding and divergence, date pinning via the git-colocation escape hatch, and unmixing docs from code.
resource: /vcs/jj/rewrites.md
tags: [vcs, jj, git, rewrites, timestamps, split, bookmarks, divergence, unmixing, jj-hunk]
status: stable
generated: { by: model:zai-coding-plan/glm-5.3-max, at: 2026-09-03 }
verified: { by: inherited from experiments0 (paste-tested in scratch repos + the live watchwoman repo, jj 0.40.0, 2026-09-03); duplicate/split timestamps, change-id assignment, bookmark forwarding, headless split failure, --allow-backwards refusal, and the jj-hunk single-hunk commit flow re-spot-checked in a scratch repo at promotion, at: 2026-09-03 }
stale_after: 2027-03-03
sources:
  - id: experiments0
    resource: /code/reconstruction/experiments0.glm53h.md
    title: "jj date preservation and split mechanics — measured on jj 0.40.0"
    author: model:zai-coding-plan/glm-5.3
    last_modified: 2026-09-03
  - id: reconstruction-readme
    resource: /code/reconstruction/README.md
    title: "Reconstruction with docs preservation — the applied procedure"
    author: model:zai-coding-plan/glm-5.3
    last_modified: 2026-09-03
  - id: jj-cheatsheet
    resource: file:///home/rektide/archive/doc/jj.md
    title: "jj/git branch spelunking cheat sheet (parent document of this set)"
    author: glm-5.3 orchestrator + verification subagent (opencode)
    last_modified: 2026-08-30
  - id: rekon-agents
    resource: /AGENTS.md
    title: "rekon house conventions — the jj-hunk section (hunk-level committing)"
    author: human:rektide
    last_modified: 2026-09-03
---

# jj rewrite surgery — timestamps, split mechanics, bookmark forwarding

Open this when you are **changing history** rather than reading it: duplicating, splitting, rebasing, unmixing, pinning dates, or dealing with bookmarks that a rewrite moved or diverged. For reading/interrogating, start at [`README.md`](/vcs/jj/README.md); for "what operation moved my ref", see [`oplog-forensics.md`](/vcs/jj/oplog-forensics.md).

**Placement rule**: the mainstream case — *jj preserves author dates across rewrite operations, which is all replay work usually needs* — is stated in the [`README.md`](/vcs/jj/README.md#the-date-guarantee-author-dates). This file is the canonical home of everything aside from that: the full both-timestamp semantics, the committer-reset behavior, and the git-colocation recipe for the rare case where committer dates genuinely matter.

Everything here was measured on **jj 0.40.0** — either in the reconstruction experiments (2026-09-03, [`experiments0`](/code/reconstruction/experiments0.glm53h.md)) or re-spot-checked at promotion in a scratch repo. The applied end-to-end procedure (snapshot → unmix → replay → verify) lives in [`code/reconstruction/README.md`](/code/reconstruction/README.md); this file is the mechanics underneath it.

## Reading timestamps

```sh
jj log -r <rev> --no-graph \
  -T 'author.timestamp().format("%Y-%m-%d %H:%M:%S") ++ " / " ++ committer.timestamp().format("%Y-%m-%d %H:%M:%S") ++ "\n"'
```

## Timestamp preservation table

jj 0.40 has **no CLI flag and no config key to set either timestamp** — checked `commit`, `describe`, `new`, `split`, `duplicate`, `bookmark` help pages and `jj config list`. Preservation is the only mechanism jj offers:

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

- Author date is what `git log` and GitHub display by default — **jj alone satisfies "preserve commit dates" end-to-end** for replay/reconstruction.
- Committer dates cannot be preserved across jj rewrites. If they must be pinned, do all DAG surgery in jj, then a final pass in a **colocated** repo with git plumbing (below) — and never rewrite through jj again.
- `jj git push` materializes exactly what jj stores: a test commit whose author was pinned to 2020-01-01 and committer to the rewrite time arrived in a local bare remote as `ai:2020-01-01 00:00:00 +0000 ci:<rewrite time>`.

## Pinning dates via git colocation (the escape hatch)

In a colocated repo, git plumbing sets dates and jj imports them faithfully:

```sh
git checkout -b datetest <jj-commit-sha>
GIT_COMMITTER_DATE="2020-01-01T00:00:00+00:00" \
  git commit --amend --allow-empty --no-edit --date="2020-01-01T00:00:00+00:00"
jj git import
jj log -r <new-sha> --no-graph -T '<timestamps template>'   # -> both 2020-01-01
```

Traps measured while doing this:

- `git commit --amend` without `--allow-empty` refuses when the amended commit would become empty (jj had already made that commit empty relative to its parent).
- After the amend, the bookmark that pointed at the pre-amend jj commit showed as **`(divergent)`** until cleaned up — the git-side branch and the jj-side bookmark resolve to two commits with the same change id ([divergence guide](https://jj-vcs.github.io/jj/latest/guides/divergence/)).
- A later `jj rebase` of the imported commit reset the committer timestamp to now while keeping the 2020 author date. **Lock dates last, in git, or accept committer drift.**

## Split mechanics

Plain `jj split -r <rev> -m "<msg>" <paths>`:

- Selected paths stay in the **original change** (same change id, new commit id) with the `-m` description; the remainder becomes a **child** commit keeping the original description. Re-verified at promotion.
- With `-A/--insert-after <dest>` (and `-B`/`-o`), the **selected** part is extracted at `<dest>` and the **remainder** keeps the original change id — the change-id assignment **inverts** between the two forms. Do not parse the output text for ids; resolve from the repo.
- Author timestamps are preserved on both halves; committer timestamps update (re-verified).
- **Bookmark forwarding**: splitting a commit that carries a bookmark moves the bookmark to the child/remaining part (verified for plain split at promotion: bookmark landed on the remainder). **Never split a commit holding a dated snapshot bookmark — duplicate first, split the duplicate.**
- Non-interactive use requires BOTH pathsets and `-m`; without `-m` jj opens an editor and dies under a harness with `Error: Failed to edit description` (re-verified; jj leaves the description draft in a `/tmp/editor-*.jjdescription` file).

### The `split -A` descendant-drag hazard

`-A` does not merely extract: to keep the DAG connected it **rebases `<dest>`'s descendants on top of the extracted commit**. If `<dest>` is an ancestor of a whole line you care about (it usually is — e.g. a growing replay tip), every split drags that line — bookmarks included — onto the extracted commit. In the watchwoman replay this dragged the source line and its dated snapshot bookmarks four times before it was caught; the damage was metadata-only (same trees, spliced ancestry), and it was diagnosed and recovered via the op log. Full incident, forensics walk, and recovery: [`oplog-forensics.md`](/vcs/jj/oplog-forensics.md) and [`experiments0`](/code/reconstruction/experiments0.glm53h.md). Plain split of a *leaf duplicate* touches nothing else — which is why the replay procedure is duplicate → split → abandon remainder → rebase.

## Unmixing docs from code

Three situations, three tools — prevention first:

1. **Before committing** (best): commit in two steps —
   `jj commit -m "docs: ..." <doc-paths>` then `jj commit -m "feat: ..."` for the remainder. `jj commit <paths>` keeps the named paths in the described commit and moves the rest to a new working-copy commit on top.
2. **Already committed on a live line**: `jj split -r <rev> -m "<docs msg>" <doc-paths>` splits it in place — mind the bookmark forwarding above: a bookmark on the commit moves to the remainder half, so if the commit carries a snapshot bookmark, duplicate-then-split instead (the replay procedure is exactly this).
3. **Doc and code changes share one file** (doc comments in source, a README hunk inside a code commit), or a dirty file mixes work from different sessions: file-level tools are not enough — commit at the **hunk** level with `jj-hunk`:

   ```sh
   jj new <parent>                     # fresh working-copy commit to receive the doc hunks
   jj restore --from <mixed> -- <shared-file>
   jj-hunk list --format text          # discover hunk ids — full ids only; short prefixes silently match nothing
   jj-hunk list --spec '<spec-json>'   # DRY-RUN: confirm exactly which hunks the spec selects
   jj-hunk commit '<spec-json>' "docs: <subject>"
   jj commit -m "feat: <subject>"      # remainder
   ```

   The spec shape is `{"files": {"<path>": {"ids": ["hunk-<sha256>", ...]}}}`. In the JSON output, `added`/`removed` are plain strings (not arrays), so e.g. `jj-hunk list | jq -r '.files[].hunks[] | select(.added | contains("<marker>")) | .id'` finds a hunk by content. The flow — list, dry-run, commit one hunk, leaving the unrelated hunks dirty for their own commit — was re-spot-checked end-to-end at promotion; it was also used for real to isolate a single addendum hunk out of a file carrying a prior session's unrelated edits (2026-09-03). Then `jj rebase` the pair into place, or use them as the replay source.

## Tracking commits across rewrites

- `jj rebase -r X -d Y` **changes X's commit id**; track the result through the stable change id: `jj log -r <change-id> -T 'commit_id'`. Following the pre-rebase sha silently builds on a hidden commit — visible in rehearsal as duplicated nodes.
- `jj duplicate` prints `Duplicated <old> as <change> <commit> <desc>` on **stderr** — a stdout pipe sees nothing. Capture with `2>&1` (field 5 is the new commit id).
- Split output columns are not stable (`Selected changes :` with a space, `Remaining changes:` without). Never parse them for ids; resolve via change id or `children(<tip>)`.

## Rewrite-time traps

1. `jj duplicate` reports on stderr (above).
2. Split output columns are unstable (above).
3. Rebase changes commit ids; follow change ids (above).
4. An empty-but-described commit (0 files) classifies as "code-only" in a file-shape audit; preserve it deliberately if its description is docs.
5. Two jj commands racing in one shared repo trigger "Concurrent modification detected, resolving automatically". Serialize jj work in a repo.
6. zsh does not word-split unquoted `$var` — classifiers that iterate over command output belong in awk.
7. If a rewrite ever drags a dated snapshot bookmark, restore it with `jj bookmark set --allow-backwards` (it refuses "backwards or sideways" moves without the flag) — the walk that finds *what* moved it is in [`oplog-forensics.md`](/vcs/jj/oplog-forensics.md).

## Cross-references

- [`vcs/jj/README.md`](/vcs/jj/README.md) — the set's entry point: reading and interrogating repos, revset/template cheats, traps.
- [`vcs/jj/oplog-forensics.md`](/vcs/jj/oplog-forensics.md) — what moved, when, and how to recover.
- [`code/reconstruction/README.md`](/code/reconstruction/README.md) — the applied procedure built on these mechanics (snapshot-first doctrine, unmix, verify, bookmark).
- [`code/reconstruction/experiments0.glm53h.md`](/code/reconstruction/experiments0.glm53h.md) — the primary measurements, including the colocation experiment (E5) and the split-`-A` incident.
- [jj divergence guide](https://jj-vcs.github.io/jj/latest/guides/divergence/) — upstream explanation of `(divergent)` changes and change-id twins.
