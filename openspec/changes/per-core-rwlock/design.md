## Context

The existing `rwlock.rs` verifies mode transitions and mutual exclusion for
a single-counter rwlock. On real multicore hardware, that single counter is
a contention point. The per-core rwlock distributes reader counts across
cores so the read fast-path is local. The writer must aggregate all cores'
counts to determine safety. This module proves the aggregation is correct
and that mutual exclusion is preserved.

## Goals / Non-Goals

**Goals:**
- Model per-core reader counts as a `Seq<u32>` indexed by core id
- Prove aggregation (sum) properties: commutativity, identity, single-increment
- Prove mutual exclusion over the aggregated state
- Prove per-core independence (modifying one core doesn't touch others)
- Prove writer drain: once write flag is set, sum is non-increasing
- Prove mode equivalence: aggregated per-core state maps to the same mode
  as the global rwlock

**Non-Goals:**
- Actual atomic operations or memory ordering (this is a logic model)
- Scheduling or fairness between cores
- NUMA topology awareness
- Bounded core count (the proofs work for any `Seq` length)
- Writer starvation (no pending-writer priority modeled here)

## Decisions

### 1. State Representation

**Choice:** Per-core reader counts as `Seq<u32>` (a Verus ghost sequence).
A `bool` for the global write-held flag. A `u8` for the observable mode.

**Rationale:** `Seq` is Verus's native mathematical sequence type, ideal
for ghost/spec reasoning. Each index represents a core. The write flag is
a single bool because only one writer can hold at a time (same as the
global rwlock). The observable mode is derived, not stored — a spec function
computes it from write_held and the sum of readers.

**Alternative:** `Vec<u32>` for exec-level modeling — rejected because this
module focuses on the logic properties, not the runtime data structure.
A future module could bridge the gap.

### 2. Aggregation Function

**Choice:** Recursive spec function `sum_readers(counts: Seq<u32>, i: nat)
-> nat` that sums `counts[0..i]`. Returns `nat` to avoid overflow concerns
in the spec layer.

**Rationale:** Recursive definition over `Seq` length is natural for Verus
induction proofs. Using `nat` for the sum avoids needing overflow
preconditions in the spec. The exec-level aggregation function returns
`u64` with an overflow check.

**Implementation:**
```
spec fn sum_readers(counts: Seq<u32>, i: nat) -> nat
    decreases i
{
    if i == 0 { 0 }
    else { sum_readers(counts, (i - 1) as nat) + counts[(i - 1) as int] as nat }
}
```

### 3. Per-Core Operations

**Choice:** Spec functions `read_acquire_core` and `read_release_core` that
take the full `Seq<u32>` and a core index, returning a new `Seq<u32>` with
only that index modified. Exec functions mirror these with a mutable slice.

**Rationale:** Expressing the operation as "produce a new sequence differing
at exactly one index" makes the independence proof trivial: for all j ≠ i,
`new_counts[j] == old_counts[j]`.

### 4. Writer Protocol

**Choice:** Two-phase: (1) set write_held = true, (2) check sum_readers == 0.
Modeled as two separate spec predicates: `can_begin_write` (mode is Free or
Read) and `writer_drain_complete` (sum is zero after flag is set).

**Rationale:** Matches how real per-CPU rwlocks work (e.g., Linux
`percpu_down_write`). The writer sets a flag that prevents new readers,
then busy-waits for existing readers to drain. Splitting into two
predicates lets us prove drain monotonicity separately.

**Alternative:** Single-step writer acquire — rejected because it doesn't
model the real protocol and misses the drain monotonicity property.

### 5. Mode Equivalence

**Choice:** A spec function `per_core_mode(write_held: bool, sum: nat) -> u8`
that returns MODE_WRITE / MODE_READ / MODE_FREE. A proof that this matches
the global rwlock's `check_mutual_exclusion_spec` when sum is mapped to
active_readers and write_held to writer_holds.

**Rationale:** This is the refinement proof — it ties the per-core model
back to the existing global model. If the modes match, all properties proved
for the global rwlock (release transitions, etc.) transfer.

### 6. Module Structure

**Choice:** Single module `per_core_rwlock.rs` with sections:
1. Aggregation spec + proofs
2. Per-core read acquire/release + independence proofs
3. Writer acquire/release + drain proofs
4. Mode equivalence + mutual exclusion refinement
5. Tests

**Rationale:** The per-core rwlock is one concept with tightly coupled
proofs. Splitting into multiple files would scatter related invariants.

## Risks / Trade-offs

**[Sum overflow in exec layer]** → The exec aggregation function uses `u64`.
With max `u32::MAX` readers per core and a reasonable core count (< 2^32),
the sum fits. The spec layer uses `nat` and is overflow-free.

**[No memory ordering model]** → This module proves the logical properties
assuming sequential consistency. Proving correctness under relaxed memory
models (e.g., that the writer's read of per-core counts sees all prior
decrements) requires a memory model, which is out of scope.

**[Seq length is unbounded]** → The proofs work for any number of cores.
A deployment would fix the core count, but the universal proofs are stronger.
