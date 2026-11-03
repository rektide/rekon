---
type: Reference
title: Known places — workspace directory
description: Maintained local directory map distinguishing the global reference space, Rekon, source archives, project documentation and filesystem aliases.
resource: /code/workspace/README.md
status: active
generated: { by: "model:openai/gpt-6-astra#high", at: 2026-11-03 }
sources:
  - { resource: "opencode://ses_e614482c2ffeXMJXi14xoFW33A", title: "Human chooses ~/r for the global reference space and commissions relocation, alias updates and a GLOBAL directory" }
  - { resource: /code/workspace/directories0.gpt61t.md, title: Terra's directory-index proposal and explicit home-path notation }
  - { resource: /code/workspace/GLOBAL.md, title: Ambient known-places directory and workspace practice }
  - { resource: "file:///home/rektide/r/README.md", title: Global reference-space index }
---

# Known places

The operator's **global reference space is `~/r/`**: broad-topic knowledge,
research notes and an evolving personal wiki. **rk means Rekon**, the separate
project at `~/src/rekon/` that maintains shared operating context and tools.
`rk` here names the project; it does not assert a `~/rk/` filesystem alias.

This map owns local directory meaning and navigation. Its
[GLOBAL exposure](/code/workspace/GLOBAL.md#known-places) puts the common rows
in ambient context. Project documents still own their procedures.

## Path notation

- Write `~/ado/patches.md` or `~/r/opencode/patches.md`, not an implicitly
  home-relative `ado/patches.md`. `~` means the current user's home in shell
  notation; this directory describes `/home/rektide` on the operator's machine.
- A bare `doc/` is relative to the named project or current directory.
  `~/doc/`, `~/r/` and project-local `doc/` are distinct locations.
- Markdown does not expand `~`: use explicit `file:///home/rektide/…` targets
  for local navigation. Rekon's `/…` document links are bundle-root-relative;
  portable external citations should use repository URLs where available.

## Directory

Local targets verified 2026-11-03. Aliases can change; the table records their
current meaning rather than reserving paths on other machines.

| Reference | Canonical location / resolution | Purpose |
| --- | --- | --- |
| [`~/r/`](file:///home/rektide/r/README.md) | `/home/rektide/r/` — real directory | Global reference/wiki space. |
| [`~/ado/`](file:///home/rektide/r/opencode/README.md) | `~/r/opencode/` | OpenCode knowledge; [patches and ideas](file:///home/rektide/r/opencode/patches.md). |
| `~/ad/`, `~/src/doc/`, `~/archive/doc/`, `~/src/d/`, `~/archive/d/` | Symlinks to `~/r/` | Preserved reference-space aliases; new references use `~/r/`. |
| `~/a/` | `~/archive/` | Source archive; `<org>/<repo>` is the usual lookup shape. |
| `~/a/a/opencode/` | `~/archive/anomalyco/opencode/` | OpenCode primary donor; preparation state belongs to its maintenance records. |
| `~/src/` | `/home/rektide/src/` | First-party development checkouts and variants. |
| `~/w/`, `~/src/working/`, `~/oc/working/` | `~/src/opencode-working/` | OpenCode compose workspace; lifecycle belongs to the patches record. |
| `~/wiki/` | Via `~/deepwiki.com` to `~/deepwiki/` | Generated architectural notes, distinct from the operator-authored reference space. |
| `~/doc/` | `~/Documents/` | Existing personal Documents alias. |
| `~/ff/` | `~/src/future-fuze/` | Kubernetes resources. |
| `/opt/`, `/srv/` | Absolute host paths | Installed software and service instances. |
| `/tmp/opencode/` | Absolute host path | Harness-approved temporary files; ephemeral evidence. |

## Reference-space relocation — 2026-11-03

The human selected `~/r/` after considering `~/rek/` and its resemblance to
Rekon. The real `~/archive/doc/` directory was renamed to `~/r/` on the same
filesystem, preserving its directory inode, contents, Git repository and jj
workspace. Both `jj workspace root` and `git rev-parse --show-toplevel` then
reported `/home/rektide/r`.

The old `~/archive/doc/` path is now a compatibility symlink. The existing
`~/src/doc/`, `~/src/d/`, `~/archive/d/` and `~/ad/` links point directly at
`~/r/`; `~/ado/` points directly at `~/r/opencode/`. Existing absolute citations
through the old directory still resolve. `~/doc/` continues to mean Documents.

The [earlier proposal](directories0.gpt61t.md) explains the ambiguity that led
here; its pre-move path observations remain historical evidence. The reference
space [links back to this directory](file:///home/rektide/r/README.md).
