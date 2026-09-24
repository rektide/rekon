---
type: Praxis
title: Bookmarking — mark the session material worth returning to
description: Practice, engine, and seed vocabulary for bookmarking decision-dense junctions, summaries, and review gates as durable session marks in cotail.
resource: /praxis/bookmarking/README.md
tags: [praxis, bookmarking, bookmarks, cotail, sessions, glossary, crossroads]
status: draft
generated: { by: "llm:glm-5.3-flash-max (opencode agent)", at: 2026-09-18 }
sources:
  - { resource: /praxis/README.md, title: Praxis — Chase the future (the engine-ing doctrine) }
  - { resource: /evidence/README.md, title: Evidence — the owner doctrine }
  - { resource: /design/session-mementos/README.md, title: Session mementos — the designed rekon client of cotail's tools }
  - { resource: "file:///home/rektide/src/cotail/README.md", title: Cotail README — the landed bookmark engine and tag conventions }
  - { resource: "file:///home/rektide/src/cotail/.design/bookmark-praxis/bookmark-praxis.glm53fx.md", title: Bookmark praxis handoff brief (cotail side, ticket cotail-bookmark-praxis) }
  - { resource: "file:///home/rektide/src/cotail/.design/session-context/session-context.glm53fx.md", title: Session-context priming handoff brief (cotail side, ticket cotail-tools-session-env) }
  - { resource: "file:///home/rektide/src/ocyc/design/fleet/crossroads0.glm53max.md", title: crossroads0 — the fleet junction (the anchor crossroads example) }
---

<a id="bookmarking"></a>

# Bookmarking

**Mark the session material worth returning to, while the reasoning is still
inspectable.** Sessions are where decisions, summaries, and gates actually
happen, and transcripts decay into scrollback. A bookmark is a durable,
owner-native mark — a resolvable session target, a note saying why it exists,
free-form tags saying what kind of mark it is — that a later session can find
from either end of time.

This is praxis in [Rekon's sense](/praxis/README.md): a repeatable way of
acting, not a tool manual. The **practice** below says when and why to mark.
The **engine** is cotail's landed bookmark CLI — the posture made useful by a
mechanism, per the engine-ing doctrine. The **seed glossary** records what our
early tags mean; it is convention, not enforcement, and its canonical meanings
file stays cotail-side.

<a id="bookmarking-practice"></a>

## The practice

- **Mark decision-dense junctions.** When an effort stands at a crossroads —
  verified ground inventoried, forward paths laid out with their buys and
  risks, paths that lead back named — bookmark it. A junction is exactly the
  material a later session needs and exactly what scrollback loses. Say in the
  note what the junction was and what would resume it.
- **Mark explicit summaries.** When a session produces an account worth
  returning to — a state map, a wind-down summary, a retained epilogue — mark
  it, so retrieval does not depend on remembering which session said what.
  [Session mementos](/design/session-mementos/README.md) is the designed
  consumer of these marks; it owns the marking semantics and authoring
  practice, and treats bookmarks as its durable substrate.
- **Mark review and design gates.** When work parks awaiting human judgment —
  a design awaiting review, a gate a slice must pass — bookmark the parked
  state. The mark is what makes "awaiting acceptance" findable, not just
  remembered.
- **Write the note as the evidentiary sentence.** A bookmark without a note is
  a pointer to a haystack. One sentence of *why this exists and what it bears
  on* is what makes the mark [evidence](/evidence/README.md) rather than a
  breadcrumb: something that can support or challenge a later judgment under
  stated conditions.

<a id="bookmarking-engine"></a>

## The engine: cotail bookmarks

The mechanism is landed: `cotail bookmark` manages durable bookmarks over
OpenCode V2 sessions in owned `rektide_cotail_*` tables
([cotail README — Bookmarks](file:///home/rektide/src/cotail/README.md)).
Everyday shape:

```sh
# Once per store and source: initialize the schema, register the source.
# The colocated default store IS the live database — initialize deliberately.
cotail bookmark init --bookmark-db ~/.local/share/cotail/bookmarks.db --create-store-file \
  --profile ~/.config/cotail/profiles/opencode-local.json
cotail bookmark register --profile ~/.config/cotail/profiles/opencode-local.json \
  --bookmark-db ~/.local/share/cotail/bookmarks.db

# Mark a session: tags for what kind of mark, a note for why it exists.
cotail bookmark create -s ses_04602d85affe... --tag review --tag design-gate \
  --note "host-design0 in tmux-interface-control awaiting human review"

# The top-down review view: find marks by tag across every registered source.
cotail bookmark list --tag review --tag-match any
cotail bookmark list --tag review --tag design-gate --tag-match all --limit 50

# One session's marks (exact-target mode); the live tag inventory; lineage.
cotail bookmark list -s ses_04602d85affe...
cotail bookmark tags
cotail bookmark tag add bmk_... --tag tmux-interface-control
cotail bookmark tag history bmk_... --tag review

# Prove a mark still resolves to a live session.
cotail bookmark resolve bmk_...
```

Tag adds and removals append immutable timestamped records — removal is a
tombstone, the effective set rolls up to the latest record per tag, and
`cotail bookmark tag history` shows the full lineage. `cotail bookmark tags`
is the live inventory: distinct effective tags with member counts, which is
also the drift view (`review: 14` beside `revew: 1` is how misspellings get
found).

<a id="bookmarking-session-env"></a>

### Session-aware by default (ticketed)

Today `create` requires the session explicitly (`-s <sessionID>`). Once
**`cotail-tools-session-env`** lands (`bd show cotail-tools-session-env`, from
`~/src/cotail`; design:
[session-context brief](file:///home/rektide/src/cotail/.design/session-context/session-context.glm53fx.md)),
cotail primes its identity from the environment that
[opencode-session-id-plugin](file:///home/rektide/src/opencode-session-id-plugin/README.md)
injects — `OPENCODE_SESSION_ID` always, plus `OPENCODE_MESSAGE_ID`,
`OPENCODE_MODEL`, and `OPENCODE_MODEL_VARIANT` when present. Inside an agent
session the marking action becomes one thought:

```sh
# After cotail-tools-session-env: no -s, no lookup step — the ambient session fills it.
cotail bookmark create --tag crossroads --note "why this junction matters"
```

Precedence is explicit user selector > `COTAIL_SESSION_ENV` gate > detected
ambient identity > none, with `--no-session-env` as the per-command opt-out.
Absence of the environment keeps today's behavior byte-identical. Until that
ticket lands, pass `-s` explicitly.

<a id="bookmarking-glossary"></a>

## The seed glossary

Tags are **free-form, case-sensitive, opaque strings** — no reserved
vocabulary, no fuzzy matching, nothing enforced. The entries below are
**convention**: recorded shared meaning so that marks can be read, not rules
that constrain writing. The **live inventory is `cotail bookmark tags`**; the
canonical meanings file will be cotail-side
`doc/bookmarks/tag-glossary.md` — authored, worked, and made visible by ticket
**`cotail-bookmark-praxis`** (`bd show cotail-bookmark-praxis`, from
`~/src/cotail`; design:
[bookmark-praxis brief](file:///home/rektide/src/cotail/.design/bookmark-praxis/bookmark-praxis.glm53fx.md)).
Per the [owner doctrine](/evidence/README.md), the glossary stays with the
engine's owner: cotail owns the meanings file, this page owns the practice and
the pointer — each links to the other.

Each seed gives one worked example, executable against the landed CLI.

<a id="tag-crossroads"></a>

### `crossroads` — a decision-dense junction worth returning to

Declare a crossroads when an effort's grounding, forward paths, and return
paths have been laid out and a later session will need that layout. Anchor
example: [ocyc's fleet junction, `crossroads0`](file:///home/rektide/src/ocyc/design/fleet/crossroads0.glm53max.md)
— the operator-declared junction at fleet-epic completion, which inventories
verified ground (391 green tests, but no acceptance fire and stale quota
export), six forward paths, and the paths that lead back, and which anticipates
cotail's crossroads type in its own bookmark-hooks section. The bookmark one
would make for it:

```sh
# Today, explicitly; after cotail-tools-session-env, drop the -s.
cotail bookmark create -s ses_f5199d455ffeHrDu9Ci2eh6PhX \
  --tag crossroads --tag fleet-complete --tag awaiting-acceptance \
  --note "fleet junction: verified ground, open paths, paths that lead back — see ocyc design/fleet/crossroads0"
```

(The extra tags are the target document's own suggested payload.)

[Fronts and horizons](/praxis/fronts/README.md#crossroads-relation) ⇄ is a
complementary practice for **equipping the choice at the junction**: compare
strategic directions and their gains, then name practical next moves at
different payoff horizons (it also hubs the wider fronts thread). Bookmarking
preserves and retrieves that crossroads; it does not select the course or
authorize the proposed work.

<a id="tag-summary"></a>

### `summary` — an explicit summary mark (a tag today, a purpose to come)

Mark the account a session produced that a later session should read instead
of the transcript. The honest finding: in cotail's model `summary` is
fundamentally a **purpose**, not a tag — and purpose is design-gated. The
landed create contract accepts only `purpose: reference`
(`src/bookmarks/contract.ts`: `purpose` is `z.literal("reference")`), and
summary-shaped marking and lookup wait on ticket
**`cotail-bookmarks-summary-lookup`** with its design at
[`.design/bookmarks/summary0.gpt6a.md`](file:///home/rektide/src/cotail/.design/bookmarks/summary0.gpt6a.md).

```sh
# Today: a reference-purpose bookmark carrying the summary tag by convention.
cotail bookmark create -s ses_04602d85affe... --tag summary \
  --note "explicit summary: session ended at the junction decision, unread follow-ups listed"

# To come, behind cotail-bookmarks-summary-lookup: a true summary-purpose mark,
# retrievable as a summary rather than by tag convention.
```

<a id="tag-review"></a>

### `review` — reviewed or reviewable work

Mark work whose state is "awaiting human review" or "has been reviewed", so
the review queue is findable top-down rather than remembered. This is one of
cotail's own recommended conventions (its `create` help names `review`,
`design-gate`, `decision-point`, and project tags). Example — cotail README's
own everyday case:

```sh
cotail bookmark create -s ses_04602d85affe... --tag review \
  --note "host-design0 in tmux-interface-control awaiting human review"
cotail bookmark list --tag review --tag-match any
```

<a id="tag-design-gate"></a>

### `design-gate` — a gate a course of work must pass

Mark the moment a design gate is set or adjudicated: the bookmark records what
was gated, on what evidence, and what passing would unblock. Combine freely
with `review` when the gate is a review. Example:

```sh
cotail bookmark create -s ses_04602d85affe... --tag design-gate --tag review \
  --note "gate: cotail-bookmark-praxis glossary acceptance before any rekon mirroring"
```

Also live in cotail's recommended conventions: `decision-point` for decision
records mid-stream, and the **project-tag pattern** (`tmux-interface-control`
and the like) — the glossary documents the pattern, it never enumerates
project tags. Whatever tags you use, spell them consistently; the inventory
shows your drift.

<a id="bookmarking-boundary"></a>

## What this does not claim

- **No enforcement.** Free-form means free-form. These seeds are vocabulary in
  use, and `cotail bookmark tags` — not this page — is the live record.
- **No rekon engine.** Rekon registers no CLI command here and installs no
  skill. The engine is cotail's; this home is the practice and the pointer.
- **No glossary duplication.** The canonical meanings file is cotail-side
  `doc/bookmarks/tag-glossary.md` once `cotail-bookmark-praxis` lands; until
  then the tickets and design briefs are authoritative.
- **A bookmark is not acceptance.** Tagging a junction or gate preserves an
  inspectable state; it decides nothing. Decisions belong to their owners.

<a id="bookmarking-cross-references"></a>

## Cross-references

- [Session mementos](/design/session-mementos/README.md) — the designed rekon
  **client** of these tools: multi-session epilogues, explicit summary marks,
  and the ownership split ("Cotail owns source-qualified bookmarks; Rekon owns
  marking semantics and the authoring practice"). Prior art cited here, not
  implemented here.
- Ticket **`cotail-tools-session-env`** (from `~/src/cotail`) and its
  [session-context design](file:///home/rektide/src/cotail/.design/session-context/session-context.glm53fx.md)
  — the ergonomic dependency that makes in-session marking one thought.
- Ticket **`cotail-bookmark-praxis`** and its
  [bookmark-praxis design](file:///home/rektide/src/cotail/.design/bookmark-praxis/bookmark-praxis.glm53fx.md)
  — the canonical, visible tag glossary this page points at, with worked
  invocations per seed tag.
- [Evidence](/evidence/README.md) — the owner doctrine this page applies:
  artifacts stay with their domain owners; `evidence/` — and this praxis —
  connect and explain, they do not relocate.
- **The other praxis.** "Praxis" also names
  [doc/praxis.glm.md](file:///home/rektide/src/doc/praxis.glm.md) in the doc
  repo — the xstate-as-execution-model practice guide (eight core principles,
  grown from the ocyc-sched transformation whose case lineage is cited live in
  ocyc's design records). That is a **domain technique guide**; Rekon's
  praxis/ tree holds **action postures** like this one. Same word, two senses;
  see also the note in [praxis/README.md](/praxis/README.md).
