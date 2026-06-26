## Context

The library has the building blocks for MMU primitives but hasn't composed
them. `bitfield.rs` does multi-bit extraction/insertion. `page.rs` does
frame number/offset/alignment decomposition. `region.rs` does address range
arithmetic with overflow safety. `slab.rs` has generation-based handle
validation. `access_mode.rs` has R/W/X ordering.

A RISC-V Sv39 page table walk is structurally simple: read 3 PTEs from
memory (one per level), extract VPN indices to find each PTE, check
validity and permissions, and compute the physical address from the leaf
PTE's PPN field. The hard part isn't the code — it's proving the
properties: that the walk is deterministic, that the resulting PA falls
within the correct physical region, that permission checking is monotonic,
and that a TLB cache agrees with a fresh walk.

Several of the operations inside the MMU decompose into smaller reusable
primitives. The page walk splits into per-level interpretation and
multi-level iteration. The TLB is a bounded tagged cache — a pattern
that transcends MMUs. ASID allocation is a bounded ID pool. Canonical
address checking is sign-extension checking. VPN extraction is radix
index extraction. Extracting these primitives keeps the MMU modules thin
and gives the library reusable pieces for other domains.

## Goals / Non-Goals

**Goals:**
- Extract reusable primitives: sign-extension check, radix index,
  walk-level interpretation, bounded tagged cache, bounded ID pool
- PTE parse/construct using bitfield, with roundtrip proofs
- Sv39 and Sv48 virtual address decomposition via radix_index
- Page walk as thin iteration over walk_level
- Fault classification that's provably exhaustive and exclusive
- Permission checking with monotonicity proofs for MXR/SUM modifiers
- TLB as thin wrapper over tagged_lookup with global-entry semantics
- ASID pool as thin wrapper over bounded_id_pool with generation logic
- Cross-module properties linking all modules
- All modules follow existing spec fn + exec fn + proof fn + tests pattern
- Every module stays under ~500 lines / ~12 pub fns (library median)

**Non-Goals:**
- Actual page table data structures in memory (walk takes a lookup
  function, not a raw memory array)
- TLB replacement policies (we model insert/evict, not LRU/PLRU)
- Hardware-managed TLB refill (we model software-visible behavior)
- x86-64 or AArch64 page table formats (Sv39/Sv48 only, but the
  walk structure and reusable primitives generalize)
- Interrupt/trap handling on page fault (we classify faults, not
  handle them)
- Virtual memory mapping management (which pages are mapped where)
- Performance modeling (TLB hit rates, walk latency)

## Decisions

### 1. Decompose into reusable primitives + thin MMU wrappers

**Choice:** Extract five reusable primitives from the MMU design:

1. `is_sign_extended` → add to `bitwise.rs`
2. `radix_index` → add to `page.rs`
3. `walk_level.rs` → new module for single-level PTE interpretation
4. `tagged_lookup.rs` → new module for bounded (tag, key) → value cache
5. `bounded_id_pool.rs` → new module for bounded unique ID allocation

The MMU-specific modules (`pte`, `vpn`, `pte_permission`, `page_walk`,
`page_fault`, `tlb`, `asid`) become thin compositions of these
primitives with domain-specific logic.

**Rationale:** The library's median module is ~400 lines, 8–12 pub fns.
Without decomposition, `page_walk` would be ~1000+ lines (step logic +
PA computation + superpage checks + multi-level iteration + determinism
proof), `tlb` would be ~800 lines (lookup + insert + 4 flushes + global
semantics + soundness proof), and `asid` bundles a general pool allocator
with MMU-specific generation logic. Splitting keeps every module under
500 lines and produces primitives that are useful outside MMU:

- `tagged_lookup` → DNS caches, routing tables, session stores
- `bounded_id_pool` → process IDs, file descriptors, interrupt vectors
- `walk_level` → page table construction, mapping validation, hypervisor
  stage-2 walks
- `radix_index` → IP prefix tries, inode trees, HAMT lookups
- `is_sign_extended` → RISC-V immediate decoding, x86-64 canonical checks

**Alternative:** Keep the original 7-module design. Rejected — three
modules exceed the library's size norms and bury reusable logic inside
MMU-specific code.

### 2. RISC-V Sv39 as primary target, Sv48 as extension

**Choice:** Implement Sv39 (3-level, 39-bit VA) first, then add Sv48
(4-level, 48-bit VA) as a thin extension that adds one more VPN level.

