# False Sharing Detection Specification

## Purpose

Defines requirements for verified false sharing detection — determining
when two addresses occupy the same cache line, proving mitigation via
padding and alignment, and computing the cache-line footprint of objects.

## Requirements

### Requirement: Same-Line Predicate

Two addresses MUST be identified as sharing a cache line iff they agree
on all bits above the offset.

#### Scenario: Same line

- GIVEN addresses 0x1040 and 0x103F with a 64-byte line size
- WHEN the same-line predicate is evaluated
- THEN the result is true (both in line starting at 0x1000)

#### Scenario: Different lines

- GIVEN addresses 0x103F and 0x1040 with a 64-byte line size
- WHEN the same-line predicate is evaluated
- THEN the result is false (0x103F in line 0x1000, 0x1040 in line 0x1040)

#### Scenario: Reflexive

- GIVEN any address A
- WHEN `same_cache_line(A, A, line_size)` is evaluated
- THEN the result is true

### Requirement: Distinct-Line Guarantee

If two addresses are on different cache lines, they MUST NOT false-share.

#### Scenario: Different tag or index

- GIVEN addresses A and B where `same_cache_line(A, B, line_size)` is false
- WHEN both are accessed concurrently
- THEN no false sharing occurs (they are on independent lines)

### Requirement: Alignment Predicate

An address MUST be identified as cache-line-aligned iff its offset bits
are all zero.

#### Scenario: Aligned address

- GIVEN address 0x1000 and a 64-byte line size
- WHEN the alignment predicate is evaluated
- THEN the result is true (0x1000 % 64 == 0)

#### Scenario: Unaligned address

- GIVEN address 0x1004 and a 64-byte line size
- WHEN the alignment predicate is evaluated
- THEN the result is false (0x1004 % 64 == 4)

### Requirement: Padding Sufficiency

If two objects are separated by at least one cache line size, they MUST
NOT share a cache line.

#### Scenario: Sufficient padding

- GIVEN object A at address 100 and object B at address 164 with a 64-byte line size
- WHEN the padding check is evaluated
- THEN A and B cannot share a line (separation ≥ 64)

#### Scenario: Insufficient padding

- GIVEN object A at address 100 and object B at address 120 with a 64-byte line size
- WHEN the padding check is evaluated
- THEN A and B MAY share a line (separation < 64)

### Requirement: Object Span

The number of cache lines an object spans MUST equal
`ceil((start_offset + size) / line_size)` where start_offset is the
object's offset within its first cache line.

#### Scenario: Aligned single-line object

- GIVEN a 32-byte object at a 64-byte-aligned address with 64-byte lines
- WHEN the span is computed
- THEN the object spans 1 cache line

#### Scenario: Unaligned cross-line object

- GIVEN a 60-byte object starting at offset 8 within a 64-byte line
- WHEN the span is computed
- THEN the object spans 2 cache lines (bytes 8-63 in line 1, bytes 0-3 in line 2)

### Requirement: Decomposition Equivalence

The same-line predicate MUST be equivalent to tag+index equality from
set-associative decomposition.

#### Scenario: Equivalence

- GIVEN addresses A and B and cache parameters (offset_bits, index_bits)
- WHEN `same_cache_line(A, B, 1 << offset_bits)` is compared to
  `tag(A) == tag(B) && index(A) == index(B)`
- THEN the results are identical
