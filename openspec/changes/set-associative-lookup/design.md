## Context

Every cache operation starts with an address lookup. The address is
decomposed into tag/index/offset, the index selects a set, and the tag
is compared against all ways in the set. This module verifies the
decomposition and lookup, parameterized by the number of offset bits,
index bits, and ways.

## Goals / Non-Goals

**Goals:**
- Parameterized decomposition: offset_bits, index_bits derive tag_bits
- Bijection proof: decompose then reconstruct is identity
- Tag comparison across all ways in a set
- Hit/miss classification with proofs
- Set isolation and no-duplicate-tags invariant

**Non-Goals:**
- Cache data storage (that's `block_cache.rs`)
- Eviction policy (that's `eviction.rs`)
- Coherence state (that's `mesi.rs`)
- Skewed or hash-based indexing (standard modular indexing only)

## Decisions

### 1. Address Decomposition

**Choice:** Three spec functions:
```
extract_offset(addr, offset_bits) = addr & ((1 << offset_bits) - 1)
extract_index(addr, offset_bits, index_bits) = (addr >> offset_bits) & ((1 << index_bits) - 1)
extract_tag(addr, offset_bits, index_bits) = addr >> (offset_bits + index_bits)
```
All operating on `u64` addresses.

**Rationale:** Standard bitfield extraction. The `by(bit_vector)` proof
mode handles the bijection proof: `(tag << (offset_bits + index_bits)) | (index << offset_bits) | offset == addr`.

### 2. Cache Geometry Parameters

**Choice:** Three parameters: `offset_bits: u32`, `index_bits: u32`,
`num_ways: u32`. Requires `offset_bits + index_bits <= 64` and
`num_ways > 0`. `num_sets = 1 << index_bits`.

**Rationale:** Powers of two for offset and index are universal in hardware.
The number of ways is not necessarily a power of two (some caches are
12-way or 24-way), so it's a plain integer. The constraint
`offset_bits + index_bits <= 64` ensures the tag field exists.

### 3. Way Storage

**Choice:** A set is modeled as `Seq<Option<u64>>` of length `num_ways`.
`Some(tag)` means the way holds a valid line with that tag. `None` means
the way is empty (invalid).

**Rationale:** `Option<u64>` distinguishes valid from invalid without a
separate valid-bit array. The `Seq` lets Verus reason about way indices
and quantify over all ways for the miss proof.

### 4. Lookup Function

**Choice:** `lookup(set: Seq<Option<u64>>, tag: u64) -> Option<u32>` returns
`Some(way_index)` on hit, `None` on miss. Scans all ways linearly.

**Rationale:** Hardware does parallel comparison, but sequential scan is
equivalent for correctness (order doesn't matter since tags are unique
within a set). The linear scan lets Verus prove miss-completeness by
induction.

### 5. No-Duplicate-Tags Invariant

**Choice:** A predicate `no_duplicate_tags(set: Seq<Option<u64>>) -> bool`
that checks: for all i ≠ j, if `set[i] == Some(t1)` and `set[j] == Some(t2)`,
then `t1 != t2`.

**Rationale:** This is a precondition for the "at most one hit" proof.
If duplicates existed, a lookup could match two ways, making the hit
ambiguous. The invariant is maintained by insertion (only insert into an
empty way or after evicting a way with a different tag).

## Risks / Trade-offs

**[64-bit address assumption]** → The model uses `u64` addresses. 32-bit
systems would use `u32`. The proofs are parameterized by offset_bits and
index_bits, so they work for any address width as long as the sum fits.

**[No replacement metadata]** → The way storage only tracks tags, not
LRU bits or valid/dirty bits. Those belong to the eviction and cache
management modules. This module is purely about address→way mapping.

**[Linear scan vs. parallel]** → The exec function uses a linear loop,
which is O(ways). Real hardware does parallel comparison. The correctness
is identical; the model just doesn't express the parallelism.
