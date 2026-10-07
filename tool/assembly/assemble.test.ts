import { NodeFileSystem } from "@effect/platform-node";
import { Deferred, Effect, Fiber, FileSystem, Option, PlatformError } from "effect";
import { mkdir, readFile, readdir, rm, stat, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { describe, expect, test } from "vitest";
import { assemble, checkAssembly, FRAGMENT_READ_CONCURRENCY, writeAssembly } from "./assemble.ts";
import { createBundle, fragmentSource, snapshotTree } from "./test-fixture.ts";

describe("declared filesystem assembly", () => {
  test("loads only declared fragments, sorts unique metadata, and preserves prose/provenance/byte costs", async () => {
    const prose = "# First 😀\r\n\r\nKeep  *this* syntax.  \r\n\r\n日本語";
    const bundle = await createBundle({
      fragments: [
        { path: "second/GLOBAL.md", raw: fragmentSource("second", 200, "# Second\n\nsecond body") },
        { path: "first/GLOBAL.md", raw: fragmentSource("first", 100, prose) },
      ],
    });
    await writeFile(
      join(dirname(bundle.manifest), "undeclared.md"),
      "This would be an invalid fragment.",
    );
    const before = await snapshotTree(bundle.root);
    const result = await Effect.runPromise(
      assemble(bundle.request).pipe(Effect.provide(NodeFileSystem.layer)),
    );
    expect(result.fragments.map(({ meta }) => meta.id)).toEqual(["first", "second"]);
    expect(result.fragments[0].prose).toBe(prose.replace(/\r\n/g, "\n"));
    expect(result.fragments[0].proseBytes).toBe(Buffer.byteLength(result.fragments[0].prose));
    expect(result.content).toContain(
      "<!-- rekon-fragment: id=first order=100 path=first/GLOBAL.md source=/first/README.md -->",
    );
    expect(result.content.indexOf("# First")).toBeLessThan(result.content.indexOf("# Second"));
    expect(result.totalBytes).toBe(Buffer.byteLength(result.content));
    expect(await snapshotTree(bundle.root)).toEqual(before);
    const again = await Effect.runPromise(
      assemble(bundle.request).pipe(Effect.provide(NodeFileSystem.layer)),
    );
    expect(again.content).toBe(result.content);
  });

  test.each([
    ["id", fragmentSource("first", 200), "DuplicateFragmentIdError"],
    ["order", fragmentSource("second", 100), "DuplicateFragmentOrderError"],
    ["fragment", "# Missing metadata\n\nbody", "FragmentFrontmatterError"],
  ])("invalid %s prevents all output mutations", async (_kind, raw, tag) => {
    const bundle = await createBundle({
      fragments: [
        { path: "first.md", raw: fragmentSource("first", 100) },
        { path: "second.md", raw },
      ],
      output: "never/created/AGENTS.md",
    });
    const before = await snapshotTree(bundle.root);
    await expect(
      Effect.runPromise(writeAssembly(bundle.request).pipe(Effect.provide(NodeFileSystem.layer))),
    ).rejects.toMatchObject({ _tag: tag });
    expect(await snapshotTree(bundle.root)).toEqual(before);
  });

  test("keeps missing input failures as contextual platform errors, not staleness", async () => {
    const bundle = await createBundle();
    const missingFragment = join(dirname(bundle.manifest), "rules/GLOBAL.md");
    await rm(missingFragment);
    const before = await snapshotTree(bundle.root);
    await expect(
      Effect.runPromise(checkAssembly(bundle.request).pipe(Effect.provide(NodeFileSystem.layer))),
    ).rejects.toMatchObject({
      _tag: "PlatformError",
      reason: { _tag: "NotFound", pathOrDescriptor: missingFragment },
    });
    await rm(bundle.manifest);
    await expect(
      Effect.runPromise(assemble(bundle.request).pipe(Effect.provide(NodeFileSystem.layer))),
    ).rejects.toMatchObject({
      _tag: "PlatformError",
      reason: { pathOrDescriptor: bundle.manifest },
    });
    expect(before).not.toEqual(await snapshotTree(bundle.root)); // only the deliberate manifest removal changed it
  });

  test("bounds concurrent reads while keeping results deterministic despite different completion order", async () => {
    const entries = Array.from({ length: 19 }, (_, index) => `f${index}.md`);
    let active = 0;
    let peak = 0;
    const layer = FileSystem.layerNoop({
      readFileString: (path) => {
        if (path.endsWith("manifest.json"))
          return Effect.succeed(JSON.stringify({ fragments: entries, output: "AGENTS.md" }));
        const index = entries.findIndex((entry) => path.endsWith(`/${entry}`));
        return Effect.gen(function* () {
          active++;
          peak = Math.max(peak, active);
          yield* Effect.sleep(`${3 - (index % 3)} millis`);
          active--;
          return fragmentSource(`fragment-${index}`, entries.length - index);
        });
      },
    });
    const result = await Effect.runPromise(
      assemble({
        manifestPath: "manifest.json",
        cwd: "/bundle",
        outputOverride: Option.none(),
      }).pipe(Effect.provide(layer)),
    );
    expect(peak).toBe(FRAGMENT_READ_CONCURRENCY);
    expect(active).toBe(0);
    expect(result.fragments.map(({ path }) => path)).toEqual(entries.toReversed());
  });
});

describe("write/check ownership", () => {
  test("atomically creates declared parent output; fresh/stale checks change no files", async () => {
    const bundle = await createBundle({ output: "../generated/AGENTS.md" });
    const result = await Effect.runPromise(
      writeAssembly(bundle.request).pipe(Effect.provide(NodeFileSystem.layer)),
    );
    expect(await readFile(bundle.output, "utf8")).toBe(result.content);
    expect((await stat(bundle.output)).size).toBe(result.totalBytes);
    expect(await readdir(dirname(bundle.output))).toEqual(["AGENTS.md"]);
    const freshSnapshot = await snapshotTree(bundle.root);
    expect(
      await Effect.runPromise(
        checkAssembly(bundle.request).pipe(Effect.provide(NodeFileSystem.layer)),
      ),
    ).toEqual({ status: "fresh" });
    expect(await snapshotTree(bundle.root)).toEqual(freshSnapshot);
    await writeFile(bundle.output, "# Manual edit\n");
    const staleSnapshot = await snapshotTree(bundle.root);
    expect(
      await Effect.runPromise(
        checkAssembly(bundle.request).pipe(Effect.provide(NodeFileSystem.layer)),
      ),
    ).toMatchObject({ status: "stale", detail: expect.stringContaining("at byte 0") });
    expect(await snapshotTree(bundle.root)).toEqual(staleSnapshot);
  });

  test("missing output check does not even create its parent directory", async () => {
    const bundle = await createBundle({ output: "missing/deeper/AGENTS.md" });
    const before = await snapshotTree(bundle.root);
    expect(
      await Effect.runPromise(
        checkAssembly(bundle.request).pipe(Effect.provide(NodeFileSystem.layer)),
      ),
    ).toEqual({ status: "stale", detail: `output missing: ${bundle.output}` });
    expect(await snapshotTree(bundle.root)).toEqual(before);
  });

  test("overrides remain cwd-relative and do not write the manifest output", async () => {
    const bundle = await createBundle();
    const request = { ...bundle.request, outputOverride: Option.some("preview/AGENTS.md") };
    const result = await Effect.runPromise(
      writeAssembly(request).pipe(Effect.provide(NodeFileSystem.layer)),
    );
    expect(result.output).toBe(join(bundle.root, "preview/AGENTS.md"));
    await expect(stat(bundle.output)).rejects.toMatchObject({ code: "ENOENT" });
    expect(
      await Effect.runPromise(checkAssembly(request).pipe(Effect.provide(NodeFileSystem.layer))),
    ).toEqual({ status: "fresh" });
  });

  test("permission failures are not missing-output staleness", async () => {
    const bundle = await createBundle();
    const failure = PlatformError.systemError({
      _tag: "PermissionDenied",
      module: "FileSystem",
      method: "readFile",
      pathOrDescriptor: bundle.output,
    });
    const result = Effect.gen(function* () {
      const fs = yield* FileSystem.FileSystem;
      return yield* checkAssembly(bundle.request).pipe(
        Effect.provideService(FileSystem.FileSystem, {
          ...fs,
          readFile: () => Effect.fail(failure),
        }),
      );
    }).pipe(Effect.provide(NodeFileSystem.layer));
    await expect(Effect.runPromise(result)).rejects.toBe(failure);
  });

  test("failed publication leaves existing output intact and removes only its staging directory", async () => {
    const bundle = await createBundle();
    await mkdir(bundle.output, { recursive: true });
    await writeFile(join(bundle.output, "keep.md"), "keep me");
    await expect(
      Effect.runPromise(writeAssembly(bundle.request).pipe(Effect.provide(NodeFileSystem.layer))),
    ).rejects.toMatchObject({ _tag: "PlatformError" });
    expect(await readFile(join(bundle.output, "keep.md"), "utf8")).toBe("keep me");
    expect(await readdir(dirname(bundle.output))).toEqual(["AGENTS.md"]);
    await expect(
      Effect.runPromise(checkAssembly(bundle.request).pipe(Effect.provide(NodeFileSystem.layer))),
    ).rejects.toMatchObject({ _tag: "PlatformError" });
  });

  test("interruption after staging preserves prior output and waits for staging cleanup", async () => {
    const bundle = await createBundle();
    await mkdir(dirname(bundle.output), { recursive: true });
    await writeFile(bundle.output, "prior output");
    const stagedFile = await Effect.runPromise(
      Effect.gen(function* () {
        const fs = yield* FileSystem.FileSystem;
        const staged = yield* Deferred.make<string>();
        const fiber = yield* Effect.forkChild(
          writeAssembly(bundle.request).pipe(
            Effect.provideService(FileSystem.FileSystem, {
              ...fs,
              writeFileString: (path, data) =>
                fs
                  .writeFileString(path, data)
                  .pipe(
                    Effect.andThen(Deferred.succeed(staged, path)),
                    Effect.andThen(Effect.never),
                  ),
            }),
          ),
        );
        const path = yield* Deferred.await(staged);
        yield* Fiber.interrupt(fiber);
        return path;
      }).pipe(Effect.provide(NodeFileSystem.layer)),
    );
    expect(await readFile(bundle.output, "utf8")).toBe("prior output");
    await expect(stat(dirname(stagedFile))).rejects.toMatchObject({ code: "ENOENT" });
    expect(await readdir(dirname(bundle.output))).toEqual(["AGENTS.md"]);
  });
});
