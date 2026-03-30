# Memory Allocator Verification Plan

## Motivation

The crate already has five allocator-adjacent modules: `buddy` (power-of-two
splitting), `slab` (fixed-size slots), `watermark` (bump allocation), `region`
(address arithmetic), and `free_space_map` (bitmap tracking). Each one verifies
its own arithmetic in isolation. None of them compose into an allocator you
could actually use — there is no allocation lifecycle (alloc → use → free),
no double-free prevention, no fragmentation reasoning that spans multiple
strategies, and no proof that combining a buddy splitter with a slab frontend
preserves the invariants of both.

This plan adds modules that close those gaps. The goal is a verified toolkit
where a real allocator can be assembled from proven pieces, and the composition
proofs guarantee the safety properties (no overlap, no double-free, conservation)
hold end-to-end.

## Existing Foundation

| Module | What it proves | Gaps |
|--------|---------------|------|
| `buddy` | Block size 2^order, buddy involution, coalesce symmetry, parent alignment | No split/coalesce *sequences*, no free-list state |
| `slab` | Slot ↔ address roundtrip, capacity, generation counters, handle pack/unpack | No allocation bitmap, no lifetime tracking |
| `watermark` | Monotonic advance, non-overlap, capacity conservation, reset | No free — bump only, no reuse |
| `region` | Contains, disjoint, adjacent, merge, split | Pure geometry — no allocation state |
| `free_space_map` | Alloc/free conservation, first-free, contiguous runs, fragmentation % | Bitmap only — no typed slots, no size classes |

Cross-module properties already proven:
- `watermark` advance is monotonic (`monotonic`)
- `watermark` successive allocs produce disjoint regions (`region`)
- `slab` slot index is page frame number (`page`)
- `slab` slot addresses fall within the containing region (`region`)
- `region` operations agree with `interval`

## New Modules

### Tier 1 — Allocation lifecycle and safety invariants

These modules add the state and proofs that turn address arithmetic into
allocation protocols with safety guarantees.

**1. `alloc_token` — Allocation lifecycle state machine**

An allocated block has a lifecycle: free → allocated → freed. Double-free
and use-after-free are the two bugs every allocator must prevent. This module
models the lifecycle as a state machine on tokens, not on the heap itself,
so the proofs compose with any backing allocator.

Functions:
- `alloc_token_new(id) → Token{id, state: Free}` — create token for a block
- `alloc_token_allocate(t) → Token{state: Allocated}` — free → allocated
- `alloc_token_release(t) → Token{state: Freed}` — allocated → freed
- `is_live(t) → bool` — true iff state == Allocated

Properties to verify:
- Allocate requires state == Free (prevents double-alloc)
- Release requires state == Allocated (prevents double-free)
- Freed is terminal — no transition out
- is_live is true only in Allocated state
- Token ID is immutable across transitions

Why: Every module below uses alloc_token to track block lifecycles. The
proofs that buddy-split doesn't lose blocks or slab-free doesn't corrupt
slots all reduce to token state machine properties.

**2. `size_class` — Size class bucketing and waste bounds**

Real allocators don't serve arbitrary sizes. They round up to size classes
and the waste (internal fragmentation) must be bounded. This module defines
the mapping from requested size to class and proves waste bounds.

Functions:
- `size_to_class(size) → class_index` — which bucket serves this size
- `class_to_size(class) → actual_size` — actual bytes the class provides
- `waste(requested, class) → actual - requested` — internal fragmentation
- `class_count(max_size) → n` — total number of size classes

Properties to verify:
- `class_to_size(size_to_class(s)) >= s` — class is big enough
- `waste(s, size_to_class(s)) < s` — waste < 100% of request (for s > 0)
- Monotonicity: larger request → same or larger class
- Roundtrip: `size_to_class(class_to_size(c)) == c` — classes are fixed points
- class_count is bounded (logarithmic in max_size for power-of-two classes)

Why: Connects `buddy` (power-of-two) and `slab` (fixed-slot) sizing to
actual allocation requests. The waste bound is a key allocator quality metric
that has no formal treatment in the existing modules.

**3. `free_list` — Intrusive free list with no-cycle and containment proofs**

A free list is the fundamental bookkeeping structure for general-purpose
allocators. The tricky part is proving it stays acyclic and every entry
falls within the managed region.

