---
id: praxis-flights
order: 402
type: ContextFragment
title: Flights — spawn, join, and carry evidence forward
description: Compact coordination cue for flights, naming the random-combo reusable model and its zero-install generator command.
resource: /praxis/flights/GLOBAL.md
source: /praxis/flights/README.md
status: draft
sources:
  - resource: "opencode://ses_e60122a28ffem0Q4yFFw0qqZEM"
    title: Commission ambient flight availability including random-combo and its slots knob
  - resource: /praxis/flights/README.md
    title: Flight practice home
---

# Flights — spawn, join, and carry evidence forward

A **flight** composes agent work as explicit promises: bounded spawns, joins
that name what a dependent step needs, and evidence carried forward. It is a
coordination model, not a scheduler and not delegation authority; humans own
authorization and decisions. Practice: [flights](/praxis/flights/README.md).

The ready reusable model is **random-combo**: draw N weighted options per game
(three by default) from a human-supplied pool, with persistent winner decay,
then join independent game responses before comparison. Its generator runs
without install from the Rekon checkout:

```sh
node tool/random-combo.ts --rows 4 --slots 3 --seed example FILE   # --help for more
```

Flight model: [random-combo](/praxis/flights/random-combo.md); generator
contract: [random-combo tool](/tool/random-combo/README.md). Depth pair for
flight design: [parallax](/praxis/parallax/README.md) (why independent views
differ) and [evidence gates](/assess/gates/README.md) (arrival vs readiness).
