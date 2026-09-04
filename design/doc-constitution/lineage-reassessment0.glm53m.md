---
type: LineageReassessment
title: Predecessor contract reassessment for the broader constitution
description: Read-only survey of which constitutional contracts emerged across is-tree, dolt-rs, first-compute, execsnoop-rc, and opencode-otel, organized as threads; which contracts the maintained constitution absorbs, which are missing or disconnected, and which belong only in procedures or future skills. Surfaces the under-codified thinking-and-talking-in-Markdown thread and the fragmentation of identity into subprinciples.
resource: /design/doc-constitution/lineage-reassessment0.glm53m.md
tags: [rekon, constitution, lineage, evidence, synthesis, waves, identity, markdown-practice]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-09-04T00:00:00-04:00 }
sources:
  - id: rekon-constitution
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution
    author: model:gpt-5.6-terra
  - id: is-tree-v1
    resource: file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md
    title: System-state research document constitution
    author: model:gpt-5.6-terra
  - id: is-tree-v2
    resource: file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md
    title: Reconciliation and synthesis amendment
    author: model:gpt-5.6-terra
  - id: dolt-rs
    resource: file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md
    title: Dependency-Ordered Research Wiki Constitution
    author: model:gpt-5.6-terra
  - id: first-compute-v1
    resource: file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md
    title: Goal-directed compute research document constitution
    author: model:gpt-5.6-terra
  - id: first-compute-v2
    resource: file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md
    title: Foundation-wave sealing and fair-synthesis amendment
    author: model:gpt-5.6-terra
  - id: execsnoop-rc
    resource: file:///home/rektide/src/execsnoop-rc/.design/constitution/constitution.glm53.md
    title: execsnoop-rc topic-document constitution (r2 registers CAPT)
    author: model:glm-5.3
  - id: opencode-otel
    resource: file:///home/rektide/src/opencode-otel/.design/otel/research/topic-document-constitution/topic-document-constitution.gpt56t.md
    title: OpenCode OTEL planning research constitution
    author: model:gpt-5.6-terra
  - id: naming-draft3
    resource: /constitution/naming/exploration/draft3.sol56x.md
    title: Rekon core draft 3
    author: integrator/gpt-5.6-sol-max
  - id: workspace-agents
    resource: /AGENTS.md
    title: Workspace agent guidance (wave, addendum, doc-pass, commit conventions)
    author: human:rektide
  - id: lineage-ticket
    resource: /.beads/issues.jsonl
    title: rekon-doc-constitution-lineage close record
    author: human:rektide
---

<a id="lineage-reassessment"></a>
# Predecessor Contract Reassessment

Read-only survey for the parent's broader `/constitution/README.md` candidate.
It answers: which constitutional contracts emerged across the predecessor
projects, which the maintained
[`/constitution/README.md`](/constitution/README.md) already absorbs, which
are missing or disconnected, and which belong only in project procedures or
future skills. Per the current steer, it is organized as **threads** to
re-find after the naming-heavy period, not as a classification of known
modules. No tickets were created or modified; every work-item shape below is
hypothetical for the parent to integrate. No existing files, code, external
repositories, or tracker state were touched.

The steer also asked whether "thinking / talking in Markdown" is a core
constitutional principle that remains poorly codified. Thread A takes that
question seriously and finds substantial predecessor evidence, without letting
it erase the other threads (evidence authority, wave mechanics, topology,
validation).

<a id="lineage-reassessment-scope"></a>
## Scope, Method, Absences

Read in full: the rekon constitution and its design record, is-tree v1/v2,
dolt-rs, first-compute v1/v2, execsnoop-rc r1/r2, opencode-otel, naming
draft3, the `/AGENTS.md` wave/addendum/doc-pass/commit conventions, and the
closed `rekon-doc-constitution-lineage` beads record. first-compute's two
constitution files are substantive adaptations of is-tree's, not copies —
verified by diff, not assumed: v1 re-frames around a cross-project
goal-directed-compute question and adds terminology discipline, comparison
axes, viability rubrics, and a lineage baseline; v2 is an independently
authored amendment about wave sealing and fair synthesis.

**Absences kept deliberate:**

- `chrome-agent-platform-fox/docs/CONSTITUTION.md` (125 lines, skimmed head
  only) is the word origin of "constitution" in this workspace but governs
  product robustness — security, permissions, CSP — not documents. The lineage
  ticket reached the same conclusion; it is vocabulary ancestry, not a
  document-constitution ancestor.
