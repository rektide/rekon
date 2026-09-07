---
type: EvidenceReport
title: Storage and data-contract ground after colorlu first light
description: A bounded source and probe pass over colorlu, luf-settings, luffect, and pinned Effect v4 storage contracts.
resource: /praxis/post-build/colorlu/storage-evidence0.gpt56s.md
tags: [praxis, post-build, colorlu, luf-settings, luffect, storage, contracts]
status: draft
generated: { by: "model:openai/gpt-5.6-sol#high", at: 2026-09-06 }
verified: { by: "model:openai/gpt-5.6-sol#high", at: 2026-09-06, scope: "revision-pinned source inspection and four bounded local probes" }
stale_after: 2026-12-06
sources:
  - id: inquiry-brief
    resource: /praxis/post-build/colorlu/brief0.gpt6a.md
    title: After first light inquiry brief
  - id: colorlu
    resource: https://github.com/rektide/colorlu/tree/222bd7ce472fee9f1f5cc1ce5796b82d42f19762
    title: Pinned colorlu product revision
  - id: luf-settings
    resource: https://github.com/rektide/luf-settings/tree/2c963db1eb20bd87bbb33265aeece41a8e8e49be
    title: Pinned luf-settings product revision
  - id: luffect
    resource: https://github.com/rektide/luffect/tree/e6d74195a40f20bdc6460dc654797c1fcaafb1f1
    title: Pinned luffect product revision
  - id: effect-v4
    resource: https://github.com/Effect-TS/effect/tree/bd393d63c19bdd0ab212d95576cec89051c8501c
    title: Later pinned Effect v4 reference revision
---

# result

The vertical found a valuable shape: explicit project identity, setting
descriptors, namespace documents, and one policy-bearing service over thin stores.
That shape already removes ambient-project writes and gives colorlu one canonical
persistence path. The evidence does **not** yet support a reusable claim that the
memory and filesystem stores implement the same data contract.

The highest-leverage move is not another colorlu wrapper. It is to make the
storage boundary carry immutable serialized snapshots (or enforce an equivalent
JSON-value snapshot contract), then give commits explicit guarantees for
visibility, cancellation outcome, crash durability, and isolation. A smaller
luffect FileSystem completion—scoped temporary resources and synced file
handles—would let luf-settings implement that policy without bespoke host I/O.

## inspected baselines

| repository | revision | role in this pass |
|---|---|---|
| `rektide/colorlu` | `222bd7ce472fee9f1f5cc1ce5796b82d42f19762` | compatibility and theme-setting obligations |
| `rektide/luf-settings` | `2c963db1eb20bd87bbb33265aeece41a8e8e49be` | service, document, codec, stores, identity, and tests |
| `rektide/luffect` | `e6d74195a40f20bdc6460dc654797c1fcaafb1f1` | cancellation, resources, and current FileSystem behavior |
| `Effect-TS/effect` | `bd393d63c19bdd0ab212d95576cec89051c8501c` | newer FileSystem, PlatformError, KeyValueStore, and Schema reference |

The earlier Effect working-copy baseline `96ced895a07f89b2dd03c3e470884f7e25063696`
was not used to infer newer behavior. All Effect claims below refer to the later
fixed revision. Prior suite counts remain reports; no complete product suite was
rerun.

# source-verified contract map

## what colorlu adds, and who should own it

Colorlu currently adds exactly three things above luf-settings:

1. **Theme domain policy.** Its descriptor fixes namespace/key/version, rejects
   unsafe theme names on decode, asserts the same invariant on encode, and declares
   the incumbent locator. This belongs to colorlu: it is the `ThemeName` contract,
   not storage machinery ([`theme/setting.lua` lines 5–12](https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/lua/colorlu/theme/setting.lua#L5-L12),
   [`theme/name.lua` lines 5–9](https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/lua/colorlu/theme/name.lua#L5-L9)).
2. **Legacy object validation.** The compatibility FileSystem wrapper rejects
   top-level non-object bytes only for direct legacy JSON files, then delegates
   parsing and descriptor decoding ([`settings/compat.lua` lines 14–31](https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/lua/colorlu/settings/compat.lua#L14-L31)).
   This should move to luf-settings' `color-persist` legacy adapter: generic
   FileSystem should return bytes, while the adapter owns the expected document
   shape. The current adapter instead skips every decoded non-table and also
   treats arrays as maps with no matching key ([`fs.lua` lines 175–199](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/fs.lua#L175-L199)).
3. **Whole-commit masking.** `protectStore` makes an entered
   `writeDocument` uninterruptible, so disposal waits rather than abandoning a
   temp file ([`settings/compat.lua` lines 36–53](https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/lua/colorlu/settings/compat.lua#L36-L53)).
   This compensates for luf-settings' temp lifecycle, not theme policy. The real
   host contract gates rename, requests interruption, then establishes that the
   commit lands and no temp remains ([`settings_contract.lua` lines 13–36](https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/spec/host/settings_contract.lua#L13-L36)).

The proper ownership split is:

| guarantee or policy | owner |
|---|---|
| effectful host operations, descriptor/resource cleanup, normalized platform errors, and a `sync` capability | **luffect FileSystem/runtime** |
| document shape, codec, snapshots, namespace serialization, temp ownership, commit protocol, migration, and legacy source shape | **luf-settings** |
| valid theme names, whether to await commits during reload/exit, conflict retry, failure presentation, and required durability profile | **colorlu/caller policy** |

## cancellation and resources: four different promises

The current luf-settings writer creates a same-destination-path temp, performs
`writeFile` then `rename`, and removes the temp only inside `Effect.catch`
([`fs.lua` lines 88–106](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/fs.lua#L88-L106)).
In luffect, `catch` handles expected failures but propagates defects and interrupts
([`internal/effect.lua` lines 427–443](https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect/internal/effect.lua#L427-L443)).
The comment and colorlu workaround are therefore source-supported: interruption
does not enter that cleanup handler.

This does not mean luffect leaks file descriptors. Its uv whole-file operations
use `acquireRelease` for the open descriptor and scoped execution
([`uv.lua` lines 264–340](https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect/uv.lua#L264-L340));
the asynchronous wrapper also attempts uv cancellation and releases a late-opened
descriptor ([`uv.lua` lines 42–89](https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect/uv.lua#L42-L89)).
The unowned resource is the **filesystem temp path** after creation.

Four guarantees must remain separate:

- **Atomic visibility:** same-filesystem replace is the intended basis. The uv
  FileSystem integration test establishes destination replacement, and the
  luf-settings success test sees new bytes and no normal-path leftovers
  ([`filesystem_spec.lua` lines 61–67](https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/spec/filesystem_spec.lua#L61-L67),
  [`fs_spec.lua` lines 134–149](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/spec/fs_spec.lua#L134-L149)).
  No concurrent-reader probe establishes old-or-new visibility on every host;
  the generic FileSystem surface itself merely exposes `rename`.
- **Crash durability:** no `fsync` is invoked for temp contents or the containing
  directory. Atomic namespace visibility does not imply persistence across power
  loss. Luffect's uv layer has low-level `fsFsync`, but its FileSystem service does
  not expose a file handle or sync operation
  ([`uv.lua` lines 126–142](https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect/uv.lua#L126-L142),
  [`FileSystem.lua` lines 81–163](https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect/FileSystem.lua#L81-L163)).
- **Cross-process isolation:** the filesystem store rereads for a compare before
  writing, but another process may commit after that read and before rename. The
  implementation explicitly documents last-writer-wins when this race escapes
  best-effort detection ([`fs.lua` lines 12–16](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/fs.lua#L12-L16),
  [`fs.lua` lines 129–171](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/fs.lua#L129-L171)).
- **Graceful shutdown:** colorlu's mask makes runtime disposal await an entered
  commit. It cannot make an abrupt editor/process exit pump the event loop, nor
  can storage alone decide how long the application should wait. This remains a
  caller lifecycle policy.

Masking also does not make interruption observationally identical to success.
Luffect records a deferred interrupt while masked and delivers it when the
interruptibility frame is restored
([`internal/effect.lua` lines 564–619](https://github.com/rektide/luffect/blob/e6d74195a40f20bdc6460dc654797c1fcaafb1f1/luffect/internal/effect.lua#L564-L619)).
The bytes can therefore be committed while the interrupted fiber does not return a
normal success. Exact retry safety would require a commit identity/read-back policy,
not a wider mask.

## the service is strong policy concentration, with hidden path differences

`ProjectSettings.make` usefully centralizes explicit `ProjectRef`, descriptor
decode/migration, legacy precedence, skip-unchanged writes, and per-project /
namespace locking ([`service.lua` lines 118–186](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/service.lua#L118-L186)).
That is a reusable architectural success.

Callers must nevertheless know these differences:

- `get` and `modify` decode an existing target envelope; `set` compares only the
  encoded representation, and `remove` deletes without decoding
  ([`service.lua` lines 189–229](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/service.lua#L189-L229),
  [`service.lua` lines 238–282](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/service.lua#L238-L282)).
  Thus a structurally valid document containing a malformed target can be
  overwritten by `set` or erased by `remove`, while `modify` fails loudly.
- `set` invokes `setting.encode(value)` eagerly before constructing the locked
  effect, whereas `modify` invokes it during effect execution. The descriptor
  declares encoding total and failures defects, but failure timing still differs
  across methods ([`setting.lua` lines 26–37](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/setting.lua#L26-L37)).
- Expected setting failures are tagged and absence is `Option.none`, which is a
  sound visible contract ([`errors.lua` lines 1–38](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/errors.lua#L1-L38)).
  Filesystem adaptation retains readable message text but does not attach the
  lower-level error in a `cause` field despite the write-error comment; only the
  source adapter currently demonstrates structured cause preservation
  ([`fs.lua` lines 61–105](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/fs.lua#L61-L105),
  [`source.lua` lines 30–45](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/source.lua#L30-L45)).

This path knowledge explains why colorlu's first vertical can be correct without
the service yet being uniformly unsurprising. A future API should either apply one
malformed-target policy to every mutator or name the exceptional operation (for
example, an explicit unchecked repair/replace).

## memory and file stores do not yet share one VALUE algebra

The document logic promises shallow-copy immutability. It copies only the root and
`settings` map; sibling envelopes, nested values, and unknown nested structures
remain shared references ([`document.lua` lines 26–63](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/document.lua#L26-L63)).
The memory store returns and installs document tables directly
([`memory.lua` lines 41–95](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/memory.lua#L41-L95));
the filesystem necessarily serializes and reparses. Consequently:

- a caller can mutate an encoded table after memory `set` and change stored state
  without another write;
- a decoder can return an alias into memory storage;
- an update callback can mutate its `current` value before choosing an action;
- memory accepts functions, non-string object keys, cycles, and non-finite numbers
  until some later operation objects, while the file codec defects during encode.

The custom JSON codec is also narrower than its “strict JSON” header suggests.
Lua `nil` is used for JSON `null`, so object properties disappear and array slots
cannot retain null; empty arrays and empty objects both become `{}`. The number
parser accepts at least leading-zero `01`
([`json.lua` lines 43–53](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/json.lua#L43-L53),
[`json.lua` lines 242–265](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/json.lua#L242-L265),
[`json.lua` lines 267–367](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/json.lua#L267-L367)).
`false` is preserved because absence checks use `== nil`, which is good, but it is
not evidence for the missing null/array distinctions.

Unknown root fields and sibling settings are deliberately preserved and tested
([`contract.lua` lines 162–176](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/spec/contract.lua#L162-L176)).
Unknown extensions on the **target envelope** are not: `put` replaces that envelope
with exactly `{ version, value }`. Future document formats also have no negotiated
policy: shape validation neither requires nor checks `format`, so an unrecognized
format can be accepted and rewritten
([`document.lua` lines 48–55](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/document.lua#L48-L55),
[`document.lua` lines 102–121](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/document.lua#L102-L121)).

Finally, project metadata makes the canonical root available for diagnosing a hash
collision, but `writeDocument` only creates metadata when absent; it never compares
an existing metadata root with the supplied `ProjectRef`
([`project.lua` lines 1–12](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/project.lua#L1-L12),
[`fs.lua` lines 161–171](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/lua/luf-settings/fs.lua#L161-L171)).
The data makes a collision manually diagnosable, not automatically rejected.

# decisive tests present and absent

The test architecture is a real strength. One shared suite runs against memory and
filesystem and establishes ordinary string setting behavior: absence, round trips,
skip-unchanged, project separation, remove/modify, unknown root and siblings,
legacy precedence, decode errors, and migration
([`contract.lua` lines 51–257](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/spec/contract.lua#L51-L257)).
Backend-specific tests establish memory CAS and in-process serialization
([`memory_spec.lua` lines 75–130](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/spec/memory_spec.lua#L75-L130)),
plus filesystem layout, normal replacement, malformed-file preservation, and
legacy precedence ([`fs_spec.lua` lines 105–225](https://github.com/rektide/luf-settings/blob/2c963db1eb20bd87bbb33265aeece41a8e8e49be/spec/fs_spec.lua#L105-L225)).

The decisive missing contract cases are:

1. a shared JSON-value matrix containing nested `false`, `null`, empty array,
   empty object, arrays containing null, boundary numbers, and invalid Lua values;
2. input, returned-value, seeded-document, and `modify(current)` mutation tests;
3. malformed target envelopes through **each** of get/set/remove/modify, plus
   eager-versus-effectful decoder/encoder throws;
4. unknown target-envelope extensions and explicit unknown `format` behavior;
5. interruption before/during temp write, before/during rename, and cleanup
   failure—without relying on a caller mask;
6. concurrent-reader visibility, real cross-process writers, and separately
   specified crash-durability tests;
7. existing `project.json` whose root disagrees with the requested project.

Passing the present string contract is therefore evidence for the service shape,
not coverage of a general reusable document algebra.

# bounded probes

One deterministic script was run from Rekon:

```text
$ .test-agent/post-build-colorlu-storage/probe.lua
PROBE json-algebra: null=nil/no-error; [] re-encodes {}; object null drops while false survives; 01 accepted
PROBE snapshot-parity: memory observes post-set mutation=2 with one write; filesystem remains=1
PROBE value-boundary: memory accepts function; filesystem fails with JSON encoding defect
PROBE failure-detail: malformed JSON keeps text/tag but SettingsReadError has no structured cause
```

The script uses explicit absolute `package.path` entries and writes only beneath
`/home/rektide/src/rekon/.test-agent/post-build-colorlu-storage/fixture-state`.
One intermediate hypothesis—that malformed parser text was lost—was rejected by
the probe; the final assertion records the narrower fact that text survives but a
structured cause does not. These are probes of four source-identified cases, not
suite coverage.

# relevant Effect v4 leverage

The newer Effect sources offer useful contracts, but none is an automatic drop-in
atomic settings store:

- `FileSystem` exposes scoped temporary files, scoped open handles, and
  `File.sync` as actual effectful operations
  ([`FileSystem.ts` lines 193–228](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/FileSystem.ts#L193-L228),
  [`FileSystem.ts` lines 996–1050](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/FileSystem.ts#L996-L1050)).
  The Node adapter backs `sync` with `fsync` and scoped temp cleanup with
  `acquireRelease`; these are runtime semantics, not type-only declarations
  ([`NodeFileSystem.ts` lines 238–275](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/platform/node-shared/src/NodeFileSystem.ts#L238-L275),
  [`NodeFileSystem.ts` lines 392–416](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/platform/node-shared/src/NodeFileSystem.ts#L392-L416)).
- `PlatformError` preserves normalized host categories, operation/path data, and
  original causes ([`PlatformError.ts` lines 75–117](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/PlatformError.ts#L75-L117),
  [`PlatformError.ts` lines 134–203](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/PlatformError.ts#L134-L203)).
- KeyValueStore narrows primitive storage to strings/bytes, then derives a
  schema-aware JSON view whose get/set failures include runtime Schema errors
  ([`KeyValueStore.ts` lines 38–95](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/unstable/persistence/KeyValueStore.ts#L38-L95),
  [`KeyValueStore.ts` lines 699–811](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/unstable/persistence/KeyValueStore.ts#L699-L811)).
  Schema itself defines `null`, arrays, and objects as distinct JSON cases and
  composes JSON parsing with structural decoding
  ([`Schema.ts` lines 9084–9137](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/Schema.ts#L9084-L9137),
  [`Schema.ts` lines 14108–14173](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/Schema.ts#L14108-L14173)).

Do not copy KeyValueStore's surface and infer stronger behavior. Its default
`modify` is get-then-set, and its filesystem layer writes the destination directly
([`KeyValueStore.ts` lines 224–255](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/unstable/persistence/KeyValueStore.ts#L224-L255),
[`KeyValueStore.ts` lines 379–421](https://github.com/Effect-TS/effect/blob/bd393d63c19bdd0ab212d95576cec89051c8501c/packages/effect/src/unstable/persistence/KeyValueStore.ts#L379-L421)).
It supplies the primitive/derived-codec pattern and runtime error composition—not
luf-settings' namespace CAS, atomic visibility, migrations, or project identity.

# proposed shared contract seams

## 1. luffect: complete portable file resources

**Owner:** luffect.

Expose the already-supported low-level capability through an Effect-aligned
FileSystem: scoped `open`, a file handle with `writeAll` and `sync`, and a scoped
same-filesystem temporary-path facility. Preserve current Lua byte-string
adaptation and uv short-write/cancellation handling. Preserve normalized host
causes instead of reducing them to message strings.

This makes interruption-safe preparation and optional crash durability possible
for every Lua storage consumer. It does **not** itself promise atomic replacement,
directory durability, or a transaction; those are policies composed by the store.

## 2. luf-settings: make DocumentStore a snapshot boundary

**Owner:** luf-settings.

Prefer an immutable raw boundary such as
`read -> Option<{ bytes, revision }>` and
`commit(expectedRevision, bytes) -> CommitResult`, with runtime capability data
describing `atomicVisibility`, `crashDurability`, and `isolation`. Lua strings give
memory and file implementations the same alias-free primitive. Keep parsing,
descriptor decode/migration, unknown-field policy, and deterministic encoding in
the derived ProjectSettings service, analogous to Effect's primitive
KeyValueStore plus runtime SchemaStore.

If the table-valued DocumentStore remains, enforce a named JSON value algebra and
snapshot/clone at every boundary. Either design must add the missing mutation and
value-matrix cases to the shared adapter suite. A pure interface annotation is not
enough; both adapters must perform the runtime conversion.

Preserve the successful semantics: explicit ProjectRef, namespace shards,
read-only legacy precedence, skip-unchanged writes, migrations, and in-process
serialization. Do not call the current filesystem check “CAS” without qualifying
its best-effort cross-process window.

## 3. luf-settings + caller: specify commit outcome and durability profiles

**Owners:** luf-settings for the protocol; colorlu/other applications for policy.

Use scoped temp ownership so interruption during preparation cleans up. Define the
rename decision point and what an interrupt after that point means; if callers need
retry certainty, return or persist a commit identity that can be reconciled. Offer
an explicit visibility-only profile and, once FileSystem supports it, a synced
profile. Require a genuinely transactional/locked backend before advertising
cross-process isolation.

Colorlu should only decide whether shutdown/reload awaits outstanding commits and
how failures are rendered. Once strict legacy shape and commit cleanup live below,
both compatibility wrappers become removable rather than reusable application
infrastructure.

# architectural implications

What becomes possible:

- memory-backed tests can become faithful proofs for rich settings, not merely a
  convenient test double;
- future consumers can reuse one project-settings service while selecting honest
  persistence capabilities;
- colorlu can delete its storage compatibility layer without losing legacy safety
  or shutdown behavior;
- decoder/encoder and malformed-document behavior can become uniform enough that
  callers need not know which mutator happens to validate existing data.

What remains impossible under the inspected contracts:

- claiming JSON round-trip fidelity for all JSON values;
- inferring filesystem behavior from a passing memory test for mutable values;
- claiming fsync-backed durability, cross-process transactions, or immediate-quit
  flushing;
- knowing from an interrupted result alone whether a masked rename committed;
- automatically detecting the project-id collision that metadata was intended to
  diagnose.

# evidence limits

This pass inspected only the assigned storage/data-contract surface. It did not
assess runtime/project/theme lifecycle architecture owned by the parent inquiry,
exercise non-POSIX hosts, simulate power loss, run concurrent processes, or rerun
the reported complete suites. Atomic visibility outside the consumed uv host,
directory-fsync portability, and an appropriate cross-process mechanism remain
research/proof obligations. Recommendations are seams for synthesis, not
architecture sign-off or implementation authorization.

# cross-references

- [The post-build praxis](/praxis/post-build/README.md) supplies the distinction
  between a working path and the scope of a reusable promise; the adapter parity
  findings are a concrete instance of that distinction.
- [The inquiry brief](/praxis/post-build/colorlu/brief0.gpt6a.md) defines the
  forward-looking commission and keeps final architectural selection with the
  parent.
- [Colorlu first light](https://github.com/rektide/colorlu/blob/222bd7ce472fee9f1f5cc1ce5796b82d42f19762/.design/first-light/first-light.gpt6astra.md)
  reports the completed vertical and its existing compatibility rationale; this
  report independently narrows the storage promises and reusable seams.
