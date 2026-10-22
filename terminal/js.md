---
type: Reference
title: Terminal — JavaScript
description: Verified facts for @bomb.sh/tty (declarative WASM terminal renderer and event parser) and the effect-cli direction for structured CLIs.
resource: /terminal/js.md
tags: [terminal, tui, js, tty, effect-cli]
status: draft
generated: { by: "model:glm-5.3-max", at: 2026-10-22 }
sources:
  - { resource: "opencode://ses_e9bc39334ffetsgyIAj4L0kp0A", title: "Record verified @bomb.sh/tty facts and the effect-cli direction" }
  - { resource: "file:///home/rektide/archive/bombshell-dev/tty/README.md", title: "@bomb.sh/tty upstream README" }
  - { resource: "file:///home/rektide/archive/bombshell-dev/tty/AGENTS.md", title: "@bomb.sh/tty spec-driven workflow and repository conventions" }
---

<a id="terminal-js"></a>
# Terminal — JavaScript

## `@bomb.sh/tty` — TUI foundation

Archived at `~/archive/bombshell-dev/tty` (canonical:
[bombshell-dev/tty](https://github.com/bombshell-dev/tty)). Verified facts:

- **Declarative terminal UI** using [Clay](https://github.com/nicbarker/clay):
  flexbox-like layout, pointer detection, and scroll containers, rendered to
  the terminal as box-drawing characters and ANSI escape sequences.
- **Zero I/O**: never reads stdin or writes stdout. Feed it bytes, get bytes
  back — pure computation, embeddable in any runtime or event loop.
- **WASM engine**: runs anywhere JavaScript runs, no native dependencies, no
  consumer build step. The repo itself is Deno-based (`deno.json`).
- Useful as a foundation for your own framework, not only directly.

When working in that repository, respect its spec-driven workflow: `specs/`
are the source of truth; propose the spec change, get approval, then
implement — never combine the two. The renderer and input parser are
architecturally independent (`renderer-spec.md`, `input-spec.md`); do not
introduce dependencies between them, and keep each spec's tests in its own
file. No agent marketing material in commits or PRs.

## effect-cli — structured CLI direction

Effect-based CLI framework, recently adopted for structured command-line
work. Not yet archived locally; this section grows as usage accumulates.
For plain argument parsing the JS guide still prefers gunshi — see
[CLI patterns](/code/js/README.md#cli-patterns); choose effect-cli when the
command structure wants Effect services and composition.
