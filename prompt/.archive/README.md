# prompt/.archive

Prompts retired from [`prompt/`](/prompt/) because nothing actively uses them.
Kept for reference & archaeology; their presence here does not imply usage.
`prompt/` now holds only this `.archive/` — there is no active prompt material.

## Layout

| Dir | Contents |
|---|---|
| [`workflows/`](/prompt/.archive/workflows/) | Home-grown working techniques: planning, discovery, review, subagent prompting |
| [`cli/`](/prompt/.archive/cli/) | CLI authoring guidance (ours + collected) |
| [`opencode/`](/prompt/.archive/opencode/) | opencode-related: vendored prompt symlinks + compaction prompt |
| [`external/`](/prompt/.archive/external/) | Collected from elsewhere: model system prompts, third-party tool guidance |
| [`uri-axioms/`](/prompt/.archive/uri-axioms/) | Creative project: image-generation prompts visualizing TBL's URI axioms |

## workflows/ — home-grown, in our voice

The most valuable material here; several techniques live on in evolved form in
`AGENTS.md` practice (design waves, syn reviews, subagent stage-setting).

- [`draft.md`](/prompt/.archive/workflows/draft.md) — draft-the-plan: planning philosophy + process. Self-describes as "the heart of the `rekon` prompt system". Plan philosophies (conceptualization not concretization, SHORT-NAMES, WORK LOG) echo through today's `design/` wave practice.
- [`discovery.md`](/prompt/.archive/workflows/discovery.md) — doc/discovery write-up workflow with journal sections; parametrized ($1..$7) template.
- [`review-appendix.md`](/prompt/.archive/workflows/review-appendix.md) — append a self-review appendix after reading peer reviews; direct ancestor of the wave `syn` cross-review practice.
- [`no-the-subagent.md`](/prompt/.archive/workflows/no-the-subagent.md) — "don't do it, elaborate a subagent prompt instead": intro / general problem / specific ask / references / requirements.
- [`commit.md`](/prompt/.archive/workflows/commit.md) — one-liner: jj commit with verbose justification; superseded by `AGENTS.md` jj section.
- [`page-scripting.md`](/prompt/.archive/workflows/page-scripting.md) — lol-html streaming page processing + playwright escape hatch.

Also home-grown but filed under [`cli/`](/prompt/.archive/cli/): [`cli.md`](/prompt/.archive/cli/cli.md), the FILE-OR-PATTERN CLI argument convention.

## cli/

- [`cli.md`](/prompt/.archive/cli/cli.md) — home-grown CLI conventions (FILE-OR-PATTERN).
- [`rust-cli.md`](/prompt/.archive/cli/rust-cli.md) + [`.url`](/prompt/.archive/cli/rust-cli.md.url) — collected: [`Dicklesworthstone/coding_agent_session_search` RUST_CLI_TOOLS_BEST_PRACTICES_GUIDE.md](https://github.com/Dicklesworthstone/coding_agent_session_search/blob/main/RUST_CLI_TOOLS_BEST_PRACTICES_GUIDE.md). Large (71 KB); predates our current Rust preferences in `AGENTS.md` (figment2, bon, jiff, clap v4), so partially outdated.
- [`cli-development-with-gunshi.md`](/prompt/.archive/cli/cli-development-with-gunshi.md) — generic "expert Gunshi developer" template; real Gunshi guidance now lives in `AGENTS.md` (gunshi + plugin-completion).

## opencode/

- [`comp-act.md`](/prompt/.archive/opencode/comp-act.md) — compaction/summarization prompt, roughly a standalone take on opencode's `compaction.txt`.
- [`opencode`](/prompt/.archive/opencode/opencode) → `vendor/opencode/packages/opencode/src/agent/prompt` — upstream agent prompts (compaction, explore, summary, title).
- [`opencode-session`](/prompt/.archive/opencode/opencode-session) → `vendor/opencode/packages/opencode/src/session/prompt` — upstream session prompts (summarizers, spoof variants).

Symlinks were retargeted (`../../../vendor/…`) through two moves so they still resolve.

## external/

- [`llama4scout-anon.md`](/prompt/.archive/external/llama4scout-anon.md), [`llama4scout-fake.md`](/prompt/.archive/external/llama4scout-fake.md) — leaked Llama 4 Scout system prompts; curiosity value.
- [`codanna.md`](/prompt/.archive/external/codanna.md) + [`.url`](/prompt/.archive/external/codanna.md.url) — [`bartolli/codanna` agent-guidance](https://github.com/bartolli/codanna/blob/main/docs/integrations/agent-guidance.md); MCP tool guidance for a tool we don't currently use.
- [`initial-project-plan.md`](/prompt/.archive/external/initial-project-plan.md) — generic "expert project planner" boilerplate; low value.

## uri-axioms/ — chronology

A creative Jan 2026 project: image-generation prompts visualizing
[Tim Berners-Lee's *Axioms of Web architecture*](https://www.w3.org/DesignIssues/Axioms.html)
and "Cool URIs don't change" across aesthetic themes. Prompt-writing only —
no generated images were ever committed. Two-model parallel effort (`glm47` /
`k2` suffixes), which predates the formal wave conventions in `AGENTS.md`.

- **Jan 5** — project starts: [`uri-axiom-of-web-architecture.url`](/prompt/.archive/uri-axioms/uri-axiom-of-web-architecture.url) saves the W3C source; [`uri-axiom-img.glm47.md`](/prompt/.archive/uri-axioms/uri-axiom-img.glm47.md) is the first image prompt (single diagram concept).
- **Jan 6** — [`uri-axiom-transcript.k2.md`](/prompt/.archive/uri-axioms/uri-axiom-transcript.k2.md) captures the session that fetched the source doc and spun up themed prompts; k2 drafts [`uri-axiom-img.k2.md`](/prompt/.archive/uri-axioms/uri-axiom-img.k2.md) (9 themes as numbered prompt lists); theme files filled in: nature-and-organic, cultural-and-historical, modern-and-abstract, minimalist-and-typographic, surreal-and-dreamlike, technical-and-industrial.
- **Jan 7** — continuation session: [`uri-axiom-img-metaprompt.k2.md`](/prompt/.archive/uri-axioms/uri-axiom-img-metaprompt.k2.md) (project braindump: 1 of 9 themes complete, 8 empty shells) and [`TODO-uri-axiom-img-files.md`](/prompt/.archive/uri-axioms/TODO-uri-axiom-img-files.md) plan the rest; remaining themes completed (architectural-and-monumental, cosmic-and-scientific, metaphorical-and-symbolic); glm47 runs its own parallel pass — summaries ([`.glm47`](/prompt/.archive/uri-axioms/uri-axiom-summary.glm47.md), [`.k2`](/prompt/.archive/uri-axioms/uri-axiom-summary.k2.md)), its copy of the TBL axioms ([`.glm47`](/prompt/.archive/uri-axioms/uri-axiom-of-web-architecture.glm47.md), [`.k2`](/prompt/.archive/uri-axioms/uri-axiom-of-web-architecture.k2.md)), and 10 themed "grand vision" metaprompts (`axiom-uri-img.*.glm47.md`: celtic knotwork, constellation, fractal, mandala, memphis, neon-noir, psychedelic, solarpunk, vaporwave, plus its metaprompt).

End state: 9 theme prompt files + 10 themed metaprompts + source summaries,
but no images generated from them. Fun artifact; zero operational use.

## History

- `COMBINED.md` (repo root) was a concatenation of four external prompts; deleted alongside the original archive since its sources are all here.
