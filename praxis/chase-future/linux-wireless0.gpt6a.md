---
type: PracticeCase
title: Linux wireless timing — implement the reachable seam
description: The motivating correction toward immediate Rust netlink work, retaining a broader timing ambition and distinct hardware-gated options.
resource: /praxis/chase-future/linux-wireless0.gpt6a.md
tags: [praxis, linux, openwrt, wifi, netlink, rust, evidence]
status: draft
generated: { by: "model:openai/gpt-6-astra", at: 2026-09-06 }
verified: { by: none, at: never }
stale_after: 2026-10-06
extensions:
  work_anchor: "wifi-ptp-now:wiptpn-modern-praxis"
  evidence_scope: Source reading and relayed user direction; no build, radio, clock, or audio validation.
sources:
  - {
      resource: "urn:opencode:session:ses_f86b2c89bffeXMeyFPJ6MJg2Cx",
      title: Delegated user correction and new deployment context,
    }
  - {
      resource: "file:///home/rektide/src/wifi-ptp-now/design/modernization/brief0.gpt6a.md",
      title: Original investigation brief,
    }
  - {
      resource: "file:///home/rektide/src/wifi-ptp-now/design/modernization/linux-interfaces0.gpt6a.md",
      title: Separate Linux wireless timing capabilities,
    }
  - {
      resource: "file:///home/rektide/src/wl-nl80211-nan/src/handle.rs",
      title: Existing Rust request surface,
    }
  - {
      resource: "file:///home/rektide/src/nan-rs/src/lib.rs",
      title: Transport-independent NAN protocol owner,
    }
---

# Implement the reachable seam

This is a **motivating case and proposed application**, not a successful hardware
experiment or a second Wi-Fi roadmap. The desired help is concrete movement
toward **Linux/OpenWrt Wi-Fi timing → PTP → AES67**, with early steps chosen for
the measurement and exploration they open. PTP is Precision Time Protocol;
AES67 is the networked-audio interoperability destination. Achieving one timing
interface is not proof of that whole destination.

