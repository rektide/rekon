---
type: ArchitecturalExploration
title: The first application reveals contracts, not just missing ports
description: Parent architectural reading of runtime, project, and theme coordination; candidate fronts to combine with the separately commissioned storage evidence before making final picks.
resource: /praxis/post-build/colorlu/fronts0.gpt6a.md
tags: [colorlu, luffect, architecture, causality, effect-v4, possibilities]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
sources:
  - { resource: /praxis/post-build/colorlu/brief0.gpt6a.md, title: Commission and pinned baselines }
  - { resource: "https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/lua/colorlu/app/workflows.lua", title: Implemented workflows }
  - { resource: "https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/Schema.ts", title: Pinned upstream Schema including JSON Patch derivation }
---

# the useful new vantage point

The new application validates the decision to stay editor-local while making
host behavior replaceable and resource ownership explicit. It also falsifies a
simplistic story about the port waves: adding `FiberHandle`, `switchMap`, and
`SubscriptionRef` did **not** make application coordination disappear. Nor should
all three be forced into the application just because they exist.

Colorlu actually uses a plain Ref, two scoped event streams, a FiberHandle with
an explicitly awaited clear, causal stamps, and a verified apply intent. Its
important complexity is **which fact authorizes which action, at which point**.
This is visible in [workflows, lines 41–188][workflow], not inferred from module
names. This note records the parent's conceptual pass; final priorities await
the storage/data evidence requested in the [brief](/praxis/post-build/colorlu/brief0.gpt6a.md).

## front 1: distinguish authority from completed fact

Four things must not become one undifferentiated “current state”:

| fact | lifetime and meaning |
|---|---|
| New cwd request | Revokes old automatic-restore authority immediately, before detection finishes |
| Resolved ProjectRef or no project | A result of a particular request; not the request itself |
| Verified editor transition | A completed fact, possibly correlated to a still-settling apply intent |
| Persisted choice | A commit under the event's captured project, not whichever project is current at write completion |

The [source captures and replaces request identity before scheduling][source].
The [workflow retains old-project events but cancels obsolete detection/restore][workflow].
The corresponding tests distinguish [completed A events, no-project transitions,
and invalidation before host submission][workflow-tests]. These are valuable
differences to preserve, not accidental duplication to erase with `switchMap`.

**Potential offering:** a project-activation module with request and resolution
views, including pending, resolved absence, and failure. Its underlying runtime
machinery belongs with hosted execution; ProjectRef/detection semantics belong
with luf-settings; “a user's new color choice beats restore” stays with colorlu.
`SubscriptionRef` could expose a resolved snapshot without pretending snapshot
replacement preserves every event that deserves durable recording.

## front 2: make host admission a reusable adapter, not a new Effect runtime

[Submission][submission] gives colorlu an owned FIFO, captured host-turn batches,
and immediate lifetime revocation. [Controller][controller] composes that with
ManagedRuntime ownership, a startup barrier, shutdown, and a predecessor-disposal
barrier. Its pick submission currently acknowledges queue admission, not verified
application or storage completion. Those milestones deserve explicit contracts
if this becomes a reusable entry point.

The useful extraction is not “all Effects must be deferred.”
[ManagedRuntime.runFork][managed] deliberately enters the normal interpreter, and
[Scheduler][scheduler] already owns subsequent runnable work. A Neovim adapter
can specify capture/submit/close behavior without altering generic Effect entry.
Its proof includes late callbacks, registration failures, reentrant host loops,
and shutdown while entered work is finishing—not merely a happy-path timer.

**Potential offering:** managed Lua applications hosted in Neovim, with commands,
autocmd subscriptions, cleanup and observable shutdown using a small, documented
host interface. Keep theme intents and cwd policy out of it. Whether shipped as
an optional luffect host namespace or a sibling host package is a packaging
decision, not a reason to invent a universal plugin framework now.

## front 3: grow the stream substrate at the source, not only at the operator

[Stream.callback][callback] currently creates its own queue, exposes synchronous
offer/fail/end functions, and optionally registers returned cleanup. A later
`Stream.buffer` does not retroactively bound that source queue. Choosing dropping,
sliding, suspension, or lossless delivery must address the admission point as well
as the downstream pump. Neovim callbacks cannot simply suspend until storage
catches up.

Upstream's [Stream.callback][upstream-callback] accepts an effectful registration
over a queue and exposes buffer strategy; its [Channel callback constructor][channel-callback]
is the shared substrate. Luffect's [merge, buffer, and switchMap][stream-pumps]
repeat substantial pump/queue/scope machinery. That is a more promising breadth
question than immediately adding three more operators: should a focused
Channel-shaped substrate concentrate acquisition, terminal causes and cancellation?

