import type { Fragment } from "./fragment.ts";

export const GENERATED_HEADER = [
  "<!-- GENERATED FILE - DO NOT EDIT",
  "Assembled by `node tool/agents.ts` from declared GLOBAL.md fragments.",
  "Edit the fragment sources and re-run `node tool/agents.ts`;",
  "`node tool/agents.ts --check` detects hand edits to this file.",
  "-->",
].join("\n");

/** Render assembler output from validated, ordered fragments without round-tripping their prose. */
export function renderDocument(fragments: ReadonlyArray<Fragment>): string {
  const sections = fragments.map(({ path, meta, prose }) => {
    const provenance = `<!-- rekon-fragment: id=${meta.id} order=${meta.order} path=${path} source=${meta.source} -->`;
    return `${provenance}\n\n${prose}`;
  });
  return `${[GENERATED_HEADER, ...sections].join("\n\n")}\n`;
}

export type DocumentCheck =
  | { readonly status: "fresh" }
  | { readonly status: "stale"; readonly detail: string };

/** Compare UTF-8 bytes, including unequal-length prefixes, not UTF-16 string indices. */
export function compareDocument(
  content: string,
  actual: Uint8Array,
  output: string,
): DocumentCheck {
  const expected = Buffer.from(content, "utf8");
  const shared = Math.min(expected.length, actual.length);
  let offset = 0;
  while (offset < shared && expected[offset] === actual[offset]) offset++;
  if (offset === expected.length && offset === actual.length) return { status: "fresh" };
  return {
    status: "stale",
    detail: `output differs from assembly at byte ${offset} (expected ${expected.length} bytes, found ${actual.length}): ${output}`,
  };
}