Functions:
- `free_list_push(list, block) → list'` — add block to free list
- `free_list_pop(list) → (block, list')` — remove first block
- `free_list_contains(list, block) → bool` — membership test
- `free_list_remove(list, block) → list'` — remove specific block
- `free_list_len(list) → count` — number of free blocks

Properties to verify:
- Push then pop returns same block (LIFO)
- No duplicates: push of already-present block is rejected
- Length tracks: push increments, pop decrements
- All entries fall within a bounding region (containment)
- No cycles: each block appears at most once
- Remove + push is not identity when list has > 1 element (order changes)

Why: `free_space_map` uses a flat bitmap. Real allocators use linked free
lists (often intrusive — the next pointer lives inside the free block).
This module proves the free list invariants that bitmap tracking doesn't
capture.

**4. `split_coalesce` — Buddy split/coalesce sequences with block conservation**

`buddy` proves the arithmetic of individual split/coalesce operations.
This module proves *sequences* of splits and coalesces conserve total
memory: splitting a block into two halves and coalescing them back
recovers the original, and a sequence of splits starting from one
large block produces disjoint blocks whose sizes sum to the original.

Functions:
- `split_block(addr, order) → (left, right)` — split into two buddies
- `coalesce_blocks(left, right, order) → parent` — merge buddies
- `split_tree_total(addr, order, target_order) → Vec<(addr, order)>` — recursive split to target
- `blocks_disjoint(blocks) → bool` — no overlap among block set
- `blocks_cover(blocks, addr, order) → bool` — blocks exactly cover parent

Properties to verify:
- split then coalesce is identity (roundtrip)
- split produces two disjoint regions whose sizes sum to parent
- coalesce requires blocks are valid buddies (uses `buddy::can_coalesce`)
- Recursive split to target order: all blocks disjoint, all same target order, sizes sum to original
- Block count after splitting order k down to order j: exactly 2^(k-j)

Why: The inductive property — "every sequence of splits/coalesces
conserves total space" — is the core buddy allocator invariant that
individual-operation proofs in `buddy.rs` can't express.

**5. `alloc_bitmap` — Typed allocation bitmap with double-free detection**

`free_space_map` tracks free/allocated as booleans. This module adds
generation counters per slot (like `slab` handles) so that freeing a
stale handle is detected, and the bitmap state agrees with `alloc_token`
lifecycle.

Functions:
- `bitmap_alloc(bm) → (slot, handle, bm')` — allocate first free slot, return generational handle
- `bitmap_free(bm, handle) → Result<bm', Error>` — free by handle, reject stale/double
- `bitmap_is_allocated(bm, slot) → bool` — check slot state
- `bitmap_generation(bm, slot) → gen` — current generation for slot
- `bitmap_utilization(bm) → (used, total)` — occupancy

Properties to verify:
- Alloc marks slot as used, generation increments
- Free marks slot as available, generation stays (incremented on next alloc)
- Double-free: free(free(alloc())) returns Err
- Stale handle: alloc, free, alloc again, free with first handle → Err
- utilization.used + utilization.free == total (conservation)
- Alloc when full returns None

Why: Combines `free_space_map` (bitmap) with `slab` (generation counters)
into a single structure that detects the two most dangerous allocator bugs
at the bookkeeping level.

### Tier 2 — Composition and strategy proofs

These modules compose Tier 1 pieces into allocator strategies and prove
that the safety properties hold through the composition boundary.

**6. `arena` — Arena (region-based) allocation with collective free**

The simplest allocator: bump-allocate from a region, free everything at
once. This is `watermark` with an explicit epoch/reset model and the proof
that all outstanding allocations become invalid on reset.

Functions:
- `arena_alloc(arena, size) → (ptr, arena')` — bump-allocate
- `arena_reset(arena) → arena'` — free all, generation increments
- `arena_remaining(arena) → bytes` — space left
- `arena_generation(arena) → gen` — current epoch
- `arena_is_valid(arena, ptr, gen) → bool` — ptr valid iff gen matches

Properties to verify:
- Successive allocs produce disjoint regions (reuse `watermark` proof)
- Reset invalidates all pointers from prior generation
- No individual free — only collective via reset
- After reset, remaining == capacity
- Generation monotonically increases
- Pointer from generation g is invalid in generation g+1

