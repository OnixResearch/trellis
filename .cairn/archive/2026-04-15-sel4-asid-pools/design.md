## Context

ASID uniqueness is one of the smallest but most reusable global kernel facts. A dedicated change gives VSpace and invariant proofs a clean allocation/release model without requiring full architecture-specific MMU logic.

## Goals / Non-Goals

**Goals:**
- Allocate unique live ASIDs.
- Allow reuse only after release.
- Respect finite pool capacity.

**Non-Goals:**
- Architecture-specific ASID hardware details
- Virtual-address mapping contents
- Capability lookup

## Decisions

### 1. Model one abstract pool as `Seq<Option<u64>>`

**Choice:** Represent one pool as a finite sequence indexed by numeric ASID, with `Some(owner_id)` for a live owner and `None` for a free slot. Owner identity is intentionally modeled as an opaque `u64` token: proofs may compare owner IDs for equality, but no richer owner structure is assumed.
**Rationale:** The requirements only need one abstract pool instance. One slot per numeric ASID makes uniqueness a structural invariant and keeps reuse/exhaustion proofs local.

**Implementation:** `src/sel4_asid.rs` will expose `AsidPool::new(capacity: usize)` plus public spec/exec helpers over opaque `u64` owner IDs, with returned runtime ASIDs also typed as `usize` and related to the spec range `0..capacity-1` through `asid as int`. `new(capacity)` initializes `slots.len() == capacity` with every slot `None`. The concrete invariant is `wf() == (self.capacity == self.slots@.len())`. Construction fixes pool length forever; `new` establishes `wf()`, and `allocate` / `release` preserve `wf()` and therefore preserve `slots.len()` / `capacity`.

### 2. Make allocation explicit and exhaustion visible

**Choice:** `allocate(owner)` returns `Some(asid)` whenever some free slot exists and returns `None` exactly when no free slot exists.
**Rationale:** The public contract needs successful ownership creation for available capacity plus explicit exhaustion.

**Implementation:** Allocation may use a linear first-free scan internally, but the public contract commits only to success when some free slot exists, in-range results, ownership creation, unchanged unrelated slots, and exhausted-pool failure with unchanged state.

### 3. Make release an exact `(asid, owner)` transition

**Choice:** `release(asid, owner)` succeeds only when `asid` is in range and currently owned by that exact `owner`; success clears the slot to `None`. Any out-of-range, free-slot, owner-mismatch, or duplicate release returns `false` and leaves the pool unchanged.
**Rationale:** The spec needs explicit release semantics so reuse is only justified after ownership has been removed.

**Implementation:** Release is a single-slot state transition with unchanged unrelated slots on success and an unchanged-state postcondition on failure.

### 4. Separate ASID ownership from mapping logic

**Choice:** Keep this change at the ownership/allocation layer only.
**Rationale:** VSpace mapping and hardware-specific ASID behavior can depend on this module later without complicating the core model.

## Public Reusable Surface

`src/sel4_asid.rs` will expose these reusable artifacts for later VSpace/invariant work, and `src/lib.rs` will re-export the module:
- runtime storage `slots: Vec<Option<u64>>` with spec view `slots@`, surfaced as `ownership_view()` for ownership-state predicates
- `AsidPool::new(capacity: usize)` with postconditions `slots.len() == capacity` and every slot initially `None`
- `AsidPool::wf()` for the single-pool shape invariant over `Vec<Option<u64>>` / `Seq<Option<u64>>`
- spec predicates for slot liveness, exact owner matching, and state-equality helpers such as `same_ownership_except_spec(...)` for unchanged-state obligations
- exec allocation/release helpers with matching ensures clauses, concretely `allocate(&mut self, owner: u64) -> Option<usize>` and `release(&mut self, asid: usize, owner: u64) -> bool`
- named proof helpers for `wf()` establishment/preservation, allocation-created ownership, allocation-targets-free-slot, allocation-preserves-unrelated-slots, release-preserves-unrelated-slots, invalid-release unchanged-state, allocation-failure unchanged-state, safe reuse, and exhausted-pool failure

## Verification Strategy

- `r[sel4-asid.uniqueness]`: prove successful allocation targets a pre-state free slot, creates a live owner binding for the returned ASID, changes no unrelated slot, and that two successful distinct live allocations cannot return the same ASID; cover this with unit tests and proptests for successful allocation, unrelated-slot preservation, same-owner-multiple-ASIDs, and distinct-allocation behavior.
- `r[sel4-asid.release-state]`: prove successful release returns success, clears only the targeted slot, preserves unrelated slots, unsuccessful release preserves state, and cover this with unit tests and proptests for out-of-range, free-slot, owner-mismatch, and duplicate release failures.
- `r[sel4-asid.reuse]`: prove conditional reuse safety (`if` the same numeric ASID is reused after a valid release, the old owner is gone) and cover this with unit tests and proptests that force same-ASID reuse only under deterministic preconditions such as capacity `1`.
- `r[sel4-asid.pool-bounds]`: prove allocation succeeds when some free slot exists, successful allocation returns an in-range ASID, and `None` appears only on a full pool with unchanged state; cover this with unit tests and proptests for both zero-capacity immediate exhaustion and ordinary capacity-sized exhaustion.
- Add matching `r[...]` entries to `docs/requirements.md` plus `r[impl]` / `r[verify]` anchors on the defining helpers, proofs, and tests so Tracey sees the new domain.

## Risks / Trade-offs

**[Reuse bugs after release]** Make release a state transition that clears ownership before reallocation can occur, then prove the cleared-slot fact directly.
**[Over-modeling hardware tags]** Stay at the abstract ownership level only.
