import { execFile as execFileCallback } from "node:child_process";
import { randomUUID } from "node:crypto";
import { mkdir, readFile, rm, stat, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { promisify } from "node:util";
import { afterEach, describe, expect, test } from "vitest";
import {
  assemble,
  checkAssembly,
  parseFragment,
  readManifest,
  renderDocument,
  validateProse,
  writeAssembly,
} from "./assemble.ts";

const execFile = promisify(execFileCallback);

const SCRATCH_ROOT = join(".test-agent", "assemble-test");

const created: string[] = [];

async function scratch(): Promise<string> {
  const dir = join(SCRATCH_ROOT, randomUUID());
  await mkdir(dir, { recursive: true });
  created.push(dir);
  return dir;
}

afterEach(async () => {
  while (created.length > 0) {
    const dir = created.pop();
    if (dir !== undefined) await rm(dir, { recursive: true, force: true });
  }
});

async function writeFragment(
  root: string,
  rel: string,
  meta: Record<string, unknown>,
  body: string,
): Promise<string> {
  const path = join(root, rel);
  await mkdir(dirname(path), { recursive: true });
  const lines = Object.entries(meta).map(([key, value]) => `${key}: ${value}`);
  await writeFile(path, `---\n${lines.join("\n")}\n---\n\n${body}\n`);
  return rel;
}

async function writeManifest(
  root: string,
  fragments: string[],
  output: string,
  rel = "manifest.json",
): Promise<string> {
  const path = join(root, rel);
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify({ fragments, output }, null, 2)}\n`);
  return path;
}

const BASE_META = {
  id: "example",
  order: 100,
  source: "/doc/example/README.md",
  status: "stable",
};

describe("assemble", () => {
  test("orders fragments by metadata order, not manifest order", async () => {
    const root = await scratch();
    await writeFragment(
      root,
      "frag/a.md",
      { ...BASE_META, id: "second", order: 200 },
      "# Second\n\nsecond prose",
    );
    await writeFragment(
      root,
      "frag/b.md",
      { ...BASE_META, id: "first", order: 100 },
      "# First\n\nfirst prose",
    );
    const manifestPath = await writeManifest(root, ["frag/a.md", "frag/b.md"], "out/AGENTS.md");

    const result = await assemble(manifestPath);
    expect(result.fragments.map((fragment) => fragment.meta.id)).toEqual(["first", "second"]);
    expect(result.bytes.indexOf("# First")).toBeGreaterThan(-1);
    expect(result.bytes.indexOf("# First")).toBeLessThan(result.bytes.indexOf("# Second"));
  });

  test("produces identical bytes for identical inputs", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", BASE_META, "# Example\n\nsome prose");
    const manifestPath = await writeManifest(root, ["a.md"], "out.md");

    const first = await assemble(manifestPath);
    const second = await assemble(manifestPath);
    expect(first.bytes).toBe(second.bytes);
  });

  test("normalizes CRLF and trailing whitespace so boundaries are stable", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", BASE_META, "# Example\r\n\r\nwindows prose\r\n");
    const manifestPath = await writeManifest(root, ["a.md"], "out.md");

    const windows = await assemble(manifestPath);

    await writeFile(
      join(root, "a.md"),
      "---\nid: example\norder: 100\nsource: /doc/example/README.md\nstatus: stable\n---\n\n# Example\n\nwindows prose\n",
    );
    const unix = await assemble(manifestPath);

    expect(windows.bytes).toBe(unix.bytes);
    expect(windows.bytes).not.toContain("\r");
    expect(windows.bytes.endsWith("\n")).toBe(true);
    expect(windows.bytes.endsWith("\n\n")).toBe(false);
  });

  test("strips frontmatter and records provenance while preserving prose", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", BASE_META, "# Example\n\nsome prose");
    const manifestPath = await writeManifest(root, ["a.md"], "out.md");

    const result = await assemble(manifestPath);
    expect(result.bytes).not.toContain("order: 100");
    expect(result.bytes).not.toContain("source: /doc/example/README.md\n");
    expect(result.bytes).toContain(
      "<!-- rekon-fragment: id=example order=100 path=a.md source=/doc/example/README.md -->",
    );
    expect(result.bytes).toContain("# Example\n\nsome prose");
    expect(result.bytes).toContain("<!-- GENERATED FILE - DO NOT EDIT");
  });

  test("applies output override while fragment paths stay manifest-relative", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", BASE_META, "# Example\n\nsome prose");
    const manifestPath = await writeManifest(root, ["a.md"], "from-manifest.md");

    const result = await assemble(manifestPath, join(root, "override.md"));
    expect(result.output).toBe(resolve(join(root, "override.md")));

    const plain = await assemble(manifestPath);
    expect(plain.output).toBe(resolve(join(root, "from-manifest.md")));
  });

  test("loads fragments concurrently without changing results", async () => {
    const root = await scratch();
    const ids = ["a", "b", "c", "d", "e", "f", "g", "h"];
    for (const [index, id] of ids.entries()) {
      await writeFragment(
        root,
        `frag/${id}.md`,
        { ...BASE_META, id, order: 100 + index * 10 },
        `# ${id}\n\nprose ${id}`,
      );
    }
    const manifestPath = await writeManifest(
      root,
      ids.map((id) => `frag/${id}.md`),
      "out.md",
    );

    const result = await assemble(manifestPath);
    expect(result.fragments.map((fragment) => fragment.meta.id)).toEqual(ids);
  });
});

