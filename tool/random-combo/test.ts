import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { mkdtemp, rm, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { generate, parseOptions, seededRandom } from "./generate.ts";
import type { Event } from "./generate.ts";

const entrypoint = fileURLToPath(new URL("../random-combo.ts", import.meta.url));

interface Result {
  code: number;
  stdout: string;
  stderr: string;
}

function cli(args: string[], input = "", entry = entrypoint): Promise<Result> {
  return new Promise((resolve, reject) => {
    const child = execFile(
      process.execPath,
      [entry, ...args],
      { encoding: "utf8" },
      (error, stdout, stderr) => {
        if (error) {
          if (typeof error.code !== "number") return reject(error);
          resolve({ code: error.code, stdout, stderr });
        } else {
          resolve({ code: 0, stdout, stderr });
        }
      },
    );
    child.stdin!.end(input);
  });
}

function draws(events: Event[]): Extract<Event, { type: "draw" }>[] {
  return events.filter((event) => event.type === "draw");
}

test("parse inline weights anywhere; ignore blank lines/comments, normalize label spacing", () => {
  assert.deepEqual(
    parseOptions(
      "\ufeff# header\r\n\r\n first   choice w=1\r\nw=.1 second choice\r\nthird w=1e-2 choice\r\nnever w=0",
    ),
    [
      { id: 1, label: "first choice", weight: 1 },
      { id: 2, label: "second choice", weight: 0.1 },
      { id: 3, label: "third choice", weight: 0.01 },
      { id: 4, label: "never", weight: 0 },
    ],
  );
});

test("malformed weights and option lists report meaningful errors", () => {
  const cases: [string, RegExp][] = [
    ["", /No options/u],
    ["# only comment", /No options/u],
    ["x", /Line 1.*exactly one/u],
    ["x w=1 w=2", /exactly one/u],
    ["w=1", /label is empty/u],
    ["x w=", /finite non-negative/u],
    ["x w=-1", /finite non-negative/u],
    ["x w=NaN", /finite non-negative/u],
    ["x w=Infinity", /finite non-negative/u],
    ["x w=1e999", /finite non-negative/u],
    ["x w=0x10", /finite non-negative/u],
    ["x w=1oops", /finite non-negative/u],
    ["x w=1e-999", /too small/u],
    ["x w=0\ny w=0", /All weights are zero/u],
    ["x w=1\nx w=.1", /Line 2.*duplicate/u],
  ];
  for (const [text, expected] of cases) assert.throws(() => parseOptions(text), expected);
});

test("replacement permits triples; decay persists through every slot and across games", () => {
  const options = parseOptions("only w=1\nnever w=0");
  const original = structuredClone(options);
  const events = [...generate(options, 2, "test", () => 0.5)];
  const wins = draws(events);
  assert.equal(wins.length, 6);
  let weight = 1;
  for (const [index, draw] of wins.entries()) {
    assert.equal(draw.optionId, 1);
    assert.equal(draw.game, Math.floor(index / 3) + 1);
    assert.equal(draw.slot, (index % 3) + 1);
    assert.equal(draw.weightBefore, weight);
    weight *= 0.8;
    assert.equal(draw.weightAfter, weight);
  }
  assert.deepEqual(
    events
      .filter((event) => event.type === "game")
      .map((event) => event.picks.map((pick) => pick.label)),
    [
      ["only", "only", "only"],
      ["only", "only", "only"],
    ],
  );
  assert.deepEqual(options, original, "run does not mutate caller's options");
  const initial = events[0];
  assert.equal(initial.type, "initial");
  if (initial.type === "initial") assert.equal(initial.weights[0]!.weight, 1);
  const final = events.at(-1);
  assert.equal(final?.type, "final");
  if (final?.type === "final") {
    assert.equal(final.weights[0]!.weight, weight);
    assert.equal(final.weights[1]!.weight, 0);
  }
});

test("a repeated option is retained, and current rather than original weight affects the next draw", () => {
  // Initially A owns [0, .5). After A wins it owns [0, .8/1.8), so .47 picks B.
  const units = [0, 0.47, 0];
  const events = [...generate(parseOptions("A w=1\nB w=1"), 1, "test", () => units.shift()!)];
  assert.deepEqual(
    draws(events).map((draw) => draw.label),
    ["A", "B", "A"],
  );
  assert.equal(draws(events)[2]!.weightBefore, 0.8);
});

test("zero never wins, including u=0; exact interval boundary selects the next option", () => {
  const units = [0, 0.5, 1 - Number.EPSILON];
  const events = [
    ...generate(parseOptions("zero w=0\nA w=1\nB w=1"), 1, "test", () => units.shift()!),
  ];
  assert.deepEqual(
    draws(events).map((draw) => draw.label),
    ["A", "B", "B"],
  );
  const boundary = draws([...generate(parseOptions("A w=1\nB w=1"), 1, "test", () => 0.5)]);
  assert.equal(boundary[0]!.label, "B");
});

test("huge finite weights do not overflow their sampling total", () => {
  const events = [...generate(parseOptions("A w=1e308\nB w=1e308"), 1, "test", () => 0.75)];
  assert.equal(draws(events)[0]!.label, "B");
  assert.equal(draws(events)[0]!.scaledTotal, 2);
  assert.ok(draws(events).every((draw) => Number.isFinite(draw.weightAfter)));
});

test("reject unsafe games, invalid random output and numerical decay exhaustion", () => {
  const options = parseOptions("A w=1");
  for (const games of [0, -1, 1.5, NaN, Infinity, Number.MAX_SAFE_INTEGER]) {
    assert.throws(() => [...generate(options, games, "test")], /Games must/u);
  }
  for (const unit of [-1, 1, NaN, Infinity]) {
    assert.throws(() => [...generate(options, 1, "test", () => unit)], /Random source/u);
  }
  assert.throws(
    () => [...generate(parseOptions("A w=5e-324"), 1, "test")],
    /floating-point decay limits/u,
  );
});

test("seed algorithm has a fixed reference vector and runs are reproducible", () => {
  const random = seededRandom("abc");
  // Hard-coded PRNG reference values pin hashing, byte order, and integer operations.
  assert.deepEqual(
    Array.from({ length: 3 }, random),
    [0.7017354508861899, 0.5422114154789597, 0.26695797010324895],
  );
  const options = parseOptions("A w=1\nB w=.1");
  assert.deepEqual([...generate(options, 5, "same")], [...generate(options, 5, "same")]);
  assert.notDeepEqual(
    draws([...generate(options, 5, "same")]).map((draw) => draw.random),
    draws([...generate(options, 5, "different")]).map((draw) => draw.random),
  );
});

test("CLI accepts stdin, file and explicit -, produces replayable JSONL and text audit", async () => {
  const directory = await mkdtemp(join(tmpdir(), "rekon-random-combo-"));
  try {
    const file = join(directory, "options.txt");
    const input = "A w=1\nB w=.1\n";
    await writeFile(file, input);
    const args = ["--games", "2", "--seed", "demo", "--jsonl"];
    const [stdin, named, dash, text] = await Promise.all([
      cli(args, input),
      cli([...args, file]),
      cli([...args, "-"], input),
      cli(["--seed", "demo"], input),
    ]);
    assert.equal(stdin.code, 0, stdin.stderr);
    assert.deepEqual(named, stdin);
    assert.deepEqual(dash, stdin);
    const events = stdin.stdout
      .trim()
      .split("\n")
      .map((line) => JSON.parse(line) as Event);
    assert.equal(events[0]!.type, "initial");
    assert.equal(draws(events).length, 6);
    assert.equal(events.at(-1)!.type, "final");
    assert.match(text.stdout, /Initial weights:/u);
    assert.match(text.stdout, /Game 1, slot 3:.*weight .* -> /u);
    assert.match(text.stdout, /Updated weights:/u);
    assert.match(text.stdout, /Final weights:/u);
    const generated = await cli(["--jsonl"], input);
    assert.equal(generated.code, 0, generated.stderr);
    const seed = JSON.parse(generated.stdout.split("\n")[0]!).seed as string;
    assert.equal((await cli(["--jsonl", "--seed", seed], input)).stdout, generated.stdout);
  } finally {
    await rm(directory, { recursive: true });
  }
});

test("rows are ordered triples from the same persistent run, replayable from stderr", async () => {
  const input = 'first choice w=100\nsecond "choice" w=100\n';
  const directory = await mkdtemp(join(tmpdir(), "rekon-random-combo-rows-"));
  try {
    const file = join(directory, "options.txt");
    await writeFile(file, input);
    const args = ["--rows", "8", "--seed", "row test"];
    const [stdin, named, dash, audit] = await Promise.all([
      cli(args, input),
      cli([...args, file]),
      cli([...args, "-"], input),
      cli(["--games", "8", "--seed", "row test", "--jsonl"], input),
    ]);
    assert.equal(stdin.code, 0, stdin.stderr);
    assert.deepEqual(named, stdin);
    assert.deepEqual(dash, stdin);
    assert.match(stdin.stderr, /^Seed: "row test"\nAlgorithm: sha256-u32le-mulberry32-v1\n$/u);
    const rows = stdin.stdout
      .trimEnd()
      .split("\n")
      .map((line) => JSON.parse(line) as string[]);
    assert.equal(rows.length, 8);
    assert.ok(rows.every((row) => row.length === 3));
    const events = audit.stdout
      .trimEnd()
      .split("\n")
      .map((line) => JSON.parse(line) as Event);
    assert.deepEqual(
      rows,
      events
        .filter((event) => event.type === "game")
        .map((event) => event.picks.map((pick) => pick.label)),
    );
    assert.notDeepEqual(rows[1], rows[0], "rows do not restart the seeded run");
    const repeated = await cli(["--rows", "2", "--seed", "repeat"], "only option w=1\n");
    assert.deepEqual(
      repeated.stdout
        .trimEnd()
        .split("\n")
        .map((line) => JSON.parse(line)),
      [
        ["only option", "only option", "only option"],
        ["only option", "only option", "only option"],
      ],
    );
    const generated = await cli(["--rows", "8"], input);
    assert.equal(generated.code, 0, generated.stderr);
    const seed = JSON.parse(generated.stderr.split("\n")[0]!.slice("Seed: ".length)) as string;
    const replay = await cli(["--rows", "8", "--seed", seed], input);
    assert.equal(replay.stdout, generated.stdout);
    assert.equal(replay.stderr, generated.stderr);
  } finally {
    await rm(directory, { recursive: true });
  }
});

test("CLI reports invalid inputs on stderr with nonzero status and no success output", async () => {
  for (const [args, input, pattern] of [
    [["--nope"], "", /Unknown option/u],
    [["--games"], "", /argument missing/u],
    [["--games", "0"], "A w=1", /Games must/u],
    [["--games", "1.5"], "A w=1", /positive integer/u],
    [["--games", "1e3"], "A w=1", /positive integer/u],
    [["--rows"], "", /argument missing/u],
    [["--rows", "0"], "A w=1", /positive safe integer/u],
    [["--rows", "1.5"], "A w=1", /--rows requires/u],
    [["--rows", "9007199254740991"], "A w=1", /positive safe integer/u],
    [["--rows", "1", "--games", "1"], "A w=1", /cannot be combined/u],
    [["--rows", "1", "--jsonl"], "A w=1", /cannot be combined/u],
    [["a", "b"], "", /at most one/u],
    [["/does-not-exist/random-combo.txt"], "", /Cannot read input/u],
    [[], "bad w=-1", /Line 1.*finite non-negative/u],
    [[], "", /No options/u],
  ] as [string[], string, RegExp][]) {
    const result = await cli(args, input);
    assert.equal(result.code, 2, result.stderr);
    assert.equal(result.stdout, "");
    assert.match(result.stderr, pattern);
  }
});

test("help needs no input; entrypoint is import-safe and works through a symlink", async () => {
  const help = await cli(["--help"]);
  assert.equal(help.code, 0);
  assert.match(help.stdout, /WITH replacement/u);
  const imported = await cli(
    [
      "--input-type=module",
      "--eval",
      `await import(${JSON.stringify(new URL("../random-combo.ts", import.meta.url).href)});`,
    ],
    "",
    "--no-warnings",
  );
  assert.equal(imported.code, 0, imported.stderr);
  assert.equal(imported.stdout, "");
  const directory = await mkdtemp(join(tmpdir(), "rekon-random-combo-"));
  try {
    const alias = join(directory, "combo.ts");
    await symlink(entrypoint, alias);
    const linked = await cli(["--help"], "", alias);
    assert.deepEqual(linked, help);
  } finally {
    await rm(directory, { recursive: true });
  }
});
