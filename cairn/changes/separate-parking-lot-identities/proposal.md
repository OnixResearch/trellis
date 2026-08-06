## Why

The parking-lot core passes lock keys and thread identities as adjacent `u64` values. Calls such as `park_waiter(table, key, thread_id)` and `raw_mutex_waiter_accounted(state, table, key, waiter, ...)` compile when those domains are exchanged. Field names make the model readable, but they do not protect public calls.

This change introduces two semantic Rust newtypes in the small, non-Verus parking-lot surface. It improves API safety without changing queue, wake, requeue, or raw-mutex behavior.

## What Changes

- Add `ParkKey` and `ThreadId` as distinct, copyable finite identity types.
- Migrate parking-lot queue records, waiter records, raw-mutex ownership, and relevant public helpers to the correct identity type.
- Keep raw `u64` conversion explicit and keep the two domains non-convertible to each other.
- Preserve all existing queue-validity, waiter-uniqueness, requeue, and raw-mutex state-machine behavior.
- Add positive runtime tests, negative behavior tests, and compile-fail evidence for exchanged identity domains.

## Impact

- **Files**: `src/parking_lot_core_model.rs`, `src/parking_lot_raw_mutex.rs`, focused tests or compile-fail fixtures, `docs/requirements.md`, and the accepted `domain-primitives` Cairn spec.
- **Testing**: focused Cargo tests, compile-fail evidence, full Cargo tests, Clippy, Tracey coverage, and Cairn validation.
- **Compatibility**: public functions that currently accept raw `u64` identities become source-incompatible. Callers must construct the declared identity type explicitly.
- **Claims**: this change provides Rust type separation. It does not add a Verus proof or claim lock implementation, scheduler, fairness, or operating-system correctness.
