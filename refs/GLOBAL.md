---
id: refs
order: 500
source: /refs/README.md
status: draft
---

# References

Read the relevant skill; resolve its support paths from its own directory.

## Skills

| Skill | Cue | Entry | Upstream | Registered |
| --- | --- | --- | --- | --- |
| motel-debug | Runtime-evidence debugging with a local OpenTelemetry ingest/query server | [SKILL.md](file:///home/rektide/archive/kitlangton/motel/skills/motel-debug/SKILL.md) | [kitlangton/motel](https://github.com/kitlangton/motel/blob/main/skills/motel-debug/SKILL.md) | — |
| polars | Polars queries and Python tabular-data work | [SKILL.md](file:///home/rektide/archive/polars-inc/skills/polars/SKILL.md) | [polars-inc/skills](https://github.com/polars-inc/skills/blob/main/polars/SKILL.md) | — |
| effect-best-practices | Effect-TS services, errors, layers, and atoms | [SKILL.md](file:///home/rektide/archive/makisuo/skills/effect-best-practices/SKILL.md) | [makisuo/skills](https://github.com/makisuo/skills/blob/main/effect-best-practices/SKILL.md) | `~/.agents/skills` |
| use-railway | Railway projects, services, deployments, infrastructure — even unspoken | [SKILL.md](file:///home/rektide/archive/railwayapp/railway-skills/plugins/railway/skills/use-railway/SKILL.md) | [railwayapp/railway-skills](https://github.com/railwayapp/railway-skills/blob/main/plugins/railway/skills/use-railway/SKILL.md) | `~/.agents/skills` |
| cplace | stream.place VOD records and `cplace` commands | [SKILL.md](file:///home/rektide/src/cli-place/skill/SKILL.md) | [rektide/cli-place](https://github.com/rektide/cli-place/blob/main/skill/SKILL.md) | `~/.agents/skills` |

Install rule: registration symlinks the Entry's parent directory into one
discovery root under the Skill name —
`ln -s "$(dirname <entry>)" ~/.agents/skills/<skill>` (first-party `cplace skill`
does this for itself). `Registered` records the current root; `—` means curated
but not registered — reach those rows by link, or apply the rule to install.

- Selecting or composing skills: [our process](/refs/skills.md).
- Writing sourced knowledge: [OKF sources](/refs/sources.md), e.g. `sources: [{resource: "urn:opencode:session:<id>", title: "Human direction"}]`.
