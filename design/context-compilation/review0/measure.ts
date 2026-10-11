#!/usr/bin/env node
/** Reproduce this report's evidence from immutable revisions, never the live working tree. */
import { execFile } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import matter from "gray-matter";
import { lexer } from "marked";

const exec = promisify(execFile);
const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, "../../..");
const repo = "https://github.com/rektide/rekon";
const revisions = {
  legacy: "ab672848a82170a5575dc32c227a5056008e3f37",
  cutover: "ed2b366b71e67e9e6eec6ec1fd3170645aca9f2a",
  recovery: "00cc45a247f26b7598864cd76a2b621f0927b107",
  audit: "3e99a5ad102531dbfaa98b572eb09f80fd592242",
  later: "fe78712b42beec706e5f08eb05657cdc577d025f",
};
type Stage = keyof typeof revisions;
type Part = { heading: string; text: string; first: number; last: number };
const cache = new Map<string, Promise<string>>();
function show(rev: string, path: string): Promise<string> {
  const key = `${rev}:${path}`;
  let pending = cache.get(key);
  if (!pending) {
    pending = exec("git", ["show", key], { cwd: root, maxBuffer: 8 * 1024 * 1024 }).then(
      ({ stdout }) => stdout,
    );
    cache.set(key, pending);
  }
  return pending;
}
function count(text: string) {
  const trimmed = text.trim();
  return {
    words: trimmed ? trimmed.split(/\s+/u).length : 0,
    bytes: Buffer.byteLength(text),
    lines: text.trimEnd().split("\n").length,
    sha256: createHash("sha256").update(text).digest("hex"),
  };
}
function sections(raw: string): Part[] {
  const result: Part[] = [];
  let offset = 0;
  let part: Part | undefined;
  for (const token of lexer(raw)) {
    if (token.type === "heading" && token.depth === 1) {
      part = {
        heading: token.text,
        text: "",
        first: raw.slice(0, offset).split("\n").length,
        last: 0,
      };
      result.push(part);
    }
    if (part) part.text += token.raw;
    offset += token.raw.length;
  }
  if (offset !== raw.length) throw new Error("Markdown section extraction lost input bytes");
  for (const part of result) part.last = part.first + part.text.trimEnd().split("\n").length - 1;
  return result;
}
function fragments(raw: string) {
  const pattern =
    /<!-- rekon-fragment: id=([^ ]+) order=(\d+) path=([^ ]+) source=([^ ]+) -->\n\n([\s\S]*?)(?=\n\n<!-- rekon-fragment:|$)/g;
  return [...raw.matchAll(pattern)].map((m) => ({
    id: m[1],
    order: Number(m[2]),
    path: m[3],
    source: m[4],
    text: m[5].trim(),
    first: raw.slice(0, m.index! + m[0].indexOf(m[5])).split("\n").length,
  }));
}
function link(rev: string, path: string, first?: number, last?: number): string {
  return `${repo}/blob/${rev}/${path}${first ? `#L${first}${last ? `-L${last}` : ""}` : ""}`;
}

const legacy = await show(revisions.legacy, "AGENTS.md");
const oldParts = sections(legacy);
const preserved = await show(revisions.recovery, "AGENTS.old.md");
if (legacy !== preserved)
  throw new Error("Pre-cutover manuscript differs from preserved legacy baseline");
const { stdout: blame } = await exec(
  "git",
  ["blame", "--line-porcelain", revisions.legacy, "--", "AGENTS.md"],
  { cwd: root, maxBuffer: 8 * 1024 * 1024 },
);
const lineage: {
  commit: string;
  originalLine: number;
  line: number;
  subject: string;
  text: string;
}[] = [];
let origin: (typeof lineage)[number] | undefined;
for (const line of blame.split("\n")) {
  const header = /^([a-f0-9]{40}) (\d+) (\d+)(?: \d+)?$/.exec(line);
  if (header)
    origin = {
      commit: header[1],
      originalLine: Number(header[2]),
      line: Number(header[3]),
      subject: "",
      text: "",
    };
  else if (origin && line.startsWith("summary ")) origin.subject = line.slice(8);
  else if (origin && line.startsWith("\t")) {
    origin.text = line.slice(1);
    lineage.push(origin);
    origin = undefined;
  }
}
if (lineage.length !== legacy.trimEnd().split("\n").length)
  throw new Error("Lineage must cover every baseline line");
