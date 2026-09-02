import matter from "gray-matter";
import { randomUUID } from "node:crypto";
import { mkdir, readFile, rename, writeFile } from "node:fs/promises";
import { lexer } from "marked";
import { dirname, resolve } from "node:path";

/**
 * Default manifest path, resolved against the working directory.
 *
 * No production manifest exists yet: the current hand-maintained `/AGENTS.md`
 * must not be overwritten until every section it contains has a declared
 * `GLOBAL.md` source fragment.
 */
export const DEFAULT_MANIFEST = "doc/agents.manifest.json";

/** Identity and assembly metadata declared in a fragment's frontmatter. */
export interface FragmentMeta {
  /** Stable fragment identity; must be unique across a manifest. */
  id: string;
  /** Assembly position; unique integer across a manifest, ascending in output. */
  order: number;
  /** Canonical source README the fragment condenses. */
  source: string;
  /** Maturity of the fragment. */
  status: "draft" | "stable";
}

/** A declared `GLOBAL.md` fragment with its frontmatter stripped. */
export interface Fragment {
  /** Fragment path exactly as written in the manifest, relative to it. */
  path: string;
  meta: FragmentMeta;
  /** Fragment Markdown body; frontmatter omitted, newlines normalized. */
  prose: string;
}

/** Explicit declaration of which fragments assemble and where output lands. */
export interface AssemblyManifest {
  /** Fragment paths, relative to the manifest file's directory. */
  fragments: string[];
  /** Output path, relative to the manifest file's directory. */
  output: string;
}

export interface AssemblyResult {
  fragments: Fragment[];
  /** Assembled document bytes. */
  bytes: string;
  /** Absolute output path, with any override applied. */
  output: string;
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

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

/**
 * Read and shape-check a manifest. Paths are kept as written; resolution
 * against the manifest directory happens when fragments are loaded.
 */
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
  for (const entry of fragments) {
    if (typeof entry !== "string" || entry.trim() === "") {
      throw new Error(`manifest 'fragments' entries must be non-empty strings: ${manifestPath}`);
    }
  }

  if (typeof output !== "string" || output.trim() === "") {
    throw new Error(`manifest 'output' must be a non-empty path string: ${manifestPath}`);
  }

  return { fragments: fragments as string[], output };
}

/** Validate fragment Markdown: meaningful content led by a top-level heading. */
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
  if (meaningful.length < 2) {
    throw new Error(`${displayPath}: fragment needs content beyond its top-level heading`);
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

  const source = data.source;
  if (typeof source !== "string" || source.trim() === "") {
    throw new Error(
      `${displayPath}: frontmatter 'source' must name the fragment's canonical source README`,
    );
  }

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
 * shape, and normalized prose. The fragment body is preserved as source
 * bytes (modulo boundary whitespace/newline normalization), never rendered.
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

  const meta = fragmentMeta(parsed.data, displayPath);
  const prose = normalizeProse(parsed.content);
  validateProse(prose, displayPath);

  return { path: displayPath, meta, prose };
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

async function loadDeclaredFragments(manifest: AssemblyManifest, dir: string): Promise<Fragment[]> {
  const seen = new Map<string, string>();
  const fragments: Fragment[] = [];
  for (const fragmentPath of manifest.fragments) {
    const resolved = resolve(dir, fragmentPath);
    const prior = seen.get(resolved);
    if (prior !== undefined) {
      throw new Error(
        `manifest lists the same fragment twice: "${prior}" and "${fragmentPath}" both resolve to ${resolved}`,
      );
    }
    seen.set(resolved, fragmentPath);
    fragments.push(await loadFragment(dir, fragmentPath));
  }
  return sortFragments(fragments);
}

function resolveOutput(dir: string, declared: string, override?: string): string {
  return override !== undefined ? resolve(override) : resolve(dir, declared);
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
  const fragments = await loadDeclaredFragments(manifest, dir);
  return {
    fragments,
    bytes: renderDocument(fragments),
    output: resolveOutput(dir, manifest.output, outputOverride),
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
  await writeFile(temporary, result.bytes);
  await rename(temporary, result.output);
  return result;
}

function firstDifference(expected: string, actual: string): number {
  const shared = Math.min(expected.length, actual.length);
  for (let index = 0; index < shared; index++) {
    if (expected[index] !== actual[index]) return index;
  }
  return shared;
}

/** Compare assembled bytes against the output without writing anything. */
export async function checkAssembly(
  manifestPath: string,
  outputOverride?: string,
): Promise<CheckResult> {
  const result = await assemble(manifestPath, outputOverride);
  let actual: string;
  try {
    actual = await readFile(result.output, "utf8");
  } catch {
    return { status: "stale", detail: `output missing: ${result.output}` };
  }
  if (actual === result.bytes) {
    return { status: "fresh" };
  }
  const offset = firstDifference(result.bytes, actual);
  return {
    status: "stale",
    detail: `output differs from assembly at byte ${offset} (expected ${result.bytes.length} bytes, found ${actual.length}): ${result.output}`,
  };
}
