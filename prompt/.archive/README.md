# prompt/.archive

Prompts retired from [`prompt/`](/prompt/) because nothing actively uses them.
Kept for reference & archaeology; their presence here does not imply usage.
[`prompt/uri/`](/prompt/uri/) remains the only active prompt material.

## Provenance

### Home-grown (authored by us, rekon voice & workflows)

The most valuable material here — these document how we like to work, and
several techniques live on in evolved form elsewhere (see cross-references).

| File | What it is | Notes / value |
|---|---|---|
| [`draft.md`](/prompt/.archive/draft.md) | draft-the-plan: planning phase philosophy + process | Self-describes as "the heart of the `rekon` prompt system". Plan philosophies (conceptualization not concretization, SHORT-NAMES, WORK LOG) echo through `design/` wave practice in `AGENTS.md` today. |
| [`discovery.md`](/prompt/.archive/discovery.md) | doc/discovery write-up workflow with journal sections | Parametrized ($1..$7) template; the journal-header technique still sees use. |
| [`review-appendix.md`](/prompt/.archive/review-appendix.md) | append a self-review appendix after reading peer reviews | Direct ancestor of the wave `syn` / cross-review practice. |
| [`no-the-subagent.md`](/prompt/.archive/no-the-subagent.md) | "don't do it, elaborate a subagent prompt instead" technique | The stage-setting + requirements-list prompt pattern used with subagents. |
| [`cli.md`](/prompt/.archive/cli.md) | CLI authoring guidelines, FILE-OR-PATTERN argument convention | Own conventions, not copied from anywhere. |
| [`commit.md`](/prompt/.archive/commit.md) | one-liner: jj commit with verbose justification | Superseded by the jj section of `AGENTS.md`. |
| [`page-scripting.md`](/prompt/.archive/page-scripting.md) | lol-html streaming page processing + playwright escape hatch | Task-specific. |

### Collected from elsewhere (external)

| File | Source | Notes / value |
|---|---|---|
| [`rust-cli.md`](/prompt/.archive/rust-cli.md) + [`.url`](/prompt/.archive/rust-cli.md.url) | [`Dicklesworthstone/coding_agent_session_search` RUST_CLI_TOOLS_BEST_PRACTICES_GUIDE.md](https://github.com/Dicklesworthstone/coding_agent_session_search/blob/main/RUST_CLI_TOOLS_BEST_PRACTICES_GUIDE.md) | Large (71 KB) guide; predates our own Rust preferences in `AGENTS.md` (figment2, bon, jiff, clap v4), so partially outdated. |
| [`codanna.md`](/prompt/.archive/codanna.md) + [`.url`](/prompt/.archive/codanna.md.url) | [`bartolli/codanna` agent-guidance](https://github.com/bartolli/codanna/blob/main/docs/integrations/agent-guidance.md) | Tool-specific MCP guidance for a tool we don't currently use. |
| [`llama4scout-anon.md`](/prompt/.archive/llama4scout-anon.md), [`llama4scout-fake.md`](/prompt/.archive/llama4scout-fake.md) | leaked Llama 4 Scout system prompts (anon & fake variants) | Curiosity value only. |
| [`comp-act.md`](/prompt/.archive/comp-act.md) | generic conversation-compaction/summarization prompt | Standard boilerplate, no unique content. |
| [`initial-project-plan.md`](/prompt/.archive/initial-project-plan.md) | generic "expert project planner" template | Low value; LLM-generated boilerplate. |
| [`cli-development-with-gunshi.md`](/prompt/.archive/cli-development-with-gunshi.md) | "expert Gunshi CLI developer" template | Generic expert-prompt style; real Gunshi guidance now lives in `AGENTS.md` (gunshi + plugin-completion). |

### Vendored references

| Link | Target |
|---|---|
| [`opencode`](/prompt/.archive/opencode) | `vendor/opencode/packages/opencode/src/agent/prompt` |
| [`opencode-session`](/prompt/.archive/opencode-session) | `vendor/opencode/packages/opencode/src/session/prompt` |

Symlinks retargeted (`../../vendor/…`) after the move so they still resolve.
Reference material for studying upstream opencode prompting; `comp-act.md` is
roughly a standalone copy of opencode's `compaction.txt` idea.

## History

- `COMBINED.md` (repo root) was a concatenation of four of the external prompts
  above; deleted alongside this archive since its sources are all here.
