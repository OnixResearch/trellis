## Phase 1: Address Decomposition

- [ ] Define `extract_offset(addr, offset_bits) -> u64`
- [ ] Define `extract_index(addr, offset_bits, index_bits) -> u64`
- [ ] Define `extract_tag(addr, offset_bits, index_bits) -> u64`
- [ ] Define `reconstruct_address(tag, index, offset, offset_bits, index_bits) -> u64`
- [ ] Prove bijection: reconstruct(decompose(addr)) == addr via `by(bit_vector)`
- [ ] Prove component bounds: offset < 2^offset_bits, index < 2^index_bits

## Phase 2: Set Lookup

- [ ] Define way storage: `Seq<Option<u64>>` of length num_ways
- [ ] Implement `lookup(set, tag) -> Option<u32>` linear scan
- [ ] Prove hit correctness: returned way holds the matching tag
- [ ] Prove miss completeness: None means no way holds the tag
- [ ] Prove at most one match when no-duplicate-tags invariant holds

## Phase 3: Set Isolation and Invariants

- [ ] Prove set isolation: different indices map to different sets
- [ ] Define `no_duplicate_tags(set) -> bool` predicate
- [ ] Prove insert into empty way preserves no-duplicate-tags
- [ ] Prove no-aliasing: different tags in same set never produce false hit

## Phase 4: Exec Functions

- [ ] Write exec-level decompose/reconstruct with ensures
- [ ] Write exec-level lookup with linear scan and ensures
- [ ] Write exec-level no_duplicate_tags checker

## Phase 5: Tests

- [ ] Unit tests for decomposition with known address/geometry pairs
- [ ] Unit tests for lookup hit, miss, empty set
- [ ] Proptest: decomposition roundtrip for random addresses and geometries
- [ ] Proptest: no aliasing — random addresses with different tags never collide
- [ ] Proptest: miss completeness — randomly filled set, lookup for absent tag