**Potential offering:** resource-safe event and byte pipelines usable by editor
integrations and process/file adapters. First compare a faithful Channel subset
with a smaller internal extraction. Prove that the existing operators retain
their distinct behavior; do not require colorlu to be rewritten as a demonstration.

## front 4: recover the real Differ direction

The earlier [drift sheet][drift] correctly found a type-only public `Differ`
interface and the absence of the old v3 combinator family. Its conclusion that
patch-based updates would therefore be new design rather than v4 parity was
too broad—even its historical account points toward the surviving implementation.

At the pinned newer revision:

- [`Differ.ts:27–31`][differ] specifies `empty`, `diff`, `combine`, `patch`.
- [`JsonPatch.get`][jsonpatch] computes deterministic structural changes;
  `JsonPatch.apply` applies them. Its implemented operation family is
  add/remove/replace, not the entire RFC 6902 operation set.
- [`Schema.toDifferJsonPatch`][schema-differ] is a public runtime capability.
  [Its implementation][differ-impl] encodes two values to JSON to diff them,
  concatenates patches to combine them, and encodes → patches → decodes on apply.

**Potential offering:** inspectable settings changes—migration previews, policy
editing, and deliberate commit of validated changes. This follows an actual v4
path. A faithful Lua JSON value model and codec contract are prerequisites; patch
algebra does not repair a lossy representation. Undo also needs recorded prior
state or inverse computation; concurrent patch application still needs an
explicit conflict policy. None of those guarantees comes merely from `Differ`.

## constraints worth preserving during synthesis

- **Path belongs to Effect; storage canonicalization is not generic Path.** The
  [port explicitly trims trailing separators to suit project digests][path].
  [Upstream normalize preserves them][upstream-path], and upstream supplies a
  POSIX Context layer. Preserve existing project IDs by retaining an explicit
  identity policy, not by treating a consumer-driven semantic change as faithful
  core behavior. The human asked to leave the current Path code alone; this
  assessment changes no code.
- **A checkout-level vertical is not package-level conformance.** Colorlu
  [modifies package.path to reach sibling checkouts][dependencies]. Luffect's
  [rockspec module table][rockspec] omits newly added Effectable, FileSystem,
  Path, SubscriptionRef, and FiberHandle. This is source-observed packaging
  drift, not the result of an installation test. A clean installed-consumer
  contract would make these offerings genuinely available outside this layout.
- **Diagnostics can be an application of the same model.** [Health][health]
  cannot currently expose settings provenance and groups absence with detection.
  Request/resolution/commit outcomes could support explanations and deterministic
  traces without adding a second state store or immediately porting an entire
  observability stack.

## cross-references and basis

This note is source inspection plus architectural inference; it did not rerun
the author-reported suites. Source coordinates use the pinned revisions in the
[brief](/praxis/post-build/colorlu/brief0.gpt6a.md); some local product commits are
not published yet, so the canonical links also serve as source coordinates.

- [Post-build praxis](/praxis/post-build/README.md) explains how these fronts are
  gathered without treating them as implementation approval.
- [High/low strategy](/assess/strategies/high-low0.gpt6a.md) supplies the
  distinction between conceptual possibilities and their close evidence.
- [First-light checkpoint][checkpoint] records what the build author tested,
  the local dependency safeguards, and the still-unperformed configured cutover.

[workflow]: https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/lua/colorlu/app/workflows.lua#L41-L188
[source]: https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/lua/colorlu/project/source.lua#L12-L30
[workflow-tests]: https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/spec/workflows_spec.lua#L156-L245
[submission]: https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/lua/colorlu/runtime/submission.lua#L10-L40
[controller]: https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/lua/colorlu/runtime/controller.lua#L66-L138
[managed]: https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect/ManagedRuntime.lua#L135-L159
[scheduler]: https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect/Scheduler.lua#L101-L136
[callback]: https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect/Stream.lua#L220-L260
[upstream-callback]: https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/Stream.ts#L695-L701
[channel-callback]: https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/Channel.ts#L526-L533
[stream-pumps]: https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect/Stream.lua#L471-L792
[drift]: https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/.design/drift/drift.glm53.md#L63-L90
[differ]: https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/Differ.ts#L27-L31
[jsonpatch]: https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/JsonPatch.ts#L139-L183
[schema-differ]: https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/Schema.ts#L14043-L14065
[differ-impl]: https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/internal/schema/toDifferJsonPatch.ts#L8-L23
[path]: https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect/Path.lua#L24-L34
[upstream-path]: https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/Path.ts#L477-L493
[dependencies]: https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/lua/colorlu/runtime/dependencies.lua#L6-L22
[rockspec]: https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect-dev-1.rockspec#L20-L69
[health]: https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/lua/colorlu/health.lua#L13-L39
[checkpoint]: https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/.design/first-light/first-light.gpt6astra.md
