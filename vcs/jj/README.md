---
type: Reference
title: "jj + git working guide — spelunking, surgery, and when to stay with git"
description: Job-organized reference for interrogating branches, PRs, and merge state in jj+git (colocated) repos, with pointers to the rewrite-surgery, op-log forensics, and git-only breakouts.
resource: /vcs/jj/README.md
tags: [vcs, jj, git, github-cli, forensics]
status: stable
generated: { by: model:zai-coding-plan/glm-5.3-max, at: 2026-09-03 }
verified: { by: inherited — jj/git pairs paste-tested 1:1 against git (jj 0.40.0) 2026-08-29/30 and rewrite/op-log behavior measured 2026-09-03, see sources; core claims re-spot-checked in a scratch repo at promotion (jj 0.40.0), see Provenance, at: 2026-09-03 }
stale_after: 2027-03-03
sources:
  - id: jj-cheatsheet
    resource: file:///home/rektide/archive/doc/jj.md
    title: "jj/git branch spelunking cheat sheet — the document this set promotes"
    author: glm-5.3 orchestrator + verification subagent (opencode)
    last_modified: 2026-08-30
  - id: experiments0
    resource: /code/reconstruction/experiments0.glm53h.md
    title: "jj date preservation and split mechanics — measured on jj 0.40.0"
    author: model:zai-coding-plan/glm-5.3
    last_modified: 2026-09-03
  - id: reconstruction-readme
    resource: /code/reconstruction/README.md
    title: "Reconstruction with docs preservation — the applied procedure that consumed this knowledge"
    author: model:zai-coding-plan/glm-5.3
    last_modified: 2026-09-03
  - id: rekon-agents
    resource: /AGENTS.md
    title: "rekon house conventions (OKF frontmatter, doc style, jj-hunk)"
    author: human:rektide
---

# jj + git working guide

