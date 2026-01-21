#!/usr/bin/env node
import { readlink, symlink, mkdir, rm, readFile } from "node:fs/promises";
import { join, dirname, basename, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { define } from 'gunshi'
import { glob } from "glob";
import { xdgConfig } from 'xdg-basedir';
import matter from 'gray-matter';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function findMarkdownFiles(dir: string, subdirs: boolean = false): Promise<string[]> {
  const pattern = subdirs ? '**/*.{md,mdx}' : '*.{md,mdx}';
  const files = await glob(pattern, { cwd: dir, absolute: true });
  return files.sort();
}

function checkFrontMatter(content: string, filePath: string, quiet: boolean): void {
  const { data } = matter(content);
  if (!data.description && !quiet) {
    console.warn(`Warning: ${filePath} missing description in front-matter`);
  }
}

type TargetStatus = 'does-not-exist' | 'already-symlinked' | 'different-symlink' | 'regular-file';

async function precheckTarget(destFile: string, absoluteSource: string): Promise<TargetStatus> {
  try {
    const linkTarget = await readlink(destFile);
    if (linkTarget === absoluteSource) {
      return 'already-symlinked';
    }
    return 'different-symlink';
  } catch (err: unknown) {
    if (err instanceof Error && (err as NodeJS.ErrnoException).code === 'ENOENT') {
      return 'does-not-exist';
    }
    return 'regular-file';
  }
}

async function run(ctx: any) {
  const force = ctx.values.f || false;
  const dryRun = ctx.values.n || false;
  const quiet = ctx.values.q || false;
  const subdirs = ctx.values.s || false;
  const projectRoot = resolve(__dirname, '..');
  const promptDir = join(projectRoot, 'prompt');
  const configDir = xdgConfig;
  const opencodeCommandDir = join(configDir || join(process.env.HOME || process.env.USERPROFILE || '.', '.config'), 'opencode', 'command');

  const markdownFiles = await findMarkdownFiles(promptDir, subdirs);

  if (!dryRun) {
    await mkdir(opencodeCommandDir, { recursive: true });
  }

  for (const sourceFile of markdownFiles) {
    const relativePath = sourceFile.slice(promptDir.length + 1);
    const destFile = join(opencodeCommandDir, basename(sourceFile));
    const absoluteSource = resolve(sourceFile);

    const content = await readFile(sourceFile, 'utf-8');
    checkFrontMatter(content, relativePath, quiet);

    const status = await precheckTarget(destFile, absoluteSource);
    
    if (status === 'already-symlinked') {
      console.log(`${relativePath}: already-done`);
      continue;
    }
    
    if (status === 'different-symlink' || status === 'regular-file') {
      if (!force) {
        console.log(`${relativePath}: error-existing-file`);
        continue;
      }
    }

    if (!dryRun) {
      try {
        if (force && (status === 'different-symlink' || status === 'regular-file')) {
          await rm(destFile, { force: true });
        }
        await symlink(absoluteSource, destFile, 'file');
      } catch (err: unknown) {
        if (err instanceof Error) {
          console.error(`${relativePath}: error - ${err.message}`);
        }
        process.exit(1);
      }
    }
    console.log(`${relativePath}: done`);
  }
}

export default define({
  name: 'install-commands',
  description: 'Install prompt/ markdown files as opencode commands',
  args: {
    f: {
      type: 'boolean',
      short: 'f',
      default: false,
      description: 'Force overwrite existing files',
    },
    n: {
      type: 'boolean',
      short: 'n',
      default: false,
      description: 'Dry run - show what would happen without making changes',
    },
    q: {
      type: 'boolean',
      short: 'q',
      default: false,
      description: 'Quiet - suppress warnings',
    },
    s: {
      type: 'boolean',
      short: 's',
      default: false,
      description: 'Include subdirectories',
    },
  },
  run,
});
