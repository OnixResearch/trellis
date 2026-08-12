# MMU Primitives Specification

## Purpose

Defines the requirements for verified MMU decision logic and the reusable
primitives extracted from it. Reusable primitives: sign-extension checking,
radix index extraction, single-level walk interpretation, bounded tagged
cache, and bounded ID pool. MMU-specific: PTE parsing/construction,
virtual address decomposition, PTE permission checking, multi-level page
walks, page fault classification, TLB modeling, and ASID management.
Targets RISC-V Sv39/Sv48 formats; core invariants are architecture-independent.

## Requirements

### Sign-Extension Check (addition to bitwise.rs)

### Requirement: Is Sign Extended
r[bitwise.is-sign-extended]

is_sign_extended(value, width) MUST return true if and only if bits
[63:width] of value are all copies of bit [width-1]. width MUST be
in [1, 64]. is_sign_extended(value, 64) MUST always return true.

#### Scenario: Positive sign extension (Sv39)

- GIVEN value = 0x0000_003F_FFFF_FFFF, width = 39
- WHEN is_sign_extended is called
- THEN the result is true (bit 38 = 0, upper bits = 0)

#### Scenario: Negative sign extension (Sv39)

- GIVEN value = 0xFFFF_FFC0_0000_0000, width = 39
- WHEN is_sign_extended is called
- THEN the result is true (bit 38 = 1, upper bits = 1)

#### Scenario: Not sign extended

- GIVEN value = 0x0000_0040_0000_0000, width = 39
- WHEN is_sign_extended is called
- THEN the result is false (bit 38 = 1, upper bits = 0)

### Requirement: Sign Extension Monotonic
r[bitwise.sign-extended-monotonic]

If is_sign_extended(value, w1) is true and w2 >= w1, then
is_sign_extended(value, w2) MUST also be true. Wider widths are
weaker constraints.

### Requirement: Sign Extension Full Width
r[bitwise.sign-extended-full]

is_sign_extended(value, 64) MUST return true for any value. At full
width there are no upper bits to check.

### Radix Index Extraction (addition to page.rs)

### Requirement: Radix Index
r[page.radix-index]

radix_index(value, level, bits_per_level, base_offset) MUST return
the field at bit position (base_offset + level * bits_per_level) with
width bits_per_level. This MUST agree with
bitfield::extract_field(value, base_offset + level * bits_per_level,
bits_per_level). bits_per_level MUST be > 0. The computed offset +
width MUST be <= 64.

#### Scenario: Sv39 VPN[0]

- GIVEN value = va, level = 0, bits_per_level = 9, base_offset = 12
- WHEN radix_index is called
- THEN the result is bits [20:12] of va

#### Scenario: Sv39 VPN[2]

- GIVEN value = va, level = 2, bits_per_level = 9, base_offset = 12
- WHEN radix_index is called
- THEN the result is bits [38:30] of va

### Requirement: Radix Index Bounded
r[page.radix-index-bounded]

radix_index MUST return a value strictly less than 2^bits_per_level.
For bits_per_level = 9, the result MUST be less than 512.

### Requirement: Radix Index Roundtrip
r[page.radix-index-roundtrip]

For a value composed of N levels of bits_per_level bits starting at
base_offset, extracting all N levels via radix_index and recombining
them (each shifted to its original position) MUST recover the
original bits in the range [base_offset, base_offset + N *
bits_per_level).

### Requirement: Radix Index Agrees With Bitfield
r[page.radix-index-bitfield]

radix_index(value, level, bits_per_level, base_offset) MUST equal
bitfield::extract_field(value, base_offset + level * bits_per_level,
bits_per_level) for valid parameters.

### Walk-Level PTE Interpretation

### Requirement: PTE Address
r[walk-level.pte-addr]

pte_addr(table_base, vpn_index) MUST return table_base + vpn_index *
PTE_SIZE (where PTE_SIZE = 8). vpn_index MUST be less than 512. The
result MUST be >= table_base and < table_base + 4096.

#### Scenario: First entry

- GIVEN table_base = 0x80000000, vpn_index = 0
- WHEN pte_addr is called
- THEN the result is 0x80000000

#### Scenario: Entry at index 5

- GIVEN table_base = 0x80000000, vpn_index = 5
- WHEN pte_addr is called
- THEN the result is 0x80000028

