# Memory Management Primitives Specification

## Purpose

Defines the requirements for verified memory management decision logic:
slab allocation arithmetic, buddy system block management, region
overlap/containment, and watermark (bump) allocation.

## Requirements

### Requirement: Slab Slot Index

r[slab.slot-index]
slot_index MUST return (addr - base) / slot_size. The result MUST be
less than slab_capacity(region_size, slot_size). slot_size MUST be
nonzero. addr MUST be >= base and < base + region_size.

#### Scenario: First slot

- GIVEN base = 0x1000, slot_size = 64, addr = 0x1000
- WHEN slot_index is called
- THEN the result is 0

#### Scenario: Later slot

- GIVEN base = 0x1000, slot_size = 64, addr = 0x1080
- WHEN slot_index is called
- THEN the result is 2

### Requirement: Slab Slot Address

r[slab.slot-address]
slot_address MUST return base + index * slot_size. The multiplication
MUST NOT overflow. The result MUST be page-aligned to slot_size. The
result MUST be >= base.

#### Scenario: Roundtrip on aligned address

- GIVEN base = 0x1000, slot_size = 64, index = 3
- WHEN slot_address is called
- THEN the result is 0x10C0

### Requirement: Slab Slot Alignment

r[slab.is-slot-aligned]
is_slot_aligned MUST return true if and only if (addr - base) %
slot_size == 0. slot_size MUST be nonzero. addr MUST be >= base.

#### Scenario: Aligned address

- GIVEN base = 0x1000, slot_size = 64, addr = 0x1080
- WHEN is_slot_aligned is called
- THEN the result is true

#### Scenario: Unaligned address

- GIVEN base = 0x1000, slot_size = 64, addr = 0x1001
- WHEN is_slot_aligned is called
- THEN the result is false

### Requirement: Slab Capacity

r[slab.capacity]
slab_capacity MUST return region_size / slot_size. slot_size MUST be
nonzero. slab_capacity * slot_size MUST NOT exceed region_size. The
remainder region_size % slot_size is wasted space.

#### Scenario: Exact fit

- GIVEN region_size = 256, slot_size = 64
- WHEN slab_capacity is called
- THEN the result is 4

#### Scenario: Partial last slot

- GIVEN region_size = 300, slot_size = 64
- WHEN slab_capacity is called
- THEN the result is 4 (300 / 64 = 4, remainder 44 wasted)

### Requirement: Generation Valid

r[slab.generation-valid]
generation_valid MUST return true if and only if handle_generation ==
current_generation. A mismatch indicates the slot has been freed and
reallocated (ABA scenario). Matching generations confirm the handle
still refers to the same allocation.

#### Scenario: Valid handle

- GIVEN handle_generation = 5, current_generation = 5
- WHEN generation_valid is called
- THEN the result is true

#### Scenario: Stale handle

- GIVEN handle_generation = 3, current_generation = 5
- WHEN generation_valid is called
- THEN the result is false

### Requirement: Generation Increment

r[slab.generation-increment]
generation_increment MUST return current_generation + 1, saturating at
u64::MAX. The result MUST always be > current_generation when not
saturated.

### Requirement: Handle Pack

r[slab.handle-pack]
handle_pack MUST combine index and generation into a single u64 by
placing index in the upper 32 bits and generation in the lower 32 bits.
index MUST fit in 32 bits. generation MUST fit in 32 bits.

### Requirement: Handle Unpack Index

r[slab.handle-unpack-index]
handle_unpack_index MUST return the upper 32 bits of the packed handle.
handle_unpack_index(handle_pack(index, generation)) MUST equal index.

### Requirement: Handle Unpack Generation

r[slab.handle-unpack-generation]
handle_unpack_generation MUST return the lower 32 bits of the packed
handle. handle_unpack_generation(handle_pack(index, generation)) MUST
equal generation.

### Requirement: Buddy Address

r[buddy.buddy-of]
buddy_of MUST return addr XOR (1 << order). The buddy address MUST
differ from addr in exactly the order bit. addr MUST be aligned to
2^order. order MUST be less than 64.

#### Scenario: Order-0 buddy

- GIVEN addr = 0x1000, order = 0
- WHEN buddy_of is called
- THEN the result is 0x1001

#### Scenario: Order-4 buddy

- GIVEN addr = 0x1000, order = 4
- WHEN buddy_of is called
- THEN the result is 0x1010

### Requirement: Block Size

r[buddy.block-size]
block_size MUST return 1 << order (i.e., 2^order). order MUST be less
than 64. The result MUST be a power of two.

#### Scenario: Order 0

- GIVEN order = 0
- WHEN block_size is called
- THEN the result is 1

#### Scenario: Order 12 (page size)

- GIVEN order = 12
- WHEN block_size is called
- THEN the result is 4096

### Requirement: Order For Size

r[buddy.order-for-size]
order_for_size MUST return the minimum order such that block_size(order)
>= size. size MUST be > 0. The result MUST satisfy block_size(result) >=
size. If result > 0, then block_size(result - 1) < size.

#### Scenario: Exact power of two

- GIVEN size = 64
- WHEN order_for_size is called
- THEN the result is 6 (2^6 = 64)

#### Scenario: Non-power-of-two

- GIVEN size = 65
- WHEN order_for_size is called
- THEN the result is 7 (2^7 = 128 >= 65)

