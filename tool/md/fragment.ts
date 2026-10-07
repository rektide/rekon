import { Effect, Result, Schema } from "effect";
import matter from "gray-matter";
import { lexer, walkTokens } from "marked";
import { normalize } from "node:path";

// eslint-disable-next-line no-control-regex -- control characters are forbidden in provenance fields
const CONTROL_CHARACTERS = /[\u0000-\u001f\u007f]/;

/** Values interpolated into the assembler's HTML provenance comments. */
export const ProvenanceText = Schema.String.check(
  Schema.makeFilter((text) => !text.includes("--"), {
    message: 'must not contain "--" in a provenance comment',
  }),
  Schema.makeFilter((text) => !CONTROL_CHARACTERS.test(text), {
    message: "must not contain control characters in a provenance comment",
  }),
);

export const FragmentId = Schema.String.check(
  Schema.isPattern(/^[a-z0-9]+(-[a-z0-9]+)*$/, {
    message: "must be a lowercase kebab-case identifier",
  }),
).pipe(Schema.brand("rekon/assembly/FragmentId"));

export const BundleSource = ProvenanceText.check(
  Schema.makeFilter((path) => path.length > 1 && path.startsWith("/") && !path.startsWith("//"), {
    message: "must be a bundle-root-relative path starting with a single '/'",
  }),
  Schema.makeFilter(
    (path) =>
      normalize(path.slice(1)) === path.slice(1) &&
      !path.split("/").some((part) => part === "." || part === ".."),
    { message: "must be a normalized source path without '.' or '..' segments" },
  ),
);

export const FragmentMeta = Schema.Struct({
  id: FragmentId,
  order: Schema.Int,
  source: BundleSource,
  status: Schema.Literals(["draft", "stable"]),
});
export type FragmentMeta = typeof FragmentMeta.Type;

export interface Fragment {
  readonly path: string;
  readonly meta: FragmentMeta;
  readonly prose: string;
  readonly proseBytes: number;
}

export class FragmentFrontmatterError extends Schema.TaggedError<FragmentFrontmatterError>()(
  "FragmentFrontmatterError",
  { path: Schema.String, message: Schema.String },
) {}

export class FragmentMetadataError extends Schema.TaggedError<FragmentMetadataError>()(
  "FragmentMetadataError",
  { path: Schema.String, message: Schema.String },
) {}

export class FragmentProseError extends Schema.TaggedError<FragmentProseError>()(
  "FragmentProseError",
  { path: Schema.String, message: Schema.String },
) {}

export class FragmentLinkError extends Schema.TaggedError<FragmentLinkError>()(
  "FragmentLinkError",
  { path: Schema.String, destination: Schema.String, message: Schema.String },
) {}

/** Validate relocation safety and structure without rendering or rewriting Markdown. */
export function validateProse(
  prose: string,
  path: string,
): Result.Result<void, FragmentProseError | FragmentLinkError> {
  const tokens = lexer(prose);
  const meaningful = tokens.filter((token) => token.type !== "space");
  const first = meaningful[0];
  const invalid = (detail: string) =>
    Result.fail(new FragmentProseError({ path, message: `${path}: ${detail}` }));

  if (first === undefined) return invalid("fragment has no Markdown content");
  if (first.type !== "heading" || first.depth !== 1) {
    return invalid("fragment must begin with a top-level (#) heading");
  }
  if (first.text.trim().length === 0) return invalid("fragment top-level heading has no text");
  const headings = meaningful.filter((token) => token.type === "heading" && token.depth === 1);
  if (headings.length !== 1) {
    return invalid(
      `fragment must contain exactly one top-level (#) heading, found ${headings.length}`,
    );
  }
  if (meaningful.length < 2) return invalid("fragment needs content beyond its top-level heading");

  const destinations: string[] = [];
  walkTokens(tokens, (token) => {
    if (token.type === "link" || token.type === "image" || token.type === "def") {
      destinations.push(token.href);
    }
  });
  for (const destination of destinations) {
    if (
      destination === "" ||
      destination.startsWith("#") ||
      (destination.startsWith("/") && !destination.startsWith("//")) ||
      /^[a-zA-Z][a-zA-Z0-9+.-]*:/.test(destination)
    ) {
      continue;
    }
    return Result.fail(
      new FragmentLinkError({
        path,
        destination,
        message: `${path}: destination "${destination}" is fragment-relative and would retarget when assembled; use a bundle-root-relative (/...) or protocol destination`,
      }),
    );
  }
  return Result.succeed(undefined);
}

/** Frontmatter is consumed; the body is preserved except CRLF and edge whitespace. */
export const parseFragment = Effect.fn("assembly.parseFragment")(function* (
  raw: string,
  path: string,
) {
  if (!matter.test(raw)) {
    return yield* new FragmentFrontmatterError({
      path,
      message: `${path}: fragment must declare frontmatter metadata (id, order, source, status)`,
    });
  }
  const parsed = yield* Effect.try({
    try: () => matter(raw),
    catch: (cause) =>
      new FragmentFrontmatterError({
        path,
        message: `${path}: invalid frontmatter: ${String(cause)}`,
      }),
  });
  yield* Schema.decodeUnknownEffect(ProvenanceText)(path).pipe(
    Effect.catchTag("SchemaError", (cause) =>
      Effect.fail(
        new FragmentMetadataError({
          path,
          message: `${path}: unsafe fragment path: ${cause.message}`,
        }),
      ),
    ),
  );
  const meta = yield* Schema.decodeUnknownEffect(FragmentMeta)(parsed.data).pipe(
    Effect.catchTag("SchemaError", (cause) =>
      Effect.fail(
        new FragmentMetadataError({
          path,
          message: `${path}: invalid frontmatter: ${cause.message}`,
        }),
      ),
    ),
  );
  const prose = parsed.content.replace(/\r\n/g, "\n").trim();
  const validation = validateProse(prose, path);
  if (Result.isFailure(validation)) return yield* validation.failure;
  return { path, meta, prose, proseBytes: Buffer.byteLength(prose, "utf8") } satisfies Fragment;
});
