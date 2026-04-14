# Final task checklist evidence

## Phase 1

- `is_back_edge` / `find_back_edges`
  - code: `src/loop_detection.rs`
  - exact contract: `find_back_edges(... ) ensures result@ == back_edges_spec(...)`
  - tests: `test_find_back_edges_nested_loops`, `prop_back_edges_match_manual`

## Phase 2

- `compute_loop_body(edges, reachable, dom, header, source)`
  - code: `src/loop_detection.rs`
  - exact contract: `ensures result@ == loop_body_spec(...)`
  - tests: `test_compute_loop_body_nested_loops`, `test_compute_loop_body_while_if_shape`, `prop_loop_body_matches_bruteforce`
- `loop_exits`
  - exact contract: `ensures result@ == loop_exits_spec(...)`
  - tests: `test_loop_exits_nested_loops`, `test_compute_loop_body_while_if_shape`, `prop_exit_edges_match_partition`

## Phase 3

- `header_dominates_body`
  - code: `src/loop_detection.rs`
  - exact contract: `ensures result == header_dominates_body_spec(...)`
  - tests: `test_header_dominates_nested_loop_bodies`, `test_compute_loop_body_while_if_shape`, `prop_headers_dominate_computed_bodies`
- `has_single_entry(edges, reachable, header, body)`
  - exact contract: `ensures result == single_entry_spec(...)`
  - tests: `test_single_entry_nested_loop`, `test_compute_loop_body_while_if_shape`, `prop_computed_loops_have_single_entry`

## Phase 4

- `nested_in`
  - exact contract: `ensures result == strict_body_subset_spec(...)`
  - tests: `test_loop_nesting_nested_loops`
- `loop_nesting_forest`
  - requirement-level contract: parent is a strict superset
  - deterministic implementation detail: choose smallest strict-superset parent, ties by lower index
  - tests: `test_loop_nesting_forest_nested_loops`, `test_loop_nesting_forest_picks_immediate_parent`, `prop_nesting_forest_parent_selection`

## Phase 5

- requirements synced: `docs/requirements.md`
- synced spec: `openspec/specs/loop-detection/spec.md`
- archived proposal/design/tasks synced under `openspec/changes/archive/2026-04-14-loop-detection/`
- module exported from `src/lib.rs`
  - current line evidence: `evidence/lib-export-transcript.txt`
  - current crate root line: `353:pub mod loop_detection;`
- README module table updated in `README.md`
