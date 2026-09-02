import matter from "gray-matter";
import { randomUUID } from "node:crypto";
import { mkdir, readFile, rename, writeFile } from "node:fs/promises";
import { lexer } from "marked";
import { dirname, isAbsolute, normalize, relative, resolve } from "node:path";

/**
 * Default manifest path, resolved against the working directory.
 *
 * No production manifest exists yet: the current hand-maintained `/AGENTS.md`
 * must not be overwritten until every section it contains has a declared
 * `GLOBAL.md` source fragment.
 */
export const DEFAULT_MANIFEST = "doc/agents.manifest.json";

/**
 * A path declared inside a manifest, relative to the manifest file's
 * directory. Parsed form: relative, normalized (no `.`, `..`, or redundant
 * separators), and contained under the manifest directory.
 */
declare const manifestRelativeBrand: unique symbol;
export type ManifestRelativePath = string & { readonly [manifestRelativeBrand]: true };

/** A normalized output path declared relative to the manifest directory. */
declare const manifestOutputBrand: unique symbol;
export type ManifestOutputPath = string & { readonly [manifestOutputBrand]: true };

/**
 * A repository-root-relative canonical path, as declared by fragment
 * `source` metadata. Parsed form: leading `/`, normalized, no traversal,
 * no control characters, and no `--` that could break an HTML comment.
 */
declare const bundleRootRelativeBrand: unique symbol;
export type BundleRootRelativePath = string & { readonly [bundleRootRelativeBrand]: true };

/** A resolved absolute filesystem path. */
declare const absoluteBrand: unique symbol;
export type AbsolutePath = string & { readonly [absoluteBrand]: true };

/** Identity and assembly metadata declared in a fragment's frontmatter. */
export interface FragmentMeta {
  /** Stable fragment identity; must be unique across a manifest. */
  id: string;
  /** Assembly position; unique integer across a manifest, ascending in output. */
  order: number;
  /** Canonical source README the fragment condenses, bundle-root relative. */
  source: BundleRootRelativePath;
  /** Maturity of the fragment. */
  status: "draft" | "stable";
}

/** A declared `GLOBAL.md` fragment with its frontmatter stripped. */
export interface Fragment {
  /** Fragment path exactly as written in the manifest, relative to it. */
  path: ManifestRelativePath;
  meta: FragmentMeta;
  /** Fragment Markdown body; frontmatter omitted, newlines normalized. */
  prose: string;
  /** UTF-8 context cost of the fragment prose, in bytes. */
  proseBytes: number;
}

/** Explicit declaration of which fragments assemble and where output lands. */
export interface AssemblyManifest {
  /** Fragment paths, relative to the manifest file's directory. */
  fragments: ManifestRelativePath[];
  /** Output path, relative to the manifest file's directory. */
  output: ManifestOutputPath;
}

export interface AssemblyResult {
  fragments: Fragment[];
  /** Assembled Markdown content. */
  content: string;
  /** UTF-8 length of the assembled document, in bytes. */
  totalBytes: number;
  /** Absolute output path, with any override applied. */
  output: AbsolutePath;
}

export type CheckResult = { status: "fresh" } | { status: "stale"; detail: string };

const GENERATED_HEADER = [
  "<!-- GENERATED FILE - DO NOT EDIT",
  "Assembled by `rekon agents` from declared GLOBAL.md fragments.",
  "Edit the fragment sources and re-run `node rekon.ts agents`;",
  "`node rekon.ts agents --check` detects hand edits to this file.",
  "-->",
].join("\n");

const ID_PATTERN = /^[a-z0-9]+(-[a-z0-9]+)*$/;

/** Control characters (including newlines) that have no place in identity fields. */
// eslint-disable-next-line no-control-regex -- detecting control characters is this regex's purpose
const CONTROL_CHARS = /[\u0000-\u001f\u007f]/;

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function isErrnoException(error: unknown, code: string): boolean {
  return (
    typeof error === "object" && error !== null && (error as NodeJS.ErrnoException).code === code
  );
}

