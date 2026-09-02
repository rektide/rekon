#!/usr/bin/env node
import { realpath } from "node:fs/promises";
import { pathToFileURL } from "node:url";
import { cli, define } from "gunshi";
import { DEFAULT_MANIFEST, checkAssembly, writeAssembly } from "../agents/assemble.ts";

export default define({
  name: "agents",
  description: "Assemble AGENTS.md from declared GLOBAL.md fragments via an explicit manifest",
  args: {
    manifest: {
      type: "string",
      short: "m",
      default: DEFAULT_MANIFEST,
      description:
        "Assembly manifest (JSON); fragment and output paths inside it resolve relative to the manifest file",
    },
    output: {
      type: "string",
      short: "o",
      description:
        "Write to this path instead of the manifest output (resolved against the working directory)",
    },
    check: {
      type: "boolean",
      short: "c",
      default: false,
      description:
        "Compare assembled bytes against the output and fail nonzero when stale, without writing",
    },
  },
  run: async (ctx) => {
    const manifest = ctx.values.manifest ?? DEFAULT_MANIFEST;
    const outputOverride = ctx.values.output;

    if (ctx.values.check) {
      const result = await checkAssembly(manifest, outputOverride);
      if (result.status === "stale") {
        throw new Error(`stale: ${result.detail}`);
      }
      console.log(`fresh: output matches assembly of ${manifest}`);
      return;
    }

    const result = await writeAssembly(manifest, outputOverride);
    console.log(
      `assembled ${result.fragments.length} fragment(s) into ${result.output} (${result.bytes.length} bytes)`,
    );
    for (const fragment of result.fragments) {
      console.log(`  ${fragment.meta.order}\t${fragment.meta.id}\t${fragment.path}`);
    }
  },
});

void (async () => {
  const mainPath = await realpath(process.argv[1]);
  const mainUrl = pathToFileURL(mainPath).href;
  if (import.meta.url === mainUrl) {
    const module = await import("./agents.ts");
    await cli(process.argv.slice(2), module.default, { name: "agents" });
  }
})();
