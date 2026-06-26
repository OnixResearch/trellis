## Phase 1: Address Decomposition

- [x] Define `extract_offset(addr, offset_bits) -> u64`
- [x] Define `extract_index(addr, offset_bits, index_bits) -> u64`
- [x] Define `extract_tag(addr, offset_bits, index_bits) -> u64`
- [x] Define `reconstruct_address(tag, index, offset, offset_bits, index_bits) -> u64`
- [x] Prove bijection: reconstruct(decompose(addr)) == addr via `by(bit_vector)`
- [x] Prove component bounds: offset < 2^offset_bits, index < 2^index_bits

## Phase 2: Set Lookup

- [x] Define way storage: `Seq<Option<u64>>` of length num_ways
- [x] Implement `lookup(set, tag) -> Option<u32>` linear scan
- [x] Prove hit correctness: returned way holds the matching tag
- [x] Prove miss completeness: None means no way holds the tag
- [x] Prove at most one match when no-duplicate-tags invariant holds

## Phase 3: Set Isolation and Invariants

- [x] Prove set isolation: different indices map to different sets
- [x] Define `no_duplicate_tags(set) -> bool` predicate
- [x] Prove insert into empty way preserves no-duplicate-tags
- [x] Prove no-aliasing: different tags in same set never produce false hit

## Phase 4: Exec Functions

- [x] Write exec-level decompose/reconstruct with ensures
- [x] Write exec-level lookup with linear scan and ensures
- [x] Write exec-level no_duplicate_tags checker

## Phase 5: Tests

- [x] Unit tests for decomposition with known address/geometry pairs
- [x] Unit tests for lookup hit, miss, empty set
- [x] Proptest: decomposition roundtrip for random addresses and geometries
- [x] Proptest: no aliasing — random addresses with different tags never collide
- [x] Proptest: miss completeness — randomly filled set, lookup for absent tag
