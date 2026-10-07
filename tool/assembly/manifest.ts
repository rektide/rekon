import * as Effect from "effect/Effect";
import * as Option from "effect/Option";
import * as Schema from "effect/Schema";
import { dirname, isAbsolute, normalize, resolve } from "node:path";
import { ProvenanceText } from "../md/fragment.ts";

/** Deliberately absent in production until the handwritten AGENTS.md has been fully decomposed. */
export const DEFAULT_MANIFEST = "constitution/agents.manifest.json";

const relativePathChecks = [
  Schema.makeFilter((path: string) => path.trim().length > 0, {
    message: "must be a non-empty path string",
  }),
  Schema.makeFilter((path: string) => !isAbsolute(path), { message: "must be a relative path" }),
  Schema.makeFilter((path: string) => normalize(path) === path, {
    message: "must be a normalized path without redundant separators",
  }),
] as const;
const RelativePath = Schema.String.check(...relativePathChecks);

const FragmentPath = ProvenanceText.check(
  ...relativePathChecks,
  Schema.makeFilter((path) => !path.split("/").some((part) => part === "." || part === ".."), {
    message:
      "fragment paths must not contain '.' or '..' segments or escape the manifest directory",
  }),
).pipe(Schema.brand("rekon/assembly/ManifestFragmentPath"));

export const AssemblyManifest = Schema.Struct({
  fragments: Schema.NonEmptyArray(FragmentPath),
  output: RelativePath,
});
export type AssemblyManifest = typeof AssemblyManifest.Type;

export class ManifestValidationError extends Schema.TaggedError<ManifestValidationError>()(
  "ManifestValidationError",
  { path: Schema.String, message: Schema.String },
) {}

export class DuplicateFragmentPathError extends Schema.TaggedError<DuplicateFragmentPathError>()(
  "DuplicateFragmentPathError",
  { path: Schema.String, fragment: Schema.String, prior: Schema.String, message: Schema.String },
) {}

export class OutputOverwritesInputError extends Schema.TaggedError<OutputOverwritesInputError>()(
  "OutputOverwritesInputError",
  { output: Schema.String, input: Schema.String, message: Schema.String },
) {}

/** Parse the explicit declaration; unknown auxiliary keys do not change selection. */
export const parseManifest = Effect.fn("assembly.parseManifest")(function* (
  raw: string,
  path: string,
) {
  return yield* Schema.decodeUnknownEffect(Schema.fromJsonString(AssemblyManifest))(raw).pipe(
    Effect.catchTag("SchemaError", (cause) =>
      Effect.fail(
        new ManifestValidationError({
          path,
          message: `${path}: invalid manifest: ${cause.message}`,
        }),
      ),
    ),
  );
});

export interface AssemblyPlan {
  readonly manifestPath: string;
  readonly fragments: ReadonlyArray<{ readonly path: string; readonly absolute: string }>;
  readonly output: string;
}

/** Lexical Node path planning: manifest-relative selection, invocation-relative override. */
export const planAssembly = Effect.fn("assembly.planAssembly")(function* (
  manifest: AssemblyManifest,
  manifestPath: string,
  cwd: string,
  outputOverride: Option.Option<string> = Option.none(),
) {
  const absoluteManifest = resolve(cwd, manifestPath);
  const directory = dirname(absoluteManifest);
  const seen = new Map<string, string>();
  const fragments: Array<{ path: string; absolute: string }> = [];
  for (const path of manifest.fragments) {
    const absolute = resolve(directory, path);
    const prior = seen.get(absolute);
    if (prior !== undefined) {
      return yield* new DuplicateFragmentPathError({
        path: manifestPath,
        fragment: path,
        prior,
        message: `${manifestPath}: manifest lists the same fragment twice: "${prior}" and "${path}" both resolve to ${absolute}`,
      });
    }
    seen.set(absolute, path);
    fragments.push({ path, absolute });
  }
  const output = Option.match(outputOverride, {
    onNone: () => resolve(directory, manifest.output),
    onSome: (override) => resolve(cwd, override),
  });
  const input = output === absoluteManifest ? absoluteManifest : seen.get(output);
  if (input !== undefined) {
    return yield* new OutputOverwritesInputError({
      output,
      input,
      message:
        output === absoluteManifest
          ? `assembly output must not overwrite its manifest: ${output}`
          : `assembly output must not overwrite source fragment: ${input}`,
    });
  }
  return { manifestPath: absoluteManifest, fragments, output } satisfies AssemblyPlan;
});
