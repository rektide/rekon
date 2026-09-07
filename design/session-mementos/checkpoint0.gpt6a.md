---
type: ArchitecturalCheckpoint
title: Session mementos — capability split and ordered termination
description: Human-refined display direction, bounded implementation authorization, and unresolved bookmark contracts.
resource: /design/session-mementos/checkpoint0.gpt6a.md
tags: [sessions, epilogue, rules, termination, visits, bookmarks]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
stale_after: 2026-10-06
extensions:
  ticket: rekon-session-mementos
sources:
  - { resource: /design/session-mementos/brief0.gpt6a.md, title: Initial human intent }
  - { resource: "urn:opencode:session:ses_f86cfeb14ffev9dX8mwiS0sZIm", title: Human rule and termination refinements }
  - { resource: "file:///home/rektide/src/opencode-term-v2/.design/term-v2/multi-session-evidence0.gpt6a.md", title: Epilogue source checkpoint }
  - { resource: /design/session-mementos/marking0.gpt6a.md, title: Rekon producer feasibility }
  - { resource: "file:///home/rektide/src/cotail/.design/bookmarks/summary0.gpt6a.md", title: Cotail bookmark proposal }
---

# What changed after the initial brief

The human selected **current + visible tabs**, **sequential narrowing**, and
**keep the current session**, while also strongly endorsing **visited in this
process** as a distinct capability with its own record and ticket. Do not resolve
that combination by secretly choosing one source and discarding the other.
Visible tabs are the proposed first inventory; visited history has its own
implementation/review path and may later be combined with or substituted for it.

The human additionally requires **early termination authority per filter**.
Filters may request termination; a filter without that authority cannot stop
the scan. Ignore its termination signal, not its ordinary keep/drop decision.

The fields displayed for each session must also be modular. Last activity is a
real per-session field, not an incidental side effect of selecting a session.
The old mandatory `Active` row is implementation precedent, not a reason to
exclude it from the requested looker model. Exact default layout remains to be
reviewed; core identity/continue information remains distinct from content.

## Ordered selection contract for the first kernel

The parent authorized a **pure epilogue-domain selector and focused tests**,
not full display wiring, visit tracking, or bookmark implementation:

1. Build a known client inventory separately from selection. The eventual
   initial source is current session plus visible tabs, not every cached session.
2. Pin the current session outside the additional-session pipeline and dedupe it.
3. Walk additional candidates newest-active first, with deterministic identity
   ties. Use `max(updated, idle)` for recorded last activity; running eligibility
   is separate from that historical value. Local visit time is a third fact.
4. Execute filtering stages in their written order. A later stage sees only
   what its predecessor yields; do not reorder rules because they look cheap.
5. A result has an ordinary disposition and an optional termination request.
   A separately declared grant determines whether the runner honors the request.
6. An age rule may request termination only when source ordering proves the
   remainder cannot qualify. Arbitrary tab order cannot supply that proof.
7. A limit stage counts its own input. `recent → take(3)` differs intentionally
   from `take(3) → recent`. A non-terminating limiter can still drop excess items
   without stopping upstream enumeration.
8. Selection precedes content lookers. Freeze remains a pure formatting step over
   retained data, never the time to fetch history or invoke plugins.

**Cost honesty:** sorting a local metadata array already visits that array.
Lazy termination avoids remaining filter/looker work; it does not create an
indexed server cursor or promise complete global discovery. A future ordered
source can exploit the same stopping contract without changing its meaning.

## Representative capabilities

All `...` prefixes below expand to `rekon-session-mementos-display`.