describe("context cost in UTF-8 bytes", () => {
  test("counts per-fragment prose bytes, not JS string length", async () => {
    const root = await scratch();
    const body = "# Título 😀\n\n日本語のテキスト and ASCII";
    await writeFragment(root, "a.md", BASE_META, body);
    const manifestPath = await writeManifest(root, ["a.md"], "out.md");

    const result = await assemble(manifestPath);
    const prose = body.replace(/\r\n/g, "\n").trim();
    expect(result.fragments[0].proseBytes).toBe(Buffer.byteLength(prose, "utf8"));
    expect(result.fragments[0].proseBytes).not.toBe(prose.length);
    expect(result.totalBytes).toBe(Buffer.byteLength(result.bytes, "utf8"));
    expect(result.totalBytes).toBeGreaterThan(result.fragments[0].proseBytes);
  });

  test("totalBytes matches the written file size", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", BASE_META, "# Emoji 😀 prose\n\n日本語");
    const manifestPath = await writeManifest(root, ["a.md"], "out/AGENTS.md");

    const result = await writeAssembly(manifestPath);
    const written = await readFile(join(root, "out", "AGENTS.md"));
    expect(written.length).toBe(result.totalBytes);
  });
});

describe("manifest failures", () => {
  test("rejects duplicate fragment paths that resolve to the same file", async () => {
    const root = await scratch();
    await writeFragment(root, "frag/a.md", BASE_META, "# Example\n\nprose");
    const manifestPath = await writeManifest(root, ["frag/a.md", "frag/../frag/a.md"], "out.md");

    await expect(assemble(manifestPath)).rejects.toThrow(
      /must be normalized paths without '\.', '\.\.', or redundant separators/,
    );
  });

  test("rejects absolute fragment entries", async () => {
    const root = await scratch();
    const manifestPath = await writeManifest(root, ["/etc/hostname"], "out.md");

    await expect(assemble(manifestPath)).rejects.toThrow(/must be relative paths/);
  });

  test("rejects fragment entries that escape the manifest directory", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", BASE_META, "# Example\n\nprose");
    const manifestPath = await writeManifest(root, ["../a.md"], "out.md");

    await expect(assemble(manifestPath)).rejects.toThrow(/escapes the manifest directory/);
  });

  test("rejects fragment paths that would break provenance comments", async () => {
    const root = await scratch();
    const manifestPath = await writeManifest(root, ["evil-->name.md"], "out.md");

    await expect(assemble(manifestPath)).rejects.toThrow(/must not contain "-->"/);
  });

  test("rejects missing fragment files", async () => {
    const root = await scratch();
    const manifestPath = await writeManifest(root, ["absent.md"], "out.md");

    await expect(assemble(manifestPath)).rejects.toThrow(/fragment not readable/);
  });

  test("rejects empty fragment lists and missing output", async () => {
    const root = await scratch();
    const empty = await writeManifest(root, [], "out.md");
    await expect(assemble(empty)).rejects.toThrow(/non-empty array/);

    await writeFragment(root, "a.md", BASE_META, "# Example\n\nprose");
    const noOutput = await writeManifest(root, ["a.md"], "");
    await expect(assemble(noOutput)).rejects.toThrow(/'output' must be a non-empty path/);
  });

  test("rejects missing and malformed manifests", async () => {
    const root = await scratch();
    await expect(assemble(join(root, "absent.json"))).rejects.toThrow(/manifest not readable/);

    const bad = join(root, "bad.json");
    await writeFile(bad, "{ not json");
    await expect(assemble(bad)).rejects.toThrow(/not valid JSON/);
  });

  test("resolves fragment paths relative to the manifest directory", async () => {
    const root = await scratch();
    await writeFragment(
      root,
      "nested/local.md",
      { ...BASE_META, id: "local" },
      "# Local\n\nlocal prose",
    );
    await writeFragment(
      root,
      "nested/deeper/up.md",
      { ...BASE_META, id: "up", order: 50 },
      "# Up\n\nup prose",
    );
    const manifestPath = await writeManifest(
      root,
      ["local.md", "deeper/up.md"],
      "../out/deep/AGENTS.md",
      "nested/manifest.json",
    );

    const result = await assemble(manifestPath);
    expect(result.fragments.map((fragment) => fragment.meta.id)).toEqual(["up", "local"]);
    expect(result.output).toBe(resolve(join(root, "out", "deep", "AGENTS.md")));
  });
});

