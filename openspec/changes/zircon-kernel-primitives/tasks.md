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

## Phase 5: Sockets and FIFOs

- [ ] Create `src/zircon_socket.rs` with paired byte-stream endpoints, bounded buffers, and short read/write semantics
- [ ] Prove write appends to peer receive buffer and read consumes in order; short reads/writes respect buffer bounds
- [ ] Add write disposition (enable/disable) with `WRITE_DISABLED` and `PEER_WRITE_DISABLED` signals
- [ ] Add configurable read/write threshold signals
- [ ] Prove peer-closed propagation following the peered-object pattern
- [ ] Create `src/zircon_fifo.rs` with fixed-element paired ring buffer using `ring_buffer` concepts
- [ ] Prove element-aligned reads/writes (no partial elements) and bounded capacity
- [ ] Prove FIFO peer-closed and readable/writable signals
- [ ] Add unit tests and proptests for socket stream scenarios and FIFO element alignment
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean

## Phase 6: Ports and Timers

- [ ] Create `src/zircon_port.rs` with packet queue, async wait registration, and signal-to-packet delivery
- [ ] Prove one-shot registration is consumed after first delivery and repeating registration re-arms
- [ ] Prove user-queued packets are delivered in FIFO order
- [ ] Prove async delivery corresponds to wait-readiness from `zircon_wait`
- [ ] Create `src/zircon_timer.rs` with Idle/Armed/Signaled state machine and set/cancel/reset operations
- [ ] Prove signal persistence until cancel/reset and deadline ordering consistency with `deadline_queue`
- [ ] Add unit tests and proptests for port delivery scenarios and timer lifecycle
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean

## Phase 7: Job Containment Hierarchy

- [ ] Create `src/zircon_job.rs` with tree structure (parent-child jobs, job-process membership)
- [ ] Prove containment invariants: unique parent for non-root jobs, unique owning job per process
- [ ] Prove kill propagation: killing a job recursively kills all descendants
- [ ] Prove critical-process semantics: abnormal exit of a critical process kills the owning job
- [ ] Prove parent-lifetime: a job with live descendants remains live even with no handles
- [ ] Add unit tests and proptests for kill propagation and critical-process scenarios
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean

## Phase 8: Cross-Module Properties and Finish

- [ ] Add `src/properties.rs` theorems linking rights checks to handle operations, peered-object signals to `signal.rs`, channel transfer to handle lifetime, port delivery to wait readiness, timer signaling to deadline ordering, job kill to process-state transitions, and futex unblock to `process_state`
- [ ] Add `pub mod zircon_*;` exports to `src/lib.rs` for all eleven Zircon modules
- [ ] Add or update main spec text under `openspec/specs/` once implementation stabilizes and the change is ready to sync
- [ ] Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, and `./scripts/verify.sh`
- [ ] Confirm Tracey reports `0 uncovered` and `0 untested` for the finished change
