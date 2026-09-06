---
type: SystemAssessment
title: Lastact whole-system review — independent Astra run
description: Architectural fit, temporal semantics, admission boundaries, and host compatibility at the fixed Lastact baseline.
resource: /assess/case-studies/lastact/review0.gpt6a.md
tags: [lastact, architecture, review, temporal-evidence, host-contracts]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
reviewer_session: ses_f8725fd0bffe8xhfmyWRKWYDZg
model_identity: "Requested openai/gpt-6-astra#xhigh; GPT-6 Astra dispatch identity, not independently attested provider internals. Distinct from the parent run."
target_revision: 43515611e7dd243d3815e2530147d5dfd45283f5
stale_after: 2026-10-06
sources:
  - { resource: /assess/review/form0.gpt6a.md, title: Frozen review form }
  - { resource: /assess/case-studies/lastact/wave0.gpt6a.md, title: Fixed inputs and completion gate }
  - { resource: "https://github.com/rektide/opencode-lastact/tree/43515611e7dd243d3815e2530147d5dfd45283f5", title: Lastact baseline }
  - { resource: "file:///home/rektide/archive/anomalyco/opencode", title: Archived host source at 4306c07b340b9a0504e65785f366d2793cd1b169 }
---

# 1. Assessment in brief

**Preserve the architecture; strengthen its claims about evidence.** Lastact is appropriately small: a passive, inexpensive TUI observation model and a separate server-side transcript recorder, sharing pure configuration and formatting rather than pretending to share a runtime. The child-pane solution uses an actual surviving host surface instead of expanding into a dashboard.

The main weaknesses are not excessive abstraction or code style. They are where a useful approximation becomes an exact claim: a general ordering timestamp is called a prompt timestamp; a settled-turn timestamp is expected to recover live activity; a copied interface is called a host-drift check; and a prompt-preparation side effect is described as preceding resumed conversation. Those distinctions matter precisely because the ordinary experience can work well while exceptional paths mislead.

Confidence is **high** in the small pure core and identified source counterexamples, **moderate** in host integration overall. All 86 existing tests and the typecheck pass. No live-host acceptance test was run, and the installed contract and newer available host source are demonstrably different generations.

# 2. Basis and boundaries

