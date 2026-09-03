---
type: Reference
title: "jj op-log forensics — what moved, when"
description: Using jj op log / op show / --at-op to diagnose bookmark and ref movement, with the walked worked case (split -A descendant drag) and the recovery playbook.
resource: /vcs/jj/oplog-forensics.md
tags: [vcs, jj, op-log, forensics, bookmarks, recovery]
status: stable
generated: { by: model:zai-coding-plan/glm-5.3-max, at: 2026-09-03 }
verified: { by: inherited from experiments0 — used for real to diagnose and recover the split -A bookmark drag in the live watchwoman repo (jj 0.40.0, 2026-09-03); not independently re-run at promotion, at: 2026-09-03 }
stale_after: 2027-03-03
sources:
  - id: experiments0
    resource: /code/reconstruction/experiments0.glm53h.md
    title: "jj date preservation and split mechanics — measured on jj 0.40.0 (contains the incident this file generalizes)"
    author: model:zai-coding-plan/glm-5.3
    last_modified: 2026-09-03
  - id: reconstruction-readme
    resource: /code/reconstruction/README.md
    title: "Reconstruction with docs preservation — the run that produced the incident"
    author: model:zai-coding-plan/glm-5.3
    last_modified: 2026-09-03
---

# jj op-log forensics — what moved, when

Open this when the question is **"what happened to this repo?"** — a bookmark moved and you don't know why, a commit you remember is hidden, state looks spliced. jj records every operation with before/after state, which git's reflog only approximates (reflogs track refs; the op log records the *effects of whole operations*, including which commits each one rewrote). Upstream concept: [operation log](https://jj-vcs.github.io/jj/latest/operation-log/).

**Rule one: `jj op show` and `jj --at-op` are the ground truth for "what moved my bookmark" — use them before theorizing.**

## The three commands

```sh
jj op log                       # every operation, newest first: id, time, command line
jj op show <op-id>              # what that operation changed: bookmarks, commits ("Changed commits:"), etc.
jj --at-op <op-id> <jj-command> # run any jj command against the repo AS OF that operation — time travel
```

`jj op log --limit 90 --no-graph -T 'id.short() ++ "\n"'` gives a walkable id list.

## The walk pattern: find the op that moved a bookmark

Walk ops backward; re-evaluate the bookmark's target at each point and print whenever it changed:

```sh
jj op log --limit 90 --no-graph -T 'id.short() ++ "\n"' | tac | while read -r op; do
  CUR=$(jj --at-op "$op" log -r '<bookmark>' --no-graph -T 'commit_id ++ "\n"')
  # ...compare with the previous CUR; on change, print $op and $CUR...
done
# then: jj op show <op-id>   -> "Changed commits:" shows exactly what that op rewrote
```

This is read-only time travel: `--at-op` never writes, it just views the repo at an earlier operation. Keep the walk's stdout clean and do comparisons in the surrounding script (see the stderr/counting traps in [`README.md`](/vcs/jj/README.md)).

## Worked case: the `split -A` descendant drag

The first live replay of the watchwoman docs line used `jj split -r <dup> -A <tip> <docs paths>` for mixed commits. `-A` rebases `<dest>`'s descendants on top of the extracted commit, and `<dest>` was an ancestor of the entire source line — so every split dragged the whole source line, including the `systemd` and dated `systemd-20260903` snapshot bookmarks, onto the docs line, four times over. Symptoms: bookmark targets that made no narrative sense; a `--allow-backwards` refusal when trying to re-point one.

The walk above pinned each bookmark move to a "split commit \<sha\>" op, and `jj op show` exposed the rebase of the bookmarked tip and the working-copy commit. Full narrative in [`experiments0`](/code/reconstruction/experiments0.glm53h.md); the mechanics of why `-A` drags in [`rewrites.md`](/vcs/jj/rewrites.md).

**Damage assessment**: `jj diff --from <old-tip> --to <new-tip>` printed `0 files changed` — same trees, spliced ancestry. Metadata-only damage is the common case for bookmark drags; verify it before panicking.

## Recovery playbook

```sh
jj bookmark set --allow-backwards <bookmark> -r <original-tip>   # restore each dragged bookmark
jj abandon <shadow-rewrites>                                     # clears (divergent) twins
jj new <original-tip>                                            # re-seat the empty working-copy commit
```

- `jj bookmark set` refuses to move a bookmark "backwards or sideways" without `--allow-backwards` (refusal text and hint verified at promotion); restoring a dragged snapshot is the legitimate use.
- Unhiding the original tip can leave a shadow rewrite of the same change visible as **divergent** until the shadow is explicitly abandoned; abandoning the shadow tip hides its whole ancestry.
- After recovery, re-verify the line itself (counts, diff scope, author dates) — in the worked case the extracted commits were never wrong, only the collateral rebases were.

## Related traps

- Two jj commands racing in one shared repo trigger "Concurrent modification detected, resolving automatically". Serialize jj work in a repo — concurrent ops also make the op log harder to read.
- An op-log walk that mixes stderr into stdout (e.g. `2>&1 | wc`) will count error/warning text as data — the same discipline failure that once produced a false "jj can't path-filter" claim (see [`README.md`](/vcs/jj/README.md) trap 13).

## Cross-references

- [`vcs/jj/rewrites.md`](/vcs/jj/rewrites.md) — why rewrites move bookmarks: split mechanics, `-A` semantics, change-id tracking.
- [`vcs/jj/README.md`](/vcs/jj/README.md) — the set's entry point; exit-code and stderr discipline.
- [`code/reconstruction/experiments0.glm53h.md`](/code/reconstruction/experiments0.glm53h.md) — the primary incident record and measurements.
- [`code/reconstruction/README.md`](/code/reconstruction/README.md) — the procedure as fixed after this incident.
- [jj operation log](https://jj-vcs.github.io/jj/latest/operation-log/) — upstream concept page.
