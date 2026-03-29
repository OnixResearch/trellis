## Phase 1: RWLock × RingBuffer + Seqlock × Monotonic

- [x] Create src/concurrent_composition.rs with module structure
- [x] Prove: rwlock write-mode mutual exclusion prevents concurrent ring_buffer mutation
- [x] Prove: rwlock read-mode allows concurrent ring_buffer len/is_empty/is_full reads
- [x] Prove: rwlock writer fencing token is monotonic (composes with fencing module)
- [x] Prove: seqlock write cycle increments version (composes with monotonic::is_strict_advance)
- [x] Prove: seqlock valid read ↔ no concurrent write (reader safety)
- [x] Prove: seqlock writer_version monotonically increases across write cycles
- [x] Proptest: rwlock acquire-operate-release scenario preserves invariants
- [x] Proptest: seqlock write-read scenario detects concurrent writes
- [x] Verus clean

## Phase 2: CAS × ABA + Hazard Pointer × RCU

- [x] Prove: CAS pack-succeed-unpack roundtrip recovers original value and bumped generation
- [x] Prove: CAS ABA scenario — same value, different generation detected by pack inequality
- [x] Prove: CAS retry terminates (attempts_remaining decreases)
- [x] Prove: hazard_pointer protected ⟹ !rcu can_free (reclamation agreement)
- [x] Prove: rcu all_quiescent + hazard_pointer all_empty ⟹ everything reclaimable
- [x] Prove: rcu grace period advance composes with hazard_pointer scan
- [x] Proptest: CAS retry loop with ABA detection scenario
- [x] Proptest: hazard protect → rcu advance → reclaim scenario
- [x] Verus clean

## Phase 3: 2PL × Lock Ordering + Ticket Lock + MPMC

- [x] Prove: 2PL growing phase + lock_ordering ascending ⟹ deadlock-free acquisition
- [x] Prove: 2PL shrinking phase permits any release order (ordering constraint relaxes)
- [x] Prove: 2PL phase transition is irreversible (growing → shrinking, never back)
- [x] Prove: ticket_lock take_ticket is strict advance (composes with monotonic)
- [x] Prove: ticket_lock FIFO — earlier ticket served before later ticket
- [x] Prove: bounded_mpmc occupancy bounded by capacity (same as ring_buffer count ≤ cap)
- [x] Prove: bounded_mpmc full/empty agrees with ring_buffer is_full/is_empty at same counts
- [x] Proptest: 2PL + lock ordering multi-lock scenario
- [x] Proptest: MPMC enqueue-dequeue capacity tracking
- [x] Verus clean

## Phase 4: Integration

- [x] Add concurrent_composition to src/lib.rs
- [x] Add requirements to docs/requirements.md
- [x] Verus + tracey clean
