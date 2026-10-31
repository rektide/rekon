---
id: forge-publication
type: ContextFragment
title: Forge publication
description: Defaults and checks for publishing repositories.
resource: /forge/GLOBAL.md
order: 240
source: /forge/publication.md
status: draft
sources:
  - { resource: "opencode://ses_e6c505360ffeVFenH2VYDN7XS0", title: Human defaults and forge guidance integration }
---

# Forge publication

- Default to public repositories, `main`, GitHub `rektide`, and SSH
  (`git@github.com:rektide/<name>`). Confirm the name, account, exact committed
  tip, and existing remotes before creation or operator-directed push.
- Tangled usually uses `jauntywunderkind.bsky.social` and
  `tngl:<handle>/<name>`; OAuth login and SSH push keys are separate. Check
  client/API currency: current Tangled uses `org.tangled.*`; old recipes may
  target legacy endpoints. [Publication guide](/forge/publication.md).
