---
type: UseCase
title: Kinefolk firmware research flight — gated launch pattern
description: Concrete agent dependency graph with parallel orientation, prewarming, external-system research, Kinefolk-fit inquiry, independent syntheses and a final returning-agent turn.
resource: /praxis/flights/kinefolk-firmware0.gpt6a.md
status: completed-research-case
generated:
  by: model:openai/gpt-6-astra#high
  at: 2026-10-31
sources:
  - resource: opencode://ses_e6c978f10ffeTGkK163WA0Ovly
    title: Human-clarified flight commission and parent-reported conduct, learning, context tradeoff and evidence limits
  - resource: opencode://ses_e6c900557ffex5YjEpZrQCueNC
    title: Relocate the parent-authored case, align its diagram with praxis and record the completed flight's observed outcome
  - resource: "file:///home/rektide/src/kinefolk-rs/design/input-model/firmware/brief0.gpt6a.md"
    title: Owning research commission, actual dispatch ledger and completed wave sequence
  - resource: "file:///home/rektide/src/kinefolk-rs/design/input-model/firmware/firmwares0.gpt6a.md"
    title: Dedicated firmware study and corrections to prewarm leads
  - resource: "file:///home/rektide/src/kinefolk-rs/design/input-model/firmware/final0.gpt6a.md"
    title: Final reconsideration, shared-evidence limits, model examples and open decisions
  - resource: /praxis/flights/README.md
    title: Flight model, lifecycle and failure semantics
  - resource: /praxis/workflow-diagrams/workflow-diagrams.gpt6a.md
    title: One-name diagram discipline and relationship conventions
  - resource: /assess/gates/README.md
    title: Arrival, readiness and release of evidence gates
  - resource: /evidence/README.md#evidence-case-stories
    title: Expected help and later assessment of actual conduct
---

# Use case: research the other, then reason about our model

