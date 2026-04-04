## Why

The library has page-level address arithmetic (`page.rs`: frame number,
offset, alignment, pages_needed), multi-bit field extraction (`bitfield.rs`:
extract/insert/clear fields), region management (`region.rs`: contains,
disjoint, merge, split), and process state machines (`process_state.rs`).
The memory-management-primitives change explicitly deferred virtual memory
page table walks as a non-goal.

That deferral leaves a gap. MMU address translation is the most
safety-critical piece of any OS or hypervisor — a single wrong bit in a
page table entry maps memory to the wrong physical frame, breaking
isolation between processes or between a guest and its host. The
arithmetic is pure: extracting VPN fields from a virtual address,
parsing/constructing PTE bit layouts, walking a radix tree of PTEs,
classifying faults, checking permissions, and modeling TLB caching. All
of it is deterministic decision logic with algebraic properties that Verus
can verify.

The building blocks already exist. PTE field parsing is `bitfield`
applied to a specific layout. VPN extraction is `page` decomposition
with 9-bit "page sizes." Permission checking is a matrix lookup. TLB
correctness reduces to proving cached translations agree with walk
results. ASID management reuses the generation-counter pattern from
`slab`.

What's missing is the composition: the modules that combine these
building blocks into a verified MMU primitive toolkit, with proofs that
the pieces agree — that parsing a PTE then reconstructing it recovers
the original word, that a page walk through valid tables always produces
an address within the mapped physical region, that a TLB hit returns
the same answer as a fresh walk.

Several of the MMU operations decompose further into reusable primitives.
The page walk splits into per-level PTE interpretation (useful on its
own for constructing or validating mappings) and a thin multi-level
iteration. The TLB is a bounded tagged cache — a pattern that shows up
in DNS caches, routing tables, and session stores. ASID allocation is
a bounded ID pool — the same pattern as process IDs, file descriptors,
and hardware interrupt vectors. Sign-extension checking (for canonical
addresses) and radix index extraction (for VPN fields) are general
bitwise primitives that belong in the existing modules.

## What Changes

Eleven modules total: four reusable primitives (three new modules plus
two additions to existing modules), and seven MMU-specific modules that
compose them. Plus cross-module properties.

### Reusable Primitives

**Sign-extension check** (added to `src/bitwise.rs`)

Check whether bits [63:width] of a u64 are all copies of bit
[width-1]. Used by VPN canonical address checking but applicable
to any ISA with sign-extended fields (RISC-V immediates, x86-64
canonical addresses).

Properties: sign_extended(value, 64) is always true.
sign_extended(value, 0) checks all 64 bits are copies of bit 0.
A value that is sign-extended at width W is also sign-extended
at any width ≥ W.

**Radix index extraction** (added to `src/page.rs`)

Extract the level-th group of N bits from a value, starting at
a base offset. Generalizes VPN extraction: `vpn[i] =
radix_index(va, i, 9, 12)`. Also useful for IP routing prefix
tries, inode trees, and hash array mapped tries.

Properties: result < 2^bits_per_level. All levels reconstruct
the original field range. Agrees with bitfield::extract_field
at the computed offset.

**Walk-level PTE interpretation** (`src/walk_level.rs`)

Single-level page table step: given a table base and VPN index,
compute the PTE address. Given a PTE and its level, classify it
as valid leaf, valid non-leaf, or specific fault. For leaves,
compute the physical address from the PPN and VA offset bits
(level-dependent width for superpages). For non-leaves, compute
the next-level table base from PPN.

Properties: PTE address < table_base + 4096 (within the table
page). Leaf PA falls within the PPN's physical region.
Superpage size is a power of two. Next table base is page-aligned.

**Bounded tagged cache** (`src/tagged_lookup.rs`)

Bounded associative lookup from (tag, key) → value with
selective invalidation. Operations: lookup by (tag, key),
insert with capacity-bounded eviction, remove all entries by
tag, remove all entries by key, and clear all. The TLB is one
instantiation; the pattern applies to any tagged cache.