const generated = new Map<Stage, Awaited<ReturnType<typeof fragments>>>();
const stages: Record<string, unknown> = {};
for (const [name, revision] of Object.entries(revisions)) {
  const raw = await show(revision, "AGENTS.md");
  const parts = fragments(raw);
  generated.set(name as Stage, parts);
  if (name === "legacy") {
    stages[name] = {
      revision,
      ...count(raw),
      sections: oldParts.map(({ text, ...p }) => ({ ...p, ...count(text) })),
    };
    continue;
  }
  const manifest = JSON.parse(await show(revision, "agents.manifest.json"));
  const sourceChecks = await Promise.all(
    parts.map(async (p) => {
      const file = await show(revision, p.path);
      const body = matter(file).content.replace(/\r\n/g, "\n").trim();
      return {
        path: p.path,
        selected: manifest.fragments.includes(p.path),
        matchesSource: body === p.text,
        assembled: count(p.text),
        source: count(body),
      };
    }),
  );
  stages[name] = {
    revision,
    ...count(raw),
    fragmentWords: parts.reduce((sum, p) => sum + count(p.text).words, 0),
    fragmentBytes: parts.reduce((sum, p) => sum + count(p.text).bytes, 0),
    fragments: parts.map(({ text, ...p }) => ({ ...p, ...count(text) })),
    sourceChecks,
  };
}

const groups = [
  {
    id: "collaboration",
    old: ["subagents"],
    now: ["agents/GLOBAL.md"],
    caveat: "New fragment also receives discussion preference from the old documentation section.",
  },
  {
    id: "documentation",
    old: ["Documentation / reference materials"],
    now: ["constitution/practice/GLOBAL.md", "constitution/GLOBAL.md"],
    caveat:
      "Operational practice and broader constitution are different layers; discussion moved to collaboration.",
  },
  {
    id: "workspace",
    old: ["Experimenting", "well known directories and files", "shell usage"],
    now: ["code/workspace/GLOBAL.md"],
    caveat: "Three legacy headings now share one owner.",
  },
  {
    id: "code-jj",
    old: ["Project Planning", "Coding advice", "jj / git", "jj-hunk"],
    now: ["code/GLOBAL.md", "vcs/jj/GLOBAL.md"],
    caveat: "Grouped to avoid counting transfers of commit/hunk rules as new content.",
  },
  {
    id: "javascript",
    old: ["Rules and guidelines for JavaScript projects", "webcomponents"],
    now: ["js/GLOBAL.md"],
    caveat: "Examples and some conditions need comparison with the linked JS README.",
  },
  {
    id: "rust",
    old: ["Rust instructions"],
    now: ["code/rust/GLOBAL.md"],
    caveat: "Configuration instructions were revised, not just compressed.",
  },
  {
    id: "beads",
    old: ["beads"],
    now: ["praxis/beads/GLOBAL.md"],
    caveat:
      "Ticket pointers are reported separately; uncommon commands moved to the new beads README.",
  },
  {
    id: "glossary",
    old: [],
    now: ["CONTEXT.md"],
    caveat:
      "Additional standalone exposure with no equivalent legacy section; concepts may overlap other prose.",
  },
  {
    id: "ticket-pointers",
    old: [],
    now: ["praxis/ticket-pointers/GLOBAL.md"],
    caveat:
      "Additional exposure, not all-new ideas; the old beads section already required append-notes.",
  },
  {
    id: "references",
    old: [],
    now: ["refs/GLOBAL.md"],
    caveat: "Additional curated skills/source-discovery exposure.",
  },
];
const recoveryParts = generated.get("recovery")!;
const groupMetrics = groups.map((g) => {
  const before = g.old.map(
    (heading) =>
      oldParts.find((p) => p.heading === heading) ??
      (() => {
        throw new Error(heading);
      })(),
  );
  const after = g.now.map(
    (path) =>
      recoveryParts.find((p) => p.path === path) ??
      (() => {
        throw new Error(path);
      })(),
  );
  const oldWords = before.reduce((n, p) => n + count(p.text).words, 0);
  const newWords = after.reduce((n, p) => n + count(p.text).words, 0);
  return {
    ...g,
    oldWords,
    newWords,
    deltaWords: newWords - oldWords,
    changePercent: oldWords ? Math.round((newWords / oldWords - 1) * 1000) / 10 : null,
  };
});

