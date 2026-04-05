## Phase 1: Same-Line Predicate

- [ ] Define `same_cache_line_spec(a, b, line_size) -> bool` as `a / line_size == b / line_size`
- [ ] Implement exec version using bit mask `(a & !mask) == (b & !mask)`
- [ ] Prove spec/exec equivalence via `by(bit_vector)` (requires power-of-two line_size)
- [ ] Prove reflexivity: `same_cache_line(a, a, line_size)` is always true
- [ ] Prove symmetry: `same_cache_line(a, b) == same_cache_line(b, a)`

## Phase 2: Alignment and Boundaries

- [ ] Define `is_line_aligned(addr, line_size) -> bool` as `addr % line_size == 0`
- [ ] Prove aligned addresses are at line boundaries
- [ ] Define `line_start(addr, line_size) -> u64` as `addr & !mask`
- [ ] Prove `same_cache_line(a, b) <==> line_start(a) == line_start(b)`

## Phase 3: Padding and Object Span

- [ ] Define `padding_prevents_sharing(addr_a, size_a, addr_b, line_size) -> bool`
- [ ] Prove: if `addr_b >= addr_a + line_size`, then not same_cache_line
- [ ] Define `cache_lines_spanned(addr, size, line_size) -> u64`
- [ ] Prove span formula: `ceil((addr % line_size + size) / line_size)`
- [ ] Prove aligned single-line: span(aligned, size <= line_size) == 1

## Phase 4: Decomposition Equivalence

- [ ] Prove `same_cache_line(a, b, 1 << offset_bits) <==> (tag(a) == tag(b) && index(a) == index(b))`
- [ ] Prove distinct-line guarantee: `!same_cache_line ==> different tag or index`

## Phase 5: Tests

- [ ] Unit tests for same-line: known address pairs on 64-byte lines
- [ ] Unit tests for alignment: aligned and unaligned addresses
- [ ] Unit tests for object span: aligned, unaligned, cross-boundary
- [ ] Proptest: same_cache_line matches brute-force division
- [ ] Proptest: padding sufficiency for random address pairs
- [ ] Proptest: decomposition equivalence against set_associative_lookup
