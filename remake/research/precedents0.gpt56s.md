---
type: ResearchReport
title: Existing Rekon practices that bound remake
description: Source-grounded ownership map, precedents, tensions, and absences relevant to a reusable practice for defensible landing order and stage-specific architectural assessment.
resource: /remake/research/precedents0.gpt56s.md
tags: [remake, research, assessment, reconstruction, sequencing, architecture]
status: draft
generated: { by: "model:openai-gpt-5.6-sol-high", at: 2026-09-06 }
stale_after: 2027-03-06
extensions:
  session: urn:opencode:session:ses_f86f5471bffemEW3v8DYYMIY3p
  scope: Rekon practice precedents only; no product assessment, method definition, implementation, tickets, or peer remake documents
sources:
  - { id: assess-home, resource: /assess/README.md, title: Assessment — become well equipped to judge and act, author: "model:gpt-6-astra" }
  - { id: evidence-gates, resource: /assess/gates/README.md, title: Evidence gates — wait until well equipped, author: "model:gpt-6-astra" }
  - { id: review-form-0, resource: /assess/review/form0.gpt6a.md, title: Architectural and close review — form 0, author: "model:gpt-6-astra" }
  - { id: review-form-1, resource: /assess/review/form1.glm53.md, title: Architectural and close review — form 1, author: "model:glm-5.3" }
  - { id: reconstruction-philosophy, resource: /code/task-reconstruction/philosophy.md, title: Reconstruct From The Present, author: "model:openai-gpt-5.6-sol" }
  - { id: reconstruction-procedure, resource: /code/task-reconstruction/README.md, title: Task reconstruction with docs preservation — the jj procedure, author: "model:zai-coding-plan/glm-5.3" }
  - { id: planning, resource: /code/planning.md, title: Planning rules, author: "model:zai-coding-plan/glm-5.3-max" }
  - { id: commits, resource: /code/commits.md, title: Commit discipline with jj, author: "model:zai-coding-plan/glm-5.3-max" }
  - { id: review-handling, resource: /code/review-handling.md, title: Handling review comments, author: "model:zai-coding-plan/glm-5.3-max" }
  - { id: documentation-constitution, resource: /constitution/README.md, title: Self-Explaining Documentation Constitution, author: "model:gpt-5.6-terra" }
  - { id: architectural-stewardship, resource: /AGENTS.md, title: Rekon workspace guidance, author: "human:rektide" }
  - { id: jj-reference, resource: /vcs/jj/README.md, title: jj + git working guide, author: "model:zai-coding-plan/glm-5.3-max" }
  - { id: phase3-calibration, resource: "https://github.com/rektide/dl/blob/e9c7c401725a93d2c4d56f41851fdb832d772b09/doc/phase3.md", title: "Phase 3: Actions Become Stages", author: unspecified }
---

<a id="remake-precedents"></a>

# Existing Rekon practices that bound `remake`

## Scope and evidence boundary

This report investigates existing Rekon primary documentation for ownership,
precedent, tension, and missing distinctions around a possible reusable
`remake` practice. The target concern is narrow: choosing a defensible
build/patch landing order and assessing each resulting stage architecturally
against high-level concerns. It does **not** define that practice, assess any
product, prescribe code changes, reconstruct a branch, or authorize work. No
other document under `remake/` was read.

Claims below are source-derived from the files listed in frontmatter. Line
ranges identify the inspected wording; explicit anchors identify durable
concept owners where available. “Missing” means absent from this inspected
Rekon practice surface, not proven absent from every historical or external
artifact.

<a id="remake-precedents-ownership"></a>

## Actual ownership boundaries

