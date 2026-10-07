---
id: refs
order: 500
source: /refs/README.md
status: draft
---

# References

Read the relevant skill; resolve its support paths from its own directory.

## Skills

| Skill | Cue | Source |
| --- | --- | --- |
| motel-debug | Runtime-evidence debugging with a local OpenTelemetry ingest/query server | [kitlangton/motel](https://github.com/kitlangton/motel/blob/main/skills/motel-debug/SKILL.md) |
| polars | Polars queries and Python tabular-data work | [polars-inc/skills](https://github.com/polars-inc/skills/blob/main/polars/SKILL.md) |
| effect-best-practices | Effect-TS services, errors, layers, and atoms | [makisuo/skills](https://github.com/makisuo/skills/blob/main/effect-best-practices/SKILL.md) |
| use-railway | Railway projects, services, deployments, infrastructure — even unspoken | [railwayapp/railway-skills](https://github.com/railwayapp/railway-skills/blob/main/plugins/railway/skills/use-railway/SKILL.md) |
| cplace | stream.place VOD records and `cplace` commands | [rektide/cli-place](https://github.com/rektide/cli-place/blob/main/skill/SKILL.md) |

Install rule: resolve a Source link to its local checkout —
`~/archive/<org>/<repo>`, or `~/src/<repo>` for first-party `rektide/*` — then
symlink the `SKILL.md` parent directory into a discovery root under the skill
name. This table is the availability record: registrations are disposable
derivations of it, not tracked state.

- Selecting or composing skills: [our process](/refs/skills.md).
- Writing sourced knowledge: [OKF sources](/refs/sources.md), e.g. `sources: [{resource: "urn:opencode:session:<id>", title: "Human direction"}]`.
