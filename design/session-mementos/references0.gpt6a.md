---
type: ArchitectureApplication
title: Native references across session bookmarks, tickets and knowledge
description: A concrete application of Rekon's namespace work, driven by Cotail key/value links and Opensesser rather than a new universal store.
resource: /design/session-mementos/references0.gpt6a.md
tags: [namespace, references, bookmarks, beads, cotail, opensesser]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-10-06
extensions:
  ticket: rekon-session-mementos-references
sources:
  - { resource: "urn:opencode:session:ses_f86cfeb14ffev9dX8mwiS0sZIm", title: Human key/value link, tool-prerequisite and global-namespace refinements }
  - { resource: /constitution/README.md#doc-constitution-namespace, title: Shared ticket and anchor namespace }
  - { resource: /constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling, title: Human ruling on location identity and cross-modal aliases }
  - { resource: /constitution/naming/exploration/draft3.sol56x.md#planes, title: Candidate native authorities and bridges }
  - { resource: /constitution/naming/exploration/beads-offerings0.sol56x.md#beads-offerings-mapping-namespace, title: Beads namespace evidence and implementation gaps }
  - { resource: "file:///home/rektide/archive/gastownhall/beads/docs/multi-agent/routing.md", title: Native hydration and external-capability dependencies }
  - { resource: "file:///home/rektide/src/opensesser/design/bookmark-links/consumer0.gpt6a.md", title: Concrete session-window consumer }
---

<a id="native-reference-application"></a>
# An incarnation of the namespace, not a replacement constitution

The human refined session links into **Cotail bookmarks with key/value metadata**:
an exact reference value can establish a definitive link, which Opensesser can
browse. Cotail should become an agent-callable tool sooner, as a prerequisite to
the consuming features. The next observation was that **Beads tickets also need
to refer across projects**, making this a concrete incarnation of Rekon's global
namespace ambition.

This is an application under `rekon-session-mementos-references`, related to the
existing `rekon-con-naming` decision. It does not declare that naming decision
closed, replace its grammar, create a new global namespace authority or require
one database for all entities.

## The useful common contract

A reference identifies **which native owner**, **what native kind**, and **which
exact target**. It may carry a convenient label or declared alias, but resolution
must not depend on guessing from that label. This is a behavioral contract, not
an accepted universal JSON schema or URI grammar.

| Native target | Qualification needed outside its original context | Authority retained |
|---|---|---|
| Cotail Session/Message Target | Cotail source identity plus its existing hierarchical Address | OpenCode owns the Session/Message; Cotail owns the bookmark/annotation. |
| Beads issue | Owning project/tracker context plus exact issue ID | Beads owns issue lifecycle, dependencies and acceptance. |
| Knowledge section | Canonical project authority, native file path and explicit anchor | The knowledge owner controls the document and its declared aliases. |
| Published capability | Owning project plus capability name | The producer owns the capability declaration; the native consumer defines satisfaction. |

The [pilot constitution](/constitution/README.md#doc-constitution-namespace)
already joins qualified document anchors and ticket names. Its assumed
"registered project prefix" does not prove a global registry exists. The
[Beads study](/constitution/naming/exploration/beads-offerings0.sol56x.md#beads-offerings-mapping-namespace)
explicitly records that gap.

The [human naming ruling](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling)
also matters: location-derived knowledge names are identity, rename costs are
deliberate, and cross-modal aliases are legitimate. Do not replace that direction
with hidden opaque IDs for every document. Conversely, a document path grammar
must not replace OpenCode's native Session ID or Beads' owned issue identity.
Shared names and explicit bridges need not erase native owners.

## Cotail bookmark key/value links

The proposed immediate shape is intentionally modest:

```text
subject: bookmark targeting Session A
key: related
value: exact Cotail Target for Session B
```

Keys can express relationship vocabulary without a rigid universal taxonomy.
Descriptive key/value metadata can coexist. A reference value must remain
distinguishable from a plain string so it can be resolved and reverse-queried
without prose parsing. Cotail must settle cardinality, mutation and canonical
wire shape in its own bookmark design; this application does not preempt them.

Later values may reference an owning Beads issue or a knowledge section. Those
are **native cross-domain references**, not invented OpenCode/Cotail Address
grains. Their resolution can be added by their actual adapters when needed.
Do not make generic reference validation, a graph engine or a registry mandatory
before exact Session/Message bookmark tools are useful.

An explicit key/value link establishes who asserted a relationship between exact
endpoints. It does not by itself establish that the relationship is true, that
the target is currently reachable, or that a dependency is fulfilled.

## Beads already supplies part of the mechanism

Read-only checks against installed **`bd 1.0.3 (dev)`** found:

- `bd dep add --help` documents `external:<project>:<capability>` targets,
  resolved through `external_projects` configuration. These are **capability
  dependencies**, not an alternative spelling for arbitrary exact issue IDs.
- The [native routing documentation](file:///home/rektide/archive/gastownhall/beads/docs/multi-agent/routing.md#L180-L196)
  describes importing other repositories' exported issues through hydration,
  then using normal issue dependencies. Hydration is a snapshot import, not
  automatically shared live authority.
- [Capability shipping](file:///home/rektide/archive/gastownhall/beads/docs/cli-reference/ship.md#L10-L21)
  uses `export:<capability>` and `provides:<capability>` labels on a closed issue.
  A referenced draft or arbitrary bookmark cannot confer that native state.
- Rekon and Opensesser both reported **`external_projects (not set)`** and
  **no additional repositories configured**. Neither hydration nor external
  dependency satisfaction was exercised here.

Therefore the earlier statement that there was "no valid native blocking edge"
across the trackers was too broad. The actual limit is **no configured and
verified bridge in these consumers yet**. Do not invent unresolved cross-project
edges and call their readiness behavior tested; equally, do not build a second
dependency scheduler before evaluating the native mechanism.

Distinguish three independently useful capabilities:

1. **Reference an exact foreign issue:** enough owner and ID to inspect it.
2. **Browse related work and backlinks:** a query/projection over declared links.
3. **Enforce a foreign prerequisite:** a native Beads relation with configured
   resolution and explicit satisfaction semantics.

A bookmark `requires = <foreign issue>` can record intent without yet performing
step 3. If we later project it into Beads, the projection must say whether it is
a maintained native edge or only advisory; two independently writable graphs
must not silently compete.

## Tool-first direction and consumer split

The next useful delivery is **Cotail callable operations**, not completion of the
global namespace:

1. Expose bounded existing query operations to agents, with a verified host/source
   binding and exact returned identities.
2. Supply exact bookmark creation, key/value updates and forward/reverse link
   queries once the source/store/metadata prerequisites are implemented.
3. Let Rekon practices and the [Opensesser consumer](file:///home/rektide/src/opensesser/design/bookmark-links/consumer0.gpt6a.md)
   use those same operations. Agent tools and browser transport are different
   adapters, not duplicated bookmark policy. OpenCode plugin RPC is a verified
   public candidate for browser access, not an already-installed Cotail service.
4. Add deferred next-response marking afterward. Its lifecycle ambiguity must
   not block useful exact-target tools or linked-session browsing.

The broader namespace application is a parallel design/verification track.
The Cotail owner defines the executable tool contracts; Rekon owns practice,
cross-project alignment and the naming discussion. No actual cross-project
Beads configuration, capability shipping, source DB mutation, UI feature or
global resolver was performed by this note.

## Small next namespace trial

Use this real effort: one Opensesser consumer ticket, its Cotail prerequisite,
one coordinating session, and the document explaining their contract. Resolve
each reference through its native owner and show the connecting relationships.
Then evaluate one native Beads external-capability dependency in an explicitly
authorized fixture/configuration. Check both unfulfilled and fulfilled cases,
unavailable projects and stale imported state before relying on readiness.

Success would make the constitution's cross-project qualification operational
for a useful small set—not certify a universal name system. Preserve readable
canonical links even if an optional resolver is unavailable.

## Cross-references

- [Constitutional reading map](/constitution/index.md) — distinguishes the pilot,
  human rulings, later proposals and unresolved decisions. This application must
  not silently upgrade any of them into accepted implementation.
- [Native planes and bridges](/constitution/naming/exploration/draft3.sol56x.md#planes)
  — supporting candidate vocabulary for shared navigation without a second owner
  of ticket state or document truth.
- [Ordered-pipeline memento](/design/ordered-pipelines/memento0.gpt6a.md) — related
  but different: composition determines evaluation, references identify what it
  evaluates. Neither generic inquiry is a prerequisite for the first Cotail tool.
- [Session mementos](/design/session-mementos/README.md) — originating feature and
  current executable selector checkpoint; no visible display integration yet.
