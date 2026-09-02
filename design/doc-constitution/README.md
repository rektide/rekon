---
type: DesignIndex
title: Document constitution design record
description: Independent drafts, synthesis, and provenance behind the workspace documentation constitution.
resource: /design/doc-constitution/README.md
tags: [rekon, documentation, constitution, design]
status: active
generated: { by: "model:gpt-5.6-terra", at: 2026-09-01T17:32:46-04:00 }
sources:
  - id: constitution-epic
    resource: /.beads/issues.jsonl
    title: rekon-doc-constitution epic and children
    author: human:rektide
---

<a id="doc-constitution-design"></a>
# Document Constitution Design Record

This directory preserves the argument behind the maintained constitution at
[`/constitution/README.md`](/constitution/README.md). The files here are evidence and design
history, not competing current guidance.

## Independent Drafts

- [`draft0.sol56m.md`](draft0.sol56m.md) develops the knowledge-flow model,
  a single hyphen-qualified ticket/anchor namespace, proportional process, and
  mutable canonical tips over durable evidence.
- [`draft0.glm53m.md`](draft0.glm53m.md) develops the manuscript-to-modules
  re-spin, the corruption/cheapness/universality ambient test, and concrete
  assembly and `GLOBAL.md` sketches.

The agents wrote these independently and did not read one another's same-wave
artifact.

## Synthesis

- [`syn0.gpt56t.md`](syn0.gpt56t.md) compares the drafts, records the choices
  made while producing the first canonical constitution, and preserves the
  unresolved tool-design questions.
- [`adoption0.sol56x.md`](adoption0.sol56x.md) and
  [`adoption0.glm53fm.md`](adoption0.glm53fm.md) independently develop the
  prospective pilot, small-project proposal, and upstream carry policy.
- [`adoption-syn0.sol56x.md`](adoption-syn0.sol56x.md) reconciles that pair and
  records the policy integrated into the canonical constitution.
- [`adoption1.glm53m.md`](adoption1.glm53m.md) records the accepted round-1
  decisions: per-repository counting, `design/` as the single island spelling,
  and tacit local deviations.

## Accepted Tip

- [`adoption.md`](adoption.md) symlinks to
  [`adoption1.glm53m.md`](adoption1.glm53m.md), the accepted tip of the
  adoption material. The wave files above remain citable evidence.

## Status

The constitution remains `status: draft` while its adoption pilot shakes out;
the initial torch-pass review is closed. Accepted changes belong in the
coherent canonical [`/constitution/README.md`](/constitution/README.md); materially distinct
proposals belong in new model-suffixed files here.
