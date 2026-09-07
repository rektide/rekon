---
type: ResumptionBrief
title: NAN restart — what is done and what to test-drive next
description: Source-grounded orientation and paste-ready commissions for human-launched NAN sessions, distinguishing existing code verification, practice trials, and future hwsim integration.
resource: /evidence/case-stories/nan-lastact/resume0.gpt6a.md
tags: [nan, resumption, digest, remake, test-drive, provenance]
status: draft
generated: { by: "model:gpt-6-astra", at: 2026-09-06 }
extensions:
  ticket: rekon-phoenix
  session: urn:opencode:session:ses_f86f757c0ffe5G5aHU26e2evOL
  scope: Rekon-authored handoff; no new NAN session or product ticket created
sources:
  - { resource: "https://github.com/rektide/nan-rs/blob/fc9e29fbf738d83bba095b9ac1a797657aa9d5e8/.design/burn-b.gpt56s.md", title: Burn B report at the current inspected NAN tip }
  - { resource: "file:///home/rektide/src/nan-rs/.design/burn-a.gpt56s.md", title: Burn A envelope and ServiceName }
  - { resource: "file:///home/rektide/src/nan-rs/tests/passive_subscription.rs", title: Complete-frame fake-input tracer }
  - { resource: "file:///home/rektide/src/nan-rs/src/lib.rs", title: Current protocol and discovery exports }
  - { resource: /evidence/case-stories/nan-lastact/genesis0.gpt6a.md, title: NAN-origin session and move into Rekon }
---

# Where NAN was left

**The implementation tracer already exists and was reported tested. The new
Rekon digest/remake practices have not been exercised against NAN.** This session
moved to Rekon before that assessment happened.

Read-only inspection on 2026-09-06 found both working copies clean:

| Owner | Inspected tip | What the recent work established |
|---|---|---|
| `~/src/nan-rs` | `fc9e29fbf738d83bba095b9ac1a797657aa9d5e8` | Burn A added the complete management envelope and checked ServiceName derivation. Burn B added the deterministic, bounded passive-subscription owner and complete-frame fake-input tracer, followed by application-neutral fixture corrections. |
| `~/src/wl-nl80211-nan` | `25fa0c9352941cdff5aade70fab3414ab2ff7efa` | The Linux transport line ends with dependency-ordered data-path example messages and concurrent event draining; preceding work includes typed frame metadata and fixture-backed channel/capability views. |

NAN's current `src/lib.rs` exports `protocol` and `discovery`; its only declared
crate dependency is `sha2`. The fake-input integration test feeds complete
management frames through the real codec and discovery owner. This is not a
live Linux receive path or a radio test disguised by a test name.

