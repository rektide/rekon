---
type: Guide
title: "jj-waltz — navigate, create, and reseed workspaces"
description: Everyday jw usage and repeatable in-place CoW refills, with explicit names, optional groups, private ignored dependencies, and recovery.
resource: /vcs/jj/waltz.md
tags: [jj, workspaces, jj-waltz, copy-on-write, reflinks]
status: draft
generated: { by: "model:openai/gpt-6-astra", at: 2026-10-14 }
sources:
  - resource: "opencode://ses_ec375b1d6ffergulkiIGGtQXA8"
    title: Integrated active jj-waltz usage and reseeding into Rekon's JJ home and ambient guidance
  - resource: "file:///home/rektide/src/jj-waltz/README.md"
    title: Active workspace lifecycle, navigation, CoW creation, and configuration guide
  - resource: "file:///home/rektide/src/jj-waltz/docs/reseed.md"
    title: Implemented one-off and batch reseeding, content policy, and recovery
  - resource: "file:///home/rektide/src/jj-waltz/src/cli.rs"
    title: Current command arguments and option conflicts
  - resource: "file:///home/rektide/src/jj-waltz/src/reseed/engine.rs"
    title: Source identity, configuration precedence, selection, planning, and recovery contracts
---

<a id="jj-waltz"></a>

# jj-waltz — navigate, create, and reseed workspaces

**Use `jw` for workspace lifecycle and navigation; use `jj` for revisions,
bookmarks, fetch/push, and graph recovery.** `jw reseed` refreshes CoW sharing in
existing workspaces while retaining their root directories. It can preserve each
feature's current revision or explicitly start fresh children of a chosen base.

