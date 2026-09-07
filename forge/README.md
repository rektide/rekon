---
type: Practice
title: Forge — collecting and judging an upstream project's tracker signal
description: A read-only practice for gathering issues, PRs, discussions, and role evidence from a project's forge, then assessing which threads actually matter for our work, using Apache Iggy as the running case study.
resource: /forge/README.md
tags: [forge, github, upstream, triage, evidence, provenance, research, priority]
status: draft
generated: { by: "llm:glm-5.3", at: 2026-09-07 }
stale_after: 2026-12-07
sources:
  - resource: file:///home/rektide/archive/apache/iggy/.design/frontiers/frontiers0.glm53.md
    title: Iggy future-frontiers report (the case study this practice generalizes)
  - resource: file:///home/rektide/archive/apache/iggy/.test-agent/iggy-frontiers/README.md
    title: Iggy corpus manifest (coverage, queries, limitations)
  - resource: /digest/README.md
    title: Digest — finding the living state of an effort
  - resource: /evidence/README.md
    title: Evidence relationships and provenance
  - resource: /upstream/README.md
    title: Upstream reference mirrors (code-side sibling of tracker-side collection)
---

<a id="forge"></a>
# Forge

**Read an upstream project's tracker the way an archivist would, then judge
it the way an engineer would.** A *forge* pass gathers the project's issues,
pull requests, discussions, reviews, and role evidence under explicit
provenance, and turns them into a defensible answer to: *which upstream
threads are real, who carries them, and which of them should change what we
do next?*

