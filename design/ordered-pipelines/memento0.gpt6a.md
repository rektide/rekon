---
type: ArchitectureMemento
title: Ordered capability pipelines — keep the dream, preserve the differences
description: A return point for reusable rule assembly, staged facts, termination authority and ordered contributions across distinct domains.
resource: /design/ordered-pipelines/memento0.gpt6a.md
tags: [composition, rules, pipelines, authority, provenance, watchwoman, sessions]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-12-06
extensions:
  ticket: rekon-session-mementos-composition
sources:
  - { resource: "urn:opencode:session:ses_f86cfeb14ffev9dX8mwiS0sZIm", title: Human request for a reusable-pattern memento and termination authority }
  - { resource: "file:///home/rektide/src/watchwoman-systemd-vcs-config/docs/CONFIGURATION.md", title: Implemented Watchwoman ordered filtering contract }
  - { resource: "file:///home/rektide/src/watchwoman-systemd-vcs-config/crates/watchwoman/src/daemon/view_filter.rs", title: Staged fact requests and classification }
  - { resource: /doc/expansion-heads.md, title: Historical Rekon expansion versus fallback inquiry }
  - { resource: /doc/flow-runtime-stage-plan.md, title: Historical stream-stage and reinjection design }
---

<a id="ordered-pipelines"></a>
# A memento, not a framework commission

The human sees a recurring shape: assemble individual instructions, run them in
order, let each add judgment or content, and make stopping explicit. Watchwoman
uses ordered filtering; the proposed session epilogue selects sessions then asks
session lookers for fields. Could there be a general deployable pattern for both?

The human also says **“Here Be Dragons”**: previous attempts have repeatedly
wrecked on this shore. The useful response is neither to build a universal engine
now nor to dismiss the idea until its motivating examples have been forgotten.
Keep concrete mementos of the dream, reasons for the differences, and an economical
way to return. `rekon-session-mementos-composition` is the review/alignment handle.

## The dream worth keeping

An operator can inspect an assembled program and answer:

- Which named contributions are active, in which order, and who supplied them?
- What fact does a contribution need, and does obtaining it cost IO?
- Did it decline, remove an item, emit content, defer, or request termination?
- Was it permitted to stop the enclosing computation, or only to give advice?
- What snapshot and clock made the decision reproducible?
- Can a different host deploy the same *kind of composition* without forcing
  unrelated domain facts into one giant context object?

The promising common grain may be an **assembly and explanation contract**, not
one evaluator. Typed domain operations could share ordered identity, provenance,
capability declarations and trace vocabulary while retaining different algebras.
This remains a hypothesis, not a new required package or serialization format.

## Verified Watchwoman precedent