**Rationale:** Sv39 is the most widely deployed RISC-V paging mode.
Sv48 is structurally identical except for one extra level. The proofs
for Sv48 are mechanical extensions of the Sv39 proofs — same walk loop,
one more iteration. Starting with Sv39 gets all the interesting proofs
done without the extra level cluttering the initial development.

**Alternative:** Architecture-generic walk parameterized by level count.
Rejected for the first pass — parameterizing over levels adds a type
parameter that complicates the Verus specs without adding proof power.
The concrete Sv39/Sv48 modules can be refactored later.

### 3. PTE as a parsed struct, not raw u64 everywhere

**Choice:** Define a `PteFields` struct with named fields (v, r, w, x,
u, g, a, d, rsw, ppn0, ppn1, ppn2). Parse once, pass the struct. The
raw u64 form is used only at parse/construct boundaries.

**Rationale:** Proofs about PTE properties (is_leaf, permission checks)
are simpler when they operate on named boolean/integer fields than on
shift-and-mask expressions. The parse/construct roundtrip proof
establishes that the struct faithfully represents the u64, after which
all downstream proofs work on the struct.

**Alternative:** Keep everything as u64 and inline the bitfield
extraction. Rejected — this makes every proof repeat the shift/mask
arithmetic. The struct factors it out.

### 4. Page walk takes a lookup function, not a memory array

**Choice:** `page_walk` takes a `table_read: spec(fn(u64) -> u64)`
parameter that maps PTE addresses to PTE values. The walk calls
`table_read(pte_addr)` at each level.

**Rationale:** The walk logic doesn't care where PTEs are stored. By
abstracting over the storage, we avoid modeling memory as a `Seq<u64>`
or `Map<u64, u64>` in the walk itself. This keeps the walk proof focused
on the address arithmetic and PTE interpretation. Callers who store page
tables in a flat array, a tree, or a hardware MMU can all instantiate
the lookup function.

For the exec version, the lookup is `Fn(u64) -> u64` (a closure or
function pointer). The Verus spec version uses `spec(fn)`.

**Alternative:** Model page tables as `Seq<Seq<u64>>` (one seq per
level). Rejected — this couples the walk to a specific storage layout
and makes the determinism proof depend on sequence indexing rather than
the pure walk logic.

### 5. Walk-level as independent module, walk as thin iteration

**Choice:** Extract the per-level logic into `walk_level.rs` (pte_addr,
interpret_pte, leaf_pa, next_table_base, is_superpage_aligned,
superpage_size). The multi-level walk in `page_walk.rs` becomes a thin
loop that calls walk_level at each level.

**Rationale:** The per-level functions are independently useful:
- `pte_addr` for constructing page table entries during map/unmap
- `interpret_pte` for validating existing mappings
- `leaf_pa` for computing physical addresses outside of a full walk
- `is_superpage_aligned` for checking alignment during map operations

The walk proof simplifies: prove each walk_level function correct once,
then the walk proof just shows the loop iterates correctly. Without this
split, the walk proof interleaves step-level reasoning with iteration
reasoning.

**Alternative:** Inline everything into page_walk.rs. Rejected —
produces a ~1000-line module with 20+ pub fns, well above the library
median. The step-level proofs (leaf_pa in region, superpage alignment)
would be tangled with the iteration proofs (determinism, bounded steps).

### 6. Fault classification as ordered priority checks

**Choice:** Classify faults by checking in order: reserved bits →
not present → invalid permission encoding (W=1,R=0) → permission
violation → privilege violation → misaligned superpage. The first
failing check determines the fault reason.

**Rationale:** The checks have a natural priority. Reserved bits and
validity are checked before permissions because there's no point
checking permissions on an invalid PTE. This ordering makes the
exclusivity proof trivial: if check N fails, checks 1..N-1 all passed,
so the fault reason is unambiguous.

**Alternative:** Return a set of all failing checks. Rejected — this
complicates the "exactly one fault reason" invariant and doesn't match
how page faults are actually delivered (one trap cause per fault).

### 7. TLB delegates to tagged_lookup, adds global-entry logic

**Choice:** The TLB wraps `tagged_lookup.rs` with ASID as tag and VPN
as key. Global entries (G=1) require special handling: they match any
ASID on lookup and survive ASID-specific flushes. This is the only
TLB-specific logic beyond what tagged_lookup provides.

**Rationale:** The bounded cache operations (insert, lookup, remove-by-tag,
remove-by-key, clear, capacity tracking) are generic. Only global-entry
matching and ASID-flush preservation are TLB-specific. Extracting the
generic part gives us a reusable `tagged_lookup` module, and the TLB
wrapper stays under 350 lines.

