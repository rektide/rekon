# code module — ambient fragment

- commit each coherent stage when available, including prompts, research, drafts, and guidance: `jj commit -m "<description>" <files...>`. do not wait for the task or wave to end. before a review-driven revamp, save the present owned state; commit the revision separately and explain its delta and destination. checkpoint is not acceptance. the empty working-copy commit is expected; do not verify, move on. depth: /code/commits.md#commits-checkpoints
- changes intermingled **within** a file → `jj-hunk`: full `hunk-<sha256>` IDs only, always dry-run the spec first. load /code/jj-hunk.md before hunk-level work.
- review comments: discuss direction as well as committing the predecessor before editing; land each correction as its own well-argued commit. /code/review-handling.md
- plans carry no time estimates, ever. /code/planning.md
- module entry point: /code/README.md
