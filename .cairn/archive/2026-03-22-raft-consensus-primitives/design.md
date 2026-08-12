## Context

Raft's safety proof follows a dependency chain:

```
quorum intersection
  → vote safety (at most one leader per term)
    → leader completeness (committed entries survive elections)
      → state machine safety (replicas converge)
```

Each step relies on the one below. The bottom layer — quorum intersection —
is pure combinatorics with no Raft-specific concepts. The upper layers
add terms, logs, and elections. We build from the bottom up.

The library already has `arithmetic::div_ceil` (for majority calculation),
`ordering::min/max` (for term comparison), `slice::prefix_eq` (for log
prefix matching), and `collections::is_sorted` (for monotonic sequences).
The new modules compose with these.

## Goals / Non-Goals

**Goals:**
- Quorum intersection proof that downstream Raft verification can cite
- Monotonic value predicates usable for terms and commit indices
- Log matching property formalized and machine-checked
- Epoch comparison with total order proofs
- All modules follow the existing pattern: spec fn + exec fn + proof fn + tests
- Cross-module properties linking to existing library

**Non-Goals:**
- Full Raft protocol verification (that's a downstream consumer)
- Network/message modeling (out of scope for a logic library)
- Liveness proofs (Raft liveness depends on timing, not pure logic)
- Leader election state machine (protocol-level, not a primitive)

## Decisions

### 1. Quorum as count-based, not set-based

**Choice:** Model quorums as `majority(n) = n/2 + 1` and
`is_quorum(count, total) = count >= majority(total)`. No explicit
set representation.

**Rationale:** The intersection theorem only needs counts, not set
membership. `count_a + count_b > total` implies overlap by pigeonhole.
Set-based modeling would require Verus `Set<NodeId>` which adds complexity
without strengthening the proof.

**Alternative:** Use `vstd::set::Set` for voter sets. Rejected — Verus set
reasoning is powerful but verbose, and the core theorem doesn't need it.

### 2. Monotonic as a predicate, not a wrapper type

**Choice:** `is_monotonic_advance(old, new) = new >= old` as a predicate.
No `MonotonicU64` newtype.

**Rationale:** A newtype would need unsafe construction or builder pattern
to enforce monotonicity. A predicate lets callers compose with their own
types. Proofs about transitivity and composition work on the predicate.

**Alternative:** Newtype with `advance(&mut self, new)` method. Rejected —
adds API surface without adding proof power. The predicate is simpler and
the proofs are the same.

### 3. Log entries as (term, value) pairs

**Choice:** Model log entries as `(u64, u64)` where the first element is
the term and the second is the value. The log is a `Seq<(u64, u64)>`.

**Rationale:** Raft's Log Matching Property only cares about terms, not
values. Using `u64` for values keeps the module generic. Downstream code
maps its `AppData` type to `u64` (or ignores the value field).

**Alternative:** Generic `LogEntry<V>`. Rejected — Verus generics add
verification complexity without benefit for the proofs we're writing.

### 4. Epoch as a struct with derive-free comparison

**Choice:** `Epoch { term: u64, index: u64 }` with explicit `cmp_spec`
and `cmp` functions. No `Ord`/`PartialOrd` derive.

**Rationale:** Verus doesn't verify derived trait impls. An explicit
comparison function with `ensures` is verifiable. The spec defines
lexicographic ordering directly.

### 5. Phased implementation: quorum first

**Choice:** Build quorum → monotonic → epoch → log, one module per phase.
Each phase ends with verus + tracey clean.

**Rationale:** Each module builds on the previous conceptually (log uses
monotonic terms, epoch uses ordering). But they're code-independent — no
module imports another. The ordering matches the Raft proof dependency
chain from bottom to top.

## Risks / Trade-offs

**[Z3 nonlinear arithmetic for quorum]** → The intersection proof
`count_a + count_b > total` is linear arithmetic. Z3 handles this
trivially. The `majority = n/2 + 1` definition uses integer division,
which may need `lemma_fundamental_div_mod` like the existing div_ceil
proofs. Mitigation: follow the chunk_count proof pattern.

**[Seq operations for log module]** → Verus `Seq` has `subrange`, `push`,
`index`, `len`. The Log Matching Property requires inductive reasoning
over log prefixes. Mitigation: use the same inductive proof pattern as
`mismatch_none_inductive` and `all_equal_count_from`.

**[Scope creep toward protocol verification]** → The modules should
provide building blocks, not model the full Raft protocol. Guard rail:
no `RaftState` struct, no message types, no state transitions. If a
function needs to know about leaders or followers, it's out of scope.
