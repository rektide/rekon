import { randomBytes } from "node:crypto";
import { readFile } from "node:fs/promises";
import { parseArgs } from "node:util";
import { ALGORITHM, generate, parseOptions, RandomComboError, validateDraws, validateGames } from "./generate.ts";
import type { Event, Option } from "./generate.ts";

const HELP = `Usage: node tool/random-combo.ts [--games N | --rows N] [--slots N] [--seed TEXT] [--jsonl] [FILE|-]

Draw --slots options per game (default 3) WITH replacement. Every win
immediately multiplies that option's CURRENT weight by 0.8, persisting across
slots and games in this run.

Input: one option per line with exactly one inline w=WEIGHT token, e.g.:
  careful exploration w=1
  w=0.1 unexpected constraint
Blank lines and whole-line # comments are ignored. Zero weights never win;
weights must be finite non-negative decimals, with at least one positive weight.

FILE defaults to stdin (also spelled -); --games defaults to 1.
--slots N sets draws per game; it defaults to 3 and must be a positive integer.
--seed supplies an exact text seed; otherwise a random seed is generated/printed.
--jsonl emits one JSON event per line (initial, draw, game, final).
--rows N emits N JSON arrays of slot labels, one per stdout line; seed and
algorithm go to stderr. It cannot be combined with --games or --jsonl.
--help prints this help. No agents are spawned and no files are modified.
`;

function formatWeights(weights: Option[]): string {
  return weights
    .map(({ id, label, weight }) => `  [${id}] ${JSON.stringify(label)} w=${weight}`)
    .join("\n");
}

export function formatEvent(event: Event): string {
  switch (event.type) {
    case "initial":
      return `Seed: ${JSON.stringify(event.seed)}\nAlgorithm: ${event.algorithm}\nGames: ${event.games}; draws per game: ${event.drawsPerGame}; decay: ${event.decay}\nInitial weights:\n${formatWeights(event.weights)}`;
    case "draw":
      return `Game ${event.game}, slot ${event.slot}: [${event.optionId}] ${JSON.stringify(event.label)}; u=${event.random}; scale=${event.scale}; scaled total=${event.scaledTotal}; weight ${event.weightBefore} -> ${event.weightAfter}`;
    case "game":
      return `Game ${event.game} combo: ${event.picks.map(({ label }) => JSON.stringify(label)).join(" + ")}\nUpdated weights:\n${formatWeights(event.weights)}`;
    case "final":
      return `Final weights:\n${formatWeights(event.weights)}`;
  }
}

async function write(text: string): Promise<void> {
  // Wait for each write, including pipe backpressure, rather than buffering a whole run.
  await new Promise<void>((resolve, reject) => {
    process.stdout.write(text, (error) => (error ? reject(error) : resolve()));
  });
}

export async function main(args: string[]): Promise<void> {
  // A closed downstream pipe reports both a write callback error and a stream event.
  const onStdoutError = () => {};
  process.stdout.on("error", onStdoutError);
  try {
    let parsed;
    try {
      parsed = parseArgs({
        args,
        allowPositionals: true,
        strict: true,
        options: {
          games: { type: "string" },
          rows: { type: "string" },
          slots: { type: "string" },
          seed: { type: "string" },
          jsonl: { type: "boolean" },
          help: { type: "boolean", short: "h" },
        },
      });
    } catch (error) {
      // Argument parsing is a user-facing boundary, not an unexpected program error.
      if (!(error instanceof TypeError)) throw error;
      throw new RandomComboError(`${error.message}\nUse --help for usage.`, { cause: error });
    }
    if (parsed.values.help) {
      await write(HELP);
      return;
    }
    if (parsed.positionals.length > 1) {
      throw new RandomComboError(
        "Expected at most one input FILE (or - for stdin). Use --help for usage.",
      );
    }
    if (
      parsed.values.rows !== undefined &&
      (parsed.values.games !== undefined || parsed.values.jsonl)
    ) {
      throw new RandomComboError("--rows cannot be combined with --games or --jsonl.");
    }
    const literal = parsed.values.rows ?? parsed.values.games ?? "1";
    const flag = parsed.values.rows !== undefined ? "--rows" : "--games";
    if (!/^\d+$/u.test(literal))
      throw new RandomComboError(
        `${flag} requires a positive integer, e.g. --games 4.`,
      );
    const games = Number(literal);
    const slotsLiteral = parsed.values.slots ?? "3";
    if (!/^\d+$/u.test(slotsLiteral))
      throw new RandomComboError(`--slots requires a positive integer, e.g. --slots 3.`);
    const drawsPerGame = Number(slotsLiteral);
    validateDraws(drawsPerGame);
    validateGames(games, drawsPerGame);
    const file = parsed.positionals[0] ?? "-";
    if (file === "-" && process.stdin.isTTY) {
      throw new RandomComboError("No input: pass a FILE, pipe options to stdin, or use --help.");
    }
    let text: string;
    if (file === "-") {
      const chunks: Buffer[] = [];
      for await (const chunk of process.stdin) chunks.push(Buffer.from(chunk));
      text = Buffer.concat(chunks).toString("utf8");
    } else {
      try {
        text = await readFile(file, "utf8");
      } catch (error) {
        if (!(error instanceof Error)) throw error;
        throw new RandomComboError(`Cannot read input ${JSON.stringify(file)}: ${error.message}`, {
          cause: error,
        });
      }
    }
    const options = parseOptions(text);
    const seed = parsed.values.seed ?? randomBytes(16).toString("hex");
    if (parsed.values.rows !== undefined) {
      process.stderr.write(`Seed: ${JSON.stringify(seed)}\nAlgorithm: ${ALGORITHM}\n`);
    }
    for (const event of generate(options, games, seed, undefined, drawsPerGame)) {
      if (parsed.values.rows !== undefined) {
        if (event.type === "game")
          await write(`${JSON.stringify(event.picks.map((pick) => pick.label))}\n`);
      } else {
        await write(`${parsed.values.jsonl ? JSON.stringify(event) : formatEvent(event)}\n`);
      }
    }
  } catch (error) {
    if (error instanceof Error && "code" in error && error.code === "EPIPE") return;
    if (!(error instanceof RandomComboError)) throw error;
    process.stderr.write(`random-combo: ${error.message}\n`);
    process.exitCode = 2;
  } finally {
    process.stdout.off("error", onStdoutError);
  }
}
