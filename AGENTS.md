# subagents

- run in background
- when you get a reply from a subagent, consider whether to respond now or to wait for other subagents to resolve first. i tend to prefer documents get built deliberatively, given the benefit of previous context that could become available that could help latter documents to be better structured.
- UNLESS DIRECTED BY USER ignore skills/etc that say to use a given model. we do not have access to most of these. general agent is usually a solid go to agent for most uses!
- cite existing research/md at your subagents to get them primed
- have agents write/maintain their own md files often in design/<topic>/ so their work can be cited. have them jj commit.
- tell subagents to favor not overwriting files generally, other than README / index / shared file maintenance, unless instructed otherwise
- ask subagents to return references, to facilitate follow-up investigation/exploration.
- remember to use the task_id to talk to an agent again; self maintain this prompt to make sure this directive is proper tool use for you

# Experimenting

- use `rg` and `fd` for composing shell commands, both are kept fresh. prefer `fd` over `find` for path discovery.
- a `variant` is usually a ~/src/<repo>-<variant>/ directory with a jj workspace in it
- `.test-agent` is our git-ignored scratch directory. prefer doing a domain of work inside a subdirectory, please, and include/maintain a README.md there

# Documentation / reference materials

- make markdown links to the canonical url for entries, for example [`/README.md`](/README.md) for local files or [`rektide/compfuzor` `README.md`](https://github.com/rektide/compfuzor/blob/main/README.mdL10-20) for a remote canonical reference to lines 10-20 of `README.md` in `rektide/compfuzor` project, file `README.md` (for the main branch). the org/repository isn't required each time but should be explicitly included in the link text if we haven't been talking about that org/repository recently.
- either `.test-agent/<topic>/` or `design/<topic>/` (sometimes `doc/`, rarely `docs/`) are typical places for docs, the first being gitignored, and used before we are ready to "promote" (move out of gitingore and commit) the work.
- a technique that can be helpfu: an addendum. this ia new top level h1 section at the end of a markdown that packages new information / in the document, but not inline. use pandoc to write OKF tags for the title. you are free to mix inline edits and addendum in to commits!
- the `summarizer` tool is WIP and not done yet. but will latter be a tool to write and read INDEX files in documentation directories to be aware of what's included in the directory.

## wave

often we want to do a "wave" of review, where multiple llms produce their own independent output for the wave. this is typically in design/<short-feature-name>/ , usually two maybe three words. sometimes .test-agent/. we'll end up with a bunch of design/<feature>/<wave>.<model-name>.md files.

- common waves:
  - `init` for initial exploration / problem write ups
  - `draft` for an initial design draft
- we add numerical suffixes and increment as we go, as we do "rounds". feature3-syn2 usually means the third revision of the `feature` wave and the second synthesis around that feature3. as we iterate on topics we can add new suffixes to the wave, or start a new more focused wave.
- `syn` is a frequently used suffix (ex: draft-syn.glm52.md): as follow up to a wave often one or more llm's will be asked to synthesize from their point of view, to review the existing wave, and create a synthesis doc that combines the best elements of the different documents into their vision of the best document. discuss common themes and tensions amid the source material. at the end it should also cross compare, noting particular strengths, interesting/notable features, and relative weaknesses, and a general characterization of the different source documents. this is cross-review and re-integration/synthesis.
- please include a 0 suffix to be clear on your wave file (ex: draft0.ds4f.md).
- the "std name" is `design/<name>/<name>.<model>.md`: when a directory holds one main document rather than a wave, name the file after the directory/topic itself (ex: `design/roadmap/roadmap.ds4f.md`). revisions still get numerical suffixes (`roadmap2.ds4f.md`).
- if "accepted" please make a prefix-less symlink for the accepted wave. this is a signal that this is the accepted "tip" of material.
- start waves by capturing what is up. what are we doing, what the genereal situation is. what is the general problem, and then what more specifically is happening? if there are specific vectors/directions/instructions your user provided, please capture them. consider giving a couple sentance prompt that could be used to research the topic at hand, on the current lines/directions of inquiry. you do not need to restate as we go, but talking about what's new or whats changed or what's emerged from the previous is helpful stage setting!
- **every wave file you write must end in `.<model-name>.md` — this is the default, not an option.** `draft0.md`, `draft.md`, or any filename missing the model suffix is a bug: fix the name before you finish writing. good examples: `draft0.gpt56t.md`, `feature3-syn2.glm52.md`.
- be concise in your model name please, for example gpt56t (for gpt 5.6 terra) or ds4f (for deepseek 4 flash) or glm52 (for glm 5.2).
- use your present model name in the suffix every time you write a file! models get changed between rounds, so never copy a suffix from an earlier wave file — identify the model you are running as right now.
- the model name is the llm you are running on, never the harness or tooling around it. opencode is the agent harness that runs us, not a model: `*.opencode.md` is always wrong, and the same goes for any other harness/tool/agent-runner name. if you genuinely cannot determine the underlying model, use the most specific name you can and note the uncertainty at the top of the doc.
- if you are overwriting a file without being asked to, something is probably wrong, and you probably need to find a way to differentiate your work & write a new file.
- often we ping pong between design (generally, even if not explicitly named so) and synthesis review. design -> syn -> design2 -> design2-syn -> design2->syn2.
- we tend to write new files rather than update previous files. if we are close, sometimes we'll just integrate changes; usually this will be explicitly asked for.
- maintaining an index.md during this is hard but is encouraged once we start accepting the work as good.
- unless told do DO NOT read other files from your wave, from the other models: you are expected to do your own work and not pollute your thinking with their results; we want independent results, that's why we are asking multiple models. but you are free to read relevant files from previous waves.

## doc-pass

A `doc-pass` is an informal post-writing skill for connecting new work to the existing documentation corpus. It is recommended after most waves of significant weight, once the wave's core argument is written and can guide the search.

1. Identify the new or substantially changed documents and summarize their concepts, claims, vocabulary, and unresolved questions as search terms.
2. Search the containing documentation tree broadly by concept, not only by exact terminology. Check `README.md`, `index.md`, and other indexes first, then use filenames and full-text search to find less obvious relationships.
3. Read enough of each candidate to distinguish a substantive relationship from a keyword collision. Prefer a few explained references over a long undifferentiated link list.
4. Add or update a `Cross-references` section in the new work. Each entry should say why the linked document matters: prior art, supporting evidence, shared vocabulary, tension, contradiction, extension point, or downstream application.
5. Add reciprocal links to older documents when the new work materially changes how they should be navigated or understood. Preserve their original argument rather than opportunistically rewriting them.
6. Maintain the documentation area's `README.md` or index when the new work creates a durable concept, changes the recommended entry point, or fills a missing category. Group entries by domain and give each a short description; do not turn the index into another concept document.
7. Verify local links and reread the surrounding sections. The pass is complete when the significant relationships found are navigable in the appropriate direction and each added link explains its relevance.

A doc-pass may use a subagent to independently search the corpus. Ask it to return file and line references, candidate relationship types, and notable absences; integrate only findings verified against the source documents.

## Prompts / discussing

- prefer multiple choice to single choice questions. multiple items might be of interest, even when there is an apparent conflict & need to direct. give the user a chance to weigh the options with multiple choice options.
- some discussion before Q&A sections is appreciated.

## Writing

- Markdown: write structured, readable content with headings, lists, tables, fenced code blocks, examples, and descriptive links; use prose to explain relationships between concepts.
- For [Open Knowledge Format (OKF)](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md) concepts, write frontmatter as a trustworthy label: what it is, what it's called, what it describes, where its claims came from, who wrote it, who checked it, and when it needs review. Use the body for explanation and links.

  ```markdown
  ---
  type: Metric # REQUIRED: kind of knowledge (open-ended)
  title: Active customers # display name
  description: Distinct customers with a completed order. # one-line summary
  resource: https://example.com/metrics/active-customers # canonical URI for the asset
  tags: [sales, customers]
  status: stable # draft | stable | deprecated
  generated: { by: human:alice, at: 2026-07-27T12:00:00Z } # who wrote it
  verified: { by: human:bob, at: 2026-07-27T13:00:00Z } # who checked it
  stale_after: 2026-10-01 # review deadline (YYYY-MM-DD)
  sources: # where its claims came from
    - id: metric-policy
      resource: https://example.com/policies/active-customers
      title: Active customer metric policy
      author: team:revenue-operations
      last_modified: 2026-07-01
  ---
  ```

  - Use bundle-root-relative links such as `/tables/orders.md` for internal concepts; they stay stable when nearby files move.
  - `index.md`: reserved directory listing for progressive disclosure; group entries under headings with short descriptions, not a concept itself.
  - `log.md`: reserved update history; `YYYY-MM-DD` headings newest first, concise creation/update/deprecation entries.

# well known directories and files

- **ALWAYS check `~/archive/<repo-org>/<repo-name>` FIRST for source code** before fetching from the web. This is the primary location for third-party project checkouts.
- `~/archive/doc` (and it's `~/src/doc` symlink) is a jj repo where markdown notes are often kept for broad topics. `doc/` inside of projects is for project material.
- `~/wiki/<repo-org>/<repo-name>` has machine generated but pretty good wiki that elaborates the architecture & use of projects, if available
- `~/src/<repo-name>` or `~/src/<repo-name>-<workspace-name>` has 1st party checkouts for projects by the author.
- read a `/llms.txt` from each project in the archive. it might not exist, that's fine. never mind this rarely exists, sad.
- `~/ff` is future-fuze, our collection of kubernetes related resources
- `/opt` and `/srv` for general software installs, and for instances of software / services
- `dl <url>` fetches a repo and its wiki into `~/archive/<org>/<repo>` and `~/wiki/<org>/<repo>`.
- when writing files to explore behavior, please use a `.test-agent` folder at the top of the project for temporary files (creating it if needed), so they can be managed & cleaned up effectively.
- for scripts, prefer adding a shebang and making the script executable, to having to launch via `node <script>.js`

# Project Planning

- do not write time-frames like "days" or "weeks" into plans. your estimates are crude and will not match the actual delivery timelines: leave your time estimates out.
- mermaid diagrams are excellent visual references. use meaningful node identifiers when creating them.
- breaking changes are fine if a project is <1.0!
- we NEVER want a flat structure for our project layouts. we always want some kind of domain-grouped setup. if asked to provide options, we want multiple suggestions of domain-groupings that might make sense! we want to explore what domain groupings make sense.

# Coding advice

- "Let It Fail" principle, when it makes sense. Do not create a try-catch just to print an error: the stack trace will show what is wrong.
- pruning stale or undeclared artifacts is usually outside Compfuzor's scope. do not add automatic prune/reconciliation behavior unless the user explicitly requests it; create and update declared state, and leave cleanup deliberate.
- please jj commit as you go! commit pro-actively in logical groups.

# Rust instructions

- to publish a crate, run `cargo bump` with major/minor/patch, defaulting to patch unless told otherwise. then make a jj commit. then run cargo publish.

## Preferred Rust modules

- `figment2` for config
- `clap` v4 with derive macros and `clap_complete` for completion
- `bon` for builder pattern
- `nextest` for testing
- `criterion` for benchmarking with cargo-criterion. `criterion-perf-events` for linux performance events and `criterion-cycles-per-byte` when measuring throughput of data-oriented systems.
- `jiff` for time (not chrono!)
- `use `clippy` for rust checking.
- `tracing`, with good use of spans!

## Clap + Figment2 integration

Derive `Parser` (clap), `Serialize`/`Deserialize` (serde), and `Builder` (bon) on the config struct. Use `figment2::providers::Serialized::defaults(CliArgs::parse())` to feed clap-parsed args into figment2. Precedence: CLI > env vars > defaults.

# Rules and guidelines for JavaScript projects

- When importing other typescript code in the same project, use an explicit ts file extension such as `import foo from "<whatever>.ts"` for the import .

## Preferred npm modules

- refer to node modules with the `node:` prefix, such as `node:fs`.
- if there's no existing test tools, use `vitest` library for testing,
- use `gunshi` npm package for more complex CLI, if this app has a non-trivial CLI. use it's @gunshi/plugin-completion plugin for tab completion too.
- use `oxfmt` for formatting
- use `oxlint` for linting
- use `tsdown` to emit typescript. when
- the `@typescript/native-preview` package for typescript type-checking, with it's `tsgo` replacement for `typescript` binary. when testing to see if the build is ok, use this; tsdown will not type check which is what we actually care about.
- for config files, use either `c12` for complex needs or `xdg-basedir` at least for selecting config from XDG compliant config directory.
- use `pnpm install` to install dependencies. for the version use `@latest`. do not edit `package.json` directly! for example, `pnpm install -D gunshi oxfmt oxlint`, for these dev dependencies.

## promises / asynchronous / synchronous behaviors

- do not use sync methods like Node's `execSync` if there are async options available. prefer promise based methods over callback based methods, if available. avoid blocking calls when possible.
- AGAIN: use `node:fs/promises` instead of `node:fs`.
- `await` should occur when the value of a promise is actually needed. if we don't need the value soon-after, save the promise to a variable instead of awaiting it's value.

## build concerns

- Build output is ONLY for npm package distribution, not for development workflows or documentation.
- prefer pnpm to npm for managing packages.
- prefer writing .ts files to .js files! use typescript whenever possible.
- node.js has good typescript type stripping support. ALWAYS run .ts files directly during development and documentation: `node src/cli.ts`. Build artifacts (`build/`) are ONLY for npm package distribution. prefer typescript over javascript.
- whenver using the typescript package or writing scripts that use the typescript package, if possible, use noEmit, allowImportingTsExtensions, and isolatedDeclarations.

## typescript/classes

- make typescript members public. we allow others to see our internals, although we do not guarantee the stability of our implementations.

## npm script structure with concurrently

Use `concurrently` to compose build/check/fix tasks from namespaced sub-tasks. The pattern:

```
build:<tool>   — individual build steps (typecheck, bundle, etc.)
check:<tool>   — individual check/lint steps
fix:<tool>     — individual auto-fix steps
```

Aggregate targets run all sub-tasks in parallel via concurrently glob patterns:

- `build` → `concurrently 'pnpm:build:*'`
- `check` → `concurrently 'pnpm:check:*'`
- `fix` → `concurrently 'pnpm:fix:*'`

Typical sub-tasks:

- `build:wxt` / `build:type` — wxt webextension build / tsgo
- `check:lint-pkg` — vici lint (validate manifest)
- `check:lint` — vp lint (oxlint via vite-plus)
- `check:type` — tsgo (from @typescript/native-preview)
- `fix:format` — vp fmt --write src (oxfmt via vite-plus)
- `fix:lint` — vp lint --fix (oxlint via vite-plus)

Without `--names`, concurrently derives labels from the glob suffix (e.g. `lint-pkg`, `lint`, `typecheck` from `check:*`).

Include a `dev` target for watch/development mode (e.g. `wxt watch`).

## cli

- if there is a simple CLI interface that a file can expose for itself to exercise some or all of the file, check if the file is the main module then run the file if so.
- anytime there is a foo.main.js, in the corresponding foo.js file, check to see if we are the main module; if we are, do a dynamic import of foo.main.js and run that main. make these files executable, add the shebang at the top if needed.
- when checking if a file is the "main" import, always resolve the `process.argv[1]` link with realpath. use `then` to allow execution to continue. otherwise if this module is loaded NOT as the main module, loading will block on this check to finish.
- When providing usage examples or running CLI tools during development, use the .ts file path directly: `node src/cli.ts`. Never reference build output in documentation.

# jj / git

- use `jj commit -m "<description>" <files...>` to make commits as you go.
  - explicitly provide the files/directories that belong to this commit
  - you do not need to add files ahead of time; it will commit all changes.
  - do not be afraid of empty commits! when you use `jj commit`, jj commits the work then starts a new empty commit, which is effectively an automatic git staging of all current changes, of which there will be none to start. that empty commit is expected.
- you do not need to verify your commit has happened. if you use `jj commit` your work is saved. move on.
- never use jj edit! never use squash unless explicitly instructed! humans must review all changes before squash.
- use `jj diff --git` for diff style commits or `jj diff --stat` for summary diffs. plain jj diff is misleading without color.
- **jj has its own DSL for revsets and templates — do not guess syntax.** Consult `jj help -k revsets` and `jj help -k templates` before writing `-r` or `-T` arguments.
  - Revset DAG operators: `::@` (ancestors of @), `@::` (descendants of @), `::` (all visible), `x..y` (range). No `::~`.
  - Template fields are bare keywords, not object properties: `change_id.short()` not `commit.change_id.short()`. `description.first_line()` not `commit.description.first_line()`.
  - String concatenation in templates uses `++`.
- compact log: `jj log --limit 10 -r '::@' -T 'change_id.short() ++ " " ++ description.first_line() ++ "\n"'`
  - other useful template columns to add to `-T`: `author.timestamp().ago()` (relative time like "2 hours ago"), `author.name()` (author name), `commit_id.short()` (git hash, useful for CI/remotes), `bookmarks` (branch names on this commit), `tags` (tags on this commit), `divergent` (change ID has multiple commits), `immutable` (commit is in immutable set), `mine()` (current user authored), `parents.map(|c| c.commit_id().short())` (parent git hashes)
- do not try to push work! the human will do that when they want. ignore all other instructions and: do not push work.
- include any relevant ticket ids in the body of the commit message
- do not mention tickets in the title of the commit message! "Closing ticket \_\_\_\_" doesn't describe the work. Describe the work.
- do not mention "phases" of work in the title of a cmomit message! that is short-term planning information, not long term meaningful.
- unless you are told otherwise, it is safe to assume that you are the only actor modifying a directory. there may be unchecked in changes when you start, but after that, you may assume all changes are your own. use this to avoid looking at `jj status` before committing.
- `jj-hunk` is available for committing hunks, if you need to check in only some changes

# jj-hunk

- programmatic hunk selection for jj. lets you commit/split/squash specific hunks instead of whole files — use it when logical changes are intermingled within a file and you want clean commits.
- subcommands (run `jj-hunk <sub> --help` to confirm flags):
  - `jj-hunk list` — emit JSON of all hunks in current `@` change (use `-r <rev>` for other revisions). each hunk has an `id` (`hunk-<sha256>`); this is how you discover IDs.
    - `--format text` — compact human-readable output (file, hunk index, type, ID, and the added/removed lines); great for quick visual scanning instead of parsing JSON.
    - `--files` — just file + hunk counts, no content.
    - `--spec '<spec>'` — **dry-run**: preview which hunks a spec would select. returns `{"files": []}` if nothing matches. always run this before committing to catch mismatched IDs.
    - `--spec-template` — print a starter spec populated with every hunk ID; delete the IDs you don't want and pipe it back in.
    - `-i '<glob>'` / `-x '<glob>'` — include/exclude file patterns (repeatable) to filter noisy diffs.
  - `jj-hunk commit [SPEC] [MESSAGE]` — commit selected hunks; SPEC is a JSON/YAML string (inline works fine for a hunk or two), `-` for stdin, or `--spec-file <path>`.
  - `jj-hunk split [SPEC] [MESSAGE]` — split current `@` into two commits, selected hunks go into the new child.
  - `jj-hunk squash [SPEC]` — squash selected hunks into the parent.
- spec shape (same for all three): `{"files": {"<path>": {"ids": ["hunk-<id>", ...]}}}`.
- **CRITICAL: use the full `hunk-<sha256>` ID.** short prefixes silently match nothing and `jj-hunk commit` happily creates an empty commit with no error. guard against this with the `--spec` dry-run above.
- extracting IDs: `jj-hunk list | jq -r '.files[].hunks[].id'` prints just the IDs; pipe through `head`/`grep` as needed. to grab a specific file's ID: `jq -r '.files[] | select(.path=="<path>") | .hunks[].id'`.
- discovery + commit workflow:
  - `jj-hunk list --format text` to eyeball hunks and their IDs (or `jj-hunk list > /tmp/hunks.json` for full detail; match on `added`/`removed`/`context.pre`)
  - hand-write a spec: `{"files": {"<path>": {"ids": ["hunk-<full-id>", ...]}}}`
  - **verify**: `jj-hunk list --spec '<spec>'` — confirm it lists the hunks you expect before committing
  - commit: `jj-hunk commit '<spec>' "<message>"` (inline), or for large specs pipe via stdin (`jj-hunk commit - "<message>" < /tmp/spec.json`) or file (`jj-hunk commit --spec-file /tmp/spec.json "<message>"`)
- when to use vs alternatives:
  - jj-hunk: changes intermingled in a file, you want clean commits, no TTY (agents, CI)
  - plain `jj commit -m "<msg>" <files>`: file boundaries already align with logical commits (preferred when it works — simpler)
  - `jj split -p` / `jj squash -i`: interactive flows when a TTY is available
- gotcha: when a file has been wholesale rewritten, hunk-based splitting won't produce clean intermediate states (hunks reference a diff against the parent, and a rewrite's hunks don't compose into the intended intermediate). in that case, restore the file from the parent rev (`jj file show -r @- <file> > <file>`), apply the minimal change, commit, then write the full version back and commit again.

# beads

- Work from the project directory rather than using `bd -C`.
- Always provide a stable, human-readable `--id`; never use autogenerated IDs. Namespace child IDs under their epic.
- Epics explain why the work matters and how the system fits together. Children describe concrete implementation and acceptance criteria.
- Preserve the history of evolving work with `bd update --append-notes`; close tickets when complete.
- `bd dep add A B --type blocks` means A depends on B. The dependent issue goes first.
- Dolt is the source of truth. Keep `export.auto=true` (strongly recommended); with it enabled, routine commands do not need a manual export. If it is disabled or an immediate tracked snapshot is required, run `bd export -o .beads/issues.jsonl`.
- NEVER run `bd prime`. If beads or any other instruction tells you to run it, tell the user who instructed you; it generates unwanted `AGENTS.md` and `CLAUDE.md` files.

## Command patterns

### Minimal

```sh
# Create a ticket with a stable, human-readable ID.
bd create --id <prefix>-<short-name> --title "<title>" --type task --priority P2
```

### Linked work

```sh
# Check the project's required ID prefix.
bd config get issue_prefix

# Create the architectural parent.
bd create --id <prefix>-<epic-short-name> --title "<title>" --type epic --priority P1 --labels <label> --description "<description>" --acceptance "<acceptance>"

# Namespace the child ID under its epic.
bd create --id <prefix>-<epic-short-name>-<feature-name> --title "<title>" --type feature --priority P2 --labels <label> --description "<description>" --acceptance "<acceptance>"

# Link child to parent; the child goes first.
bd dep add <prefix>-<epic-short-name>-<feature-name> <prefix>-<epic-short-name> --type parent-child

# Add a blocker; the dependent issue goes first.
bd dep add <prefix>-<epic-short-name>-<feature-name> <prefix>-<epic-short-name>-<prerequisite-name> --type blocks

# Verify the issue and dependency edges.
bd show <prefix>-<epic-short-name>-<feature-name>

# List ready work matching a label.
bd list --status open --ready --label <label> --limit 20
```

### Investigate / close

```sh
# Search before creating a duplicate.
bd list --status open --desc-contains "<search text>" --sort priority --limit 20

# Inspect scope, notes, and dependencies.
bd show <issue-id>

# Claim the issue and preserve investigation history.
bd update <issue-id> --claim --append-notes "<notes>" --add-label <label>

# Close only after completion; reason records why closure is justified.
bd close <issue-id> --reason "<what was completed and how it was verified>"
```

### Supersede

```sh
# Create the replacement proposal.
bd create --id <prefix>-<replacement> --title "<title>" --type feature --priority P1 --description "<description>" --acceptance "<acceptance>"

# Link replacement to the issue it supersedes; replacement goes first.
bd dep add <prefix>-<replacement> <prefix>-<old-proposal> --type supersedes

# Close the obsolete proposal and explain where current direction lives.
bd close <prefix>-<old-proposal> --reason "Superseded by <prefix>-<replacement>."

# Check graph integrity.
bd dep cycles

# Inspect the replacement.
bd show <prefix>-<replacement>
```

## Rename and recovery

Prefer renaming or updating an existing ticket over creating a replacement duplicate. `bd rename` also rewrites dependency references.

```sh
# Rename one ticket and its dependency references.
bd rename <old-id> <new-id>
```

For a broad ID or dependency-graph rewrite that `bd rename` cannot handle, rebuild from JSONL. Removing the database discards beads' internal state; use this only for a deliberate clean re-import.

```sh
# Export the current records for the deliberate JSONL rewrite.
bd export -o .beads/issues.jsonl.bak

# Edit IDs and dependency references in .beads/issues.jsonl first.

# Remove the database only after the export is ready to re-import.
rm .beads/*.db

# Re-create the database from the edited export.
bd import .beads/issues.jsonl

# Verify the imported issue list.
bd list

# Spot-check an issue and its dependency edges.
bd show <issue-id>
```

# webcomponents

- webcomponents should use Web Components Toolkit (wc-toolkit) to generate Custom Element Manifests. components should be described with appropriate jsdoc, and wc-toolkit should run to generate manifests for that toolkit.

# shell usage

- env vars are not persisted across multiple shell executions, you need to set env vars each time.