/**
 * Guard a value that is interpolated into an HTML provenance comment:
 * no double hyphen that could invalidate or close the comment, no control
 * characters.
 */
function safeCommentText(value: string, label: string, displayPath: string): string {
  if (value.includes("--")) {
    throw new Error(`${displayPath}: ${label} must not contain "--"`);
  }
  if (CONTROL_CHARS.test(value)) {
    throw new Error(`${displayPath}: ${label} must not contain control characters`);
  }
  return value;
}

/** Validate an output path as normalized and relative to its manifest. */
function toManifestOutputPath(value: unknown, manifestPath: string): ManifestOutputPath {
  if (typeof value !== "string" || value.trim() === "") {
    throw new Error(`manifest 'output' must be a non-empty path string: ${manifestPath}`);
  }
  if (isAbsolute(value)) {
    throw new Error(`manifest 'output' must be a relative path: "${value}" in ${manifestPath}`);
  }
  if (normalize(value) !== value) {
    throw new Error(`manifest 'output' must be a normalized path: "${value}" in ${manifestPath}`);
  }
  return value as ManifestOutputPath;
}

/**
 * Validate a manifest fragment entry: relative, normalized (no `.`, `..`,
 * or redundant separators), and free of comment-breaking text. Containment
 * under the manifest directory is asserted again at load time.
 */
function toManifestRelativePath(entry: unknown, manifestPath: string): ManifestRelativePath {
  if (typeof entry !== "string" || entry.trim() === "") {
    throw new Error(`manifest 'fragments' entries must be non-empty strings: ${manifestPath}`);
  }
  if (isAbsolute(entry)) {
    throw new Error(
      `manifest fragment entries must be relative paths: "${entry}" in ${manifestPath}`,
    );
  }
  if (normalize(entry) !== entry) {
    throw new Error(
      `manifest fragment entries must be normalized paths without '.', '..', or redundant separators: "${entry}" in ${manifestPath}`,
    );
  }
  safeCommentText(entry, "fragment path", manifestPath);
  return entry as ManifestRelativePath;
}

/** Validate fragment `source` metadata as a bundle-root-relative canonical path. */
function toBundleRootRelativePath(value: unknown, displayPath: string): BundleRootRelativePath {
  if (
    typeof value !== "string" ||
    !value.startsWith("/") ||
    value.startsWith("//") ||
    value.length < 2
  ) {
    throw new Error(
      `${displayPath}: frontmatter 'source' must be a bundle-root-relative path starting with '/' (e.g. /doc/README.md)`,
    );
  }
  const rest = value.slice(1);
  const segments = rest.split("/");
  if (normalize(rest) !== rest || segments.some((segment) => segment === "." || segment === "..")) {
    throw new Error(
      `${displayPath}: frontmatter 'source' must be a normalized path without '.' or '..' segments: ${value}`,
    );
  }
  if (value.includes("--")) {
    throw new Error(`${displayPath}: frontmatter 'source' must not contain "--"`);
  }
  if (CONTROL_CHARS.test(value)) {
    throw new Error(`${displayPath}: frontmatter 'source' must not contain control characters`);
  }
  return value as BundleRootRelativePath;
}

