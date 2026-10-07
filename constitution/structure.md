---
type: Structure
title: Rekon's working structure
description: Where Rekon explains itself, maintains practices, selects references, and builds tools; the local placement contract.
resource: /constitution/structure.md
tags: [rekon, structure, naming, tools, documentation]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-10-06 }
sources:
  - { resource: "urn:opencode:session:ses_eec5afbf4ffeIhL6wxmv8NF3gn", title: Human direction for shortcodes g/p/t and one domain-grouped tool toolkit }
  - { resource: /README.md, title: Rekon purpose and practice map }
  - { resource: /constitution/naming/exploration/proposal4.glm53m.md, title: Recorded location-identity and abbreviation ruling }
  - { resource: /constitution/reassessment0.gpt6a.md, title: Broader constitutional compact and its boundaries }
  - { resource: /design/rekon/doc-cull0.glm53m.md, title: Separating dl-owned history from Rekon }
---

<a id="structure"></a>
# Rekon's working structure

**Rekon maintains the understanding, practices, references, and tools through
which people and agents do cumulative technical work.** Its structure should
make those responsibilities visible. A topic's explanation lives with the
concern it explains; executable tools bring their explanation and code together.

This is the Rekon-specific placement contract. The October 6 direction selects
**`tool/` as one shared toolkit** and **`t` as its shortcode**. `p` names praxis
and `g` names global guidance; shortcodes do not dictate directory spelling.
The broader constitutional compact remains under development. Other repositories retain
their own documentation conventions and the constitution's general adoption
guidance.

<a id="homes"></a>
## Homes and responsibilities

| Home | Owns | Start here when… |
|---|---|---|
| [Root README](/README.md) | Rekon's identity, offerings, and entry points | You need to understand what this project is. |
| [Constitution](/constitution/README.md) (`con`) | Shared commitments, knowledge ownership, naming, and this structural contract | You need to know what people can rely on or where a concern belongs. |
| [Praxis](/praxis/README.md) (`p`) | How we prepare, act, exercise judgment, and learn from experience | You need a working posture or a way to choose the next action. |
| [Tool toolkit](/tool/README.md) (`t`) | Executable mechanisms, their contracts, maintenance, and shared domain-grouped source | You need to run, understand, or change a tool. |
| [References](/refs/README.md) (`refs`) | Selected external resources, why we use them, source identity, and access | You need to find a reference or understand our [skill selection and composition](/refs/skills.md). |
| [Evidence](/evidence/README.md) | How sources bear on judgments and what happened when practices were used | You need the grounds for a claim or lessons from an actual attempt. |
| Existing practice and domain homes | Their own maintained explanations and implementation-specific knowledge | The concern already has an owner, such as [assessment](/assess/README.md), [code](/code/README.md), or [jj](/vcs/jj/README.md). |

The constitution explains the shared compact and structure. Praxis gives its
commitments practical expression. A proposed **`p0`**, “what we are / how we
work,” could name the opening account of praxis; it does not make praxis the
owner of the repository's directory map. That alias remains a discussion point.

<a id="placement"></a>
## Where to write

1. **Find the concern's owner first.** Extend its README or add a linked support
   document there. A tool's maintained usage and contract belong in
   `tool/` under its substantive domain; a practice's explanation belongs with
   that practice.
2. **Give new concerns a domain home.** The directory name should say what it
   owns. A `README.md` explains its purpose, current state, and useful neighbors.
3. **Keep inquiry recognizable.** Existing `design/<topic>/` waves retain their
   evidence and exact identities. New inquiry can continue there; supported
   conclusions are synthesized into the concern's maintained home. This does
   not reopen the entire design-directory policy.
4. **Keep sources with their native owners.** Project-specific mechanisms and
   histories stay in their project. Rekon owns the reusable practice and its
   explained references. Archive checkouts supply upstream bodies in place.
5. **State standing explicitly.** A directory or README does not establish
   verification, acceptance, or deployment. Name those facts separately.

**New Rekon explanations no longer default to `doc/<topic>/`.** The older
`doc/` intake rule is superseded here by domain ownership. Its remaining files
are being rehomed deliberately, with citation repair. This local change does
not replace the general `doc/` convention used in other software repositories.

<a id="tools"></a>
## Tool shape

The toolkit has top-level entrypoints and a shared README. Source subdivisions
name actual responsibilities; each can maintain its own deeper explanation:

```text
tool/
  README.md
  agents.ts               executable/import-safe assembler entrypoint
  <other-tool>.ts          other entrypoints as they are implemented
  cmd/                    command composition and terminal behavior
  assembly/               fragment assembly, manifest planning, write/check
  md/                     Markdown-specific logic
  hyalo/                  Hyalo integration when adopted
```

`cmd/`, `md/`, or a library-specific `hyalo/` seam are responsibilities justified
by the tools' work, not mandatory empty scaffolding. Use meaningful domain
directories rather than a generic `src/` or `lib/` wrapper. Keep tests beside
the behavior they exercise. Development invokes the TypeScript entrypoint
directly; distribution builds are a separate concern.

The first new implementation is the Effect CLI version of the GLOBAL-fragment
assembler at `tool/agents.ts`. The Gunshi CLI in [`rekon.ts`](/rekon.ts)
and [`src/`](/src/) remains the legacy implementation while capabilities gain
their new homes. Each command's guide must say which implementation it
actually executes. Existing legacy tool guides can move first and identify
their current source explicitly; their code migration is a separate step.

<a id="names"></a>
## Names we can use

The [naming home](/constitution/naming/README.md#short-codes) keeps the concise
reference table: `g` = global, `p` = praxis, `t` = tool; `con` abbreviates
`constitution`. A scoped phrase such as **`t-agents`** should be paired with
its ordinary Markdown link on introduction. The path and section make its
referent recoverable; an abbreviation does not require an automated resolver.
The initial draft mistakenly turned `t` into a directory; the human correction
explicitly separates shorthand from physical layout.

Moves change names. Repair current citations and use revision-bound references
when a reader needs the earlier artifact, preserving the distinctions in the
[human naming ruling](/constitution/naming/exploration/proposal4.glm53m.md#the-override-stated-as-ruling).

<a id="global"></a>
### Global guidance — `g`

Global guidance is the shared context carried today by [AGENTS.md](/AGENTS.md)
and developed in domain-owned `GLOBAL.md` contributions. Its shortcode is `g`.
The owning domain maintains each contribution; an assembled exposure connects
them. This records the human-provided abbreviation, not a new `g/` directory
or an automatic assignment of numbered global clauses.

<a id="evolution"></a>
## Making the skeleton real

The immediate work is concrete: separate departed dl material, establish tool
homes, build the new assembler alongside its predecessor, recover the Hyalo
ToC lineage, and compare the OpenCode recovery guide with `~/ado/`. Those
checks determine what is maintained here and what can be referenced elsewhere.

The longer constitutional question is which commitments define Rekon beyond
documentation mechanics. The [reassessment](/constitution/reassessment0.gpt6a.md#boundary)
already sets that destination: purpose, obligations, relationships, and a short
reading map. This structure gives that account useful homes without claiming
the wider compact is finished.
