## Context

The library uses `by(bit_vector)` proofs for all bitwise operations.
Z3's bitvector theory handles shift-and-mask arithmetic well for concrete
widths. The existing bitwise.rs module proves single-bit operations
using this approach.

## Goals / Non-Goals

**Goals:** verified multi-bit field extraction/insertion with round-trip
and non-interference proofs.

**Non-Goals:** variable-width integer packing (like protobuf varints),
struct-to-bitfield derive macros, fields wider than 64 bits.

## Decisions

### 1. Shift-and-mask implementation

**Choice:** Use `(value >> offset) & mask` for extract and
`(value & !mask) | ((field & width_mask) << offset)` for insert.

**Rationale:** Direct bitwise ops map 1:1 to Z3's bitvector theory.
No loops, no recursion — each function is a single expression.

**Implementation:**
- `field_mask(offset, width)` = `((1u64 << width) - 1) << offset`
- `extract_field(value, offset, width)` = `(value >> offset) & ((1u64 << width) - 1)`
- `insert_field(value, field, offset, width)` = `(value & !field_mask(offset, width)) | ((field & ((1u64 << width) - 1)) << offset)`
- `clear_field(value, offset, width)` = `value & !field_mask(offset, width)`

### 2. Precondition: offset + width <= 64

**Choice:** Require `offset + width <= 64` on all functions.

**Rationale:** Shifts by >= 64 are undefined behavior in hardware and
produce 0 in Rust. The precondition keeps the math clean and avoids
a class of edge cases that don't correspond to real use.

### 3. by(bit_vector) for all proofs

**Choice:** Use `by(bit_vector)` for round-trip, non-interference,
clear-then-extract, and boundedness proofs.

**Rationale:** Z3's bitvector theory decides these automatically.
No manual induction or nonlinear_arith needed.

## Risks / Trade-offs

**[bit_vector solver limits]** → The proofs may need `offset + width <= 64`
split into cases if Z3 times out on the full 64-bit domain. Mitigation:
test with concrete values first, widen.