/** Read and shape-check a manifest, with fragment entries parsed and deduplicated. */
export async function readManifest(manifestPath: string): Promise<AssemblyManifest> {
  let raw: string;
  try {
    raw = await readFile(manifestPath, "utf8");
  } catch (error) {
    throw new Error(`manifest not readable: ${manifestPath}`, { cause: error });
  }

  let json: unknown;
  try {
    json = JSON.parse(raw);
  } catch (error) {
    throw new Error(`manifest is not valid JSON: ${manifestPath}`, { cause: error });
  }

  if (typeof json !== "object" || json === null || Array.isArray(json)) {
    throw new Error(`manifest must be a JSON object: ${manifestPath}`);
  }
  const { fragments, output } = json as Record<string, unknown>;

  if (!Array.isArray(fragments) || fragments.length === 0) {
    throw new Error(
      `manifest 'fragments' must be a non-empty array of fragment paths: ${manifestPath}`,
    );
  }

  const dir = dirname(resolve(manifestPath));
  const seen = new Map<string, ManifestRelativePath>();
  const entries: ManifestRelativePath[] = [];
  for (const entry of fragments) {
    const path = toManifestRelativePath(entry, manifestPath);
    const resolved = resolve(dir, path);
    const prior = seen.get(resolved);
    if (prior !== undefined) {
      throw new Error(
        `manifest lists the same fragment twice: "${prior}" and "${path}" both resolve to ${resolved}`,
      );
    }
    seen.set(resolved, path);
    entries.push(path);
  }

  return { fragments: entries, output: toManifestOutputPath(output, manifestPath) };
}

/**
 * Destinations that mean the same thing regardless of where the fragment
 * file lives: self-references, anchors, bundle-root-relative paths, and
 * protocol URIs. Anything else resolves against the fragment's own
 * directory and would silently retarget when assembled elsewhere.
 */
function isLocationIndependent(href: string): boolean {
  if (href === "" || href.startsWith("#")) return true;
  if (href === "/" || (href.startsWith("/") && !href.startsWith("//"))) return true;
  return /^[a-zA-Z][a-zA-Z0-9+.-]*:/.test(href);
}

interface LinkDestination {
  type: string;
  href: string;
}

/** Walk every nested token array, collecting link, image, and definition hrefs. */
function collectLinkDestinations(value: unknown, found: LinkDestination[]): void {
  if (Array.isArray(value)) {
    for (const item of value) collectLinkDestinations(item, found);
    return;
  }
  if (typeof value !== "object" || value === null) return;
  const record = value as Record<string, unknown>;
  if (typeof record.type === "string" && typeof record.href === "string") {
    if (record.type === "link" || record.type === "image" || record.type === "def") {
      found.push({ type: record.type, href: record.href });
    }
  }
  for (const nested of Object.values(record)) collectLinkDestinations(nested, found);
}

/**
 * Validate fragment Markdown structure: exactly one depth-1 heading in
 * leading position, content beyond it, and no fragment-relative link or
 * image destinations that would silently retarget once assembled.
 */
export function validateProse(prose: string, displayPath: string): void {
  const tokens = lexer(prose);
  const meaningful = tokens.filter((token) => token.type !== "space");
  if (meaningful.length === 0) {
    throw new Error(`${displayPath}: fragment has no Markdown content`);
  }
  const first = meaningful[0];
  if (first === undefined) {
    throw new Error(`${displayPath}: fragment has no Markdown content`);
  }
  if (first.type !== "heading") {
    throw new Error(
      `${displayPath}: fragment must begin with a top-level (#) heading, found ${first.type}`,
    );
  }
  if (first.depth !== 1) {
    throw new Error(
      `${displayPath}: fragment must begin with a top-level (#) heading, found heading depth ${first.depth}`,
    );
  }
  if (first.text.trim() === "") {
    throw new Error(`${displayPath}: fragment top-level heading has no text`);
  }

  const headings = meaningful.filter(
    (token) => token.type === "heading" && (token as { depth: number }).depth === 1,
  );
  if (headings.length !== 1) {
    throw new Error(
      `${displayPath}: fragment must contain exactly one top-level (#) heading, found ${headings.length}`,
    );
  }
  if (meaningful.length < 2) {
    throw new Error(`${displayPath}: fragment needs content beyond its top-level heading`);
  }

  const destinations: LinkDestination[] = [];
  collectLinkDestinations(tokens, destinations);
  for (const destination of destinations) {
    if (isLocationIndependent(destination.href)) continue;
    throw new Error(
      `${displayPath}: ${destination.type} destination "${destination.href}" is fragment-relative and would retarget when assembled; use a bundle-root-relative (/...) or protocol destination`,
    );
  }
}

