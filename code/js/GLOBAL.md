---
id: javascript
order: 250
source: /code/js/README.md
status: draft
---

# JavaScript and TypeScript

- Prefer TypeScript. Use explicit `.ts` extensions for local imports and the
  `node:` prefix for Node builtins. Keep class members public; visibility does
  not guarantee API stability.
- Prefer pnpm. Add dependencies through `pnpm install <package>@latest` (or `-D`),
  rather than hand-editing dependency entries. Run source directly with
  `node path/to/cli.ts` in development and examples; build output is for npm
  distribution only.
- Prefer promise-based async APIs and `node:fs/promises`, not sync methods or
  callbacks where promises exist. Start independent work together; await when
  its value is needed.
- Default tools: oxfmt formatting, oxlint linting, tsdown distribution, vitest
  tests when no test stack exists, and `@typescript/native-preview` / `tsgo`
  for typechecking. Bundling does not typecheck. With TypeScript configuration,
  use noEmit, allowImportingTsExtensions, and isolatedDeclarations where suitable.
- For complex CLI use gunshi with `@gunshi/plugin-completion`; for config use
  c12 or at least xdg-basedir. Respect a project's established tooling; the
  accepted Rekon toolchain is Rust-first for new capabilities.
- Compose `build:*`, `check:*`, and `fix:*` tasks using concurrently glob targets,
  such as `concurrently 'pnpm:check:*'`. Include a dev/watch target. Read
  [the JS guide](/code/js/README.md) for vite-plus version/peer setup and full recipes.
- Executable modules should detect direct invocation, asynchronously realpath
  `process.argv[1]` before comparing it, then dynamically load their main module.
  Use `.then` so importing the module does not block on invocation detection;
  include a shebang and executable bit where appropriate.
- Web components use wc-toolkit (Web Components Toolkit) with descriptive JSDoc
  to generate Custom Element Manifests; wire manifest generation into the toolkit.