This set promotes the [jj/git branch spelunking cheat sheet](file:///home/rektide/archive/doc/jj.md) — born from a 50-branch review of `anomalyco/opencode` (2026-08-29) — into a small knowledge home for jj+git work, folding in the measured rewrite and op-log behavior from the [reconstruction experiments](/code/reconstruction/experiments0.glm53h.md) (2026-09-03). Everything here was verified on **jj 0.40.0**; both sources paste-tested their commands against live repos, and the core claims were re-spot-checked at promotion time ([Provenance](#provenance--the-verification-story)).

The organization is by **job to be done**. Command blocks pair git with its jj equivalent (`# jj:` comments) wherever the pairing was actually tested; where jj has no honest equivalent, that is stated and the git command stays canonical. House rule from both sources: jj has its own DSL for revsets and templates — consult `jj help -k revsets` / `jj help -k templates` rather than guessing syntax.

## Orientation: what jj is good at, what needs git

**Headline lesson** (from the source review): in a jj+git *collocated* clone, remote-ref forensics is plain git — all five review agents used **zero jj commands**, on purpose, and never missed them. jj's DSL earns its keep for bookmark inventory, revset set-logic, and working-copy questions; git against `origin/*` refs is safer and more composable for read-only branch investigation.

| reach for jj | stay with git |
| --- | --- |
| bookmark inventory and name-glob filtering | live remote heads over the network — `git ls-remote` |
| revset set-logic: ancestry, ranges, description/author/file predicates in one expression | patch-equivalence / cherry-pick detection — `git cherry`, `git patch-id` |
| reading any commit or file without a checkout (`jj file show/list`, `jj log -r`) | conflict dry-runs that create nothing — `git merge-tree --write-tree` |
| history surgery with stable change ids (`duplicate`, `split`, `rebase`) — author dates survive | exact per-file +/- counts — `git diff --numstat` |
| op-log forensics: what operation moved what, when (`jj op log`, `--at-op`) | match extraction at a revision — `git grep -o/-c <rev> -- <path>` |
| author-date-preserving replay/reconstruction | pinning committer dates — git-colocation escape hatch, see [rewrites](/vcs/jj/rewrites.md) |

The right column is owned by [`git-only.md`](/vcs/jj/git-only.md); the surgery row by [`rewrites.md`](/vcs/jj/rewrites.md).

## Using this set

| file | open when… |
| --- | --- |
| this README | you are interrogating an existing repo: bookmarks, merged-ness, contents, PRs |
| [`rewrites.md`](/vcs/jj/rewrites.md) | you are changing history: timestamps, split mechanics, bookmark forwarding, date pinning, unmixing docs from code |
| [`oplog-forensics.md`](/vcs/jj/oplog-forensics.md) | something moved and you need to know what and when; recovering a dragged bookmark |
| [`git-only.md`](/vcs/jj/git-only.md) | the job has no jj equivalent: live remotes, conflict dry-runs, patch-ids, exact counts |

`SKILLS.md` is a symlink to this file so the set can be consumed as a skill.

## Inventory & topology

```sh
jj bookmark list --all-remotes 'browser*'   # jj: name-glob filter (legacy --all still works); reads LAST-FETCHED refs
git ls-remote --heads origin 'browser*'     # AUTHORITATIVE live remote heads (network) — jj has NO equivalent probe
jj git remote list                          # jj: ≡ git remote -v
git remote -v                               # remotes from git's side
```

- `jj bookmark list` filters by bookmark NAME (glob is the default; `substring:`/`exact:`/`regex:` kinds, `-i` suffix for case-insensitive). To use bookmarks as a *revset*: `jj log -r 'remote_bookmarks(glob:"browser*")'`.
- jj resolves symbols as `name`, `name@remote`, or commit/change id. An unknown symbol exits 1 and prints a **did-you-mean hint** — useful discovery in itself (it is how `main@origin` was shown not to exist):

  ```
  Error: Revision `browser-v3` doesn't exist
  Hint: Did you mean `browser-v2`?
  ```

- **Local bookmark ≠ remote head**: in the source review the local `dev` bookmark was 6 months stale while `origin/dev` moved daily. Always date both before narrating (see [Stacks, freshness, conflicts](#stacks-freshness-conflicts)).

## Is it merged? — four layered signals

PRs that land via squash/rebase break ancestry, so no single check suffices. Cheap → expensive:

```sh
# 1. Ancestry (did the branch's commits themselves land?)
git merge-base --is-ancestor origin/<branch> origin/v2 && echo MERGED
jj log -r '<branch>@origin & ::v2@origin' --no-graph -T '"MERGED\n"'   # jj: non-empty output = ancestor
# (single-tip operand needs no heads(); verified: prints MERGED / silence on both sides of the test)

# 2. Patch-id (did equivalent patches land, e.g. cherry-picks?)
git cherry origin/v2 origin/<branch>            # '-' = patch in base, '+' = unique
git show $sha | git patch-id --stable | cut -d' ' -f1     # then grep that id over candidate v2 commits:
git log origin/v2 | head -400 | while read -r _ sha _; do git show $sha | git patch-id --stable; done | grep -q "^$pid"
# jj: NO equivalent to cherry/patch-id — stay with git for patch-equivalence

# 3. Content (does the branch's key file/change exist on the target at all?)
git cat-file -e origin/v2:<path> && echo EXISTS || echo MISSING
jj file show -r v2@origin <path> >/dev/null 2>&1 && echo EXISTS || echo MISSING   # jj: rc=1 "No such path" (verified)
git grep -c '<marker>' origin/v2 -- <path>       # grep INSIDE a remote ref
jj file show -r v2@origin <path> | grep -c '<marker>'    # jj: byte-identical content, pipe to grep (verified 291=291)

# 4. GitHub PR state (authoritative for squash-merge repos)
gh pr list -R <org>/<repo> --head <branch> --state all --json number,state,baseRefName,mergedAt,title
```

- jj exit-code semantics differ from git instincts here — see [Exit codes & silence](#exit-codes--silence-jj-vs-git-instincts).
- Patch-id equivalence detects cherry-pick-style landings only. A squash+rename consolidation (`pty-pane-polish` → merged #44971) is invisible to it — corroborate with file-existence probes, `--grep`, and PR closure comments. Deep dive: [`git-only.md`](/vcs/jj/git-only.md).
- A `git rev-parse origin/<base>` failure can *be* the answer: the base branch was deleted after its PR merged. Interpret, don't retry.

## Reading without a checkout

### Two-dot vs three-dot is load-bearing

`origin/v2..b` **lists commits** (exactly what the branch adds); `origin/v2...b` **diffs from the merge-base** (branch-side work only). Using two-dot for the *diff* drags in unrelated v2-tip drift; using tip-vs-tip (`git diff origin/A origin/B`) across divergent baselines produced a fake "1390 files, ±120k lines" catastrophe in the source review.

```sh
git log --format='%h %ad %s' --date=short origin/v2..origin/<b>   # two-dot: exactly what the branch adds
jj log -r 'v2@origin..<b>@origin' --no-graph \
  -T 'commit_id.short() ++ " " ++ author.timestamp().format("%Y-%m-%d") ++ " " ++ description.first_line() ++ "\n"'
git diff origin/v2...origin/<b> --stat                            # THREE-dot: vs merge-base (branch-side work)
jj diff --from 'heads(::v2@origin & ::<b>@origin)' --to <b>@origin --stat    # jj merge-base idiom (verified same base sha)
git diff origin/v2...origin/<b> -- <path1> <path2>                # file-scoped deep read
jj diff --from 'heads(::v2@origin & ::<b>@origin)' --to <b>@origin -- <path1> <path2>
```

jj's `--from/--to` is tip-vs-tip (two-dot style) — the **three-dot equivalent is `--from 'heads(::A & ::B)'`**, which `git merge-base` confirmed to the exact commit sha (re-verified at promotion). There is no `merge_base()` revset function and no `...` diff operator.

### Path-scoped history

```sh
git log [<range>] -- <path>                     # path-scoped history
jj log -r '<range> & files(<path>)' --no-graph -T 'commit_id.short() ++ " " ++ description.first_line() ++ "\n"'
```

- Set-identical to git (verified 26=26 commits on a file, 375=375 on a directory; ~0.5 s over all of `::v2@origin`). `files()` takes a **fileset**: `files("dir")` matches the dir recursively, `files(glob:"dir/*.ts")` one level. Reference: [fileset language](https://jj-vcs.github.io/jj/latest/filesets/).
- The positional form `jj log -r <range> -- <path>` filters commits too — zero matches prints `Warning: No matching entries for paths: <path>` on **stderr** with rc 0 and empty stdout (verified at promotion). That warning means *zero commits matched*, not "paths unsupported" — see trap 13.

### Files at a revision

```sh
git show --stat <sha>;  git show <sha> -- <path>                  # commit-scoped; --stat=250 avoids path truncation
jj show -r <rev> --stat                                            # jj: but `jj show` takes NO fileset args —
jj diff -r <rev> -- <path>                                         # ...use `jj diff -r` for `git show <sha> -- <path>`
git show origin/<b>:<path>                                        # final file state at branch tip, no worktree
jj file show -r <b>@origin <path>                                 # jj: byte-identical (verified via diff)
git show origin/<b>:<path> | sed -n '/functionName/,+30p'         # extract one function from a big blob
jj file show -r <b>@origin <path> | sed -n '/functionName/,+30p'  # jj: same pipe composes fine
git ls-tree origin/<b> --name-only | rg -i <pat> || echo absent   # tree inventory; make emptiness explicit
jj file list -r <b>@origin | rg -i <pat> || echo absent           # jj: ≡ ls-tree -r --name-only (verified 6707=6707)
git grep -o '"v2\.[a-z]*\.[a-zA-Z]*"' origin/<b> -- packages/sdk/openapi.json | sort -u
jj file search -r <b>@origin --pattern 'v2\.[a-z]*\.[a-zA-Z]*' packages/<path>   # jj: CONTAINMENT only — prints
                                                                   # matching FILE paths, not the matches; extract
                                                                   # via `jj file show | grep -o ... | sort -u`
```

- **jj and git `--stat` totals can disagree slightly**: same 79 files, but git said `6329(+)/37(-)` vs jj `6332/40` — one file with repeated text blocks aligned differently by jj's diff algorithm, same hunk region, different +/- split. Never cross-compare exact line counts between the two tools; per-file counts matched for 78/79.
- **Diff causality inverts on stale bases**: when v2 moved past the branch's base, the `-` lines are v2's *newer* code and `+` is the older branch. Date the path first: `git log --format='%h %ad %s' --date=short origin/v2 -3 -- <path>`.
- For stacked PRs, diff against the parent branch, not v2: `git log origin/plugin-inspection..origin/tui-plugin-apply`.
- PR JSON `files[]` are creation-time snapshots — the branch tip may no longer contain them. `git diff v2...b --stat` is current truth.

## Stacks, freshness, conflicts

```sh
git merge-base --is-ancestor origin/A origin/B && echo stacked    # true stack vs merely-similar branches
jj log -r 'A@origin & ::B@origin' --no-graph -T '"STACKED\n"'     # jj: non-empty = ancestor (verified both ways)
git merge-base origin/v2 origin/<b> | xargs git log -1 --format='%h %ad %s' --date=short   # base age
jj log -r 'heads(::v2@origin & ::<b>@origin)' --no-graph \
  -T 'commit_id.short() ++ " " ++ author.timestamp().format("%Y-%m-%d") ++ " " ++ description.first_line() ++ "\n"'
git merge-tree --write-tree origin/v2 origin/<b> >/dev/null 2>&1; echo $?   # 0=clean 1=conflicts, no worktree touched
# jj: NO conflict dry-run for two remote commits — `jj new A B` would create real commits. Stay with git.
git log -1 --format='%ci' origin/<b>; git log -1 --format='%ci' origin/v2   # staleness heuristic
jj log -r <b>@origin --no-graph -T 'committer.timestamp().format("%Y-%m-%d %H:%M:%S %z") ++ "\n"'
# ^ verified byte-equal to %ci; `author.timestamp()` ≡ %ai (exact). %ad/--date=short ≡ author.timestamp().format("%Y-%m-%d")
```

## The date guarantee (author dates)

**jj preserves author dates across `duplicate`, `split`, and `rebase` — natively, zero extra machinery.** Author date is what `git log` and GitHub display, so for replay/reconstruction work this is the load-bearing guarantee: replayed lines keep the dates that say when the work was actually written. Measured in [experiments0](/code/reconstruction/experiments0.glm53h.md); re-spot-checked at promotion (a duplicate and both halves of a split kept the original author timestamp).

One-line verification — run it over any replayed line before bookmarking it:

```sh
jj log -r 'main@origin..<tip>' --no-graph \
  -T 'author.timestamp() ++ " " ++ description.first_line() ++ "\n"'
```

Committer dates reset to now on every rewrite, and jj 0.40 cannot *set* either timestamp — that material, including the git-colocation recipe for pinning dates when committer timestamps genuinely matter, is kept as an aside in [`rewrites.md`](/vcs/jj/rewrites.md).

## PR & issue forensics (gh)

```sh
gh pr view <n> -R <org>/<repo> --json state,closedAt,comments,reviews \
  --jq '{state, closedAt, n: (.comments|length), last: (.comments[-1].body // "none")}'
gh pr view <n> --json comments --jq '.comments[].body'        # closure rationale often lives ONLY here
gh api repos/<org>/<repo>/issues/<n>/events \
  --jq '.[] | select(.event=="closed" or .event=="reopened")'  # WHO closed and WHEN; PRs are issues
gh pr list -R <org>/<repo> --state all --search "topic in:title" --json number,title,state   # successor hunt
gh pr list -R <org>/<repo> --state all --author <login> --limit 15 --json ...                # author recency scan
gh issue view <n> --json title,state                          # resolve "Refs #n"/"Closes #n"
```

- `--json comments` returning **empty means zero comments, not an error** — disambiguate with `(.comments|length)` before concluding "silent closure". Reviews and timeline events are separate surfaces; escalate to the issues/events API.
- `gh --json` validates field names and the error lists the valid set — read it instead of guessing (`stateReason` isn't one).
- Branch name ≠ PR 1:1: `plugin-updates` carried a merged PR and later a closed one. List *all* PRs per head branch first.
- Exact closure timestamps from the events API exposed a **batch closure** (four PRs closed within seconds = deliberate series withdrawal) that no per-PR view shows.
- PR author ≠ head-commit author (bot-authored commits are common). Report both: `git log -1 --format='%an %ad' --date=short origin/<b>`.

## Supersession hunting (closed-but-maybe-landed)

```sh
git log origin/v2 --oneline -i --grep=<keyword> [--all] -- <path>   # equivalent work under other PR/titles
jj log -r '::v2@origin & description(substring-i:"<keyword>")' --no-graph \
  -T 'commit_id.short() ++ " " ++ description.first_line() ++ "\n"'   # verified 360=360, identical ordering
jj log -r '::v2@origin & description(substring-i:"<keyword>") & files(<path>)'   # + path filter, all in the revset
# `--all` ≈ widening the revset (e.g. `all()` / `::`).
git show $branchSha | git patch-id --stable                          # cherry-pick detection (see git-only.md)
git diff <merged-squash-sha> origin/<b> -- <files>                   # per-file diff vs the sibling squash commit
jj diff --from <merged-squash-sha> --to <b>@origin -- <files>        # jj: direct revset args, no rev-parse needed
gh pr view <n> --json comments --jq '.comments[].body'               # "Superseded by #X" statements
```

- Keep `--all` on the `--grep` deliberately when hunting: it's what finds superseding commits on other refs.
- Comparing a merged squash's `--stat` against the branch twin's exposes consolidation renames (`persistent-terminal-pane.tsx` → `terminal-pane.tsx`).

## Revset cheat

`::x` ancestors-of, `x..y` = `y & ::-x`, `heads()` to trim to tips, `files(<fileset>)` commits-touching-path (the `git log -- <path>` equivalent — verified set-identical to git), `<name>@<remote>` for remote-tracking symbols, quote weird symbols `'"x-"'`. No `merge_base()` function — the idiom is `heads(::A & ::B)`. String patterns (used by `description()`, `bookmarks()`, `remote_bookmarks()`, `author()`…):

- bare `"x"` is a **glob** — not substring!
- then `exact:"x"`, `substring:"x"`, `regex:"x"`, with `-i` suffix for case-insensitive (`substring-i:"kw"` ≡ `git log -i --grep=kw`)
- beware `description("foo\n")` — **descriptions end with a newline**, so `exact:` rarely matches a subject line; `substring:` beats `exact:` for message search (this trap bit the promoting agent's own verification script — live confirmation).

Reference: [revset language](https://jj-vcs.github.io/jj/latest/revsets/).

## Template cheat

`change_id.short()`, `commit_id.short()`, `description.first_line()`, `author.name()`, `author.timestamp()`, `committer.timestamp()`, `.ago()`, `.format("%Y-%m-%d %H:%M:%S %z")` (strftime; verified byte-equal to git's `%ci`/`%ai`), `bookmarks`, concat with `++`. One `-T` flag per invocation — given twice, the last silently wins. Reference: [templating language](https://jj-vcs.github.io/jj/latest/templates/).

```sh
# counting trick:
jj log -r '<revset>' --no-graph -T '"."' | wc -c
# existence probes (rc distinguishes!):
jj log -r <ref> >/dev/null 2>&1; echo $?   # 1 = symbol unknown (stderr carries did-you-mean), 0 = resolves
jj file show -r <rev> <path> >/dev/null 2>&1; echo $?   # 1 = "No such path", 0 = exists (≡ git cat-file -e)
```

## Exit codes & silence (jj vs git instincts)

- An **empty revset** exits 0 with no output — it cannot be tested with `&&`. A **nonexistent symbol** exits 1 with a did-you-mean hint. Both re-verified at promotion.
- `jj file show -r <rev> <path>` on a missing path: rc 1, `Error: No such path: <path>` (≡ `git cat-file -e` failing).
- `jj file search` no-match **and** bad-pathspec → rc 0 (warning on stderr only).
- Silence is not failure and failure is not loud — echo `$?` and read stderr. `jj log -r <ref> >/dev/null 2>&1; echo $?` is the `rev-parse`-style existence probe.

## Traps (each one bit somebody in the source review)

1. **zsh `echo ===` → `zsh: == not found`** (leading `=` triggers `=cmd` path expansion). Two agents hit it; it can abort a compound command mid-chain and swallow later output. Use `echo "---"` and prefer `&&` chains so failures are loud. Related zsh trap: unquoted `$var` does **not** word-split — a `for` over a space-separated string silently iterates once (it misclassified a docs/code audit until the classifier moved into awk).
2. **Suppressed errors fabricate zeros**: piping a failing revset into `wc -c` under `2>/dev/null` produced a confident-but-false "0 unique commits vs main" (the ref didn't exist). Echo `$?` or verify refs exist before counting.
3. **jj `-T` given twice = last one wins** (and a stray `-T 'head: '` silently vanishes). One template, with `++` concatenation.
4. **Tip-vs-tip diffs across drifted baselines are garbage** — always merge-base (`...`) or range (`base..branch`), and scope to files when either side is stale.
5. **Patch-id can't see squash-merge consolidation** — pair with `git cat-file -e`, `git grep <rev>`, and PR comments.
6. **Empty output is ambiguous** (no match vs bad pathspec vs missing ref): add `|| echo` guards; `git grep` fails silently on wrong pathspecs — broaden before concluding absence.
7. **Cached PR JSON surprises**: `closedAt`/`updatedAt` can be `null` despite `state: CLOSED`; run `jq 'keys'` on an unfamiliar payload before trusting fields. Don't guess GitHub field names (`baseRefName`, not `base`).
8. **Local bookmarks go stale** while `origin/*` moves: date both before claiming a line is dead — "dev is frozen" turned out to be a stale local ref; dev shipped daily.
9. **PR metadata is historical** (files[] snapshots); the branch tip is current truth.
10. **jj exit codes invert grep instincts**: empty revset → rc 0 + no output; `jj file search` no-match AND bad-pathspec → rc 0 (warning on stderr only). See [Exit codes & silence](#exit-codes--silence-jj-vs-git-instincts).
11. **jj/git `--stat` line counts can differ** (imara-diff vs Myers alignment through repeated text blocks; verified 6329/37 vs 6332/40 on identical trees): same files, same patch. Don't cross-compare exact counts between tools.
12. **`jj show` takes no fileset args** (`jj show -r <rev> -- <path>` is a usage error): `git show <sha> -- <path>` → `jj diff -r <rev> -- <path>` (verified). `git log -- <path>` DOES have a jj form — see `files()`.
13. **"Warning: No matching entries for paths" ≠ "paths unsupported"**: positional path args on `jj log` filter commits; that warning means *zero commits matched*, with empty stdout and rc 0. Merging stderr into the pipe (`2>&1 | wc`) counts the 70-char warning as output — which is exactly how a "verified" false "jj can't path-filter" claim slipped into an earlier revision of the source doc. Keep stderr out of counts, and read warnings literally.

## Provenance & the verification story

- **2026-08-29** — the source review: five parallel review agents plus an orchestrating session interrogated 50 bookmarks of `anomalyco/opencode`; afterwards each agent reported exactly which commands they used and which misled them. Full narrative in [`branch-review.glm53.md`](file:///home/rektide/archive/anomalyco/opencode/.design/branch-review/branch-review.glm53.md).
- **2026-08-29/30** — every `# jj:` line in the cheat sheet was paste-tested 1:1 against its git counterpart (jj 0.40.0, collocated clone): identical output or documented divergence.
- **2026-09-03** — rewrite-surgery, timestamp, and op-log behavior measured in scratch repos and the live watchwoman replay ([experiments0](/code/reconstruction/experiments0.glm53h.md)); the applied procedure lives in [`code/reconstruction/README.md`](/code/reconstruction/README.md).
- **2026-09-03 (promotion)** — this reorganization. The promoting agent re-spot-checked core claims in a fresh scratch repo (jj 0.40.0): exit codes (empty revset rc 0 silent; unknown symbol rc 1 + did-you-mean hint; `jj file show` missing path rc 1), the path-filter warning text with rc 0, `files()` counts (file 1, dir glob 3), the `heads(::A & ::B)` merge-base idiom against `git merge-base` (same sha), containment both ways, `duplicate` stderr output + author-preserved/committer-now timestamps, plain-`split` change-id assignment + bookmark forwarding to the remainder, headless split without `-m` failing with "Failed to edit description", the `bookmark set` backwards refusal + `--allow-backwards` acceptance, and the `jj-hunk` list → dry-run → single-hunk commit flow.

**The famous correction.** The original cheat sheet once claimed "jj log can't path-filter". It was wrong — `jj log -r <range> & files(<path>)` and positional `jj log ... -- <path>` both path-filter commits, set-identically to `git log -- <path>` (26=26 on a file, 375=375 on a directory). The false claim survived a "verification" pass because a `2>&1 | wc -c` count read the zero-match warning text as output (trap 13): *the paste-test ran, but its stderr discipline didn't.* Kept here deliberately — it is the doc's own cautionary tale about verification discipline, and it recurs: at promotion time, the `description(exact:)` newline trap bit the verifying agent's first script the same way a documented trap always does, quietly.

## Cross-references

- [`vcs/jj/rewrites.md`](/vcs/jj/rewrites.md) — history surgery: timestamp semantics, split mechanics, bookmark forwarding/divergence, date pinning via git colocation.
- [`vcs/jj/oplog-forensics.md`](/vcs/jj/oplog-forensics.md) — op-log walks with `--at-op`, diagnosing "what moved my bookmark", and recovery.
- [`vcs/jj/git-only.md`](/vcs/jj/git-only.md) — the jobs git keeps canonically, with the no-jj-equivalent table.
- [`code/reconstruction/README.md`](/code/reconstruction/README.md) — the applied replay procedure that consumed this knowledge (snapshot-first, unmix, verify, bookmark).
- [`code/reconstruction/experiments0.glm53h.md`](/code/reconstruction/experiments0.glm53h.md) — the measured jj 0.40.0 behavior backing the rewrite claims.
- [jj-vcs/jj](https://github.com/jj-vcs/jj) — upstream; [docs](https://jj-vcs.github.io/jj/latest/), [revsets](https://jj-vcs.github.io/jj/latest/revsets/), [templates](https://jj-vcs.github.io/jj/latest/templates/), [filesets](https://jj-vcs.github.io/jj/latest/filesets/), [git command table](https://jj-vcs.github.io/jj/latest/git-command-table/).
