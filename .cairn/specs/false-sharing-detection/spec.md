# False Sharing Detection Specification

## Purpose

Defines requirements for verified false sharing detection — determining
when two addresses occupy the same cache line, proving mitigation via
padding and alignment, and computing the cache-line footprint of objects.

## Shared Parameters and Preconditions

Unless a requirement says otherwise:

- `line_size > 0`
- `line_size` is a power of two
- addresses and object sizes are byte counts
- object-span and padding requirements talk about positive-size objects
- for object-span, `(addr % line_size) + size` fits in `u64`

## Requirements

### Requirement: Same-Line Predicate
r[false-sharing.same-line]


Two addresses MUST be identified as sharing a cache line iff they agree
on all bits above the offset.

#### Scenario: Same line

- GIVEN addresses 0x1000 and 0x103F with a 64-byte line size
- WHEN the same-line predicate is evaluated
- THEN the result is true (both in the line starting at 0x1000)

#### Scenario: Different lines

- GIVEN addresses 0x103F and 0x1040 with a 64-byte line size
- WHEN the same-line predicate is evaluated
- THEN the result is false (0x103F in line 0x1000, 0x1040 in line 0x1040)

#### Scenario: Reflexive

- GIVEN any address A
- WHEN `same_cache_line(A, A, line_size)` is evaluated
- THEN the result is true

### Requirement: Distinct-Line Guarantee
r[false-sharing.distinct-line]


If two addresses are on different cache lines, they MUST NOT false-share.

#### Scenario: Different lines stay independent

- GIVEN addresses 0x103F and 0x1040 with a 64-byte line size
- WHEN both are accessed concurrently
- THEN no false sharing occurs because one touches line 0x1000 and the other touches line 0x1040

### Requirement: Alignment Predicate
r[false-sharing.alignment]


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
r[false-sharing.padding]


If object A occupies bytes `[addr_a, addr_a + size_a)` and object B
occupies bytes `[addr_b, addr_b + size_b)`, and object B starts at least
one full cache line after the end of object A
(`addr_b >= addr_a + size_a + line_size`), then the two objects MUST NOT
share a cache line.

#### Scenario: Sufficient gap after object A

- GIVEN object A at address 100 with size 40, object B at address 204 with size 16, and a 64-byte line size
- WHEN the padding check is evaluated
- THEN A and B cannot share a line (A ends at 140 and B starts 64 bytes later)

#### Scenario: Insufficient gap after object A

- GIVEN object A at address 100 with size 40, object B at address 150 with size 16, and a 64-byte line size
- WHEN the padding check is evaluated
- THEN A and B MAY share a line (both touch the line starting at 0x80)

### Requirement: Object Span
r[false-sharing.object-span]


The number of cache lines a positive-size object spans MUST equal
`ceil((start_offset + size) / line_size)` where `start_offset` is the
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
r[false-sharing.decomposition-equivalence]


For the canonical decomposition in `set_associative_lookup` —
`extract_tag_spec` and `extract_index_spec` with
`line_size == 1 << offset_bits` — the same-line predicate MUST be
equivalent to tag+index equality.

#### Scenario: Equivalence

- GIVEN addresses A and B and cache parameters `(offset_bits, index_bits)` with `offset_bits < 64` and `offset_bits + index_bits <= 64`
- WHEN `same_cache_line(A, B, 1 << offset_bits)` is compared to
  `set_associative_lookup::extract_tag_spec(A, offset_bits, index_bits) == set_associative_lookup::extract_tag_spec(B, offset_bits, index_bits)` and
  `set_associative_lookup::extract_index_spec(A, offset_bits, index_bits) == set_associative_lookup::extract_index_spec(B, offset_bits, index_bits)`
- THEN the results are identical
