#!/usr/bin/env node
/** Validate this evidence bundle's local/revision-bound references without network access. */
import { execFile } from "node:child_process";
import { createHash } from "node:crypto";
import { access, readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { lexer, walkTokens } from "marked";

const exec = promisify(execFile);
const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, "../../..");
const files = [
  "design/context-compilation/README.md",
  "design/context-compilation/review0.gpt6a.md",
  "design/context-compilation/review0/README.md",
  "design/context-compilation/review0/evidence0.gpt6a.md",
];
const blobs = new Map<string, Promise<string>>();
const commits = new Set<string>();
const visited = new Set<string>();
const problems: string[] = [];
function blob(id: string, path: string) {
  const key = `${id}:${path}`;
  let result = blobs.get(key);
  if (!result) {
    result = exec("git", ["show", key], { cwd: root }).then(({ stdout }) => stdout);
    blobs.set(key, result);
  }
  return result;
}
async function commit(id: string) {
  if (!/^[a-f0-9]{40}$/.test(id)) throw new Error(`not a full commit ID: ${id}`);
  if (commits.has(id)) return;
  await exec("git", ["cat-file", "-e", `${id}^{commit}`], { cwd: root });
  commits.add(id);
}
for (const path of files) {
  const markdown = await readFile(resolve(root, path), "utf8");
  const links: string[] = [];
  walkTokens(lexer(markdown), (token) => {
    if (token.type === "link") links.push(token.href);
  });
  for (const href of links) {
    if (visited.has(href)) continue;
    visited.add(href);
    try {
      if (href.startsWith("/")) {
        const [file, anchor] = href.slice(1).split("#");
        await access(resolve(root, file));
        if (anchor) {
          const text = await readFile(resolve(root, file), "utf8");
          if (!text.includes(`id="${anchor}"`))
            throw new Error(`missing explicit anchor ${anchor}`);
        }
      } else if (href.startsWith("https://github.com/rektide/rekon/")) {
        const url = new URL(href);
        const [, , , mode, coordinate, ...path] = url.pathname.split("/");
        if (mode === "compare") {
          for (const rev of coordinate.split("...")) await commit(rev);
        } else {
          await commit(coordinate);
          if (mode === "blob") {
            const text = await blob(coordinate, path.join("/"));
            if (url.hash) {
              const range = /^#L(\d+)(?:-L(\d+))?$/.exec(url.hash);
              if (!range) throw new Error(`unexpected blob fragment ${url.hash}`);
              const first = Number(range[1]),
                last = Number(range[2] ?? range[1]);
              if (first < 1 || last < first || last > text.trimEnd().split("\n").length)
                throw new Error(`invalid source line range ${url.hash}`);
            }
          }
        }
      }
    } catch (error) {
      problems.push(`${href}: ${String(error)}`);
    }
  }
}
const report = await readFile(resolve(root, "design/context-compilation/review0.gpt6a.md"), "utf8");
const cohorts = [...report.matchAll(/^\| (\d+) \| (\d+)–(\d+),/gm)];
let expected = 1;
for (const [_, row, first, last] of cohorts) {
  if (Number(first) !== expected)
    problems.push(`cohort ${row}: expected start ${expected}, found ${first}`);
  expected = Number(last) + 1;
}
if (cohorts.length !== 32 || expected !== 372)
  problems.push("cohorts must cover 1–371 in 32 contiguous rows");

const generated = [
  "metrics.json",
  "lineage.json",
  "evidence0.gpt6a.md",
  "diffs/legacy-cutover.diff",
  "diffs/cutover-recovery.diff",
  "diffs/recovery-later.diff",
];
const hash = async (file: string) =>
  createHash("sha256")
    .update(await readFile(resolve(here, file)))
    .digest("hex");
const before = await Promise.all(generated.map(hash));
await exec(process.execPath, [resolve(here, "measure.ts")], { cwd: root });
const after = await Promise.all(generated.map(hash));
for (let i = 0; i < before.length; i++)
  if (before[i] !== after[i]) problems.push(`nonreproducible generated artifact: ${generated[i]}`);
if (problems.length) {
  console.error(problems.join("\n"));
  process.exitCode = 1;
} else {
  console.log(
    `${visited.size} unique references checked locally; ${commits.size} commit objects and ${blobs.size} pinned file blobs resolve; 32 cohorts cover 371 lines; ${generated.length} artifacts reproduce byte-for-byte.`,
  );
}
