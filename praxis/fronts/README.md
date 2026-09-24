---
type: Praxis
title: Fronts and horizons — connect strategic options to tactical use
description: Map directions worth pursuing at different payoff horizons, then choose concrete next moves without shrinking the vision into a backlog or turning possibilities into commitments.
resource: /praxis/fronts/README.md
status: draft
tags: [praxis, strategy, tactics, fronts, horizons, crossroads, yield]
generated: { by: "model:openai/gpt-6-astra", at: 2026-09-23 }
sources:
  - resource: urn:opencode:session:ses_f3aa8012cffee5Mp1KdjDdhfaI
    title: Human request to capture the strategic/tactical, different-horizon technique from the Opencoattails follow-up discussion
    author: human:rektide
  - resource: /praxis/post-build/README.md
    title: Gather the possibilities a working system reveals
  - resource: /praxis/chase-future/chase-future0.gpt6a.md
    title: Keep opportunity map, selected course, and next slice distinct
  - resource: /praxis/bookmarking/README.md#tag-crossroads
    title: Crossroads as a decision-dense junction worth returning to
  - resource: /assess/strategies/high-low0.gpt6a.md
    title: Expansive possibilities grounded in close evidence
  - resource: file:///home/rektide/src/opencoattails/design/selection-cursor/hardening-build0.gpt6a.md
    title: Opencoattails core-hardening checkpoint that prompted the question
  - resource: "urn:opencode:session:ses_f2ebf7b1effewnCNYbBQ8Ewey2"
    title: Human direction to hub the fronts thread here and adopt the ⇄ maintained-link convention
  - resource: file:///home/rektide/src/kube-kubeless/design/horizon/frontiers0.gpt.md
    title: Project-native fronts-as-campaigns briefing, external prior art
  - resource: file:///home/rektide/src/kube-kubeless/design/horizon/vision0.gpt56t.md
    title: Four-horizons candidate visions, external prior art
---

<a id="fronts-and-horizons"></a>

# Fronts and horizons

**Look outward at what the work could become, and inward at what we can put
to use next. Connect those views without making either stand in for the
other.** A strong next-step assessment offers both meaningful destinations
and concrete movement—not just a repair list, and not just a vision.

The motivating prompt followed a completed Opencoattails core-hardening pass:
is the current architecture still a good account of where we are; which
future fronts offer worthwhile gains; and what sensible, practical follow-up
would put the work into more use? The human called out this combination of
strategic and tactical analysis, across different horizons, as a practice to
retain in Rekon.

This is draft praxis, not an installed skill or a mandatory planning
ceremony. Product facts, tickets, and implementation authority stay with
their native projects.

<a id="distinctions"></a>

## Three distinctions that make the technique useful

| Lens | Question | Do not confuse it with |
|---|---|---|
| **Strategic direction** | What kind of capability, experience, or position is worth building toward, and why? | A large task, a distant deadline, or an already-approved roadmap |
| **Tactical move** | What bounded action would produce use, evidence, or a consequential decision from here? | Merely the smallest diff, or an instruction to ignore the larger direction |
| **Payoff horizon** | How much adoption, composition, evidence, or commitment separates this action from its useful return? | A calendar estimate or a ranking of importance |

A small source-identity decision can be strategic because it shapes many
later consumers. A large mechanical migration can be tactical. A tiny
consumer can have immediate utility **and** strategic information value by
testing whether a shared interface really removes caller obligations.

