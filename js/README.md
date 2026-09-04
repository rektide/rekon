# JavaScript / TypeScript development skill

Practical conventions and getting-started recipes for npm / Node.js / TypeScript work:
package management with pnpm, the vite-plus (`vp`) toolchain, running `.ts` files directly,
oxfmt/oxlint, tsdown library builds, vitest tests, CLI structure, and web components. Use when
scaffolding a new JavaScript/TypeScript package, wiring up its toolchain, or deciding how
scripts, config, and module layout should be organized.

This is the promoted home for the npm/Node.js guidance that previously lived only in the global
`AGENTS.md`. Until this skill is fully accepted, the global file remains authoritative where
they overlap; reconcile differences here rather than diverging silently.

## Getting started: a new package

```sh
mkdir my-package && cd my-package
git init  # or jj git init --colocate
pnpm init
pnpm install -D vite-plus vitest @typescript/native-preview
```

`package.json` essentials — `"type": "module"`, `"packageManager": "pnpm@<version>"`, and the
vite-plus override wiring so transitive `vite`/`vitest` resolve to the vite-plus builds:

```json
{
	"type": "module",
	"scripts": {
		"build": "vp pack",
		"dev": "vp pack --watch",
		"test": "vp test",
		"typecheck": "tsgo --noEmit"
	},
	"devDependencies": {
		"@typescript/native-preview": "7.0.0-dev.<current>",
		"vite-plus": "latest",
		"vitest": "npm:@voidzero-dev/vite-plus-test@latest"
	},
	"pnpm": {
		"overrides": {
			"vite": "npm:@voidzero-dev/vite-plus-core@latest",
			"vitest": "npm:@voidzero-dev/vite-plus-test@latest"
		}
	}
}
```

Libraries bundle with `vp pack` (tsdown). Applications instead use `vp dev` / `vp build` /
`vp preview` with a dev server and an `index.html` entry.

A minimal `vite.config.ts`:

```ts
import { defineConfig } from 'vite-plus'

export default defineConfig({
	fmt: { semi: false, useTabs: true, printWidth: 4000 },
	pack: { dts: { tsgo: true }, exports: true },
	lint: { options: { typeAware: true, typeCheck: true } },
})
```

A minimal `tsconfig.json` (bundler-style, no emit, allows `.ts` import specifiers):

```json
{
	"compilerOptions": {
		"target": "esnext",
		"module": "preserve",
		"moduleResolution": "bundler",
		"moduleDetection": "force",
		"strict": true,
		"isolatedModules": true,
		"verbatimModuleSyntax": true,
		"allowImportingTsExtensions": true,
		"noEmit": true,
		"skipLibCheck": true
	},
	"include": ["src"]
}
```

Add `"isolatedDeclarations": true` when emitting `.d.ts` for a published library.

## Package management

- Use `pnpm`, never bare npm/yarn. Install with `pnpm install [-D] <pkg>@latest` — do not edit
  `package.json` dependency fields by hand.
- Refer to node built-ins with the `node:` prefix, e.g. `import { readFile } from 'node:fs/promises'`.
- Preferred modules: `vitest` for tests, `gunshi` (+ `@gunshi/plugin-completion`) for non-trivial
  CLIs, `oxfmt`/`oxlint` (bundled in vite-plus) for format/lint, `tsdown` for emitting
  TypeScript, `@typescript/native-preview` (`tsgo`) for type-checking, `c12` for complex config
  loading, `xdg-basedir` for XDG-compliant config discovery.

## Running TypeScript

- Prefer `.ts` sources; only emit build artifacts (`build/`, `dist/`) for npm distribution.
- Node strips types natively — during development and docs, run entrypoints directly:
  `node src/cli.ts`.
- Import sibling TypeScript modules with explicit `.ts` extensions:
  `import { foo } from './foo.ts'`.

## Async discipline

- Use `node:fs/promises` instead of `node:fs`; avoid sync/execSync when async exists.
- `await` a promise only when its value is needed right away; otherwise keep the promise in a
  variable and await it later, so independent work proceeds concurrently.

## Testing

`vp test` runs vitest. Import test APIs from `vite-plus/test` when using the vite-plus override
wiring. With no prior test tooling, pick vitest.

## npm script structure with concurrently

Compose build/check/fix tasks from namespaced sub-tasks and aggregate them in parallel:

- `build:<tool>` — individual build steps (typecheck, bundle, …)
- `check:<tool>` — individual check/lint steps
- `fix:<tool>` — individual auto-fix steps

Aggregates run sub-tasks via globs: `build` → `concurrently 'pnpm:build:*'`, likewise `check`
and `fix`. Include a `dev` target for watch mode. Example:

```json
{
	"scripts": {
		"build:type": "tsgo --noEmit",
		"build:wxt": "wxt build",
		"build": "concurrently 'pnpm:build:*'",
		"check:lint": "vp lint",
		"check:type": "tsgo --noEmit",
		"check": "concurrently 'pnpm:check:*'",
		"fix:format": "vp fmt --write src",
		"fix:lint": "vp lint --fix",
		"fix": "concurrently 'pnpm:fix:*'",
		"dev": "vp dev"
	}
}
```

Build output is for package distribution only — never for dev workflows or documentation.

## CLI patterns

- If a file exposes a simple CLI, check whether it is the main module and run a sibling
  `foo.main.js` (or `.ts`) when it is, with shebang + executable bit:

```ts
import { realpath } from 'node:fs/promises'

const isMain = await realpath(process.argv[1]).then(
	(path) => path === import.meta.filename,
	() => false,
)

if (isMain) {
	await import('./foo.main.ts')
}
```

Use `.then` rather than top-level `await` for the main-module check so importing the module as
a library never blocks. Document usage with direct `.ts` paths (`node src/cli.ts`), never build
output. Reach for `gunshi` when argument parsing grows beyond trivial.

## TypeScript style

- Class members are public — expose internals, without promising implementation stability.
- `verbatimModuleSyntax` keeps type imports honest (`import type` where applicable).
- Type-check with `tsgo --noEmit` (from `@typescript/native-preview`); tsdown does not
  type-check.

## Web components

Use [Web Components Toolkit (wc-toolkit)](https://github.com/thescientist13/wc-toolkit) to
generate Custom Element Manifests. Describe components with appropriate JSDoc and run the
manifest generation as part of `build:` or `check:` scripts.

## Reference projects

- `~/src/ocif-schema/package/effect-schema-v4` — canonical vite-plus library setup (package.json
  overrides, `vite.config.ts`, `tsconfig.json`).
- `~/src/seven-ten` — vite-plus application (dev server + custom elements + preact), including
  the OAuth browser dev-server pattern.

## Log

- 2026-09-04: created; distilled from global `AGENTS.md` npm/Node sections plus the
  `effect-schema-v4` vite-plus setup.