The [Burn B verification record](https://github.com/rektide/nan-rs/blob/fc9e29fbf738d83bba095b9ac1a797657aa9d5e8/.design/burn-b.gpt56s.md#L610-L637)
reports **76 passing tests**, clean formatting and clippy, docs generation,
shell checks, and replay of 12 hostap-oracle scenarios. Those are the earlier
run's receipts; this Rekon session has not rerun Rust checks or oracle builds.

The [future Burn C boundary](https://github.com/rektide/nan-rs/blob/fc9e29fbf738d83bba095b9ac1a797657aa9d5e8/.design/burn-b.gpt56s.md#L694-L738)
is explicit: two-radio hwsim execution, a typed Linux receive path into the
deterministic owner, and isolation/setup/teardown evidence remain future work.
Publish APIs, active Subscribe, follow-up, Availability/ULW, NDP, security, and
application integration are not delivered by Burn B.

## Suggested first new session: digest and baseline exercise

Start or fork a session in **`~/src/nan-rs`** and paste:

```text
Resume the NAN effort and test-drive Rekon's digest practice. Work in nan-rs;
read its current AGENTS.md and /home/rektide/src/rekon/digest/README.md.
This follows Rekon session ses_f86f757c0ffe5G5aHU26e2evOL, which began here
but moved to Rekon to define the practices under rekon-phoenix.

First record your actual session ID, working directory, source revisions, and
this predecessor reference in your report. Do not invent an explicit message
bookmark or mark unless you have a real tool/receipt for it. Do not move the
session back to Rekon: this is the project application.

Give me the present architecture, recent practical differences, active intent
fronts, and the next meaningful decision. Start with .design/burn-a.gpt56s.md,
.design/burn-b.gpt56s.md, the current sources/tests, and tools/README.md.
Read older balance/library/improve plans as lineage, not current completion
claims. Inspect ~/src/wl-nl80211-nan read-only where a Linux boundary matters.
Honor the separately declared kernel and hostap AUTHORITY pins; oracle-generated
bytes are not captured radio traffic or normative specification proof.

You may run the existing offline baseline checks:
  cargo fmt --check
  cargo test --offline --locked
  cargo clippy --offline --locked --all-targets -- -D warnings
Record unavailable tools/dependencies as blockers rather than installing them.
Do not rewrite code, change interfaces, rebase, push, install/configure hwsim,
create product tickets, or start the next implementation burn.

Write and promptly commit .design/digest0.<your-actual-model>.md. Distinguish
observed checks from prior reports. Also evaluate whether the digest practice
helped recover direction: this is a practice trial, not only a code report.
Offer a small choice of next courses, including no-change where justified.
```

The baseline exercise is optional if orientation alone is the immediate goal;
remove those check permissions in that case. Running the existing tests again
would produce a new environment receipt, not a new capability.

## A second, separate session: paper remake

Use the first digest as an input, then paste:

```text
Test-drive /home/rektide/src/rekon/remake/README.md and its sequence/form0.gpt6a.md
against the currently implemented NAN effort. Read the preceding digest and
pin your own source and landing bases; recover this activity's lineage from
/home/rektide/src/rekon/evidence/case-stories/nan-lastact/resume0.gpt6a.md.

Lead with a defensible patch-landing order for the implemented capability set,
not an implementation of the whole historical roadmap. Keep nan-rs protocol/
discovery policy separate from wl-nl80211-nan Linux ABI transport. Distinguish
learning order, code/contract dependencies, and assessment checkpoints; keep
owner-specific series and their rendezvous visible. Account for late fixes and
application-neutrality corrections without modifying actual history.

For each consequential prefix, explain its new capability, active invariants,
state ownership, high-level concern, and required evidence. Label unbuilt
prefixes proposed; a green current tip is not proof the reconstructed prefixes
compile. Use earlier check receipts without rerunning them solely for reassurance.

Write and promptly commit .design/remake0.<your-actual-model>.md. No code edits,
history surgery, actual patch replay, radio setup, or new product tickets.
Evaluate the method itself: did this give us a clearer landing decision, or
only another long document? Return open choices before implementing anything.
```

This second session depends on the digest if it is commissioned that way. Do
not launch it as an independent reader and simultaneously claim it consumed
the first session's conclusions. The human can fork and sequence the sessions.

## If the aim is product progress instead

Ask for a **Burn C feasibility and owner-seam pitch**, not “finish NAN.” It
should establish the available kernel/hwsim capabilities, typed complete-frame
ingress, resource ownership, and an isolated test/teardown plan. It should
propose the smallest Linux-to-`DiscoveryEngine` seam before any public generic
transport framework. Live radio/namespace changes and implementation need their
own explicit permission.

## What has and has not happened here

- Existing Burn A/B artifacts and current source boundaries were inspected.
- Fresh read-only source-tip receipts were taken for both workspaces.
- No Rust tests, hwsim setup, new NAN session, NAN ticket, or product edit was
  performed by this Rekon handoff.
- `rekon-phoenix` owns the practice experiment's lineage. Future product work
  should use its actual project owner when the human commissions it, not
  silently turn Rekon method tickets into NAN implementation tickets.
