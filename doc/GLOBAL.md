# Documentation

Knowledge lives in domain modules: `README.md` is canonical, `SKILL.md` is an
optional relative symlink for on-demand loading, and `GLOBAL.md` is the module's
small ambient contribution. `design/<topic>/` is for building, exploring, and
learning; `doc/<topic>/` is stable user-facing knowledge. Promotion is synthesis,
not moving or deleting evidence.

In upstream-owned repositories, fit upstream documentation conventions when
work is intended upstream. Otherwise keep workspace-specific knowledge in one
domain subtree so it remains easy to filter and carry across upstream changes.

Give durable sections explicit semantic anchors. Local anchors omit the beads
project prefix; the qualified knowledge ID restores it. Tickets coordinate work
lifecycle and may forward-anchor unwritten knowledge, but documents and sections
do not require ticket parity. Cross-references explain why their targets matter.

For waves, evidence classes, ticket-anchor details, promotion, history, and
opt-in research patterns, load `/home/rektide/src/rekon/doc/SKILL.md`.
