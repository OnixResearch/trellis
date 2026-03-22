## Context

verified-logic has 34 modules covering arithmetic, data structures, and
Raft-adjacent consensus primitives. The Verus toolchain, project patterns,
and enforcement scripts are mature. Adding 14 distributed systems modules
follows the same architecture — no new infrastructure needed.

The existing modules provide building blocks that the new modules compose:

- `monotonic` → fencing tokens, ballot term ordering
- `quorum` → ballot election safety (intersection theorem)
- `epoch` → ballot log-up-to-date check
- `interval` → lease disjointness
- `expiration` → lease validity
- `ordering` → vector clock pointwise max
- `bitmap` → bloom filter bit operations
- `chain` → merkle hash axiom
- `integer` → backoff power-of-two, merkle log2

## Goals / Non-Goals

**Goals:**
- Verified spec/exec/proof for each primitive
- Cross-module properties connecting new modules to existing ones
- Runtime property tests as supplementary oracle
- Tracey requirement coverage for every pub fn

**Non-Goals:**
- Networking or I/O — these are pure decision functions
- Parameterized replica counts at the type level — use runtime values
- Real hash functions inside Verus — use uninterpreted function axioms
  (same pattern as `chain` module)
- Floating-point reasoning — gossip convergence and bloom FP rate use
  integer approximations

## Decisions

### 1. Fixed-size Vector Clocks

**Choice:** Vector clocks use a compile-time const generic `N` for the
number of replicas, backed by `[u64; N]` arrays.

**Rationale:** Seq-based variable-length clocks fight Z3 on quantifier
instantiation. Fixed-size arrays let Verus unroll comparisons for small
N and use bounded quantifiers. The const generic keeps it flexible
without dynamic allocation.

**Alternative:** `Seq<u64>` with length invariants. Rejected — quantifier
triggers over Seq indexing are fragile (see slice module experience with
suffix_eq triggers).

**Implementation:** Struct with `pub entries: [u64; N]`. Spec fns operate
on the array directly. Proofs use `assert forall|i: int| 0 <= i < N ==> ...`
with explicit triggers on `entries[i]`.

### 2. CRDTs as Separate Structs

**Choice:** G-Counter, PN-Counter, and LWW-Register are separate structs,
not a trait hierarchy.

**Rationale:** Verus doesn't support trait-based verification well.
Separate structs with independent spec/exec pairs are simpler and each
gets its own proof obligations. The merge laws (commutativity,
associativity, idempotency) are proved per type.

**Alternative:** A `Mergeable` trait with blanket proofs. Rejected —
Verus can't verify trait impls, and the proof structure differs per type
(G-Counter uses pointwise max, PN-Counter uses two G-Counters,
LWW-Register uses timestamp comparison).

**Implementation:** G-Counter mirrors vector_clock (pointwise max merge,
sum for value). PN-Counter wraps two G-Counters. LWW-Register is a
(value, timestamp) pair with max-timestamp merge.

### 3. Ballot Module Separate from Epoch

**Choice:** New `ballot` module for vote tracking, reusing `epoch` and
`quorum` as dependencies rather than extending them.

**Rationale:** Epoch handles (term, index) comparison. Quorum handles
majority thresholds. Neither tracks per-node vote state. The ballot
module adds vote-grant logic, single-vote-per-term enforcement, and the
election safety proof that connects quorum intersection to leader
uniqueness. Keeping it separate follows the single-responsibility pattern.

**Implementation:** `VoteState` struct with `voted_for: Option<u64>` and
`term: u64`. Functions: `can_grant_vote`, `grant_vote`, `has_won_election`.
Election safety proof: if both A and B have quorum votes in term t, then
A == B (by quorum intersection, the shared voter voted for only one).

### 4. Fencing Tokens as Thin Wrapper

**Choice:** Minimal module — `issue_token`, `is_valid_write`,
`is_stale`. Three functions, three proofs.

**Rationale:** Fencing is conceptually monotonic issuance + comparison.
The proofs are: (1) issued tokens are strictly increasing, (2) valid
writes are never rejected, (3) stale writes are always rejected. These
follow directly from `monotonic::is_strict_advance`. The module exists
to name the pattern and state the safety properties explicitly.

**Implementation:** `issue_token(current: u64) -> u64` returns
`current + 1`. `is_valid_write(request_token: u64, fence: u64) -> bool`
returns `request_token >= fence`. `is_stale(token: u64, fence: u64) -> bool`
returns `token < fence`.

### 5. Lease Uses Interval Module

**Choice:** Lease validity is modeled as half-open `[start, expiry)`
intervals, reusing `interval::overlaps` and `interval::contains`.

**Rationale:** The disjointness proof reduces to: if `s_b >= e_a`, then
`interval::overlaps([s_a, e_a), [s_b, e_b))` is false. This is already
proved in the interval module. The lease module adds grant/renew/revoke
semantics on top.

