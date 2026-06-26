## Context

The `aspen-cluster-handler` crate embeds pure decision logic inside async
handler functions. The logic is correct but unverified — it relies on unit
tests and code review. Extracting this logic into verified-logic modules
gives us machine-checked proofs that connect to the existing `quorum.rs`
foundation.

Three modules are needed. Each follows the established pattern in
verified-logic: spec fn (ghost), exec fn (real), proof fn (theorem),
proptest (runtime cross-check).

## Goals / Non-Goals

**Goals:** Verify the pure decision logic for membership changes, peer
collection, and node classification. Prove they maintain their invariants
under all inputs. Connect membership proofs to existing quorum arithmetic.

**Non-Goals:** Verify async handler orchestration, network I/O, Raft
controller interactions, error serialization, or deploy coordination.
Those are inherently effectful and belong in integration tests, not formal
proofs.

## Decisions

### 1. Module structure: three focused modules, not one monolith

**Choice:** `voter_set.rs`, `bounded_peers.rs`, `node_classify.rs` as
separate modules.

**Rationale:** Each module has distinct types and invariants. voter_set
depends on quorum.rs but the other two don't depend on each other. Separate
modules keep the proof obligations isolated — a change to peer collection
bounds can't accidentally break a membership proof.

**Alternative:** A single `cluster.rs` module. Rejected because it would
mix unrelated invariants and make verus verification slower.

### 2. Seq-based modeling for voter sets

**Choice:** Model voter lists as `Seq<u64>` in Verus specs. The exec
functions take `&[u64]` and return `Vec<u64>`.

**Rationale:** Verus has strong support for `Seq` reasoning — membership
(`contains`), length, append, filter. The quorum module already works with
`u64` counts. Mapping between `Seq` length and quorum's `u64` count is
direct.

**Alternative:** `Set<u64>` in specs. Rejected because the handler code
preserves insertion order (push to end, retain preserves order), and the
tests care about ordering. `Seq` matches the actual `Vec` behavior.

### 3. Bounded peers uses a capacity parameter, not a constant

**Choice:** `bounded_peers` takes `max_peers: u64` as a parameter rather
than hardcoding `MAX_BOOTSTRAP_PEERS = 16`.

**Rationale:** The verification should prove the bound holds for ANY
max_peers value. Hardcoding 16 would prove the property for one number.
The downstream code can call with 16 and get the full proof for free.

**Alternative:** Const generic or hardcoded constant. Rejected — parameter
is strictly more general and Verus handles it fine.

### 4. Node classification as a pure function, not a struct method

**Choice:** `classify_node(node_id, member_ids, leader_id) -> (bool, bool, bool)`
returning `(is_voter, is_learner, is_leader)`. Separate `classify_nodes`
function for batch classification with bounds.

**Rationale:** The handler builds `NodeDescriptor` structs, but the
decision logic is three boolean predicates. Verifying the predicates
individually is cleaner than verifying struct construction. The caller
can map the booleans into whatever struct it needs.

**Alternative:** Return a `NodeRole` enum. Rejected because the handler
code uses three independent booleans (`is_voter`, `is_learner`, `is_leader`)
and the spec needs to talk about their relationships (leader implies voter,
voter xor learner).

### 5. Quorum preservation proven as a conditional, not a precondition

**Choice:** The promote function always succeeds. A separate
`preserves_quorum` proof shows that IF the input had a quorum, the
output does too. The caller decides whether to check quorum before
calling.

**Rationale:** The handler code in aspen doesn't check quorum before
promoting — it delegates to the Raft controller which handles the error.
Making quorum a precondition would mismatch the actual usage. The proof
is an implication: `old_quorum ∧ promote → new_quorum`.

**Alternative:** Require quorum as a precondition on promote. Rejected
because it forces callers to prove quorum holds before they can call the
function, even when they don't need the quorum guarantee.

## Risks / Trade-offs

**[Seq reasoning complexity]** Verus `Seq::filter` and `Seq::contains`
can trigger expensive SMT queries. Mitigation: keep voter sets small
(bounded by MAX_CLUSTER_NODES = 100) and use `assert by` blocks with
explicit witnesses when the solver struggles.

**[Cross-module proof coupling]** `voter_set` depends on `quorum.rs`
proofs. If quorum's API changes, voter_set proofs break. Mitigation:
quorum's API is stable (it matches textbook definitions) and voter_set
only uses `majority_spec`, `is_quorum_spec`, and `can_remove_safely_spec`.

**[Ordering sensitivity]** The handler code does `push` then `retain`,
which means the learner is added at the end and the replaced node is
removed wherever it was. The Verus spec needs to match this exact order
of operations. Mitigation: mirror the handler's control flow directly
in the spec function.
