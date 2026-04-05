## Why

False sharing is a performance pathology where two threads access different
variables that happen to reside on the same cache line. Each write
invalidates the other core's copy, causing constant cache-line bouncing
despite no actual data dependency. It's invisible at the source level and
devastating to throughput — often 10-100x slowdowns on tight loops.

Detecting false sharing requires reasoning about address-to-cache-line
mapping: two addresses share a line when they agree on all bits above the
offset. The detection predicate is simple, but proving it correct relative
to the set-associative lookup (same line ↔ same tag + same index) and
proving that distinct lines never alias requires connecting the address
decomposition to the sharing predicate.

## What Changes

- **false_sharing**: A new module modeling false sharing detection and
  cache line boundary reasoning.

  Proves:
  - **Same-line predicate**: Two addresses share a cache line iff they
    agree on all bits above the offset. Equivalent to: their tag and
    index are identical.
  - **Distinct-line guarantee**: If two addresses differ in tag or index,
    they are on different lines and cannot false-share.
  - **Alignment boundary**: An address is cache-line-aligned iff its
    offset bits are all zero. Aligned allocations start at line boundaries.
  - **Padding sufficiency**: If two objects are separated by at least one
    cache line size, they cannot share a line. Proves the standard
    padding mitigation is correct.
  - **Object span**: An object of size S starting at address A occupies
    `ceil((A % line_size + S) / line_size)` cache lines. Proves the
    line-count formula.
  - **Composability with set-associative lookup**: The same-line predicate
    is equivalent to `decompose(a).tag == decompose(b).tag && decompose(a).index == decompose(b).index`.

## Capabilities

### New Capabilities

- `false-sharing.same-line`: Verified same-cache-line predicate
- `false-sharing.distinct-line`: Verified non-sharing guarantee
- `false-sharing.alignment`: Verified cache-line alignment predicate
- `false-sharing.padding`: Verified padding sufficiency
- `false-sharing.object-span`: Verified cache-line count for an object
- `false-sharing.decomposition-equivalence`: Verified link to set-associative decomposition

## Impact

- **Files**: One new `src/false_sharing.rs`, module declaration in `lib.rs`
- **APIs**: `same_cache_line`, `is_line_aligned`, `padding_prevents_sharing`,
  `cache_lines_spanned`, parameterized by line_size (power of 2)
- **Dependencies**: Depends on `vstd`, references `set_associative_lookup`
  for decomposition equivalence proof
- **Testing**: Verus proofs for all properties, proptest for same-line
  predicate against brute-force mask comparison