**Implementation:** `Lease` struct with `resource_id: u64`, `start: u64`,
`expiry: u64`, `revoked_at: Option<u64>`. `is_valid(lease, now)` checks
`contains([start, effective_expiry), now)` where `effective_expiry =
min(expiry, revoked_at.unwrap_or(u64::MAX))`.

### 6. Two-Phase Commit as Pure Decision

**Choice:** Model the coordinator's decision function, not the protocol
state machine. Input: array of `Option<bool>` (None = no vote, Some(true)
= yes, Some(false) = no). Output: commit, abort, or pending.

**Rationale:** The protocol's state transitions involve network I/O.
The safety property is purely about the decision: commit iff all voted
yes. Modeling just the decision keeps it verifiable. The state machine
can be layered on top by callers.

**Implementation:** `can_commit(votes: &[Option<bool>]) -> bool` returns
true iff every element is `Some(true)`. `decide(votes) -> Decision`
(Commit/Abort/Pending). Safety proof: `can_commit(v) ==> forall|i| v[i]
== Some(true)`.

### 7. Circuit Breaker State Machine

**Choice:** Encode states as u8 constants (CLOSED=0, OPEN=1, HALF_OPEN=2),
transitions as pure functions.

**Rationale:** Verus can't match on enum variants in spec functions
(same limitation as batch module's FlushDecision). Integer encoding with
named constants works. The state machine has 4 transitions and 2
predicates (allow_request, record_outcome).

**Implementation:** `next_state(current: u8, failure_count: u64,
threshold: u64, elapsed: u64, timeout: u64, probe_succeeded: bool) -> u8`.
Validity proof: output is always in {0, 1, 2}. Transition legality: only
the 4 valid transitions can occur.

### 8. Consistent Hash Ring as Sorted Array

**Choice:** The ring is a sorted array of `(hash: u64, node_id: u64)`
pairs. Lookup uses binary search to find the first hash >= key_hash,
wrapping to index 0 if none found.

**Rationale:** A ring backed by a sorted array makes the coverage proof
(every key maps to exactly one node) follow from the wrap-around logic.
Minimal disruption on add/remove follows from the sorted insert/remove
affecting only adjacent elements. Reuses `collections::sorted_insert`
and `collections::binary_search`.

**Implementation:** `lookup(ring: &[(u64, u64)], key_hash: u64) -> u64`
finds first node with hash >= key_hash, wrapping. Coverage proof:
for non-empty ring, lookup always returns a valid node_id.

### 9. Merkle Proofs Use Chain Hash Axiom

**Choice:** Reuse the `chain` module's uninterpreted hash function and
determinism axiom. Merkle internal nodes hash `(left_child, right_child)`.

**Rationale:** Verus can't reason about real hash functions (blake3, sha256).
The chain module already established the pattern: model hash as a closed
spec fn with a trusted determinism axiom, verify structural properties,
test the actual hash at runtime with proptest. Same approach for Merkle.

**Implementation:** `verify_proof(leaf_hash: u64, index: u64,
siblings: &[u64], root: u64) -> bool` walks the sibling path computing
hashes. Proof length: `siblings.len() == ceil_log2(tree_size)`.

### 10. Gossip and Bloom Use Integer Bounds

**Choice:** Gossip infection bound uses integer exponentiation
(`min(n, f.pow(k))`). Bloom false-positive rate is not modeled
(would need real-valued arithmetic).

**Rationale:** Verus operates on integers. The gossip bound is exact
for ideal conditions. The bloom module proves the structural property
(no false negatives, monotonic insertion) and leaves the probabilistic
bound to runtime property tests.

**Implementation:** Gossip: `infection_lower_bound(n: u64, fanout: u64,
rounds: u64) -> u64`. Proof by induction on rounds. Bloom:
`BloomFilter` with `insert` and `query`. Proof: `inserted(x) ==>
query(x) == true`.

## Risks / Trade-offs

**[Quantifier blow-up in vector clock proofs]** Fixed-size arrays help,
but associativity of merge over N components needs `forall|i|` quantifiers
that Z3 may struggle with for large N. Mitigation: prove for arbitrary N
using bounded quantifiers with explicit triggers. Fall back to small N
(3-5) concrete proofs if general proof is intractable.

**[Circuit breaker state encoding]** Integer encoding is ugly but
necessary (Verus enum limitation). If Verus adds enum matching in specs,
refactor to proper enums. Same trade-off as batch module.

**[Merkle tree recursion]** Tree construction as a recursive spec fn may
hit Z3 resource limits for deep trees. Mitigation: verify the proof
checker (iterative), not tree construction. Bound tree depth to
reasonable values (≤ 64 levels = 2^64 leaves).

**[Gossip exponentiation overflow]** `f^k` overflows u64 quickly.
Mitigation: the bound is `min(n, f^k)` — use saturating pow and rely
on the min to cap at n. The spec uses int (no overflow), the exec
uses checked/saturating arithmetic.

**[Bloom filter hash functions]** Real Bloom filters use k independent
hash functions. Modeling k hashes as k positions (array of bit indices)
abstracts away the hash function entirely. The structural property
(set bits, check bits) is what matters for the no-false-negatives proof.
