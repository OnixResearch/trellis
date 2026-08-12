## Phase 1: Spec Foundation

- [x] [serial] Write proposal, design, and composite-primitives delta spec for bounded graph-query composition.
- [x] [serial] Validate the active OpenSpec package and keep implementation work deferred to a drain slice.

## Phase 2: Implementation

- [x] [serial] Add `src/graph_query_composition.rs` with finite model, triple-pattern, binding, path-witness, join/filter, index-equivalence, and serialized-envelope predicates.
- [x] [depends:implementation] Export `graph_query_composition` from `src/lib.rs` and add focused tests for positive and negative admission cases.
- [x] [depends:implementation] Add Verus proof functions for every `composite-primitives.graph-query.*` requirement and keep public APIs annotated with `r[impl]`/`r[verify]` coverage.
- [x] [depends:implementation] Sync the delta into `openspec/specs/composite-primitives/spec.md` without clobbering existing composite requirements.

## Phase 3: Verification and Archive

- [x] [depends:implementation] Run focused graph-query tests, `./scripts/verus.sh src/lib.rs`, Tracey prefix checks for `composite-primitives.graph-query`, and `./scripts/verify.sh`; save reviewable evidence.
  - Evidence: `cargo test --quiet graph_query_composition` passed 8 tests; `cargo clippy --all-targets -- -D warnings` passed; `./scripts/verus.sh src/lib.rs` reported `9801 verified, 0 errors`; `tracey query uncovered` and `tracey query untested` reported 0 graph-query hits; `./scripts/verify.sh` ended with `=== ALL CHECKS PASSED ===` and Tracey `0 uncovered` / `0 untested` out of 1866 rules.
- [x] [depends:verification] Mark implementation/evidence tasks complete, archive `add-graph-query-composition`, validate touched canonical specs, commit, push, and verify clean state.
