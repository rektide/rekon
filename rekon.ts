#!/usr/bin/env node
import { cli } from "gunshi";
import { command as combineCommand } from "./command/combine.ts";

await cli(process.argv.slice(2), combineCommand, {
  name: "rekon",
  version: "1.0.0",
  description: "Rekon CLI tool",
});
