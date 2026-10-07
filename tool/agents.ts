#!/usr/bin/env node
import { realpath } from "node:fs/promises";
import { pathToFileURL } from "node:url";

const invocation = process.argv[1];
if (invocation !== undefined) {
  void realpath(invocation).then((mainPath) => {
    if (pathToFileURL(mainPath).href !== import.meta.url) return;
    return import("./cmd/agents.ts").then(({ main }) => main(process.argv.slice(2)));
  });
}
