import * as NodeRuntime from "@effect/platform-node/NodeRuntime";
import * as NodeServices from "@effect/platform-node/NodeServices";
import * as Console from "effect/Console";
import * as Effect from "effect/Effect";
import * as Schema from "effect/Schema";
import * as Command from "effect/cli/Command";
import * as Flag from "effect/cli/Flag";
import packageMetadata from "../../package.json" with { type: "json" };
import { checkAssembly, writeAssembly } from "../assembly/assemble.ts";
import { DEFAULT_MANIFEST } from "../assembly/manifest.ts";

export class StaleAssemblyError extends Schema.TaggedError<StaleAssemblyError>()(
  "StaleAssemblyError",
  { message: Schema.String },
) {}

const flags = {
  manifest: Flag.String("manifest").pipe(
    Flag.withAlias("m"),
    Flag.withDefault(DEFAULT_MANIFEST),
    Flag.withDescription("JSON manifest; declared paths resolve relative to the manifest file"),
  ),
  output: Flag.String("output").pipe(
    Flag.withAlias("o"),
    Flag.optional,
    Flag.withDescription("Output override, resolved against the working directory"),
  ),
  check: Flag.Boolean("check").pipe(
    Flag.withAlias("c"),
    Flag.withDefault(false),
    Flag.withDescription("Compare UTF-8 bytes without writing; fail when the output is stale"),
  ),
};

export const agentsCommand = Command.make(
  "agents",
  flags,
  Effect.fn("agents.command")(function* ({
    manifest,
    output,
    check,
  }: Command.Command.Config.Infer<typeof flags>) {
    const request = { manifestPath: manifest, cwd: process.cwd(), outputOverride: output };
    if (check) {
      const result = yield* checkAssembly(request);
      if (result.status === "stale") {
        return yield* new StaleAssemblyError({ message: `stale: ${result.detail}` });
      }
      yield* Console.log(`fresh: output matches assembly of ${manifest}`);
      return;
    }
    const result = yield* writeAssembly(request);
    yield* Console.log(
      `assembled ${result.fragments.length} fragment(s) into ${result.output} (${result.totalBytes} bytes)`,
    );
    for (const { meta, path, proseBytes } of result.fragments) {
      yield* Console.log(`  ${meta.order}\t${meta.id}\t${path}\t(${proseBytes} bytes)`);
    }
  }),
).pipe(Command.withDescription("Assemble AGENTS.md from manifest-selected GLOBAL.md fragments"));

/** Embeddable CLI effect; only the application boundary chooses the Node runtime. */
export const runAgents = Command.runWith(agentsCommand, { version: packageMetadata.version });

export function main(args: ReadonlyArray<string>): void {
  NodeRuntime.runMain(runAgents(args).pipe(Effect.provide(NodeServices.layer)));
}
