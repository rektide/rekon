---
type: Reference
title: "git-only deep forensics — the jobs jj doesn't have"
description: The jobs where git stays canonical in a jj+git collocated clone — live remote heads, conflict dry-runs, patch-equivalence, exact per-file counts, match extraction — with the no-jj-equivalent table.
resource: /vcs/jj/git-only.md
tags: [vcs, git, jj, forensics, patch-id, merge-tree, ls-remote]
status: stable
generated: { by: model:zai-coding-plan/glm-5.3-max, at: 2026-09-03 }
verified: { by: inherited from the jj cheat sheet — git commands used in the live 50-branch review, jj absences verified by scanning the full jj 0.40.0 command list (2026-08-29/30), at: 2026-09-03 }
stale_after: 2027-03-03
sources:
  - id: jj-cheatsheet
    resource: file:///home/rektide/archive/doc/jj.md
    title: "jj/git branch spelunking cheat sheet — the 'No jj equivalent' section and paired commands"
    author: glm-5.3 orchestrator + verification subagent (opencode)
    last_modified: 2026-08-30
  - id: experiments0
    resource: /code/reconstruction/experiments0.glm53h.md
    title: "jj 0.40.0 measured behavior — the version pin for the absences claimed here"
    author: model:zai-coding-plan/glm-5.3
    last_modified: 2026-09-03
---

# git-only deep forensics — the jobs jj doesn't have

Open this when the job falls in jj's honest gaps. **Doctrine** (from the 50-branch review): in a jj+git *collocated* clone, remote-ref forensics is plain git — five review agents used zero jj commands on purpose and never missed them. Where jj has no honest equivalent, the git command stays canonical; this file owns those jobs so the pairing tables elsewhere can simply point here. All absences were verified against the full jj 0.40.0 command list; re-check on major version bumps.

| job | git | nearest jj idiom |
| --- | --- | --- |
| live remote heads (network) | `git ls-remote --heads origin 'glob'` | none — `jj bookmark list --all-remotes` shows last-fetched refs only |
| conflict dry-run, no worktree | `git merge-tree --write-tree A B` | none — `jj new A B` creates real commits (a write); `jj resolve` only works in a checked-out merge |
| patch-equivalence / cherry detection | `git cherry`, `git patch-id --stable` | none — no content-hash equivalent of commits |
| exact +/- per file | `git diff --numstat` | none — `jj diff --stat` only (and its counts differ slightly from git's) |
| match extraction in a revision | `git grep -o/-c <pat> <rev> -- <path>` | partial — `jj file search` names matching files (rc=0 on no-match); pipe `jj file show -r <rev> <path> \| grep` for counts/extraction |

## Live remote heads: `ls-remote`

```sh
git ls-remote --heads origin 'browser*'     # AUTHORITATIVE, over the network, right now
```

`jj bookmark list --all-remotes` reads **last-fetched** refs — fine for most narrative work, wrong the moment you need to know what the remote holds *now* (e.g. before declaring a branch deleted). `ls-remote` is the only probe that answers that. ([git-ls-remote](https://git-scm.com/docs/git-ls-remote))

## Conflict dry-run: `merge-tree`

```sh
git merge-tree --write-tree origin/v2 origin/<b> >/dev/null 2>&1; echo $?
# 0 = clean, 1 = conflicts — no worktree touched, no commits created
```

jj has no equivalent that doesn't write: `jj new A B` creates real commits, and `jj resolve` only works in a checked-out merge. For a read-only "would these two remote refs conflict", stay with git. ([git-merge-tree](https://git-scm.com/docs/git-merge-tree))

## Patch-equivalence: `cherry` and `patch-id`

```sh
git cherry origin/v2 origin/<branch>            # '-' = patch already in base, '+' = unique
git show $sha | git patch-id --stable | cut -d' ' -f1     # then grep that id over candidate commits:
git log origin/v2 | head -400 | while read -r _ sha _; do git show $sha | git patch-id --stable; done | grep -q "^$pid"
```

([git-cherry](https://git-scm.com/docs/git-cherry), [git-patch-id](https://git-scm.com/docs/git-patch-id))

**Limitations — this is where review agents got burned:**

- Patch-id detects cherry-pick-style landings only. A squash+rename consolidation (`pty-pane-polish` → merged #44971, `persistent-terminal-pane.tsx` → `terminal-pane.tsx`) is invisible to it.
- Corroborate before concluding: file-existence probes (`git cat-file -e origin/v2:<path>`), `git grep <rev> -- <path>`, message search (`git log -i --grep` / jj `description(substring-i:)`), per-file diff against the sibling squash commit (`git diff <merged-squash-sha> origin/<b> -- <files>`), and PR closure comments (`gh pr view <n> --json comments --jq '.comments[].body'`).
- Never cross-compare `--stat`/line counts between git and jj on the same change — jj's diff algorithm (imara-diff) can split identical hunks differently (verified: same 79 files, `6329(+)/37(-)` vs `6332/40`; per-file matched 78/79).

## Exact per-file counts: `--numstat`

jj has no numstat; `jj diff --stat` gives per-file +/- bars whose totals can drift from git's (above). When the number must be exact and attributable per file, `git diff --numstat` is canonical.

## Match extraction at a revision

```sh
git grep -o '"v2\.[a-z]*\.[a-zA-Z]*"' origin/<b> -- packages/sdk/openapi.json | sort -u
git grep -c '<marker>' origin/v2 -- <path>       # counts inside a remote ref
# jj near-equivalent (containment only):
jj file search -r <b>@origin --pattern '<pat>' <path>    # prints matching FILE paths, not the matches; rc=0 on no-match
jj file show -r <b>@origin <path> | grep -o '<pat>' | sort -u    # extraction via pipe — byte-identical content
```

`jj file search` answers "which files contain a match", and its rc is 0 even on no-match (warning on stderr only) — don't script it as a boolean.

## Cross-references

- [`vcs/jj/README.md`](/vcs/jj/README.md) — everything that *does* have a jj pairing: inventory, merged-ness, reading without a checkout, revsets/templates.
- [`vcs/jj/rewrites.md`](/vcs/jj/rewrites.md) — the one place git is also canonical inside jj work: pinning committer dates via colocation.
- [`code/reconstruction/experiments0.glm53h.md`](/code/reconstruction/experiments0.glm53h.md) — the git-colocation escape hatch measurements.
- [jj git command table](https://jj-vcs.github.io/jj/latest/git-command-table/) — upstream's own git↔jj mapping; useful for spotting what moved between versions.
