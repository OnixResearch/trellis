## Context

False sharing is detected by checking whether two addresses map to the
same cache line. This is a bitwise predicate: mask off the offset bits
and compare. The module connects this predicate to the set-associative
decomposition and proves mitigation strategies (padding, alignment).

## Goals / Non-Goals

**Goals:**
- Same-line predicate as bit masking
- Equivalence with tag+index equality from decomposition
- Alignment predicate and its relationship to line boundaries
- Padding sufficiency proof
- Object cache-line span formula

**Non-Goals:**
- Runtime false sharing detection (perf counters, sampling)
- Automatic padding insertion or struct layout optimization
- NUMA-aware sharing analysis
- Performance impact estimation

## Decisions

### 1. Same-Line Predicate

**Choice:** `same_cache_line(a: u64, b: u64, line_size: u64) -> bool`
uses the same quotient-equality definition in both spec and exec:
`a / line_size == b / line_size`.

**Rationale:** Integer division is the clearest contract and verified much
more cleanly than a mask-based exec surface in this repo. The bit-level
view still appears in the decomposition-equivalence proof, which is where
it matters semantically for cache-line identity.

### 2. Line Size Constraint

**Choice:** `line_size` must be a power of two greater than zero. Expressed
as `exists k: u32 :: line_size == 1 << k` or equivalently
`line_size > 0 && (line_size & (line_size - 1)) == 0`.

**Rationale:** All real hardware uses power-of-two cache lines (32, 64,
128 bytes). The power-of-two constraint enables bit masking.

### 3. Decomposition Equivalence

**Choice:** Prove `same_cache_line(a, b, line_size) <==> (extract_tag(a, offset_bits, index_bits) == extract_tag(b, offset_bits, index_bits) && extract_index(a, offset_bits, index_bits) == extract_index(b, offset_bits, index_bits))` where `line_size == 1 << offset_bits`.

**Rationale:** This links the false-sharing module to the set-associative
lookup module. The same-line predicate is just "same tag and same index."
The offset is the only part that can differ within a line.

### 4. Padding Proof

**Choice:** `padding_prevents_sharing(addr_a: u64, size_a: u64, addr_b: u64, line_size: u64) -> bool` states that object B begins at least one full cache line after the end of object A, i.e.
`addr_b >= addr_a + size_a + line_size`. Prove this implies that no byte
of object A can share a cache line with any byte of object B.

**Rationale:** The mitigation needs an explicit end-to-start gap, not just
start-address distance. That formulation stays correct even when object A
is unaligned or already spans multiple lines.

### 5. Object Span Formula

**Choice:** `cache_lines_spanned(addr: u64, size: u64, line_size: u64) -> u64`
computes `div_ceil((addr % line_size) + size, line_size)` for positive-size
objects, with a precondition that the intermediate sum fits in `u64`.
The inner-offset of the start address plus the object size, rounded up.

**Rationale:** An object starting at a non-aligned address may span one
more line than its size suggests. A 60-byte object starting at byte 4 of
a 64-byte line spans 2 lines, not 1. Using `div_ceil` matches the repo's
existing verified ceiling-division surface and avoids a bespoke overflow
story.

### 6. Alignment Helper

**Choice:** `is_line_aligned(addr, line_size)` uses `addr % line_size == 0`
in the spec and exec code, while `line_start(addr, line_size)` returns the
floor multiple `addr - (addr % line_size)` / `(addr / line_size) * line_size`.

**Rationale:** Alignment and line-start are the natural helper views for the
same-line proof family. The floor-multiple helper preserves the exact line
boundary value without introducing extra mask-specific proof obligations.

## Risks / Trade-offs

**[Only models same-line, not same-set]** → Two addresses can be in the
same set but different lines (same index, different tag). That causes
conflict misses, not false sharing. The module only models same-line.

**[One-sided gap API]** → `padding_prevents_sharing` is phrased for the
ordered case where object B starts after object A. Callers comparing
arbitrary object pairs should normalize the order first.