Properties: insert-then-lookup roundtrip. Remove-by-tag
completeness (no matching entries remain). Count ≤ capacity
after any operation. Clear → count == 0.

**Bounded ID pool** (`src/bounded_id_pool.rs`)

Allocate unique IDs from a bounded range [1, max_id]. Deallocate
returns an ID to the free set. The ASID pool is one instantiation;
the pattern applies to process IDs, file descriptors, hardware
resource IDs.

Properties: allocated IDs are unique. Count ≤ max_id. ID 0 is
never allocated. Deallocate-then-allocate can return the same ID.

### MMU-Specific Modules

**Page table entry parsing** (`src/pte.rs`)

Parse and construct RISC-V Sv39/Sv48 page table entries using
`bitfield` extraction. A PTE is a u64 with defined bit fields:
V (valid), R/W/X (permissions), U (user), G (global), A (accessed),
D (dirty), RSW (software), and PPN (physical page number). Parse
a u64 into its constituent fields, construct a PTE from fields,
and classify a PTE as leaf vs non-leaf. Dirty/accessed bit
management: update_accessed, update_dirty, clear_accessed_dirty.

Properties: construct-then-parse roundtrip recovers all fields.
Parse-then-construct roundtrip recovers the original word (when
reserved bits are zero). Leaf detection agrees with R|W|X != 0.
PPN extraction agrees with bitfield::extract_field at the correct
offset/width. Dirty implies accessed. Update operations are
idempotent.

**Virtual address decomposition** (`src/vpn.rs`)

Split a virtual address into VPN fields and page offset for
multi-level page table indexing. Delegates to
page::radix_index for field extraction and
bitwise::is_sign_extended for canonical checking. Sv39 uses
three 9-bit VPN fields plus a 12-bit page offset. Sv48 adds a
fourth level.

Properties: decomposition roundtrip (via radix_index roundtrip).
Each VPN field < 512, offset < 4096 (via radix_index boundedness).
Canonical check delegates to is_sign_extended at width 39/48.

**PTE permission checking** (`src/pte_permission.rs`)

Check whether an access type (load, store, fetch) is permitted by
a PTE's R/W/X bits under a given privilege mode. Models MXR and
SUM modifiers.

Properties: permission monotonicity. MXR widening. SUM widening.

**Page table walk** (`src/page_walk.rs`)

Multi-level walk composing walk_level::interpret_pte at each
level. Thin iteration: extract VPN at level, call walk_level,
either descend or produce result. Sv39 (3 levels) and Sv48
(4 levels).

Properties: determinism (delegates to walk_level determinism per
step). Bounded steps (structural from level count). PA-in-region
(delegates to walk_level::leaf_pa proof).

**Page fault classification** (`src/page_fault.rs`)

Five fault reasons as an enum. classify_fault checks in priority
order: reserved bits → not present → invalid permission encoding
→ permission violation → privilege violation → misaligned superpage.

Properties: exhaustive (total over failed walks). Exclusive (at
most one reason). Valid-implies-clear (no fault means all checks
pass).

**TLB model** (`src/tlb.rs`)

Thin wrapper over tagged_lookup instantiated with (ASID, VPN) →
(PPN, permissions, page_size). Adds global-entry semantics: G=1
entries match any ASID and survive ASID-specific flushes.

Properties: soundness (hit agrees with walk, given unchanged
tables). Global entries match any ASID. ASID flush preserves
global entries.

**ASID management** (`src/asid.rs`)

Wrapper over bounded_id_pool adding generation-based reuse and
TLB flush coordination. Generation rollover frees all ASIDs and
sets flush_needed. Context switch detects stale generations.

Properties: unique-in-generation (from pool uniqueness + generation
stamp). Rollover → flush_needed. Generation follows slab pattern.

