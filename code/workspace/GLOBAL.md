---
id: workspace
order: 170
source: /code/workspace/GLOBAL.md
status: draft
---

# Workspace, research locations, and shell

- Check `~/archive/<repo-org>/<repo-name>` first for third-party source before
  fetching from the web. `~/wiki/<repo-org>/<repo-name>` may hold generated
  architectural notes. Check an archived project's `llms.txt` if present.
- First-party checkouts are usually `~/src/<repo-name>`; a variant is usually
  `~/src/<repo>-<variant>/` with a jj workspace. `~/archive/doc` (also reachable
  via `~/src/doc`) holds broad-topic notes. Project `doc/` holds project material,
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