describe("duplicate identity failures", () => {
  test("rejects duplicate ids", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", { ...BASE_META, order: 100 }, "# A\n\nprose a");
    await writeFragment(root, "b.md", { ...BASE_META, order: 200 }, "# B\n\nprose b");
    const manifestPath = await writeManifest(root, ["a.md", "b.md"], "out.md");

    await expect(assemble(manifestPath)).rejects.toThrow(/duplicate fragment id "example"/);
  });

  test("rejects duplicate orders", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", { ...BASE_META, id: "alpha" }, "# A\n\nprose a");
    await writeFragment(root, "b.md", { ...BASE_META, id: "beta" }, "# B\n\nprose b");
    const manifestPath = await writeManifest(root, ["a.md", "b.md"], "out.md");

    await expect(assemble(manifestPath)).rejects.toThrow(/duplicate fragment order 100/);
  });
});

describe("frontmatter validation", () => {
  test("rejects fragments without frontmatter", () => {
    expect(() => parseFragment("# Example\n\nprose", "a.md")).toThrow(/must declare frontmatter/);
  });

  test("rejects malformed frontmatter", () => {
    expect(() => parseFragment("---\nid: [unclosed\n---\n\n# Example\n\nprose", "a.md")).toThrow(
      /invalid frontmatter/,
    );
  });

  test.each([
    ["missing id", { order: 1, source: "/s.md", status: "draft" }, /'id' must be/],
    ["bad id", { ...BASE_META, id: "Not Kebab!" }, /'id' must be/],
    ["missing order", { id: "x", source: "/s.md", status: "draft" }, /'order' must be/],
    ["non-integer order", { ...BASE_META, order: 1.5 }, /'order' must be/],
    ["missing source", { id: "x", order: 1, status: "draft" }, /'source' must be a bundle-root/],
    [
      "relative source",
      { ...BASE_META, source: "doc/README.md" },
      /'source' must be a bundle-root-relative path starting with '\/'/,
    ],
    [
      "traversal source",
      { ...BASE_META, source: "/doc/../secret.md" },
      /normalized path without '\.' or '\.\.' segments/,
    ],
    [
      "comment-breaking source",
      { ...BASE_META, source: "/doc/--x.md" },
      /'source' must not contain "--"/,
    ],
    [
      "control-character source",
      { ...BASE_META, source: "/doc/\tx.md" },
      /'source' must not contain control characters/,
    ],
    ["missing status", { id: "x", order: 1, source: "/s.md" }, /'status' must/],
    ["bad status", { ...BASE_META, status: "wip" }, /'status' must/],
  ])("rejects %s", (_name, meta, pattern) => {
    const frontmatter = Object.entries(meta)
      .map(([key, value]) => `${key}: ${JSON.stringify(value)}`)
      .join("\n");
    expect(() => parseFragment(`---\n${frontmatter}\n---\n\n# Example\n\nprose`, "a.md")).toThrow(
      pattern,
    );
  });

  test("rejects fragment paths that would break provenance comments", () => {
    expect(() =>
      parseFragment(
        "---\nid: x\norder: 1\nsource: /s.md\nstatus: draft\n---\n\n# E\n\np",
        "a-->b.md",
      ),
    ).toThrow(/fragment path must not contain "-->"/);
  });
});