Why: Arenas are the most-used allocator in compilers, game engines, and
request handlers. The key property — reset invalidates everything — is
trivial to state but easy to get wrong when generations wrap.

**7. `pool` — Fixed-size object pool with bounded capacity**

A slab allocator specialized for one object size. Combines `slab` (slot
math) with `alloc_bitmap` (lifecycle tracking) and proves the composition
maintains both modules' invariants.

Functions:
- `pool_new(capacity, obj_size) → Pool` — create pool
- `pool_alloc(pool) → Option<(handle, addr)>` — allocate one slot
- `pool_free(pool, handle) → Result<(), Error>` — free by handle
- `pool_is_full(pool) → bool` — at capacity
- `pool_count(pool) → (allocated, free)` — occupancy

Properties to verify:
- Returned address is slot-aligned (via `slab::is_slot_aligned`)
- Address falls within pool region (via `region::contains_addr`)
- No two live allocations share an address
- alloc/free cycle: pool_count is conserved
- Double-free returns Err (via `alloc_bitmap` generation check)
- Full pool returns None, free then alloc succeeds

Why: This is the first module that composes two existing modules (`slab`
+ `alloc_bitmap`) and proves the composition. The proof that
`slab::slot_address` always lands within the pool's `region` is the
kind of cross-module property the crate needs more of.

**8. `segregated_free` — Segregated free lists by size class**

Multiple free lists, one per size class. Allocation picks the right
class and pops from that list. Freeing pushes back to the correct class.
This is how dlmalloc, jemalloc, and mimalloc actually work.

Functions:
- `seg_alloc(seg, requested_size) → Option<(block, class)>` — allocate from appropriate class
- `seg_free(seg, block, class) → seg'` — return block to its class list
- `seg_total_free(seg) → count` — total free blocks across all classes
- `seg_class_for(seg, size) → class` — which free list handles this size

Properties to verify:
- Returned block size >= requested (via `size_class::class_to_size`)
- Block goes back to same class it was allocated from
- Total free across classes is conserved on alloc+free cycle
- No block appears in two different class lists simultaneously
- Each class list is internally valid (`free_list` invariants hold per class)

Why: Composes `size_class` (bucketing) with `free_list` (per-bucket
bookkeeping). The cross-class uniqueness invariant — a block is in
exactly one list — is the central correctness property of segregated-fit
allocators and has no analog in the existing modules.

**9. `coalesce_policy` — When and how to coalesce free blocks**

Coalescing isn't always immediate. Deferred coalescing batches merges for
throughput. This module defines coalescing policies and proves they
preserve the buddy invariants.

Functions:
- `should_coalesce_immediate(block, buddy_state) → bool` — eager policy
- `should_coalesce_deferred(block, buddy_state, free_count, threshold) → bool` — lazy policy
- `deferred_queue_add(queue, block) → queue'` — queue block for later coalesce
- `deferred_queue_drain(queue) → Vec<(block, buddy)>` — flush coalesceable pairs
- `coalesce_preserves_total(blocks_before, blocks_after) → bool` — total bytes unchanged

Properties to verify:
- Immediate coalesce: if buddy is free, coalesce happens, block count decreases by 1
- Deferred: queue size bounded by total blocks
- Drain produces only valid buddy pairs (via `buddy::can_coalesce`)
- Both policies preserve total managed bytes
- Deferred converges to same state as immediate (given enough drains)

Why: Real buddy allocators defer coalescing to avoid pathological
split-coalesce-split cycles. The convergence proof — deferred eventually
reaches the same state as immediate — is the key correctness argument
for the optimization.

**10. `fragmentation` — Fragmentation metrics with formal bounds**

`free_space_map` has a `fragmentation_pct` function but no formal
connection to allocation failure. This module defines when fragmentation
causes an allocation to fail despite sufficient total free space, and
proves bounds on fragmentation under different strategies.

Functions:
- `external_frag(free_total, largest_free_block) → ratio` — external fragmentation
- `internal_frag(requested, allocated) → waste` — internal (same as `size_class::waste`)
- `can_satisfy(free_map, requested_size) → bool` — enough *contiguous* space exists
- `frag_bound_buddy(total, min_order) → max_waste` — worst-case buddy internal fragmentation
- `frag_bound_slab(obj_size, slot_size) → max_waste` — worst-case slab waste per slot

