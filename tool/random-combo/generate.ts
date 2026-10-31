import { createHash } from "node:crypto";

export const DECAY = 0.8;
export const DRAWS_PER_GAME = 3;
export const ALGORITHM = "sha256-u32le-mulberry32-v1";

export class RandomComboError extends Error {}

export interface Option {
  id: number;
  label: string;
  weight: number;
}

export interface Pick {
  optionId: number;
  label: string;
}

export type Event =
  | {
      type: "initial";
      algorithm: string;
      seed: string;
      games: number;
      drawsPerGame: number;
      decay: number;
      weights: Option[];
    }
  | {
      type: "draw";
      game: number;
      slot: number;
      optionId: number;
      label: string;
      random: number;
      scale: number;
      scaledTotal: number;
      weightBefore: number;
      weightAfter: number;
    }
  | { type: "game"; game: number; picks: Pick[]; weights: Option[] }
  | { type: "final"; weights: Option[] };

/** One option per nonblank/noncomment line; exactly one whitespace-delimited w= token. */
export function parseOptions(text: string): Option[] {
  const options: Option[] = [];
  const labels = new Set<string>();
  for (const [index, raw] of text.split(/\r?\n/u).entries()) {
    const line = raw.trim();
    if (line === "" || line.startsWith("#")) continue;
    const tokens = line.split(/\s+/u);
    const weights = tokens.filter((token) => token.startsWith("w="));
    const at = `Line ${index + 1}`;
    if (weights.length !== 1) {
      throw new RandomComboError(`${at}: expected exactly one inline weight token, e.g. w=1.`);
    }
    const literal = weights[0]!.slice(2);
    const weight = Number(literal);
    if (!/^(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/u.test(literal) || !Number.isFinite(weight)) {
      throw new RandomComboError(
        `${at}: weight must be a finite non-negative decimal; got ${JSON.stringify(literal)}.`,
      );
    }
    if (weight === 0 && /[1-9]/u.test(literal.split(/[eE]/u)[0]!)) {
      throw new RandomComboError(
        `${at}: positive weight ${literal} is too small for a JavaScript number.`,
      );
    }
    const label = tokens.filter((token) => !token.startsWith("w=")).join(" ");
    if (label === "") throw new RandomComboError(`${at}: option label is empty.`);
    if (labels.has(label)) {
      throw new RandomComboError(`${at}: duplicate option label ${JSON.stringify(label)}.`);
    }
    labels.add(label);
    options.push({ id: options.length + 1, label, weight });
  }
  validateOptions(options);
  return options;
}

function validateOptions(options: readonly Option[]): void {
  if (options.length === 0)
    throw new RandomComboError("No options: supply one labeled option with w=WEIGHT per line.");
  const ids = new Set<number>();
  const labels = new Set<string>();
  for (const option of options) {
    if (!Number.isSafeInteger(option.id) || option.id < 1 || ids.has(option.id)) {
      throw new RandomComboError("Option IDs must be unique positive safe integers.");
    }
    if (!option.label.trim() || labels.has(option.label)) {
      throw new RandomComboError("Option labels must be nonempty and unique.");
    }
    if (!Number.isFinite(option.weight) || option.weight < 0) {
      throw new RandomComboError(`Option ${option.id}: weight must be finite and non-negative.`);
    }
    ids.add(option.id);
    labels.add(option.label);
  }
  if (!options.some((option) => option.weight > 0)) {
    throw new RandomComboError(
      "All weights are zero: at least one option must have a positive weight.",
    );
  }
}

/** SHA-256 UTF-8 seed -> first little-endian uint32 -> Mulberry32. Not cryptographic. */
export function seededRandom(seed: string): () => number {
  let state = createHash("sha256").update(seed, "utf8").digest().readUInt32LE(0);
  return () => {
    state = (state + 0x6d2b79f5) >>> 0;
    let value = Math.imul(state ^ (state >>> 15), state | 1);
    value ^= value + Math.imul(value ^ (value >>> 7), value | 61);
    return ((value ^ (value >>> 14)) >>> 0) / 0x100000000;
  };
}

export function validateGames(games: number): void {
  if (
    !Number.isSafeInteger(games) ||
    games < 1 ||
    games > Math.floor(Number.MAX_SAFE_INTEGER / DRAWS_PER_GAME)
  ) {
    throw new RandomComboError(
      "Games must be a positive safe integer with 3 × games also a safe integer.",
    );
  }
}

/** Lazily emit an audit trail. State is local to this run; caller input is never mutated. */
export function* generate(
  options: readonly Option[],
  games: number,
  seed: string,
  random: () => number = seededRandom(seed),
): Generator<Event> {
  validateOptions(options);
  validateGames(games);
  const current = options.map((option) => ({ ...option }));
  const snapshot = (): Option[] => current.map((option) => ({ ...option }));
  yield {
    type: "initial",
    algorithm: ALGORITHM,
    seed,
    games,
    drawsPerGame: DRAWS_PER_GAME,
    decay: DECAY,
    weights: snapshot(),
  };
  for (let game = 1; game <= games; game++) {
    const picks: Pick[] = [];
    for (let slot = 1; slot <= DRAWS_PER_GAME; slot++) {
      // Dividing by the maximum avoids overflow when finite input weights sum to Infinity.
      let scale = 0;
      for (const option of current) scale = Math.max(scale, option.weight);
      const scaled = current.map((option) => option.weight / scale);
      const scaledTotal = scaled.reduce((sum, weight) => sum + weight, 0);
      const unit = random();
      if (!Number.isFinite(unit) || unit < 0 || unit >= 1) {
        throw new RandomComboError("Random source must return a finite number in [0, 1).");
      }
      let remaining = unit * scaledTotal;
      let winner: Option | undefined;
      for (const [index, option] of current.entries()) {
        if (scaled[index]! <= 0) continue;
        // Retain last positive option as a fallback for rounded-up cumulative boundaries.
        winner = option;
        if (remaining < scaled[index]!) break;
        remaining -= scaled[index]!;
      }
      if (!winner)
        throw new RandomComboError(`Game ${game}, slot ${slot}: no selectable weight remains.`);
      const before = winner.weight;
      const after = before * DECAY;
      if (after === 0 || after === before) {
        throw new RandomComboError(
          `Game ${game}, slot ${slot}: weight for ${JSON.stringify(winner.label)} reached floating-point decay limits; shorten the run or increase initial weights.`,
        );
      }
      winner.weight = after;
      picks.push({ optionId: winner.id, label: winner.label });
      yield {
        type: "draw",
        game,
        slot,
        optionId: winner.id,
        label: winner.label,
        random: unit,
        scale,
        scaledTotal,
        weightBefore: before,
        weightAfter: after,
      };
    }
    yield { type: "game", game, picks, weights: snapshot() };
  }
  yield { type: "final", weights: snapshot() };
}
