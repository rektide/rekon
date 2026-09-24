---
type: PracticeConcept
title: Chase the future — current frontier, opportunity-opening slices
description: A working method for acting on current upstream capabilities without losing ambition, evidence boundaries, or alternate courses.
resource: /praxis/chase-future/chase-future0.gpt6a.md
tags: [praxis, upstream, sequencing, opportunity, navigation]
status: draft
generated: { by: "model:openai/gpt-6-astra", at: 2026-09-06 }
verified: { by: none, at: never }
stale_after: 2026-12-06
extensions:
  work_anchor: "wifi-ptp-now:wiptpn-modern-praxis"
sources:
  - {
      resource: "urn:opencode:session:ses_f86b2c89bffeXMeyFPJ6MJg2Cx",
      title: Human direction relayed for this Rekon contribution,
    }
  - {
      resource: /assess/strategies/high-low0.gpt6a.md,
      title: Expansive possibilities and close evidence,
    }
  - {
      resource: /code/task-reconstruction/philosophy.md,
      title: Present goals rather than historical gravity,
    }
  - {
      resource: /constitution/work-map0.gpt6a.md#navigation,
      title: Existing Navigation concern and ownership,
    }
  - { resource: /remake/README.md, title: Coherent increments and intermediate-state assessment }
---

# Chase the future

The immediate pressure came from Linux Wi-Fi timing work: keep the ambition of
Linux/OpenWrt Wi-Fi timing → PTP → AES67, follow current upstream trunk, and
write the Rust netlink parts that are possible **today**. Do not turn uncertain
driver or firmware support into a reason to postpone all concrete work. Equally,
do not label an implementable userspace request as a working hardware clock.

The general problem is broader: technical work can become trapped either by an
old installed baseline or by an ever-growing map that never changes what we can
do. This practice chooses an opportunity-opening action while retaining the map.
It belongs to [Rekon praxis](/praxis/README.md), not to a particular radio.

> From the current upstream frontier and the human's desired future, what can
> we implement now that gives useful capability or makes consequential questions
> cheaper to answer? What other courses remain interesting, and what evidence,
> cost change, or decision would make us take them?

<a id="frontier"></a>

## Current trunk is the frontier; a pin is the evidence basis

Read the current upstream development line before concluding that a capability
is unavailable or designing around an older workaround. Check local source
archives first. Identify the actual upstream, branch, exact inspected commit,
and freshness of that observation; a local checkout called `main` is not proof
that we inspected today's upstream tip.

Keep these bases separate:

| Basis                               | What it answers                                             | What it must not silently become                      |
| ----------------------------------- | ----------------------------------------------------------- | ----------------------------------------------------- |
| Current upstream frontier           | What interfaces and mechanisms can we build toward now?     | A promise that the deployed system already has them   |
| Pinned implementation/test revision | Which exact source and dependencies does this result cover? | A permanent ceiling on design opportunities           |
| Installed deployment                | What can this particular machine run or reveal now?         | The default definition of everything worth attempting |

Pin a coherent implementation and its tests. Refresh deliberately at meaningful
checkpoints, and earlier if a relevant upstream change defeats a load-bearing
assumption or supplies a substantially better path. Do not rebase every small
edit; do not let a long-running pin prevent noticing the changed frontier.

Real compatibility or deployment obligations still matter. If they require an
older target, state the obligation and the cost of using a newer capability;
choose a port, separate experiment, or explicit deferral. Do not invent that
obligation merely because an older package is installed. Upgrading or operating
the live system remains a separate authorized action.

<a id="opportunity"></a>

## Least cost for the most opportunity, not the least change

Compare a few plausible next moves, including a somewhat bolder one. Prefer a
coherent slice that offers useful behavior **and/or** opens measurement,
experimentation, or reuse. A small instrument can be a real deliverable; it need
not pretend to be the final product to justify implementation.

The comparison is an architectural argument, not a fabricated numerical score:

| Consider               | Ask                                                                                                |
| ---------------------- | -------------------------------------------------------------------------------------------------- |
| Immediate use          | What can someone actually do after this slice?                                                     |
| Information gain       | Which consequential uncertainty becomes observable or testable?                                    |
| Opened courses         | Which different useful outcomes become easier to pursue?                                           |
| Cost                   | What implementation, integration, equipment, review, operation, and ongoing carry does it require? |
| Commitment             | What boundary settles, what becomes harder to change, and how would we undo the bet?               |
| Evidence and authority | What is already supported, what remains a hypothesis, and which actions are permitted?             |

Avoid optimizing one axis without naming the loss. A tiny compatibility shim
can preserve the wrong architecture. A large generic framework can consume more
than the options it preserves are worth. A modest reusable client or instrument
can beat both by enabling several concrete experiments through one verified
boundary. Prefer existing suitable machinery, but verify its contract rather
than inheriting the promise suggested by its name.

**Once an interface is sufficiently understood for the selected slice, implement
it within the granted scope.** Do not require full end-to-end certainty first.
Record the unresolved question the implementation will let us investigate.
Research continues where it can change that implementation or its next use;
it is not a prerequisite to exhaust the whole horizon.

<a id="courses"></a>

## Keep the map, the course, and the next slice distinct

- **Opportunity map:** useful destinations and approaches, including alternatives
  that may never join the current route. Preserve why they are interesting,
  supporting and contrary evidence, uncertainty, cost, and revisit triggers.
- **Selected course:** the direction presently chosen, with its reasons,
  dependencies, observable arrival conditions, and reroute conditions. It need
  not specify every future implementation.
- **Next slice:** the bounded action we have actually selected and authorized,
  what it will make possible, and what claims its evidence can support.

These can be sections of one short note. They are not three mandatory files or
parallel trackers. Keep work lifecycle with its native ticket and explanations
with their maintained owners.