function fragmentMeta(data: Record<string, unknown>, displayPath: string): FragmentMeta {
  const id = data.id;
  if (typeof id !== "string" || !ID_PATTERN.test(id)) {
    throw new Error(`${displayPath}: frontmatter 'id' must be a lowercase kebab-case identifier`);
  }

  const order = data.order;
  if (typeof order !== "number" || !Number.isInteger(order)) {
    throw new Error(`${displayPath}: frontmatter 'order' must be an integer`);
  }

  const source = toBundleRootRelativePath(data.source, displayPath);

  const status = data.status;
  if (status !== "draft" && status !== "stable") {
    throw new Error(`${displayPath}: frontmatter 'status' must be "draft" or "stable"`);
  }

  return { id, order, source, status };
}

function normalizeProse(content: string): string {
  return content.replace(/\r\n/g, "\n").trim();
}

/**
 * Parse and validate fragment source text: frontmatter metadata, Markdown
 * shape, link destinations, and normalized prose. The fragment body is
 * preserved as source bytes (modulo boundary whitespace/newline
 * normalization), never rendered.
 */
export function parseFragment(raw: string, displayPath: string): Fragment {
  if (!matter.test(raw)) {
    throw new Error(
      `${displayPath}: fragment must declare frontmatter metadata (id, order, source, status)`,
    );
  }

  let parsed: matter.GrayMatterFile<string>;
  try {
    parsed = matter(raw);
  } catch (error) {
    throw new Error(`${displayPath}: invalid frontmatter: ${errorMessage(error)}`, {
      cause: error,
    });
  }

  safeCommentText(displayPath, "fragment path", displayPath);
  const meta = fragmentMeta(parsed.data, displayPath);
  const prose = normalizeProse(parsed.content);
  validateProse(prose, displayPath);

  return {
    path: displayPath as ManifestRelativePath,
    meta,
    prose,
    proseBytes: Buffer.byteLength(prose, "utf8"),
  };
}

async function loadFragment(dir: string, fragmentPath: string): Promise<Fragment> {
  const absolute = resolve(dir, fragmentPath);
  let raw: string;
  try {
    raw = await readFile(absolute, "utf8");
  } catch (error) {
    throw new Error(`fragment not readable: ${fragmentPath} (${absolute})`, {
      cause: error,
    });
  }
  return parseFragment(raw, fragmentPath);
}

/** Sort by unique identity; reject duplicate ids and duplicate orders. */
function sortFragments(fragments: Fragment[]): Fragment[] {
  const byId = new Map<string, string>();
  const byOrder = new Map<number, string>();
  for (const fragment of fragments) {
    const priorId = byId.get(fragment.meta.id);
    if (priorId !== undefined) {
      throw new Error(
        `duplicate fragment id "${fragment.meta.id}" in ${fragment.path} and ${priorId}`,
      );
    }
    const priorOrder = byOrder.get(fragment.meta.order);
    if (priorOrder !== undefined) {
      throw new Error(
        `duplicate fragment order ${fragment.meta.order} in ${fragment.path} and ${priorOrder}`,
      );
    }
    byId.set(fragment.meta.id, fragment.path);
    byOrder.set(fragment.meta.order, fragment.path);
  }
  return fragments.sort((a, b) => a.meta.order - b.meta.order);
}

/**
 * Load every declared fragment. Duplicate manifest paths were already
 * rejected by readManifest; here each declared path is asserted to stay
 * inside the manifest directory, and the fragment reads run concurrently.
 */
