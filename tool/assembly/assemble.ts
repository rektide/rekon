import * as Effect from "effect/Effect";
import * as FileSystem from "effect/FileSystem";
import * as Option from "effect/Option";
import * as Schema from "effect/Schema";
import { dirname, join, resolve } from "node:path";
import { compareDocument, renderDocument, type DocumentCheck } from "../md/document.ts";
import { parseFragment, type Fragment } from "../md/fragment.ts";
import { parseManifest, planAssembly } from "./manifest.ts";

export const FRAGMENT_READ_CONCURRENCY = 8;

/** Invocation context is explicit: neither a moved manifest nor an override changes input ownership. */
export interface AssemblyRequest {
  readonly manifestPath: string;
  readonly cwd: string;
  readonly outputOverride: Option.Option<string>;
}

export interface AssemblyResult {
  readonly fragments: ReadonlyArray<Fragment>;
  readonly content: string;
  readonly totalBytes: number;
  readonly output: string;
}

export class DuplicateFragmentIdError extends Schema.TaggedError<DuplicateFragmentIdError>()(
  "DuplicateFragmentIdError",
  { id: Schema.String, path: Schema.String, prior: Schema.String, message: Schema.String },
) {}

export class DuplicateFragmentOrderError extends Schema.TaggedError<DuplicateFragmentOrderError>()(
  "DuplicateFragmentOrderError",
  { order: Schema.Int, path: Schema.String, prior: Schema.String, message: Schema.String },
) {}

const orderFragments = Effect.fn("assembly.orderFragments")(function* (
  fragments: ReadonlyArray<Fragment>,
) {
  const ids = new Map<string, string>();
  const orders = new Map<number, string>();
  for (const { meta, path } of fragments) {
    const priorId = ids.get(meta.id);
    if (priorId !== undefined) {
      return yield* new DuplicateFragmentIdError({
        id: meta.id,
        path,
        prior: priorId,
        message: `duplicate fragment id "${meta.id}" in ${path} and ${priorId}`,
      });
    }
    const priorOrder = orders.get(meta.order);
    if (priorOrder !== undefined) {
      return yield* new DuplicateFragmentOrderError({
        order: meta.order,
        path,
        prior: priorOrder,
        message: `duplicate fragment order ${meta.order} in ${path} and ${priorOrder}`,
      });
    }
    ids.set(meta.id, path);
    orders.set(meta.order, path);
  }
  return [...fragments].sort((first, second) => first.meta.order - second.meta.order);
});

/** Read the explicit selection, validate it entirely, then render deterministically. No writes. */
export const assemble = Effect.fn("assembly.assemble")(function* (request: AssemblyRequest) {
  const fs = yield* FileSystem.FileSystem;
  const raw = yield* fs.readFileString(resolve(request.cwd, request.manifestPath));
  const manifest = yield* parseManifest(raw, request.manifestPath);
  const plan = yield* planAssembly(
    manifest,
    request.manifestPath,
    request.cwd,
    request.outputOverride,
  );
  const loaded = yield* Effect.forEach(
    plan.fragments,
    Effect.fn("assembly.readFragment")(function* ({
      path,
      absolute,
    }: {
      path: string;
      absolute: string;
    }) {
      const source = yield* fs.readFileString(absolute);
      return yield* parseFragment(source, path);
    }),
    { concurrency: FRAGMENT_READ_CONCURRENCY },
  );
  const fragments = yield* orderFragments(loaded);
  const content = renderDocument(fragments);
  return {
    fragments,
    content,
    totalBytes: Buffer.byteLength(content, "utf8"),
    output: plan.output,
  } satisfies AssemblyResult;
});

/** Same-filesystem rename publishes only a complete document; scope owns staging cleanup. */
export const writeAssembly = Effect.fn("assembly.writeAssembly")(function* (
  request: AssemblyRequest,
) {
  const result = yield* assemble(request);
  const fs = yield* FileSystem.FileSystem;
  const parent = dirname(result.output);
  yield* fs.makeDirectory(parent, { recursive: true });
  const staging = yield* fs.makeTempDirectoryScoped({
    directory: parent,
    prefix: ".rekon-agents-",
  });
  const temporary = join(staging, "AGENTS.md");
  yield* fs.writeFileString(temporary, result.content);
  yield* fs.rename(temporary, result.output);
  return result;
}, Effect.scoped);

/** A missing output is stale. Every other platform failure remains a platform failure. */
export const checkAssembly = Effect.fn("assembly.checkAssembly")(function* (
  request: AssemblyRequest,
) {
  const result = yield* assemble(request);
  const fs = yield* FileSystem.FileSystem;
  const actual = yield* fs.readFile(result.output).pipe(
    Effect.map(Option.some),
    Effect.catchTag("PlatformError", (error) =>
      error.reason._tag === "NotFound" ? Effect.succeed(Option.none()) : Effect.fail(error),
    ),
  );
  return Option.match(actual, {
    onNone: (): DocumentCheck => ({ status: "stale", detail: `output missing: ${result.output}` }),
    onSome: (bytes) => compareDocument(result.content, bytes, result.output),
  });
});
