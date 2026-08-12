## Context

The library already has page-level decomposition (`page.rs`: frame number,
offset, alignment, pages_needed), bitwise operations (`bitwise.rs`:
power-of-two, bit manipulation), integer alignment (`integer.rs`:
align_down, align_up, log2_floor, next_power_of_two), half-open intervals
(`interval.rs`: contains, overlaps, split, merge, adjacent), and monotonic
predicates (`monotonic.rs`: advance, strict advance).

Memory allocators compose these building blocks. A slab allocator is
page-style decomposition with arbitrary slot sizes. A buddy system is
bitwise XOR plus power-of-two arithmetic. Region management is interval
arithmetic with overflow-safe 64-bit addresses. Watermark allocation
is monotonic advancement with alignment.

## Goals / Non-Goals

**Goals:**
- Slab slot arithmetic with generation-based ABA prevention
- Buddy system block arithmetic with involution and coalescing proofs
- Region overlap/containment/merge/split with overflow-safe arithmetic
- Watermark bump allocation with monotonicity and non-overlap proofs
- Cross-module properties linking to page, integer, interval, monotonic,
  bitwise modules
- All modules follow the existing spec fn + exec fn + proof fn + tests
  pattern

**Non-Goals:**
- Actual allocator state machines (free lists, allocation bitmaps)
- Concurrency or locking (pure decision logic only)
- Virtual memory page table walks (page.rs covers the arithmetic)
- Fragmentation analysis or allocation policies (higher-level concerns)
- Heap metadata layout or slab coloring

## Decisions

### 1. Slab as address decomposition, not free-list management

**Choice:** Model slab allocation as arithmetic: slot_index, slot_address,
capacity, generation_valid. No free-list, no allocation state.

**Rationale:** The library models decisions, not data structures. The
interesting properties are the roundtrip (slot_address ∘ slot_index is
identity on aligned addresses), the capacity bound (slot_index < capacity),
and ABA safety (generation mismatch rejects stale handles). A free-list
would require mutable state and add no proof power for the arithmetic.

**Alternative:** Model a free-list with push/pop operations. Rejected —
that's allocator state, not a reusable arithmetic primitive. Downstream
allocators can compose the arithmetic with their own state tracking.

### 2. Handle packing with 32/32 split

**Choice:** Pack (index, generation) into a single u64 with 32 bits each.
Upper 32 bits for index, lower 32 for generation.

**Rationale:** The 32/32 split is the most common layout in practice.
4 billion slots and 4 billion generations per slot covers any realistic
slab. The pack/unpack roundtrip is verifiable with bitwise shift and mask
operations that Verus handles well.

**Alternative:** Configurable bit widths. Rejected — adds complexity
without meaningful proof differences. The roundtrip proof works the same
regardless of the split point.

### 3. Buddy as XOR with power-of-two arithmetic

**Choice:** `buddy_of(addr, order) = addr ^ (1 << order)`. The buddy
relationship is an XOR involution.

**Rationale:** XOR on the order bit is the standard buddy computation.
The key proof is involution: `buddy_of(buddy_of(addr, order), order) ==
addr` because XOR is its own inverse. This plus alignment checking gives
the coalescing decision. Block sizes are `1 << order`, linking directly
to the bitwise and integer modules.

**Implementation:** The `bitwise.rs` module already has `toggle_bit`,
which is exactly XOR with a single bit. `buddy_of` can be expressed as
`toggle_bit(addr, order)` when we extend to u64 addresses. The involution
proof follows from toggle-involution (already proven in properties.rs).

### 4. Region as (base, size) with overflow guards

**Choice:** Regions are `(base: u64, size: u64)` with the invariant
`base + size <= u64::MAX + 1` (i.e., the end address doesn't wrap).
Operations use checked or saturating arithmetic.

**Rationale:** Half-open intervals [base, base + size) with u64 addresses
can overflow. The interval module uses `(lo, hi)` pairs where `hi` is
the exclusive end — fine for abstract intervals but dangerous for address
arithmetic where `base + size` might wrap. Using `(base, size)` with
overflow guards matches how real systems represent memory regions (Linux
`struct resource`, Windows `MEMORY_BASIC_INFORMATION`).

**Alternative:** Use the interval module directly. Rejected — interval
uses `u64` lo/hi and doesn't guard against `lo + length` overflow. Region
ops need overflow safety as a first-class concern. The cross-module
properties prove agreement where overflow isn't a factor.

### 5. Watermark as monotonic state

**Choice:** Model watermark allocation as a monotonic value that advances
by aligned allocation sizes. The key invariant is that the watermark
never moves backward (except on explicit reset).

**Rationale:** A bump allocator's allocation frontier is a monotonic
counter — exactly what `monotonic.rs` models. The watermark starts at
base, advances by `align_up(current, align) + size` on each allocation,
and can only decrease via explicit reset. This gives non-overlap for free:
if allocation A returns address `a` and advances the watermark past
`a + size_a`, allocation B starts at or after that point.

The alignment step uses `integer::align_up`, linking to an existing
module. The remaining-space calculation mirrors `pages_needed` in
structure.

### 6. Phased implementation: region first

**Choice:** Build region → slab → buddy → watermark, one module per phase.

**Rationale:** Region is the simplest — it's interval arithmetic with
overflow guards. Many proofs delegate to existing interval proofs. Slab
builds on region (a slab is a region divided into equal slots). Buddy
builds on bitwise (XOR involution) and integer (power-of-two). Watermark
builds on integer (alignment) and monotonic (advance). Each phase ends
with verus + tracey clean.

**Phase 1 — Region** (`src/region.rs`)
- contains_addr, disjoint, contains_region, adjacent, merge, split
- Cross-module: region ↔ interval agreement proofs

**Phase 2 — Slab** (`src/slab.rs`)
- slot_index, slot_address, is_slot_aligned, capacity
- generation_valid, generation_increment
- handle_pack, handle_unpack_index, handle_unpack_generation
- Cross-module: slab ↔ page decomposition duality

**Phase 3 — Buddy** (`src/buddy.rs`)
- buddy_of, block_size, order_for_size, can_coalesce, parent_block,
  is_aligned_to_order
- Cross-module: buddy ↔ integer (log2, next_pow2), bitwise (toggle)

**Phase 4 — Watermark** (`src/watermark.rs`)
- alloc, after_alloc, remaining, fits, reset, is_empty
- Cross-module: watermark ↔ monotonic (advance), integer (align_up)

## Risks / Trade-offs

**[Overflow arithmetic in region module]** → Region operations need
`base + size` which can overflow u64. Using `requires base as int +
size as int <= u64::MAX as int + 1` as a precondition is safe but means
callers must prove non-overflow. Mitigation: this matches how real systems
work — regions that wrap around the address space are invalid.

**[Z3 nonlinear arithmetic for slab capacity]** → `slot_index < capacity`
requires `(addr - base) / slot_size < region_size / slot_size`, which is
integer division reasoning. Mitigation: use `lemma_fundamental_div_mod`
as in the page and chunk modules.

**[Buddy XOR reasoning]** → Verus handles bitwise XOR through `bit_xor`
but involution proofs may need manual bit-level reasoning. Mitigation:
the toggle-involution proof already exists in `properties.rs` for
`toggle_bit`. Buddy involution is the same proof at the u64 level.

**[Watermark alignment rounding]** → `align_up(current, align)` can
overflow when current is near u64::MAX. The existing `integer::align_up`
returns `Option<u64>` for this case. Watermark_alloc propagates the None.
No additional risk.
