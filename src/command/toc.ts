#!/usr/bin/env node
// toc — line-numbered section ToCs in managed marker regions.
//
// REFERENCE: TS reference port, not the production ToC. The end-state is a
// rust `rekon toc` subcommand following `combine`
// (design/toolchain/draft0.glm53m.md addendum 2026-10-08; beads
// rekon-tools-toc-adopt / rekon-tools-toc-replace).
// Lineage: ~/ado/bin/hyalo-toc.ts, design/rekon/hyalo-toc0.glm53m.md.
// EXTERNAL DEPENDENCY: the `hyalo` binary, >= 0.24.1, at
// /usr/local/bin/hyalo (not an npm dependency).
//
// Data source: `hyalo find --fields sections,lines` (always current — never a
// stale cache). Each ToC entry carries its line RANGE hidden in an HTML
// comment so LLMs can jump and budget precisely:
//
//   hyalo read --file patches.md --lines 663:2013
//
// while humans see a clean list in the rendered page. The region between the
// markers is regenerated wholesale; prose outside the markers is untouched.
// Idempotent; deterministic; same discipline as `hyalo okf index`.
//
// FIXED POINT: the ToC's own insertion shifts the headings it indexes, so
// each file regenerates from its post-write state until the embedded numbers
// stabilize (one entry per listed heading keeps the block size constant, so
// pass 2 converges; capped at 3).
//
// Usage:
//   node rekon.ts toc patches.md patches-log.md plugins.md   # regenerate
//   node rekon.ts toc --check <files>                        # exit 1 on drift
//
// Convention: entries are level <= 2 headings; a range ends where the next
// listed heading begins. Line numbers are FILE-relative for files without
// YAML frontmatter; for frontmatter files they are body-relative, matching
// `hyalo read --lines`.

import { execFile } from "node:child_process";
import { readFile, writeFile, realpath } from "node:fs/promises";
import { pathToFileURL } from "node:url";
import { define, cli } from "gunshi";

const BEGIN_MARK = "<!-- hyalo-toc:begin";
const BEGIN =
  "<!-- hyalo-toc:begin (regenerate: node rekon.ts toc; ranges are line numbers: hyalo read --file <this-file> --lines START:END) -->";
const END = "<!-- hyalo-toc:end -->";
const MAX_PASSES = 3;

interface Section {
  heading: string;
  level: number;
  line: number;
}

interface FindResult {
  sections: Section[];
  lines: number;
}

const run = (args: string[]) =>
  new Promise<string>((resolve, reject) =>
    execFile("hyalo", args, { maxBuffer: 16 * 1024 * 1024 }, (error, stdout) =>
      error ? reject(error) : resolve(stdout),
    ),
  );

async function tocBlock(file: string) {
  const out = await run(["find", "--file", file, "--fields", "sections,lines", "--format", "json"]);
  const result = (JSON.parse(out) as { results: FindResult[] }).results[0];
  const listed = result.sections.filter((s) => s.level <= 2);
  const entries = listed.map((section, i) => ({
    heading: section.heading,
    start: section.line,
    stop: i + 1 < listed.length ? listed[i + 1].line - 1 : result.lines,
  }));
  return entries.map((e) => `- ${e.heading} <!-- L${e.start}-${e.stop} -->`).join("\n");
}

const tocFor = async (file: string) => `${BEGIN}\n${await tocBlock(file)}\n${END}`;

function spliceRegion(content: string, generated: string) {
  const begin = content.indexOf(BEGIN_MARK);
  if (begin !== -1) {
    const end = content.indexOf(END, begin);
    if (end !== -1) return content.slice(0, begin) + generated + content.slice(end + END.length);
  }
  // no region yet: insert after the first H1 (or at top when there is none)
  const h1 = /^#[^\n]*$/m.exec(content);
  if (!h1) return `${generated}\n\n${content}`;
  const at = h1.index + h1[0].length;
  return `${content.slice(0, at)}\n\n${generated}\n${content.slice(at)}`;
}

function extractRegion(content: string) {
  const begin = content.indexOf(BEGIN_MARK);
  if (begin === -1) return "";
  const end = content.indexOf(END, begin);
  return end === -1 ? "" : content.slice(begin, end + END.length);
}

async function processFile(file: string, check: boolean) {
  for (let pass = 0; pass < MAX_PASSES; pass++) {
    const generated = await tocFor(file);
    const content = await readFile(file, "utf8");
    if (extractRegion(content) === generated) return false;
    if (check) return true;
    await writeFile(file, spliceRegion(content, generated));
  }
  return false;
}

export default define({
  name: "toc",
  description:
    "Maintain line-numbered section ToCs in managed marker regions (TS reference port; rust rekon toc to follow combine; requires hyalo >= 0.24.1)",
  args: {
    files: {
      type: "positional",
      multiple: true,
      description: "Markdown files to regenerate the ToC in",
    },
    check: {
      type: "boolean",
      description: "Exit 1 when any file's ToC has drifted from its regenerated form",
    },
  },
  run: async (ctx) => {
    const files = ctx.values.files;
    const check = ctx.values.check;
    if (files.length === 0) {
      console.error("usage: rekon.ts toc [--check] <file.md>...");
      process.exit(2);
    }

    let drift = false;
    for (const file of files) {
      if (await processFile(file, check)) drift = true;
      console.log(`toc: ${file}`);
    }
    process.exitCode = drift ? 1 : 0;
  },
});

void (async () => {
  const mainPath = await realpath(process.argv[1]);
  const mainUrl = pathToFileURL(mainPath).href;
  if (import.meta.url === mainUrl) {
    const module = await import("./toc.ts");
    await cli(process.argv.slice(2), module.default, { name: "toc" });
  }
})();