**Alternative:** Build the TLB from scratch without the generic cache.
Rejected — duplicates bounded-cache logic that has value elsewhere.

### 8. ASID delegates to bounded_id_pool, adds generation logic

**Choice:** ASID allocation wraps `bounded_id_pool.rs` and adds
generation counters. Allocation stamps IDs with the current generation.
Validity is `id.generation == pool.current_generation` (same as
slab::generation_valid). Generation rollover frees the pool and sets
flush_needed.

**Rationale:** The bounded unique ID allocation (allocate from [1, max],
deallocate, uniqueness, boundedness) is generic — process IDs, file
descriptors, and hardware resource IDs all need this. The ASID-specific
part is the generation rollover and TLB flush coordination. Separating
them gives a reusable pool and a thin ASID wrapper.

**Alternative:** Build ASID allocation directly on alloc_bitmap.
Considered — alloc_bitmap already exists in the library. But
alloc_bitmap uses slot-indexed bitmaps with generation handles, which is
heavier than needed for a simple ID pool. bounded_id_pool is simpler
(just a free set) and matches the ASID use case more directly.

### 9. Phased implementation: primitives first, then MMU modules

**Choice:** Build in dependency order, reusable primitives before
MMU-specific modules:

**Phase 1 — sign_extend, radix_index** (add to existing modules)
**Phase 2 — PTE** (depends on bitfield)
**Phase 3 — VPN** (depends on radix_index, sign_extend)
**Phase 4 — PTE Permission** (depends on PTE)
**Phase 5 — Dirty/Accessed** (depends on PTE)
**Phase 6 — walk_level** (depends on PTE, region)
**Phase 7 — bounded_id_pool** (standalone)
**Phase 8 — tagged_lookup** (standalone)
**Phase 9 — page_walk** (depends on walk_level, VPN)
**Phase 10 — page_fault** (depends on PTE, walk_level)
**Phase 11 — TLB** (depends on tagged_lookup, page_walk)
**Phase 12 — ASID** (depends on bounded_id_pool, slab)
**Phase 13 — Cross-module properties**

Phases 1, 7, and 8 are independent of each other and can proceed in
parallel. Phase 2 depends only on existing bitfield. Phases 3–5
depend on prior phases sequentially. Phase 6 depends on 2 but not
3–5. The three standalone primitives (phases 1, 7, 8) can all be done
before any MMU module.

## Risks / Trade-offs

**[Spec-level table lookup in page walk]** The walk spec uses
`spec(fn(u64) -> u64)` for table reads. Verus handles spec closures
but triggering on closure application can be finicky. Mitigation: if
spec closures cause issues, fall back to `Map<u64, u64>` for the spec
and `Fn(u64) -> u64` for the exec, with an agreement proof between them.

**[Superpage alignment proof]** Proving that PPN[0] = 0 for a level-1
leaf requires connecting the bitfield extraction of PPN[0] (bits 10–18)
to the superpage alignment condition (PA aligned to 2 MiB = 2^21).
This needs `by(bit_vector)` reasoning that the extracted field being
zero implies the corresponding bits in the PA are zero. The bitfield
module's extract_field proofs should handle this, but it may need an
explicit lemma bridging PPN subfield extraction to address alignment.

**[TLB soundness proof complexity]** The central theorem — TLB hit
agrees with walk — requires showing that the stored (PPN, permissions,
page_size) in the TLB entry are exactly what the walk would produce.
This is straightforward if inserts only come from walk results, but the
proof needs to handle the case where page table contents change between
the walk that populated the TLB and the lookup. Mitigation: state the
soundness theorem with an explicit "table contents unchanged" precondition.

**[VPN decomposition roundtrip with sign extension]** Sv39 addresses
have bits 63–39 as sign extensions of bit 38. The roundtrip proof
(extract VPNs + offset, recombine, get original VA) needs to handle
the sign-extended upper bits. Mitigation: state the roundtrip for the
39-bit (or 48-bit) value, not the full 64-bit address. The canonical
check is a separate predicate.

**[Z3 bitvector performance for PTE proofs]** PTE parsing involves
many shift-and-mask operations on a 64-bit word. Individual field
proofs should be fast, but the full roundtrip (all 10+ fields) may
hit Z3's bitvector timeout. Mitigation: prove each field's roundtrip
individually, then compose.

**[tagged_lookup generality vs simplicity]** Making the cache generic
means it can't use domain-specific optimizations (e.g., ASID-aware
hashing). Mitigation: the TLB wrapper can add domain logic on top. The
generic cache just needs to be correct, not fast — this is a
verification library, not a runtime implementation.