| Ticket | Capability and acceptance seam |
|---|---|
| `...-selection` | Ordered lazy filtering, activity order and explicit termination authority; pure executable contract. |
| `...-inventory` | Enumerate current session plus this client's visible tabs, with honest shared-storage semantics. |
| `...-visits` | Record actual process-local visits independently from tabs and server-global viewed time. Decide retention and identity before persistence. |
| `...-batch` | Retain and print multiple session envelopes atomically, freeze before teardown, preserve once-only output and isolation. |
| `...-lookers` | Assemble per-session fields in order, including last activity and plugin contributions, without a JSX-slot rewrite. |
| `...-excerpts` | Safe bounded delivered-question/response excerpts with explicit cache, pending-input, revert and partial-output behavior. |
| `...-summaries` | Read already-bound Cotail summary marks for selected sessions while live; never infer a summary or perform IO at exit. |

The display epic tracks the combined user outcome; the capability tickets can
carry independent evidence and acceptance. The selector alone does not make the
multi-session feature visible. The visits ticket is not a blocker for visible-tab
output. Bookmarks are not a blocker for ordinary activity or Q/response output.

## Bookmark review: agreement is not yet one accepted contract

Both owner-authored drafts agree on source-qualified exact targets, explicit
summary provenance, separate pending intent versus bound bookmark, Cotail-owned
atomic binding, and honest `needs-review` outcomes when continuity is unprovable.
An external Rekon tool appears feasible, but a reliable next-response binder is
not supplied merely by a live plugin subscription or Cotail's sampled watch.

The parent read both full relevant contract sections and found material choices:

| Question | Cotail proposal | Rekon producer proposal | Current disposition |
|---|---|---|---|
| Mixed visible text and local tools | First text-bearing successfully settled Step counts. | Deferred selection waits for a narrower tool-free/settled `stop` response. | Unresolved: next visible response and next final-like response are different selectors, not synonymous implementations. |
| Explicit partial/failed historical summary | Allowed with outcome honestly retained. | Initial summary producer requires completed non-error assistant text. | Prefer preserving exact user selection with honest outcome; producer restriction still needs review. |
| Arming an idle child | First delivered user input establishes context. | Initial policy requires ongoing child work. | Defer idle-child intent semantics; exact child targets and child self-marking remain required. |
| New input / interrupted candidate | End unbound with explicit cause. | Cancel or needs-review, depending on evidence. | Agree no silent retargeting; normalize lifecycle vocabulary before code. |

Do not average these differences into an ambiguous “next summary” promise. An
exact historical-mark slice can precede the deferred binder. No marking guidance
has been installed, and no bookmark runtime or live database write is authorized.

## Source and channel finding

Cotail's profile label and executable facts are not source identity. Its effective
query locator is the explicit `--db` override or the selected profile path. The
inspected OpenCode implementation gives `OPENCODE_DB` precedence; `local` normally
uses `opencode-local.db`, while several release channels share `opencode.db`.
The default bookmark store must follow the **resolved source**, not the spelling
`opencode`, a guessed channel, or the TUI client's storage channel.

The Cotail draft proposes explicit writes to `rektide_cotail_*` owned tables and
an alternate bookmark DB, leaving ordinary query connections read-only. This is
a reviewed proposal's direction, not an installed schema or proof that current
profiles authenticate their database files.

## Cross-references

- [Original brief](/design/session-mementos/brief0.gpt6a.md) — preserves the request
  before the human sharpened visit recording and terminating authority.
- [Epilogue evidence](file:///home/rektide/src/opencode-term-v2/.design/term-v2/multi-session-evidence0.gpt6a.md)
  — actual single-session ownership, shared tabs, incomplete history and test seams.
- [Rekon marking proposal](/design/session-mementos/marking0.gpt6a.md#marking-binding)
  — structural response selection and host limitations, not a semantic-final hook.
- [Cotail proposal](file:///home/rektide/src/cotail/.design/bookmarks/summary0.gpt6a.md)
  — selectable writer, source catalog and differing next-visible-response policy.
- [Ordered-pipeline memento](/design/ordered-pipelines/memento0.gpt6a.md)
  — broader shared pattern, concrete Watchwoman precedent and limits of unification.