This document is a practice guide grown out of one concrete investigation:
the [Apache Iggy future-frontiers research](file:///home/rektide/archive/apache/iggy/.design/frontiers/frontiers0.glm53.md)
(September 2026). Iggy is the running case study throughout — every
technique below is one that worked (or was visibly missing) there, with
canonical links so the examples can be checked.

## Why this is a practice and not a one-off

The [Digest practice](/digest/README.md) asks "where are we now, and what is
this becoming?" for an effort. A forge pass is the upstream-facing
instrument of that question: most interesting dependencies change faster
than their documentation, and their *tracker* — issues, PRs, discussions,
votes — is where the future is negotiated in public before it exists in
releases. [Chase the future](/praxis/chase-future/chase-future0.gpt6a.md)
then needs exactly this: current upstream possibilities, sorted by evidence
rather than enthusiasm, with alternate courses preserved.

The Iggy case showed the stakes concretely. The headline question — *does
Iggy support fan-out, or one consumer per stream?* — is answered "yes,
fan-out" by code at the pin, but the surrounding futures that made it matter
to us (server-push, offset custody, connector guarantees) split into: merged
but unreleased ([VSR / 0.9.0 release discussion](https://github.com/apache/iggy/discussions/3885)),
in active review ([sink-controlled offset commit #3954](https://github.com/apache/iggy/pull/3954)),
design-only ([deferred-response long polling #2854](https://github.com/apache/iggy/discussions/2854)),
explicitly rejected ([queue semantics #3648](https://github.com/apache/iggy/discussions/3648)),
and maintainer-sequenced-behind-something-else ([transport work after VSR,
#3419](https://github.com/apache/iggy/issues/3419)). One number or one label
cannot hold that; a corpus plus a disciplined reading can.

## The corpus: own it beside the source, not inside the consumer

Keep the collection **source-project-local**: a scratch corpus under the
upstream checkout (Iggy: `.test-agent/iggy-frontiers/` inside
`~/archive/apache/iggy/`), and the durable, link-rich report under the
project's own docs tree (Iggy: `.design/frontiers/`). Consumer projects keep
only their own conclusions and tickets. In the Iggy case this was learned by
correction: the corpus began in the consumer repo (`intrlnk.at`) and was
relocated intact, with the query log preserved, once the human pointed out
that research about Iggy belongs with Iggy where future passes will look for
it.

Split the corpus into three layers, and never let them blur:

| Layer | Iggy example | Version control |
|---|---|---|
| **Raw snapshots** — primary JSON/HTML exactly as fetched, dated | `github/issues-open-p{1,2}.json`, `github/discussions/discussion-3885.html` | ignored (`.test-agent/`), never force-added |
| **Derived indexes** — your interpretation as data | `derived/frontiers-index.json`, `derived/people.json`, `derived/open-issues-index.json` | ignored with the corpus, but regenerable from raw + documented transforms |
| **Durable report** — argued conclusions with canonical links | `.design/frontiers/frontiers0.glm53.md` | committed |

Raw snapshots are *versioned by capture date*, not by commit: record
`captured_at` UTC, the source revision pin, and — critically — the
distinction between the pin and public state. For Iggy the local checkout
happened to equal `origin/master` (`25e9d44d`) at capture, but public
releases stopped at `server-0.8.0`: VSR was **merged but unreleased**. A
local checkout proves what is merged; only release tags, package registries
(crates.io showed SDK `0.10.0` stable / `0.11.0-edge.6`), or vote threads
prove what shipped. Local history is *not* proof a proposal is public, and
local commits are not proof of release.

## Coverage you can audit

A corpus nobody can audit is a hallucination with extra steps. The Iggy
manifest records, per dataset: coverage, completeness, and how it was
checked.

- **Full pagination where the question is "what exists."** All open issues
  were enumerated (`/issues?state=open`, 2 pages × 100, Link-header-driven)
  and reconciled against the repo's own `open_issues_count` (186 = 186).
- **Sampling where the question is "what shape."** Merged PRs in a 180-day
  window were *counted* via search (682) but only *sampled* (100 most
  recently updated); the volume and authorship claims were then
  cross-checked against local `git log` (681 first-parent commits) — free,
  complete, and independent of API budgets.
- **Targeted depth over breadth.** Comment threads and review streams were
  fetched in full only for the ~20 threads that carried frontier questions.
- **Honest absences.** The dev@iggy.apache.org mailing list (the official
  vote venue) is JS-gated for scripted reads; the corpus records that the
  graduation thread's content came from a maintainer's verbatim repost
  ([#3597](https://github.com/apache/iggy/discussions/3597)) rather than
  pretending the list was captured.

Every fetch is one line in a `queries.jsonl` (timestamp, path, HTTP code),
and the fetch helpers (`ghget.sh`, `ghsearch.sh`) are part of the corpus, so
a future pass can re-run or extend rather than guess what happened.

## Reading the tracker correctly

### Separate true issues from PR entries

GitHub's issues API returns pull requests mixed in. Split on the
`pull_request` key before counting anything: Iggy's 186 open entries were
142 issues + 44 PRs, with different label vocabularies on each side (issues:
`connectors`, `gateways`, `good first issue`; PRs: `S-waiting-on-author`,
`S-waiting-on-review`, `S-stale`). Confusing them makes "open problem
count" and "open change count" into the same wrong number.

### Closed ≠ done, and closed ≠ dead

Read `state_reason` and the closing comments before summarizing. Three Iggy
closures with three different meanings:

- [#290 tiered storage](https://github.com/apache/iggy/issues/290) — closed
  *implemented* (archiver PRs #1053/#1055 landed in 2024).
- [#269 log compaction](https://github.com/apache/iggy/issues/269) — closed
  *abandoned-by-architecture-change* ("our architecture has changed so
  much").
- [#2562 clustering-status docs](https://github.com/apache/iggy/issues/2562)
  — closed for contributor inactivity, with the underlying question
  answered later by shipped code.

Age alone is not evidence of abandonment either: Iggy's [VSR issue
#1914](https://github.com/apache/iggy/issues/1914) sat quiet for months and
then landed as the 0.9.0 centerpiece.

### Find the venue where design actually happens

READMEs and contributor guides tell you. Iggy's `AGENTS.md` says *"Design:
github.com/apache/iggy/discussions"*, and the community enforces it — a
maintainer's process note on [#3228](https://github.com/apache/iggy/issues/3228)
asks that big features "land as a GitHub Discussion before an issue."
Discussions carry a `design` label and a "Discussions needing community
input" tag, which is their de-facto RFC convention. Enumerate categories
(Announcements / General / Ideas / Polls / Q&A / Show and tell), then index
the design-labeled threads. For Iggy this surfaced the entire real frontier
surface — long polling, tiering, DLQ, queue semantics, SHM transport,
graduation — none of which was visible from open-issue titles alone.

### Comments, reviews, and votes are the substance

Titles are marketing; review threads are the product. The single richest
artifact in the Iggy pass was the review stream of
[PR #3975](https://github.com/apache/iggy/pull/3975) (39 review events): the
reviewer *reproduced crash shapes by hand* ("a clean restart re-mints offset
0"), iterated three rounds of changes-requested, and a second maintainer
approved — that is what "real and moving" looks like, versus the release
thread's seven 🎉 emoji reactions, which are popularity, not engagement.

### Linked evidence changes an item's meaning

Treat each thread as a node and follow its links before judging it:

- An issue + an open PR + a maintainer's changes-requested review = in
  flight ([#2928](https://github.com/apache/iggy/issues/2928) →
  [PR #3954](https://github.com/apache/iggy/pull/3954), review verifying
  "the core claim holds").
- A merged PR can still be *dead history*: [PR #544](https://github.com/apache/iggy/pull/544)
  (concurrent QUIC streams, merged 2024) mattered only until the legacy
  server it touched was deleted by the server-ng promotion; the current
  transport is serial by design and names pipelining as *future* work.
- A proposal "closing" into a bigger venue is progress:
  [#2927](https://github.com/apache/iggy/issues/2927) →
  [unified design discussion #3518](https://github.com/apache/iggy/discussions/3518).
- Releases re-contextualize everything below them: 0.9.0's breaking wire
  protocol and no-migration data directories made every pre-0.9 benchmark
  and SDK observation historical.

## Roles: verify, then label the basis

Three different systems are constantly confused. Keep them separate:

1. **Governance roles** (committer, PMC, chair) come from the project's
   official roster, not GitHub. Iggy's [team
   page](https://iggy.apache.org/community/team/) lists 9 PPMC members and
   12 committers with Apache IDs — that page is the citation, not a badge.
2. **GitHub metadata** (`author_association`, Collaborator badge) is
   context: "has had a PR merged" or "has repo write access," respectively.
   The Iggy corpus carries a built-in falsification example: `numinnex` and
   `slbotbm` hold Collaborator badges but appear on no Apache roster, while
   most PMC members' day-to-day GitHub associations read mere CONTRIBUTOR.
3. **Inferred maintainership** from merged work (e.g., Grzegorz Koszyk's 47
   core-crate commits in 180 days) is a separately labeled inference —
   strong, but say what it is.

Name-to-identity mappings (which GitHub account *is* this Apache committer)
should be verified by name + activity overlap and recorded with their basis
(Iggy: `derived/people.json` marks `hubcio` = Hubert Gruszecki as verified,
and leaves Koszyk's GitHub account honestly unmapped). Comment counts,
stars, and assignments are never role evidence. Bots are identified and
excluded from human claims — dependabot authored 142 of Iggy's 681 merged
commits in the window, which would otherwise make it the project's second
most active "engineer."

## Substantive engagement vs popularity

Record *support and dissent as arguments*, not tallies. From the Iggy pass:

- Substantive support: a maintainer reproducing bugs and approving after
  three review rounds (#3975); the founder endorsing a DLQ envelope design
  in [#3100](https://github.com/apache/iggy/discussions/3100).
- Substantive dissent: [queue semantics #3648](https://github.com/apache/iggy/discussions/3648),
  where the creator answered "total infrastructure rewrite… not something we
  would like to incorporate in the near future," and the top committer
  pushed back on vagueness — capturing the objection matters more than the
  +1 that opened the thread.
- Lively disagreement *inside* the maintainers: on
  [#2854](https://github.com/apache/iggy/discussions/2854) the top committer
  rejected a collaborator's WASM-notification idea in favor of long polling;
  both positions are recorded, and neither is yet "the roadmap."
- Silence is data: a well-formed proposal with zero maintainer response
  ([message filtering #3428](https://github.com/apache/iggy/discussions/3428))
  is *low signal*, not *negative* signal — say so rather than guessing.

## Assessment: separate dimensions, then triage

Do not collapse engagement, readiness, relevance, and risk into one score.
In Iggy terms:

| Dimension | Question | Iggy illustration |
|---|---|---|
| **Engagement** | Who is actually carrying it, with what effort? | #3975: 39 review events, two maintainers. #3428: nobody replied. |
| **Readiness** | Idea / discussion / RFC / open PR / merged / released? | VSR: merged, unreleased. Long polling: design text only. Tiering: explicitly deferred "until after VSR." |
| **Relevance** | Does it change *our* design, whoever wants it? | QUIC pipelining is low upstream urgency but directly shapes our ingress runner's concurrency contract. |
| **Risk** | What does trusting it cost if it stalls or breaks? | Core carried by ~2-3 people (bus factor); 0.9.0 break means probes must pin exact edge pairs and may not transfer. |

Then triage with **interpretable levels and stated bases**, never a magic
numeric score. The Iggy report used high / medium / low / rejected /
unclear, each with its evidence list attached — a reader can disagree with
the level because they can see the basis. "High" meant *merged code at the
pin plus named verified-role carriers*; "medium" meant *open PR or design
with substantive maintainer engagement*; "rejected" required a maintainer
saying so on the record.

One more separation the Iggy case forced: **research priority is not
upstream priority.** Iggy's maintainers sequence VSR → SDK migration →
everything else; our ingress questions ranked connector custody and QUIC
concurrency first even though upstream defers both. Report both orderings:
what the project will build next, and what matters to the consumer anyway,
so nobody mistakes their queue for ours.

## Limits, noise, and ethics

- **Rate and API limits shape method.** The Iggy pass ran with an invalid
  `gh` token: 60 unauthenticated core calls/hour meant full enumeration was
  spent on the open set, search's separate 10/minute bucket carried topic
  discovery, Discussions (GraphQL-gated) came from HTML snapshots, and local
  git substituted for merged-PR pagination entirely. Report what the budget
  prevented rather than quietly thinning coverage.
- **No claims beyond observed sources.** If the mailing list was not
  captured, say so; if a mapping is unverified, leave it unmapped; if a
  discussion was only partially parsed, list which ones. Uncertainty is
  part of the deliverable.
- **Read-only, always.** No comments, votes, reactions, issues, or posts.
  Least-privilege access where auth is used; never write tokens into
  commands, logs, scripts, JSON, or docs. Respect that some venues (Apache
  votes) are observable but not ours to participate in from a research pass.
- **Not GitHub everywhere.** The practice generalizes: git history stands in
  for merge data; registries (crates.io/npm/PyPI) for release truth;
  official rosters and cwikis for governance; mailing-list archives for
  decisions; company blogs and conference talks for direction. Forges other
  than GitHub (GitLab, Gerrit, Sourcehut, Jira) have analogous artifacts —
  enumerate *their* equivalents rather than assuming GitHub's schema. What
  stays constant is the layered corpus, the provenance, the role
  verification, and the dimensional assessment.

## Turning findings into local work

Translate, don't transcribe:

- **Do not import the upstream tracker.** Iggy had 186 open entries; the
  consumer's board needed one epic (`intrlnk-iggy`) and one open spike
  (`intrlnk-iggy-ingress-spike`). The corpus is citable backing, not a
  backlog copy.
- **Encode futures as conditions, not dependencies.** "Re-price the
  connector sink path *when* #3954 merges and releases" watches upstream
  without fabricating a dependency on volunteers we don't employ.
- **Don't mark open questions settled.** Fan-out semantics, offset
  durability, and delivery model each got "as of pin/capture" phrasing with
  the exact revision and date, because all three were visibly in motion.
- **Keep the consumer's architectural decisions at home.** The Iggy corpus
  informed the [Intrlnk ingress baseline](file:///home/rektide/src/intrlnk.at/design/iggy-ingress/iggy-ingress.gpt6astra.md);
  it did not adopt Iggy, choose transports, or file implementation tickets
  on its behalf.

## Refreshing

A forge corpus decays by design. Refresh means: re-run the query log, diff
the open set, re-capture changed threads of record, re-derive the indexes,
and write the *deltas* into the durable report (dated addenda beat silent
rewrites). Supersession chains matter more on refresh than on first pass —
last cycle's "open PR" may now be three merged commits plus a follow-up
issue, and the report should show the chain, not just the latest state.

## Cross-references

- [Digest](/digest/README.md): a forge pass is upstream-facing input to a
  digest; the digest owns the "what is this becoming" narrative, the forge
  corpus owns its raw upstream evidence.
- [Evidence](/evidence/README.md): claim ↔ evidence ↔ provenance is the
  relation every line of a forge report instantiates; raw snapshots are
  provenance-bearing evidence, not conclusions.
- [Upstream reference mirrors](/upstream/README.md): the code-side sibling —
  snapshot upstream *source* with provenance before adapting it; forge
  collection snapshots upstream *decision state* the same way.
- [Chase the future](/praxis/chase-future/chase-future0.gpt6a.md): the
  consumer of triaged possibilities; its opportunity-opening slices should
  cite forge evidence for what is actually available upstream.
- [Commit and checkpoint discipline](/code/commits.md): commit the durable
  report stage before revising it; the corpus itself stays ignored with its
  manifest as the map.
- Iggy case study artifacts (outside this repo):
  [frontiers report](file:///home/rektide/archive/apache/iggy/.design/frontiers/frontiers0.glm53.md),
  [corpus manifest](file:///home/rektide/archive/apache/iggy/.test-agent/iggy-frontiers/README.md),
  [parent brief](file:///home/rektide/archive/apache/iggy/.design/frontiers/brief0.gpt6astra.md).
