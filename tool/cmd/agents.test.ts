import { spawn } from "node:child_process";
import { mkdir, readFile, stat, symlink, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { describe, expect, test } from "vitest";
import { createBundle, fragmentSource, snapshotTree } from "../assembly/test-fixture.ts";

const entrypoint = resolve("tool/agents.ts");

interface CommandResult {
  readonly code: number | null;
  readonly stdout: string;
  readonly stderr: string;
}

function runNode(args: ReadonlyArray<string>, cwd = process.cwd()): Promise<CommandResult> {
  return new Promise((resolveResult, reject) => {
    const child = spawn(process.execPath, args, { cwd, stdio: ["ignore", "pipe", "pipe"] });
    let stdout = "";
    let stderr = "";
    child.stdout.setEncoding("utf8").on("data", (chunk: string) => {
      stdout += chunk;
    });
    child.stderr.setEncoding("utf8").on("data", (chunk: string) => {
      stderr += chunk;
    });
    child.on("error", reject);
    child.on("close", (code) => resolveResult({ code, stdout, stderr }));
  });
}

const runCli = (args: ReadonlyArray<string>, cwd?: string) => runNode([entrypoint, ...args], cwd);

describe("source-executed Effect agents CLI", { timeout: 30_000 }, () => {
  test("write/fresh/stale modes expose real exit codes, ordered provenance and UTF-8 cost reports", async () => {
    const first = "# First 😀\n\n日本語";
    const bundle = await createBundle({
      fragments: [
        { path: "later.md", raw: fragmentSource("later", 200, "# Later\n\nsecond body") },
        { path: "first.md", raw: fragmentSource("first", 100, first) },
      ],
    });
    const write = await runCli(["--manifest", bundle.manifest], bundle.root);
    expect(write.code).toBe(0);
    expect(write.stderr).toBe("");
    expect(write.stdout).toContain(`assembled 2 fragment(s) into ${bundle.output}`);
    expect(write.stdout).toContain(`100\tfirst\tfirst.md\t(${Buffer.byteLength(first)} bytes)`);
    expect(write.stdout.indexOf("first.md")).toBeLessThan(write.stdout.indexOf("later.md"));
    const bytes = await readFile(bundle.output);
    expect(write.stdout).toContain(`(${bytes.length} bytes)`);
    expect(bytes.toString()).toContain("`node tool/agents.ts --check`");
    const freshSnapshot = await snapshotTree(bundle.root);
    const fresh = await runCli(["-m", bundle.manifest, "-c"], bundle.root);
    expect(fresh.code).toBe(0);
    expect(fresh.stdout).toContain("fresh: output matches assembly");
    expect(await snapshotTree(bundle.root)).toEqual(freshSnapshot);
    await writeFile(bundle.output, "# Hand edit\n");
    const staleSnapshot = await snapshotTree(bundle.root);
    const stale = await runCli(["-m", bundle.manifest, "--check"], bundle.root);
    expect(stale.code).toBe(1);
    expect(stale.stdout + stale.stderr).toContain("stale: output differs from assembly at byte 0");
    expect(await snapshotTree(bundle.root)).toEqual(staleSnapshot);
  });

  test("missing-output check exits one without creating anything", async () => {
    const bundle = await createBundle({ output: "never/created/AGENTS.md" });
    const before = await snapshotTree(bundle.root);
    const result = await runCli(["--manifest", bundle.manifest, "--check"], bundle.root);
    expect(result.code).toBe(1);
    expect(result.stdout + result.stderr).toContain(`stale: output missing: ${bundle.output}`);
    expect(await snapshotTree(bundle.root)).toEqual(before);
  });

  test("relative override uses invocation cwd; manifest output may deliberately target its parent", async () => {
    const bundle = await createBundle({ output: "../AGENTS.parent.md" });
    const parent = await runCli(["-m", "bundle/manifest.json"], bundle.root);
    expect(parent.code).toBe(0);
    expect(await readFile(bundle.output, "utf8")).toContain("# Example");
    const override = await runCli(
      ["-m", "bundle/manifest.json", "-o", "preview/AGENTS.md"],
      bundle.root,
    );
    expect(override.code).toBe(0);
    expect(await readFile(join(bundle.root, "preview/AGENTS.md"))).toEqual(
      await readFile(bundle.output),
    );
    const checked = await runCli(
      ["-m", "bundle/manifest.json", "-o", "preview/AGENTS.md", "-c"],
      bundle.root,
    );
    expect(checked.code).toBe(0);
  });

  test("default manifest works when explicitly supplied by the invocation workspace", async () => {
    const bundle = await createBundle({ manifest: "constitution/agents.manifest.json" });
    const result = await runCli([], bundle.root);
    expect(result.code).toBe(0);
    expect(await readFile(bundle.output, "utf8")).toContain("# Example");
    const missing = await createBundle();
    const before = await snapshotTree(missing.root);
    const absent = await runCli([], missing.root);
    expect(absent.code).toBe(1);
    expect(absent.stdout + absent.stderr).toContain("constitution/agents.manifest.json");
    expect(await snapshotTree(missing.root)).toEqual(before);
  });

  test("help succeeds, unknown arguments fail, and neither mode assembles", async () => {
    const bundle = await createBundle();
    const before = await snapshotTree(bundle.root);
    const help = await runCli(["--help"], bundle.root);
    expect(help.code).toBe(0);
    expect(help.stdout).toContain("--manifest, -m");
    expect(help.stdout).toContain("--output, -o");
    expect(help.stdout).toContain("--check, -c");
    const invalid = await runCli(["--unrecognized"], bundle.root);
    expect(invalid.code).toBe(1);
    expect(invalid.stdout + invalid.stderr).toContain("unrecognized");
    expect(await snapshotTree(bundle.root)).toEqual(before);
  });

  test.each([
    ["manifest", "{not JSON", "invalid manifest"],
    ["fragment", "# No frontmatter\n\nbody", "rules/GLOBAL.md"],
    [
      "fragment",
      fragmentSource("example", 10, "# Broken\n\n[relative](local.md)"),
      "fragment-relative",
    ],
  ])(
    "%s errors retain actionable context and cannot create output",
    async (kind, text, expected) => {
      const bundle = await createBundle();
      await writeFile(
        kind === "manifest" ? bundle.manifest : join(dirname(bundle.manifest), "rules/GLOBAL.md"),
        text,
      );
      const before = await snapshotTree(bundle.root);
      const result = await runCli(["-m", bundle.manifest], bundle.root);
      expect(result.code).toBe(1);
      expect(result.stdout + result.stderr).toContain(expected);
      expect(await snapshotTree(bundle.root)).toEqual(before);
    },
  );

  test("refuses to overwrite a selected input via an output override", async () => {
    const bundle = await createBundle();
    const before = await snapshotTree(bundle.root);
    const result = await runCli(
      ["-m", bundle.manifest, "-o", "bundle/rules/GLOBAL.md"],
      bundle.root,
    );
    expect(result.code).toBe(1);
    expect(result.stdout + result.stderr).toContain("must not overwrite source fragment");
    expect(await snapshotTree(bundle.root)).toEqual(before);
  });

  test("nonmissing output read failures are reported as I/O failures, not stale", async () => {
    const bundle = await createBundle();
    await mkdir(bundle.output, { recursive: true });
    const before = await snapshotTree(bundle.root);
    const result = await runCli(["-m", bundle.manifest, "-c"], bundle.root);
    expect(result.code).toBe(1);
    expect(result.stdout + result.stderr).toMatch(/EISDIR|illegal operation/);
    expect(result.stdout + result.stderr).not.toContain("stale:");
    expect(await snapshotTree(bundle.root)).toEqual(before);
  });

  test("imports do not parse arguments or assemble, including absent argv[1]", async () => {
    const bundle = await createBundle();
    const before = await snapshotTree(bundle.root);
    const url = pathToFileURL(entrypoint).href;
    const absent = await runNode(
      ["--input-type=module", "-e", `await import(${JSON.stringify(url)});`],
      bundle.root,
    );
    expect(absent).toEqual({ code: 0, stdout: "", stderr: "" });
    expect(await snapshotTree(bundle.root)).toEqual(before);
    const importer = join(bundle.root, "importer.ts");
    await writeFile(importer, `await import(${JSON.stringify(url)});\n`);
    const withImporter = await snapshotTree(bundle.root);
    const present = await runNode([importer, "--manifest", bundle.manifest], bundle.root);
    expect(present).toEqual({ code: 0, stdout: "", stderr: "" });
    await expect(stat(bundle.output)).rejects.toMatchObject({ code: "ENOENT" });
    expect(await snapshotTree(bundle.root)).toEqual(withImporter);
  });

  test("symlink launch resolves its main file and executes normally", async () => {
    const bundle = await createBundle();
    const link = join(bundle.root, "agents-link.ts");
    await symlink(entrypoint, link);
    const result = await runNode([link, "-m", bundle.manifest], bundle.root);
    expect(result.code).toBe(0);
    expect(result.stdout).toContain("assembled 1 fragment(s)");
    expect(await readFile(bundle.output, "utf8")).toContain("# Example");
  });
});
