## Why

Surelock is an interesting target for this repo because it claims a sharp,
useful property: if the code compiles, lock acquisition cannot deadlock.
That claim rests on a small set of composable primitives rather than on a
whole-program scheduler model:

- stable, unique lock identities (`LockId`)
- a total order on lock levels (`Level<N>` / `LockAfter`)
- canonical same-level acquisition (`LockSet`)
- a consumed-and-re-emitted scope token (`MutexKey`) that only moves upward

Those are the kinds of invariants this repo already verifies well. A
machine-checked model of surelock's core rules would give us a reference for
reasoning about deadlock-freedom by construction, and it would add a new
concurrency domain to the library that is not covered by the existing queue,
quorum, cache-coherence, or replication modules.

The external surelock crate also makes a useful distinction between what must
be enforced dynamically and what can be enforced statically. Bringing the core
of that design into verified-logic would let us prove the ordering theorems
once and keep the ergonomic Rust API questions separate.

## What Changes

- **surelock_id**: Verified lock-identity primitives for unique, monotonic
  lock IDs and canonical ordering by ID.
- **surelock_level**: Verified level-order primitives for `Level<N>` style
  ordering, `lock_after`, and `new_higher` level derivation from one or two
  parents.
- **surelock_key**: Verified scope-token state transitions that model
  consumed-and-re-emitted keys, upward-only acquisition, and subscope
  inheritance.
- **surelock_lockset**: Verified same-level lock-set preparation that sorts by
  lock ID, rejects duplicates, and preserves the caller's logical lock list
  while using a canonical acquisition plan.
- **surelock_deadlock**: A proof layer connecting the primitives above to a
  no-circular-wait theorem for all admitted acquisition traces in the core
  model.

## Capabilities

### New Capabilities

- `surelock-lock-id`: Verified unique lock identity and canonical ID ordering
- `surelock-level-order`: Verified total level ordering and `new_higher`
  derivation
- `surelock-key-flow`: Verified upward-only key progression across lock scopes
- `surelock-lockset`: Verified duplicate-free canonical same-level acquisition
- `surelock-deadlock-freedom`: Verified absence of circular wait in the core
  surelock model

## Impact

- **Files**: New `src/surelock_*.rs` modules, `src/lib.rs` exports, a new
  `openspec/changes/surelock-primitives/` change, and requirement markers in
  the new modules
- **APIs**: New public spec/exec/proof functions for lock IDs, levels, lock
  sets, key transitions, and deadlock-freedom properties
- **Dependencies**: No new runtime dependencies beyond the existing Verus/
  `vstd` setup
- **Testing**: Verus proofs for the invariants, cargo tests for executable
  helpers, and Tracey coverage for the new requirements

## Out of Scope

- Reproducing surelock's exact public Rust API surface
- Verifying raw mutex backends or blocking behavior
- Modeling thread-local scope uniqueness or borrow-checker errors directly
- Roadmap items from surelock such as `RwLock`, poisoning, or specialized
  bare-metal backends
- Full verification of mixed-level atomic tuple acquisition beyond the core
  same-level lock-set model