async function loadDeclaredFragments(manifest: AssemblyManifest, dir: string): Promise<Fragment[]> {
  const pending = manifest.fragments.map((fragmentPath) => {
    const absolute = resolve(dir, fragmentPath);
    const contained = relative(dir, absolute);
    if (contained.startsWith("..") || isAbsolute(contained)) {
      throw new Error(
        `manifest fragment escapes the manifest directory: "${fragmentPath}" (${absolute})`,
      );
    }
    return loadFragment(dir, fragmentPath);
  });
  return sortFragments(await Promise.all(pending));
}

function resolveOutput(dir: string, declared: ManifestOutputPath, override?: string): AbsolutePath {
  return (override !== undefined ? resolve(override) : resolve(dir, declared)) as AbsolutePath;
}

function assertOutputDoesNotOverwriteInputs(
  output: AbsolutePath,
  manifestPath: string,
  manifest: AssemblyManifest,
  dir: string,
): void {
  const manifestAbsolute = resolve(manifestPath);
  if (output === manifestAbsolute) {
    throw new Error(`assembly output must not overwrite its manifest: ${output}`);
  }
  for (const fragmentPath of manifest.fragments) {
    if (output === resolve(dir, fragmentPath)) {
      throw new Error(`assembly output must not overwrite source fragment: ${fragmentPath}`);
    }
  }
}

/** Render the assembled document: generated marker, provenance, source prose. */
export function renderDocument(fragments: Fragment[]): string {
  const sections = fragments.map((fragment) => {
    const provenance = `<!-- rekon-fragment: id=${fragment.meta.id} order=${fragment.meta.order} path=${fragment.path} source=${fragment.meta.source} -->`;
    return `${provenance}\n\n${fragment.prose}`;
  });
  return `${[GENERATED_HEADER, ...sections].join("\n\n")}\n`;
}

/** Load, validate, and order all declared fragments; compute output bytes. */
export async function assemble(
  manifestPath: string,
  outputOverride?: string,
): Promise<AssemblyResult> {
  const manifest = await readManifest(manifestPath);
  const dir = dirname(resolve(manifestPath));
  const output = resolveOutput(dir, manifest.output, outputOverride);
  assertOutputDoesNotOverwriteInputs(output, manifestPath, manifest, dir);
  const fragments = await loadDeclaredFragments(manifest, dir);
  const content = renderDocument(fragments);
  return {
    fragments,
    content,
    totalBytes: Buffer.byteLength(content, "utf8"),
    output,
  };
}

/** Assemble and write output atomically (temporary file, then rename). */
export async function writeAssembly(
  manifestPath: string,
  outputOverride?: string,
): Promise<AssemblyResult> {
  const result = await assemble(manifestPath, outputOverride);
  await mkdir(dirname(result.output), { recursive: true });
  const temporary = `${result.output}.tmp-${process.pid}-${randomUUID().slice(0, 8)}`;
  await writeFile(temporary, result.content);
  await rename(temporary, result.output);
  return result;
}

function firstByteDifference(expected: Buffer, actual: Buffer): number {
  const shared = Math.min(expected.length, actual.length);
  for (let index = 0; index < shared; index++) {
    if (expected[index] !== actual[index]) return index;
  }
  return shared;
}

/**
 * Compare assembled bytes against the output without writing anything.
 * Only a missing output counts as missing/stale: permission, directory,
 * and other I/O failures surface.
 */
export async function checkAssembly(
  manifestPath: string,
  outputOverride?: string,
): Promise<CheckResult> {
  const result = await assemble(manifestPath, outputOverride);
  let actual: Buffer;
  try {
    actual = await readFile(result.output);
  } catch (error) {
    if (!isErrnoException(error, "ENOENT")) throw error;
    return { status: "stale", detail: `output missing: ${result.output}` };
  }
  const expected = Buffer.from(result.content, "utf8");
  if (expected.equals(actual)) {
    return { status: "fresh" };
  }
  const offset = firstByteDifference(expected, actual);
  return {
    status: "stale",
    detail: `output differs from assembly at byte ${offset} (expected ${expected.length} bytes, found ${actual.length}): ${result.output}`,
  };
}
