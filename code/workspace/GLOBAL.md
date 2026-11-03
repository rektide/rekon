---
id: workspace
order: 170
source: /code/workspace/GLOBAL.md
type: ContextFragment
title: Workspace and known places
description: Global directory of known places, explicit home-path notation, and workspace operating guidance.
resource: /code/workspace/GLOBAL.md
status: draft
updated: { by: "model:openai/gpt-6-astra#high", at: 2026-11-03 }
sources:
  - { resource: "opencode://ses_e614482c2ffeXMJXi14xoFW33A", title: "Human selects ~/r as the global reference space; move the notes repository, preserve aliases and publish the known-places directory" }
  - { resource: /code/workspace/README.md, title: Maintained known-places directory and migration account }
---

# Workspace, research locations, and shell

## Known places

**Spell home paths with `~/`:** bare `ado/` or `doc/` looks relative to the
current directory. `~/r/` is the operator's global reference/wiki space;
**rk means Rekon**, the separate `~/src/rekon/` project. A project-local `doc/`
belongs to that project. [Directory home and alias details](/code/workspace/README.md).

| Path / alias | Place and purpose |
| --- | --- |
| `~/r/` | Global reference space: broad-topic notes, research and a growing personal wiki. |
| `~/ado/` | `~/r/opencode/`: OpenCode notes, including `~/ado/patches.md`. |
| `~/ad/`, `~/archive/doc/`, `~/src/doc/` | Compatibility aliases to `~/r/`; use `~/r/` for new references. |
| `~/a/` → `~/archive/` | Archived source repositories, normally `<org>/<repo>`. |
| `~/src/` | First-party checkouts and development variants. |
| `~/w/` | Resolves to `~/src/opencode-working/`, the OpenCode compose workspace. |
| `~/wiki/` | Generated architectural notes (currently resolves to `~/deepwiki/`), separate from `~/r/`. |
| `~/doc/` | Points to `~/Documents/`, separate from the reference space and project-local `doc/`. |
| `~/ff/` | `~/src/future-fuze/`: Kubernetes resources. |
| `/opt/`, `/srv/` | Installed software and service instances. |

## Workspace practice

- Check `~/archive/<repo-org>/<repo-name>` first for third-party source before
  fetching from the web. `~/wiki/<repo-org>/<repo-name>` may hold generated
  architectural notes. Check an archived project's `llms.txt` if present.
- First-party checkouts are usually `~/src/<repo-name>`; a variant is usually
  `~/src/<repo>-<variant>/` with a jj workspace. `~/r/` holds broad-topic notes.
  Project `doc/` holds project material,
  subject to native conventions and Rekon's own domain-first structure.
- `dl <url>` fetches a repository and its wiki to those archive/wiki locations.
  `~/ff` holds future-fuze Kubernetes resources; `/opt` and `/srv` hold installs
  and service instances.
- Use `rg` and `fd`; prefer `fd` over `find` for discovery.
- Put exploratory scripts and files under `.test-agent/<topic>/` at the project
  root, with a README explaining the scratch work. It is gitignored; promote
  load-bearing findings deliberately. Prefer a shebang and executable scripts.
- Environment variables do not persist across shell tool executions. Set needed
  variables each time. Use the harness's approved temporary directory when a
  file must live outside project scratch.

Source: [legacy workspace guidance](/AGENTS.old.md#well-known-directories-and-files).
