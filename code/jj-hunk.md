---
type: Reference
title: "jj-hunk — programmatic hunk selection for jj"
description: Subcommand reference, spec shape, and discovery→verify→commit workflow for jj-hunk; when to use it instead of plain jj commit or interactive split; the full-ID requirement and the wholesale-rewrite gotcha.
resource: /code/jj-hunk.md
tags: [code, jj, jj-hunk, commits, hunks, splitting]
status: stable
generated: { by: model:zai-coding-plan/glm-5.3-max, at: 2026-09-04 }
stale_after: 2027-03-04
sources:
  - id: agents-operating-manual
    resource: /AGENTS.md
    title: "The jj-hunk section of the workspace operating manual (human-codified, evolved since 2026-02)"
    author: human:rektide
    last_modified: 2026-09-04
  - id: unmixing-flow
    resource: /vcs/jj/rewrites.md
    title: "Verified hunk-level unmixing flow for shared/mixed files (paste-tested 2026-09-03)"
  - id: promotion-spotcheck
    resource: /vcs/jj/README.md
    title: "jj-hunk list → dry-run → single-hunk commit flow re-spot-checked at the vcs/jj promotion (jj 0.40.0)"
    last_modified: 2026-09-03
  - id: patches-mention
    resource: file:///home/rektide/archive/doc/opencode/patches.md
    title: "First written mention: unmix docs/mixed commits with jj split or jj-hunk"
---

<a id="jj-hunk"></a>

# jj-hunk — programmatic hunk selection for jj

`jj-hunk` selects **individual hunks** of a diff and commits, splits, or
squashes only those. It is the tool for the case plain `jj commit <paths>`
cannot express: logical changes that are intermingled *within* files.

`jj commit -m "<msg>" <files...>` keeps the named files in the described
commit and moves everything else to a new working-copy commit on top. When
the file boundaries already align with the logical change, that is the
simpler tool — prefer it. `jj-hunk` is for when they don't.

<a id="jj-hunk-when"></a>

## When to use which

| Tool | Use when |
| --- | --- |
| `jj commit -m "<msg>" <files>` | File boundaries align with the logical commit (the common case — try this first). |
| `jj-hunk` | Logical changes intermingle **inside** files, or you need clean commits without a TTY (agents, CI). |
| `jj split -p` / `jj squash -i` | Same job as jj-hunk, but interactive — only when a TTY is available. |

<a id="jj-hunk-subcommands"></a>

## Subcommands

Run `jj-hunk <sub> --help` to confirm flags; the shape as of this writing:

| Subcommand | Purpose |
| --- | --- |
| `jj-hunk list` | Emit JSON of all hunks in the current `@` change (`-r <rev>` for another). Each hunk has an `id` (`hunk-<sha256>`) — this is how you discover IDs. |
| `--format text` | Compact human-readable list (file, hunk index, type, ID, added/removed lines) — for eyeballing instead of parsing JSON. |
| `--files` | Just file + hunk counts, no content. |
| `--spec '<spec>'` | **Dry-run**: preview which hunks a spec would select. Returns `{"files": []}` if nothing matches. Always run this before committing. |
| `--spec-template` | Print a starter spec populated with every hunk ID; delete the ones you don't want and pipe it back in. |
| `-i '<glob>'` / `-x '<glob>'` | Include/exclude file patterns (repeatable) to filter noisy diffs. |
| `jj-hunk commit [SPEC] [MESSAGE]` | Commit the selected hunks. SPEC is a JSON/YAML string (inline works for a hunk or two), `-` for stdin, or `--spec-file <path>`. |
| `jj-hunk split [SPEC] [MESSAGE]` | Split current `@` into two commits; selected hunks go into the new child. |
| `jj-hunk squash [SPEC]` | Squash the selected hunks into the parent. |

<a id="jj-hunk-spec"></a>

## Spec shape

Same shape for commit, split, and squash — a map of files to hunk IDs:

