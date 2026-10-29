---
type: Proposal
title: Eventual document integration and an optional doc helper
description: Batch parent navigation and corpus connections at coherent checkpoints so authors can concentrate on substantive documents.
resource: /constitution/practice/eventual-integration0.gpt6a.md
status: draft
tags: [documentation, doc-pass, collaboration, integration]
generated: { by: "model:openai/gpt-6-astra#medium", at: 2026-10-30 }
sources:
  - resource: "opencode://ses_e7172da2affeJ5Mx7eqRYTsOu8"
    title: Commissioned idea for eventual parent integration and an optional collaborative doc helper; proposal and owning-practice pointer
  - resource: /constitution/README.md
    title: Proportional process, canonical ownership, supporting artifacts, and opt-in patterns
  - resource: /constitution/practice/GLOBAL.md
    title: Existing durable writing, independent waves, and doc-pass obligations
  - resource: /constitution/structure.md
    title: Domain-owned placement and explicit standing
---

<a id="eventual-integration"></a>
# Eventual document integration

**Idea: do substantive writing locally, then integrate navigation and connections
at a coherent checkpoint.** Updating a child document should not automatically
entail rewriting its parent README, an intermediate index, and a root overview.

The human-reported friction is repeated hierarchy maintenance consuming time
and context while the underlying argument is still changing. This proposal
records that concern and a possible practice; it does not establish a new
workspace-wide requirement. The optional `doc` helper below is a collaborative
role, not a configured agent. Its effectiveness remains to be tried.

<a id="eventual-integration-cadence"></a>
## A checkpoint cadence

Let an author finish a coherent argument, revision, or owned slice before
refreshing the wider reading map. Useful integration checkpoints include a
completed wave, a synthesis ready for review, acceptance of a direction, or
handoff to readers who need the new navigation. A commit may preserve work
before that integration checkpoint; every save or child edit need not trigger
a parent edit.

At the checkpoint, collect the changes and ask what a reader now needs to
understand or find. Update the nearest useful landing page and any other
documents whose meaning or routing actually changed. Stop when the reader's
route is coherent. A changed child is not, by itself, a reason to propagate a
summary through every ancestor. A root overview may remain accurate throughout
many substantial changes below it.

Keep the deferred work recoverable with a brief handoff: changed paths, their
meaning, likely connections, and who will integrate them at which checkpoint.
An existing session handoff or work anchor can hold this; a new tracking system
is unnecessary. If the checkpoint moves, carry that note forward explicitly.
When ending work before integration, report the pending pass rather than
claiming the corpus is fully connected.

<a id="eventual-integration-correctness"></a>
## Local correctness and eventual connections

This changes the cadence of the existing
[doc-pass](/constitution/practice/GLOBAL.md#doc-pass), while preserving its
substance:

- During writing, keep the owned document's claims, provenance, status, links,
  and anchors sound. Check explicit changed Markdown before handing it off.
- Search the surrounding corpus by concept and read candidate sources enough
  to distinguish useful relationships. Authors can do this while researching;
  integration can combine the resulting findings once the documents settle.
- At the checkpoint, add explained cross-references, useful reciprocal links,
  and needed index updates; verify links and surrounding meaning after edits.
- Repair a reference broken by a move or anchor change with that change. Do not
  leave a parent asserting an obsolete accepted direction while readers rely
  on it: make the smallest truthful correction or visible pending-integration
  note, then batch the fuller rewrite.

Mechanical link checks and semantic integration answer different questions.
A passing checker does not establish a useful reading map; an eventual reading
map does not excuse publishing broken links. A maintained ⇄ relationship also
keeps its reciprocal maintenance duty when either side changes.

<a id="eventual-integration-helper"></a>
## Optional collaborative `doc` helper

Where several authors share parent documents, a commissioned helper can own
the integration pass. Authors concentrate on their assigned artifacts and
relay a small packet when a coherent slice is ready:

```text
Changed: exact paths and relevant sections; commit/revision if available
Meaning: what changed, why it matters, and its standing
Connections: candidate parents/neighbors and the intended relationship
Pending: unresolved claims, integration checkpoint, urgent reference repairs
```

The coordinator names the helper's editable landing pages and checkpoint so
authors and helper do not compete over the same prose. The helper reads the
actual changed documents and the relevant surrounding corpus, then integrates
one coherent account: navigation, concise summaries where helpful, explained
cross-references, and useful backlinks. It preserves disagreements and draft
standing; a relay is a lead to inspect, not authority to declare acceptance.

The helper checks its explicit changed files and returns the edited paths,
commit, and any deferred or disputed connections. The coordinator reviews the
semantic fit and resolves decisions that exceed the helper's remit. Authors
remain responsible for their substantive claims. If no helper is commissioned,
an author or coordinator can perform exactly the same checkpoint pass.

Independent waves retain their isolation until the agreed integration stage.
The helper should not feed peers' same-wave findings back into unfinished
independent drafts. It need not rewrite the children to make them agree.

<a id="eventual-integration-trial"></a>
## Trial and adoption

A bounded trial could compare parent-edit churn, author context spent on
navigation, and whether the eventual reading map lets someone find the work
without session knowledge. Record missed connections as well as savings.
The hypothesis is fewer repetitive edits with equal or better discoverability,
not merely fewer touched files.

The immediate Colorlu dependency-stack effort has a specific direction to defer
parent-document updates. That local instruction is context for this commission,
not evidence of a completed trial or general adoption. This proposal is reusable
independently of that effort.

Further adoption would decide whether checkpoint integration becomes general
writing guidance and whether a particular collaboration benefits from a helper.
An actual agent definition or ambient instruction change would be separate
work. The present write-up can be discussed and used in an explicitly scoped
trial without making either change.

<a id="eventual-integration-relations"></a>
## Connections

- [Proportional process](/constitution/README.md#doc-constitution-principles)
  motivates batching repeated maintenance around useful outcomes.
- [Supporting artifacts](/constitution/README.md#doc-constitution-module-support)
  establishes the landing-page and index purposes preserved by the checkpoint.
- [Links carry relationships](/constitution/README.md#doc-constitution-references-links)
  constrains integration to meaningful connections rather than ancestor churn.
- [Opt-in patterns](/constitution/README.md#doc-constitution-patterns) provides
  the owning practice's entry point to this proposal.
