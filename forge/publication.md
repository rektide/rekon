---
type: Practice
title: Forge publication and maintenance
description: Practical defaults and checks for publishing our repositories on GitHub and Tangled.
resource: /forge/publication.md
status: draft
generated: { by: "model:openai/gpt-6.1-terra#medium", at: 2026-10-31 }
sources:
  - { resource: "opencode://ses_e6c505360ffeVFenH2VYDN7XS0", title: Human direction and integration of repository publication guidance }
  - { resource: "opencode://ses_e6bf5d356ffeiJzx1Fc2TcWK0L", title: Forge client and current Tangled protocol research }
  - { resource: /forge/README.md, title: Read-only upstream tracker practice }
  - { resource: /vcs/jj/README.md, title: jj operating guidance }
  - { resource: "https://docs.tangled.org/", title: Tangled documentation, consulted 2026-10-31 }
---

# Forge publication and maintenance

This is the publish-our-repositories sibling of [reading upstream trackers](/forge/README.md).
For ATProto client sessions, see [CLI authentication](/atproto/cli.md).

## Defaults and checks

- New repositories are **public**, use **`main`**, and go under GitHub account
  **`rektide`**, over SSH. Confirm the intended published name: a checkout or
  existing remote may still carry a predecessor's name.
- Push only when the operator directs it. Inspect the exact committed revision,
  bookmark, existing remotes, and uncommitted work before publication; don't
  accidentally publish an old `main` or overwrite an unrelated repository.
- Authenticate and verify the account before creating anything. On GitHub,
  `gh auth status` checks the CLI; a normal creation is
  `gh repo create rektide/<name> --public`. The SSH URL is
  `git@github.com:rektide/<name>`. Add a remote only after checking whether
  that name is already in use, then push the intended bookmark deliberately.
- On Tangled, the usual identity is `jauntywunderkind.bsky.social`; the local
  SSH alias is `tngl:<handle>/<name>`. Login and SSH push-key authorization are
  separate. Create via the web interface if a compatible authenticated CLI is
  unavailable; verify the repository and its default branch after pushing.
- Check the remote's advertised refs and host UI after publishing. Do not
  automatically prune or mirror all refs; preserve existing remotes unless a
  change is intended.

## Tangled API currency

Research on 2026-10-31 found current Tangled documentation using
`org.tangled.*`; some older `sh.tangled.*` endpoints returned 404 on the
managed knot. This is not evidence that every legacy endpoint is gone. Check
the current client and knot before relying on an old recipe. The local
`atgc` 0.15.0 was logged out and older than upstream; a creation dry run
stopped at authentication, so creation compatibility was not established.
Current Tangled repositories have their own DIDs. Protocol changes and Git
object-format support should be verified for the actual host rather than
assumed from an older checkout.
