#!/usr/bin/env node
import { cli } from "gunshi";
import { readFile, writeFile } from "node:fs/promises";
import { glob } from "glob";
import { basename, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

//#region command/combine.ts
const __dirname = dirname(fileURLToPath(import.meta.url));
async function* prompts(patterns, baseDir = "prompt/") {
	for (const pattern of patterns) yield* await glob(pattern.startsWith("prompt/") ? pattern : join(baseDir, pattern), { cwd: join(__dirname, "..") });
}
async function* planFiles(patterns, baseDir = "prompt/") {
	for await (const file of prompts(patterns, baseDir)) {
		const fullPath = join(__dirname, "..", file);
		try {
			yield {
				name: file,
				content: await readFile(fullPath, "utf-8")
			};
		} catch (error) {
			if (error.code !== "ENOENT") throw error;
		}
	}
}
function ensureHeader(name, content) {
	if (content.trimStart().startsWith("#")) return content;
	return `# ${basename(name, ".md")}\n\nThis is the prompt called ${name}\n\n${content}`;
}
async function writeCombined(patterns, outputFile = "COMBINED.md") {
	const combinedContent = [];
	for await (const { name, content } of planFiles(patterns)) {
		const contentWithHeader = ensureHeader(name, content);
		combinedContent.push(contentWithHeader);
		combinedContent.push("\n\n---\n\n");
	}
	const finalContent = combinedContent.join("");
	await writeFile(join(__dirname, "..", outputFile), finalContent, "utf-8");
	return outputFile;
}
const command = {
	name: "combine",
	description: "Combine markdown plans from prompt/ directory",
	args: { output: {
		type: "string",
		short: "o",
		default: "COMBINED.md",
		description: "Output file path"
	} },
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
	}
};

//#endregion
//#region rekon.ts
await cli(process.argv.slice(2), command, {
	name: "rekon",
	version: "1.0.0",
	description: "Rekon CLI tool"
});

//#endregion
export {  };