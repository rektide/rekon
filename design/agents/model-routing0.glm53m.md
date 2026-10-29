---
type: Report
title: Agent model routing — why configured subagent models were silently dropped
description: Troubleshooting and fix receipt for subagents (sol, g, ast, f, l) inheriting the parent session's model instead of their configured variant-pinned model; covers the deployed opencode loader chain, the rekon-side fix, the upstream patch, and the remaining live-reload gap.
resource: /design/agents/model-routing0.glm53m.md
tags: [agents, opencode, model, variant, subagent, routing, incident]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-10-29 }
sources:
  - resource: opencode://ses_e79dfcb60ffeyCsbsrZxwsHhST
    title: G investigates incorrect Sol model selection, fixes rekon definitions and patches upstream modelSelection
  - resource: opencode://ses_e7a2f3e82ffeoCJWDWTcPQ3Ely
    title: Incident child session that ran Astra xhigh under the sol agent
  - resource: file:///home/rektide/src/opencode-working/packages/core/src/config/plugin/agent.ts
    title: Deployed loader — legacy routing on any unsupported frontmatter key
  - resource: file:///home/rektide/src/opencode-working/packages/core/src/v1/config/migrate.ts
    title: Deployed V1 migration — modelSelection regex rejecting embedded variants
  - resource: file:///home/rektide/src/opencode-working/packages/core/src/tool/plugin/subagent.ts
    title: Subagent tool model precedence (override, agent, parent)
  - resource: /agents/sol.md
    title: Fixed agent definition (model + variant split)
---

# Agent model routing — configured models silently dropped

Incident (2026-10-29): dispatching the `sol` subagent
(configured `openai/gpt-6.1-sol#high`) ran the parent session's
`openai/gpt-6-astra#xhigh` instead. Child session
`ses_e7a2f3e82ffeoCJWDWTcPQ3Ely` (agent `sol`) carries
`{"id":"gpt-6-astra","providerID":"openai","variant":"xhigh"}` in
`session_v2`; its actual parent `ses_e7bc69007ffeAbnuUQISP5w56g` ran
Astra xhigh. All five pinned agents (`sol`, `g`, `ast`, `f`, `l`) were
affected identically; `general` intentionally has no model.

## Root cause

The deployed server is the local build `v0.0.0-local-202610110425`
(source: `/home/rektide/src/opencode-working`). Three steps compose:

1. **Legacy routing** — `packages/core/src/config/plugin/agent.ts`
   `decode()`: any frontmatter key outside the V2 agent schema routes the
   whole file through V1 legacy migration. Our `note:` key (required by
   `rkt codes` for the codes-glossary row) triggers this. Current upstream
   (`origin/dev` @ `161d466966`) only softens this with a `nativeKeys`
   exception our files do not meet, so it behaves the same.
2. **Silent model drop** — `packages/core/src/v1/config/migrate.ts`
   `modelSelection()`: the regex `/^[^/#]+\/[^#]+$/` rejects
   `providerID/modelID#variant` outright and returns `undefined`, dropping
   the whole selection. V1 semantics then forward `note` into
   `request.body` (inert in practice, but visible via `/api/agent/sol`).
3. **Parent inheritance** — `packages/core/src/tool/plugin/subagent.ts`:
   `const model = override ?? agent.model ?? parent.model`. With
   `agent.model` gone, the child inherits the parent's model. Explicit
   per-call `model` overrides were never affected.

Reproduction: `.test-agent/agent-model/check.ts` replicates the deployed
`decode()` with the real modules from opencode-working. Before the fix
all five agents decoded with `model: undefined`; the incident child's DB
row is the live-system confirmation.

## Fixes

- **Rekon** ([`/agents/*.md`](/agents/sol.md)): split `model: provider/id#variant` into
  `model: provider/id` + `variant:` keys. The separate-key form survives
  V1 migration (`modelSelection(input, variant)` combines them) and the
  V2 decode path (its join logic re-embeds the variant), so it is correct
  on both the deployed build and current upstream. `note` stays; it is
  required by `rkt codes`.
- **Upstream** (workspace `opencode-agent-model`, commit `94aeb137`,
  based on `origin/dev` @ `161d466966`): `modelSelection` now parses the
  embedded variant instead of rejecting it; an embedded variant wins over
  a separate `variant:` key, matching V2 decode semantics. Regression
  cases in `packages/core/test/config/agent.test.ts` route Markdown
  through V1 migration and fail on the old code (verified red, then
  green; 41/41 pass; `bun typecheck` clean; normalization and
  v1-migration suites pass). Not deployed; no push/publication.

## Live status and remaining action

The on-disk rekon fix is verified against the deployed loader, but the
running service has **not** reloaded it: the server's config watches were
attached via watchman, which died 2026-10-26, and the fallback never
reattached for the global config root (Parcel itself sees the changes —
proven with a direct subscription in `.test-agent/agent-model/watch-test.ts`).
`GET /api/agent/sol` still reports no model.

Smallest remaining action: restart the elected `opencode` service at a
convenient time (or restore watchman and touch a global agents file).
Then verify with `GET /api/agent/sol` → `model: gpt-6.1-sol, variant:
high` before relying on default (non-override) subagent dispatches.

Until then, pin models explicitly on dispatch (the `model` parameter)
when the agent's configured model matters; explicit overrides always
worked.