describe("markdown validation", () => {
  test("rejects prose without any top-level heading", () => {
    expect(() => validateProse("just text, no heading", "a.md")).toThrow(
      /must begin with a top-level/,
    );
  });

  test("rejects leading sub-headings", () => {
    expect(() => validateProse("## Only sub-heading\n\nprose", "a.md")).toThrow(
      /found heading depth 2/,
    );
  });

  test("rejects a top-level heading that is not in leading position", () => {
    expect(() => validateProse("## Intro\n\n# Late heading\n\nprose", "a.md")).toThrow(
      /must begin with a top-level/,
    );
  });

  test("rejects multiple top-level headings", () => {
    expect(() => validateProse("# First\n\n# Second\n\nprose", "a.md")).toThrow(
      /exactly one top-level \(#\) heading, found 2/,
    );
  });

  test("rejects heading-only fragments", () => {
    expect(() => validateProse("# Example", "a.md")).toThrow(
      /content beyond its top-level heading/,
    );
  });

  test("rejects empty prose", () => {
    expect(() => validateProse("", "a.md")).toThrow(/no Markdown content/);
  });

  test("accepts heading-led prose with content", () => {
    expect(() => validateProse("# Example\n\nprose", "a.md")).not.toThrow();
  });
});

describe("link destination validation", () => {
  test("rejects fragment-relative links", () => {
    expect(() => validateProse("# E\n\nsee [other](other.md)", "a.md")).toThrow(
      /link destination "other\.md" is fragment-relative/,
    );
  });

  test("rejects fragment-relative images", () => {
    expect(() => validateProse("# E\n\n![alt](img/pic.png)", "a.md")).toThrow(
      /image destination "img\/pic\.png" is fragment-relative/,
    );
  });

  test("rejects fragment-relative reference-style links as resolved by marked", () => {
    expect(() =>
      validateProse("# E\n\nsee [thing][ref]\n\n[ref]: notes/details.md", "a.md"),
    ).toThrow(/link destination "notes\/details\.md" is fragment-relative/);
  });

  test("rejects fragment-relative links nested in lists and tables", () => {
    expect(() => validateProse("# E\n\n- [list](nested.md)", "a.md")).toThrow(
      /link destination "nested\.md" is fragment-relative/,
    );
    expect(() => validateProse("# E\n\n| a |\n|---|\n| [tbl](table.md) |", "a.md")).toThrow(
      /link destination "table\.md" is fragment-relative/,
    );
  });

  test("rejects protocol-relative destinations", () => {
    expect(() => validateProse("# E\n\n[cdn](//host/x.md)", "a.md")).toThrow(
      /is fragment-relative/,
    );
  });

  test("permits anchors, bundle-root, and protocol destinations", () => {
    const prose = [
      "# E",
      "",
      "[anchor](#section), [root](/doc/README.md), [site](https://example.com/x), [mail](mailto:a@b.c), [self]()",
    ].join("\n");
    expect(() => validateProse(prose, "a.md")).not.toThrow();
  });

  test("ignores destinations inside code spans and code blocks", () => {
    expect(() =>
      validateProse("# E\n\nuse `see [x](y.md)` inline\n\n```\n[z](w.md)\n```\n", "a.md"),
    ).not.toThrow();
  });
});

describe("write and check", () => {
  test("writes atomically and reports fresh until the output changes", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", BASE_META, "# Example\n\nsome prose");
    const manifestPath = await writeManifest(root, ["a.md"], "out/AGENTS.md");

    await writeAssembly(manifestPath);
    const written = await readFile(join(root, "out", "AGENTS.md"), "utf8");
    expect(written).toContain("# Example");

    await expect(checkAssembly(manifestPath)).resolves.toEqual({ status: "fresh" });

    const st = await stat(join(root, "out"));
    expect(st.isDirectory()).toBe(true);

    await writeFile(join(root, "out", "AGENTS.md"), "# hand edited\n");
    const stale = await checkAssembly(manifestPath);
    expect(stale.status).toBe("stale");
    if (stale.status === "stale") {
      expect(stale.detail).toMatch(/differs from assembly at byte 0/);
    }

    // check never writes: the stale hand edit is still present afterwards
    expect(await readFile(join(root, "out", "AGENTS.md"), "utf8")).toBe("# hand edited\n");
  });

  test("reports stale when output is missing", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", BASE_META, "# Example\n\nsome prose");
    const manifestPath = await writeManifest(root, ["a.md"], "never-written.md");

    const stale = await checkAssembly(manifestPath);
    expect(stale.status).toBe("stale");
    if (stale.status === "stale") {
      expect(stale.detail).toMatch(/output missing/);
    }
  });

  test("compares buffers and reports true byte offsets with multibyte content", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", BASE_META, "# Título 😀\n\n日本語の文章です");
    const manifestPath = await writeManifest(root, ["a.md"], "unicode.md");

    const result = await writeAssembly(manifestPath);
    expect(result.totalBytes).not.toBe(result.bytes.length);

    const output = join(root, "unicode.md");
    const expected = await readFile(output);
    // truncate inside the multibyte heading region
    await writeFile(output, expected.subarray(0, 40));
    const stale = await checkAssembly(manifestPath);
    expect(stale.status).toBe("stale");
    if (stale.status === "stale") {
      expect(stale.detail).toContain("at byte 40");
      expect(stale.detail).toContain(`expected ${expected.length} bytes, found 40`);
    }
  });

  test("surfaces non-ENOENT read failures instead of treating them as stale", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", BASE_META, "# Example\n\nprose");
    const manifestPath = await writeManifest(root, ["a.md"], "occupied.md");

    // the output path is a directory: reading it fails with EISDIR, not ENOENT
    await mkdir(join(root, "occupied.md"));
    await expect(checkAssembly(manifestPath)).rejects.toThrow(/EISDIR|illegal operation/);
  });

  test("check honors the output override", async () => {
    const root = await scratch();
    await writeFragment(root, "a.md", BASE_META, "# Example\n\nsome prose");
    const manifestPath = await writeManifest(root, ["a.md"], "from-manifest.md");
    const override = join(root, "override.md");

    await writeAssembly(manifestPath, override);
    await expect(checkAssembly(manifestPath, override)).resolves.toEqual({ status: "fresh" });
    // the manifest output was never written
    await expect(checkAssembly(manifestPath)).resolves.toEqual({
      status: "stale",
      detail: expect.stringContaining("output missing"),
    });
  });
});

