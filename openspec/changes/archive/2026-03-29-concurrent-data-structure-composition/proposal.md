## Why

The library has 12+ concurrency modules (cas_loop, rwlock, seqlock,
hazard_pointer, rcu, two_phase_lock, lock_ordering, ticket_lock,
futex_state, spin_backoff, bounded_mpmc, work_stealing) and several
data structure modules (ring_buffer, bitmap, btree_insert/delete).
Each is individually verified. Missing: proofs that the concurrency
protocols compose correctly with data structure invariants.

A rwlock protecting a ring buffer should preserve the ring buffer's
well-formedness invariant. A seqlock guarding a bitmap should guarantee
readers see consistent snapshots. A CAS loop updating a counter should
compose with the ABA-detection pack/unpack to produce correct values.
These compositions are where real bugs hide — the individual pieces
work, but their interaction is where invariants break.

## What Changes

One new module proving cross-module compositions between concurrency
protocols and data structure invariants:

- **`concurrent_composition`** — Proof-only module (no new exec fns).
  Chains existing specs to prove:

  1. **RWLock × RingBuffer**: read-acquire preserves wf, write-release
     after push/pop preserves wf, mutual exclusion prevents concurrent
     mutation
  2. **Seqlock × version monotonicity**: write cycle increments version,
     valid reads see consistent versions, invalid reads are detected
  3. **CAS × ABA detection**: pack-CAS-unpack roundtrip, generation
     distinguishes ABA scenarios, retry loop terminates
  4. **Hazard pointer × RCU agreement**: both reclamation protocols
     agree on "safe to free" when all threads are quiescent
  5. **Two-phase lock × lock ordering**: 2PL growing phase respects
     lock order, shrinking phase can release in any order
  6. **Ticket lock × fairness**: FIFO ordering, bounded wait
  7. **Bounded MPMC × ring buffer capacity**: occupancy tracks count,
     full/empty agree

## Capabilities

### New Capabilities
- `concurrent-rwlock-invariant`: RWLock protocol preserves data structure well-formedness
- `concurrent-seqlock-consistency`: Seqlock guarantees consistent snapshot reads
- `concurrent-cas-aba-safety`: CAS+ABA pack/unpack composition is correct
- `concurrent-reclamation-agreement`: Hazard pointer and RCU agree on safe reclamation
- `concurrent-2pl-ordering`: Two-phase locking composes with lock ordering
- `concurrent-queue-capacity`: MPMC queue occupancy matches ring buffer count

### Modified Capabilities
- None — composes existing module specs

## Impact

- **Files**: 1 new src/ module (concurrent_composition.rs), src/lib.rs addition
- **APIs**: ~25 pub proof fns, no new exec fns
- **Dependencies**: rwlock, ring_buffer, seqlock, cas_loop, hazard_pointer, rcu,
  two_phase_lock, lock_ordering, ticket_lock, bounded_mpmc, work_stealing specs
- **Testing**: Verus verification (all proof fns) + proptest scenario oracles
