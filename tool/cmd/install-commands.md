---
type: HistoricalPlan
title: Parallelize the legacy install-commands tool
description: Preserved concurrency proposal for the old prompt-command installer, with its present source and retired prompt context made explicit.
resource: /tool/cmd/install-commands.md
tags: [tool, prompts, commands, concurrency, history]
status: deprecated
generated: { by: "model:gpt-6-astra", at: 2026-10-06 }
sources:
  - {
      resource: "https://github.com/rektide/rekon/blob/ece454d44301fdf1a7b00a21eaf6b42ad6eeee90/doc/plan-parallelize.md",
      title: Original installer concurrency proposal,
    }
  - { resource: /src/command/install-commands.ts, title: Existing sequential Gunshi installer }
  - { resource: /prompt/.archive/README.md, title: Retired prompt inventory }
---

# Parallelize install-commands

**`t-install-commands` — historical proposal.** This material moved from
`doc/plan-parallelize.md` into the [toolkit](/tool/README.md). The Gunshi
implementation remains in
[`src/command/install-commands.ts`](/src/command/install-commands.ts) and the
root CLI still registers it. [`prompt/` now contains only archived material](/prompt/.archive/README.md),
so this plan is context for a legacy tool, not a current instruction to revive
the prompt installer. The original proposal follows.

## Problem

Current `install-commands` processes files sequentially in a for-loop. With many files, this is slower than necessary since most operations (reading files, checking symlinks, creating symlinks) are I/O bound and can happen concurrently.

## Goals

- Process multiple files concurrently instead of sequentially
- Maintain same output ordering for predictability
- Handle errors gracefully
- Respect system limits

## Approach

### Batch Processing

Process files in batches using `Promise.all()` or `Promise.allSettled()`:

```typescript
const BATCH_SIZE = 10;

for (let i = 0; i < markdownFiles.length; i += BATCH_SIZE) {
  const batch = markdownFiles.slice(i, i + BATCH_SIZE);
  await Promise.all(batch.map((file) => processFile(file)));
}
```

### Output Ordering

To maintain consistent output order, collect results then sort by original index:

```typescript
const results = await Promise.all(
  markdownFiles.map(async (file, index) => ({
    index,
    result: await processFile(file),
  })),
);

results.sort((a, b) => a.index - b.index);
results.forEach((r) => console.log(r.result));
```

### Error Handling

Use `Promise.allSettled()` to handle individual failures without stopping all work:

```typescript
const settled = await Promise.allSettled(markdownFiles.map((file) => processFile(file)));

settled.forEach((result) => {
  if (result.status === "fulfilled") {
    console.log(result.value);
  } else {
    console.error(result.reason);
  }
});
```

## Implementation Considerations

### File System Limits

- Don't overwhelm the file system with too many concurrent operations
- Batch size of 5-10 is reasonable for most systems
- Consider configurable batch size

### Dry Run Mode

In dry-run mode, we're only reading files and checking status, which is very fast. Parallelization benefits may be minimal but still valuable.

### Front-Matter Parsing

Already reading file content for front-matter check, so no additional overhead.

### Precheck Function

`precheckTarget` uses `readlink` which is async-friendly. No blocking operations.

## Proposed Changes

1. Add configurable `BATCH_SIZE` constant
2. Replace sequential for-loop with batched `Promise.allSettled()`
3. Collect and sort results for consistent output
4. Maintain quiet mode suppression of warnings
5. Preserve all current functionality and flags

## Benefits

- Faster execution for large numbers of files
- Better utilization of async I/O
- Same user-visible behavior (ordering, output)
- Graceful error handling continues on individual failures
