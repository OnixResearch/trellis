## Phase 1: Back edge classification

- [x] Add `src/loop_detection.rs` with `is_back_edge` and `find_back_edges`
- [x] Cover back-edge classification against a brute-force dominator oracle
- [x] Keep the back-edge check valid for forward, cross, and self-loop edges

## Phase 2: Loop bodies and exits

- [x] Implement `compute_loop_body(edges, n, header, source)` for reachable non-self loops
- [x] Check computed bodies against a brute-force path oracle
- [x] Implement `loop_exits` and verify exit-edge tests against direct filtering
- [x] Exercise nested-loop and while/if-style shapes with unit tests

## Phase 3: Structural properties

- [x] Implement `header_dominates_body`
- [x] Implement `has_single_entry` for reachable side-entry checks
- [x] Cover header-dominance and single-entry properties with randomized tests

## Phase 4: Nesting

- [x] Implement `nested_in` over strict body inclusion
- [x] Implement `loop_nesting_forest` parent links for strict supersets
- [x] Cover nested-loop parent selection with unit tests and random loop sets built from CFGs

## Phase 5: Integration and docs

- [x] Add Tracey requirements in `docs/requirements.md`
- [x] Export `loop_detection` from `src/lib.rs`
- [x] Document module in `README.md`
- [x] Update change proposal/spec wording to match reachable non-self loop scope
