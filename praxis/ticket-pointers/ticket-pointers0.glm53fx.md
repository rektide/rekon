---
type: Praxis
title: Ticket pointers — tickets point at the living tip
description: Help tickets point to current explanatory documents and preserve concise implementation reports when useful, while letting users choose how much work to coordinate through tickets.
resource: /praxis/ticket-pointers/ticket-pointers0.glm53fx.md
tags: [praxis, tickets, beads, pointers, implementation-reports, drift, history, documentation]
status: draft
generated: { by: "model:glm-5.3-flash-max", at: 2026-09-17 }
verified: { by: none, at: never }
stale_after: 2026-12-17
extensions:
  tickets: [rekon-ticket-link-explorer, rekon-ticket-agent]
sources:
  - { resource: "urn:opencode:session:ses_f4f6e3a46ffeHZhJlkIVQcoKYL", title: "Human direction: tickets should point at their two to four most relevant documents, stay pointed at the tip, and deserve a link-history explorer" }
  - { resource: "urn:opencode:session:ses_f2f224b43ffeFFCVKEFko1ondu", title: "Human direction: ticket pointers, user-shaped ticketing, implementation reports, and a possible ticket agent" }
  - { resource: /AGENTS.md#beads, title: Beads command patterns, including append-only note history }
  - { resource: /design/session-mementos/references0.gpt6a.md, title: Qualified references across bookmarks, tickets, and knowledge }
---

<a id="ticket-pointers"></a>
# Ticket pointers

**A ticket should hand its next reader the documents, not make them excavate.**
When a ticket marks a desire or an effort, its body points at the **two to four
documents that currently best explain that work**. Agents working on or around
the ticket keep those pointers current: when a design wave, roadmap, check-in,
or milestone document becomes the better entry point, the ticket's pointers
move to it. **A ticket is a pointer to the tip, not a snapshot** of what was
true the day it was filed.

This is a human-directed tactic within [Rekon praxis](/praxis/README.md). It
uses the ticket mechanics rekon already runs — beads descriptions and
append-only notes — and the [balanced ticket coordination](/constitution/README.md#doc-constitution-tickets)
division of labor: tickets hold intent, lifecycle, and acceptance; documents
hold arguments, evidence, and contracts. The pointer is the edge between them.
It installs no skill, no tracker, and no new command. Its companion
[`GLOBAL.md`](/praxis/ticket-pointers/GLOBAL.md) carries only condensed ambient
guidance.

## Ticketing follows the user's desired coordination

Treat ticketing as a conversation, not a universal gate. When a user is working
under tickets, help keep the relevant ticket current; when ticketing would add
noise, don't force it. If the right level of coordination is unclear, ask the
user how they want the work tracked. Creating a ticket is one possible pointer
to the work, not a prerequisite for implementation or a report.

When work materially advances an existing ticket, consider appending a concise
implementation report as a Beads note. Include the current OpenCode session ID
so the work can be traced to its context. Keep the report proportional: what
changed, what was checked, and what remains or is blocked. This is guidance to
consider, not a requirement to manufacture a ticket or report every small edit.

## The practice

File the desire in the ticket; point at the explanation. When creating a ticket
that marks work, include a short **pointers** note naming the two to four most
relevant documents for that work, each with a few words on why it matters. The
pointer set is **curated for the tip**:

- **Two to four.** One document is often a lone anchor with no course;
  more than four is an archive wearing a ticket's clothes. Enough to orient a
  cold reader; few enough to actually be read.
- **Current, not complete.** Pick the documents that best answer the work's
  questions *now* — the accepted doc for practice, the newest wave for open
  inquiry, the roadmap or check-in for direction. Superseded drafts belong to
  history, not the pointer set.
- **Qualified across owners.** When a pointer crosses repositories, name the
  owning project and the exact path or ticket id, per the
  [native reference contract](/design/session-mementos/references0.gpt6a.md).
  A bare relative path is only meaningful inside its own bundle.

## Keep the pointers at the tip

Pointers are maintained, not monuments. Any agent working on or around a ticket
keeps its pointer set current:

- **When work produces new documents** — a fresh design wave, a synthesis, a
  roadmap or check-in revision, a promoted `doc/` page — move the pointers to
  the new tip. The commissioning documents that launched the ticket do not get
  grandfathered in; relevance is the only criterion.
- **Append, never rewrite.** Beads notes are append-only
  (`bd update --append-notes`, per [`/AGENTS.md`](/AGENTS.md#beads)). Add a new
  note carrying the refreshed pointer set and one line on what moved and why.
  Earlier pointer sets remain in place as the audit trail. The ticket's
  **current pointers are the tip note's pointers**; its **history is every
  prior set**. These are different artifacts serving different needs, and the
  append-only mechanic keeps both honestly.
- **Check pointers when you touch the ticket** — claiming it, appending to it,
  or closing it. Closing a ticket whose pointers all dead-end or point at
  superseded drafts fails this practice even if the work shipped.

The interplay is the point: appending preserves the history that curation would
otherwise erase. A ticket that has been re-pointed five times tells the story of
five course corrections — but only if each pointer set was *added*, not edited
in place.

## Pointer rot is the failure mode

Every practice named above decays the same way: a ticket filed against last
month's tip silently becomes a signpost to a superseded draft. Nothing today
shows whether a ticket's pointers drifted, when they moved, or whether they
still point at the tip. Diagnosing that today means hand-diffing note history.

So this praxis explicitly files a tool desire: **create and integrate into
rekon's docs a tool that helps explore tickets over time** — walking a ticket's
history, and especially surfacing **how internal document links change over
time, in short output form**: a compact diff-style readout (pointers added,
removed, and changed between note revisions), not full dumps of every note.

The substrate already exists: beads records notes and an
[events journal](/constitution/naming/exploration/beads-offerings0.sol56x.md),
and this praxis's append discipline guarantees that pointer history is
structured enough to walk. What is missing is the readout. The desire is filed
as **`rekon-ticket-link-explorer`**; it is a marker, not an authorized build.
A first design pass decides scope, output form, and integration home, and sets
that ticket's concrete acceptance criteria.

<a id="ticket-pointers-motivating-use"></a>
## Motivating use: the figments-rs firma tickets

The commissioning act, in the figments-rs repository: ticket
`figments-rs:firma-eventlog` marks an epic whose body points at
`design/eventlog/eventlog.glm53.md` as the valid start, and ticket
`figments-rs:firma-profiles-plural` carries a pointers note listing four tip
documents. The tickets hold desire and lifecycle; the pointed documents hold
the actual arguments. Both are exactly the kind of ticket that will accumulate
new waves and check-ins — and exactly the kind whose pointers will go quiet
unless someone moves them to the tip.

## Possible ticket agent

**`rekon-ticket-agent`** records an unimplemented possibility: a well-known
project-level agent session that other agents can contact to understand the
board and help place or coordinate work. It may be one instance of a broader
family of project agents. The ticket is an inquiry marker, not build
authorization; see **`rekon-ticket-agent`**. This possibility stays here rather
than in ambient `GLOBAL.md` because it is exploratory and should not silently
impose a new workflow.

## Common ways to miss

- **Archive creep.** Every new document gets added and nothing leaves. Four
  pointers becomes fourteen, and the ticket is again an archive.
- **Set and forget.** Pointers filed once at creation and never revisited. The
  practice is the *maintenance*, not the initial list.
- **History laundering.** Editing old notes so the record always looks current.
  History is append-only; drift is information, not embarrassment.
- **Pointer without work.** A ticket that only curates documents and never
  advances the desire it marks. Pointers serve the work; they are not a
  substitute for it.
- **Snapshot tickets.** Duplicating document content into the ticket body so
  the ticket "stands alone". Now two copies drift. Point, don't copy.

The practice worked if a cold reader can open a ticket, follow two to four
links, and land on the documents that actually govern the work — and if an
auditor can see how those pointers earned their current shape.

## Cross-references

- [Balanced Ticket Coordination](/constitution/README.md#doc-constitution-tickets)
  fixes the division of labor this praxis connects: tickets and documents
  reinforce one another **without one-to-one parity** — the curated pointer is
  the many-to-many edge.
- [Native references across bookmarks, tickets, and knowledge](/design/session-mementos/references0.gpt6a.md)
  supplies the **qualification contract** for pointers that cross projects or
  knowledge kinds: name the owner, the kind, the exact target.
- [Beads system offerings](/constitution/naming/exploration/beads-offerings0.sol56x.md)
  records the **events-journal and history evidence** that a link-history
  explorer would walk.
- [Chase the future](/praxis/chase-future/chase-future0.gpt6a.md) is the
  **posture underneath the maintenance discipline**: point at what the work
  makes possible now, and preserve the history of how the course changed.
- [`/AGENTS.md` beads section](/AGENTS.md#beads) holds the **command
  mechanics** — `bd update --append-notes` and the export conventions this
  practice relies on.
