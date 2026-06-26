## Phase 1: Same-Line Predicate

- [x] Add `src/false_sharing.rs` and `lib.rs` module declaration
- [x] Define `same_cache_line_spec(a, b, line_size) -> bool` as `a / line_size == b / line_size`
- [x] Implement exec version using quotient equality `a / line_size == b / line_size`
- [x] Prove spec/exec equivalence directly from the shared quotient definition
- [x] Prove reflexivity: `same_cache_line(a, a, line_size)` is always true
- [x] Prove symmetry: `same_cache_line(a, b) == same_cache_line(b, a)`

## Phase 2: Alignment and Boundaries

- [x] Define `is_line_aligned(addr, line_size) -> bool` as `addr % line_size == 0`
- [x] Prove aligned addresses are at line boundaries
- [x] Define `line_start(addr, line_size) -> u64` as the floor multiple of `line_size`
- [x] Prove `same_cache_line(a, b) <==> line_start(a) == line_start(b)`

## Phase 3: Padding and Object Span

- [x] Define `padding_prevents_sharing(addr_a, size_a, addr_b, line_size) -> bool`
- [x] Prove: if `addr_b >= addr_a + size_a + line_size`, then no byte of object A shares a cache line with any byte of object B
- [x] Define `cache_lines_spanned(addr, size, line_size) -> u64`
- [x] Prove span formula: `ceil((addr % line_size + size) / line_size)`
- [x] Prove aligned single-line: span(aligned, size <= line_size) == 1

## Phase 4: Decomposition Equivalence

- [x] Prove `same_cache_line(a, b, 1 << offset_bits) <==> (tag(a) == tag(b) && index(a) == index(b))`
- [x] Prove distinct-line guarantee: `!same_cache_line ==> different tag or index`

## Phase 5: Tracey, Tests, and Validation

- [x] Add `r[false-sharing.*]` requirements to `docs/requirements.md`
- [x] Add `r[impl false-sharing.*]` / `r[verify false-sharing.*]` coverage across `src/false_sharing.rs`
- [x] Unit tests for same-line: known address pairs on 64-byte lines
- [x] Unit tests for alignment: aligned and unaligned addresses
- [x] Unit tests for object span: aligned, unaligned, cross-boundary
- [x] Proptest: same_cache_line matches brute-force division
- [x] Proptest: padding sufficiency for random address pairs
- [x] Proptest: decomposition equivalence against set_associative_lookup
- [x] Run `./scripts/verify.sh` and capture fresh passing evidence before archive
