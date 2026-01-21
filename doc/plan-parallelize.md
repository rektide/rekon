# Parallelize install-commands

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
  await Promise.all(batch.map(file => processFile(file)));
}
```

### Output Ordering

To maintain consistent output order, collect results then sort by original index:

```typescript
const results = await Promise.all(
  markdownFiles.map(async (file, index) => ({
    index,
    result: await processFile(file)
  }))
);

results.sort((a, b) => a.index - b.index);
results.forEach(r => console.log(r.result));
```

### Error Handling

Use `Promise.allSettled()` to handle individual failures without stopping all work:

```typescript
const settled = await Promise.allSettled(
  markdownFiles.map(file => processFile(file))
);

settled.forEach(result => {
  if (result.status === 'fulfilled') {
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
