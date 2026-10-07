---
type: Practice
title: Curating and composing skills
description: Our archive-first process for selecting skills, composing local guidance, and publishing concise links in global context.
resource: /refs/skills.md
tags: [references, skills, curation, global]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-10-06 }
extensions:
  ticket: rekon-skill-management
sources:
  - { resource: "urn:opencode:session:ses_eec6a0bd5ffe45v03zJ7px5vgb", title: Human direction for archive-backed skill curation and a KISS global index }
  - { resource: /design/skillify/README1.sol56x.md, title: Skillification and canonical bodies }
  - { resource: /constitution/agents-assembly.md, title: Implemented GLOBAL fragment assembly }
  - { resource: "https://github.com/kitlangton/motel/blob/31186212365f705ba539c4befbf34a9b9e4bbcfe/skills/motel-debug/SKILL.md", title: Motel debugging skill }
  - { resource: "https://github.com/polars-inc/skills/blob/70bec73cd09722411492dfd3613c73bb6df979b8/polars/SKILL.md", title: Polars skill }
---

# Curating and composing skills

**Rekon owns our skill selection, composition, and advertisement.** Upstream
skill bodies can stay in their archive checkouts. Our compact
[GLOBAL index](/refs/GLOBAL.md) is the entry point; follow its links to load the
relevant skill. Any harness-specific registration should derive from that same
selection.

We maintain this catalog by hand today. The skill-manager tool and automated
index generation are tracked by `rekon-skill-management`; the existing
[AGENTS assembler](/constitution/agents-assembly.md) supplies the fragment
assembly mechanism.

## Select and expose

1. **Locate the source.** Check `~/archive/<org>/<repo>` first. Fetch a missing
   repository with `dl --archive <org>/<repo>`. Find its actual skill entry file
   and read it together with the support material relevant to its use.
2. **Choose its role.** Say what task should load the skill. Use upstream
   metadata as input to our description, and account for instructions in the
   body: a narrower index description changes routing, not the loaded method.
3. **Record its provenance.** Keep the canonical repository/entry URL and local
   path in the supporting account, with a revision when recording source-specific
   findings. Follow [our source practice](/refs/sources.md) for OKF `sources:`.
   One repository may contain several independently selected skills.
4. **Publish a direct pointer.** Add a short task cue and link to
   `refs/GLOBAL.md`. Use a concrete `file:///home/rektide/archive/.../SKILL.md`
   link for this workspace. Keep the original directory intact, including its
   references and scripts. Our process uses these sources in place; upstream
   marketplace installers and copied skill folders are not required.
5. **Check the route.** Open the advertised file, follow its support paths, and
   check that the cue leads to the intended capability. Having its instructions
   available does not establish that the accompanying program is installed.

For another machine, resolve the archive root to that machine's actual path
before publishing its index. Canonical upstream URLs preserve source identity;
the local link makes the available checkout directly usable.

## Load and compose

Resolve a skill's relative references and script paths from the directory
containing its entry file, even when the target project is elsewhere. Project
operations still target that project; locating a bundled script does not change
what it should operate on.

Our own skills live with the domain that owns their practice. A composition
should explain which sources to read, in what situation or order, and what our
own guidance adds. Link to upstream depth and keep local decisions in the
Rekon-owned body. Where a composition changes an upstream instruction, state the
specific replacement and its scope; concatenation alone does not settle a
conflict. Advertise the composition's entry when it is the intended route.

## Keep GLOBAL simple

**A useful cue plus a direct link is the default.** Keep selection procedures,
source history, installation details, examples, and composition rationale here
or in the owning skill. GLOBAL should let an agent recognize a need and reach
the knowledge with minimal ambient text. Add a stand-alone instruction only
when it must act before the linked material is loaded.

## Initial sources

### Motel

Read [the archived Motel skill](file:///home/rektide/archive/kitlangton/motel/skills/motel-debug/SKILL.md)
for runtime-evidence debugging with a local OpenTelemetry ingest/query server.
Upstream: [kitlangton/motel — `motel-debug`](https://github.com/kitlangton/motel/blob/main/skills/motel-debug/SKILL.md).

Its support base is `skills/motel-debug/`: `references/effect.md` covers Effect
integration, and `scripts/clear-motel-debug.ts` removes tagged instrumentation.
The skill's setup guidance owns how to run Motel; the index just gets us there.

### Polars

Read [the archived Polars skill](file:///home/rektide/archive/polars-inc/skills/polars/SKILL.md)
for Polars queries and Python tabular-data work. Upstream:
[polars-inc/skills — `polars`](https://github.com/polars-inc/skills/blob/main/polars/SKILL.md).

Its support base is `polars/`, with query recipes and API guidance under
`references/`. The source recommends Polars for unspecified Python data work;
decide that preference explicitly when composing it with other guidance.

## Maintain the selection

After refreshing an archive checkout, review changed entry paths, routing
metadata, body instructions, and support resources before updating our entry or
composition. Check local links and relevant GLOBAL assembly validation. Keep
source-specific findings revision-bound, while the usable skill link follows
the chosen local checkout.

Update or retire catalog entries deliberately. Removing an entry is a curation
decision; it does not request deletion of the archive checkout.

For the broader capability model, see
[skillification](/design/skillify/README1.sol56x.md). For where references fit,
return to [refs](/refs/README.md).
