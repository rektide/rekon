---
type: Reference
title: Agent model and variant lookup
description: Concise, dated mapping from Rekon agent codes to expected model IDs, configured defaults and advertised variants.
resource: /agents/models/README.md
status: draft
generated: { by: "model:zai-coding-plan/glm-5.3#max", at: 2026-10-31 }
tags: [agents, models, variants, dispatch, reference]
sources:
  - { resource: "opencode://ses_e6c1f991dffeK5oewTLCA2j5zu", title: Human moves rate-limit fallback guidance into the G and F agent definitions }
  - { resource: "opencode://ses_e6ccb4e5effe6EWqmToxh2oLHO", title: Human requests a common-model lookup; parent reports models-tool results checked 2026-10-31 }
  - { resource: "opencode://ses_e6cc6c15bffe34JypBfFJ1p6L1", title: Rekon entrypoint investigation and model-reference home }
  - { resource: "opencode://ses_e6cc34212ffelIc5sLrW1xqvi7", title: Reconcile configured agent defaults with the dated runtime model inventory }
  - { resource: /agents/t.md, title: Terra agent definition }
  - { resource: /agents/ast.md, title: Astra agent definition }
  - { resource: /agents/sol.md, title: Sol agent definition }
  - { resource: /agents/l.md, title: Luna agent definition }
  - { resource: /agents/g.md, title: GLM agent definition and no-highspeed rule }
  - { resource: /agents/f.md, title: Flash agent definition and no-highspeed rule }
  - { resource: /design/agents/model-routing0.glm53m.md, title: Routing incident and separate model and variant definition keys }
---

# Agent model and variant lookup

Use this quick reference for common **explicitly requested** models and variants;
it is not permission to override a model or to delegate. The individual agent
definitions own configured defaults and usage guidance. This table records the
parent session's `tools.opencode.models` inventory checked **2026-10-31**, alongside
the definitions inspected in this session. Runtime availability, provisioning and
advertised variants can change: rerun `tools.opencode.models` for unlisted,
changed, or uncertain cases. For regular GLM 5.3 use `all: true` if necessary;
the default newest-family query may omit it.

| Agent code and definition | Model ID | Configured default | Advertised variants (2026-10-31) |
| --- | --- | --- | --- |
| [`t`](/agents/t.md) | `openai/gpt-6.1-terra` | `#medium` | none, `low`, `medium`, `high`, `xhigh`, `max` |
| [`ast`](/agents/ast.md) | `openai/gpt-6-astra` | `#high` | `low`, `medium`, `high`, `xhigh`, `max` |
| [`sol`](/agents/sol.md) | `openai/gpt-6.1-sol` | `#high` | `low`, `medium`, `high`, `xhigh`, `max` |
| [`l`](/agents/l.md) | `openai/gpt-6-luna` | `#xhigh` | none, `low`, `medium`, `high`, `xhigh`, `max` |
| [`g`](/agents/g.md) | `zai-coding-plan/glm-5.3` | `#max` | `low`, `high`, `max` |
| [`f`](/agents/f.md) | `zai-coding-plan/glm-5.3-flash` | `#max` | `low`, `high`, `max` |

“None” means no variant suffix, not the literal `#none`. A listed variant is an
observed catalog option, not a promise that the account can always run it. `ast`
is the registered code in these definitions; “A” or “a” in a human brief may
mean Astra, but is not itself a registered agent code. Check the active harness's
available agents: the `t` definition on disk need not be exposed in every session.

## Dispatch examples

For an explicitly requested variant, the subagent tool's `model` parameter takes
the exact `provider/model#variant` string; select an available agent appropriate
to the work separately. For example:

| Explicit request | Agent if available | Exact `model` override |
| --- | --- | --- |
| T xhigh | `t` | `openai/gpt-6.1-terra#xhigh` |
| A / ast medium | `ast` | `openai/gpt-6-astra#medium` |
| A / ast max | `ast` | `openai/gpt-6-astra#max` |
| G max | `g` | `zai-coding-plan/glm-5.3#max` |

If a requested agent is unavailable, do not invent its code: use an appropriate
available agent only when the commission and harness permit, while preserving the
explicitly requested model, or report the blocker. Do not set the `model` override
unless the user explicitly asks for the particular model or variant. In particular,
Luna is only for explicit requests. **Never select a `-highspeed` model**, including
for GLM or Flash. Follow the applicable agent definition for requested lower GLM
levels; availability in the table is not a broader selection policy.

For rate-limit fallbacks, follow the individual [`g`](/agents/g.md) and
[`f`](/agents/f.md) definitions.

The notation above is for a dispatch override, not agent-definition frontmatter.
The definitions deliberately keep `model:` and `variant:` separate: the
[routing incident](/design/agents/model-routing0.glm53m.md) documents why the
deployed loader could otherwise drop the selection. Configured default is not
proof of the model actually used; that incident also records a historical reload
problem. Verify runtime selection when it matters, without treating that old
incident as proof of the service's present state.
