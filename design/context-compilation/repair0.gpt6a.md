---
type: Assessment
title: Restore operational coverage in assembled context
description: Diagnose the loss of legacy operating advice and connect composition repair to a runnable coverage audit.
resource: /design/context-compilation/repair0.gpt6a.md
status: draft
generated: { by: "model:openai/gpt-6-astra", at: 2026-10-11 }
sources:
  - { resource: "urn:opencode:session:ses_f908bad7fffeW0pr4FLFa4VFyn", title: Human request to assess and repair incomplete AGENTS and advance the tools }
  - { resource: /AGENTS.old.md, title: Preserved operating manual }
  - { resource: /agents.manifest.json, title: Production selection at diagnosis }
  - { resource: /constitution/agents-assembly.md#agents-assembly-status, title: Explicit October 7 cutover with extraction incomplete }
  - { resource: /design/toolchain/draft0.glm53m.md, title: Accepted Rust-first tool direction and prior recognition of the coverage gap }
---

# Restore operational coverage

## Diagnosis

The October 7 cutover selected five fragments while preserving unextracted
instructions only in `AGENTS.old.md`. The production assembler successfully
reproduces that incomplete selection: `node tool/agents.ts --check` reports
fresh on October 11. Preservation in version control or a legacy file did not
preserve activation. Beads' stable `--id`, child namespace, dependency direction,
and command recipes disappeared from ambient context together.

This is a composition failure, not a byte-rendering failure. The selected
constitution and code fragments also still call themselves unselected candidates
and refer to a live manuscript that is no longer the producer. The prior
[modularization design](/design/rekon/agents-modularization0.sol56x.md)
already distinguished extraction from deleting ambient obligations; the
[toolchain roadmap](/design/toolchain/draft0.glm53m.md#assembly-roadmap)
identified the missing lanes, but neither was an executable coverage check.

## Repair direction

- Keep the current useful glossary and domain fragments; restore the missing
  operating advice under domain owners. Common actions remain inline in GLOBAL,
  particularly ticket naming and the basic beads loop. Detailed recovery belongs
  behind a useful pointer.
- Give beads a README/SKILL/GLOBAL home under `praxis/beads/`, composed alongside
  ticket pointers. Optional ticketing does not mean arbitrary IDs once tickets
  are used. Verify the installed CLI instead of copying stale database recovery.
- Restore collaboration, workspace, writing/waves, jj, and language conventions.
  The old file remains evidence; it is not concatenated as a competing authority.
- Add a Rust `audit-context` subcommand alongside the pending combiner. A coverage
  declaration must account for each real top-level legacy section, selected
  fragments, and explicit required instructional excerpts. Report all omissions.
  A fresh byte assembly and passing coverage audit are separate checks.
- Keep the Effect assembler as production producer. Completing the Rust renderer
  is a separate job; the existing uncommitted `markdown.rs` is not part of this
  repair. No new TypeScript composition layer is needed.

## Proof and limits

Test the observed failure: an output can be regenerated successfully after beads
is dropped from the manifest, yet the coverage audit must fail. Also fail when
its naming instruction is removed while its file remains selected, or when an
unaccounted legacy section appears. Require explicit review when the baseline
changes. These are declared coverage checks, not machine proof of semantic
equivalence or editorial quality. Keep the reviewed omissions and refinements
visible beside the declaration.

The user asked for iteration and advancement, so the deliverable is restored
runtime context plus a working audit, not just another proposal. Use existing
`rekon-agents-maintenance` work history rather than a ticket per fragment.
