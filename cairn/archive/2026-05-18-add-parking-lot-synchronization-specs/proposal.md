## Why

`indexable-inc/parking_lot` concentrates the synchronization behavior Trellis should be able to specify and verify: compact raw lock state machines, parked-thread queues, requeue-based condition variables, fair handoff, timed waits, and deadlock detection. Trellis already has adjacent primitives (`spin_backoff`, `futex_state`, `rwlock`, `ticket_lock`, `lock_ordering`, `surelock_*`, and `blocking_bounded_channel`), but lacks the parking-lot-specific queue/requeue contracts that compose those pieces into lock slow paths.

## What Changes

- Specify a finite parking-lot core model: key-addressed wait queues, park/unpark/requeue operations, uniqueness, and no lost wakeups.
- Specify raw mutex, condition variable, RwLock, fairness/handoff, timed wait, reentrant mutex, Once/OnceLock, raw-lock trait contracts, and deadlock detection requirements inspired by `parking_lot`.
- Keep the work spec-first: requirements describe finite deterministic predicates and proof obligations, not direct OS-thread blocking or unsafe atomic implementations.

## Scope

In scope: finite models, executable admission/checker functions, proof bridges, positive/negative tests, Tracey coverage, and validation evidence for parking-lot-style synchronization semantics.

Out of scope:
- Copying `parking_lot` implementation code or exposing a production replacement lock crate.
- Real OS parking, futex syscalls, thread scheduling, hardware lock elision, or unsafe atomic memory reclamation.
- Benchmark parity with `parking_lot`.

## Verification

Validate this OpenSpec package, then implement the requirements as Trellis modules with focused tests, Verus verification, Tracey coverage, clippy, and repository-level validation evidence.