- `/constitution/reassessment0.gpt6a.md` and any same-round parent output were
  not read, so this survey's independence from the parent's synthesis is
  preserved.
- One lifecycle observation worth recording: execsnoop-rc revision 2 is the
  only predecessor constitution marked `status: stable`
  ([constitution2.glm53.md:7](file:///home/rektide/src/execsnoop-rc/.design/constitution/constitution2.glm53.md));
  every other lineage member remains `draft`. The rekon constitution itself
  reserves stable status for "evidence from real modules and an operable
  assembly/validation path, not model agreement alone"
  ([/constitution/README.md:819-822](/constitution/README.md)) — execsnoop-rc
  reached stability with far less apparatus, which is itself a data point
  about proportionality.

<a id="lineage-reassessment-map"></a>
## Lineage Map (verified)

From the closed lineage ticket's close reason, cross-checked against each
file's own frontmatter `sources`:

- **Root:** is-tree v1 (gpt56t, Aug 31) — document identity, anchors,
  doc-refs, evidence classes, independent waves, append-only refinement.
- **Parallel sibling:** dolt-rs wiki constitution (Aug 31, later same day) —
  dependency topology, relationship vocabulary, layer boundaries, agent
  packets, mutable tips over immutable evidence. Not a descendant of is-tree.
- **Reconciler:** is-tree v2 (Sep 1) — adopts dolt-rs practices (Derived class,
  relationship labels, inherited/value-add/stop, durable sources, return
  packet), records deliberate differences, and poses the convergence question
  (D27 Q1) that the rekon epic answers.
- **Expanders:** first-compute v1/v2 — wave protocol, terminology discipline,
  comparison axes, viability rubric, lineage baseline, then assignment
  manifests, wave seals, fair synthesis, inherited premises, resource
  identity.
- **Adapters:** execsnoop-rc (single-coordinator engineering program,
  multi-letter prefixes, stable r2) and opencode-otel (snapshot-pinned
  upstream evidence, delivery-leverage focus).
- **Workspace answer:** the rekon constitution, whose lineage section
  ([/constitution/README.md:824-865](/constitution/README.md)) declares itself
  the response to is-tree v2's convergence question.

<a id="lineage-reassessment-thread-thinking"></a>
# Thread A: Thinking And Talking In Markdown

The steer suspects this is a core constitutional principle that stayed
implicit while identity/naming absorbed attention. The predecessor corpus
supports that suspicion strongly. The pattern, stated as a candidate
principle: **reasoning happens through durable, addressable, revisable
documents and their exchanges — and through deliberate commits of those
documents as work progresses — not merely in a final write-up.**

## A1. Discoveries belong in committed documents, not chat

The strongest single sentence in the lineage is dolt-rs W1.1:

> "Later research may reveal a better prefix, missing dependency, citation
> rule, document section, or maintenance tool. Such discoveries should update
> this constitution in a dedicated commit rather than survive only in an agent
> prompt or chat transcript."
> — [dolt-rs constitution.gpt56t.md:34](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-purpose-status)

This is not a documentation-adjacent nicety; it is an epistemic claim about
where thought lives. Every successor inherits it structurally: all five
predecessor constitutions evolved through new committed revisions rather than
prompt drift (is-tree v1 D11:312-323, is-tree v2 D21:27-31, first-compute
D13:492-497, execsnoop-rc C10:235-240).

## A2. Agents exchange understanding *through* the artifacts

dolt-rs states the wiki's purpose as letting "dependency-ordered agents
exchange understanding through durable Markdown artifacts"
([constitution.gpt56t.md:27](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-purpose)).
Both return-packet contracts subordinate the conversational channel to the
document: "This return packet is a dispatch aid. The Markdown artifact remains
the source of shared research truth" (dolt-rs W5.2:334); "The Markdown
document remains authoritative. The packet exists to support later dispatch
and synthesis" (is-tree v2 D22.6:122-123). The rekon constitution's preamble
already carries this as description — "Documentation here is not inert
output. It is the durable substrate through which a workspace learns,
explains itself, activates specialized behavior, and preserves enough history
to learn again"
([/constitution/README.md:87-90](/constitution/README.md)) — but never names
it as an operating principle with obligations.

## A3. Revisions make confidence changes visible

Append-only refinement is the thinking-out-loud mechanism at section scale:

> "Append-only refinement preserves the reasoning history and makes changes in
> confidence visible. A later synthesis may reorganize the result into a new
> model-suffixed file."
> — is-tree v1 [D8.1:270-271](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-append-only-refinement)

The same contract recurs at execsnoop-rc C8.1:209-215 and first-compute
D10.1:394-404, the latter adding an attribution rule: a synthesis "MUST not
present that reorganization as the untouched view of an earlier agent." The
rekon constitution deliberately relaxed this to an option — "An addendum is
useful when the evolution of one artifact's confidence matters. It is not a
requirement" ([/constitution/README.md:768-772](/constitution/README.md)) —
while `/AGENTS.md:22` keeps the addendum as a live workspace technique. That
relaxation is correct for a general constitution, but the *value* underneath
(changes of mind should leave a trace) lost its named home.

## A4. Conversation structure: isolation, then dialogue, through files

The wave protocols are dialogue choreography mediated by files, with timing
rules about when reading peers is evidence and when it is required:

- Independence-as-evidence: is-tree v1 D8 — paired drafts "SHOULD optimize
  for useful disagreement" (line 254); agents "MUST NOT read the other
  model's same-topic file before completing its own initial draft"
  (D8:251-253).
- Dialogue begins at synthesis: is-tree v2 D25.1 — "The same-wave no-peer rule
  ends after independent drafts and appendices are complete. A synthesis
  author MUST read both model files for every topic in scope and cite exact
  sections for load-bearing claims"
  ([topic-document-constitution2.gpt56t.md:168-172](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-synthesis-inputs)).
- Four speech-act types: first-compute D10:376-393 names foundation,
  refinement, synthesis, and adversarial waves — a structured argument, not a
  transcript.
- The rekon naming line demonstrates the same practice as artifact chain:
  draft3's own sources record discussion, adversarial review, human ruling,
  and ask-audit files as the conversation's turns
  ([draft3.sol56x.md:28-63](/constitution/naming/exploration/draft3.sol56x.md)).

## A5. The commit is part of the sentence

Intentional commits are not bookkeeping after thought; they are checkpoints
*of* thought. Evidence:

- Workspace rule: "please jj commit as you go! commit pro-actively in logical
  groups" ([/AGENTS.md:120](/AGENTS.md)).
- Pre-commit verification treats the commit as the unit of completed
  reasoning: dolt-rs W5.3:336-349 ends a nine-step check with "commit only
  the authored artifact with `jj commit`".
- The seal makes the commit the conversation's turn boundary: first-compute
  D15.2 — a synthesis input is defined by a commit ID plus per-file SHA-256
  digests, and "Any later correction or append-only refinement creates a new
  round and a new seal"
  ([topic-document-constitution2.gpt56t.md:55-69](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#w-wave-seal)).
- Coordinator-commit exceptions exist precisely because the shared working
  copy makes racing commits unsafe for a *conversation*: is-tree v1 D10:306-309,
  opencode-otel D7:114, first-compute D12:486-489.
- History is legible through commits: the lineage ticket verified ancestry
  "via OKF sources frontmatter and jj committer timestamps" — the
  constitutional conversation itself is auditable in version control.

## A6. Coverage, gap, and work-item shape

**Coverage:** described but not codified. The constitution's preamble and
history section carry the idea
([/constitution/README.md:87-96,756-780](/constitution/README.md)); `/AGENTS.md`
carries its habits (waves, addenda, commit-as-you-go, doc-pass). What no
maintained document does is name the principle and state its obligations:
reasoning between agents resolves into addressable revisions; conversations
end in documents or tickets, not scrollback; commits are intentional
checkpoints with messages that explain the thought; corrections of record
supersede via new artifacts rather than silent rewrites.

**Work-item shape (hypothetical):** a decision item for the broader
constitution candidate — elevate "thinking and talking in Markdown" to a named
principle with a short obligation list, splitting cleanly into: *value*
(durable thought beats ephemeral chat), *principle* (exchange through
addressable, revisable documents), *mechanisms* (waves, addenda, seals,
coordinator commits, doc-pass), and *tension* (append-only visibility vs
canonical coherence — see the tensions map). This is constitution-level, not
a skill: every predecessor program assumed it regardless of domain.

<a id="lineage-reassessment-thread-identity"></a>
# Thread B: Identity Fragments Into Subprinciples

The steer asks whether identity/names/referentiality fragments into multiple
subprinciples rather than one omnibus topic. The predecessor evidence says
yes — and the fragments have different maturity and different homes:

1. **Section identity.** Semantic anchors, separate from display coordinates
   that "may change after a new section is inserted, while `#v-tuple-ordering`
   remains the stable link target" (dolt-rs W3.2:176). Absorbed by the rekon
   constitution
   ([/constitution/README.md:492-519](/constitution/README.md)).
2. **Topic/namespace identity.** Prefix registries with amendment-before-use
   (dolt-rs W2.3:116-134; is-tree v1 D2.1:73-91). Absorbed as an opt-in
   pattern ("Display coordinates and prefix registries", line 796).
   execsnoop-rc shows registries scale down (multi-letter prefixes, human
   directive, C2.1:69-84) and across programs (reserving is-tree's `E`).
3. **Authorship identity.** Model suffixes naming the underlying model,
   never the harness — stated in all five predecessors (is-tree v1
   D2:60-63; first-compute D2:101-105; execsnoop-rc C2:59-64; opencode-otel
   D2:38) and in `/AGENTS.md:38`. The naming draft3 extends it to actor/hat/
   role pairs (draft3.sol56x.md:390-416) — pending human acceptance.
4. **Evidence identity.** The doc-ref vs source-ref split (is-tree v1
   D5:150-190), commit-pinned canonical citations (dolt-rs W4.3:262; is-tree
   v2 D22.5:99-106), and pinned inspected revisions (first-compute D6:218).
   Partially absorbed (see Datapoint 4).
5. **Epistemic identity.** Claim classes tag *propositions*, not just files
   (is-tree v1 D6; is-tree v2 D22.1 adds Derived; first-compute D6 adds
   Source-stated design and External result). Partially absorbed (Datapoint
   2).
6. **Lifecycle identity.** Immutable evidence vs mutable tips — dolt-rs W6.1
   ("Documents should not claim to have inherited a later revision they never
   consumed", line 366); first-compute D15.8 pins checked-in doc-refs to
   immutable model-suffixed files while suffixless symlinks navigate
   (164-174). Absorbed by the rekon constitution's history section
   ([/constitution/README.md:756-780](/constitution/README.md)).
7. **Federation identity.** Identical local resource paths in different
   repos are not the same knowledge asset (first-compute D15.9:176-183).
   Being generalized by the naming core (Datapoint 7).
8. **Migration identity.** Compatibility anchors, aliases, tombstones on
   renumbering or rename (dolt-rs W3.2 rules 6-7 and W6.3:375-387; is-tree
   v1 D4:142-146). Absorbed
   ([/constitution/README.md:512-519](/constitution/README.md)); draft3
   generalizes into moves/tombstones/successors (436-489) pending acceptance.
9. **Work-vs-knowledge identity.** Shared ticket/anchor namespace without
   parity requirements (rekon-native,
   [/constitution/README.md:521-599,647-680](/constitution/README.md)).

**Observation for the parent:** these fragments do not want one omnibus
section. They already live in different layers of the maintained document
(references, namespace, tickets, history) plus the pending naming core. The
useful work-item shape is a mapping check — ensure the broader candidate
names each fragment once, in its right layer, and cross-links them — rather
than a new unified identity chapter. That is a task, not a decision.

<a id="lineage-reassessment-datapoints"></a>
# Datapoints: Absorbed, Missing, Procedure-Only

Eight concrete contracts with original problem, reusable obligation, current
coverage, and work-item shape or fitting existing ticket.

## 1. Dependency-gated knowledge topology

- **Source:** [dolt-rs W2.2](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-topology-gates)
  lines 76–114 and
  [W2.3](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-topology-prefix-registry)
  lines 116–134. Key passage: "The orchestrator waits for each required
  document to be written, checked, and committed before launching a dependent
  author... An agent may discover that the graph is wrong. It should record
  the evidence and return the proposed edge change" (112–114). The registry
  carries a "Required prior research" column per topic.
- **Problem:** dependent research launched before prerequisites settle
  paraphrases unsettled contracts as new facts and duplicates vocabulary.
- **Obligation:** a program should be able to *declare* knowledge
  prerequisites and gate dispatch on their settlement, with an amendment path
  when the graph is discovered wrong.
- **Coverage:** absorbed only as the opt-in pattern row "Dependency-gated
  dispatch" ([/constitution/README.md:795](/constitution/README.md)). The
  namespace section explicitly disclaims structure: "Lexical ancestry improves
  legibility but does not create beads dependency edges" (569–571). is-tree
  v2 D23.1 records the deliberate contrast — independent topics may run in
  parallel — so this is an **observed program mechanism, not a universal
  requirement**.
- **Gap:** draft3's relation family
  ([draft3.sol56x.md:377-387](/constitution/naming/exploration/draft3.sol56x.md))
  covers supports/contrasts/derives-from/extends/supersedes/copies/mirrors but
  has no prerequisite/depends-on knowledge relation, and no registry surface
  survives outside the opt-in pattern.
- **Work-item shape:** decision under open `rekon-con-naming` — add a
  `prerequisite` (or `depends-on`) knowledge relation, or declare the
  registry a derived view, so a topic can gate on another topic's settlement
  without inventing a beads edge.

## 2. Claim authority: source-stated design vs built behavior; inherited premises

- **Source:** [first-compute D6](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-evidence-discipline)
  lines 204–221 — six claim classes including "Source-stated design: proposed
  by a primary project document but not established as working behavior" and
  "External result", plus "Agents MUST pin or record the inspected revision
  for mutable repositories" (218). And [first-compute v2
  D15.4](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-inherited-premises)
  lines 89–108: a draft relying on a shared-baseline claim "MUST cite the
  inspected primary project source and revision or label the claim
  **inherited and unverified in this assignment**", with a four-row
  synthesis-treatment table distinguishing repeated-from-orientation,
  same-source re-verification, separate-implementation convergence, and
  design-only correspondence.
- **Problem:** shared orientation packets manufacture phantom agreement;
  design intent gets cited as working behavior. The recurring aphorism —
  "Code without a passing test is evidence of shape, not proof of working
  behavior" — appears at is-tree v1 D6:211, execsnoop-rc C6:172, and
  opencode-otel D5:82, a three-project convergence on one rule.
- **Obligation:** distinguish design-stated from built; label inherited
  premises; never let repetition or citation count as verification.
- **Coverage:** partial — the constitution has five claim classes
  ([/constitution/README.md:638-641](/constitution/README.md)) and "a ticket
  records intent, not implementation" boundary (644–645: "Frontmatter is
  identity and orientation, not claim-level proof"). Missing: the
  source-stated-vs-built class, inherited-premise labeling, and the
  shape-is-not-behavior rule.
- **Work-item shape:** task under open epic `rekon-doc-constitution` to
  extend the broader candidate's claim-maturity guidance. Runtime-assurance
  framing: the rule is exactly the split between runtime assurance (tests,
  exercised behavior) and knowledge claims (what documents assert).

## 3. Inherited / value-add / stop boundary

- **Source:** [dolt-rs W4.4](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-content-layering)
  lines 264–273 ("Do not repeat a lower algorithm merely to make the new
  document look self-contained... Repetition is justified only where the
  higher format transforms the lower value in a load-bearing way"); is-tree
  v2 [D22.3](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-layer-contract)
  lines 78–90 ("This prevents synthesis from copying every source document
  into one larger summary").
- **Problem:** synthesis bloat; prerequisite contracts get paraphrased into
  new "facts" with weaker provenance.
- **Coverage:** **adequately absorbed** — pattern row 798
  ([/constitution/README.md:798](/constitution/README.md)) plus
  promotion-as-synthesis steps 2 and 5 (317–323: reconcile competing
  artifacts; cite exact design evidence and leave it in place).
- **Verdict:** belongs in procedures and skills for heavy research programs,
  not universal constitutional law — the constitution's own closing caution
  applies: "None becomes universal ceremony merely because it was effective
  in a large research program" (801–802).
- **Work-item shape:** teach it inside in-progress
  `rekon-agents-maintenance-skillify` (the wave/synthesis skill is its natural
  carrier).

## 4. Source freeze and revision pinning

- **Source:** [opencode-otel D5](file:///home/rektide/src/opencode-otel/.design/otel/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-evidence)
  line 80, the most explicit freeze contract in the lineage: initial snapshot
  pinned at `v2@origin` commit `fac875dba07c`, refreshed to `c907d2ba2765`
  on 2026-09-04 "and used revision-qualified reads from that commit... A
  checked-out workspace may lag its fetched remote and must not be mistaken
  for current `v2`." Plus first-compute D6:218 ("MUST pin or record the
  inspected revision for mutable repositories") and is-tree v2
  [D22.5](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-source-durability)
  lines 99–106: "If a local WIP path later disappears, repair the citation to
  the exact historical commit rather than changing the claim to match current
  code."
- **Problem:** research claims rot as upstreams move; two agents quote
  different "current" trees and neither citation reproduces.
- **Obligation:** record the inspected revision for mutable repositories;
  refresh through revision-qualified reads; repair citations against
  history, not by rewriting claims.
- **Coverage:** partial — the constitution prefers "canonical commit-pinned
  resources for published cross-project evidence" and allows labeled
  `file://` for local WIP
  ([/constitution/README.md:605-609](/constitution/README.md)), but has no
  record-the-inspected-revision obligation, no workspace-lag caution, and no
  citation-repair rule.
- **Work-item shape:** task under `rekon-doc-constitution` adding
  evidence-pinning obligations to the broader candidate. Distinct from
  Datapoint 2 (authority of claim classes): this one is about *reproducibility
  of evidence*, not classification of assertions.

## 5. Assignment manifests, wave seals, model-identity stop rule

- **Source:** [first-compute v2
  D15.1/D15.2/D15.6](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-assignment-manifest)
  lines 35–69 and 129–141. The manifest reserves one output path per agent,
  names forbidden same-wave peer inputs, and forbids index edits until
  sealed; the seal is "a seal manifest naming the commit ID and a SHA-256
  digest for each included file... Any later correction or append-only
  refinement creates a new round and a new seal. A synthesis agent MUST cite
  the seal it read so two agents cannot unknowingly synthesize different
  corpora" (63–69); D15.6 adds "An agent MUST stop and notify the coordinator
  if its actual underlying model does not match the reserved suffix"
  (132–134).
- **Problem:** parallel agents share one working copy; synthesis inputs are
  not reproducible; model identity drifts silently.
- **Coverage:** compressed to the pattern row "Assignment manifests and wave
  seals" ([/constitution/README.md:794](/constitution/README.md)). The
  `/AGENTS.md` wave rules (lines 28–46) cover suffixes, rounds, isolation,
  and acceptance symlinks but say nothing about manifests, seals, or a
  stop-on-model-mismatch rule. The lineage's deepest coordinator machinery
  currently exists as a five-word pattern name — the clearest
  **disconnection** in the maintained corpus.
- **Work-item shape:** two parts. (a) Decision — do seals become required for
  rekon design waves, or stay opt-in with a written procedure for when they
  earn their cost? (b) Feature — a manifest/seal template and checklist
  inside the wave skill under `rekon-agents-maintenance` / skillify. The
  stop-on-model-mismatch rule is cheap enough to belong in `/AGENTS.md` wave
  guidance directly (procedure, not constitution).

## 6. Fair synthesis vs consensus counting

- **Source:** [first-compute v2
  D15.3](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-fair-synthesis)
  lines 71–87. Core rules: "assess each file against its own assignment
  before comparing it with others; compare only overlapping, typed claims or
  common axes; label dimensions that are not comparable; trace repeated
  claims to their shared premise or source; avoid model rankings, consensus
  counts, or majority language; distinguish agreement caused by a shared
  source packet from independent convergence."
- **Problem:** unlike-topic drafts get misread as votes; repetition is
  laundered into confidence — an epistemic failure mode the workspace hits
  every time it runs multi-model waves on heterogeneous assignments.
- **Coverage:** **missing** — `/AGENTS.md:33` defines `syn` with cross-review
  and comparative characterization but no anti-majority rule and no
  shared-source-vs-convergence distinction; the constitution's promotion
  section (313–327) requires reconciling rather than averaging but never
  addresses counting.
- **Verdict:** procedure/skill content more than constitutional law, though
  the promotion section could carry one sentence ("model count is not
  confidence").
- **Work-item shape:** task under `rekon-agents-maintenance-skillify` to
  encode fair-synthesis rules in the synthesis procedure; optionally one
  sentence in the broader candidate's promotion section.

## 7. Cross-repository resource identity

- **Source:** [first-compute v2
  D15.9](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-resource-identity)
  lines 176–183: "Bundle-root-relative `resource` values identify files only
  within this repository. Cross-repository aggregators MUST qualify them with
  the canonical repository... Identical local resource paths in first-compute
  and is-tree are not the same knowledge asset."
- **Problem:** bundle-root-relative paths collide across projects; an
  aggregator or citation cannot tell two same-named documents apart.
- **Coverage:** current convention pairs the qualified ID with a canonical
  URL ([/constitution/README.md:545-549](/constitution/README.md)); the
  naming core draft3 generalizes the whole grammar (authority URI + path +
  anchor, [draft3.sol56x.md:237-253](/constitution/naming/exploration/draft3.sol56x.md)).
- **Verdict:** the open `rekon-con-naming` ticket is the right vehicle and
  this obligation is already on its plate. Important provenance note:
  draft3 is a **model proposal pending human acceptance** — its own
  acceptance section (535–557) enumerates the pending human decisions
  (structured paths, authority URI, `/core/README.md` home, etc.). The
  naming line should not be re-litigated here.

## 8. Corpus validation mechanics

- **Source:** is-tree v2
  [D26](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-validation)
  lines 201–214 — a scratch validator checks frontmatter labels,
  coordinate/heading-depth consistency, anchors, and link targets, and "The
  validator SHOULD be promoted into a tested project script only after
  synthesis clarifies whether this research layout will remain durable."
  dolt-rs [W5.3](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-agent-workflow-verification)
  lines 336–349 gives the nine-step pre-commit check; W7 (389–564) supplies a
  streaming reindexer with explicitly declared limitations (552–564).
- **Problem:** repeated corpus structure justifies mechanical checks; hand
  validation does not scale across waves, and anchors/links rot silently.
- **Coverage:** doc-pass is promoted to an `/AGENTS.md` procedure (48–60) and
  to pattern row 800; the naming draft3 demonstrates rigorous structural
  verification on itself (draft3.sol56x.md:600–609). **No validator tool is
  promoted in rekon** — the is-tree checker still lives in scratch, exactly
  where D26 said it should wait.
- **Work-item shape:** feature task under the `rekon-con-naming`
  implementation plan (draft3's guarded Validate/Inventory tools,
  491–510), honoring the lineage's own restraint: promote tooling only after
  stable need is demonstrated.

<a id="lineage-reassessment-vpmt"></a>
# Values, Principles, Mechanisms, Tensions

Per the steer, separated without predeclaring a finished taxonomy:

**Values** (why any of this exists; observed across all predecessors):

- Durable thought: reasoning should survive its session (A1, A3).
- Honest provenance: authorship, revision inspected, and claim maturity are
  never fabricated or blurred (Datapoints 2, 5; identity fragments 3–5).
- Reproducible evidence: a citation must resolve to what was actually seen
  (Datapoint 4).
- Independence as evidence; disagreement as asset (is-tree D8:254; first-
  compute D1.2:60-68 "The program MUST preserve useful disagreement").
- Proportionality: ceremony must be earned (rekon 801–802; dolt-rs W7.2:564;
  is-tree D26:212-214; execsnoop-rc's whole minimal shape).

**Principles** (normative rules the corpus converged on):

- Identity is separate from presentation (anchors vs coordinates vs prose).
- Mutable current account over immutable exact evidence; neither erases the
  other (dolt-rs W6.1; rekon 756–762).
- Exchange through documents; the artifact is the truth, packets are aids
  (A2).
- Tickets coordinate; documents explain (rekon 649–659).
- Claims carry maturity; tickets record intent, not implementation (A2,
  Datapoint 2).
- Changes apply prospectively; history keeps the constitution it used
  (dolt-rs W1.1:36; is-tree D11; rekon 814–818).

**Mechanisms** (optional machinery, domain-scaled):

anchors; prefix registries; doc-refs; claim-class tables; append-only
addenda; independent/synthesis/adversarial waves; assignment manifests and
seals; dependency-gated dispatch with registries; reindexers and validators;
doc-pass; return packets; accepted-tip symlinks; compatibility anchors and
tombstones; carry islands.

**Unresolved tensions** (live, not settled by any predecessor):

- Append-only visibility vs canonical coherence — rekon relaxed append-only
  (768–772); first-compute made it mandatory for refinement waves (D10.1).
  Neither position is wrong; the boundary is per-program and currently
  implicit.
- Isolation vs dialogue timing — when does the no-peer rule end? is-tree v2
  says "after drafts and appendices are complete" (D25.1); rekon's AGENTS
  says same-wave isolation is "part of the evidence" with no timing rule for
  syn.
- Seal rigor vs proportional ceremony (Datapoint 5's decision).
- Coordinates vs anchors remains dual bookkeeping everywhere (display numbers
  exist in all five predecessors; rekon demoted them to optional reading
  aids, 517–519).
- Frontmatter discipline as the only gate once disposition replaces location
  (hierarchy1's tension 1 — rekon-internal, noted as current work).
- Stable status is under-earned across the lineage: only execsnoop-rc r2
  claims it, with the least process — either stability is cheap or its
  evidence bar is unclear.

<a id="lineage-reassessment-provenance"></a>
# Provenance Discipline

**Actual human decisions observed in the record:** the hyphen-qualified
namespace (syn0:60–64 — "the human asked for the omitted beads project prefix
to become implied qualification"); the torch-pass closure of initial tensions
(rekon 819–822); adoption round-1 decisions (per-repository counting,
`design/` as the single island spelling — adoption1 tip); execsnoop-rc topic
prefixes assigned by human directive (C2.1:69–71); the naming proposal4 human
ruling (draft3 sources). **Model suggestions pending acceptance:** everything
in draft3 (`/core/README.md` home, relation family, guarded tools, actor
model), seals, and the fair-synthesis rules. **Observed mechanisms vs
universal requirements:** dependency gating, append-only refinement, and
prefix registries are mechanisms particular programs chose — is-tree v2 D23.1
explicitly declines dolt-rs's sequencing, and the rekon constitution already
converted most heavy mechanics to opt-in patterns. Nothing in this survey
should be promoted to universal law by this survey; that is the parent's and
the human's call.

<a id="lineage-reassessment-workmap"></a>
# Hypothetical Work Map (no tracker entries made)

| Shape | Item | Existing fit |
|---|---|---|
| Decision | Elevate thinking/talking-in-Markdown to a named principle (Thread A6) | broader constitution candidate (parent) |
| Decision | `prerequisite` knowledge relation or registry-as-view (D1) | `rekon-con-naming` (open) |
| Decision | Seal requirement for rekon design waves (D5a) | `rekon-agents-maintenance` (open epic) |
| Task | Extend claim-maturity: source-stated-vs-built, inherited premises, shape-vs-behavior (D2) | `rekon-doc-constitution` (open epic) |
| Task | Evidence-pinning obligations (D4) | `rekon-doc-constitution` (open epic) |
| Task | Fair-synthesis rules in syn procedure + one promotion sentence (D6) | `rekon-agents-maintenance-skillify` (in progress) |
| Task | Manifest/seal template + stop-on-model-mismatch rule in wave guidance (D5b) | `rekon-agents-maintenance-skillify` (in progress) |
| Task | Identity-fragment mapping check across the candidate's layers (Thread B) | parent integration |
| Feature | Anchor/link validator promotion (D8) | `rekon-con-naming` implementation plan |

<a id="lineage-reassessment-cross-references"></a>
# Cross-References

- [`/constitution/README.md`](/constitution/README.md) **inherits** the
  absorbed contracts; its opt-in pattern table (786–801) is the compression
  layer every verdict above was checked against.
- [`syn0.gpt56t.md`](syn0.gpt56t.md) **records** which predecessor ideas
  became canonical in the first synthesis and which tool questions were
  deferred to the assembler ticket.
- [`README.md`](README.md) in this directory **catalogues** the constitution
  design record and its accepted tips; this file joins that wave as evidence,
  not guidance.
- Closed ticket `rekon-doc-constitution-lineage` **establishes** the ancestry
  map used here (root is-tree v1; parallel sibling dolt-rs; is-tree v2 D27 Q1
  as the convergence question the rekon epic answers).
- [`draft3.sol56x.md`](/constitution/naming/exploration/draft3.sol56x.md)
  **supersedes-pending** the namespace section for identity questions; it is
  a model proposal, not accepted law, and this survey defers to it rather
  than competing.
- [`/AGENTS.md`](/AGENTS.md) **carries** the living practice (waves,
  addenda, commit discipline, doc-pass) that Thread A argues deserves named
  constitutional status.
- Open tickets `rekon-con-naming`, `rekon-agents-maintenance-skillify`, and
  epics `rekon-doc-constitution`, `rekon-agents-maintenance` **carry** the
  hypothetical work-item shapes above; none were modified.