### Requirement: Can Coalesce

r[buddy.can-coalesce]
can_coalesce MUST return true if and only if addr and buddy_addr are
buddies at the given order: buddy_of(addr, order) == buddy_addr. Both
addresses MUST be aligned to 2^order.

### Requirement: Parent Block

r[buddy.parent-block]
parent_block MUST return the smaller of addr and buddy_of(addr, order).
The result MUST be aligned to 2^(order + 1). order MUST be less than 63
(room for parent order).

#### Scenario: Lower buddy

- GIVEN addr = 0x1000, order = 4
- WHEN parent_block is called
- THEN the result is 0x1000 (addr < buddy)

#### Scenario: Upper buddy

- GIVEN addr = 0x1010, order = 4
- WHEN parent_block is called
- THEN the result is 0x1000 (buddy < addr)

### Requirement: Block Aligned

r[buddy.is-aligned]
is_aligned_to_order MUST return true if and only if addr % (1 << order)
== 0. order MUST be less than 64.

### Requirement: Region Contains Address

r[region.contains-addr]
region_contains_addr MUST return true if and only if addr >= base AND
addr < base + size, using saturating addition for overflow safety. An
empty region (size == 0) MUST contain no address.

#### Scenario: Address within region

- GIVEN base = 0x1000, size = 0x100, addr = 0x1050
- WHEN region_contains_addr is called
- THEN the result is true

#### Scenario: Address at boundary

- GIVEN base = 0x1000, size = 0x100, addr = 0x1100
- WHEN region_contains_addr is called
- THEN the result is false (half-open: end is exclusive)

### Requirement: Regions Disjoint

r[region.disjoint]
regions_disjoint MUST return true if and only if the two regions share
no address. Empty regions MUST be disjoint from everything. Disjoint
MUST be symmetric: regions_disjoint(a, b) == regions_disjoint(b, a).

#### Scenario: Non-overlapping

- GIVEN region A = (0x1000, 0x100), region B = (0x1100, 0x100)
- WHEN regions_disjoint is called
- THEN the result is true

#### Scenario: Overlapping

- GIVEN region A = (0x1000, 0x100), region B = (0x1080, 0x100)
- WHEN regions_disjoint is called
- THEN the result is false

### Requirement: Region Contains Region

r[region.contains-region]
region_contains_region MUST return true if and only if every address in
the inner region is also in the outer region. An empty inner region MUST
always be contained. Containment MUST be reflexive and transitive.

### Requirement: Region Adjacent

r[region.adjacent]
region_adjacent MUST return true if and only if one region ends where
the other begins (base_a + size_a == base_b OR base_b + size_b ==
base_a), using overflow-safe arithmetic. Adjacent MUST be symmetric.
Empty regions MUST NOT be adjacent.

### Requirement: Region Merge

r[region.merge]
region_merge MUST combine two adjacent regions into one covering both.
The result base MUST be min(base_a, base_b). The result size MUST be
size_a + size_b. The regions MUST be adjacent. Merge of a split MUST
recover the original region.

### Requirement: Region Split

r[region.split]
region_split MUST divide a region at the given offset into two adjacent
sub-regions: [base, base + offset) and [base + offset, base + size).
offset MUST be > 0 and < size. The two results MUST be adjacent. Their
sizes MUST sum to the original size.

### Requirement: Watermark Allocate

r[watermark.alloc]
watermark_alloc MUST return Some(aligned_current) when the allocation
fits (aligned_current + size <= limit), or None when it does not.
aligned_current is current rounded up to the given alignment. The
returned address MUST be >= current and aligned to the given alignment.
alignment MUST be a power of two and nonzero.

#### Scenario: Allocation fits

- GIVEN current = 100, size = 32, align = 8, limit = 200
- WHEN watermark_alloc is called
- THEN the result is Some(104) (100 rounded up to 8-byte alignment)

#### Scenario: Allocation does not fit

- GIVEN current = 180, size = 32, align = 8, limit = 200
- WHEN watermark_alloc is called
- THEN the result is None (184 + 32 = 216 > 200)

### Requirement: Watermark After Alloc

r[watermark.after-alloc]
watermark_after_alloc MUST return aligned_current + size. The result
MUST be > current (non-zero-size allocations always advance). The
result MUST be <= limit.

### Requirement: Watermark Remaining

r[watermark.remaining]
watermark_remaining MUST return limit - current when current <= limit,
and 0 otherwise. remaining + current MUST equal limit when current <=
limit.

#### Scenario: Space available

- GIVEN current = 100, limit = 200
- WHEN watermark_remaining is called
- THEN the result is 100

#### Scenario: At limit

- GIVEN current = 200, limit = 200
- WHEN watermark_remaining is called
- THEN the result is 0

### Requirement: Watermark Fits

r[watermark.fits]
watermark_fits MUST return true if and only if watermark_alloc would
return Some. This is a read-only check that does not modify the
watermark. alignment MUST be a power of two and nonzero.

### Requirement: Watermark Reset

r[watermark.reset]
watermark_reset MUST return the saved mark value. The mark MUST be >=
base and <= current. Reset reclaims all memory allocated after the mark.
reset(base) reclaims everything.

### Requirement: Watermark Is Empty

r[watermark.is-empty]
watermark_is_empty MUST return true if and only if current == base.
An empty watermark has allocated nothing.
