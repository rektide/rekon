---
id: jj
order: 220
source: /vcs/jj/README.md
status: draft
sources:
  - resource: "opencode://ses_ec375b1d6ffergulkiIGGtQXA8"
    title: Added compact jj-waltz creation and reseed examples with and without groups
  - resource: /vcs/jj/waltz.md
    title: Active jj-waltz usage and reseeding guide
---

# jj and git operating rules

- Never use `jj edit`. Never squash unless explicitly instructed; humans review
  changes before squash. Do not push unless explicitly directed by the operator.
- Commit explicit owned paths with `jj commit -m "<description>" <files...>`;
  unrelated changes remain in the working copy. Inspect unfamiliar starting
  changes; after that assume sole ownership unless told otherwise. No routine
  `jj status` before every commit. The new empty working-copy commit is expected.
- Titles describe the work, not ticket IDs or temporary "phases". Put relevant
  ticket IDs in the commit message body. Use `jj diff --git` or `jj diff --stat`;
  plain diff is misleading without color.
- jj has its own DSL: consult `jj help -k revsets` and `jj help -k templates`
  before composing `-r` / `-T`. Ancestors `::@`, descendants `@::`, range `x..y`;
  no `::~`. Fields are bare (`change_id.short()`), not `commit.change_id`;
  concatenate strings with `++`.
- Name dated bookmarks `<feature>-<YYYYMMDD>` using the local date the work was
  last active (recover with `jj op log`), not automatically today's date.
- Working-copy author dates come from creation and can become stale. Read
  [date surgery](/vcs/jj/rewrites.md#backdating-a-fresh-commit-stale-working-copy-dates)
  before deliberate correction through colocated git; jj itself cannot set them.
- For hunks, use full IDs and a dry-run as described in the code fragment.
  Wholesale rewrites do not hunk-split into clean intermediates: preserve the
  rewritten version, restore the parent version, apply/commit the minimal change,
  then restore/commit the full rewrite. Load [jj-hunk](/code/jj-hunk.md).
- [The jj guide](/vcs/jj/README.md) owns command recipes and forensics.

## jj-waltz workspaces

`jw` owns workspace lifecycle; `jj` owns graph/fetch/bookmarks. [Usage guide](/vcs/jj/waltz.md):

```sh
eval "$(jw shell init zsh)"  # enables cd; add creates, switch creates/switches
jw add worker-a worker-b --at 'dev@origin' --cow --no-colocate
jw s worker-a               # ^ default, - previous; list/status/doctor inspect
jw reseed worker-a worker-b --from /path/to/prepared-source --source-at 'dev@origin' \
  --copy-ignored node_modules --at 'dev@origin' --dry-run
jw reseed --group workers --at 'dev@origin' --dry-run # optional local config
```

Reseed keeps roots; omit `--at` to keep feature revisions. Remove `--dry-run` to
refill; pause writers. Prepare source with JJ/install first. [Groups](/vcs/jj/waltz.md#jj-waltz-groups)
are optional literal-name lists in default-workspace `.jwreseed.local.toml`;
named use needs none. `--copy-ignored` replaces selected roots with private CoW
copies; links share symlinks. `--replace-content` deletes other unrecorded contents
(including `.env`) except `.jj`. Pending refill: `jw reseed NAME --resume` before
snapshotting. [Preparation, restrictions, recovery](/vcs/jj/waltz.md#jj-waltz-source).