An alternate course may reach a different worthwhile destination, not just
rescue the current one after failure. A measurement technique can remain useful
even if another route delivers the product. Do not rename every preserved
option “later”: that quietly turns interest into a promised backlog.

Use dispositions such as **selected now**, **bounded exploration**, **preserved
option**, and **not pursuing on this evidence**. For the last, preserve the
reason and what could change it. “Unsupported on this device/firmware/revision”
and “not yet investigated” are different statements. Unknowns should remain
named, not forced into false yes/no conclusions for a tidy chart.

### A compact course record

```markdown
## Bearings

Desired future and present decision:
Current upstream source/revision and observation freshness:
Implementation/test basis; deployed basis and genuine constraints:

## Selected next slice

Action, owner, and authority:
What becomes usable or measurable:
Why this beats the smaller and bolder alternatives on cost/opportunity:
Evidence needed for its own claim; claims it does not make:
Checkpoint and conditions to stop, refresh, or reroute:

## Courses of enduring interest

| Course / distinct payoff | Disposition | Evidence / unknown | Exploration and preservation cost | Revisit trigger |

## Return from action

Actual revision, artifact, checks, observations, and limits:
What became possible, impossible, or newly interesting:
Effect on the selected course and preserved options:
Next action or decision; did this practice help?
```

A useful revisit trigger is a changed fact: a new upstream interface, accessible
hardware, a successful event-correlation probe, an unacceptable measured error,
a consumer with a different requirement, or a cheaper dependency. “Someday”
does not tell a future reader when to look again. An option's preservation cost
may be only an explained link; retaining a live backend or abstraction costs
more and needs its own justification.

<a id="gates"></a>

## Gate the claim that needs evidence, not every action in the effort

A command encoder can be built before hardware exists. A hardware result needs
the actual supported path. A timing-accuracy claim needs a suitable measurement.
An operational rollout needs its own authorization. Those gates protect
different things; none implies that all earlier useful work must stop.

Choose implementation when current contracts suffice; choose a focused probe
when an unknown controls the design. Ask for a decision when a proposed future
materially changes scope or architecture. Continue genuinely independent slices
while their neighbors await evidence. [Evidence gates](/assess/gates/README.md)
already supply this dependency discipline; there is no new global stage gate.

Preserve raw observations and their provenance where later interpretation may
change. Keep absence, failure, unsupported capability, and inconclusive evidence
distinguishable. This opens exploration without manufacturing a stronger API
contract or product claim.

At a consequential checkpoint, explain what became possible or impossible,
which architectural choices were made and why, remaining tensions, and the
proposed next slice. Commit the coherent state before changing it. Revisit
options affected by the new evidence, including those made _more_ attractive;
review is not only a search for reasons to stop.

<a id="learn"></a>

## Learn whether the practice opened anything

Useful outcomes include working behavior, a reusable test boundary, a resolved
uncertainty, a better course, or a newly reachable experiment. A failed probe
can still be valuable if it changes what we know and prevents an unjustified
bet. Document volume, green tests unrelated to the intended claim, and a longer
backlog are not substitutes.

Capture expected versus actual opportunity in an
[evidence case story](/evidence/README.md#evidence-case-stories). Was the move too
timid, too costly, or helpfully bold? Did research equip implementation or become
its replacement? Did the option map retain useful possibilities without making
all of them implementation obligations? Revise the practice from that experience.

## Cross-references and deliberate tensions

- [Linux wireless case](/praxis/chase-future/linux-wireless0.gpt6a.md) is the
  **motivating application**: reachable Rust netlink work now, with separate
  source support, hardware evidence, and synchronization/audio claims. Its
  course comparison is an illustration, not this practice's universal roadmap.
- [High/low passes](/assess/strategies/high-low0.gpt6a.md) are **prior art** for
  complementary expansive and grounded thinking. This practice adds the choice
  of an immediate opportunity-opening investment; it does not require an agent
  wave before a straightforward implementation.
- [Fronts and horizons](/praxis/fronts/README.md#fronts-thread) ⇄ **owns the
  map-structuring lenses this practice chooses from**: strategic directions,
  tactical moves, and payoff horizons in one report. Chase the future keeps
  the choose-and-slice half — course selection, dispositions, the bounded
  next action — and shares the preservation duty with that map.
- [Reconstruction's baseline guidance](/code/task-reconstruction/philosophy.md#reconstruct-current-baseline)
  is **foundation and tension**: start from current upstream and pin the work;
  its preference for refresh after a complete green implementation avoids churn.
  Here, a consequential upstream change can reopen the decision sooner, without
  treating the installed baseline as the frontier or authorizing constant rebases.
- [Remake](/remake/README.md#remake-order) provides **sequencing vocabulary**:
  learning order, landing order, and assessment order differ. An instrument may
  open the route before the final product architecture is settled.
- [Navigation work map](/constitution/work-map0.gpt6a.md#navigation) is the
  **existing owner of general course design**. [The recovered Watchman synthesis](file:///home/rektide/src/opencode-watchman-old/.design/watchman/nav0-syn0.gpt56sol.md#where-the-expanded-problem-spaces-go)
  is **concrete prior art**: expanded problem spaces were useful; making all of
  them appear equally immediate was the error. That local source is not a claim
  that its product-specific route is current or adopted here.
- [Upstream reference mirrors](/upstream/README.md) supply **provenance mechanics**,
  not a ceiling on possibilities: an inactive snapshot, an active adaptation,
  and the moving frontier remain different objects.
- [Evidence](/evidence/README.md#evidence-reference) **limits inference**;
  [commit checkpoints](/code/commits.md#commits-checkpoints) **preserve the
  change of mind**. Neither source identity nor saving a draft establishes
  acceptance, compatibility, or deployment.
