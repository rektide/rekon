import { Effect } from "effect";
import { describe, expect, test } from "vitest";
import { compareDocument, GENERATED_HEADER, renderDocument } from "./document.ts";
import { parseFragment } from "./fragment.ts";

describe("assembly document bytes", () => {
  test("names the actual tool and emits precise provenance/source boundaries", async () => {
    const fragment = await Effect.runPromise(
      parseFragment(
        "---\nid: alpha\norder: 10\nsource: /README.md\nstatus: draft\n---\n\n# Alpha\n\nDo *not* re-render.\n",
        "rules/GLOBAL.md",
      ),
    );
    const expected =
      [
        GENERATED_HEADER,
        "<!-- rekon-fragment: id=alpha order=10 path=rules/GLOBAL.md source=/README.md -->",
        "# Alpha\n\nDo *not* re-render.",
      ].join("\n\n") + "\n";
    expect(renderDocument([fragment])).toBe(expected);
    expect(GENERATED_HEADER).toContain("`node tool/agents.ts --check`");
    expect(GENERATED_HEADER).not.toContain("rekon agents");
  });

  test("freshness is exact byte equality", () => {
    expect(compareDocument("日本語\n", Buffer.from("日本語\n"), "out.md")).toEqual({
      status: "fresh",
    });
  });

  test("reports true differing UTF-8 byte positions, not character positions", () => {
    const expected = "日本語😀 unchanged\n";
    const actual = Buffer.from("日本語😀 changed\n");
    expect(compareDocument(expected, actual, "out.md")).toEqual({
      status: "stale",
      detail: `output differs from assembly at byte 14 (expected ${Buffer.byteLength(expected)} bytes, found ${actual.length}): out.md`,
    });
  });

  test.each([Buffer.from("prefix"), Buffer.from("prefix\nextra")])(
    "detects unequal-length prefixes",
    (actual) => {
      expect(compareDocument("prefix\n", actual, "out.md")).toMatchObject({
        status: "stale",
        detail: expect.stringContaining(`at byte ${Math.min(actual.length, 7)}`),
      });
    },
  );
});
