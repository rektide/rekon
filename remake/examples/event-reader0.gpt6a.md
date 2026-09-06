---
type: WorkedExample
title: Remaking a bounded event reader — an illustrative sequence
description: A fictional example separating learning order, patch order, and stage-specific architectural assessment; no implementation or checks were performed.
resource: /remake/examples/event-reader0.gpt6a.md
tags: [remake, sequencing, assessment, example, state-ownership]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
extensions:
  ticket: rekon-phoenix
sources:
  - { resource: /remake/design/brief0.gpt6a.md, title: Remake commissioning question }
  - { resource: /code/task-reconstruction/philosophy.md#reconstruct-landing, title: Dependency-ordered coherent changes }
  - { resource: /assess/review/form1.glm53.md, title: Architectural assessment and host-boundary evidence }
---

# A bounded event reader

**Fictional example, not a NAN or Lastact assessment. No candidate patches
exist and no checks have run.** It illustrates the proposal shape; it cannot
validate the practice by itself.

## Destination and constraints

Suppose a developed library reads records from a host event stream and gives
callers a bounded current-state view. Accepted requirements are: malformed
records cannot mutate state; callers can detect lost continuity; recovery can
restore a known current state; closing a reader stops its input task.

The proposed architecture has a decoder, a deterministic state owner, a host
adapter, and a small reader facade. This example assumes that assessment has
found those boundaries useful. It does not infer them from an old file tree.

Two host facts remain unknown: whether reconnection supplies a usable resume
position, and whether the host can supply a snapshot with an ordering boundary
that lets the reader safely resume event consumption. Neither a local fake nor
the name `resume()` establishes those contracts.

## Learning before landing

Investigate those host contracts before committing to the recovery model.
An import-only inspection might settle them; if not, propose an explicitly
authorized isolated host experiment. Preserve its source, observations, and
limits. A throwaway probe is not automatically patch 1 of the production
series.

If neither usable replay nor an ordered snapshot exists, the desired recovery
promise cannot currently be justified. Stop and change the proposed contract
with the owner; do not hide the gap behind an endlessly retrying adapter.

For the following **conditional** sequence, assume an ordered snapshot contract
has been established. This is an example premise, not an observed result.

## Candidate landing sequence

| Slice | Capability and owning boundary | Main architectural question | Proposed evidence and ceiling |
|---|---|---|---|
| `decode` | Decode host records into typed domain inputs; retain useful unknown fields and reject malformed input. | Does the decoder isolate host grammar without deciding state or recovery policy? | Pinned independently derived records, malformed/truncated cases, no state mutation. Proves representation only. |
| `state` | Apply typed inputs in a bounded deterministic owner; expose loss of continuity and admit a replacement snapshot. | Who owns truth after overflow, and can a caller distinguish unknown from empty? | Explicit overflow, duplicate, invalid-input, and snapshot-replacement traces. Proves local transitions, not host ordering. |
| `adapter` | Drive that owner from the host; reconnect, obtain an ordered snapshot when necessary, and dispose of the input task. | Is there exactly one recovery owner, and does the real host satisfy the ordering assumption? | Host-backed interruption/recovery and shutdown checks against a named version. Decoder and reducer tests alone cannot close this gate. |
| `reader` | Expose and document the supported caller contract through the composed system. | Does the facade hide transport mechanics without hiding loss of continuity? | Public-API scenario: read → interrupt → observe unknown → recover → read → close. Assert task termination and the stated resource bound. |

Hard code/contract dependencies are `decode → state → adapter → reader` for
this chosen decomposition. The host-contract investigation is an **evidence
dependency of selecting the recovery design**, not a production-code node.

Each proposed patch includes its focused tests and honest documentation. The
state-owner patch must already define boundedness and overflow behavior;
the plan does not defer those responsibilities to a final hardening patch.
The facade may stay unexported until composition is available. That limits
intermediate promises without making the earlier modules untestable.

## A worked checkpoint: state ownership

Before `state`, the system can explain bytes but cannot claim a current view.
After `state`, one owner can explain whether its retained state is known,
unknown, or being replaced. Recovery triggers belong to that owner; performing
host I/O belongs to the adapter.

The attractive alternative is to put overflow handling in the adapter, where
the stream already lives. That reduces the reducer's vocabulary, but allows
fake input and future adapters to disagree about what state loss means.
Keeping the transition in the deterministic owner makes that invariant
testable without a host. The adapter still chooses no independent meaning for
"current."

The price is an explicit continuity state in the domain interface. If source
inspection instead established a host abstraction that already owns bounded
retention and ordered recovery for every supported caller, that could change
the placement decision. This is an ownership judgment, not a preference for
more state machines.

**Assessment result here:** proposed and conditional; no prefix is verified.
The stage-local proof cannot establish that a real reconnect obeys the snapshot
boundary. That remains the adapter gate, and changes to the adapter's ordering
assumptions reopen the state-owner assessment.

## What would change the order?

- If host feasibility were the largest uncertainty, investigate it first while
  retaining the production landing sequence above.
- If an existing decoder and state owner already met the contracts, omit their
  replacement patches and cite the pinned baseline capabilities instead.
- If the facade's shape were the major uncertainty, sketch a consumer tracer
  early. Do not export a placeholder public API merely to reproduce that
  learning order in the landing series.
- If two owners needed different upstream submissions, use separate series
  with an explicit compatible-revision rendezvous. A single numbered list
  would conceal that coordination requirement.

## Cross-references

- [Remake](/remake/README.md) **owns the method** illustrated here.
- [Sequence and checkpoint form](/remake/sequence/form0.gpt6a.md) **provides
  the reusable report shape**, including provenance and verification status.
- [Review form 1](/assess/review/form1.glm53.md) **deepens the host and recovery
  inspection** when a real adapter exists; this example supplies no substitute
  runtime evidence.
