---
type: ResearchReport
title: Shortcode lineage — g, p, t and numbered labels
description: Historical recovery of actual g/p/t shortcode usage and requests, p0/g0/t0 numbered labels, their relation to con/exp/res and section names, with evidence separated from interpretation.
resource: /design/rekon/shortcodes0.glm53m.md
tags: [rekon, naming, shortcodes, g, p, t, con, exp, res, cotail]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-10-06 }
sources:
  - { resource: "urn:opencode:session:ses_eec5afbf4ffeIhL6wxmv8NF3gn", title: Ruling session (2026-10-06/07), author: "session:happy-moon" }
  - { resource: "urn:opencode:session:ses_f9cf5c4b", title: Naming wave session (2026-09-02→24), author: "session:playful-garden" }
  - { resource: "urn:opencode:session:ses_eec6a0bd", title: Global AGENTS.md skill-manager session (2026-10-06), author: "session:brave-mountain" }
  - { resource: /constitution/naming/exploration/proposal4.glm53m.md, title: Human ruling incl. glossary aliases, author: "model:gpt-6-astra" }
  - { resource: /constitution/naming/README.md, title: Short codes in use table, author: "session:happy-moon" }
  - { resource: /constitution/structure.md#global, title: g as global guidance, author: "session:happy-moon" }
---

# Shortcode Lineage — `g`, `p`, `t` And Numbered Labels

<a id="shortcodes0-scope"></a>
## Scope and method

Question: recover **actual prior usage/requests** for the shortcodes global
(g), praxis (p), tool (t), numbered labels such as p0/g0/t0, and their
relation to `con`/`exp`/`res` and section names — earlier human direction and
practical examples, not another naming theory.

Searches run, all against the live session DB (read-only):

- `cotail search -q 'short codes' -q 'shortcodes' --since 30d` — bounded.
- Same queries `--since off` — **explicit exhaustive widening**: unlike the
  Hyalo task (whose 30d bound stands), this question is historical, so the
  widening is deliberate and is *not* attributable to the earlier 30d query.
- `cotail search -q g0 -q p0 -q t0 --since off --fixed-strings
  --directory ~/src/rekon` — every `g0`/`t0` hit and nearly every `p0` hit is
  **substring noise from session IDs** (e.g. `…T7G0Bhp7…`, `…EKOriP0`); the
  only human `p0` is quoted below.
- Repo/git history: `jj log -r files(...)`, `git log -S` on
  `constitution/naming/README.md` and `constitution/structure.md`; beads
  `.beads/issues.jsonl` ID prefixes; `rg` over `~/src/doc` (no pre-rekon
  shortcode material found there).

Timestamps below are UTC (DB renders UTC); today's evening items are
2026-10-06 ≈20:30–21:00 EDT = 2026-10-07 00:30–01:00 UTC.

<a id="shortcodes0-ruling"></a>
## The ruling moment (evidence, verbatim)

Session `ses_eec5afbf4ffeIhL6wxmv8NF3gn` ("Clarifying doc/'s purpose…",
rekon):

- **seq 396, 2026-10-06 ~23:55 UTC** — the originating direction:
  > "…praxis also being a structure system feels weird but maybe that's just
  > like **p0, what we are**, of praxis. **what happened to having short
  > codes, prefixes for things?** naming? that naming thing has really gone
  > amiss in my practice. … we need some part of rekon that is clearly tool
  > oriented i feel like. and a prefix. **`t` tool is fine.** … we have the
  > assembler thing for **global.md**? and it integrates global.md? what is
  > that thing, that's what i'm thinking of for tool."
- **seq 580–584, 2026-10-07 00:35:12 UTC** — the correction:
  > "whoa **t is a prefix not a dir**" · "due crap i really want to **unfuck
  > this from history** this" · "**this is shortcodes yes**" · "**g is the
  > global shortcode, p praxis.**"

The "unfuck" refers to the t/-as-directory conflation already propagated that
evening into the provisional structure map, `t/README.md`, AGENTS.md, one
subagent prompt, and
[`hyalo-toc0.glm53m.md`](/design/rekon/hyalo-toc0.glm53m.md) (since
corrected, commit `68cdfb9a`).

<a id="shortcodes0-earlier"></a>
## Earlier lineage (the practice the user was remembering)

**Naming wave, `ses_f9cf5c4b` ("Glossary terms for constitution doc
hierarchy", 2026-09-02 → 09-24):**

- **seq 189, 2026-09-02 20:15:46 UTC** — origin of `con`/`exp` and the
  glossary-alias idea:
  > "we should have projects define **aliases, short terms, in their
  > glossary** (usually within the readme). **con-naming-exp-agent-naming**
  > is not the worst, is an ok example i would endorse."
  Same message endorses the full-name grammar
  (`constitution-naming-exploration-proposal0-stage`) and the
  anchor-implies-the-rest-of-the-name idea.
- **seq 675, 2026-09-03 18:34:26 UTC** — origin of short+long references:
  > "we need to have **multiple references** for some things, **a short
  > reference like C2 and a long reference with an elaborated name**. this
  > implies multiple anchors. we should also make explicit the **anchor =
  > section name connection**…"
  Pattern source cited there:
  `~/src/opencode-watchman/.design/watchman/carrier0.gpt56s.md` (commit
  references). Recorded downstream in
  [`ask-audit0.glm53m.md`](/constitution/naming/exploration/ask-audit0.glm53m.md)
  row 80: "Short plus long references, C2-style (r4) | Survives".

**The human ruling** ([`proposal4.glm53m.md`](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling),
item 4, Sep wave) — the only pre-existing *governed* shortcode statement:

> "Cross-modal aliases are precedent, not grandfathered anomaly.
> `rekon-con-naming` — an alias living inside a beads ID — is the endorsed
> practice… Abbreviations may cross modalities freely; **glossary aliases
> (`con`, `exp`, `res`) live as resolve-register conveniences** with no
> separate permission required."

**Practical usage inventory (what has actually been used):**

- Beads: `rekon-con-naming{,-acceptance,-tooling}` — 3 of 57 issues; the only
  cross-modal shortcode prefix in tickets. No `g`/`p`/`t` bead prefixes exist.
- `con-naming-exp-…` — the endorsed scoped-reference example (seq 189).
- `C2` — a **per-document citation register** (second C-numbered reference in
  one doc), not a global numbering scheme.
- The maintained [`Short codes in use`](/constitution/naming/README.md#short-codes)
  table is **new — landed 2026-10-06 20:34–20:43 EDT** (commits `62b4bab`,
  `cac1a0c`, `0fac313`, `16d1daf`), not September: the 2026-09-23 README
  (`74ee9e1`) had no such section. It codifies the Sep-wave practice
  (`con`/`exp`/`res`) plus today's `g`/`p`/`t`.

<a id="shortcodes0-g"></a>
## What `g` means — evidence vs interpretation

- **Evidence (all of it):** seq 584 — "g is the global shortcode, p praxis."
  No older definition exists anywhere I searched. The concurrent referents in
  the same evening: (a) seq 396's "assembler thing for **global.md**";
  (b) the sibling session `ses_eec6a0bd` (2026-10-06 23:39 UTC) on "our
  **global agents.md**" as skill index/citation entry point; (c) the Sep-wave
  "project namespace (within a **global namespace**)" concept
  (seq 128, 2026-09-02 19:59 UTC; survives in `ask-audit0` r1 as "locality +
  federation").
- **Interpretation (parent's, now landed, consistent with the above):**
  [`constitution/structure.md#global`](/constitution/structure.md#global) —
  "**Global guidance** is the shared context carried today by AGENTS.md and
  developed in domain-owned `GLOBAL.md` contributions. Its shortcode is `g`."
- So `g` is **neither a single fixed document nor a recorded "exposure
  class"**: the human definition is domain-level ("global"); the concrete
  carriers are the assembled `AGENTS.md` and `GLOBAL.md` contributions. There
  is no evidence for reading `g` as one specific file's name, and no recorded
  notion of g-as-exposure-class — that vocabulary appears nowhere in the
  history recovered here.

<a id="shortcodes0-numbered"></a>
## Numbered labels: p0, g0, t0

- **`p0` — the only attested usage.** Exactly one human use, seq 396:
  "maybe that's just like p0, what we are, of praxis" — an *opening account
  slot* in praxis. Now landed as "suggested opening 'what we are / how we
  work' account, still being shaped"
  ([naming README](/constitution/naming/README.md#short-codes),
  [structure](/constitution/structure.md) "A proposed `p0`"). Both landings
  are 2026-10-06; before that evening `p0` appears nowhere in sessions or
  docs (git `-S` on structure.md returns only today's commits).
- **`g0` / `t0` — zero attested usages.** All search hits were session-ID
  substring noise. The naming README explicitly disclaims manufacturing a
  `g0`/`p0`/`t0` numbering scheme.
- **`C2` is a different thing** — per-document short citation registers
  (seq 675), not domain numbering.

<a id="shortcodes0-relate"></a>
## Relation to con/exp/res and section names

- **Kinds (interpretation, marked as such):** `con`/`exp`/`res` arose as
  *reference abbreviations inside scoped names* (`con-naming-exp-…`),
  resolve-register conveniences per the ruling; `g`/`p`/`t` arose as *domain
  shortcodes for the three top-level concerns* (global guidance, praxis,
  tool). Both kinds are reference vocabulary — the README now states
  "Shortcodes are reference vocabulary, not directory names," which is the
  correction's distilled rule.
- **Section names:** the ruling's item 1 ("location-implied naming is
  identity… the tree is the namespace") plus seq 675's "anchor = section name
  connection" and seq 189's anchor-implies-name idea are the standing law:
  durable names are path + anchor; shortcodes (and C2-style registers) are
  conversational/citation tokens layered on top, introduced with an ordinary
  link to the home they abbreviate.

<a id="shortcodes0-uncertain"></a>
## Boundaries and uncertainties

- The exhaustive session search covers the rekon directory plus unrestricted
  `short codes`/`shortcodes` queries; I did not sweep every other project's
  history for prefix practice, and `~/src/doc` grep found nothing —
  "gone amiss in my practice" may refer to habits with no surviving machine
  record. Unverifiable from here.
- cotail truncates displayed IDs (8 hex chars); full IDs above were resolved
  by prefix against the DB and are exact.
- The Sep-wave quotes are from the sessions that produced the naming
  exploration; if subagent documents of that wave recorded further examples,
  the parent's already-read corpus (proposal2 L278-314, ask-audit0,
  draft3.sol56x) covers them — nothing further surfaced beyond the C2 row and
  ruling item 4 cited above.
- Recommendation (evidence-supported, offered only for the table): keep the
  README's current shape — each shortcode introduced with a link, `p0` marked
  as proposed, no manufactured numbering — it matches the entire recovered
  record.

<a id="shortcodes0-xref"></a>
## Cross-references

- [`/constitution/naming/README.md#short-codes`](/constitution/naming/README.md#short-codes)
  — the maintained table this history supports (landed 2026-10-06).
- [`/constitution/naming/exploration/proposal4.glm53m.md`](/constitution/naming/exploration/proposal4.glm53m.md)
  — ruling item 4, the governed ancestor of the con/exp/res rows.
- [`/constitution/structure.md#global`](/constitution/structure.md#global) —
  the landed interpretation of `g`.
- [`/design/rekon/hyalo-toc0.glm53m.md`](/design/rekon/hyalo-toc0.glm53m.md)
  — the report whose correction note links here; same evening, same ruling.
- Sessions (full IDs): `ses_eec5afbf4ffeIhL6wxmv8NF3gn` (ruling),
  `ses_f9cf5c4b…` (naming wave), `ses_eec6a0bd…` (global AGENTS.md),
  `ses_eec3b15f4ffegxdPubbIgSJ3VH` (this investigation's sibling Hyalo
  recovery, mine).
