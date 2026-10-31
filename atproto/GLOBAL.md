---
id: atproto-cli
type: ContextFragment
title: ATProto CLI sessions
description: Quick client identity and authentication checks.
resource: /atproto/GLOBAL.md
order: 280
source: /atproto/cli.md
status: draft
sources:
  - { resource: "opencode://ses_e6c505360ffeVFenH2VYDN7XS0", title: Human direction and ATProto CLI guidance integration }
---

# ATProto CLI sessions

- Check version and identity with `atgc --version` and `atgc auth status`;
  login interactively with `atgc auth login <handle>`. Local status alone is
  not proof remote access works. Different `tg` tools exist; identify the
  binary before using its recipes. Never expose tokens or passwords.
  [CLI guide](/atproto/cli.md).