Properties to verify:
- `can_satisfy` implies `free_total >= requested_size` (necessary condition)
- `!can_satisfy` with `free_total >= requested_size` implies external fragmentation > 0
- Buddy worst case: internal waste < requested size (< 50% for sizes > 1)
- Slab worst case: waste == `slot_size - obj_size` (constant per class)
- External fragmentation is 0 iff largest_free_block == free_total (one contiguous chunk)

Why: Fragmentation is the failure mode of all allocators. Having formal
bounds lets a system prove things like "with this buddy order range and
these allocation patterns, we never fail due to fragmentation when free
space exceeds threshold T."

### Tier 3 — End-to-end composition proofs

These go in `properties.rs` and prove that the modules compose correctly.

**11. Cross-module properties to add:**

- `pool` slot addresses are `slab`-aligned AND within the pool `region`
- `pool` alloc/free agree with `alloc_token` lifecycle (allocated ↔ Live)
- `arena` successive allocs agree with `watermark` non-overlap proof
- `arena` reset increments generation AND `watermark` resets to base
- `segregated_free` class selection agrees with `size_class` mapping
- `segregated_free` + `buddy` split: returned block satisfies size class
- `split_coalesce` + `free_space_map`: split updates bitmap conservatively
- `alloc_bitmap` + `slab` generation: handle encodes both slot and generation
- `fragmentation` bounds agree with `free_space_map::fragmentation_pct`
- `coalesce_policy` deferred drain + `buddy::can_coalesce` → valid merges

## Execution Order

| Phase | Modules | Builds on | Key proof |
|-------|---------|-----------|-----------|
| 1 | `alloc_token`, `size_class` | — (standalone) | Lifecycle FSM, waste bounds |
| 2 | `free_list` | `region` | No cycles, containment |
| 3 | `split_coalesce` | `buddy`, `region` | Recursive split conservation |
| 4 | `alloc_bitmap` | `free_space_map`, `slab` | Double-free detection via generations |
| 5 | `arena` | `watermark`, `alloc_token` | Reset invalidates all pointers |
| 6 | `pool` | `slab`, `alloc_bitmap`, `region` | First two-module composition |
| 7 | `segregated_free` | `size_class`, `free_list` | Cross-class uniqueness |
| 8 | `coalesce_policy` | `buddy`, `split_coalesce` | Deferred convergence |
| 9 | `fragmentation` | `free_space_map`, `buddy`, `slab` | Allocation failure bounds |
| 10 | Cross-module props | all of the above | End-to-end safety |

Phases 1–2 are independent and can be done in parallel. Phase 3–4
depend on phase 1. Phases 5–7 depend on 1–4. Phases 8–9 can proceed
in parallel once phase 3 is done. Phase 10 comes last.

## Estimated Scope

| | Modules | Spec fns | Exec fns | Proofs | Proptests | Requirements |
|--|---------|----------|----------|--------|-----------|--------------|
| New | 9 | ~50 | ~40 | ~80 | ~50 | ~45 |
| Existing (allocator-adjacent) | 5 | 35 | 26 | 60 | 40 | 41 |
| Cross-module (properties.rs) | — | ~10 | — | ~10 | ~10 | ~10 |

## Verification Challenges to Expect

**Sequence reasoning for `split_coalesce`.** Proving that a recursive
split to depth k produces 2^k blocks whose sizes sum to the original
requires induction on the split depth. Verus handles this with
`decreases` clauses but the trigger for the sum-of-powers lemma will
need a manual `assert_by` with `nonlinear_arith`.

**Free list acyclicity.** Representing a free list as `Seq<u64>` and
proving no duplicates requires a quadratic-looking spec (forall i, j
where i != j, list[i] != list[j]). In practice, push-checks-contains
makes the proof incremental, but the quantifier may need explicit
triggers on `Seq::index`.

**Generation counter overflow in `alloc_bitmap`.** Slab uses `u32`
generations. The proof that stale-handle detection works requires
generation != old_generation, which fails if generations wrap to the
same value. Either prove the wrap distance exceeds practical handle
lifetime (needs a usage bound assumption) or use `u64` and prove
no overflow within the block count.

**Cross-module composition.** The `pool` module must show that
`slab::slot_address` output satisfies `region::contains_addr`. This
requires connecting `slab_capacity * slot_size <= region_size` with
the slot address formula. Existing `properties.rs` already has this
proptest; the new work is lifting it to a Verus proof.
