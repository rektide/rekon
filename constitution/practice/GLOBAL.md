---
id: documentation-practice
order: 120
source: /constitution/README.md
status: draft
sources:
  - resource: "opencode://ses_e612fa800ffeeNJGAuTzJeLwFY"
    title: Expose inspectable decision rationale for OpenAI agents in ambient writing guidance
  - resource: "opencode://ses_e61a523a6ffef0z5ECCK5z73dC"
    title: Human requests a practice capturing considered alternatives and the agent's lean
  - resource: "opencode://ses_ed2db3069ffeQV7v6JC7p69Vte"
    title: Align durable-writing provenance with keyed positional OpenCode URIs and session authority
  - resource: "opencode://ses_ed2eda746ffefem0MPL9M5MYX7"
    title: Human direction to adopt OpenCode URIs with keyed positional IDs as the canonical grammar
---

# Writing, independent waves, and document connections

## Durable writing

- **ChatGPT/OpenAI agents:** when discussion or a durable assessment calls for
  judgment, make the rationale inspectable: meaningful alternatives, material
  reasons and tradeoffs, assumptions, your tentative lean and confidence or
  uncertainty, and evidence or preferences that would change it. Distinguish a
  recommendation from an accepted decision or commissioned work. Give a concise
  decision rationale, not private chain-of-thought, hidden scratch reasoning, or
  routine tool narration; scale detail to the choice. No compulsory research
  wave or separate document. [Candidates practice](/praxis/candidates/README.md#make-the-lean-inspectable).
- Use structured Markdown with descriptive links to canonical files and sections.
  Prefer bundle-root-relative links inside Rekon and canonical repository URLs
  for external sources; identify the repository when context does not already.
- Give knowledge documents honest OKF-style frontmatter: type, title,
  description, resource, status, author/model and date, sources actually read,
  and verification/review information only when justified. See
  [source practice](/refs/sources.md) and [the constitution](/constitution/README.md).
- **Include your current native session ID in OKF `sources:` whenever you create
  or substantively revise a knowledge document.** Use
  `resource: "opencode://<actual-session-id>"` and a title explaining
  the session's contribution (direction, reasoning, research, or revision).
  This is vital provenance: it lets the next reader recover why the document
  exists and how it took shape. Use the ID supplied by the harness, not a
  petname, placeholder, or another session's ID copied from an example.
  Preserve earlier session sources; add the revising session's source.
  Maintain your session's own source entry as the work develops, gradually
  updating its title to match its actual contribution; keep the session ID stable.
  Session-only citations may omit `?source=<registered-source-id>`; portable
  cross-store exports include it. The native session ID is the URI authority;
  there are no named route components. Preserve legacy URN and named-route
  sources. For message/part targets and selected-source resolution, follow the
  [OpenCode URI convention](/refs/sources.md#our-opencode-session-convention).
- `index.md` is a grouped progressive-disclosure listing, not another concept
  document. `log.md` is dated update history, newest first. An addendum can retain
  an earlier argument while recording new understanding; mutable READMEs may
  integrate accepted conclusions directly.

## Waves

- Start with the situation, present ask, directions of inquiry, and what changed
  since the previous work. Waves are independent research/design outputs, commonly
  under `design/<topic>/`; they are not compulsory for every task.
- Every wave filename ends in the current model's concise suffix:
  `draft0.gpt6a.md`, `draft0.glm53m.md`. Use a numerical round starting at 0.
  Never use the harness name as the model or copy another model's suffix.
- For one main document, use `<topic>.<model>.md`; revisions can use
  `<topic>2.<model>.md`. Prefer new files for materially new arguments; preserve
  independent drafts. Maintain a README/index as the corpus becomes useful.
- Do not read other authors' same-wave files unless directed. Previous waves are
  available context. Synthesis follows the evidence: integrate common themes
  and tensions, then compare sources' strengths, weaknesses, and distinctive ideas.
  Names such as `draft0-syn0.<model>.md` record the wave and synthesis round.
- When a tip is accepted, a suffix-less symlink can identify it while exact
  model-suffixed evidence stays available. Acceptance is not deletion or a bulk
  move; promotion synthesizes into the maintained domain home.

## Doc-pass

After substantial writing, search indexes and the surrounding corpus by concept,
not just exact keywords. Read candidate sources enough to distinguish meaningful
relationships. Add explained Cross-references (evidence, tension, precedent,
extension), useful reciprocal links, and index updates. Verify the links and
surrounding text. A separate search agent is optional when authorized; integrate
only findings checked against sources.

These restore [legacy writing practice](/AGENTS.old.md#documentation--reference-materials).
The old `summarizer` mention described unfinished work, not an available tool;
consult [the toolkit map](/tool/README.md) for current capabilities.
