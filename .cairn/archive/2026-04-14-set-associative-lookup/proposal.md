## Why

The existing `block_cache.rs` models a cache at the slot level — occupancy,
dirty tracking, clock-hand sweep — but abstracts over how an address maps
to a cache location. The fundamental operation missing is set-associative
lookup: decompose an address into tag/index/offset, use the index to select
a set, compare the tag against all ways in that set, and return hit/miss.

Every cache operation (read, write, evict, prefetch, coherence action)
starts with this lookup. If the tag decomposition aliases two distinct
addresses to the same tag+index, data corruption follows. If the way
comparison misses a valid entry, performance degrades and correctness
breaks for coherence protocols that assume cache state is accurate.

## What Changes

- **set_associative_lookup**: A new module modeling the address
  decomposition and N-way set-associative lookup.

  Proves:
  - **Decomposition bijection**: `(tag, index, offset)` extracted from an
    address can reconstruct the original address. No information is lost.
  - **No aliasing**: Two distinct addresses with the same index but
    different tags never produce a false hit. Tag comparison is exact.
  - **Hit correctness**: A hit in way W means the tag stored in way W
    matches the requested tag. The matched way identifies the correct
    cache line.
  - **Miss completeness**: A miss means no way in the selected set holds
    a matching tag. Proved by exhaustive comparison across all ways.
  - **Set isolation**: Addresses with different indices map to different
    sets. A lookup in set S never inspects or modifies set S'.
  - **Way selection determinism**: For a given tag and set, at most one
    way can match. No duplicate tags within a set.

## Capabilities

### New Capabilities

- `set-assoc.decomposition`: Verified address→(tag, index, offset) bijection
- `set-assoc.no-aliasing`: Verified tag uniqueness prevents false hits
- `set-assoc.hit`: Verified hit returns the correct matching way/tag
- `set-assoc.miss`: Verified miss means exhaustive way comparison failed
- `set-assoc.set-isolation`: Verified cross-set non-interference
- `set-assoc.no-duplicates`: Verified at most one matching way per set

## Impact

- **Files**: One new `src/set_associative_lookup.rs`, module declaration
  in `lib.rs`
- **APIs**: `decompose_address`, `reconstruct_address`, `lookup_set`,
  `find_way`, `check_no_duplicate_tags`, parameterized by offset_bits,
  index_bits, num_ways
- **Dependencies**: Depends on `vstd`, composable with `block_cache.rs`
  and `eviction.rs`
- **Testing**: Verus proofs for all properties, proptest for decomposition
  roundtrip and exhaustive small-cache aliasing checks
