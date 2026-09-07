---
type: PracticeConcept
title: README-driven development — write the experience we want to build
description: Use a future-forward README to clarify a project's promise, learn from upstream possibilities, and steer coherent increments without confusing intent with delivery.
resource: /praxis/readme-driven/README.md
tags: [praxis, readme, direction, architecture, upstream, collaboration]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
sources:
  - { resource: "urn:opencode:session:ses_f96a76b7dffeQ4y5764g9miWV0", title: Human direction to steer iroh-aware through a future-forward README and capture the tactic in Rekon }
  - { resource: /praxis/chase-future/chase-future0.gpt6a.md, title: Current upstream possibilities and opportunity-opening slices }
  - { resource: /AGENTS.md#architectural-stewardship, title: Cooperative learning and consequential architectural checkpoints }
  - { resource: /code/commits.md#commits-checkpoints, title: Capture the stage before changing direction }
---

# README-driven development

**Write the project you want someone to understand and use; let that account
help decide what to build next.** Start with the useful experience, not the
current file tree or a backlog arranged into prose. A future-forward README
gives an effort a readable direction before every mechanism has been selected.

This is a human-requested tactic within [Rekon praxis](/praxis/README.md), not a
new mandatory workflow, installed skill, or executable command. It can start a
project or help an existing one recover a coherent direction after accumulating
many threads.

## Future-forward, not fictional

Describe the intended experience confidently under a clear **Direction** or
**What we are building** heading. Separately say what someone can run today.
Do not weaken every sentence with a caveat; do not let the framing disappear
halfway down the page either.

Keep three things legible:

| Surface | Its job | What it must not imply |
|---|---|---|
| Intended experience | Explain who benefits, what becomes possible, and the important guarantees | That the feature already ships |
| Present implementation | Give real commands, working pieces, and observed limits | That today's workaround is the enduring architecture |
| Open choices | Name the few decisions that materially change the destination or contract | That every interesting possibility is an accepted milestone |

Examples of proposed APIs or commands must be labeled as proposals and kept
distinct from instructions that work today. Source support, a passing local
test, and a hardware observation support different claims.

## The loop: write, learn, negotiate, build

1. **Recover the intent.** Read the human's direction and enough of the current
   project to distinguish goals from accidents. An old README or ticket title
   is not authority to revive a direction the human rejected.
2. **Write a useful future.** Explain one concrete journey, the important domain
   words, and the responsibilities of the project and its dependencies. Let
   broader possibilities inform the shape without making all of them scope.
3. **Learn what can support it.** Read current upstream source from local
   checkouts first. Record the inspected revision and distinguish an API's
   existence from a working integration. Look for capabilities that enlarge the
   offer, not only constraints that shrink it.
4. **Negotiate consequential choices.** A collaborator teaches back what became
   possible or impossible, the evidence, and the alternatives. The parent reads
   the important source, challenges assumptions, and brings scope-changing
   decisions back to the human. Revise the narrative in light of that exchange.
5. **Select a coherent next slice.** Choose a useful behavior or experiment that
   advances a README promise. Explain what the slice will demonstrate, what it
   leaves open, and why it is worth doing. Writing the README does not itself
   authorize implementation, migration, or publication.
6. **Return the learning.** After authorized work, compare the result with the
   promise. Update current capability, correct the intended contract if needed,
   and reconsider possibilities the new evidence opens. Save the prior stage
   before revision; a commit preserves an argument without declaring it accepted.

This is a conversation with evidence, not a one-pass handoff to “write a better
README.” The document, code, and human's intent should inform one another.

## A useful README shape

Use only the sections the project needs. A compact starting point is:

- **The offer:** what useful thing this project makes possible, and for whom.
- **One journey:** a user scenario, example, or small diagram from discovery of
  the feature to its useful result.
- **The model:** the few concepts needed to understand that journey; explain
  identity, ownership, or trust boundaries before relying on them.
- **How the pieces fit:** which project owns which responsibility and where
  platform-specific behavior remains different.
- **Today:** real entry points, current capability, evidence, and limitations.
- **Next and open:** the next meaningful proof and a few choices worth steering.
- **Further reading:** explained links to the native designs and work anchors.

Keep byte layouts, experiment logs, detailed alternatives, and historical
arguments in their maintained homes. Link to them rather than turning the
README into another exhaustive specification or a duplicate issue tracker.

## Common ways to miss

- **Polished repetition:** faithfully synthesizing stale documents preserves
  their mistakes. Verify direction as well as factual claims.
- **Premature narrowing:** a minimal wire profile, one platform, or a fixed
  protocol may be a good tracer without defining the permanent product.
- **Speculative sprawl:** every attractive future becomes a promised feature.
  Preserve options with reasons and revisit conditions instead.
- **Borrowed guarantees:** a dependency's capability is described as if the
  integration already delivers it. Trace the boundary that still needs proof.
- **New prose, unchanged direction:** the README reads better but still cannot
  help choose between two plausible next investments.

The tactic helped if the human can explain the project more clearly, identify
the next worthwhile action, and tell which evidence could change the course.
A longer README, a committed draft, or agreement between two models is not proof.

## Motivating use: iroh-aware

The commissioning conversation asked for an Astra collaborator to help steer
iroh-aware incrementally, beginning with a future-forward README. The human then
pointed to `~/src/nan-rs` and `~/wl-nan80211-nan`, asking the collaborator to learn
and teach rather than receive a fully prescribed architecture. Archived iroh is
another grounding source. Those paths record the research commission, not a
claim that all integrations already work.

The immediate pressure was many competing threads, an accidental NDN direction,
and an unclear identity story. The first teaching checkpoint challenged earlier
socket-integration assumptions and distinguished generic NAN responsibilities
from the application's iroh integration. It also surfaced a consequential
consumer choice: embedded applications, reference-app users, or local processes
using a broker. The human selected the first two; broker operation remains an
option, not an accidental commitment.

The collaborator then rewrote the [iroh-aware README](file:///home/rektide/src/iroh-aware/README.md)
around that direction (`b923869f`). Parent review retained the story and clarified
the private/public key relationship and protocol-negotiation timing in a separate
commit (`fc79b4c2`). Source-backed disagreement changed the binding claims rather
than merely polishing them; the README now distinguishes reusable integration as
a destination from today's Android experiment.

These are steering and documentation results, not a delivered connection or proof
that the whole method worked. The human's response to the revised introduction
and the next implementation's evidence remain to be learned. The committed
[iroh-aware learning report](file:///home/rektide/src/iroh-aware/.design/readme-direction/learning0.gpt6a.md)
and [human-selected direction](file:///home/rektide/src/iroh-aware/.design/readme-direction/steering0.gpt6a.md)
remain in the local product checkout; the commissioning session in this
document's sources retains their conversational provenance. Product proposals
and implementation lifecycle stay in iroh-aware. This home records the general
tactic, not a second roadmap.

## Cross-references

- [Chase the future](/praxis/chase-future/chase-future0.gpt6a.md) supplies the
  **opportunity-seeking posture**. A README expresses a selected destination;
  it should not erase the other courses that practice preserves.
- [Digest](/digest/README.md) **recovers present bearings and intent**. README-
  driven work uses those bearings to articulate the offer, rather than merely
  compressing the current state.
- [Post-build](/praxis/post-build/README.md) **opens possibilities after a working
  vertical**. Those discoveries can change the next README-led direction.
- [Architectural stewardship](/AGENTS.md#architectural-stewardship) supplies the
  **collaboration boundary**: delegate learning and bounded work while retaining
  responsibility for fit, tradeoffs, and human steering.
- [Checkpoint discipline](/code/commits.md#commits-checkpoints) preserves the
  **before and after** of a changed promise without conflating saving, acceptance,
  and permission to act.