await mkdir(join(here, "diffs"), { recursive: true });
const comparisonFiles = [
  "AGENTS.md",
  "agents.manifest.json",
  "CONTEXT.md",
  "constitution/GLOBAL.md",
  "constitution/practice/GLOBAL.md",
  "agents/GLOBAL.md",
  "code/GLOBAL.md",
  "code/workspace/GLOBAL.md",
  "vcs/jj/GLOBAL.md",
  "js/GLOBAL.md",
  "code/rust/GLOBAL.md",
  "praxis/beads",
  "praxis/ticket-pointers/GLOBAL.md",
  "refs/GLOBAL.md",
];
const transitions: [Stage, Stage][] = [
  ["legacy", "cutover"],
  ["cutover", "recovery"],
  ["recovery", "later"],
];
for (const [from, to] of transitions) {
  const { stdout } = await exec(
    "git",
    [
      "diff",
      "--no-ext-diff",
      "--no-color",
      revisions[from],
      revisions[to],
      "--",
      ...comparisonFiles,
    ],
    { cwd: root, maxBuffer: 8 * 1024 * 1024 },
  );
  await writeFile(join(here, "diffs", `${from}-${to}.diff`), stdout);
}

const evidence: string[] = [
  "---",
  "type: Evidence",
  "title: Paired ambient guidance excerpts",
  "status: draft",
  "generated: { by: 'model:openai/gpt-6-astra', at: 2026-10-11 }",
  "sources:",
  "  - { resource: 'urn:opencode:session:ses_f908bad7fffeW0pr4FLFa4VFyn', title: Revision-bound measurement and source-pair generation for the decomposition review }",
  `  - { resource: '${repo}/commit/${revisions.legacy}', title: Legacy manuscript }`,
  `  - { resource: '${repo}/commit/${revisions.recovery}', title: Recovered ambient fragments }`,
  "---",
  "",
  "# Paired ambient guidance excerpts",
  "",
  "Generated by [`measure.ts`](/design/context-compilation/review0/measure.ts). Read the [review](/design/context-compilation/review0.gpt6a.md) for interpretation. These are source pairs, not claims of semantic equivalence. Word counts include Markdown syntax and code examples, with whitespace tokenization. Source headings inside fences are not counted as sections.",
  "",
];
for (const g of groupMetrics) {
  evidence.push(
    `<a id="${g.id}"></a>`,
    `## ${g.id}`,
    "",
    `${g.oldWords} → ${g.newWords} words. ${g.caveat}`,
    "",
  );
  for (const heading of g.old) {
    const p = oldParts.find((p) => p.heading === heading)!;
    evidence.push(
      `### Before: [${heading}](${link(revisions.legacy, "AGENTS.md", p.first, p.last)})`,
      "",
      "````markdown",
      p.text.trimEnd(),
      "````",
      "",
    );
  }
  for (const path of g.now) {
    const raw = await show(revisions.recovery, path);
    const p = recoveryParts.find((p) => p.path === path)!;
    const start = raw.slice(0, raw.indexOf(p.text)).split("\n").length;
    if (!raw.includes(p.text)) throw new Error(`Fragment body not found: ${path}`);
    evidence.push(
      `### After: [${path}](${link(revisions.recovery, path, start, start + p.text.split("\n").length - 1)})`,
      "",
      "````markdown",
      p.text,
      "````",
      "",
    );
  }
}
const allOld = groups.flatMap((g) => g.old);
if (new Set(allOld).size !== oldParts.length || allOld.length !== oldParts.length)
  throw new Error("Domain mapping must account for each legacy H1 exactly once");
const allNew = groups.flatMap((g) => g.now);
if (new Set(allNew).size !== recoveryParts.length || allNew.length !== recoveryParts.length)
  throw new Error("Domain mapping must account for each recovery fragment exactly once");
const measures = {
  method:
    "Whitespace words in raw Markdown, code included; UTF-8 byte counts; per-section raw bytes versus trimmed fragment bodies. Not model-token counts. Sources read only via full commit IDs.",
  revisions,
  legacyEqualsPreserved: true,
  stages,
  groups: groupMetrics,
};
await writeFile(join(here, "metrics.json"), `${JSON.stringify(measures, null, 2)}\n`);
await writeFile(
  join(here, "lineage.json"),
  `${JSON.stringify({ revision: revisions.legacy, path: "AGENTS.md", lines: lineage }, null, 2)}\n`,
);
await writeFile(join(here, "evidence0.gpt6a.md"), `${evidence.join("\n")}\n`);
console.log(
  JSON.stringify(
    {
      groups: groupMetrics,
      stages: Object.fromEntries(
        Object.entries(stages).map(([key, value]) => {
          const v = value as {
            words: number;
            bytes: number;
            lines: number;
            fragmentWords?: number;
            sourceChecks?: { path: string; matchesSource: boolean }[];
          };
          return [
            key,
            {
              words: v.words,
              bytes: v.bytes,
              lines: v.lines,
              fragmentWords: v.fragmentWords,
              sourceDrift: v.sourceChecks?.filter((c) => !c.matchesSource).map((c) => c.path),
            },
          ];
        }),
      ),
    },
    null,
    2,
  ),
);
