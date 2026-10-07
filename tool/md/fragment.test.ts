import { Effect, Result } from "effect";
import { describe, expect, test } from "vitest";
import { parseFragment, validateProse } from "./fragment.ts";

const metadata = { id: "example", order: 10, source: "/example/README.md", status: "stable" };

function source(body: string, fields: Record<string, unknown> = metadata): string {
  const frontmatter = Object.entries(fields).map(
    ([name, value]) => `${name}: ${JSON.stringify(value)}`,
  );
  return `---\n${frontmatter.join("\n")}\n---\n\n${body}\n`;
}

describe("assembly fragment parsing", () => {
  test("consumes metadata without re-rendering source prose or undercounting UTF-8", async () => {
    const prose = "# Título 😀\r\n\r\nKeep  *this* syntax.  \r\n\r\n- 日本語\r\n  - `nested`";
    const result = await Effect.runPromise(
      parseFragment(source(` \n${prose}\n\n `), "rules/GLOBAL.md"),
    );
    expect(result).toEqual({
      path: "rules/GLOBAL.md",
      meta: metadata,
      prose: prose.replace(/\r\n/g, "\n"),
      proseBytes: Buffer.byteLength(prose.replace(/\r\n/g, "\n"), "utf8"),
    });
    expect(result.proseBytes).toBeGreaterThan(result.prose.length);
  });

  test.each([
    ["missing", "# Example\n\nbody"],
    ["malformed", "---\nid: [unfinished\n---\n# Example\n\nbody"],
  ])("reports %s frontmatter with the fragment path", async (_name, raw) => {
    await expect(Effect.runPromise(parseFragment(raw, "broken.md"))).rejects.toMatchObject({
      _tag: "FragmentFrontmatterError",
      path: "broken.md",
    });
  });

  test.each([
    ["id", "Not-Kebab"],
    ["order", 1.5],
    ["source", "README.md"],
    ["source", "//host/README.md"],
    ["source", "/rules/../README.md"],
    ["source", "/rules/--injected.md"],
    ["source", "/rules/\nREADME.md"],
    ["status", "unsettled"],
  ])("rejects unsafe %s metadata (%j)", async (field, value) => {
    await expect(
      Effect.runPromise(
        parseFragment(source("# Example\n\nbody", { ...metadata, [field]: value }), "bad.md"),
      ),
    ).rejects.toMatchObject({ _tag: "FragmentMetadataError", path: "bad.md" });
  });

  test("requires every metadata field and validates the path used in provenance", async () => {
    const { order: _order, ...missingOrder } = metadata;
    await expect(
      Effect.runPromise(parseFragment(source("# Example\n\nbody", missingOrder), "bad.md")),
    ).rejects.toMatchObject({ _tag: "FragmentMetadataError" });
    await expect(
      Effect.runPromise(parseFragment(source("# Example\n\nbody"), "bad-->path.md")),
    ).rejects.toMatchObject({ _tag: "FragmentMetadataError" });
  });
});

describe("relocation-safe Markdown", () => {
  test.each([
    "",
    "body before heading\n\n# Late",
    "## Subheading\n\nbody",
    "# Heading only",
    "# First\n\nbody\n\n# Second",
  ])("rejects invalid heading/content structure: %j", (body) => {
    expect(validateProse(body, "shape.md")).toMatchObject({
      _tag: "Failure",
      failure: { _tag: "FragmentProseError" },
    });
  });

  test.each([
    "[link](neighbor.md)",
    "![image](images/pic.png)",
    "- nested **[link](../README.md)**",
    "| column |\n|---|\n| [table](table.md) |",
    "[used][reference]\n\n[reference]: local.md",
    "plain prose\n\n[unused]: unused.md",
    "[cdn](//host/image.png)",
  ])("rejects relocating destinations including nested/unused references: %j", (body) => {
    expect(validateProse(`# Example\n\n${body}`, "links.md")).toMatchObject({
      _tag: "Failure",
      failure: { _tag: "FragmentLinkError", path: "links.md" },
    });
  });

  test("allows portable destinations and leaves code samples alone", () => {
    const body = [
      "# Example",
      "",
      "[root](/README.md) [anchor](#section) [self]() [site](https://example.org) [mail](mailto:a@b.c) [file](file:///notes.md)",
      "",
      "`[literal](neighbor.md)`",
      "",
      "```md",
      "# Not another heading",
      "![literal](image.png)",
      "```",
    ].join("\n");
    expect(Result.isSuccess(validateProse(body, "portable.md"))).toBe(true);
  });
});
