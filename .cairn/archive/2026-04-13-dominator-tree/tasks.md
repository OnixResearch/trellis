## Phase 1: Implementation

- [x] Add `src/dominator_tree.rs` with graph validity specs, predecessor collection, iterative dominance-matrix computation, immediate-dominator extraction, and dominance-frontier computation.
- [x] Export `dominator_tree` from `src/lib.rs`.

## Phase 2: Requirements and docs

- [x] Add `dominator-tree.*` requirements to `docs/requirements.md` and annotate the implementation/tests with Tracey tags.
- [x] Sync the dominator-tree OpenSpec domain into `openspec/specs/dominator-tree/spec.md`.
- [x] Add `dominator_tree` to the README graph-algorithms inventory.

## Phase 3: Verification

- [x] Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/verify.sh`, `./scripts/check-ensures.sh`, and direct `tracey query uncovered` / `tracey query untested`.
- [x] Keep unit tests and proptests covering entry dominance, partial order, immediate-dominator/tree shape, and dominance-frontier behavior.
