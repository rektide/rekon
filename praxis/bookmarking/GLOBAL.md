---
id: praxis-bookmarking
order: 408
type: ContextFragment
title: Bookmarking — mark session material worth returning to
description: Compact cue for when to bookmark decision-dense junctions, summaries, and review gates, naming the cotail engine commands.
resource: /praxis/bookmarking/GLOBAL.md
source: /praxis/bookmarking/README.md
status: draft
sources:
  - resource: "opencode://ses_e60122a28ffem0Q4yFFw0qqZEM"
    title: Commission ambient bookmarking availability naming the cotail engine
  - resource: /praxis/bookmarking/README.md
    title: Bookmarking practice, engine, and seed glossary
---

# Bookmarking — mark session material worth returning to

Mark decision-dense junctions (crossroads), explicit summaries, and parked
review/design gates while the reasoning is still inspectable. Write the note as
the evidentiary sentence — why this mark exists and what it bears on — and give
it free-form tags. A bookmark preserves an inspectable state; it decides
nothing. Practice and seed glossary:
[bookmarking](/praxis/bookmarking/README.md).

The engine is cotail's landed CLI:

```sh
cotail bookmark create -s ses_... --tag crossroads --note "why this junction matters"
cotail bookmark list --tag review --tag-match any   # top-down review view
cotail bookmark tags                                # live tag inventory
```

Tags are convention, not enforced vocabulary; the live inventory is
`cotail bookmark tags`. Until cotail's session-env ticket lands, pass `-s`
explicitly.