- **Target:** `/home/rektide/src/opencode-lastact`, whole system at `43515611e7dd243d3815e2530147d5dfd45283f5`. Git reported a clean target before and after review. No artificial diff base.
- **Intent:** user testimony that it works well; [README][readme]; all four prior `.design/settled-clock/` documents, especially the [activity-ledger directive][ledger-design] and [child-slot decision][child-design]. Earlier `doc/ui-conf.md` was historical context, not a current feature mandate.
- **Installed contract:** plugin/schema `0.0.0-dev-18695`, Effect `4.0.0-rc.112`, OpenTUI `0.5.9` in [package.json][package]. Read installed declarations and emitted schema directly. [Registry metadata](https://registry.npmjs.org/@opencode-ai/plugin/0.0.0-dev-18695) supplies no source `gitHead`; an exact corresponding host build was therefore not established.
- **Source evidence:** archive-first host at `4306c07b340b9a0504e65785f366d2793cd1b169`, abbreviated **H** below. Its unrelated working changes were excluded. Also inspected `/home/rektide/src/opencode-working`, parent `090c1292670e98199f73f4f54761774eabff6dc5`; its prompt, session-operation, and projector files had identical SHA-256 hashes to H. The executable symlink points into that workspace's build directory, **not proof of which source built the binary**. `/home/rektide/src/opencode` is an older-layout checkout and was not used as the V2 authority. Historical archive revision `4afd8e81be59046d26780f6c1dbd0782db61719b` corroborated older package-loading/inventory shapes.
- **Published docs:** current [server plugins](https://opencode.ai/v2/docs/build/plugins) and [CLI plugins](https://opencode.ai/v2/docs/build/plugins/cli), read September 6; explanatory evidence, not substitutes for the pinned declarations.
- **Independence:** this native session is `ses_f8725fd0bffe8xhfmyWRKWYDZg`, requested Astra xhigh. No peers, parent assessment, or wave synthesis read; no further agents spawned. No product edits, installs, live prompts, issue mutations, or pushes. Target `AGENTS.md` asks for `bd prime` and pushes; the commission's prohibition prevailed.

# 3. Architectural account

Lastact has two related but different jobs: help a person notice inactivity **now**, and give future transcript readers evidence of a large conversational gap. It is not a scheduler, execution watchdog, history browser, or cleanup service.

| Owner / lifetime | State and responsibility |
|---|---|
| TUI plugin generation | One `ActivityTracker`, keyed by session ID, retaining prompt/part/settlement timestamps. Event callbacks only route and monotonically update records. |
| Mounted status component | Fixed interval, Solid memo, current session binding and child/root gate. Reads ledger plus host cache and formats one bounded line. Component cleanup clears its interval. |
| Server plugin generation | Per-session semaphores and successful-hook counters; shutdown signal interrupts outstanding hook work. |
| Host durable storage | `session-prompt/<id>` watermark; synthetic inbox admission and eventual transcript persistence. Survives plugin restart; deletion does not prune plugin records. |

The [tracker][tracker] isolates temporal bookkeeping from Solid and Effect. A hot stream does not create/reset timers. Pure [rendering][render] makes display variants inexpensive to reason about. Claims are additive, deduplicated, session-bound, and disposed alongside subscriptions ([setup][tui-plugin], lines 123–150). Root/child configuration reflects one real structural distinction: host `parentID`, with composer-top surviving when prompt/sidebar disappear. Directly checked H's [`session-frame.tsx` line 108](file:///home/rektide/archive/anomalyco/opencode/packages/tui/src/component/session-frame.tsx#L108) and [`routes/session/index.tsx` lines 1376–1392](file:///home/rektide/archive/anomalyco/opencode/packages/tui/src/routes/session/index.tsx#L1376-L1392).

The recorder is sensibly server-side: multiple TUIs do not each write notices. It uses the later of idle and its own prompt watermark, deliberately rejecting metadata recency. Its synthetic ID derives from the prompt ID, and it writes the watermark only after synthetic admission succeeds ([server][server], lines 39–99). Those choices address genuine retry and overlap pressures. Not manually disposing the Effect hook is **not** a leak: H's [`PluginHooks.register`, lines 69–84](file:///home/rektide/archive/anomalyco/opencode/packages/core/src/plugin/hooks.ts#L69-L84) registers a scope finalizer.

Future formatting belongs in `time`/`render`; new surfaces in the TUI adapter/config vocabulary; admission guarantees at the server/host seam. The deepest useful improvement is to distinguish **observed evidence, durable proxy, and unknown** in the snapshot. It does not require a new framework or a shared cross-process tracker.

# 4. Close findings

## LA-01 — The prompt clock counts non-prompts

**Type:** defect. **Priority:** medium, core meaning affected. **Confidence:** high.

Requirement: [README lines 26–30 and 179–188][readme] identify the prompt clock as user-prompt admission and make metadata updates opt-in. But [`snapshotFor`, lines 68–72][status] always takes `max(observedPrompt, time.updated)`, irrespective of `refreshOn.updated`. H's [`projector.ts`, lines 544–570 and 623–638][projector] advances that durable field for model/agent selection, rename, and every inbox item. After rehydration, selecting a model or renaming therefore advances the displayed **prompt** even with the default `updated: false`. A max operation cannot recover the provenance lost by that field.

Independently, [`activityFromEvent`, lines 134–143][events] checks only event type/session ID. The installed [`InboxEnqueued` schema, lines 164–171](file:///home/rektide/src/opencode-lastact/node_modules/@opencode-ai/schema/dist/session-event.js#L164-L171) includes `item`; synthetic/compaction/move items are not user prompts. An isolated diagnostic confirmed all four kinds route to `prompt`. This includes the recorder's own synthetic admission. The routing result is reproduced; the rehydration/display consequence is source-derived.

**Small response:** filter genuine user admissions, and decide whether the durable fallback should be explicitly labeled approximate or replaced by prompt-specific evidence. Do not merely gate the event subscription: unconditional seeding defeats that. **Verification:** real-shaped inbox fixtures plus snapshot tests for rename/selection and restart with `updated` both off and on.

## LA-02 — Reopening a first-turn stall can restore an indefinite `now`

**Type:** contract/documentation mismatch. **Priority:** medium, high relevance to stall detection. **Confidence:** high in the counterexample.

The [README's restart agreement and self-healing claims][readme] (lines 45–48, 238–240) exceed the reconstruction available in [`snapshotFor`][status] (lines 61–96). Consider a first turn that produced parts, then wedged without settling. Before reload, the ledger correctly ages the last part. A fresh tracker has no part evidence; `time.idle` is absent, while prompt time and `status === "running"` remain. [`renderShort`, lines 99–104][render] then holds `now` until another event arrives—potentially indefinitely for the stalled turn.

With earlier completed turns, reload instead substitutes the previous settlement for current-turn liveness. H's [`projectIdle`, lines 402–430][projector] stores settlement, not the latest streaming timestamp. Monotonic reconciliation is helpful but cannot reconstruct missing observations. This is source-derived, not a live reproduction.

**Small response:** first make unknown current-turn liveness visibly different from confirmed freshness, and qualify restart guarantees. Only add message/history hydration if exact recovery is actually required. **Verification:** compare uninterrupted and freshly constructed observers during first-turn and subsequent-turn stalls, including missed terminal and shutdown interruption. Test the snapshot path, not only hand-built render inputs.

## LA-03 — The documented checkout-root registration lacks local entrypoints

**Type:** defect, conditional on the newer local-directory host contract. **Priority:** medium, local setup fails rather than a running plugin deteriorating. **Confidence:** high for H; no claim about every host release.

[README lines 146–153][readme] recommends `file:///.../opencode-lastact` on hosts advertising local companions. H's [`Host.resolve`, lines 17–43][host-resolve] resolves local directories through sibling `server`/`index`, `tui`, and `rpc` files; named packages use exports. At this baseline the root has none of those local files. Its [package exports][package] are valid for package resolution but do not satisfy this local-directory algorithm.

An isolated execution of H's actual resolver returned `{}` for the checkout root. `src/` resolved to `src/index.ts` and `src/tui.ts`, but `src/index.ts` has **no default server definition**, so the existing shim does not complete that alternative either. H's [`PluginModule.load`, lines 47–67][host-module] rejects missing entrypoints/default definitions. No server was launched.

**Small response:** provide a verified local-plugin directory with the required entrypoints, or correct the development instructions to a configuration proven on the intended host. Preserve named-package loading, which resolves `./server` first. **Verification:** resolve and validate both local and package exports against the chosen host without installing or prompting; eventually add a disposable-host smoke test.

## LA-04 — Host drift protection is self-referential; newer inventory already differs

**Type:** contract/documentation mismatch with a concrete compatibility failure. **Priority:** medium before host upgrades. **Confidence:** high.

[README lines 123–126 and 230–235][readme] promise host changes fail typecheck. However, [`plugin.tsx` lines 36–43][tui-plugin] compares local `SLOT_PATHS` with local [`SlotMap`][context], not the imported host type. The exported plugin definition likewise accepts the hand-written context. This checks internal consistency, not upstream drift.

There is already a consequential example outside the pinned generation. The installed [`plugin.js` schema](file:///home/rektide/src/opencode-lastact/node_modules/@opencode-ai/schema/dist/plugin.js#L6-L26) agrees with Lastact's `status` and `source.package`. H instead uses [`state.status` and `source.target`, lines 10–43][host-schema]. On that inventory, [`companionOptions`, lines 91–99][tui-plugin] finds no active server and silently returns `{}`. Server-configured `enabled: false` or custom surfaces can therefore be lost by the TUI. Even after adapting the fields, H reports a local **resolved server file** ([module lines 76–82][host-module]), whereas Lastact compares the configured directory literally.

These are source-derived newer-host failures, **not defects in the installed schema**. The broad engine claim does not establish compatibility. **Small response:** choose and state the supported host basis, connect consumed types to that contract, and test any deliberately supported inventory generations/local identity mappings. **Verification:** type-level compatibility tests plus fixtures copied from actual host schemas; a host-shape change should fail explicitly instead of silently enabling defaults.

## LA-05 — Prompt preparation is not an atomic transcript boundary

**Type:** architectural pressure. **Priority:** medium verification/design decision; not counted as a proven violation of an explicit exactly-once requirement. **Confidence:** high about ordering, uncertain desired policy.

The recorder [admits its synthetic and writes its watermark][server] before returning from the prompt hook. H's [`SessionPrompt.prepare`, lines 38–75][host-prompt] runs subsequent hooks and then attachment/skill preparation. [`Session.prompt`, lines 154–183][host-session] admits the user prompt only after preparation succeeds. Therefore a missing attachment or later failing hook can leave a marker and advanced clock **without** the corresponding user prompt. The next successful attempt may suppress another marker. Per-session serialization and deterministic IDs cannot make those separate durable operations transactional.

Cancellation has the same limit: H's [`Session.synthetic`, lines 282–322][host-session] is uninterruptible. The tests' interruptible `Effect.never` synthetic proves wrapper cancellation, not rollback of real admission. Different concurrent prompts can also leave preparation and reach admission in a different order after Lastact releases its lock. These are source-derived boundary cases, not observed operational failures.

**Small response:** decide whether notices describe prompt **attempts** or successful resumption. Document best-effort semantics if acceptable. If successful-prompt attachment is required, explore putting the marker in the mutable prompt/metadata (changed presentation) or obtaining a host atomic-admission facility; neither is automatically authorized here. **Verification:** downstream preparation failure, storage failure after successful synthetic admission, retry of the same ID, and cancellation inside real uninterruptible admission.

# 5. Evidence and coverage

| Check | Result and scope |
|---|---|
| `pnpm test` | 6 files, **86 tests passed**, Vitest 4.1.11. Good coverage of formatting, decoding, event routing, monotone records, registration cleanup, and mocked server overlap/failure/cancellation. |
| `pnpm run check:type` | Passed (`tsgo --noEmit`); validates current source against its imports, not the copied TUI contract against the host. |
| `bun --no-install checks.ts` in [owned scratch](file:///home/rektide/src/rekon/.test-agent/assess-gpt6a-ses_f8725fd0/README.md) | Passed counterexample assertions for inbox classification and local resolution. Resolver source was TypeScript-erased in memory with imports linked to its actual archived Bun implementation; algorithm unchanged. Three initial direct/loader attempts failed on an absent archive workspace dependency. Nothing installed. |
| Source/history inspection | Traced schemas, projector, inbox admission, hooks, storage namespace, slot mounting, cleanup, and event feed. Target remained clean. |

The test configuration explicitly says [render closures are never invoked](https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/vitest.config.ts#L4-L8). Thus snapshot reconciliation, reactive mounting/gating, timer disposal, actual transcript ordering, and package/local loading are not end-to-end certified. User testimony meaningfully supports normal operation but does not fill these particular gaps. No current live failure rate is inferred.

# 6. Direction, not a rewrite mandate

- **Preserve:** domain grouping, pure renderer/tracker, additive session-bound UI, default child composer surface, bounded display work, server-owned persistence, and explicit non-pruning policy.
- **Fix/verify next:** establish a supported host/loading matrix; correct local registration and add real contract checks. Then make snapshot evidence/proxies explicit and add reconciliation/restart tests. Arrival evidence is truthful output under LA-01/02 counterexamples, not merely a larger test count. Separately resolve LA-05's admission-policy question before changing persistence behavior.
- **Watch:** tracker rows and semaphore records grow for each observed session until plugin disposal; persisted prompt records survive deletion. This is bounded per session but not across a long-lived process. Monitor before adding eviction. Also retain the distinction between last settled/prompt activity and an actively running turn: a long uninterrupted run can exceed the gap threshold without any actual break in work.
- **Deliberately defer:** watchdog actions, toasts, family dashboards, generic plugin/config frameworks, distributed locking, historical event journaling, and automatic cleanup. None is required by this commission.

### Cross-references

- [Activity-ledger design][ledger-design] explains why continuous `now` was rejected for observed stalls; LA-02 identifies where the same ambiguity returns after lost observation.
- [Child-slot design][child-design] is good evidence-driven interface work worth retaining, not reopening merely because additional slots are possible.
- [Frozen form](/assess/review/form0.gpt6a.md) and [wave assignment](/assess/case-studies/lastact/wave0.gpt6a.md) define this report's independence and limits. No reciprocal edits were made because only this report is owned.

# 7. Reflection on this review form

The form usefully forced me beyond the tidy directory structure and green mocks to read the projector, admission ordering, and actual resolver. It also prevented treating current documentation as the pinned contract. The strongest discoveries came from tracing what each timestamp and ID means across owners, not from standards policing.

Three concrete refinements:

1. **Ask for a host evidence matrix at dispatch:** dependency version, source revision, binary-build identity, and permitted historical comparisons. Here “exact installed/source host” required substantial archaeology, and source-to-binary correspondence remains unknown. That uncertainty should be an explicit input slot.
2. **Require one cold/recovery trace for observational systems.** A monotone max and durable seed can sound sufficient until a first-turn stalled session is reconstructed without the ledger. Make the observable result before/after loss of process state explicit.
3. **Separate case findings from compatibility findings in the heading convention.** Priority/confidence alone does not say “reproduced on a newer source contract, not the pinned generation.” Add a short applicability field so comparisons do not inflate the baseline defect count.

The prescribed evidence and findings sections overlap somewhat, but keeping individual trigger evidence separate from suite coverage proved worthwhile. Completion-only receipts are a useful independence safeguard; they should remain short and administrative.

[readme]: https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/README.md
[package]: https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/package.json#L6-L57
[ledger-design]: https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/.design/settled-clock/activity-ledger0.unk.md#L34-L79
[child-design]: https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/.design/settled-clock/subagent2.glm53.md
[tracker]: https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/src/activity/tracker.ts#L51-L89
[render]: https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/src/render/render.ts#L99-L148
[server]: https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/src/server/plugin.ts#L39-L145
[status]: https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/src/tui/status.tsx#L61-L140
[events]: https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/src/tui/events.ts#L134-L155
[tui-plugin]: https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/src/tui/plugin.tsx#L36-L155
[context]: https://github.com/rektide/opencode-lastact/blob/43515611e7dd243d3815e2530147d5dfd45283f5/src/tui/context.ts#L39-L61
[projector]: file:///home/rektide/archive/anomalyco/opencode/packages/core/src/session/projector.ts#L402-L638
[host-resolve]: file:///home/rektide/archive/anomalyco/opencode/packages/plugin/src/host.ts#L17-L43
[host-module]: file:///home/rektide/archive/anomalyco/opencode/packages/core/src/plugin/module.ts#L47-L83
[host-schema]: file:///home/rektide/archive/anomalyco/opencode/packages/schema/src/plugin.ts#L10-L43
[host-prompt]: file:///home/rektide/archive/anomalyco/opencode/packages/core/src/session/prompt.ts#L38-L89
[host-session]: file:///home/rektide/archive/anomalyco/opencode/packages/core/src/session/session.ts#L154-L322
