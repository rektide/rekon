---
type: ConstitutionAdoptionClause
title: Adoption, migration restraint, and upstream intent
description: >
  A normative adoption clause for the draft documentation constitution plus an
  operational background-agent proposal procedure, an upstream-intent decision
  table, safeguards against automatic migration, and open ambiguities.
resource: /design/doc-constitution/adoption0.glm53fm.md
tags: [documentation, constitution, adoption, migration, upstream]
status: draft
generated: { by: "model:glm-5.3-flash-max", at: 2026-09-01T22:15:08-04:00 }
sources:
  - id: constitution
    resource: /constitution/README.md
    title: Self-Explaining Documentation Constitution (draft)
    author: model:gpt-5.6-terra
  - id: synthesis
    resource: /design/doc-constitution/syn0.gpt56t.md
    title: Self-explaining documentation constitution synthesis
    author: model:gpt-5.6-terra
  - id: workspace-guidance
    resource: /AGENTS.md
    title: Workspace agent guidance
    author: human:rektide
  - id: human-direction
    resource: urn:opencode:session:ses_fa0263b29ffejUmXXWHEKOriP0
    title: Adoption trial, small-project proposal flow, and upstream-intent direction
    author: human:rektide
---

<a id="adoption-clause"></a>
# Adoption And Migration Restraint