describe("renderDocument", () => {
  test("places provenance between marker and prose with single trailing newline", () => {
    const fragment = parseFragment(
      "---\nid: alpha\norder: 10\nsource: /README.md\nstatus: draft\n---\n\n# Alpha\n\nalpha prose\n",
      "a.md",
    );
    const bytes = renderDocument([fragment]);
    expect(bytes).toBe(
      [
        "<!-- GENERATED FILE - DO NOT EDIT",
        "Assembled by `rekon agents` from declared GLOBAL.md fragments.",
        "Edit the fragment sources and re-run `node rekon.ts agents`;",
        "`node rekon.ts agents --check` detects hand edits to this file.",
        "-->",
        "",
        "<!-- rekon-fragment: id=alpha order=10 path=a.md source=/README.md -->",
        "",
        "# Alpha",
        "",
        "alpha prose",
      ].join("\n") + "\n",
    );
  });
});

describe("readManifest", () => {
  test("keeps declared paths as written", async () => {
    const root = await scratch();
    const manifestPath = await writeManifest(root, ["frag/a.md"], "out/AGENTS.md");

    await expect(readManifest(manifestPath)).resolves.toEqual({
      fragments: ["frag/a.md"],
      output: "out/AGENTS.md",
    });
  });
});

describe("agents CLI end to end", () => {
  interface ExecError extends Error {
    code?: number | string;
    stdout?: string;
    stderr?: string;
  }

  async function runAgents(...args: string[]): Promise<{ stdout: string; stderr: string }> {
    return execFile("node", ["rekon.ts", "agents", ...args], {
      cwd: process.cwd(),
      encoding: "utf8",
    });
  }

  async function runAgentsExpectingFailure(...args: string[]): Promise<ExecError> {
    try {
      await runAgents(...args);
    } catch (error) {
      return error as ExecError;
    }
    throw new Error(`expected command to fail: rekon agents ${args.join(" ")}`);
  }

  test("assembles, verifies fresh, and detects stale without writing", async () => {
    const root = await scratch();
    await writeFragment(root, "fragment.md", BASE_META, "# CLI Example\n\nassembled prose");
    const manifestPath = await writeManifest(root, ["fragment.md"], "AGENTS.out.md");

    const assembled = await runAgents("--manifest", manifestPath);
    expect(assembled.stdout).toContain("assembled 1 fragment(s)");
    expect(assembled.stdout).toContain(
      "(100 bytes)".replace("100", String(Buffer.byteLength("# CLI Example\n\nassembled prose"))),
    );
    const outputPath = join(root, "AGENTS.out.md");
    expect(await readFile(outputPath, "utf8")).toContain("# CLI Example");

    await expect(runAgents("--manifest", manifestPath, "--check")).resolves.toMatchObject({
      stdout: expect.stringContaining("fresh:"),
    });

    await writeFile(outputPath, "# hand edited\n");
    const stale = await runAgentsExpectingFailure("--manifest", manifestPath, "--check");
    expect(stale.code).not.toBe(0);
    expect(`${stale.stderr ?? ""}${stale.stdout ?? ""}`).toContain("stale: output differs");
    // the failed check did not overwrite the hand edit
    expect(await readFile(outputPath, "utf8")).toBe("# hand edited\n");
  }, 30_000);

  test("wires --output override and the default manifest path", async () => {
    const root = await scratch();
    await writeFragment(root, "fragment.md", BASE_META, "# Override Example\n\nprose");
    const manifestPath = await writeManifest(root, ["fragment.md"], "declared-out.md");
    const overridePath = join(root, "override.out.md");

    const result = await runAgents("--manifest", manifestPath, "--output", overridePath);
    expect(result.stdout).toContain(overridePath);
    expect(await readFile(overridePath, "utf8")).toContain("# Override Example");
    await expect(stat(join(root, "declared-out.md"))).rejects.toMatchObject({ code: "ENOENT" });

    // with no --manifest, the default doc/agents.manifest.json is wired in
    // (and correctly absent: no production manifest exists)
    const missing = await runAgentsExpectingFailure();
    expect(missing.code).not.toBe(0);
    expect(`${missing.stderr ?? ""}${missing.stdout ?? ""}`).toContain(
      "manifest not readable: doc/agents.manifest.json",
    );
  }, 30_000);
});