The operator wants agents to understand QMK and ZMK, then use that evidence to
reason about Kinefolk's capabilities, fit, expressiveness, APIs and model design.
The parent commissions work, carries completed evidence across gates, learns the
code and intended design itself, and defers major conclusions until all waves
return. Kinefolk's [commission](file:///home/rektide/src/kinefolk-rs/design/input-model/firmware/brief0.gpt6a.md)
owns the research questions and dispatch record. The commissioned research
sequence has completed; the [observation below](#observed-outcome-and-reflection)
distinguishes useful learning from acceptance or demonstrated runtime behavior.
The parent-authored draft was moved here from Kinefolk's
`design/input-model/firmware/launch-use-case0.gpt6a.md` (commit `067904e7`);
the original author and human commission remain in the provenance above.

## Flight dependency chart

Each rectangular node is an agent turn, diamonds are **all-of evidence joins**,
and rounded nodes are the commission and parent integration. Solid edges express
required sequencing, not authorization for additional work. Parallel branches are
independent within their wave. A returning-agent node resumes the named native
session; a fresh node starts a new session. Each bare node name resolves to its
local anchor below; the names are case-local and are not work tickets.

```mermaid
flowchart TD
    commission([commission])
    orientation-join{orientation-join}
    firmware-join{firmware-join}
    integration-join{integration-join}
    synthesis-join{synthesis-join}
    parent-integration([parent-integration])

    commission --> orient-terra & orient-g
    orient-terra & orient-g --> orientation-join
    orientation-join --> prewarm
    prewarm --> firmware-study
    firmware-study --> firmware-join
    firmware-join --> integration-astra & integration-terra & integration-g
    integration-astra & integration-terra & integration-g --> integration-join
    integration-join --> synth-returning-astra & synth-returning-g
    integration-join --> synth-fresh-astra & synth-fresh-terra & synth-fresh-g
    synth-returning-astra & synth-returning-g --> synthesis-join
    synth-fresh-astra & synth-fresh-terra & synth-fresh-g --> synthesis-join
    synthesis-join --> final-astra
    final-astra --> parent-integration
```

## Node meanings and execution semantics

<a id="commission"></a>
**commission:** the human requests the sequence and clarifies the external-study
versus Kinefolk-fit distinction; the product brief retains the detailed scope.

<a id="orient-terra"></a>
**orient-terra:** Terra max independently orients to Kinefolk.

<a id="orient-g"></a>
**orient-g:** G independently orients to Kinefolk in parallel.

<a id="orientation-join"></a>
**orientation-join:** both orientation reports must return and be read; F receives
both reports and the parent's explanation. Launching either cannot satisfy it.

<a id="prewarm"></a>
**prewarm:** F refreshes both QMK and ZMK and records repository locations,
refresh outcomes, immutable revisions, structure and useful entrypoints.

<a id="firmware-study"></a>
**firmware-study:** one dedicated Astra high studies both firmware projects in
their own terms, using the completed prewarm.

<a id="firmware-join"></a>
**firmware-join:** the firmware study and handoff provide a read and sufficient
common factual foundation before the integration wave starts.

<a id="integration-astra"></a>
**integration-astra:** a new Astra high independently investigates Kinefolk fit.

<a id="integration-terra"></a>
**integration-terra:** Terra medium independently investigates Kinefolk fit.

<a id="integration-g"></a>
**integration-g:** G independently investigates Kinefolk fit. All three examine
capabilities, expressiveness, APIs, model design and meta-layering; peer same-wave
outputs are withheld until synthesis.

<a id="integration-join"></a>
**integration-join:** all three integration reports must return and be read before
the five independent syntheses receive their common completed corpus.

<a id="synth-returning-astra"></a>
**synth-returning-astra:** resume the integration Astra for a synthesis.

<a id="synth-returning-g"></a>
**synth-returning-g:** resume the integration G for a synthesis.

<a id="synth-fresh-astra"></a>
**synth-fresh-astra:** a fresh Astra high synthesizes independently.

<a id="synth-fresh-terra"></a>
**synth-fresh-terra:** a fresh Terra high synthesizes independently.

<a id="synth-fresh-g"></a>
**synth-fresh-g:** a fresh G synthesizes independently. All five receive the
completed integration corpus, but not their concurrent siblings' syntheses.

<a id="synthesis-join"></a>
**synthesis-join:** all five synthesis artifacts must return and be read before
the final reconsideration; a completed turn is not itself evidence sufficiency.

<a id="final-astra"></a>
**final-astra:** resume the integration Astra for one further turn that may
examine all five perspectives, including its own synthesis.

<a id="parent-integration"></a>
**parent-integration:** integrate completed evidence and open model choices.
Parent learning from code and design proceeds during the run; major conclusions
wait for the completed sequence. Preserve disagreements and qualifications.

### Failure and partial results

These all-of joins require the named successful artifacts, not merely terminal
notifications. If a required turn fails or returns partial work, record the slot's
disposition and available artifact; the join remains closed unless the human
authorizes a partial release or changed cohort. Retry or replace only within the
authorized scope. Label any narrower evidence base and treat a later return as an
explicit addendum or new synthesis. Independent work whose own prerequisites are
met can continue. [Evidence gates](/assess/gates/README.md) distinguish arrival,
readiness and deliberate release; the parent judges sufficiency after reading.

## Artifacts and continuation identity

Each turn returns its owned document paths, evidence/revision pointers, check
results and commit. The parent records the returned native `sessionID` and uses
it to resume returning agents. Similar model names do not establish session
identity. Reports use model-suffixed filenames; shared navigation is parent-owned.

This use case specifies a coordination plan, not an executable graph runner.
Execution uses the harness's background-agent dispatch and completion
notifications. [Model lookup](/agents/models/README.md) explains the model and
variant names; neither this case nor that reference grants dispatch authority.

## Expected help before the flight

The gates should make the firmware prewarm relevant to Kinefolk, preserve a clear
external-system evidence base, and keep model recommendations from racing ahead
of the comparative inquiry. Returning and fresh sessions should expose both
continuity of reasoning and assumptions inherited from earlier turns.

The original assessment question was whether those expectations held: which
findings changed the parent's understanding, where synthesis found disagreement,
whether handoffs lost important context, and what model decisions remained open.
The presence of all expected documents alone would not establish the pattern's
value. [Case-story practice](/evidence/README.md#evidence-case-stories) supplies
the expected-versus-actual assessment; [flights](/praxis/flights/README.md)
owns reusable coordination, while the product brief owns the inquiry itself.

## Observed outcome and reflection

### Conduct and correction

The parent reports that the commissioned topology ran through the two-orientation
join, F prewarm, one dedicated Astra's study of both firmware projects, three
separate independent integration inquiries, five independent syntheses (two
returning and three fresh), and the final returning integration-Astra turn. All
required arrivals were read before dependent launches; there was no reported
same-wave peer leakage. The product [dispatch ledger](file:///home/rektide/src/kinefolk-rs/design/input-model/firmware/brief0.gpt6a.md#dispatch-record)
owns the sessions, commits, checks and exact sequence. A separate flights-orientation
and documentation branch supported the work independently; it was not a main
firmware gate.

Initially the parent collapsed external firmware study and Kinefolk-fit inquiry.
The human clarified that these were distinct stages **before either was
dispatched**. Restoring the separation corrected the orchestration design; it
was not a failed evidence gate. The result supplied a common external-system
basis before independent product-fit reasoning, as the expectation intended.

### Learning and evidence

The parent reports directly reading Kinefolk's `core/model` intent,
`core/apply` composition and `integration/workbench` preview, the earlier model
and capability synthesis, the full firmware comparison and key detailed
cache/edit sections, all three integration inquiries, all five syntheses, and
the [final reconsideration](file:///home/rektide/src/kinefolk-rs/design/input-model/firmware/final0.gpt6a.md).
Their acquired understanding is that delivered Kinefolk is a **configuration
preparation** workbench: it conjoins sparse requirements, retains a captured
native basis, composes first and plans once. Native source identity is not
runtime revision authority. This is the parent's reported learning from the
artifacts and code, not outside acceptance or proof of a new runtime.

The resulting candidate distinction separates current-identity activation for
future lookup from retained admitted-action cleanup: a held K can release the
original Left while a new K sees an edited Home. A named Nav bundle can coordinate
key and precision domains without one universal layer operator; definition and
stage-instance identity and multiplicity determine whether two bundles share one
half-scale stage or apply two. These are reasoned examples from the
[final proposal](file:///home/rektide/src/kinefolk-rs/design/input-model/firmware/final0.gpt6a.md#concrete-nav-plus-precision-example),
not demonstrated runtime behavior.

The dedicated source study corrected the prewarm's shorthand about release caches,
NVM and observer ordering. Integration and synthesis comparisons exposed and
corrected blanket VIA no-observation, host-synthesized CAS, retention-as-total-
ladder, unimplemented-executor exactness and overbroad native type-reuse claims.
The returning and fresh Astra syntheses independently identified a Studio metadata
restriction narrowing the two-axis edit probe; the final report preserves its
scope. These are tangible corrections and learning, not simply artifact production.
The five syntheses relied substantially on the same firmware evidence base, so
their agreement is not five behavioral verifications.

The final reconsideration also addressed a real dependency tension: an evaluator
for compound traces needs candidate composition laws before it can execute those
traces, while production representation can remain open. The source, evidence
and model artifacts equip further decisions; no firmware behavior was executed
and no runtime conformance or implementation was demonstrated by this flight.
Owner choices remain for APIs, semantic defaults, representation, capture,
executor and product priorities. None automatically commissions implementation.

### Continuity, cost and limits

The parent reports the dedicated external Astra ended around **350.1k** context
tokens. The returning integration Astra ended its first inquiry around **290.5k**,
its synthesis around **322.1k**, and the final continuation around **373k**.
That exceeds the preferred soft ~300k target, especially in the final continuation;
the human explicitly wanted the same Astra's final turn, and the parent accepted
coherent completion rather than interrupting at a numeric threshold. The agent
reported completing its comparisons without omission. These are reported context
measurements and a continuity tradeoff, not proof that all relevant evidence was
retained or that a new gate is needed. Returning sessions preserve reasoning
history while consuming context; fresh synthesis sessions offer independent
perspectives with a fresh context window.

This case supports the narrower observation that the flight exposed consequential
corrections and improved the parent's mechanical understanding. It does not yet
measure downstream adoption, validate the proposed semantics, or prove this
coordination pattern generally effective. Revisit when the owner chooses or tests
a concrete model or realization.
