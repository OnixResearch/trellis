## Why

Zircon has a small, sharp kernel object model. User space does not talk to
objects directly; it talks through handles. Rights live on handles, not on
objects. Object state is surfaced as signals. Waiting is signal-driven.
Channels move both messages and handles. Futexes cover the contended path for
blocking synchronization.

This repo already has pieces of that world: generic signal-set algebra in
`src/signal.rs`, reference-count reasoning in `src/ref_count.rs`, a simple
futex state machine in `src/futex_state.rs`, and process blocking/run-state
logic in `src/process_state.rs`. What is missing is Zircon-specific
composition: rights attenuation on duplicate/replace, in-transit handle
lifetime during channel transfer, peer-closed signal propagation, and
wait/wake/requeue rules that line up with Zircon's primitives.

A verified model of those primitives would give downstream users a reusable
reference for capability-style handle semantics, signal-driven waiting, and
microkernel IPC/synchronization rules without needing a full Fuchsia kernel
model.

## What Changes

- **zircon_rights**: Rights-mask primitives for containment, attenuation,
  and operation checks (`duplicate`, `transfer`, `wait`, `signal`,
  `signal_peer`).
- **zircon_handle**: Handle/object model with stable object identity
  (`koid`), per-handle rights, and ownership state (`bound`, `in_transit`,
  `closed`). Proves duplicate/replace preserve object identity and only drop
  rights.
- **zircon_wait**: Wait-readiness predicates over Zircon signal masks,
  including single-object wait and wait-set evaluation.
- **zircon_event**: Event and eventpair semantics, including user-signal
  mutation and `PEER_CLOSED` propagation.
- **zircon_channel**: Datagram channel endpoints with readable/
  peer-closed signal rules, handle transfer, and in-flight-handle discard
  on peer close.
- **zircon_futex**: Futex wait/wake/requeue logic that refines the existing
  `futex_state` model into a multi-waiter Zircon-style primitive.
- **Cross-module properties**: Proofs linking rights to operations,
  channel transfer to handle lifetime, eventpair state to signal algebra,
  and futex wake/requeue to blocked/ready task transitions.

## Capabilities

### New Capabilities

- `zircon-rights.attenuation`: Verified rights reduction without privilege gain
- `zircon-handle.lifecycle`: Verified bound/in-transit/closed handle state
- `zircon-handle.identity`: Verified stable object identity across handle ops
- `zircon-wait.readiness`: Verified wait satisfaction from signal masks
- `zircon-event.peer-closed`: Verified eventpair peer-close propagation
- `zircon-channel.transfer`: Verified handle transfer through channels
- `zircon-channel.discard`: Verified unread in-flight handles close on peer loss
- `zircon-futex.wait-wake`: Verified compare/wait/wake behavior
- `zircon-futex.requeue`: Verified waiter-preserving requeue semantics

## Impact

- **Files**: New `src/zircon_*.rs` modules, `src/lib.rs` exports,
  `src/properties.rs` additions, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for rights masks, handle
  transitions, wait predicates, event/eventpair state, channel message
  transfer, and futex requeue logic
- **Dependencies**: Reuses existing `signal`, `ref_count`, `futex_state`,
  and `process_state` modules; no new runtime deps expected
- **Testing**: Verus proofs, unit tests for concrete scenarios, and proptest
  model oracles for channel transfer and futex wake/requeue sequences

## Out of Scope

- Raw syscall ABI details such as exact status codes or handle integer values
- VMOs, VMARs, pagers, and address-space management
- Job/process/thread creation trees beyond the blocking/ready state needed
  for wait/futex interaction
- Ports, packet delivery, and async wait registration
- Full scheduler modeling beyond existing deadline/fair queue primitives
- Driver, exception, or resource-object semantics
