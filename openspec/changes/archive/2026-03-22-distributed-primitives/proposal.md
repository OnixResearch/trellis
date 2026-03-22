## Why

The library has strong coverage of arithmetic, data structures, and
Raft-adjacent primitives (quorum, epoch, log, monotonic). But distributed
systems need more than leader election mechanics. Causality tracking,
conflict-free replication, stale-write prevention, lease safety, protocol
state machines, and failure isolation are where production systems actually
break.

These primitives share a trait: their correctness properties are algebraic
(semilattice laws, partial orders, state machine invariants) and map
directly to Verus specs. A vector clock merge is a join-semilattice
operation. A G-Counter merge is commutative, associative, idempotent.
A fencing token is a monotonic sequence. A lease pair is a
non-overlapping interval constraint. A two-phase commit decision is a
universal quantifier over participant votes. A circuit breaker is a
finite state machine with guarded transitions.

None of this requires new Verus techniques — the existing patterns
(spec fn + exec fn + ensures + proof fn) handle all of it.

## What Changes

Fourteen new modules across four tiers, plus cross-module properties
linking them to existing modules:

**Tier 1 — Causality and conflict resolution**

- **`vector_clock`** — Lamport-style vector clocks over fixed-size
  arrays. Partial ordering (happens-before), concurrent detection,
  merge via pointwise max. Semilattice proofs: associativity,
  commutativity, idempotency of merge.

- **`crdt`** — Conflict-free replicated data types. G-Counter
  (grow-only), PN-Counter (positive-negative), LWW-Register
  (last-writer-wins by timestamp). Merge laws for each type.

- **`ballot`** — Vote tracking for leader election. Single-vote-per-term,
  quorum-based winner determination, at-most-one-winner proof using
  quorum intersection.

- **`fencing`** — Monotonic fencing tokens for stale-write prevention.
  Token issuance, validation, stale detection. No valid write is ever
  rejected, no stale write is ever accepted.

**Tier 2 — Protocol safety**

- **`lease`** — Time-bounded exclusive access. Grant, renew, revoke.
  Disjointness proof: two sequential leases for the same resource
  cannot both be valid at any time t. Builds on expiration + interval.

- **`two_phase`** — Two-phase commit coordinator logic. Participant
  vote collection, commit/abort decision. Safety: commit requires
  unanimous yes. Abort is always safe. No transaction observes both
  commit and abort.

- **`circuit_breaker`** — Failure isolation state machine.
  Closed → open → half-open → closed transitions with failure
  counting. No request passes in open state. Half-open admits
  exactly one probe.

- **`consistent_hash`** — Ring-based key-to-node assignment. Every
  key maps to exactly one node. Adding a node moves keys from at most
  one neighbor. Removing moves keys to exactly one neighbor.

- **`backoff`** — Bounded exponential backoff. Growth is
  `min(base * 2^attempt, cap)`. No overflow. Jitter stays within
  `[0, backoff]`.

**Tier 3 — Advanced**

- **`merkle`** — Hash-based inclusion proofs. Sibling path from leaf
  to root recomputes root hash. Proof length = log2(tree size).
  Builds on chain module's hash axiom.

- **`gossip`** — Epidemic broadcast infection bounds. After k rounds
  with fanout f, infection count ≥ min(n, f^k). Convergence in
  O(log n) rounds.

- **`bloom`** — Probabilistic set membership. No false negatives
  (insert then query always returns true). Membership is monotonic
  (insert-only, never reverts). False positive bound as integer
  approximation.

**Tier 4 — Distributed ordering**

- **`causal_order`** — Causal delivery predicate. A message m1
  causally precedes m2 implies m1 is delivered before m2 at every
  correct process. Uses vector_clock for ordering.

- **`snapshot`** — Consistent snapshot predicate. A cut is consistent
  if no message crosses the cut boundary (sent after, received before).
  Consistent cuts form a lattice.

Cross-module properties linking new modules to existing ones:

- `vector_clock::merge` ↔ `ordering::max` (pointwise)
- `ballot::winner` → `quorum::is_quorum` (election safety)
- `fencing::is_valid` → `monotonic::is_advance` (token ordering)
- `lease::disjoint` → `interval::overlaps` (time intervals)
- `two_phase::can_commit` ↔ `collections::find_first` (first no-vote)
- `backoff::compute` ↔ `integer::next_power_of_two` (exponential)
- `merkle::proof_length` ↔ `integer::log2` (tree depth)
- `bloom::no_false_negatives` → `bitmap::get` (bit membership)

## Capabilities

### New Capabilities

- `causality-tracking`: Vector clocks with happens-before partial order
  and semilattice merge
- `crdt-merge-laws`: Machine-checked commutativity, associativity,
  idempotency for G-Counter, PN-Counter, LWW-Register
- `election-safety`: At most one leader per term via quorum intersection
- `stale-write-prevention`: Fencing tokens with monotonic ordering proof
- `lease-disjointness`: Time-bounded exclusive access with overlap proof
- `two-phase-safety`: Commit requires unanimity, abort is always safe
- `circuit-breaker-invariants`: Verified state machine transitions
- `consistent-hash-coverage`: Every key assigned, minimal disruption on
  membership change
- `backoff-bounds`: Exponential growth with overflow-safe cap
- `merkle-inclusion`: Hash path verification with depth bound
- `gossip-convergence`: Infection spread lower bound
- `bloom-no-false-negatives`: Insert-then-query correctness
- `causal-delivery`: Delivery ordering respects causality
- `consistent-snapshots`: Cut consistency as lattice property

### Modified Capabilities

- `cross-module-properties`: ~15 new properties linking distributed
  modules to existing arithmetic, ordering, interval, quorum modules

## Impact

- **Files**: 14 new `src/*.rs` modules, `src/properties.rs` additions,
  `src/lib.rs` module declarations, `docs/requirements.md` additions
- **APIs**: ~80–100 new pub fns across 14 modules
- **Dependencies**: None — pure logic using existing vstd
- **Testing**: Verus verification + proptest property tests + model oracles
