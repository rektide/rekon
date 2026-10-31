---
type: Reference
title: Agent model and variant lookup
description: Concise, dated mapping from Rekon agent codes to expected model IDs, configured defaults and advertised variants, with the standard variant abbreviations and the variant-column matrix.
resource: /agents/models/README.md
status: draft
generated: { by: "model:zai-coding-plan/glm-5.3#max", at: 2026-10-31 }
tags: [agents, models, variants, dispatch, reference]
sources:
  - { resource: "opencode://ses_e6c1f991dffeK5oewTLCA2j5zu", title: Human moves fallback into agent definitions and requests concise descriptions and field visibility documentation }
  - { resource: "https://opencode.ai/v2/docs/agents", title: OpenCode agent description and model configuration }
  - { resource: "opencode://ses_e6bb4e75effew3IrG08fLz8002", title: F investigates OpenCode subagent catalog and model selection }
  - { resource: "file:///home/rektide/a/a/opencode/packages/core/src/tool/plugin/subagent.ts", title: Inspected OpenCode subagent catalog and selection implementation }
  - { resource: "opencode://ses_e6ccb4e5effe6EWqmToxh2oLHO", title: Human requests a common-model lookup; parent reports models-tool results checked 2026-10-31 }
  - { resource: "opencode://ses_e6cc6c15bffe34JypBfFJ1p6L1", title: Rekon entrypoint investigation and model-reference home }
  - { resource: "opencode://ses_e6cc34212ffelIc5sLrW1xqvi7", title: Reconcile configured agent defaults with the dated runtime model inventory }
  - { resource: "opencode://ses_e6c20690cffeyOxxE0zP6Z4JVT", title: "Standard variant abbreviations, default #variant notation, and the variant-column matrix" }
  - { resource: /agents/t.md, title: Terra agent definition }
  - { resource: /agents/ast.md, title: Astra agent definition }
  - { resource: /agents/sol.md, title: Sol agent definition }
  - { resource: /agents/l.md, title: Luna agent definition }
  - { resource: /agents/g.md, title: GLM agent definition and no-highspeed rule }
  - { resource: /agents/f.md, title: Flash agent definition and no-highspeed rule }
  - { resource: /design/agents/model-routing0.glm53m.md, title: Routing incident and separate model and variant definition keys }
---

# Agent model and variant lookup

<a id="variant-notation"></a>
## Variant notation

Standard abbreviations, usable in conversation, tickets, and tables:

| Abbreviation | Variant |
| --- | --- |
| `∅` | no variant suffix |
| `lo` | low |
| `med` | medium |
| `hi` | high |
| `xh` | xhigh |
| `max` | max |

`default #xx` names an agent's configured default variant, as the codes-glossary
rows do; in the variant table the default is marked `xx*`. The abbreviations are
**reading and speaking notation only**: an actual dispatch `model` override always
uses the full variant name — `openai/gpt-6-astra#xhigh`, never `#xh`.

Use this quick reference for common **explicitly requested** models and variants;
it is not permission to override a model or to delegate. The individual agent
definitions own configured defaults and usage guidance. This table records the
parent session's `tools.opencode.models` inventory checked **2026-10-31**, alongside
the definitions inspected in this session. Runtime availability, provisioning and
advertised variants can change: rerun `tools.opencode.models` for unlisted,
changed, or uncertain cases. For regular GLM 5.3 use `all: true` if necessary;
the default newest-family query may omit it.

<a id="variant-table"></a>
## Variant table

One variants column lists whatever the dated catalog offered for each agent;
`*` marks the configured default. The provider identifies the family; the model
appears only where it is special — `f` runs the flash model rather than plain
GLM 5.3. Exact model IDs for overrides live in the dispatch examples and the
agent definitions.

| Agent code and definition | Provider / model | Variants |
| --- | --- | --- |
| [`t`](/agents/t.md) | `openai` | `∅ lo med* hi xh max` |
| [`ast`](/agents/ast.md) | `openai` | `lo med hi* xh max` |
| [`sol`](/agents/sol.md) | `openai` | `lo med hi* xh max` |
| [`l`](/agents/l.md) | `openai` | `∅ lo med hi xh* max` |
| [`g`](/agents/g.md) | `zai-coding-plan` | `lo hi max*` |
| [`f`](/agents/f.md) | `zai-coding-plan` · `glm-5.3-flash` | `lo hi max*` |

`∅` is the model's no-suffix form, not a literal `#none`. A marked variant is an
observed catalog option, not a promise that the account can always run it. `ast`
is the registered code in these definitions; “A” or “a” in a human brief may
mean Astra, but is not itself a registered agent code. Check the active harness's
available agents: the `t` definition on disk need not be exposed in every session.

## Dispatch examples

For an explicitly requested variant, the subagent tool's `model` parameter takes
the exact `provider/model#variant` string with the **full** variant name; select
an available agent appropriate to the work separately. A shorthand request
expands to the exact override:

| Shorthand request | Agent if available | Exact `model` override |
| --- | --- | --- |
| t xh | `t` | `openai/gpt-6.1-terra#xhigh` |
| ast med | `ast` | `openai/gpt-6-astra#medium` |
| ast max | `ast` | `openai/gpt-6-astra#max` |
| g max | `g` | `zai-coding-plan/glm-5.3#max` |

If a requested agent is unavailable, do not invent its code: use an appropriate
available agent only when the commission and harness permit, while preserving the
explicitly requested model, or report the blocker. Do not set the `model` override
unless the user explicitly asks for the particular model or variant. In particular,
Luna is only for explicit requests. **Never select a `-highspeed` model**, including
for GLM or Flash. Follow the applicable agent definition for requested lower GLM
levels; availability in the table is not a broader selection policy.

For rate-limit fallbacks, follow the individual [`g`](/agents/g.md) and
[`f`](/agents/f.md) definitions.

## What the parent sees

OpenCode's [agent documentation](https://opencode.ai/v2/docs/agents) says the
parent model sees a subagent's `description` when choosing an agent. In the
inspected [subagent implementation](file:///home/rektide/a/a/opencode/packages/core/src/tool/plugin/subagent.ts)
at checkout `4592000085`, the catalog exposes only the agent ID and description,
not its configured model or `note`. Put brief selection and fallback guidance
in the description. Rekon's `note` supplies the row in the generated
[codes glossary](/codes/GLOSSARY.md).

For a new child, model selection is explicit override, then configured agent
model, then parent model. The description can also state the exact ID for a
parent considering an explicitly requested override. When continuing an existing
child, the same agent retains its current model unless a model override is
provided; switching agents applies the new agent's model. Configuration and
catalog entries do not prove a request will succeed or that a running service
has reloaded a changed definition.

The notation above is for a dispatch override, not agent-definition frontmatter.
The definitions deliberately keep `model:` and `variant:` separate: the
[routing incident](/design/agents/model-routing0.glm53m.md) documents why the
deployed loader could otherwise drop the selection. Configured default is not
proof of the model actually used; that incident also records a historical reload
problem. Verify runtime selection when it matters, without treating that old
incident as proof of the service's present state.
