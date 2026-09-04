---
type: LineageReassessment
title: Predecessor contract reassessment for the broader constitution
description: Read-only survey of which constitutional contracts emerged across is-tree, dolt-rs, first-compute, execsnoop-rc, and opencode-otel are absorbed, missing, or belong only in procedures and future skills.
resource: /design/doc-constitution/lineage-reassessment0.glm53m.md
tags: [rekon, constitution, lineage, evidence, synthesis, waves]
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
  - id: lineage-ticket
    resource: /.beads/issues.jsonl
    title: rekon-doc-constitution-lineage close record
    author: human:rektide
---

<a id="lineage-reassessment"></a>
# Predecessor Contract Reassessment

Read-only survey: which constitutional contracts emerged across the
predecessor projects, which the maintained
[`/constitution/README.md`](/constitution/README.md) absorbs, which are
missing or disconnected, and which belong only in procedures or future skills.
No tickets were created or modified; work-item shapes are hypothetical for the
parent to integrate.

<a id="lineage-reassessment-scope"></a>
## Scope, Method, Absences

Read in full: rekon constitution + design record, is-tree v1/v2, dolt-rs,
first-compute v1/v2, execsnoop-rc r1/r2, opencode-otel, naming draft3,
`/AGENTS.md` wave/doc-pass conventions, and the closed
`rekon-doc-constitution-lineage` record. first-compute's files are substantive
adaptations of is-tree's, not copies (verified by diff). **Absences:**
`chrome-agent-platform-fox/docs/CONSTITUTION.md` is the word origin but governs
product robustness, not documents (head skim only, per lineage ticket);
`/constitution/reassessment0.gpt6a.md` and same-round outputs were not read.
Notable: execsnoop-rc r2 is the only predecessor marked `status: stable`
([constitution2.glm53.md:7](file:///home/rektide/src/execsnoop-rc/.design/constitution/constitution2.glm53.md)).

<a id="lineage-reassessment-datapoints"></a>
## Datapoints

### 1. Dependency-gated knowledge topology

- **Source:** dolt-rs [W2.2](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-topology-gates)
  76–114 (orchestrator waits; agents propose graph corrections);
  [W2.3](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-topology-prefix-registry)
  116–134 (prefix registry with "Required prior research" column).
- **Problem/obligation:** dependent research launched before prerequisites
  settle paraphrases unsettled contracts as new facts; a program should
  *declare* knowledge prerequisites and gate dispatch on their settlement.
- **Coverage:** absorbed only as pattern row "Dependency-gated dispatch"
  ([/constitution/README.md:795](/constitution/README.md)); the namespace
  disclaims lexical ancestry as dependency edges (569–571). is-tree v2 D23.1
  deliberately contrasts — a **program mechanism, not a universal**.
- **Gap:** draft3's relation family
  ([draft3.sol56x.md:377-387](/constitution/naming/exploration/draft3.sol56x.md))
  has no prerequisite/depends-on knowledge relation or registry surface.
- **Work-item:** decision under open `rekon-con-naming` — add a
  `prerequisite` relation (or registry-as-derived-view) to core relations.

### 2. Claim authority: source-stated design vs built; inherited premises

- **Source:** first-compute
  [D6](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-evidence-discipline)
  204–221 (six classes incl. "Source-stated design"; pin inspected revisions);
  [D15.4](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-inherited-premises)
  89–108 ("inherited and unverified in this assignment"). "Code without a
  passing test is evidence of shape, not proof of behavior" recurs at is-tree
  D6:211, execsnoop-rc C6:172, opencode-otel D5:82.
- **Problem/obligation:** shared orientation packets manufacture phantom
  agreement and design intent gets cited as working behavior; distinguish
  design-stated from built, label inherited premises, and never treat
  repetition or citation count as verification.
- **Coverage:** partial — five claim classes exist
  ([/constitution/README.md:638-641](/constitution/README.md)) and "a ticket
  records intent... not implementation evidence" (644–645), but no
  source-stated-vs-built class, inherited-premise labeling, or
  shape-is-not-behavior rule.
- **Work-item:** task under open epic `rekon-doc-constitution` extending the
  broader candidate's claim-maturity guidance.

### 3. Inherited / value-add / stop boundary

- **Source:** dolt-rs [W4.4](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-content-layering)
  264–273; is-tree v2 [D22.3](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-layer-contract)
  78–90.
- **Problem:** synthesis copies prerequisite documents into larger summaries.
- **Coverage:** **adequately absorbed** — pattern row 798 plus
  promotion-as-synthesis steps
  ([/constitution/README.md:313-327](/constitution/README.md)).
- **Verdict:** belongs in procedures/skills for heavy research, not universal
  law. **Work-item:** teach it inside in-progress
  `rekon-agents-maintenance-skillify`.

### 4. Source freeze and revision pinning

- **Source:** opencode-otel [D5](file:///home/rektide/src/opencode-otel/.design/otel/research/topic-document-constitution/topic-document-constitution.gpt56t.md#d-evidence)
  line 80 (pinned snapshot and refresh commits; "a checked-out workspace may
  lag its fetched remote"); first-compute D6:218 ("MUST pin or record the
  inspected revision"); is-tree v2 [D22.5](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-source-durability)
  99–106 (repair citations to the historical commit, not the claim).
- **Problem/obligation:** research claims rot as upstreams move and two
  agents quote different "current" trees; record inspected revisions and
  repair citations against history.
- **Coverage:** partial — commit-pinned canonical links preferred
  ([/constitution/README.md:605-609](/constitution/README.md)), but no
  record-the-inspected-revision obligation, workspace-lag caution, or
  citation-repair rule.
- **Work-item:** task under `rekon-doc-constitution` adding evidence-pinning
  obligations to the broader candidate.

### 5. Assignment manifests, wave seals, model-identity stop rule

- **Source:** first-compute v2
  [D15.1](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-assignment-manifest)
  35–53;
  [D15.2](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#w-wave-seal)
  55–69 (seal = commit ID + SHA-256 per file; synthesis cites its seal);
  [D15.6](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-model-identity)
  129–141 (agent MUST stop on model/suffix mismatch).
- **Problem:** parallel agents share one working copy, synthesis inputs are
  not reproducible, model identity drifts.
- **Coverage:** compressed to one pattern row (794); `/AGENTS.md` wave rules
  cover suffixes and isolation but not seals, manifests, or the stop rule —
  the lineage's deepest coordinator machinery exists only as a five-word
  pattern name.
- **Work-item:** decision (do seals become required for rekon design waves?)
  plus feature task putting a manifest/seal template into the wave skill under
  `rekon-agents-maintenance` / skillify.

### 6. Fair synthesis vs consensus counting

- **Source:** first-compute v2 [D15.3](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-fair-synthesis)
  71–87: assess each draft against its own assignment; no model rankings,
  consensus counts, or majority language; distinguish shared-packet agreement
  from independent convergence.
- **Problem:** unlike-topic drafts get misread as votes; repetition is
  laundered into confidence.
- **Coverage:** **missing** — `/AGENTS.md:33` defines `syn` with
  cross-comparison but no anti-majority rule or shared-source distinction; the
  constitution's promotion section is silent.
- **Verdict:** procedure/skill content, not constitutional law.
- **Work-item:** task under `rekon-agents-maintenance-skillify` encoding
  fair-synthesis rules in the synthesis procedure.

### 7. Cross-repository resource identity

- **Source:** first-compute v2 [D15.9](file:///home/rektide/src/first-compute/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-resource-identity)
  176–183: identical local `resource` paths in different repos are not the
  same knowledge asset.
- **Problem:** bundle-root-relative paths collide across projects.
- **Coverage:** current convention pairs qualified ID with canonical URL
  ([/constitution/README.md:545-549](/constitution/README.md)); naming core
  draft3 generalizes it (authority URI + path + anchor,
  [draft3.sol56x.md:237-253](/constitution/naming/exploration/draft3.sol56x.md)).
- **Verdict:** open `rekon-con-naming` is the right vehicle — but draft3 is a
  **model proposal pending human acceptance** (its acceptance section,
  535–557, lists the pending human decisions).

### 8. Corpus validation mechanics

- **Source:** is-tree v2 [D26](file:///home/rektide/src/is-tree/.design/research/topic-document-constitution/topic-document-constitution2.gpt56t.md#d-validation)
  201–214 (scratch validator; promote to tested tool only after the layout
  proves durable); dolt-rs [W5.3](file:///home/rektide/src/dolt-rs/.design/research/constitution/constitution.gpt56t.md#w-agent-workflow-verification)
  336–349 (nine-step pre-commit check); W7 reindexer limits (552–564).
- **Problem:** repeated structure justifies mechanical checks; hand
  validation does not scale across waves.
- **Coverage:** doc-pass is promoted to an `/AGENTS.md` procedure (48–60) and
  pattern row 800; **no validator tool is promoted in rekon** — the is-tree
  checker still lives in scratch.
- **Work-item:** feature task under the `rekon-con-naming` implementation plan
  (draft3's guarded Validate/Inventory tools, 491–510), honoring dolt-rs
  restraint: promote only after stable need.

<a id="lineage-reassessment-provenance"></a>
## Provenance Discipline

**Human decisions observed:** hyphen-qualified namespace (syn0:60–64, "the
human asked"); torch-pass closure (constitution:819–822); adoption1 revisions
(per-repo counting, `design/` spelling); execsnoop-rc prefixes assigned by
human directive (C2.1:69–71); naming proposal4 human ruling (draft3 sources).
**Model suggestions pending:** everything in draft3 (`/core/README.md` home,
relation family, guarded tools), seals, fair-synthesis rules. **Mechanisms vs
universals:** dependency gating, append-only refinement, and prefix registries
are observed program mechanisms — is-tree v2 D23.1 explicitly declines
dolt-rs's sequencing, and the rekon constitution already relaxed append-only
refinement (768–772). Nothing here is a universal requirement until the
broader constitution says so.

<a id="lineage-reassessment-cross-references"></a>
## Cross-References

- [`/constitution/README.md`](/constitution/README.md) **inherits** the
  absorbed contracts; its opt-in pattern table (786–801) is the compression
  layer checked against here.
- [`syn0.gpt56t.md`](syn0.gpt56t.md) **records** which predecessor ideas
  became canonical and which tool questions were deferred.
- Closed ticket `rekon-doc-constitution-lineage` **establishes** the ancestry
  map (root is-tree v1; parallel sibling dolt-rs; is-tree v2 D27 Q1 as the
  convergence question this epic answers).
- [`draft3.sol56x.md`](/constitution/naming/exploration/draft3.sol56x.md)
  **supersedes-pending** the namespace section for identity questions; not yet
  accepted law.
- Open tickets `rekon-con-naming`, `rekon-agents-maintenance-skillify`, and
  epic `rekon-doc-constitution` **carry** the hypothetical work-item shapes.
