#!/usr/bin/env node
import { cli, define } from "gunshi";
import agentsCommand from "./src/command/agents.ts";
import combineCommand from "./src/command/combine.ts";
import installCommandsCommand from "./src/command/install-commands.ts";
import interpolateCommand from "./src/command/interpolate.ts";
import projectFilesCommand from "./src/command/project-files.ts";
import lnCommand from "./src/command/ln.ts";
import completion from "@gunshi/plugin-completion";

const mainCommand = define({
  name: "rekon",
  description: "Rekon CLI tool",
  run: () => {
    console.log(
      "Available commands: agents, combine, install-commands, interpolate, project-files, ln",
    );
    console.log('Run "rekon --help" for more information');
  },
});

await cli(process.argv.slice(2), mainCommand, {
  name: "rekon",
  version: "1.0.0",
  description: "Rekon CLI tool",
  subCommands: {
    agents: agentsCommand,
    combine: combineCommand,
    "install-commands": installCommandsCommand,
    interpolate: interpolateCommand,
    "project-files": projectFilesCommand,
    ln: lnCommand,
  },
  plugins: [completion()],
});
