---
type: Reference
title: ATProto CLI sessions and identity
description: Practical authentication and identity checks for ATProto command-line clients.
resource: /atproto/cli.md
status: draft
generated: { by: "model:openai/gpt-6.1-terra#medium", at: 2026-10-31 }
sources:
  - { resource: "opencode://ses_e6c505360ffeVFenH2VYDN7XS0", title: Human direction and integration of CLI guidance }
  - { resource: "opencode://ses_e6bf5d354ffelsII2ETAWRk4md", title: Research into CLI authentication, identity checks and version differences }
  - { resource: /design/atproto/cli0.glm53fx.md, title: Detailed research and client sources }
  - { resource: /atproto/README.md, title: Public ATProto identity and record lookups }
---

# ATProto CLI sessions and identity

For public handle and record lookups see [ATProto lookup](/atproto/README.md).
For publishing, see [forge publication](/forge/publication.md).

## Check the client and account

As checked 2026-10-31, local `atgc` is 0.15.0. Use `atgc --version` and
`atgc auth status` to identify the installed version and configured accounts;
`atgc doctor` offers further checks on this version. Login is interactive:

```sh
atgc auth login jauntywunderkind.bsky.social
atgc auth status
```

`atgc` uses OAuth; a configured account or local status is not by itself proof
that a remote operation will succeed. Its upstream 0.16.0 changes command and
JSON names, including `auth switch` to `auth default` and splitting `doctor`
into `doctor local` and `doctor remote`; consult installed `--help` after an
upgrade. SSH push keys are separate from OAuth login.

Several unrelated tools are named `tg`. Identify the binary and its upstream
before applying instructions. The archived Python client at
`~/archive/oaustegard/claude-tangled-spoke/bin/tg` supports `tg auth status`,
but that status reads the local session file; it is not a remote validity
check. It is not established to be the same client as
[`aly.codes/tg`](https://tangled.org/aly.codes/tg).

The `goat` client offers `goat account check-auth` and
`goat xrpc query @pds com.atproto.server.getSession`; it was not installed
here when checked. Its session store includes a cleartext password. Verify
client versions and current behavior before use.

Never paste tokens or passwords into chat, logs, or commits. Avoid passing
passwords in command arguments; prefer OAuth when available and follow the
client's secure input method. File timestamps do not prove credentials work.
