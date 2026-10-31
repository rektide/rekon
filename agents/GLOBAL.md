---
id: collaboration
order: 150
source: /agents/GLOBAL.md
status: draft
type: ContextFragment
title: Collaboration and architectural stewardship
description: Delegation, context budgeting, architectural review and collaborative decision practices.
resource: /agents/GLOBAL.md
generated: { by: "model:zai-coding-plan/glm-5.3#max", at: "2026-10-31" }
sources:
  - resource: "opencode://ses_e6c1f991dffeK5oewTLCA2j5zu"
    title: Human moves rate-limit fallback from global dispatch into agent definitions
  - resource: "opencode://ses_e6cc34212ffelIc5sLrW1xqvi7"
    title: Add a concise pointer to the dated model and variant lookup
  - resource: "opencode://ses_e7bc138ffffeHcfSJmcsHKoO9Z"
    title: Human requests a soft 300k delegated-session context target, tolerates modest overshoot and directs the nearly finished Sol slice to wrap
---

# Collaboration and architectural stewardship

## Dispatch and return

- Run delegated agents in the background. Favor deliberate integration once
  prerequisite results are available; do not rush dependent documents.
- Aim to keep delegated sessions under **~300k context tokens**. Modest overshoot
  to 330–350k is acceptable near a coherent finish; let an almost-finished slice
  wrap rather than interrupting solely for this target. Start later stages in
  fresh sessions with concise handoffs and links to receipts and relevant code.
- Unless the user directs model selection, do not obey skills' demands for
  unavailable models. `general` is a capable default; for high-value work and
  deep research, ask which agents the user wants for which tasks. See
  [the general-agent guidance](/agents/general.md).
- For explicitly requested common models and variants, use the dated
  [model lookup](/agents/models/README.md); recheck uncertain or changed availability.
- Prime agents with existing research and relevant Markdown. Ask them to return
  file/section references and to write and commit their own model-suffixed
  documents under the effort's domain, often `design/<topic>/`.
- Preserve other authors' files; favor new files over overwriting independent
  drafts. Use the continuation identifier actually returned by the harness
  (`sessionID` here) to resume the same agent.

## Stewardship

- Delegate implementation and routine verification, not architectural
  responsibility. The parent assesses interfaces, domain concepts, reuse,
  tradeoffs, user intent, and verified upstream capabilities.
- Review at consequential architectural checkpoints rather than arbitrary
  file-by-file phases. Ask what became possible or impossible, choices and
  evidence, unresolved tensions, and the proposed next coherent slice.
- Trust reported checks; do not rerun them just to confirm. Read relevant code
  and evidence, and add verification when it answers a specific concern.
- Cooperatively anneal broader solutions. Explore plausible futures and surface
  their assumptions, costs, and opportunities. Distinguish possibilities worth
  preserving from capabilities authorized for implementation; let agents choose
  coherent slices within the agreed direction. Discuss forks that materially
  alter the shape rather than silently building or dismissing them.

## Discussion

Prefer multiple-choice questions allowing multiple selections when several
directions may matter. Explain the situation before the questions.

Recovered from [the legacy operating manual](/AGENTS.old.md#subagents);
[assessment](/assess/README.md) supplies deeper review practices.