The constitution ([`/constitution/README.md`](/constitution/README.md)) already says it is a
prospective default: existing documents are grandfathered and adoption is not a
demand to rewrite the corpus
([`doc-constitution-lifecycle-existing`](/constitution/README.md#doc-constitution-lifecycle-existing)).
This artifact turns that posture into an operable adoption clause responding to
a specific human direction (Sep 1 2026):

1. The constitution is **still on trial** — being tried, shaken out, iterated,
   and assessed. It must not become a general migration target yet.
2. For projects with **fewer than 16 documentation files**, adopting the
   module shape is a **tentative recommendation**. A background Flash
   Max-class agent may inspect and **propose** a migration; it must never
   perform one without acceptance.
3. **Upstream-bound work adapts to upstream conventions** rather than imposing
   our layout.
4. Unless work is known to be upstream-bound, prefer our domain subdirectory
   patterns (`design/<topic>/`, `doc/<topic>/`, or an explicitly documented
   local equivalent) so workspace-only artifacts stay **path-local**: easy to
   filter, carry across upstream updates, or remove deliberately.
5. Treat `README.md` / `SKILL.md` / `GLOBAL.md` as **foreign or meaningless in
   upstream-owned trees**; do not scatter workspace-only machinery through
   upstream layouts.

Sections below deliver, in order: the normative clause ready to synthesize into
[`/constitution/README.md`](/constitution/README.md); the operational procedure for the <16-doc
proposal flow; the upstream-intent decision table; safeguards; and unresolved
ambiguities for human attention.

---

<a id="adoption-clause-normative"></a>
# 1. Normative Adoption Clause (for `/constitution/README.md`)

Ready to synthesize as a new section following
[`doc-constitution-lifecycle-existing`](/constitution/README.md#doc-constitution-lifecycle-existing).
Suggested anchor IDs follow the constitution's inductive refinement
(`doc-constitution-adoption-*` under `doc-constitution`).

````markdown
<a id="doc-constitution-adoption"></a>
## Adoption, Trial Status, And Upstream Boundaries

This constitution is a prospective default under active trial. It governs new
knowledge work immediately; it does not yet make the workspace's other
repositories a migration target. Adoption elsewhere is deliberate,
per-project, and reversible. A project becomes a migration target only by
human decision, after this constitution has stabilized and proven itself.

<a id="doc-constitution-adoption-small"></a>
### The Small-Project Path

For a project with fewer than 16 tracked documentation files, adopting the
module shape is a tentative recommendation, not a requirement. A background
agent MAY inspect the project and produce a migration proposal. The agent MUST
NOT edit, move, delete, commit, or symlink anything in that project: it
proposes, and a human accepts before any change occurs. Projects at or above
the threshold are not excluded from adoption; they simply require the same
deliberate human-led decision without the lightweight proposal shortcut.
Declining a proposal is a normal outcome, not a failure of the project.

<a id="doc-constitution-adoption-upstream"></a>
### Upstream Intent

Much workspace work happens in repositories whose documentation is owned
upstream. Work intended for upstream MUST adapt to that project's existing
documentation directories, structure, and tone; this constitution's layout
MUST NOT be imposed on it. `README.md`, `SKILL.md`, and `GLOBAL.md` have
different or no meaning in such trees: a foreign `README.md` is not our module
landing page, and workspace machinery MUST NOT be scattered through
upstream-owned files or added to upstream configuration.

Unless work is known to be upstream-bound, it SHOULD live in workspace domain
subdirectory patterns — `design/<topic>/`, `doc/<topic>/`, or an explicitly
documented local equivalent — so workspace-only artifacts stay path-local:
easy to filter out of upstream diffs, carry across upstream updates, and
remove deliberately. When local exploration matures into an upstream
contribution, the distilled content is ported into the upstream project's
conventions; the machinery that produced it stays behind.
````

Notes for the synthesizer:

- The clause deliberately adds only one MUST about mutation ("MUST NOT edit,
  move, delete, commit, or symlink") and keeps the rest MAY/SHOULD, matching
  the constitution's proportional-process principle.
- "Explicitly documented local equivalent" inherits the constitution's
  permission for local deviation
  ([`doc-constitution-scope`](/constitution/README.md#doc-constitution-scope)); where
  the documentation of the deviation lives is an open question
  (see §5.6).
- The threshold number (16) is stated as a constitutional constant but is
  cheap to revise by amendment; it gates *proposing*, never *doing*.

---

<a id="adoption-clause-procedure"></a>
# 2. Operational Procedure: Background Proposal For A <16-Doc Project

Executor: a background subagent (Flash Max-class, e.g. `glm53fm`), dispatched
so it never blocks the primary session. Dispatch happens when a human or the
session lead raises adoption for a specific project — **not** automatically
whenever an agent happens to touch a small repository.

## Step 0 — Classify before counting

Before any proposal work, determine the project's situation using the
[upstream-intent decision table](#adoption-clause-table):

- Where does the checkout live? `~/src/<name>` (first-party) vs
  `~/archive/<org>/<repo>` (third-party reference, per workspace guidance in
  [`/AGENTS.md`](/AGENTS.md)).
- `jj git remote list` / `git remote -v`: does it point at an upstream the
  workspace does not own? Are there contribution signals (`CONTRIBUTING`,
  patch mailboxes, PR templates, forge bookmarks)?
- Is the current workstream known to be upstream-bound?

If the situation is E (read-only mirror), the agent reports the classification
and stops: no proposal, no overlay, notes belong elsewhere.

## Step 1 — Count documentation files

Primary count (tracked files; workspace-owned content only, scratch like
`.test-agent/` is ignored so it does not pollute the count):

```sh
jj file list | rg -c '\.md$'
```

Fallback when not a jj/git repo or when untracked-but-visible files matter:

```sh
fd -e md -t f --exclude node_modules --exclude .git --exclude .jj \
  --exclude build --exclude dist --exclude vendor | wc -l
```

Rules:

- Record the exact command and raw output in the proposal; the count must be
  reproducible and auditable.
- Default scope is tracked `.md` files, excluding vendored, generated, and
  dependency trees. Whether `.mdx`/`.rst`/`.adoc` and housekeeping files like
  `CHANGELOG.md` count is an open question (§5.1); state the choice made.
- **If the count is ≥ 16**: stop. Report "tentative small-project path does
  not apply; adoption requires deliberate human-led planning." Do not produce
  a migration proposal. An inventory-only memo is acceptable if the requester
  asked for one.

## Step 2 — Read-only inventory

For every counted file, record: path, one-line purpose, inferred audience
(Diataxis mode where obvious), and a verdict:

| Verdict | Meaning |
|---|---|
| module-candidate | Belongs in a proposed `design/<topic>/` or `doc/<topic>/` |
| keep | Fine where it is (root README, upstream convention files) |
| upstream-owned | Do not touch; layout belongs to the upstream project |
| scratch-or-stale | Note only; **never** propose deletion (see §4.3) |

## Step 3 — Compose the proposal

Propose module grouping using our domain subdirectory patterns (or the
project's documented local equivalent if it has one). Every proposed change is
a move that preserves history (`jj rename`/copy semantics) or an annotation
(frontmatter, anchors). **Zero deletions. Zero rewrites of body prose.**

Machinery decisions are conservative by default:

- `SKILL.md` symlinks: propose only where on-demand routing value is concrete;
  state the discovery caveat from
  [`doc-constitution-skill`](/constitution/README.md#doc-constitution-skill).
- `GLOBAL.md` fragments: default to **none**. Ambient context is a budget and
  a fresh migration has not earned ambient load.
- Anchors and OKF frontmatter: proposed for module READMEs, not sprayed over
  support files.

## Step 4 — Plan upstream interaction

- Workspace-only artifacts (overlay dirs, `.test-agent/`) should be hidden
  from the project's VCS via `.git/info/exclude` (or the jj equivalent) rather
  than by editing the project's `.gitignore`, which upstream owns.
- Flag future merge/rebase friction for any tracked path that diverges from
  upstream layout, and prefer ignored overlay paths when friction is real.
- For situations C/D (third-party checkout), the proposal covers the overlay
  only; upstream files are listed as `upstream-owned` and out of scope.

## Step 5 — Write the artifact, return, stop

Write the proposal to the target project's git-ignored scratch, as a
model-suffixed wave file that never overwrites peers:

```text
<project>/.test-agent/doc-migration/proposal0.<model>.md
```

(Adapt the path if the target project documents a different scratch
convention.) Return to the caller: the artifact path, the count with command,
the classification (table row), a five-line summary, and this sentence,
verbatim in spirit: *"No changes were made, and none will be until a human
accepts this proposal."*

### What a proposal returns (contract)

1. Frontmatter: model, timestamp, target repo, count command + output,
   upstream-intent classification.
2. Inventory table (Step 2 verdicts).
3. Proposed mapping: `old path → new path`, action ∈ {move, annotate, leave}.
4. Explicit non-actions: everything the migration would not touch.
5. Machinery plan: skills / globals / anchors / frontmatter, with defaults
   from Step 3.
6. Upstream plan: ignored paths, exclude mechanism, friction notes.
7. Open questions for the human.
8. Acceptance checklist: `accept / amend / reject`, with a note that execution
   is a separate reviewed task (small, history-preserving commits — never
   squash; see [`jj` guidance in /AGENTS.md](/AGENTS.md)).

---

<a id="adoption-clause-table"></a>
# 3. Upstream-Intent Decision Table

| # | Situation | Doc home for this work | Conventions followed | Workspace machinery (`SKILL.md`/`GLOBAL.md`/waves) | Proposal behavior |
|---|---|---|---|---|---|
| A | First-party repo; workstream is workspace-local | `design/<topic>/`, `doc/<topic>/` under the project | Ours | Allowed, path-local | Full module-shape proposal (tentative when <16 docs) |
| B | First-party repo; workstream destined upstream | Contributed content in upstream's doc dirs; supporting evidence in our overlay | Upstream's for contributed files; ours for overlay | Overlay only; never in contributed files | Overlay proposal; content port follows upstream review |
| C | Third-party checkout (`~/archive/<org>/<repo>`); local exploration only | Overlay: `design/<topic>/` committed locally-only, or ignored scratch — chosen deliberately | Ours for overlay files; upstream's for everything else | Confined to overlay | Overlay-only proposal; upstream docs marked do-not-touch |
| D | Third-party checkout; work intended as upstream contribution | Draft in overlay; final text enters upstream's docs via PR/patch | Translate our patterns: anchors → upstream heading style; OKF frontmatter dropped or minimized to upstream taste; links rebased | Stays behind entirely | Adaptation mapping, not migration |
| E | Mirror / reference-only checkout | None here — notes go in the depending project's `design/` or `~/src/doc` | n/a | None | Refuse; point to the note-home |

### What `README.md` / `SKILL.md` / `GLOBAL.md` mean upstream

| File | Meaning in an upstream-owned tree | Consequence for us |
|---|---|---|
| `README.md` | The project's landing page, owned and shaped by upstream | Never convert it into an OKF module page in place; in an accepted overlay adoption, our module READMEs live in overlay dirs while the root README remains upstream's |
| `SKILL.md` | Not an upstream concept at all | Do not add it to upstream-owned layouts; expose upstream-domain knowledge as skills from workspace-side paths or harness config. Allowed inside overlay modules of locally-carried checkouts (situations A–C) |
| `GLOBAL.md` | Our AGENTS.md assembly input; foreign upstream | Never upstreamed; a rule that must be ambient for our agents working in that repo is hosted in the overlay fragment or the workspace-level AGENTS.md chain, not in upstream files |

The recurring test: **would this path survive `jj`/`git` rebase onto a fresh
upstream pull without conflict or apology?** If not, it belongs in the overlay
or the workspace, not the shared tree.

---

<a id="adoption-clause-safeguards"></a>
# 4. Safeguards

1. **Proposal-only mandate.** The background agent is read-only with respect
   to the target project: no edits, moves, deletions, commits, symlinks, or
   config changes. One artifact is produced; execution requires explicit
   human acceptance that references the proposal, and is a separate reviewed
   task.
2. **No migration target while on trial.** The constitution is draft until the
   torch passes (`rekon-doc-constitution-torch`). No batch conversions, no
   compliance sweeps, no "bring the corpus into conformance" passes.
   Grandfathering
   ([`doc-constitution-lifecycle-existing`](/constitution/README.md#doc-constitution-lifecycle-existing))
   remains in force; repair happens only when a file is materially reworked
   anyway.
3. **No pruning, ever, by adoption.** Adoption never deletes or relocates-to-
   oblivion stale artifacts; pruning is outside scope per workspace guidance.
   Scratch-or-stale files get a note in the proposal, and cleanup stays a
   deliberate human act.
4. **No layout imposition upstream.** Upstream trees keep their directories,
   naming, tone, and tooling. Workspace machinery never lands in files or
   config upstream owns; hide overlay paths via `.git/info/exclude` rather
   than editing upstream `.gitignore`.
5. **Reversibility.** Executed migrations use history-preserving moves,
   small logical commits, human review; no squash. Any accepted migration can
   be carried forward or dropped on the next upstream pull.
6. **Ambient budget protection.** Proposals default to zero `GLOBAL.md`
   fragments; skill exposure requires demonstrated routing value and, per the
   constitution, skill discovery is not guaranteed by a symlink alone.
7. **Threshold honesty.** <16 gates *proposing*, never *doing*: it neither
   obligates small projects nor excludes larger ones from deliberate
   adoption.
8. **Scratch discipline.** Proposal artifacts are model-suffixed, live in
   ignored paths, and never overwrite peers — the same-wave rules of
   [`doc-constitution-history`](/constitution/README.md#doc-constitution-history)
   apply to proposals too.
9. **Auditability.** Every proposal carries the count command and raw output
   so its classification can be reproduced; miscounts invalidate the proposal,
   not the project.

---

<a id="adoption-clause-ambiguities"></a>
# 5. Unresolved Ambiguities Worth Human Attention

1. **Count definition.** Do `.mdx`, `.rst`, `.adoc`, man pages, or housekeeping
   files (`CHANGELOG.md`, `LICENSE.md`) count toward 16? Default here is
   tracked `.md` only, but the constitution should state the rule, not just
   the number.
2. **Threshold scope.** Per repository or per documentation root? A monorepo
   with three small doc trees could be "small" three times or "large" once.
3. **Proposal discoverability.** A proposal written to an ignored scratch path
   in a third-party checkout is invisible to later sessions. Should each
   issued proposal get a beads ticket (forward anchor to the artifact) so
   proposals, acceptances, and rejections are findable? Recommended: yes.
4. **Trigger policy.** Who may dispatch the background proposal agent? This
   artifact assumes human- or session-lead-initiated dispatch, never ambient
   auto-triggering on entering a small repo. Confirm.
5. **Rejection memory.** If a proposal is declined, what prevents a future
   agent from re-proposing the same migration? A bd ticket note or a line in
   the project's local doc-deviation note would stick; neither exists yet.
6. **Where local equivalents are documented.** "An explicitly documented local
   equivalent" needs a home: the project's own `AGENTS.md`? Its root README?
   A workspace registry? Unstated in the constitution.
7. **Upstream translation contract.** How much of our vocabulary (semantic
   anchors, OKF frontmatter, evidence classes) may cross into an upstream PR?
   A worked example of situation D (overlay draft → upstreamed text) would
   settle the minimal translation.
8. **Overlay naming on foreign checkouts.** Keep our `design/` names for
   predictable filtering (this artifact's assumption), or mirror upstream's
   doc directory names for less oddity? Ours seems right; confirm.
9. **Symlink-unaware surfaces.** `SKILL.md → README.md` in overlay modules
   inherits the packaging concern already deferred by the synthesis
   ([`syn0.gpt56t.md`](/design/doc-constitution/syn0.gpt56t.md), tool
   questions); adoption into carried checkouts makes it more likely to bite.
10. **Execution staging.** Should an accepted migration land as one commit or
   staged commits, and who runs the doc-pass / link-integrity check
   afterward? The procedure says "small, reviewed commits" but not the
   verification step's owner.

---

<a id="adoption-clause-crossrefs"></a>
# Cross-References

- [`/constitution/README.md`](/constitution/README.md)
  [**Prospective Adoption And Existing Files**](/constitution/README.md#doc-constitution-lifecycle-existing)
  — the clause here operationalizes its grandfathering posture and adds the
  <16 proposal path and upstream boundaries it does not yet cover.
- [`syn0.gpt56t.md`](/design/doc-constitution/syn0.gpt56t.md) — records
  "prospective workspace default" as a choice made and defers tool questions
  (symlink packaging, skill discovery) that this clause's ambiguities inherit.
- [`/AGENTS.md`](/AGENTS.md) — supplies the `~/archive` third-party checkout
  convention that drives decision-table rows C–E, and the no-pruning and
  jj-commit discipline the safeguards rely on.
- [`draft0.glm53m.md`](/design/doc-constitution/draft0.glm53m.md) — its
  context-budget warning is the basis for the default-zero `GLOBAL.md`
  safeguard.
- Beads epic `rekon-doc-constitution` — acceptance of this clause would
  belong to that epic's scope; §5.3 proposes one ticket per issued migration
  proposal.
