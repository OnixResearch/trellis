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
defined as `a / line_size == b / line_size` (integer division) in the spec,
with a bit-mask implementation `(a & !mask) == (b & !mask)` where
`mask = line_size - 1` in the exec function. Requires `line_size` is a
power of two.

**Rationale:** Integer division is the clearest spec. Bit masking is the
efficient implementation. The equivalence proof (division equals masking
when divisor is a power of two) is a standard `by(bit_vector)` result.

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

**Choice:** `padding_prevents_sharing(addr_a: u64, size_a: u64, addr_b: u64, line_size: u64) -> bool` where `addr_b >= addr_a + size_a` and
`addr_b - addr_a >= line_size`. Prove this implies `!same_cache_line(addr_a, addr_b, line_size)`.

**Rationale:** The standard mitigation is to pad structs to cache-line
boundaries. If two objects are at least one line size apart, they can't
share a line (assuming each fits within one line).

### 5. Object Span Formula

**Choice:** `cache_lines_spanned(addr: u64, size: u64, line_size: u64) -> u64`
computes `((addr % line_size) + size + line_size - 1) / line_size`. The
inner-offset of the start address plus the object size, rounded up.

**Rationale:** An object starting at a non-aligned address may span one
more line than its size suggests. A 60-byte object starting at byte 4 of
a 64-byte line spans 2 lines, not 1.

## Risks / Trade-offs

**[Only models same-line, not same-set]** → Two addresses can be in the
same set but different lines (same index, different tag). That causes
conflict misses, not false sharing. The module only models same-line.

**[Single-object padding]** → The padding proof assumes each object fits
within one cache line. Multi-line objects need the span formula instead.