### Requirement: Interpret PTE
r[walk-level.interpret]

interpret_pte(pte_fields, level) MUST classify the PTE at a given
level as one of: VALID_LEAF (is_leaf and V=1), VALID_NON_LEAF
(not leaf and V=1), or a specific fault reason (not valid, reserved
bits set, invalid encoding, permission/privilege violation,
misaligned superpage). The classification MUST be exhaustive and
exclusive over valid level values.

#### Scenario: Valid leaf at level 0

- GIVEN PTE with V=1, R=1, W=0, X=0, level=0
- WHEN interpret_pte is called
- THEN the result is VALID_LEAF

#### Scenario: Valid non-leaf at level 2

- GIVEN PTE with V=1, R=0, W=0, X=0, level=2
- WHEN interpret_pte is called
- THEN the result is VALID_NON_LEAF

#### Scenario: Invalid PTE

- GIVEN PTE with V=0
- WHEN interpret_pte is called
- THEN the result is a NOT_PRESENT fault

### Requirement: Leaf PA Computation
r[walk-level.leaf-pa]

leaf_pa(ppn, level, va) MUST compute the physical address from a
leaf PTE's PPN and the virtual address. For level 0 (4 KiB): PA =
ppn * 4096 + va[11:0]. For level 1 (2 MiB superpage): PA =
ppn[43:9] << 21 | va[20:0]. For level 2 (1 GiB superpage): PA =
ppn[43:18] << 30 | va[29:0]. The PA MUST fall within
[ppn_base, ppn_base + page_size) where ppn_base is the page-aligned
start and page_size is the size at the given level.

#### Scenario: 4 KiB page

- GIVEN ppn = 0x12345, level = 0, va with offset = 0xABC
- WHEN leaf_pa is called
- THEN PA = 0x12345ABC

#### Scenario: 2 MiB superpage

- GIVEN ppn with PPN[0]=0, level = 1, va with bits[20:0] = 0x1ABCDE
- WHEN leaf_pa is called
- THEN PA includes PPN[2:1] shifted left by 21, OR'd with va[20:0]

### Requirement: Leaf PA In Region
r[walk-level.leaf-pa-in-region]

leaf_pa(ppn, level, va) MUST produce a PA that satisfies
region::contains_addr(ppn_base, page_size_at_level, PA) where
ppn_base is the aligned physical base for the given PPN and level.

### Requirement: Next Table Base
r[walk-level.next-table-base]

next_table_base(ppn) MUST return ppn * 4096. The result MUST be
page-aligned (via page::is_page_aligned). This is the base address
of the next-level page table.

### Requirement: Superpage Aligned
r[walk-level.superpage-aligned]

is_superpage_aligned(ppn, level) MUST return true if and only if
the PPN bits corresponding to lower levels are zero. For level 1:
PPN[0] (bits 0–8 of ppn) MUST be zero. For level 2: PPN[0] and
PPN[1] (bits 0–17 of ppn) MUST both be zero. Level 0 MUST always
return true (no superpage alignment needed).

#### Scenario: Aligned level-1 superpage

- GIVEN ppn with PPN[0] = 0, level = 1
- WHEN is_superpage_aligned is called
- THEN the result is true

#### Scenario: Misaligned level-1 superpage

- GIVEN ppn with PPN[0] = 7, level = 1
- WHEN is_superpage_aligned is called
- THEN the result is false

### Requirement: Superpage Size
r[walk-level.superpage-size]

superpage_size(level) MUST return 4096 for level 0, 2^21 (2 MiB)
for level 1, and 2^30 (1 GiB) for level 2. The result MUST be a
power of two. The result MUST equal 4096 * 512^level.

### Requirement: PTE Address Bounded
r[walk-level.pte-addr-bounded]

pte_addr(table_base, vpn_index) MUST satisfy
region::contains_addr(table_base, 4096, result). The PTE address
MUST fall within the table page.

### Bounded Tagged Cache

### Requirement: Tagged Lookup
r[tagged-lookup.lookup]

tagged_lookup(entries, tag, key) MUST return the value associated
with the first entry matching (tag, key), or None if no match
exists.

#### Scenario: Match found

- GIVEN entries containing (tag=5, key=100) → value=42
- WHEN tagged_lookup(entries, 5, 100) is called
- THEN the result is Some(42)