**Cross-module properties** (in `src/properties.rs`)

- `pte` field extraction agrees with `bitfield::extract_field`
- `vpn` offset agrees with `page::page_offset`
- `vpn` extraction uses `page::radix_index`
- `vpn` canonical uses `bitwise::is_sign_extended`
- `walk_level` leaf PA satisfies `region::contains_addr`
- `page_walk` delegates to `walk_level` per step
- `pte_permission` full-access matches `access_mode` FULL mode
- `tlb` delegates to `tagged_lookup` operations
- `tlb` soundness: lookup agrees with `page_walk`
- `asid` delegates to `bounded_id_pool` + `slab` generations
- `page_fault` classification partitions walk outcomes

## Capabilities

### New Capabilities

Reusable primitives:
- `sign-extended-check`: Bits above width are copies of sign bit
- `sign-extended-monotonic`: Width W implies width ≥ W
- `radix-index-bounded`: Result < 2^bits_per_level
- `radix-index-roundtrip`: All levels reconstruct the field range
- `tagged-lookup-roundtrip`: Insert then lookup returns entry
- `tagged-remove-complete`: Remove-by-tag leaves no matching entries
- `tagged-capacity-bounded`: Count ≤ capacity after any operation
- `id-pool-unique`: Allocated IDs are unique
- `id-pool-bounded`: Count ≤ max_id, ID 0 never allocated
- `walk-level-pa-in-region`: Leaf PA within PPN's physical region
- `walk-level-pte-addr-bounded`: PTE address within table page
- `walk-level-superpage-pow2`: Superpage size is a power of two

MMU-specific:
- `pte-roundtrip`: Construct-then-parse and parse-then-construct
- `pte-leaf-detection`: Leaf iff R|W|X != 0
- `pte-ppn-bitfield`: PPN agrees with bitfield at known offsets
- `pte-reserved-clear`: construct_pte has reserved bits zero
- `pte-dirty-implies-accessed`: D=1 → A=1
- `pte-update-idempotent`: update_accessed/update_dirty idempotent
- `vpn-decomposition-roundtrip`: VPN fields + offset reconstruct VA
- `vpn-field-bounded`: Each VPN < 512, offset < 4096
- `vpn-canonical`: Sign extension via is_sign_extended
- `walk-deterministic`: Same tables + same VA → same result
- `walk-bounded-steps`: Terminates in ≤ N levels
- `fault-exhaustive`: Every failed walk → exactly one fault
- `fault-exclusive`: No two simultaneous fault reasons
- `fault-valid-implies-clear`: No fault → all checks pass
- `permission-monotonic`: Adding R/W/X bits never denies access
- `permission-mxr-widens`: MXR=true allows ≥ MXR=false
- `permission-sum-widens`: SUM=true allows ≥ SUM=false
- `tlb-soundness`: Hit returns same answer as walk
- `tlb-global-any-asid`: G=1 entries match any ASID
- `tlb-asid-flush-preserves-global`: ASID flush keeps G=1 entries
- `asid-unique-in-generation`: No duplicate ASIDs within generation
- `asid-rollover-flush`: Generation rollover requires TLB flush
- `asid-pool-bounded`: Never exceeds hardware maximum

### Modified Capabilities

- `cross-module-properties`: ~11 new properties linking MMU and
  reusable modules to existing page, bitfield, region, access_mode,
  slab modules

## Impact

- **Files**: 5 new `src/*.rs` modules (walk_level, tagged_lookup,
  bounded_id_pool, pte, vpn, pte_permission, page_walk, page_fault,
  tlb, asid), additions to `src/bitwise.rs` and `src/page.rs`,
  `src/properties.rs` additions, `src/lib.rs` module declarations,
  `docs/requirements.md` additions
- **APIs**: ~80–90 new pub fns across 9 new modules + 2 extended modules
- **Dependencies**: None — pure logic using existing vstd
- **Testing**: Verus verification + proptest property tests