This guide describes the active implementation at
[`~/src/jj-waltz`](file:///home/rektide/src/jj-waltz/README.md), inspected on
2026-10-14. Its reseed command is implemented; the archived default checkout's
older proposal wording and a released binary may lag it. Check your binary's
`jw reseed --help` before using these recipes. Verification here is documentation
and source inspection, not execution against production workspaces.

Jump to [navigation](#jj-waltz-navigation), [new CoW workspaces](#jj-waltz-creation),
[source preparation](#jj-waltz-source), [reseeding without groups](#jj-waltz-reseed),
[ignored contents](#jj-waltz-content), [Groups](#jj-waltz-groups), or
[recovery](#jj-waltz-recovery).

<a id="jj-waltz-navigation"></a>

## Navigate and inspect

Initialize shell integration so switching can change the **parent shell's**
directory. Put the appropriate initialization in your shell startup file:

```sh
eval "$(jw shell init zsh)"     # zsh; use bash instead for bash
```

Fish uses `jw shell init fish | source`; Elvish and PowerShell are also supported.
The raw executable cannot change its caller's working directory. Reinitialize
after upgrades that change shell behavior. Generate completions with
`jw shell completions zsh` (or your supported shell).

```sh
jw add frontend tests docs          # create without switching
jw switch frontend tests docs       # create missing ones; switch to the last
jw s frontend                      # short alias for switch
jw ^                               # default workspace
jw -                               # previous workspace
jw switch -x opencode feature-ui    # switch, then launch a command
jw list                            # aliases: l, ls
jw status @                        # explain the current workspace
jw list --format=json --refresh=none
jw doctor                          # consistency and link diagnostics
jw context /path/to/checkout        # inspect Git topology and JJ identity
jw path frontend                   # locate a named workspace
jw current
jw root
```

Switching preserves the current subdirectory when possible. `@`, `-`, and `^`
route to current, previous, and default workspaces in commands supporting those
selectors; they are **not literal reseed target names**. `context` discovers
identity without refreshing a checkout. JSON list/status supports explicit
refresh policy; `--refresh=none` avoids requesting working-copy refreshes.
Bare `jw` normally opens the interactive manager in a terminal; `jw ui` selects
it explicitly. `doctor` reports unhealthy state before exiting nonzero.

<a id="jj-waltz-creation"></a>

## Create a workspace, with or without CoW

```sh
jw add feature-api --at 'dev@origin'
jw add worker-a worker-b --at 'dev@origin' --cow --no-colocate
jw switch worker-c --at 'dev@origin' --cow --no-colocate
```

`--at` resolves one exact creation base. Without it, creation uses the sole
`parents(@)` revision, making a sibling of the current working copy rather than
copying its uncommitted feature work. Multiple parents require an explicit base;
`--at @` intentionally creates from the current revision. Creation flags apply
to newly created workspaces, not to existing workspaces visited by `switch`.

CoW files share storage until one copy changes. `--cow` clones only **tracked**
files from the current workspace and preserves its sparse patterns; ignored and
untracked `.env`, `node_modules`, and caches are excluded. Creation currently
copies private JJ `tree_state` and restores the requested checkout; unsupported
cloning, Watchman, or rejected copied state can fall back to ordinary creation.
Reseeding instead uses public JJ sparse commands, requires working CoW, and does
not need a JJ filesystem-`--adopt` fork.

User configuration lives in `$XDG_CONFIG_HOME/jj-waltz/config.toml`, falling back
to `~/.config/jj-waltz/config.toml`:

```toml
[workspace]
copy_on_write = true
colocate = false
```

`--no-cow` overrides configured CoW. Colocation can otherwise inherit from the
primary checkout when JJ supports it; automatic inheritance chooses JJ-only for
CoW. Explicit colocation and CoW conflict, so `--no-colocate` is useful when a
configured `colocate = true` would interfere. Colocated creation gives Git-aware
tools a linked Git worktree; reseeding currently requires **JJ-only targets**.

Optional `workspace.create_bookmark = true` and
`workspace.bookmark_template = "{workspace}"` create associated bookmarks.
`--bookmark custom-name` overrides this for a single new workspace;
`--no-bookmark` suppresses creation. Batch creation uses the configured template.

<a id="jj-waltz-source"></a>

## Prepare a reusable source

A prepared source is a full, conflict-free JJ workspace in the **same JJ
repository** as its targets. Its tracked tree must match `--source-at` exactly;
its `@` may be an empty child of that baseline. An ordinary colocated source is
allowed: its Git metadata is not copied. Use an absolute workspace-root path.

For the OpenCode pool, the intended source is `~/a/a/opencode` (locally an alias
of `~/archive/anomalyco/opencode`). These are preparation instructions, not a
claim that the checkout is already prepared. Inspect and account for existing
source work before deliberately starting the new child:

```sh
jj -R ~/a/a/opencode status
jj -R ~/a/a/opencode git fetch --remote origin
jj -R ~/a/a/opencode new 'dev@origin'
cd ~/a/a/opencode
bun install --frozen-lockfile
```

The frozen install is project-specific; use the project's dependency procedure.
It prepares ignored dependency trees, while tracked files must still match the
baseline. `jw` neither fetches nor installs. Keep the prepared source unchanged
while a refill or recovery uses it; pause writers in both source and targets.
For a later baseline, fetch, prepare the source, and install deliberately again.

<a id="jj-waltz-reseed"></a>

## Reseed without groups

**No config file or group is required.** Run within the repository and select
existing workspaces by their literal registered names:

```sh
# One feature: keep its snapshotted revision and differences.
jw reseed my-feature --from /home/rektide/a/a/opencode --source-at 'dev@origin'

# Batch of existing workers; preview first, then repeat without --dry-run.
jw reseed ocw1 ocw2 ocw3 --from /home/rektide/a/a/opencode \
  --source-at 'dev@origin' --at 'dev@origin' \
  --copy-ignored node_modules --copy-ignored 'packages/*/node_modules' --dry-run
```

The names are examples: use `jw list` to select actual workspaces. An empty
selection never means all. Reseed does not create missing targets.

| Choice | Result |
|---|---|
| No `--at` | Keep each target's snapshotted working-copy revision, including feature changes. |
| `--at 'dev@origin'` | Start a fresh empty child of that base; an already-empty child at the same base may be reused. |
| `--source-at 'dev@origin'` | Assert the prepared source's tracked baseline; this does **not** reset the target. |
| `--dry-run` | Validate and report a plan without clearing/refilling contents; planning can snapshot working copies. |

Tracked target work is snapshotted before mutation. With `--at`, it remains in
JJ history instead of entering the new child. This is not a rebase or a bookmark
move. Reflinks refill unchanged source paths; JJ materializes the target's
differences. Repeating the operation renews sharing even when content is identical.
The root directory's inode, `.jj`, registration, workspace name, bookmarks, and
lifecycle metadata are retained. This is useful for agents and launchers that
keep stable workspace paths.

<a id="jj-waltz-content"></a>

## Choose what happens to ignored and untracked contents

By default, other ignored/untracked contents remain. Add repeatable selections
to replace particular ignored directories with **private CoW copies**:

```sh
jw reseed my-feature --from /home/rektide/a/a/opencode --source-at 'dev@origin' \
  --copy-ignored node_modules --copy-ignored 'packages/*/node_modules'
```

Quote globs to defer expansion to `jw`. Selected directories must be ignored in
both source and target, with no tracked-file or administrative overlap. Missing
matches are reported as skipped. Selected roots are replaced, removing obsolete
entries inside them; other paths remain. A feature with changed package manifests
may need its own frozen install afterward. Symlinks are copied without traversal;
absolute links back into the source are not independent dependency snapshots.

`--replace-content` authorizes deleting **all other unrecorded target contents,
including `.env`, except `.jj`**, before refilling, without another prompt. It
still preserves the root inode and does not copy every source dotfile. Use it
only when that broader replacement is the intended policy; it is unnecessary
merely to replace selected dependency roots.

### Shared links are a different choice

`.jwlinks.toml` in the default workspace defines shared symlinks:

```toml
[[link]]
source = "data"
target = "../shared-data/data"
required = true
```

`source` is the receiving workspace's path; a relative `target` resolves from
that receiving workspace. `.jwlinks.local.toml` overrides entries by `source`.
Switching applies links unless `--no-links` is used; `jw links apply` applies
them manually. Reseed also validates/applies configured links: do not select the
same path for a private ignored copy and a shared link. Private CoW directories
can diverge independently; symlinks expose the same destination to every user.
`jw doctor` checks link health.

<a id="jj-waltz-groups"></a>

## Groups

Groups are optional named lists of **literal workspace names**, not directories,
revsets, ownership classes, or an implicit selection of every workspace. They
save typing when the same pool is refilled repeatedly.

Put the following machine-local `.jwreseed.local.toml` in the **default workspace**
and keep it ignored using that repository's local ignore arrangement:

```toml
source = "/home/rektide/a/a/opencode"
source_at = "dev@origin"
copy_ignored = ["node_modules", "packages/*/node_modules"]

[groups]
workers = ["ocw1", "ocw2", "ocw3"]
reviewers = ["review-a", "review-b"]
```

The source defaults and group definitions are independent. You may omit the
entire `[groups]` table and still use configured source/dependency defaults:

```sh
jw reseed ocw1                       # config, WITHOUT groups; retain revision
jw reseed ocw1 ocw2 --at 'dev@origin' # explicit named batch, still no group
jw reseed --group workers            # WITH a group; retain each revision
jw reseed --group workers --at 'dev@origin' --dry-run
jw reseed ocw1 --group workers --group reviewers
```

Repeat `--group` or combine it with positional names; duplicates are removed.
Unknown groups and empty selections fail rather than widening the operation.

CLI `--from` and `--source-at` override local configuration. Relative source
paths resolve from the default workspace; absolute paths make this unambiguous.
Repeated CLI `--copy-ignored` flags **replace**, rather than append to, configured
selections. `--no-copy-ignored` disables configured copying for the run:

```sh
jw reseed ocw1 --copy-ignored node_modules # only this selection
jw reseed --group workers --no-copy-ignored
```

`--at` cannot be configured: every reset must request it explicitly. Keep
`--source-at` or `source_at` explicit for a reproducible prepared baseline;
the implementation otherwise uses the source's snapshotted `@`.

<a id="jj-waltz-recovery"></a>

## Boundaries and interrupted-operation recovery

The current implementation requires full, conflict-free checkouts and verifiable
workspace identities. Primary/source targets, colocated or externally owned
targets, nested repositories, mounted children, and unsupported special files
are rejected. CoW unavailability fails before clearing rather than falling back
to ordinary copying. Watchman-enabled mutation is unsupported. Resolve stale
workspaces explicitly with JJ before a new attempt (inspect, then use
`jj workspace update-stale` in the affected workspace when appropriate).

Pause builds, agents, editors, and independent JJ writers during reseeding. The
repository advisory lock serializes only other reseed calls. A dry run is a
planning check, not a promise that concurrent changes cannot invalidate it.

An interrupted operation retains a repository-scoped pending record outside the
replaceable contents. **Do not run normal snapshotting JJ commands in a
half-filled target.** Resume the recorded operation by literal name:

```sh
jw reseed ocw1 --resume
```

Resume uses the frozen plan and validates source, target identity, registration,
and revision. Do not supply groups, source/revision overrides, ignored-copy flags,
or `--replace-content` with it. Changed source/identity stops recovery and retains
the record; restore the frozen source before retrying. Batch output distinguishes
completed, failed, and pending targets and exits nonzero for partial failure.
Tracked refill alone is not completion if ignored copying or link application
failed. Recovery does not use repository-wide `jj undo`.

<a id="jj-waltz-lifecycle"></a>

## Lifecycle metadata and removal

`jw adopt existing-name --base 'dev@origin' --no-bookmark` records management
intent for an existing JJ workspace; it does not adopt a filesystem tree into
JJ, initialize a Git worktree, move revisions, or refresh files. `jw repair`
updates an existing readable lifecycle record with an explicit base and bookmark
association; it likewise does not rewrite graph state.

For deliberate cleanup, `jw remove --keep-bookmark NAME` removes a workspace
while retaining its bookmark; `--delete-bookmark` requests deletion of the
associated bookmark. `--keep-dir` forgets a workspace while preserving files.
Ordinary removal deletes ignored/untracked contents too, beyond what JJ can
recover. `jw prune` concerns already-missing directories, not reseeding. Native
apps retain ownership of checkouts they create: let the app clean them up, then
use `jw reconcile-external NAME` for leftover JJ state where applicable.

<a id="jj-waltz-related"></a>

## Cross-references and evidence

- [JJ working guide](/vcs/jj/README.md) owns graph inspection, bookmarks, and
  revision-language reference; this guide adds workspace lifecycle recipes.
- [Operation-log forensics](/vcs/jj/oplog-forensics.md) helps recover the history
  of graph changes. Reseed's pending-plan recovery is a separate filesystem task.
- [Active jj-waltz README](file:///home/rektide/src/jj-waltz/README.md) owns the
  broader command surface, shell variants, UI, and lifecycle semantics.
- [Active reseed guide](file:///home/rektide/src/jj-waltz/docs/reseed.md) is the
  implementation-owned usage contract. These local WIP references deliberately
  target the active checkout rather than the older archived README.
- [CLI](file:///home/rektide/src/jj-waltz/src/cli.rs) and
  [reseed engine](file:///home/rektide/src/jj-waltz/src/reseed/engine.rs) were
  inspected for argument conflicts, source matching, config precedence, literal
  selections, and recovery. No suitable `jw` binary was available on PATH during
  this documentation pass; production mutations were not run.