#### Scenario: No match

- GIVEN entries not containing (tag=5, key=200)
- WHEN tagged_lookup(entries, 5, 200) is called
- THEN the result is None

### Requirement: Tagged Insert
r[tagged-lookup.insert]

tagged_insert(entries, tag, key, value, capacity) MUST add an
entry for (tag, key) → value. If an entry with the same (tag, key)
already exists, it MUST be replaced. If the entry count equals
capacity and no existing entry is replaced, one entry MUST be
evicted. After insert, tagged_lookup(entries', tag, key) MUST
return Some(value).

### Requirement: Tagged Remove By Tag
r[tagged-lookup.remove-by-tag]

tagged_remove_by_tag(entries, tag) MUST remove all entries whose
tag matches. After removal, tagged_lookup(entries', tag, any_key)
MUST return None. Entries with other tags MUST be preserved.

### Requirement: Tagged Remove By Key
r[tagged-lookup.remove-by-key]

tagged_remove_by_key(entries, key) MUST remove all entries whose
key matches, regardless of tag. After removal,
tagged_lookup(entries', any_tag, key) MUST return None.

### Requirement: Tagged Remove All
r[tagged-lookup.remove-all]

tagged_remove_all(entries) MUST return an empty entry set.
tagged_count after remove-all MUST be zero.

### Requirement: Tagged Count
r[tagged-lookup.count]

tagged_count(entries) MUST return the number of entries. The count
MUST never exceed the configured capacity after any insert operation.

### Requirement: Tagged Insert Lookup Roundtrip
r[tagged-lookup.insert-lookup-roundtrip]

tagged_lookup(tagged_insert(entries, t, k, v, cap), t, k) MUST
return Some(v). Insert then lookup with the same (tag, key) MUST
always succeed.

### Requirement: Tagged Remove Complete
r[tagged-lookup.remove-complete]

After tagged_remove_by_tag(entries, t), for all keys k,
tagged_lookup(entries', t, k) MUST return None. No entries with
the removed tag may remain.

### Requirement: Tagged Capacity Bounded
r[tagged-lookup.capacity-bounded]

tagged_count(entries) MUST be <= capacity after any sequence of
insert and remove operations.

### Bounded ID Pool

### Requirement: Pool Allocate
r[bounded-id-pool.allocate]

pool_allocate(pool) MUST return Some(id) where id is in [1, max_id]
and id is not currently allocated, or None if all IDs are in use.
After allocation, pool_contains(pool', id) MUST be true.

#### Scenario: Allocation succeeds

- GIVEN a pool with max_id=255, 100 allocated
- WHEN pool_allocate is called
- THEN a fresh ID in [1, 255] is returned

#### Scenario: Pool exhausted

- GIVEN a pool with max_id=255, 255 allocated
- WHEN pool_allocate is called
- THEN None is returned

### Requirement: Pool Deallocate
r[bounded-id-pool.deallocate]

pool_deallocate(pool, id) MUST return id to the free set. id MUST
have been previously allocated. After deallocation,
pool_contains(pool', id) MUST be false.

### Requirement: Pool Contains
r[bounded-id-pool.contains]

pool_contains(pool, id) MUST return true if and only if id is
currently allocated.

### Requirement: Pool Active Count
r[bounded-id-pool.active-count]

pool_active_count(pool) MUST return the number of currently
allocated IDs. active_count MUST increase by 1 on allocate and
decrease by 1 on deallocate.

### Requirement: Pool Is Exhausted
r[bounded-id-pool.is-exhausted]

pool_is_exhausted(pool) MUST return true if and only if
active_count == max_id.

### Requirement: Pool IDs Unique
r[bounded-id-pool.unique]

pool_allocate MUST never return an ID that is already allocated.
No two concurrent allocations (without intervening deallocate)
MUST produce the same ID.

### Requirement: Pool Bounded
r[bounded-id-pool.bounded]

pool_active_count(pool) MUST never exceed max_id. ID 0 MUST
never be allocated.

### Requirement: Pool Recycle
r[bounded-id-pool.recycle]

After pool_deallocate(pool, id) and then pool_allocate(pool'),
the returned ID MAY equal id (the freed ID is eligible for reuse).

### Page Table Entry (PTE)

### Requirement: PTE Field Layout
r[pte.field-layout]

A RISC-V Sv39/Sv48 PTE is a 64-bit word with the following fields:
V (bit 0), R (bit 1), W (bit 2), X (bit 3), U (bit 4), G (bit 5),
A (bit 6), D (bit 7), RSW (bits 8–9), PPN[0] (bits 10–18),
PPN[1] (bits 19–27), PPN[2] (bits 28–53), and reserved (bits 54–63).
All field extractions MUST use the specified bit positions.

### Requirement: PTE Parse
r[pte.parse]

parse_pte MUST extract all fields from a 64-bit PTE word. Each field
MUST agree with bitfield::extract_field at the corresponding offset
and width. Parsing MUST be a pure function of the input word.

#### Scenario: Valid leaf PTE

- GIVEN a PTE word with V=1, R=1, W=0, X=0, PPN=0x12345
- WHEN parse_pte is called
- THEN V is true, R is true, W is false, X is false, PPN is 0x12345

#### Scenario: Non-leaf PTE

- GIVEN a PTE word with V=1, R=0, W=0, X=0, PPN=0xABCDE
- WHEN parse_pte is called
- THEN V is true, R/W/X are all false, PPN is 0xABCDE

### Requirement: PTE Construct
r[pte.construct]

construct_pte MUST build a 64-bit PTE word from individual fields
using bitfield::insert_field at the correct offsets. Reserved bits
(54–63) MUST be zero in the constructed word.

#### Scenario: Construct read-only leaf

- GIVEN V=1, R=1, W=0, X=0, U=1, PPN=0x80000
- WHEN construct_pte is called
- THEN the result word has V=1, R=1, U=1 at their bit positions and PPN=0x80000

### Requirement: PTE Roundtrip
r[pte.roundtrip]

parse_pte(construct_pte(fields)) MUST recover all original fields.
construct_pte(parse_pte(word)) MUST recover the original word when
reserved bits (54–63) are zero.

#### Scenario: Construct-then-parse

- GIVEN fields V=1, R=1, W=1, X=0, U=0, G=0, A=1, D=1, PPN=0x1000
- WHEN construct_pte then parse_pte is applied
- THEN all fields match the originals

#### Scenario: Parse-then-construct

- GIVEN a word with reserved bits zero
- WHEN parse_pte then construct_pte is applied
- THEN the result equals the original word

### Requirement: PTE Leaf Detection
r[pte.is-leaf]

is_leaf_pte MUST return true if and only if at least one of R, W, X
is set (nonzero). A PTE with V=1 and R=W=X=0 is a non-leaf (pointer
to next-level table).

#### Scenario: Leaf PTE

- GIVEN R=1, W=0, X=0
- WHEN is_leaf_pte is called
- THEN the result is true

#### Scenario: Non-leaf PTE

- GIVEN R=0, W=0, X=0
- WHEN is_leaf_pte is called
- THEN the result is false

### Requirement: PTE Reserved Clear
r[pte.reserved-clear]

has_reserved_bits MUST return true if any of bits 54–63 are nonzero.
A well-formed PTE MUST have has_reserved_bits == false. construct_pte
MUST always produce a PTE with has_reserved_bits == false.

### Requirement: PTE Write-No-Read Invalid
r[pte.write-no-read]

A PTE with W=1 and R=0 is invalid per the RISC-V specification.
is_valid_permission_bits MUST return false when W=1 and R=0.

#### Scenario: W=1, R=0

- GIVEN W=1, R=0
- WHEN is_valid_permission_bits is called
- THEN the result is false

#### Scenario: W=1, R=1

- GIVEN W=1, R=1
- WHEN is_valid_permission_bits is called
- THEN the result is true

### Dirty/Accessed Bit Management

### Requirement: Dirty Implies Accessed
r[dirty-accessed.dirty-implies-accessed]

If the D (dirty) bit is set, the A (accessed) bit MUST also be
set. A PTE with D=1, A=0 is malformed.

#### Scenario: Dirty and accessed

- GIVEN D=1, A=1
- WHEN is_well_formed is called
- THEN the result is true

#### Scenario: Dirty without accessed

- GIVEN D=1, A=0
- WHEN is_well_formed is called
- THEN the result is false

### Requirement: Update Accessed
r[dirty-accessed.update-accessed]

update_accessed MUST set the A bit to 1 without modifying any
other bits. If A is already 1, the operation MUST be idempotent.

### Requirement: Update Dirty
r[dirty-accessed.update-dirty]

update_dirty MUST set both the D bit and the A bit to 1 without
modifying any other bits. Setting D MUST always also set A
(maintaining the dirty-implies-accessed invariant).

### Requirement: Clear Accessed Dirty
r[dirty-accessed.clear-for-eviction]

clear_accessed_dirty MUST set both A and D to 0 without modifying
other bits. After clearing, the PTE MUST have A=0 and D=0.

### Virtual Address Decomposition (VPN)

### Requirement: Sv39 VPN Extraction
r[vpn.sv39-extract]

vpn_extract_sv39 MUST split a 64-bit virtual address into four
components: VPN[2] (bits 38–30), VPN[1] (bits 29–21), VPN[0]
(bits 20–12), and page offset (bits 11–0). Each VPN field MUST
be less than 512. The offset MUST be less than 4096. Each VPN
field MUST agree with page::radix_index(va, level, 9, 12).

#### Scenario: Zero address

- GIVEN va = 0
- WHEN vpn_extract_sv39 is called
- THEN VPN[2]=0, VPN[1]=0, VPN[0]=0, offset=0

#### Scenario: Address 0x80400000

- GIVEN va = 0x80400000
- WHEN vpn_extract_sv39 is called
- THEN VPN[2]=2, VPN[1]=2, VPN[0]=0, offset=0

### Requirement: Sv48 VPN Extraction
r[vpn.sv48-extract]

vpn_extract_sv48 MUST split a 64-bit virtual address into five
components: VPN[3] (bits 47–39), VPN[2] (bits 38–30), VPN[1]
(bits 29–21), VPN[0] (bits 20–12), and page offset (bits 11–0).
Each VPN field MUST be less than 512. The offset MUST be less
than 4096. Each VPN field MUST agree with
page::radix_index(va, level, 9, 12).

### Requirement: VPN Decomposition Roundtrip
r[vpn.roundtrip]

Concatenating all VPN fields (shifted to their original positions)
plus the page offset MUST recover the original virtual address
(masked to the valid address width: 39 bits for Sv39, 48 bits
for Sv48). This follows from page::radix_index_roundtrip.

#### Scenario: Sv39 roundtrip

- GIVEN any virtual address va (with canonical upper bits)
- WHEN vpn_extract_sv39 is applied and the fields are recombined
- THEN the result equals va masked to 39 bits

### Requirement: VPN Canonical Check
r[vpn.canonical]

is_canonical_sv39 MUST return true if and only if
bitwise::is_sign_extended(va, 39) is true.
is_canonical_sv48 MUST return true if and only if
bitwise::is_sign_extended(va, 48) is true.

#### Scenario: Canonical address (user space)

- GIVEN va = 0x0000_003F_FFFF_FFFF (bit 38 = 0, upper bits = 0)
- WHEN is_canonical_sv39 is called
- THEN the result is true

#### Scenario: Non-canonical address

- GIVEN va = 0x0000_0040_0000_0000 (bit 38 = 1, upper bits = 0)
- WHEN is_canonical_sv39 is called
- THEN the result is false

### Requirement: VPN Index Bounded
r[vpn.index-bounded]

Every VPN index returned by vpn_extract MUST be strictly less than
512. The page offset MUST be strictly less than 4096. These follow
from page::radix_index_bounded and bitfield::extract_field_bounded.

### PTE Permission Checking

### Requirement: Access Type Check
r[pte-permission.access-check]

check_pte_access MUST return true if and only if the access type
is permitted by the PTE's R/W/X bits. Load requires R=1 (or X=1
if MXR is set). Store requires W=1 (and R=1, per the W-implies-R
rule). Fetch requires X=1.

#### Scenario: Load from readable page

- GIVEN R=1, access_type=Load, MXR=false
- WHEN check_pte_access is called
- THEN the result is true

#### Scenario: Load from execute-only page

- GIVEN R=0, X=1, access_type=Load, MXR=false
- WHEN check_pte_access is called
- THEN the result is false

#### Scenario: Load from execute-only page with MXR

- GIVEN R=0, X=1, access_type=Load, MXR=true
- WHEN check_pte_access is called
- THEN the result is true

#### Scenario: Store to read-only page

- GIVEN R=1, W=0, access_type=Store
- WHEN check_pte_access is called
- THEN the result is false

### Requirement: Privilege Check
r[pte-permission.privilege-check]

check_pte_privilege MUST return true if and only if the access is
permitted given the PTE's U bit and the current privilege mode.
User mode requires U=1. Supervisor mode requires U=0, unless SUM
is set (in which case both U=0 and U=1 are permitted).

#### Scenario: User accessing U=1 page

- GIVEN U=1, mode=User
- WHEN check_pte_privilege is called
- THEN the result is true

#### Scenario: User accessing U=0 page

- GIVEN U=0, mode=User
- WHEN check_pte_privilege is called
- THEN the result is false

#### Scenario: Supervisor accessing U=1 page without SUM

- GIVEN U=1, mode=Supervisor, SUM=false
- WHEN check_pte_privilege is called
- THEN the result is false

#### Scenario: Supervisor accessing U=1 page with SUM

- GIVEN U=1, mode=Supervisor, SUM=true
- WHEN check_pte_privilege is called
- THEN the result is true

### Requirement: Permission Monotonicity
r[pte-permission.monotonic]

Adding a permission bit (setting R, W, or X from 0 to 1) in a PTE
MUST NOT cause a previously-allowed access to be denied. If
check_pte_access(pte, access_type) is true, then
check_pte_access(pte_with_more_bits, access_type) MUST also be true.

### Requirement: MXR Widening
r[pte-permission.mxr-widens]

For any PTE and access type, check_pte_access with MXR=true MUST
allow at least as many accesses as with MXR=false. MXR=true MUST
never deny an access that MXR=false allows.

### Requirement: SUM Widening
r[pte-permission.sum-widens]

For any PTE and privilege mode, check_pte_privilege with SUM=true
MUST allow at least as many accesses as with SUM=false. SUM=true
MUST never deny an access that SUM=false allows.

### Page Table Walk

### Requirement: Walk Result
r[page-walk.result]

page_walk MUST return either a successful translation (physical
address, page size, permissions) or a fault reason. The result
type MUST be an enum with exactly these two variants.

### Requirement: Walk Determinism
r[page-walk.deterministic]

page_walk with the same root table base, virtual address, access
type, and privilege mode MUST produce the same result when the
page table contents are unchanged. The walk is a pure function
of its inputs and the table contents.

### Requirement: Walk Delegates To Walk Level
r[page-walk.delegates-walk-level]

At each level, page_walk MUST compute the VPN index via
vpn_extract, compute the PTE address via walk_level::pte_addr,
read the PTE, and classify it via walk_level::interpret_pte.
For leaves, the PA MUST be computed via walk_level::leaf_pa.
For non-leaves, the next table base MUST be computed via
walk_level::next_table_base.

### Requirement: Walk Bounded Steps
r[page-walk.bounded-steps]

The walk MUST terminate in at most N steps, where N is the number
of page table levels (3 for Sv39, 4 for Sv48). Each step either
finds a leaf, encounters an invalid PTE, or descends to the next
level. The walk MUST NOT loop.

### Requirement: Walk PA In Region
r[page-walk.pa-in-region]

A successful walk MUST produce a physical address that satisfies
walk_level::leaf_pa_in_region. This delegates to
region::contains_addr(ppn_base, page_size, PA).

### Page Fault Classification

### Requirement: Fault Reasons
r[page-fault.reasons]

Page fault classification MUST distinguish exactly five fault
reasons: NOT_PRESENT (V=0), PERMISSION_VIOLATION (access type
not permitted by R/W/X), PRIVILEGE_VIOLATION (U-bit vs mode),
MISALIGNED_SUPERPAGE (nonzero low PPN bits on intermediate leaf),
and RESERVED_BITS_SET (bits 54–63 nonzero). The fault reason
type MUST have exactly these five variants.

#### Scenario: Invalid PTE

- GIVEN a PTE with V=0
- WHEN the fault is classified
- THEN the reason is NOT_PRESENT

#### Scenario: Write to read-only page

- GIVEN a leaf PTE with R=1, W=0 and a store access
- WHEN the fault is classified
- THEN the reason is PERMISSION_VIOLATION

#### Scenario: User access to supervisor page

- GIVEN a leaf PTE with U=0 and user-mode access
- WHEN the fault is classified
- THEN the reason is PRIVILEGE_VIOLATION

### Requirement: Fault Exhaustive
r[page-fault.exhaustive]

Every page walk that does not produce a valid physical address
MUST produce exactly one fault reason. The classification MUST
be total — no walk result is unclassified.

### Requirement: Fault Exclusive
r[page-fault.exclusive]

No page walk MUST produce two fault reasons simultaneously.
The five fault categories MUST be mutually exclusive when
evaluated in priority order: reserved bits → not present →
permission → privilege → misaligned superpage.

### Requirement: Valid Implies No Fault
r[page-fault.valid-implies-clear]

A successful walk (valid PA produced) MUST imply: V=1, R/W/X
permissions match the access type, U-bit matches the privilege
mode, superpage alignment is correct, and reserved bits are zero.
This is the contrapositive of fault classification.

### TLB Model

### Requirement: TLB Lookup
r[tlb.lookup]

tlb_lookup MUST delegate to tagged_lookup::tagged_lookup with
ASID as tag and VPN as key. Additionally, global entries (G=1)
MUST match any ASID — if tagged_lookup misses, the TLB MUST
also check entries with the global flag set.

#### Scenario: Cache hit

- GIVEN a TLB containing (asid=5, vpn=0x1000) → (ppn=0x2000)
- WHEN tlb_lookup(asid=5, vpn=0x1000) is called
- THEN the result is Some(ppn=0x2000)

#### Scenario: Cache miss

- GIVEN a TLB not containing (asid=5, vpn=0x3000)
- WHEN tlb_lookup(asid=5, vpn=0x3000) is called
- THEN the result is None

#### Scenario: Global entry matches any ASID

- GIVEN a TLB containing (global=true, vpn=0x1000) → (ppn=0x2000)
- WHEN tlb_lookup(asid=99, vpn=0x1000) is called
- THEN the result is Some(ppn=0x2000)

### Requirement: TLB Insert
r[tlb.insert]

tlb_insert MUST delegate to tagged_lookup::tagged_insert. After
insert, tlb_lookup for the same (ASID, VPN) MUST return the
inserted entry.

### Requirement: TLB Flush By ASID
r[tlb.flush-asid]

tlb_flush_asid MUST delegate to tagged_lookup::tagged_remove_by_tag.
Global entries (G=1) MUST NOT be removed by ASID flush — global
entries MUST be filtered out before removal or re-inserted after.

#### Scenario: Flush removes matching, preserves global

- GIVEN a TLB with entries for asid=5 (non-global) and a global entry
- WHEN tlb_flush_asid(5) is called
- THEN non-global asid=5 entries are removed, global entry remains

### Requirement: TLB Flush By VPN
r[tlb.flush-vpn]

tlb_flush_vpn MUST delegate to tagged_lookup::tagged_remove_by_key.
After flush, tlb_lookup for that VPN with any ASID MUST return None.

### Requirement: TLB Flush Full
r[tlb.flush-full]

tlb_flush_full MUST delegate to tagged_lookup::tagged_remove_all.
After full flush, tlb_lookup for any (ASID, VPN) MUST return None.

### Requirement: TLB Soundness
r[tlb.soundness]

If tlb_lookup(asid, vpn) returns Some(ppn, perms, page_size), then
page_walk with the same virtual address and page table contents MUST
produce the same (ppn, perms, page_size). A TLB hit MUST agree with
a fresh walk. This is the central correctness invariant.

### Requirement: TLB Capacity Bounded
r[tlb.capacity-bounded]

The TLB entry count MUST never exceed capacity. This follows from
tagged_lookup::capacity_bounded.

### ASID Management

### Requirement: ASID Allocate
r[asid.allocate]

asid_allocate MUST delegate to bounded_id_pool::pool_allocate and
stamp the returned ID with the current generation. The returned
ASID MUST be in [1, max_asid]. ASID 0 is reserved.

#### Scenario: Allocation succeeds

- GIVEN a pool with max_asid=255 and 100 allocated
- WHEN asid_allocate is called
- THEN a fresh ASID in [1, 255] is returned with the current generation

#### Scenario: Pool exhausted

- GIVEN a pool with max_asid=255 and 255 allocated
- WHEN asid_allocate is called
- THEN None is returned (generation rollover needed)

### Requirement: ASID Deallocate
r[asid.deallocate]

asid_deallocate MUST delegate to bounded_id_pool::pool_deallocate.

### Requirement: ASID Unique In Generation
r[asid.unique-in-generation]

No two active address spaces MUST hold the same ASID within the
same generation. This follows from bounded_id_pool::unique plus
the generation stamp.

### Requirement: ASID Generation Rollover
r[asid.generation-rollover]

asid_new_generation MUST increment the generation counter and
reset the pool (all IDs free). flush_needed MUST return true
after rollover.

#### Scenario: Rollover triggers flush

- GIVEN generation=5, all ASIDs allocated
- WHEN asid_new_generation is called
- THEN generation=6, all ASIDs free, flush_needed is true

### Requirement: ASID Pool Bounded
r[asid.pool-bounded]

Allocated ASID count MUST never exceed max_asid. ASID 0 MUST
never be allocated. This follows from bounded_id_pool::bounded.

### Requirement: ASID Context Switch
r[asid.context-switch]

asid_switch MUST return whether a TLB flush is needed. A flush
is needed if the target ASID's generation differs from the current
generation (stale ASID from a prior generation). This uses the
same pattern as slab::generation_valid.

#### Scenario: Same generation, no flush

- GIVEN current_gen=5, target allocated in gen=5
- WHEN asid_switch is called
- THEN flush_needed is false

#### Scenario: Stale generation, flush needed

- GIVEN current_gen=5, target last seen in gen=3
- WHEN asid_switch is called
- THEN flush_needed is true

## Cross-Module Properties

### Requirement: PTE Uses Bitfield
r[properties.pte-uses-bitfield]

parse_pte field extraction MUST agree with bitfield::extract_field
at the specified offsets and widths for all fields (V at offset 0
width 1, R at offset 1 width 1, ..., PPN[2] at offset 28 width 26).

### Requirement: VPN Uses Radix Index
r[properties.vpn-uses-radix-index]

vpn_extract_sv39 VPN[i] MUST equal page::radix_index(va, i, 9, 12).
vpn_extract_sv48 VPN[i] MUST equal page::radix_index(va, i, 9, 12).

### Requirement: VPN Canonical Uses Sign Extended
r[properties.vpn-canonical-uses-sign-extended]

is_canonical_sv39(va) MUST equal bitwise::is_sign_extended(va, 39).
is_canonical_sv48(va) MUST equal bitwise::is_sign_extended(va, 48).

### Requirement: VPN Offset Agrees With Page
r[properties.vpn-agrees-page]

vpn_extract offset field MUST agree with page::page_offset(va, 4096).

### Requirement: Walk Level PA In Region
r[properties.walk-level-pa-in-region]

walk_level::leaf_pa result MUST satisfy region::contains_addr at the
PPN-derived base and level-derived page size.

### Requirement: Walk Delegates Walk Level
r[properties.walk-delegates-walk-level]

page_walk at each level MUST use walk_level::pte_addr,
walk_level::interpret_pte, walk_level::leaf_pa, and
walk_level::next_table_base.

### Requirement: Permission Relates To Access Mode
r[properties.permission-access-mode]

check_pte_access with R=1,W=1,X=1 MUST allow all access types,
matching access_mode::mode_permits(FULL, any_mode).

### Requirement: TLB Uses Tagged Lookup
r[properties.tlb-uses-tagged-lookup]

TLB operations MUST delegate to tagged_lookup operations with
ASID as tag and VPN as key.

### Requirement: TLB Agrees With Walk
r[properties.tlb-agrees-walk]

tlb_lookup hit MUST imply page_walk with unchanged tables produces
the same result.

### Requirement: ASID Uses Bounded ID Pool
r[properties.asid-uses-bounded-id-pool]

ASID allocation/deallocation MUST delegate to bounded_id_pool
operations.

### Requirement: ASID Uses Slab Generations
r[properties.asid-uses-slab-generations]

ASID generation validity MUST follow the same pattern as
slab::generation_valid.

### Requirement: Fault Partitions Walk Outcomes
r[properties.fault-partitions-walk]

The union of "successful walk" and the five fault reasons MUST
cover all possible walk outcomes. No outcome MUST be unclassified
or belong to two categories.
