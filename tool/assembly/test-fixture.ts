import { Option } from "effect";
import { randomUUID } from "node:crypto";
import { mkdir, readFile, readdir, rm, stat, writeFile } from "node:fs/promises";
import { dirname, join, relative, resolve } from "node:path";
import { afterEach } from "vitest";

const roots: string[] = [];
afterEach(async () => {
  await Promise.all(roots.splice(0).map((root) => rm(root, { recursive: true, force: true })));
});

export function fragmentSource(
  id = "example",
  order = 10,
  body = "# Example\n\nExample prose.",
): string {
  return `---\nid: ${id}\norder: ${order}\nsource: /${id}/README.md\nstatus: stable\n---\n\n${body}\n`;
}

export async function createBundle(
  options: {
    readonly fragments?: ReadonlyArray<{ readonly path: string; readonly raw: string }>;
    readonly output?: string;
    readonly manifest?: string;
  } = {},
) {
  const root = resolve(".test-agent", "agents-effect", randomUUID());
  roots.push(root);
  const manifest = join(root, options.manifest ?? "bundle/manifest.json");
  await mkdir(dirname(manifest), { recursive: true });
  const fragments = options.fragments ?? [{ path: "rules/GLOBAL.md", raw: fragmentSource() }];
  await Promise.all(
    fragments.map(async ({ path, raw }) => {
      const absolute = join(dirname(manifest), path);
      await mkdir(dirname(absolute), { recursive: true });
      await writeFile(absolute, raw);
    }),
  );
  const output = options.output ?? "out/AGENTS.md";
  await writeFile(
    manifest,
    JSON.stringify({ fragments: fragments.map(({ path }) => path), output }),
  );
  return {
    root,
    manifest,
    output: resolve(dirname(manifest), output),
    request: {
      manifestPath: relative(root, manifest),
      cwd: root,
      outputOverride: Option.none<string>(),
    },
  };
}

/** Exclude read-induced atime; detect any create/remove/write or directory-mtime change. */
export async function snapshotTree(root: string): Promise<ReadonlyArray<unknown>> {
  const information = await stat(root);
  const entries = await readdir(root, { withFileTypes: true });
  const children = await Promise.all(
    entries
      .sort((a, b) => a.name.localeCompare(b.name))
      .map(async (entry) => {
        const path = join(root, entry.name);
        if (entry.isDirectory()) return [entry.name, await snapshotTree(path)];
        const file = await stat(path);
        return [entry.name, file.mtimeMs, (await readFile(path)).toString("hex")];
      }),
  );
  return [information.mtimeMs, ...children];
}