| Concern | Existing owner | Relationship to a future `remake` practice |
|---|---|---|
| Judge a whole system or exact diff against intent, architecture, contracts, tests, and evidence | [`assess/`](/assess/README.md#assess), especially [review form 1](/assess/review/form1.glm53.md#L53-L109) | `remake` should **consume or specialize** this assessment vocabulary, not become a second general review protocol. Assessment explicitly is not a rewrite license ([`assess/README.md` lines 19–27](/assess/README.md#L19-L27)). |
| Decide what a historically informed implementation should become now | [Reconstruction philosophy](/code/task-reconstruction/philosophy.md#reconstruct-job) | `remake` should **inherit** mode classification, present-baseline authority, and “history as evidence, not gravity”; it should not silently choose clean rework or replacement. |
| Derive green commit slices from a selected destination | [Plan the landing](/code/task-reconstruction/philosophy.md#reconstruct-landing) | This is the closest direct precedent. `remake` may **extend** its ordering concern with stage-specific architectural judgment, but should not reverse its dependency: destination first, sequence second ([lines 395–415](/code/task-reconstruction/philosophy.md#L395-L415)). |
| Preserve documentation history while restarting a code line | [Task-reconstruction jj procedure](/code/task-reconstruction/README.md#L37-L69) | `remake` should **delegate** snapshot, duplicate, split, replay, timestamp, and bookmark mechanics. That procedure implements a specific docs-preservation constraint, not general patch-order semantics. |
| Inspect branch topology, patch equivalence, merge bases, and rewrite mechanics | [`vcs/jj/`](/vcs/jj/README.md#L39-L63) | `remake` should **use as evidence tooling**. These commands can establish ancestry or content relationships, not architectural justification. |
| Express ordinary plan constraints and work dependencies | [Planning rules](/code/planning.md#planning) and [Balanced Ticket Coordination](/constitution/README.md#doc-constitution-tickets) | `remake` should **coordinate with**, not absorb, planning and tickets. Plans use sequence/dependency rather than calendars ([`planning.md` lines 24–28](/code/planning.md#L24-L28)); tickets own lifecycle, blockers, and acceptance while documents own concepts and evidence ([constitution lines 649–659](/constitution/README.md#L649-L659)). |
| Make and explain ordinary commits | [Commit discipline](/code/commits.md#commits) | `remake` should **inherit** [logical grouping](/code/commits.md#L22-L24) and [honest review surfaces](/code/commits.md#L52-L60), but this owner does not choose a multi-commit architecture. |
| Respond after review feedback arrives | [Review handling](/code/review-handling.md#review-handling-protocol) | `remake` should **hand off to**, not subsume, correction handling. Comments require conversation before edits and one cohesive correction per accepted response ([lines 24–33](/code/review-handling.md#L24-L33)). |

The corpus therefore leaves a plausible unowned seam: no inspected canonical
document joins destination-derived landing order to an architectural account
of what each intermediate stage establishes, constrains, preserves, or leaves
deliberately incomplete.

<a id="remake-precedents-sequencing"></a>

## Sequencing precedents and tensions

### Destination order is not historical order

The strongest existing rule is explicit: design the destination before its
commit sequence ([reconstruction lines 128–140](/code/task-reconstruction/philosophy.md#L128-L140)), then derive commits from final modules and dependencies rather than the path by which the old implementation evolved
([lines 399–415](/code/task-reconstruction/philosophy.md#L399-L415)). History is
mined by claim rather than chronology ([lines 185–201](/code/task-reconstruction/philosophy.md#L185-L201)). This supports a defensibility test for a proposed order without turning the exercise into a tidy replay.

There is an important interpretive limit. “Chronology does not prove the choice
is still appropriate” ([lines 199–201](/code/task-reconstruction/philosophy.md#L199-L201)) is not a claim that the historical choice was irrational in its original constraints. The same guide says chronology explains why people made a choice, while historical incidents and negative knowledge remain authoritative evidence ([lines 203–227](/code/task-reconstruction/philosophy.md#L203-L227)). A present preferred order and an account of past decision quality are therefore different claims. The corpus contains this distinction in substance but does not name a standard reporting field for it.

### “Green and coherent” is necessary but underspecified architecturally

Reconstruction says commits should be green, behaviorally coherent,
final-form for introduced modules, reviewable through explicit interfaces,
separable when ownership/upstreamability/rollback differ, and free of
add-then-delete scaffolding ([lines 404–415](/code/task-reconstruction/philosophy.md#L404-L415)). Commit discipline adds that each commit must be a logical unit
([lines 22–24](/code/commits.md#L22-L24)) whose message and diff explain what
changed and why ([lines 52–60](/code/commits.md#L52-L60)).

These are strong slice properties, but they do not say how to assess the
architecture of an intentionally intermediate stage. In particular, the
inspected sources do not define whether a stage is judged against its own
declared capability, the eventual destination, or both; how later-owned
behavior is marked as absent rather than defective; or which final invariants
must already hold from the first landing.

### Checkpoints follow decisions, not filenames

Architectural stewardship requires checkpoints around consequential decisions,
with review of emerging interfaces, domain concepts, reuse, tradeoffs, and
evidence before authorization of the next coherent slice ([`AGENTS.md` lines
12–19](/AGENTS.md#L12-L19)). Reports should explain what became possible or
impossible, choices and rationale, unresolved tensions, and the proposed next
slice ([lines 16–17](/AGENTS.md#L16-L17)). This is the clearest precedent for
stage-specific architectural attention.

It also resists a mechanical “review every file-shaped step equally” rule.
Checkpoint placement follows architectural consequence, while commit boundaries
may additionally follow ownership, rollback, and upstreamability. Those
topologies can overlap without being identical.

<a id="remake-precedents-assessment"></a>

## Assessment precedents and limitations

Review form 1 provides reusable lenses: start from intent, map real and
substituted seams, follow state/ownership/external-operation/recovery paths,
resolve authoritative host evidence per seam, distinguish baseline defects
from cross-generation compatibility findings, and map claims to testimony,
tests, and source ([conduct, lines 70–109](/assess/review/form1.glm53.md#L70-L109);
[deliverable, lines 122–173](/assess/review/form1.glm53.md#L122-L173)). Its finding taxonomy separates defects, contract mismatches,
architectural pressure, coverage gaps, and open questions, with impact and
confidence independently reported ([lines 142–163](/assess/review/form1.glm53.md#L142-L163)). These distinctions can prevent architectural concern from being inflated into failure.

However, the form commissions either “the system as it stands” or an exact
change range ([lines 53–68](/assess/review/form1.glm53.md#L53-L68)); it does not
model a sequence of related partial systems. Its host-basis matrix resolves
dependency generations, not stage ancestry, intended per-stage capability, or
which omissions belong to later slices. Applying it unchanged at every stage
would risk repetitive whole-system reviews or false findings against an
unfinished endpoint.

Its authorization boundary is directly reusable: proposals are preserve,
fix/verify, watch, or defer; findings are not implementation authorization
([lines 175–181](/assess/review/form1.glm53.md#L175-L181)). The assessment home
also assigns responding to feedback to review handling and forbids assessment
from silently entering correction mode while evidence arrives
([`assess/README.md` lines 55–65, 79–81](/assess/README.md#L55-L65)).

Evidence gates add another useful but bounded distinction: **arrival** of
inputs is not **readiness** to judge them ([`gates/README.md` lines 50–62](/assess/gates/README.md#L50-L62)). Gates protect evidence sufficiency; they are
not, by themselves, a quality rubric, merge approval, or claim that passing
tests settles architecture. The coordinator retains sufficiency and next-slice
authorization ([lines 68–77](/assess/gates/README.md#L68-L77)).

<a id="remake-precedents-work-planes"></a>

## VCS, plans, and tickets are different work planes

VCS inspection can establish whether a series is truly stacked, whether a
patch is equivalent, and which merge base makes a diff meaningful. The jj
reference warns that patch identity misses squash-and-rename consolidation and
that tip-to-tip diffs across drifted bases create false catastrophes
([`vcs/jj/README.md` lines 84–130](/vcs/jj/README.md#L84-L130)). These
are evidentiary prerequisites for honest assessment, not reasons for an order.

The docs-preservation replay is even narrower: it preserves every docs commit
and author date on a fresh upstream base while leaving code history as evidence
([`task-reconstruction/README.md` lines 37–69](/code/task-reconstruction/README.md#L37-L69)). Treating that executable procedure as the general meaning of
“remake” would collapse architectural reconstruction into one carrier mechanic.

Tickets likewise do not map one-to-one to patches. The constitution states
that one ticket can update several sections, one section can support several
tickets, and either may exist without the other ([lines 647–659](/constitution/README.md#L647-L659)). Ticketing earns its cost where dependencies,
acceptance, ownership, risk, or cross-session lifecycle matter
([lines 661–680](/constitution/README.md#L661-L680)). A related Rekon design
precedent uses epics for a domain and children for “independently acceptable
tracer slices,” while warning against ticket-per-paragraph parity
([`agents-modularization0.sol56x.md` lines 445–462](/design/rekon/agents-modularization0.sol56x.md#L445-L462)). This supports coordination without making ticket topology the patch topology by definition.

<a id="remake-precedents-absences"></a>

## Notable absences in the inspected corpus

1. **No integrated series object.** There is no canonical artifact contract
   that pins a destination, ordered stage baselines, stage intent, dependency
   rationale, expected partial capabilities, and final acceptance in one
   referentiable account.
2. **No stage-relative architectural rubric.** Review lenses exist, and
   checkpoint questions exist, but no source defines how their depth changes
   for a foundation patch, behavior patch, integration patch, deletion patch,
   or other intermediate role—or whether such a role taxonomy should exist.
3. **No explicit two-baseline judgment.** Existing review distinguishes runtime
   and dependency generations, but not “is this stage sound for what it claims
   now?” from “does it advance coherently toward the accepted destination?”
4. **No named anti-retrospective-blame distinction.** Reconstruction demotes
   chronology from authority while retaining it as explanatory evidence, but
   there is no explicit reporting convention separating “prefer a different
   landing now” from “the original development order was wrong then.”
5. **No series-level completion proof.** Per-commit green checks and final
   outcome validation are prescribed, but the sources do not define evidence
   that the ordered series preserves key invariants throughout or that the
   final tree realizes the destination without hidden dependence on discarded
   scaffolding.

<a id="remake-precedents-relationships"></a>

## Candidate relationship types for later documentation

These are ownership-safe cross-reference descriptions, not a proposed method:

- `remake` **inherits** present-baseline and history-as-evidence constraints from
  [Reconstruct From The Present](/code/task-reconstruction/philosophy.md#reconstruct).
- `remake` **extends** the destination-derived landing-slice precedent at
  [Plan The Landing](/code/task-reconstruction/philosophy.md#reconstruct-landing)
  with an otherwise missing series/stage architectural concern.
- `remake` **consumes** assessment lenses and evidence maturity from
  [review form 1](/assess/review/form1.glm53.md) and the
  [constitution](/constitution/README.md#doc-constitution-references-evidence),
  without redefining general review.
- `remake` **coordinates with** evidence gates and architectural checkpoints;
  neither automatically approves the next landing.
- `remake` **delegates** history interrogation and surgery to [`vcs/jj/`](/vcs/jj/README.md)
  and the specialized [docs replay procedure](/code/task-reconstruction/README.md).
- `remake` **complements** plans and tickets: it may explain one landing series
  while those owners retain work lifecycle, dependencies, and acceptance.
- `remake` **hands accepted feedback to** [review handling](/code/review-handling.md#review-handling)
  rather than editing merely because an assessment raised a concern.

A neighboring `digest` action, if defined elsewhere, would have a supportable
relationship without changing this ownership map: it could **supply** a
recency-aware present-state and intent-front input, while `remake` remains
responsible for reasoning about a proposed landing sequence. This follows the
reconstruction requirement to inspect the current baseline before deciding
what to carry ([lines 169–183](/code/task-reconstruction/philosophy.md#L169-L183))
and the constitution's deliberate preservation of both coherent current truth
and exact historical evidence ([lines 753–780](/constitution/README.md#L753-L780)).
Those sources support a relationship, not a new evidence store or permission
for this report to define `digest`.

<a id="remake-precedents-cautions"></a>

## Concrete cautions for a reusable practice

1. **Do not let historical commits choose the destination—or use a present
   cleaner order to condemn historical actors.** Preserve original rationale
   as evidence, then justify the current order against current constraints.
2. **Do not assess an intermediate stage as though it claims the final feature.**
   Without a stage-scoped intent and exact baseline, ordinary review categories
   will manufacture defects from deliberate incompleteness.
3. **Do not confuse green checks, evidence readiness, architectural approval,
   human acceptance, and implementation authorization.** Rekon repeatedly
   assigns these signals to different owners.
4. **Do not make commit, checkpoint, ticket, and VCS ancestry graphs identical
   by fiat.** Each encodes a different concern; correspondence must be explained
   where it matters rather than assumed globally.

The strongest empirical local precedent is [`phase3.md` in rektide/dl](https://github.com/rektide/dl/blob/e9c7c401725a93d2c4d56f41851fdb832d772b09/doc/phase3.md#L18-L62): it calibrates completed work against intended outcomes, records “same goal, different mechanism,” and distinguishes delivered, partial, uncertain, and deferred claims without rewriting the earlier plan as foolish. It is a worked product-specific example, not yet reusable `remake` ownership.
