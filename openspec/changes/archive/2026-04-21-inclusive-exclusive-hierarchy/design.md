## Context

Caches are hierarchical: L1 is small and fast, L2 is larger and slower.
The inclusion policy governs the relationship between levels. This change
models Inclusive, Exclusive, and NonInclusive two-level hierarchies and
verifies exact transition/checker/query semantics for that surface.

## Goals / Non-Goals

**Goals:**
- Model a two-level hierarchy (`L1`, `L2`) with explicit policy
- Verify exact semantics for `l1_fill`, `l1_evict`, `l2_fill`, `l2_evict`, and `l2_to_l1_promote`
- Verify checker/query APIs: `check_inclusive_invariant`, `check_exclusive_invariant`, `check_valid`, `effective_capacity`, and `l2_miss_implies_l1_miss`
- Cover inclusive back-invalidation, exclusive lower-level removal on promotion, current effective occupancy formulas, and inclusive snoop scope
- Add negative tests for broken inclusive state plus Exclusive/NonInclusive L2-only snooping counterexamples

**Non-Goals:**
- Three-level hierarchy composition proofs
- Data-value coherence or writeback correctness
- Replacement policy details or timing/bandwidth modeling
- Hardware transient-state modeling between levels

## Decisions

### 1. Runtime model

**Choice:** Represent each level as a duplicate-avoiding `Vec<u64>` of line
IDs at runtime, with set-style presence specs over the line values.

**Rationale:** The properties in scope are about line presence, not line
data. `Vec<u64>` keeps the exec surface small and test-friendly while the
spec surface still reasons in set-like terms.

### 2. Policy-generic APIs

**Choice:** Keep one `CacheHierarchy` value with policy-tagged exact APIs:
- `new_hierarchy(policy)` creates empty levels
- `check_inclusive_invariant`, `check_exclusive_invariant`, `check_valid`
- `l1_fill`, `l1_evict`, `l2_fill`, `l2_evict`, `l2_to_l1_promote`
- `effective_capacity`, `l2_miss_implies_l1_miss`
- `src/lib.rs` exports `cache_hierarchy`

**Rationale:** One shared state type keeps policy differences local to the
transition logic and avoids parallel API families.

### 3. Exact mutator semantics

**Inclusive**
- `l1_fill(line)`: add `line` to L1 and add it to L2 if missing
- `l1_evict(line)`: remove `line` from L1 only
- `l2_fill(line)`: add `line` to L2 only
- `l2_evict(line)`: remove `line` from L2 and also remove it from L1
- `l2_to_l1_promote(line)`: if `line` is present in L2, add it to L1 and keep the L2 copy

**Exclusive**
- `l1_fill(line)`: add `line` to L1 and remove it from L2
- `l1_evict(line)`: remove `line` from L1 and add it to L2
- `l2_fill(line)`: add `line` to L2 and remove it from L1
- `l2_evict(line)`: remove `line` from L2 only
- `l2_to_l1_promote(line)`: if `line` is present in L2, add it to L1 and remove it from L2

**NonInclusive**
- `l1_fill(line)`: add `line` to L1 only
- `l1_evict(line)`: remove `line` from L1 only
- `l2_fill(line)`: add `line` to L2 only
- `l2_evict(line)`: remove `line` from L2 only
- `l2_to_l1_promote(line)`: if `line` is present in L2, add it to L1 and keep the L2 copy

**Absent-line / already-present behavior**
- Fill helpers are idempotent because levels are duplicate-avoiding
- Evicting an absent line is a no-op for the unaffected level
- Promoting an L2-absent line is a no-op

**Rationale:** The implementation is total. Exact end-state semantics are
more useful here than partial precondition-heavy APIs.

### 4. Checker/query semantics

- `check_inclusive_invariant` returns true iff every L1 line is also in L2
- `check_exclusive_invariant` returns true iff no line is present in both levels
- `check_valid` dispatches by policy:
  - Inclusive → `check_inclusive_invariant`
  - Exclusive → `check_exclusive_invariant`
  - NonInclusive → always true
- `effective_capacity` reports current effective occupancy:
  - Inclusive → `|L2|`
  - Exclusive → `|L1| + |L2|`
  - NonInclusive → `|L1| + |L2|`
- `l2_miss_implies_l1_miss(line)` returns the Inclusive L2-only snooping implication and can be false for Exclusive or NonInclusive states

**Rationale:** These are direct query APIs over the modeled state, so the
most useful guarantee is exact return-value semantics.

### 5. Representation invariant

**Choice:** All public mutators preserve duplicate-free levels by using
push-if-absent and remove-by-value helpers.

**Rationale:** Exact occupancy formulas (`|L2|`, `|L1| + |L2|`) only make
sense if the runtime representation cannot accumulate duplicate line IDs.

### 6. Verification matrix

**Proof-backed / exact-contract surfaces:**
- Exact postconditions for `new_hierarchy`, the five mutators, the two
  invariant checkers, `check_valid`, `effective_capacity`, and
  `l2_miss_implies_l1_miss`
- Small direct proof helpers for NonInclusive acceptance, capacity
  formulas, and the Inclusive snoop implication

**Test-backed sequence evidence:**
- Unit tests for inclusive, exclusive, and non-inclusive scenarios
- Negative tests for missing back-invalidation and both Exclusive and
  NonInclusive L2-only snooping counterexamples
- Proptests over random operation sequences for Inclusive and Exclusive
  checker preservation and capacity formulas

## Risks / Trade-offs

**[Two levels only]** Three-level hierarchy behavior is intentionally out
of scope for this landing.

**[Presence only]** This model does not prove same-data agreement between
levels; that belongs in cache-data/coherence models.

**[Exact end states, not transients]** Exclusive swap behavior is modeled
by the resulting state after each API call, not by intermediate hardware
microstates.
