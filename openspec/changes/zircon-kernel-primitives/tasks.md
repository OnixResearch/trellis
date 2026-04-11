## Phase 1: Rights and Handle Identity

- [ ] Create `src/zircon_rights.rs` with rights-mask containment, attenuation, and operation predicates for duplicate/transfer/wait/signal/signal-peer
- [ ] Create `src/zircon_handle.rs` with stable `koid`, per-handle rights, and owner states `bound`, `in_transit`, and `closed`
- [ ] Prove duplicate and replace preserve `koid` and never add rights
- [ ] Prove close and transfer preserve object lifetime rules via `ref_count`-style reasoning
- [ ] Add Tracey annotations and unit tests for rights attenuation and handle state transitions
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean

## Phase 2: Signals, Waiting, Events, and Eventpairs

- [ ] Create `src/zircon_wait.rs` as a Zircon wrapper over `src/signal.rs` for wait readiness and wait-set evaluation
- [ ] Create `src/zircon_event.rs` for event and eventpair state, user-signal mutation, and peer-close derivation
- [ ] Prove wait readiness is exactly non-empty intersection between observed and requested signal masks for waitable objects
- [ ] Prove event user-signal operations are idempotent and eventpair `PEER_CLOSED` follows peer-handle loss
- [ ] Add unit tests and proptests for user signals, wait satisfaction, and peer-close scenarios
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean

## Phase 3: Channels and Handle Transfer

- [ ] Create `src/zircon_channel.rs` with endpoint queues, datagram message semantics, and optional transferred handles
- [ ] Prove channel write/read roundtrip for messages that fit and channel readable/peer-closed signal rules
- [ ] Prove sending a handle moves it to `in_transit`, receiving rebinds it to the destination process, and rights/object identity are preserved
- [ ] Prove closing a destination endpoint discards unread messages and closes all handles still in flight to that endpoint
- [ ] Add model-oracle tests for send/receive/close races and transferred-handle cleanup
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean

## Phase 4: Futex Wait/Wake/Requeue

- [ ] Create `src/zircon_futex.rs` or extend `src/futex_state.rs` with Zircon-style multi-waiter wait/wake/requeue semantics
- [ ] Prove futex wait only blocks when the observed value still matches the expected value
- [ ] Prove wake releases at most the requested number of waiters and waking an empty futex is a no-op
- [ ] Prove requeue conserves total blocked waiters except those explicitly woken and preserves destination waitability
- [ ] Connect futex wake/requeue outcomes to `process_state` blocked->ready transitions
- [ ] Add unit tests and proptests for wake counts, empty wakes, and requeue conservation
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean

## Phase 5: Cross-Module Properties and Finish

- [ ] Add `src/properties.rs` theorems linking rights checks to handle operations, event/eventpair state to `signal.rs`, channel transfer to handle lifetime, and futex unblock to `process_state`
- [ ] Add `pub mod zircon_*;` exports to `src/lib.rs`
- [ ] Add or update main spec text under `openspec/specs/` once implementation stabilizes and the change is ready to sync
- [ ] Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, and `./scripts/verify.sh`
- [ ] Confirm Tracey reports `0 uncovered` and `0 untested` for the finished change