```json
{"files": {"<path>": {"ids": ["hunk-<sha256>", ...]}}}
```

<a id="jj-hunk-ids"></a>

## Full IDs are critical

Use the **full `hunk-<sha256>` ID.** Short prefixes **silently match
nothing**, and `jj-hunk commit` happily creates an empty commit with no
error. Guard against this with the dry-run: `jj-hunk list --spec '<spec>'`
must list exactly the hunks you expect before you commit.

Extracting IDs:

```sh
# just the IDs
jj-hunk list | jq -r '.files[].hunks[].id'
# a specific file's IDs
jq -r '.files[] | select(.path=="<path>") | .hunks[].id' 
# find a hunk by content — added/removed are plain strings, not arrays
jj-hunk list | jq -r '.files[].hunks[] | select(.added | contains("<marker>")) | .id'
```

<a id="jj-hunk-workflow"></a>

## Discovery + commit workflow

1. `jj-hunk list --format text` — eyeball hunks and their IDs (or
   `jj-hunk list > /tmp/hunks.json` and match on `added`/`removed`/`context.pre`).
2. Hand-write the spec with full IDs.
3. **Verify**: `jj-hunk list --spec '<spec>'` — confirm it selects what you
   expect. This is the step that catches mismatched/truncated IDs.
4. Commit: `jj-hunk commit '<spec>' "<message>"` (inline), via stdin
   (`jj-hunk commit - "<message>" < spec.json`), or
   `jj-hunk commit --spec-file <path> "<message>"`.

Unselected hunks stay dirty in the working-copy commit, ready for their own
commit — which is exactly the "list, dry-run, commit one hunk, leave the
rest" loop for isolating one logical change out of a dirty file.

<a id="jj-hunk-shared-files"></a>

## Worked flow: unmixing hunks from a shared file

Docs and code changes inside **one file** (doc comments in source, a README
hunk inside a code commit), or a dirty file carrying edits from two
sessions: file-level tools are not enough. Pull the mixed state into a fresh
working-copy commit and select at the hunk level:

```sh
jj new <parent-of-mixed>          # fresh commit to receive the doc hunks
jj restore --from <mixed> -- <shared-file>
jj-hunk list --format text          # discover hunk ids — full ids only
jj-hunk list --spec '<spec-json>'   # DRY-RUN: confirm the selection
jj-hunk commit '<spec-json>' "docs: <subject>"
jj commit -m "feat: <subject>"      # the remainder
```

Then `jj rebase` the pair into place, or use them as the replay source (the
[task-reconstruction procedure](/code/task-reconstruction/README.md) consumes
exactly this). The three-situation unmixing decision table — two-step
commit-time, split-in-place, hunk-level — lives in
[`/vcs/jj/rewrites.md`](/vcs/jj/rewrites.md).

<a id="jj-hunk-rewrite-gotcha"></a>

## Gotcha: wholesale-rewritten files

When a file has been **wholesale rewritten**, hunk-based splitting will not
produce clean intermediate states: hunks are a diff against the parent, and
a rewrite's hunks don't compose into the intended intermediate. Instead:

1. Restore the file from the parent revision:
   `jj file show -r @- <file> > <file>`
2. Apply the minimal change by hand, commit.
3. Write the full version back, commit again.

<a id="jj-hunk-cross-references"></a>

## Cross-references

- [`vcs/jj/README.md`](/vcs/jj/README.md) — the jj+git knowledge home:
  reading history, revset/template cheats, traps. jj-hunk is the
  writing-history counterpart.
- [`vcs/jj/rewrites.md`](/vcs/jj/rewrites.md) — unmixing docs from code
  (the three-situation table), split mechanics, timestamp preservation
  across rewrites.
- [`code/task-reconstruction/README.md`](/code/task-reconstruction/README.md)
  — the replay procedure that uses hunk-level selection for shared-file
  mixed commits.
- [`/AGENTS.md`](/AGENTS.md) — the operating-manual section this doc
  calved from; the manual now points here for depth.
