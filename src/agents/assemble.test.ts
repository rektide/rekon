import { randomUUID } from "node:crypto";
import { mkdir, readFile, rm, stat, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
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
  source: "doc/example/README.md",
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
      "---\nid: example\norder: 100\nsource: doc/example/README.md\nstatus: stable\n---\n\n# Example\n\nwindows prose\n",
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
    expect(result.bytes).not.toContain("source: doc/example/README.md\n");
    expect(result.bytes).toContain(
      "<!-- rekon-fragment: id=example order=100 path=a.md source=doc/example/README.md -->",
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
});

describe("manifest failures", () => {
  test("rejects duplicate fragment paths that resolve to the same file", async () => {
    const root = await scratch();
    await writeFragment(root, "frag/a.md", BASE_META, "# Example\n\nprose");
    const manifestPath = await writeManifest(root, ["frag/a.md", "./frag/a.md"], "out.md");

    await expect(assemble(manifestPath)).rejects.toThrow(/same fragment twice/);
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
      "other/up.md",
      { ...BASE_META, id: "up", order: 50 },
      "# Up\n\nup prose",
    );
    const manifestPath = await writeManifest(
      root,
      ["./local.md", "../other/up.md"],
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
    ["missing id", { order: 1, source: "s.md", status: "draft" }, /'id' must be/],
    ["bad id", { ...BASE_META, id: "Not Kebab!" }, /'id' must be/],
    ["missing order", { id: "x", source: "s.md", status: "draft" }, /'order' must be/],
    ["non-integer order", { ...BASE_META, order: 1.5 }, /'order' must be/],
    ["missing source", { id: "x", order: 1, status: "draft" }, /'source' must/],
    ["missing status", { id: "x", order: 1, source: "s.md" }, /'status' must/],
    ["bad status", { ...BASE_META, status: "wip" }, /'status' must/],
  ])("rejects %s", (_name, meta, pattern) => {
    const frontmatter = Object.entries(meta)
      .map(([key, value]) => `${key}: ${JSON.stringify(value)}`)
      .join("\n");
    expect(() => parseFragment(`---\n${frontmatter}\n---\n\n# Example\n\nprose`, "a.md")).toThrow(
      pattern,
    );
  });
});

describe("markdown validation", () => {
  test("rejects prose without a top-level heading", () => {
    expect(() => validateProse("just text, no heading", "a.md")).toThrow(
      /must begin with a top-level/,
    );
  });

  test("rejects leading sub-headings", () => {
    expect(() => validateProse("## Only sub-heading\n\nprose", "a.md")).toThrow(
      /found heading depth 2/,
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
    const bytes = renderDocument([
      {
        path: "a.md",
        meta: { id: "alpha", order: 10, source: "README.md", status: "draft" },
        prose: "# Alpha\n\nalpha prose",
      },
    ]);
    expect(bytes).toBe(
      [
        "<!-- GENERATED FILE - DO NOT EDIT",
        "Assembled by `rekon agents` from declared GLOBAL.md fragments.",
        "Edit the fragment sources and re-run `node rekon.ts agents`;",
        "`node rekon.ts agents --check` detects hand edits to this file.",
        "-->",
        "",
        "<!-- rekon-fragment: id=alpha order=10 path=a.md source=README.md -->",
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