An **opportunity front** is a coherent possible direction with a recognizable
payoff: for example, an investigation workbench, trustworthy automation, or
durable review memory. It is not automatically a ticket, component, release,
or [active intent front](/CONTEXT.md#intent-front). A front can remain worth
preserving without joining the selected course.

## Work at several horizons, not a ladder of promises

Use as many horizons as the situation warrants. A useful starting set is:

- **Put the foundation to use.** A real consumer, workflow, or acceptance
  exercise turns landed machinery into something someone can actually do.
- **Compose adjacent capabilities.** Connect existing pieces into a more
  complete loop; settle the few shared contracts that make it reliable.
- **Change the offering or reach.** Explore a different host, stronger
  guarantee, larger audience, or more ambitious mode of use.

These can overlap and proceed independently. A farther-horizon direction
may deserve a small probe now. A near-use opportunity may be blocked on one
decision. Do not put every interesting possibility into a compulsory
near/next/later delivery sequence, and do not use speculative delivery dates.

<a id="crossroads-relation"></a>

## How this relates to crossroads

[Crossroads](/CONTEXT.md#crossroads) names the **junction**: verified ground,
open paths, and paths that lead back, preserved while the reasoning is
inspectable. Fronts-and-horizons analysis is one way to **equip the choice at
that junction**. [Bookmarking](/praxis/bookmarking/README.md#tag-crossroads)
makes the junction recoverable; it does not perform the analysis or choose
the route.

The [OCYC fleet crossroads](file:///home/rektide/src/ocyc/design/fleet/crossroads0.glm53max.md)
is prior art: it distinguishes passing tests from unperformed live
acceptance, compares forward paths by buys/risks/gates, and records paths
back. The horizon lens adds an explicit comparison between practical use of
the new foundation, adjacent combinations, and transformations of the
offering. It does not replace the ground inventory or the return paths.

An architectural shortcut, an integration probe, or reopening an earlier
decision can be a sensible next move. “Forward” does not mean another feature
must be added, and returning to a foundation is not necessarily regression.

<a id="practice"></a>

## A practical pass

### 1. Establish the actual position

Start with the human's desired experience and the latest accepted evidence.
Separate what is **implemented**, **tested**, **used by real consumers**, and
**still assumed**. Keep the maintained architecture and the acceptance
checkpoint distinct: one explains the system; the other records what was
proved at a revision. Read the consequential code when claims depend on it.

### 2. Name fronts by the value they could create

Group possibilities by meaningful outcomes, not by whichever directory or
ticket happens to exist. Develop a few genuinely different directions. A
flat backlog cannot show whether several tasks serve one opportunity, or
whether similar-sounding tasks lead to different products.

### 3. Make the yield legible

For each front, identify:

- **Use:** who gains what ability; the before/after workflow.
- **Leverage:** which existing foundation it uses, and which repeated
  obligations it could remove for several consumers.
- **Learning:** the consequential uncertainty a first slice would resolve.
- **Reach/options:** which additional courses become easier, and which it
  might make harder.
- **Cost and commitment:** integration, conceptual machinery, operational
  burden, review, and ongoing carry—not just implementation size.
- **Ground and gate:** evidence already available, assumptions, owner
  decisions, and a concrete next proof.

Yield is an argument supported by evidence, not an invented numerical score.
Call out where a quick useful result and a larger strategic return differ.

### 4. Write the tactical section separately

Make the practical recommendation easy to find. For each recommended move,
name the consumer or user loop, the existing interface it crosses, the
bounded deliverable, its acceptance evidence, and what it deliberately
leaves unsettled. Say whether it **uses**, **tests**, **unlocks**, or merely
**preserves** the larger front.

Prefer a working vertical when library machinery has accumulated without
consumers. Pair adapters or real result paths can reveal more than another
record-shape example. A focused diagnostic, documentation repair, or decision
can also be the right move when it removes the real obstacle to use.

### 5. Recommend a course while preserving alternatives

Explain why the leading move is attractive **now**, and what it sacrifices.
Offer a distinct alternative when the human's desired kind of payoff could
change the choice. Name actual dependency gates; do not let an unresolved
large question block unrelated useful work.

Distinguish **recommended**, **selected/authorized**, **bounded inquiry**,
**preserved option**, and **held by an existing decision**. An interesting
front is not automatically authorized work or a promised backlog item.
Record a factual revisit trigger for parked options.

### 6. Return from use

After the next action, record what became usable, what the interface taught
us, and whether the predicted yield appeared. Update the front map when
evidence changes the available paths. Preserve the old checkpoint; do not
silently rewrite the prediction as if it had always matched the outcome.

<a id="prompt"></a>

## A reusable prompt

> Establish where we actually are from the current architecture, code, and
> accepted evidence. Map several worthwhile fronts we could head into,
> across different payoff horizons: practical use of what now exists,
> adjacent capabilities we could compose, and larger changes in what the
> system could offer. For each, explain the concrete gains, leverage,
> assumptions, costs, gates, and what would make it worth pursuing.
>
> Then give a separate, direct tactical section: a few sensible next moves
> that put the work into use or produce decisive evidence. Recommend an
> order where dependencies justify one, say which larger fronts those moves
> serve, and preserve meaningful alternatives and paths back. Distinguish
> observations, proposals, and authorized commitments; do not invent delivery
> dates or turn every possibility into a ticket.

## Compact output shape

```markdown
## Where we stand
Implemented / verified / actually used / still unproved; references.

## Strategic fronts and payoff horizons
| Front and user loop | Yield | Existing ground | Missing decision/proof | Cost/commitment | Horizon and disposition |

## Sensible practical follow-up
For each leading move: real consumer, bounded outcome, acceptance evidence,
front served, and what is deliberately not being decided.

## Recommendation and alternatives
Why this next; what could change the order; paths back and revisit triggers.
```

## Failure modes

- **Vision without a next encounter:** no real consumer or experiment can
  teach whether the proposed capability helps.
- **Tactics without a front:** several tasks complete, but no useful loop or
  strategic position improves.
- **A backlog disguised as strategy:** priority labels substitute for an
  explanation of gains, dependencies, and competing destinations.
- **Readiness laundering:** tests imply adoption, a library method implies
  a shipped workflow, or a label check implies durable identity.
- **A roadmap by accident:** options become commitments through headings
  such as “phase three” or arbitrary dates, without a selection decision.
- **Never leaving the crossroads:** another comprehensive analysis replaces
  an already-authorized, sufficiently understood move.

## Cross-references

- [Post-build possibility gathering](/praxis/post-build/README.md) provides
  the **vantage point** after a working vertical; this practice supplies an
  explicit strategic/tactical and payoff-horizon structure for the return.
- [Chase the future](/praxis/chase-future/chase-future0.gpt6a.md#courses)
  owns the **map/course/slice distinction** and preserving alternate courses;
  the present technique makes their different gains comparable.
- [High/low assessment](/assess/strategies/high-low0.gpt6a.md) is a
  **complementary inquiry strategy**. Altitude is where inquiry aims; horizon
  is how a capability reaches useful return. Neither is model size or a
  schedule. Different agents are optional, not required by this practice.
- [Candidates](/praxis/candidates/README.md) supplies **fair comparison of
  genuinely different options**, rather than a predetermined answer dressed
  as a menu.
- [Bookmarking's crossroads practice](/praxis/bookmarking/README.md#tag-crossroads)
  supplies **recovery of the junction**, while this page equips the choice.
- [Opencoattails hardening acceptance](file:///home/rektide/src/opencoattails/design/selection-cursor/hardening-build0.gpt6a.md)
  is the **motivating checkpoint**, not evidence that all proposed future
  fronts have been implemented or that this praxis is independently proven.
- [Opencoattails future fronts](file:///home/rektide/src/opencoattails/design/engine/future-fronts0.gpt6a.md)
  is a **worked application** of the prompting technique: a grounded capability
  map, six distinct directions and their gains, followed by a separate tactical
  choice between navigable queries and exact forensic inspection. Its proposals
  are not implementation commitments or proof of realized payoff.

---

<a id="fronts-thread"></a>

# Addendum: the fronts thread

Added 2026-09-23 from the wiring session
(`urn:opencode:session:ses_f2ebf7b1effewnCNYbBQ8Ewey2`): the human confirmed
this practice as the hub of the surrounding map/orientation family, adopted
the **⇄** [bidirectional-link convention](/CONTEXT.md#bidirectional-link) for
its maintained pairs, and asked for external prior art as pointers only. The
body above is unchanged; this addendum states the grouping.

## Map → choose → act → return

The nearby practices describe one maneuver at different moments. This page is
the hub of that thread; each member keeps a distinct job rather than a
competing table:

| Moment | Practice | What it owns |
|---|---|---|
| Map | this page | The map's structure: strategic directions × tactical moves × payoff horizons; the yield questions; the report shape |
| Map (generate) | [Candidates](/praxis/candidates/README.md) ⇄ | Plurality before the map hardens — genuinely different possibilities, not a preselected answer |
| Map (feed) | [Parallax](/praxis/parallax/README.md) | Contrasting positions and design axes that populate the map |
| Choose | [Chase the future](/praxis/chase-future/chase-future0.gpt6a.md#courses) ⇄ | Turning a map into a selected course and a bounded next slice; dispositions; the preservation duty |
| Trigger | [Post-build](/praxis/post-build/README.md) ⇄ | The vantage point — when a completed vertical makes redrawing the map worthwhile |
| Altitude | [High/low passes](/assess/strategies/high-low0.gpt6a.md) ⇄ | Where inquiry aims: high passes generate the front map; low passes ground the tactical moves |
| Maintain | [Digest](/digest/README.md#digest-attention) ⇄ | The map's living instance — intent fronts — between deliberate passes |
| Origin | [Lineage research fronts](/assess/rekon/lineage0.gpt6a.md#fronts) ⇄ | Where the fronts vocabulary first crystallized in rekon |
| Junction | [Bookmarking](/praxis/bookmarking/README.md#tag-crossroads) ⇄ | Recovering the crossroads this page equips |
| Return | [Case stories](/evidence/README.md#evidence-case-stories) | Did the predicted yield appear when the move met use |

Three lenses run through the thread and are worth keeping unconfused:
**altitude** (where inquiry aims), **horizon** (payoff distance of a
direction), and **structure** (strategic vs tactical within one report).
Digest maintains the map's instance; this practice is the maneuver that
redraws it.

## External prior art — pointers, not merges

These stay project-native; a ⇄ pair is not maintained across repositories.

- [kube-kubeless frontiers briefing](file:///home/rektide/src/kube-kubeless/design/horizon/frontiers0.gpt.md)
  runs **fronts as campaign units with lifecycle** — "loop front closed",
  "families front accepted" — closed by consolidated verification and opened
  by human sign-off. Its [four-horizons vision](file:///home/rektide/src/kube-kubeless/design/horizon/vision0.gpt56t.md)
  predates this page's horizon lens. Vocabulary bridge: a kube-kubeless
  *front* is a campaign; an *opportunity front* here is a possible direction,
  not a ticket or authorization.
- [cotail's future fronts](file:///home/rektide/src/cotail/design/engine/future-fronts0.gpt6a.md)
  is the worked application cited above.
- [ocyc's fleet crossroads](file:///home/rektide/src/ocyc/design/fleet/crossroads0.glm53max.md)
  is the junction-recovery anchor cited above.

The ⇄ reciprocity itself has a filed desire, not a build:
`rekon-bidi-link-checker`.
