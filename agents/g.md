---
description: "GLM 5.3 max subagent through the Z.AI Coding Plan. Use for most tasks when GLM is requested. Runs at #max; when the user asks for less, pass model zai-coding-plan/glm-5.3#low or #high. Replaces the former gh (#high) and gl (#low) files — dispatch those levels by model override, not by agent name. DO NOT TRY HIGHSPEED — if you override this agent's model, keep regular zai-coding-plan/glm-5.3 with its #max variant; never select glm-5.3-highspeed or any other -highspeed model. Regular glm-5.3 is omitted from the models tool default query (newest families only); run the query with all:true to see it."
note: "GLM 5.3 max — the default heavy lifter for most tasks; gh/gl folded in, other levels via model override"
mode: subagent
# keep model and variant as separate keys; the deployed opencode loader drops embedded #variants (design/agents/model-routing0.glm53m.md)
model: zai-coding-plan/glm-5.3
variant: max
---
