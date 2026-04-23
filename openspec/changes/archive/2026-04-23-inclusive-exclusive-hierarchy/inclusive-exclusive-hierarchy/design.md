## Context

Caches are hierarchical: L1 is small and fast, L2 is larger and slower,
L3 is largest. The inclusion policy governs the relationship between
levels. This module models all three policies (inclusive, exclusive,
non-inclusive) and proves their invariants are inductive.

## Goals / Non-Goals

**Goals:**
- Model two-level hierarchy (L1, L2) with parameterized policy
- Inclusive: L1⊆L2 as inductive invariant
- Exclusive: L1∩L2=∅ as inductive invariant
- Non-inclusive: no structural constraint
- Back-invalidation for inclusive L2 eviction
- Atomic swap for exclusive fill
- Capacity formulas
- Coherence snoop scope

**Non-Goals:**
- Three-level hierarchy (L1/L2/L3) — two levels are sufficient for
  the fundamental proofs, and three levels compose
- Latency or bandwidth modeling
- Specific replacement policies per level
- Victim caches or write-back buffers between levels

## Decisions

### 1. Level Model

**Choice:** Each level is modeled semantically as a finite set of line
addresses, but the exec surface uses canonical deduplicated `Vec<u64>`
sequences plus set-style membership predicates (`contains_line_spec`).
The policy determines the invariant between the levels.

**Rationale:** For hierarchy invariant proofs, we only need to know which
lines are present, not their data or coherence state. A deduplicated
sequence is a Verus-friendly exec encoding of a finite set while keeping
the proof surface set-like. Data correctness follows from "if it's in
L1, it's valid" which is the cache management layer's job.

### 2. Operations

**Choice:** Five operations, each parameterized by policy:
- `l1_fill(line, l1, l2, policy) -> (l1', l2')` — install a line in L1
- `l1_evict(line, l1, l2, policy) -> (l1', l2')` — remove a line from L1; under exclusive policy this is the victim-to-L2 demotion surface
- `l2_fill(line, l1, l2, policy) -> (l1', l2')` — install a line in L2; under exclusive policy we deliberately model a swap-for-generality surface, so a line already present in L1 is removed there when filled into L2
- `l2_evict(line, l1, l2, policy) -> (l1', l2')` — remove a line from L2
- `l2_to_l1_promote(line, l1, l2, policy) -> (l1', l2')` — move from L2 to L1

Every operation preserves canonical deduplicated levels by construction:
`append_unique` never adds a second copy of a line and `remove_line`
only deletes matching entries.

**Rationale:** Each operation must maintain the policy's invariant. Making
the policy a parameter lets us prove all three variants with a single
set of proof lemmas (one per operation per policy).

### 3. Inclusive Invariant

**Choice:** `inclusive_invariant(l1, l2) = l1.subset_of(l2)`. Prove:
- Initial state: both empty → trivially holds
- `l1_fill`: adds to L1, also adds to L2 if not already present
- `l1_evict`: removes from L1 only → L1 shrinks, still ⊆ L2
- `l2_fill`: adds to L2 → L2 grows, L1 still ⊆ L2
- `l2_evict`: removes from L2, and if line is in L1, back-invalidates L1

**Rationale:** The only interesting case is `l2_evict`: it must
back-invalidate L1 to maintain the invariant. The proof shows that
forgetting the back-invalidation breaks the invariant.

### 4. Exclusive Invariant

**Choice:** `exclusive_invariant(l1, l2) = l1.disjoint(l2)`. Prove:
- `l1_fill`: adds to L1, removes from L2 (swap semantics)
- `l2_fill`: adds to L2, removes from L1 (swap semantics)
- `l1_evict`: demotes a present line from L1 into L2 (victim-cache semantics)
- `l2_evict`: plain removal, L1 unaffected

**Rationale:** Exclusive caches maximize aggregate capacity. Every fill
at one level implies removal at the other. The swap is what makes the
exclusive invariant tricky — it must be atomic to avoid a window where
the line is in both levels.

### 5. Capacity Proof

**Choice:** Under inclusive: `effective_capacity = |l2|` (L1 is a subset,
adds no unique lines). Under exclusive: `effective_capacity = |l1| + |l2|`
(no overlap). These counts are exact for canonical deduplicated levels and
are proved from the invariants.

**Rationale:** This is why architects choose one policy over the other.
The proof is a direct consequence of the subset/disjointness invariant.

### 6. Snoop Scope

**Choice:** Under inclusive: `snoop_l2_only(l1, l2, line) -> bool` —
if the line is not in L2, it's not in L1 either (by inclusion). Under
exclusive: must check both levels. Under non-inclusive: snoop answers are
the union of both levels because neither subset nor exclusion holds.

**Rationale:** Inclusive L3 caches are popular because snooping only the
L3 is sufficient, reducing snoop traffic. The proof is a one-line
consequence of the inclusive invariant.

## Risks / Trade-offs

**[Two levels only]** → Three-level hierarchies compose: (L1, L2) inclusive
+ (L2, L3) inclusive implies (L1, L3) inclusive. The composition proof is
a separate lemma but follows from transitivity of subset.

**[Set model loses data]** → The set model doesn't track line data, so
we can't prove data consistency (e.g., L1 and L2 hold the same data for
the same line). That's the cache management + coherence layer's job.

**[Atomic swap assumption]** → The exclusive swap is modeled as atomic.
Real hardware may have a transient state where the line is in neither
level (in the fill buffer). The model doesn't capture that window.

## Verification Strategy

- Prove per-operation preservation lemmas for inclusive and exclusive
  invariants over set-style membership predicates.
- Use `Vec<u64>` exec helpers only as a canonical finite-set encoding;
  proofs reason about membership, not ordering.
- Prove capacity and snoop-scope facts as consequence lemmas from the
  preserved invariants.
- Use unit tests for policy-specific happy/sad paths and proptests for
  invariant preservation across random operation sequences.
