---
id: jj
order: 220
source: /vcs/jj/README.md
status: draft
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