The remembered checkout is
[`watchwoman-systemd-vcs-config`](file:///home/rektide/src/watchwoman-systemd-vcs-config/README.md),
not `systemd-watchwoman`. Related `watchwoman-systemd` and
`watchwoman-systemd-production-lifecycle` variants exist; the VCS-config variant
contains the relevant newer rule implementation inspected here.

Its [configuration contract](file:///home/rektide/src/watchwoman-systemd-vcs-config/docs/CONFIGURATION.md#rule-priority)
separates mode-source precedence from effective rule priority. Machine rules,
root-local rules, cookie built-ins and a chosen VCS profile compile into an
ordered list. The **first complete matching rule wins**, with implicit allow.

Its [classifier](file:///home/rektide/src/watchwoman-systemd-vcs-config/crates/watchwoman/src/daemon/view_filter.rs#L317-L349)
can return `Need(Kind)` or `Need(Metadata)` before it returns a disposition. A
missing expensive fact is not a failed predicate that allows a later fallback to
win. The scanner owns obtaining the fact; pure policy owns deciding what it needs.

The disposition has two distinct axes: **track this node** and **descend through
this directory**. A denied directory can be traversed to reach a higher-priority
allowed descendant without itself appearing in results. The
[design assessment](file:///home/rektide/src/watchwoman-systemd-vcs-config/.design/opencode/vcs-config2-assessment0.gpt56s.md#central-correction-one-policy-two-axes)
documents why the earlier Boolean and “later rule” formulations were wrong.

This is real reusable learning: preserve the distinctions callers otherwise
have to reconstruct. It is not evidence that all ordered processing has
Watchwoman's first-match or descendant-feasibility semantics.

## Different composition laws, similar surface shape

| Pattern | What an item means | How results combine | What stops |
|---|---|---|---|
| First-match policy | One candidate plus staged facts | First authoritative matching disposition wins. | Rule scan for that candidate; tree traversal is a separate decision. |
| Sequential narrowing | Candidate stream | Each stage consumes the preceding stage's output. | Authorized stage may end upstream enumeration. |
| Fallback lookup | Attempts to answer one question | First satisfactory answer, with explicit decline/unavailable/error meanings. | Remaining attempts, not unrelated outputs. |
| Ordered lookers | Selected session and available content | Contributions accumulate in declared order. | A field or output budget may stop collection; success alone does not. |
| Expansion and verification | Competing interpretations | Union/generation first, then qualification; potentially many valid results. | Consumer policy or bounded search, not arbitrary first producer ownership. |
| Effectful work plan | Actions with dependencies | State transitions and explicit failure policy. | Cancellation or execution policy; stopping has cleanup consequences. |

The word *chain* does not choose among these. “Chain of command” describes
authority; the conventional “chain of responsibility” often means first handler
wins. Neither automatically describes an accumulating sequence of lookers.

## Termination is a request plus a grant

For the immediate epilogue, the human specified:

> Filters declare whether they have terminating authority. Ignore a termination
> signal from a filter that does not have that authority.

Keep three questions separate:

1. **Disposition:** keep or drop this session?
2. **Request:** does the filter recommend stopping the stream here?
3. **Authority and validity:** may it stop this stream, and does its ordering
   assumption justify the request?

An unauthorized `drop + stop` still drops this item and continues. An authorized
`keep + stop` may emit this final item and then close the upstream iterator. A
stage-local `take(3)` counts its own input, so `recent → take(3)` and
`take(3) → recent` intentionally have different results.

For a newest-active-first source, finding an older-than-2d session can prove the
remaining idle sessions are also too old. In arbitrary tab order, it proves
nothing about the remainder. **A permission bit does not turn an invalid proof
into a valid one.** The domain must supply the ordering guarantee or withhold the
request; a future planner could verify it, but no planner is needed to explicitly
guarantee one order now.

The grant is operator/host composition policy, not an assertion a plugin can
self-authorize by returning a special value. Diagnostics should distinguish
“requested but ignored” from “honored.” A general trace database is not required.

## Why selected sessions and lookers stay separate

Select cheaply before gathering expensive content. The epilogue needs to decide
which sessions belong to the output before extracting loaded Q/response text or
reading a future bookmark store. A looker contributes a named semantic field;
the host owns safe bounded rendering and freezes plain data before shutdown.

“Last activity,” “last question,” “response excerpt,” and “last marked summary”
are different facts. No looker should fetch or manufacture a supposedly final
answer merely because its neighboring field was absent. An optional future
fallback looker could explicitly prefer a marked summary over a response excerpt;
ordinary ordered accumulation must not acquire that behavior by accident.

## Dragons to recognize on return

- **One sparse context object.** Every consumer learns every unrelated fact and
  every possible absence. Prefer typed domain facts and named fact needs.
- **One Boolean result.** It loses unknown/unavailable, traversal, decline,
  partial output, and stop authority—the differences that mattered in practice.
- **One order.** Config precedence, evaluation priority, candidate recency and
  presentation order are not interchangeable. Changing one must not quietly
  reorder the others.
- **Optimization changing meaning.** Reordering stages, treating cache absence as
  false, or stopping on unsorted activity silently removes valid results.
- **Registration becoming a programming language.** Arbitrary callbacks, loops,
  reinjection, retries and effects need lifecycle/cancellation semantics. A
  generic registry is not proof that those semantics exist.
- **Cross-domain package extraction too soon.** Rust filesystem traversal and
  TypeScript reactive terminal presentation cannot meaningfully share code just
  because both contain a loop over rules.
- **Infinite preservation without trials.** Mementos should supply falsifiable
  examples, not expand forever into an architecture nobody exercises.

## A small future trial

Do not change either production module to conduct this trial. Given one recorded
Watchwoman classification and one session-selection run, try to describe both
with the same *assembly/trace headings*: ordered contribution identity, origin,
required facts, disposition, requested stop, granted stop, snapshot, output.

Then ask whether that common description improves debugging without erasing
`track/descend` or inventing a first-wins meaning for lookers. If the common
description helps but a common evaluator needs domain-tagged branches everywhere,
share the descriptive protocol or testing recipe—not the evaluator. Two real
consumers and a concrete deletion test should precede a deployable package.

Possible topic-first homes if this earns implementation:

- **Domain-owned modules** with shared documentation/conformance fixtures: least
  coupling; current recommended path.
- **Composition-domain package** containing assembly, provenance and traces,
  with separate policy, stream and contribution interpreters: only if multiple
  implementations actually reuse it.
- **Host-integration grouping** with typed domain adapters: useful only when
  deployment across real hosts, rather than conceptual similarity, is the need.

## Cross-references

- [Session checkpoint](/design/session-mementos/checkpoint0.gpt6a.md) — immediate
  application and implementation gate; the generic inquiry does not block it.
- [Historical expansion-heads inquiry](/doc/expansion-heads.md) — especially useful
  counterexample: first-producer exclusivity destroyed intended multi-candidate
  expansion. Its code claims are historical, not verified current Rekon runtime.
- [Historical flow-runtime plan](/doc/flow-runtime-stage-plan.md) — explicit stream
  composition and reinjection, showing where a filter becomes an execution model.
- [Historical preliminary execution design](/doc/dl-preliminary-execution-design.md)
  — known/unknown facts, dependency scheduling and ordered reporting. Supporting
  design vocabulary only; not evidence these modules exist in this checkout.
- [Phoenix inquiry](/design/rekon/phoenix0.gpt6a.md) — keeping the intent, reasons
  and experiments when a particular expression is replaced.
- [Skillification](/design/skillify/README1.sol56x.md#skillify-not) — a deployable
  capability is not synonymous with a skill or a universal container.
