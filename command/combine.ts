#!/usr/bin/env node
import { readFile, writeFile } from "node:fs/promises";
import { glob } from "glob";
import { join, dirname, basename } from "node:path";
import { fileURLToPath } from "node:url";
import { define } from 'gunshi'

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function* prompts(patterns, baseDir = "prompt/") {
  for (const pattern of patterns) {
    const searchPattern = pattern.startsWith("prompt/") ? pattern : join(baseDir, pattern);
    const files = await glob(searchPattern, { cwd: join(__dirname, "..") });
    yield* files;
  }
}

async function* planFiles(patterns, baseDir = "prompt/") {
  for await (const file of prompts(patterns, baseDir)) {
    const fullPath = join(__dirname, "..", file);
    try {
      const content = await readFile(fullPath, "utf-8");
      yield { name: file, content };
    } catch (error) {
      if (error.code !== "ENOENT") {
        throw error;
      }
    }
  }
}

function ensureHeader(name, content) {
  const trimmed = content.trimStart();
  if (trimmed.startsWith("#")) {
    return content;
  }
  const baseName = basename(name, ".md");
  return `# ${baseName}\n\nThis is prompt called ${name}\n\n${content}`;
}

async function writeCombined(patterns, outputFile = "COMBINED.md") {
  const combinedContent = [];
  for await (const { name, content } of planFiles(patterns)) {
    const contentWithHeader = ensureHeader(name, content);
    combinedContent.push(contentWithHeader);
    combinedContent.push("\n\n---\n\n");
  }
  const finalContent = combinedContent.join("");
  const outputPath = outputFile.startsWith("/") ? outputFile : join(__dirname, "..", outputFile);
  await writeFile(outputPath, finalContent, "utf-8");
  return outputFile;
}

export default define({
  name: 'combine',
  description: 'Combine markdown plans from prompt/ directory',
  args: {
    output: {
      type: 'string',
      short: 'o',
      default: 'COMBINED.md',
      description: 'Output file path',
    },
  },
  run: async (ctx) => {
    const rawArgs = process.argv.slice(2);
    const outputFlagIndex = rawArgs.findIndex((arg) => arg === "-o" || arg === "--output");
    let outputFile = ctx.values.output;
    let patterns = rawArgs;

    if (outputFlagIndex >= 0) {
      outputFile = rawArgs[outputFlagIndex + 1] || outputFile;
      patterns = rawArgs.slice(0, outputFlagIndex).concat(rawArgs.slice(outputFlagIndex + 2));
    }

    if (patterns.length === 0) {
      console.error("Error: at least one pattern is required");
      process.exit(1);
    }

    const resultFile = await writeCombined(patterns, outputFile);
    console.log(`Combined ${patterns.length} file(s) into ${resultFile}`);
  },
});