The [initial Wi-Fi investigation brief](file:///home/rektide/src/wifi-ptp-now/design/modernization/brief0.gpt6a.md)
and [Linux interface study](file:///home/rektide/src/wifi-ptp-now/design/modernization/linux-interfaces0.gpt6a.md)
establish the technical starting questions. The user subsequently names
**RAX120 v2, OpenWrt, an all-Linux setting, `~/src/nan-rs`, and
`~/src/wl-nl80211-nan`** as relevant context. Those additions are relayed human
context, not facts inferred from the older brief or a newly inspected router.
This pass has not established its installed image, radio/firmware capabilities,
or timing behavior. OpenWrt and Linux source access are opportunities to work;
they do not remove firmware or hardware constraints.

The important correction is not “be less careful about hardware.” It is:
**we already know enough to write useful Rust netlink code; do that now rather
than wait for certainty about every possible synchronization route.** Chase the
current upstream development line instead of defining the opportunity by an
older installed baseline. Keep unknown end-to-end feasibility visible.

<a id="source-basis"></a>

## What was actually read

| Source basis                                                             | Narrow finding                                                                                                                                                                                                                                                                                        | Consequence                                                                                                                                                             |
| ------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Linux `/usr/local/src/linux`, `66498c75b4f8017f62d720d9b59675bdf3abce91` | The [nl80211 peer-timestamp handler](https://github.com/torvalds/linux/blob/66498c75b4f8017f62d720d9b59675bdf3abce91/net/wireless/nl80211.c#L19230-L19250) accepts a peer/enable request and rejects unavailable peer timestamping.                                                                   | A concrete control-plane request can be implemented independently of proving device support. This is the exact inspected local basis, not a fresh remote-tip assertion. |
| Local `wl-nl80211-nan` line, `25fa0c9352941cdff5aade70fab3414ab2ff7efa`  | [Request handles](file:///home/rektide/src/wl-nl80211-nan/src/handle.rs#L25-L74) expose interface/PHY queries and generic requests; [frame-event parsers](file:///home/rektide/src/wl-nl80211-nan/src/event.rs#L327-L408) preserve optional TX/RX timestamps, frame data, attributes, and TX cookies. | Reuse existing transport and observation machinery. Do not start another netlink stack or pretend all timing-specific conveniences already exist.                       |
| Local `nan-rs`, `fc9e29fbf738d83bba095b9ac1a797657aa9d5e8`               | [The library](file:///home/rektide/src/nan-rs/src/lib.rs#L1-L7) and [manifest](file:///home/rektide/src/nan-rs/Cargo.toml) expose transport-independent NAN protocol/discovery code; they do not make netlink its dependency.                                                                         | NAN protocol work is a neighboring reuse opportunity, not a mandatory dependency of Wi-Fi PTP or evidence that NAN provides the required application clock.             |

The transport's upstream is [`rust-netlink/wl-nl80211`](https://github.com/rust-netlink/wl-nl80211);
the inspected `-nan` workspace is a local development line, not a claim about
published upstream or crate coverage. The protocol repository is
[`rektide/nan-rs`](https://github.com/rektide/nan-rs). Local file links above
identify the actual inspected development artifacts; their pinned revisions
identify the evidence basis. The new Wi-Fi research documents likewise live in
the local `wifi-ptp-now` checkout; this note does not invent published URLs for
them in the historical donor repository.

The [kernel ABI description](https://github.com/torvalds/linux/blob/66498c75b4f8017f62d720d9b59675bdf3abce91/include/uapi/linux/nl80211.h#L1324-L1332)
names **Timing Measurement and Fine Timing Measurement (TM/FTM)** management
frames. Its [timestamp definitions](https://github.com/torvalds/linux/blob/66498c75b4f8017f62d720d9b59675bdf3abce91/include/uapi/linux/nl80211.h#L2928-L2953)
name device-clock values and reset conditions. Therefore:

**An encodable request ≠ a supported device operation ≠ an observed hardware
timestamp ≠ a PTP hardware clock (PHC) ≠ hardware-timestamped ordinary PTP
traffic ≠ demonstrated AES67 interoperability.** These are separate capability
claims, not milestones that automatically imply one another.

<a id="selected-slice"></a>

## Selected priority and a concrete slice to commission

**User-selected priority:** immediate Rust implementation of known netlink
surfaces. **Proposed coherent slice:** capability inventory plus timing-control
request construction and faithful observation output, reusing the existing
transport. Keep shared Linux ABI improvements with its owner and the
Wi-Fi-timing-specific consumer with the Wi-Fi effort.

The slice should let us query relevant advertised capabilities, construct and
test the peer timestamp request, and preserve actual management timing events
when a supported system supplies them. Distinguish a kernel rejection, an absent
timestamp, an observation gap, and no eligible hardware. Retain source, device,
peer/frame correlation, clock-domain facts when known, and unmodified raw values.
A host receive time must not be renamed a radio cross-timestamp.

Request/attribute fixtures and decoder tests can establish userspace behavior
without a radio. Device operation and event meaning need separate target
evidence. Building the client does not authorize issuing state-changing commands
on a live router, flashing OpenWrt, loading a driver, or disciplining a host clock.
The product owner should record its exact commissioned slice and execution scope;
this Rekon note is not a substitute for that work record.

Why this investment, rather than the extremes?

- **Less:** another static capability table costs little but does not create a
  reusable way to interrogate the actual systems as they become available.
- **Selected priority:** a Rust control/observation slice adds implementation and
  test cost, but opens repeatable measurements across devices and several timing
  routes while reusing existing machinery.
- **More:** a complete ath11k PHC/socket timestamp bridge immediately would make
  larger driver, firmware-semantics, clock-continuity, and packet-correlation
  bets. A generic abstraction for all such routes would multiply those bets.
  Neither is required to make the useful userspace slice real.

<a id="preserved-courses"></a>

## Courses worth retaining beside that slice

These are **preserved options**, not accepted follow-on tickets or a complete
hardware feasibility assessment. Product evidence and changing user priorities
should maintain their disposition. Their interest does not depend solely on the
selected route failing.

| Course and distinct payoff                                                       | Evidence / unresolved question                                                                                                                            | Exploration or preservation cost                                                                                                                                          | Revisit trigger                                                                                           |
| -------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| TM/FTM management timing as a measurement path                                   | The interface study identifies a concrete iwlwifi precedent; target peer/driver/firmware support is not established. It is not ordinary UDP PTP.          | Existing request/event machinery helps; still needs suitable peers, protocol handling, and measured timestamp semantics. Keep a linked option, not an unneeded backend.   | An accessible target advertises support and returns attributable timing events.                           |
| Relative radio timing through receive observations or passive driver diagnostics | Candidate radio observations may help compare clocks without proving UTC/TAI, two-way delay, or a stable PHC.                                             | Raw capture/analysis is separable; custom driver diagnostics add test-build and target-access cost. Preserve provenance before choosing conversion machinery.             | A repeatable observation establishes its reference point, domain, reset behavior, and useful uncertainty. |
| Ordinary-packet hardware PTP on Wi-Fi                                            | The interface study separates PHC, RX/TX association, and socket plumbing; no complete ath11k path is established here.                                   | Potential shared mac80211 plus driver work, firmware evidence, and end-to-end validation. Preserve the contract and open questions rather than prebuild the whole bridge. | Verified readable clock and packet-event paths justify a concrete standard-interface integration.         |
| Software timestamping and a wired-reference/hybrid topology                      | These can supply a comparison or a different route toward the audio goal, but wireless software TX delivery itself needs verification in the cited study. | Socket probes and reference equipment/topology setup; fewer radio-clock assumptions, but different deployment and accuracy tradeoffs.                                     | A measured software baseline or accessible wired reference changes the cost/opportunity comparison.       |
| NAN discovery/protocol reuse                                                     | Existing neighboring Rust work supplies discovery and frame-handling knowledge, not a demonstrated application synchronization clock.                     | Evaluate a specific reusable boundary; importing an entire session/protocol stack would cost more. A reference may preserve enough.                                       | Peer discovery or a shared management-frame consumer becomes an actual need.                              |

## What this case has and has not established

This pass establishes a source-grounded reason to prioritize implementable
userspace work and a durable account of the user's correction. It has **not**
built the proposed client, tested either Rust project, measured a radio, validated
RAX120 v2/OpenWrt support, adjusted a clock, or demonstrated PTP/AES67 performance.
No numerical accuracy or success claim follows from the source reading.

The next case-story evidence should come from the actual commissioned action:
what became usable or measurable, which route became cheaper or untenable, and
whether preserving alternatives improved the next decision. The general lesson
to test is [Chase the future](/praxis/chase-future/chase-future0.gpt6a.md): **make
the reachable seam real, without borrowing the destination's claims.**

## Cross-references

- [Praxis entry point](/praxis/README.md#chase-the-future) **generalizes the
  commission** without importing radio-specific requirements into all Rekon work.
- [Linux interfaces study](file:///home/rektide/src/wifi-ptp-now/design/modernization/linux-interfaces0.gpt6a.md)
  supplies **technical evidence and limits**, including software timestamping
  verification and virtual-clock reuse conditions; the Wi-Fi repository owns
  revisions to those findings.
- [Original brief](file:///home/rektide/src/wifi-ptp-now/design/modernization/brief0.gpt6a.md)
  records **prior scope and provenance**. This note distinguishes the new user
  context rather than pretending that it was already in that brief.
- [Evidence case stories](/evidence/README.md#evidence-case-stories) supply the
  **evaluation contract**: a useful proposed slice is not yet an observed success.
