import { Effect, Option } from "effect";
import { describe, expect, test } from "vitest";
import { parseManifest, planAssembly } from "./manifest.ts";

const declaration = { fragments: ["rules/GLOBAL.md"], output: "AGENTS.md" };

describe("explicit assembly declarations", () => {
  test("preserves declaration order and spellings without discovering extra files", async () => {
    const input = { ...declaration, fragments: ["second.md", "first.md"], unrelated: "ignored" };
    expect(await Effect.runPromise(parseManifest(JSON.stringify(input), "manifest.json"))).toEqual({
      fragments: ["second.md", "first.md"],
      output: "AGENTS.md",
    });
  });

  test.each([
    "{ invalid json",
    "null",
    "[]",
    JSON.stringify({ ...declaration, fragments: [] }),
    JSON.stringify({ ...declaration, fragments: [42] }),
    JSON.stringify({ fragments: declaration.fragments }),
    ...["", "/absolute.md", "./out.md", "out//AGENTS.md"].map((output) =>
      JSON.stringify({ ...declaration, output }),
    ),
  ])("rejects malformed manifest data: %s", async (raw) => {
    await expect(Effect.runPromise(parseManifest(raw, "broken.json"))).rejects.toMatchObject({
      _tag: "ManifestValidationError",
      path: "broken.json",
    });
  });

  test.each([
    "",
    "/etc/hostname",
    "../outside.md",
    "frag/../inside.md",
    "./inside.md",
    "frag//inside.md",
    "evil-->name.md",
    "control\nname.md",
  ])("rejects unsafe declaration path: %j", async (path) => {
    await expect(
      Effect.runPromise(
        parseManifest(JSON.stringify({ ...declaration, fragments: [path] }), "bad.json"),
      ),
    ).rejects.toMatchObject({ _tag: "ManifestValidationError" });
  });
});

describe("assembly path planning without I/O", () => {
  test("keeps inputs manifest-relative, deliberate parent output and override cwd-relative", async () => {
    const manifest = await Effect.runPromise(
      parseManifest(
        JSON.stringify({
          fragments: ["local.md", "nested/GLOBAL.md"],
          output: "../output/AGENTS.md",
        }),
        "bundle/manifest.json",
      ),
    );
    const plain = await Effect.runPromise(
      planAssembly(manifest, "bundle/manifest.json", "/workspace"),
    );
    expect(plain).toEqual({
      manifestPath: "/workspace/bundle/manifest.json",
      fragments: [
        { path: "local.md", absolute: "/workspace/bundle/local.md" },
        { path: "nested/GLOBAL.md", absolute: "/workspace/bundle/nested/GLOBAL.md" },
      ],
      output: "/workspace/output/AGENTS.md",
    });
    const override = await Effect.runPromise(
      planAssembly(manifest, "bundle/manifest.json", "/workspace", Option.some("preview.md")),
    );
    expect(override.output).toBe("/workspace/preview.md");
    expect(override.fragments).toEqual(plain.fragments);
    const absoluteOverride = await Effect.runPromise(
      planAssembly(
        manifest,
        "bundle/manifest.json",
        "/workspace",
        Option.some("/preview/AGENTS.md"),
      ),
    );
    expect(absoluteOverride.output).toBe("/preview/AGENTS.md");
  });

  test("directory names beginning with two dots are not traversal segments", async () => {
    const manifest = await Effect.runPromise(
      parseManifest(
        JSON.stringify({ ...declaration, fragments: ["..notes/GLOBAL.md"] }),
        "manifest.json",
      ),
    );
    const plan = await Effect.runPromise(planAssembly(manifest, "manifest.json", "/workspace"));
    expect(plan.fragments[0].absolute).toBe("/workspace/..notes/GLOBAL.md");
  });

  test("rejects duplicate resolved fragment paths", async () => {
    const manifest = await Effect.runPromise(
      parseManifest(
        JSON.stringify({ ...declaration, fragments: ["rules/GLOBAL.md", "rules/GLOBAL.md"] }),
        "manifest.json",
      ),
    );
    await expect(
      Effect.runPromise(planAssembly(manifest, "manifest.json", "/workspace")),
    ).rejects.toMatchObject({
      _tag: "DuplicateFragmentPathError",
      fragment: "rules/GLOBAL.md",
      prior: "rules/GLOBAL.md",
    });
  });

  test.each(["manifest.json", "rules/GLOBAL.md"])(
    "rejects declared or override output overwriting %s",
    async (output) => {
      const manifest = await Effect.runPromise(
        parseManifest(JSON.stringify({ ...declaration, output }), "manifest.json"),
      );
      await expect(
        Effect.runPromise(planAssembly(manifest, "manifest.json", "/workspace")),
      ).rejects.toMatchObject({ _tag: "OutputOverwritesInputError" });
      const ordinary = await Effect.runPromise(
        parseManifest(JSON.stringify(declaration), "manifest.json"),
      );
      await expect(
        Effect.runPromise(
          planAssembly(ordinary, "manifest.json", "/workspace", Option.some(output)),
        ),
      ).rejects.toMatchObject({ _tag: "OutputOverwritesInputError" });
    },
  );
});
