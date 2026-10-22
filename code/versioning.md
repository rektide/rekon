---
type: Practice
title: Pre-alpha versioning — API markers and reviewed revisions
description: A preferred default for evolving first-party software, using minor API-change markers, coordinated workspace versions, and deliberate consumer revision uptake.
resource: /code/versioning.md
status: draft
tags: [code, versioning, pre-alpha, api, dependencies, consumers]
generated: { by: "model:openai/gpt-6-astra#high", at: 2026-10-22 }
sources:
  - resource: "opencode://ses_e9a41f6dfffexEmfMeIjFhXDko"
    title: Human proposes Kubeless versioning as an optional preferred default; integrate independent research into code practice and ambient guidance
  - resource: "opencode://ses_e9a408b7fffeb871k0DxQ8m0sR"
    title: Sol medium researches Kubeless policy, revision history, bump mechanism, and receiving-lock failures
  - resource: "opencode://ses_e9a4069d1ffe2s0rUu0m5gAsnZ"
    title: Independent Sol medium placement research recommends code/versioning.md and the existing code GLOBAL
  - resource: file:///home/rektide/src/kubeless/AGENTS.md
    title: Kubeless pre-alpha status and versioning policy
  - resource: file:///home/rektide/src/kubeless/README.md
    title: Source-native distribution through paths or reviewed Git revisions
  - resource: file:///home/rektide/src/kubeless/scripts/bump-workspace-version.py
    title: Coordinated manifest pins and optional root and standalone lock refresh
  - resource: file:///home/rektide/src/kubeless/design/native-work/reappraisal0.gpt6a.md
    title: Receiving checks broken by stale standalone locks after the version bump
  - resource: /code/planning.md
    title: Existing pre-1.0 design posture
  - resource: /constitution/structure.md
    title: Reusable practice belongs with its concern owner; project mechanisms retain native homes
---

# Pre-alpha versioning

**Preferred default, not mandatory policy:** for first-party pre-alpha work,
favor visible API-change markers and deliberate adoption of reviewed source
revisions. This records the human's current preference, drawn from Kubeless.
Existing project policies and actual publication, compatibility, and persisted-data
obligations determine applicability; adopting this guidance is a project choice.

## Terms

- **API-change marker:** a version increment that signals public-interface work.
- **Reviewed revision:** the exact source commit a consumer has selected and
  checked for its use.
- **Lockstep workspace:** a coordinated package family sharing one version and
  moving its internal version requirements together.

## The default

1. **Prefer the coherent design.** Public interfaces may evolve or break during
   pre-alpha work. Preserve load-bearing semantics and account for real consumers;
   avoid preserving superseded API shapes merely because they already exist.
2. **Bump the minor for every API-bearing change.** Any public-interface addition,
   modification, or removal increments `0.N.0` to `0.(N+1).0` in the same change.
   Purely internal fixes need no bump. The trigger is interface change, rather
   than a release milestone or only a breaking change.
3. **Use lockstep where packages evolve together.** Keep one inherited workspace
   version and exact internal requirements (`=X.Y.Z`). Move those requirements,
   the root version, and affected lockfiles together. Independently released
   packages can retain their own versioning policies.
4. **Make consumer uptake revision-specific.** Consumers select a reviewed Git
   revision and verify their integration against it. Under this policy, version
   equality establishes neither source identity nor compatibility: internal work
   can leave the version unchanged. The minor is a signal, not a SemVer
   compatibility promise.

This convention is intentionally unconventional. A package manager still applies
its normal version-range rules; state the policy locally so those rules are not
mistaken for the project's compatibility commitment.

## Delivering a version change

Prefer a project-owned bump mechanism that updates the coordinated manifests
and checks for stale requirements. Refresh affected root and standalone receiving
lockfiles, including probes or examples with their own workspaces. Then run the
relevant locked resolution and integration checks against the final bumped
checkpoint. Earlier passing checks describe the earlier checkpoint.

A moving sibling-path dependency remains moving even with an exact version
requirement. The requirement is a useful drift alarm, not an immutable source pin.
Record the selected revision when using paths; merely raising a consumer's
requirement does not demonstrate compatibility. Coordinate external consumer
updates at their own receiving boundary.

Version markers and registry releases have different triggers. If the project
publishes packages, follow its release policy for artifact identity and version
selection; the no-bump rule for internal work does not permit replacing an
already published artifact. Rust's publishing mechanics remain in
[Rust conventions](/code/rust/GLOBAL.md).

## Kubeless precedent

[Kubeless's working guidance](file:///home/rektide/src/kubeless/AGENTS.md#pre-alpha-status-and-versioning)
declares unpublished, workspace-internal crates, one inherited version, exact
internal pins, and a minor bump for every public-interface change. Its
[README](file:///home/rektide/src/kubeless/README.md) describes path or
reviewed-revision Git adoption. Unpublished status describes this precedent;
each adopting project decides its own distribution policy.

The research traced the API-change rule to Kubeless Git commit
`4e4d07071e23873598482abdb3584f2e27a602cc`, which replaced the earlier
milestone-only bump rule. The
[bump script](file:///home/rektide/src/kubeless/scripts/bump-workspace-version.py)
updates jj-tracked Cargo manifests and offers separate `--check` and `--locks`
operations. It accepts a supplied pre-1.0 version; deciding whether an API changed
and choosing the next minor remain the author's responsibility. Its manifest
check does not establish consumer compatibility or lockfile freshness.

The [native-work reappraisal](file:///home/rektide/src/kubeless/design/native-work/reappraisal0.gpt6a.md)
records standalone receiving locks becoming stale after the 0.2.0 bump. That
failure grounds the final-checkpoint and receiving-lock guidance above. The
project's script and detailed history stay in Kubeless; this page owns the
reusable preference.

## Related practice

- [Planning's pre-1.0 posture](/code/planning.md#planning-breaking-changes)
  supplies the design latitude; this page adds version markers and consumer uptake.
- [Commit discipline](/code/commits.md) preserves coherent checkpoints throughout
  development, including work that needs no API version bump.
- [jj and Git](/vcs/jj/README.md) owns source-history mechanics and investigation.
- [Coding practice](/code/README.md#code-inventory) indexes this maintained home;
  [code's ambient contribution](/code/GLOBAL.md) carries its brief default cue.
