## Why

The library has page-level address arithmetic (`page.rs`), copy-on-write
decisions (`cow.rs`), LRU eviction (`eviction.rs`), and alignment
primitives (`integer.rs`). These cover individual operations but not the
allocation decisions that tie them together.

Real allocators — slab allocators, buddy systems, bump arenas — make
decisions that are pure arithmetic: which slot does this address fall in?
What is the buddy of this block? Does this allocation fit? Can these two
free blocks coalesce? These decisions have algebraic properties (roundtrip
decomposition, power-of-two relationships, monotonic watermarks, region
disjointness) that map directly to Verus specs.

The same patterns appear across systems code. Slab allocation drives
kernel object caches and connection pools. Buddy systems underpin page
frame allocators and GPU memory managers. Bump allocation is the backbone
of arena allocators, network packet buffers, and scratch memory. Region
arithmetic shows up anywhere memory maps or address ranges need management.

None of these require new Verus techniques. The existing spec fn + exec
fn + ensures + proof fn pattern handles all of it. The proofs are
structural: slot decomposition mirrors page decomposition, buddy
addresses use the bitwise primitives already in the library, region
overlap delegates to interval overlap.

## What Changes

Four new modules covering allocation decision logic, plus cross-module
properties linking them to existing modules:

**Slab allocation arithmetic** (`src/slab.rs`)

Fixed-size slot allocation over a contiguous region. Given a base
address, slot size, and region capacity, compute which slot an
address belongs to, where a slot starts, and how many slots fit.
Generation counters for ABA-safe handle validation. Handle
packing combines an index and generation into a single u64.

Properties: slot_address ∘ slot_index is the identity on aligned
addresses. Generation validity is a monotonic predicate. Handle
pack/unpack round-trips. Slot capacity equals the dual of
pages_needed.

**Buddy system arithmetic** (`src/buddy.rs`)

Power-of-two block management for splitting and coalescing.
Compute a block's buddy address by flipping the order bit. Compute
block sizes from orders. Find the minimum order for a requested
size. Determine whether two buddies can coalesce.

Properties: buddy_of is an involution (buddy of buddy is self).
Coalescing two buddies produces a block at the next order. Block
size doubles with each order. order_for_size agrees with
log2_floor/next_power_of_two.

**Region arithmetic** (`src/region.rs`)

Address region management: overlap detection, containment, adjacency,
merge, and split. A region is (base, size) — equivalent to the
half-open interval [base, base + size) but with overflow-safe
arithmetic for 64-bit address spaces.

Properties: disjointness is symmetric. Containment is reflexive
and transitive. Merging adjacent regions recovers the combined
span. Splitting then merging is a roundtrip. Region operations
agree with interval module operations where intervals don't
overflow.

**Watermark allocation** (`src/watermark.rs`)

Bump/arena allocation decisions. A watermark tracks the current
allocation frontier within a fixed region. Allocation advances
the watermark by the requested size (with alignment). Remaining
space decreases monotonically. Reset to a saved mark reclaims
everything allocated after it.

Properties: successive allocations never overlap. Watermark
advance is a monotonic operation. Remaining + allocated = capacity.
Aligned watermark is always >= the unaligned position. Reset to
base reclaims everything.

**Cross-module properties** (in `src/properties.rs`)

- `slab::slot_address` ↔ `page::page_start` (same decomposition)
- `slab::slab_capacity` ↔ `page::pages_needed` (dual)
- `buddy::block_size` ↔ `integer::next_power_of_two`
- `buddy::order_for_size` ↔ `integer::log2_floor`
- `region::contains_addr` ↔ `interval::contains`
- `region::disjoint` ↔ `interval::overlaps` (negation)
- `region::adjacent` ↔ `interval::adjacent`
- `watermark::alloc` advance ↔ `monotonic::is_advance`

## Capabilities

### New Capabilities

- `slab-slot-decomposition`: Slot index/address roundtrip, capacity
- `slab-generation-safety`: ABA prevention via generation counters
- `slab-handle-packing`: Index+generation pack/unpack roundtrip
- `buddy-involution`: Buddy of buddy is self
- `buddy-coalesce-order`: Coalescing produces next-order block
- `buddy-size-doubling`: Block size doubles per order
- `buddy-order-agreement`: order_for_size matches log2/next_pow2
- `region-disjoint-symmetric`: Non-overlap is symmetric
- `region-containment-transitive`: Containment is reflexive+transitive
- `region-merge-split-roundtrip`: Split then merge recovers original
- `region-interval-agreement`: Region ops agree with interval ops
- `watermark-monotonic`: Allocation only moves forward
- `watermark-non-overlap`: Successive allocations don't overlap
- `watermark-capacity-conservation`: Remaining + used = capacity
- `watermark-reset-reclaim`: Reset to mark reclaims allocations

### Modified Capabilities

- `cross-module-properties`: ~8 new properties linking memory modules
  to existing page, integer, interval, monotonic modules

## Impact

- **Files**: 4 new `src/*.rs` modules, `src/properties.rs` additions,
  `src/lib.rs` module declarations, `docs/requirements.md` additions
- **APIs**: ~35–40 new pub fns across 4 modules
- **Dependencies**: None — pure logic using existing vstd
- **Testing**: Verus verification + proptest property tests
