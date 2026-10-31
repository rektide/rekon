---
type: UseCase
title: Kinefolk firmware research flight — gated launch pattern
description: Concrete agent dependency graph with parallel orientation, prewarming, external-system research, Kinefolk-fit inquiry, independent syntheses and a final returning-agent turn.
resource: /praxis/flights/kinefolk-firmware0.gpt6a.md
status: commissioned
generated:
  by: model:openai/gpt-6-astra#high
  at: 2026-10-31
sources:
  - resource: opencode://ses_e6c978f10ffeTGkK163WA0Ovly
    title: Human requests this launch pattern as a Mermaid dependency-chart use case and clarifies external research versus Kinefolk integration research
  - resource: opencode://ses_e6c900557ffex5YjEpZrQCueNC
    title: Relocate the parent-authored case into flights, align diagram names and joins with native praxis, and repair cross-repository navigation
  - resource: "file:///home/rektide/src/kinefolk-rs/design/input-model/firmware/brief0.gpt6a.md"
    title: Owning research commission and clarified wave sequence
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
owns the research questions and current dispatch record. This is a commissioned
flight and case seed, not a claim that the full sequence has run or succeeded.
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

## Expected help and later assessment

The gates should make the firmware prewarm relevant to Kinefolk, preserve a clear
external-system evidence base, and keep model recommendations from racing ahead
of the comparative inquiry. Returning and fresh sessions should expose both
continuity of reasoning and assumptions inherited from earlier turns.

At completion, record whether those expectations held: which findings changed
the parent's understanding, where synthesis found real disagreement, whether
handoffs lost important context, and what model decisions remain open. The
presence of all expected documents alone does not establish the pattern's value.
The [case-story practice](/evidence/README.md#evidence-case-stories) supplies the
expected-versus-actual assessment; this is a prospective case seed until outcomes
and use are recorded. [Flights](/praxis/flights/README.md) owns the reusable
coordination practice, while the product brief owns the inquiry itself.
