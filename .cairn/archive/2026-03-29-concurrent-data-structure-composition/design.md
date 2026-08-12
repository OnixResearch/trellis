## Context

The concurrency modules model protocol state machines (lock modes,
sequence counters, hazard arrays). The data structure modules model
structural invariants (ring buffer wf, sorted keys, bit positions).
Both are verified individually. The gap: no proofs that protocol
transitions preserve structural invariants, or that protocol
combinations (e.g., 2PL + lock ordering) compose without contradiction.

## Goals / Non-Goals

**Goals:**
- Prove protocol transitions preserve data structure invariants
- Prove protocol combinations compose correctly
- All proofs in a single module using existing specs (no new exec fns)

**Non-Goals:**
- Not modeling actual concurrent execution (interleavings, linearizability)
- Not adding runtime concurrency primitives
- Not modifying existing modules

## Decisions

### 1. Proof-only module

**Choice:** Pure proof fns calling existing spec fns across modules
**Rationale:** The interesting properties are relationships between
specs — no new runtime code needed. Same pattern as consensus_path.rs.
**Alternative:** Adding methods to existing structs — rejected because
it couples the modules.

### 2. Seven composition groups

**Choice:** Group proofs by which pair of modules they connect:
1. rwlock × ring_buffer (invariant preservation)
2. seqlock × monotonic (version consistency)
3. cas_loop × bitwise (ABA pack/unpack safety)
4. hazard_pointer × rcu (reclamation agreement)
5. two_phase_lock × lock_ordering (protocol composition)
6. ticket_lock × monotonic (FIFO fairness)
7. bounded_mpmc × ring_buffer (capacity agreement)

**Rationale:** Mirrors the cross-module pattern in properties.rs but
focused on concurrency × data structure pairs.

### 3. Proptest scenario oracles

**Choice:** Each group gets a proptest that simulates a multi-step
scenario (acquire → operate → release) and checks invariants hold
throughout.
**Rationale:** Proofs verify the mathematical relationships; proptests
verify the scenario logic matches realistic usage patterns.

## Risks / Trade-offs

**[Z3 trigger matching across modules]** → Use explicit assert hints
to bridge spec definitions, same pattern as consensus_path.rs.

**[Spec visibility]** → All referenced specs must be `pub open spec fn`.
The existing modules already follow this convention.
