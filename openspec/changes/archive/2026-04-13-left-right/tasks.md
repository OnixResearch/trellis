## Phase 1: Core Left-Right State

- [x] Implement `src/left_right.rs` with the two-side state model, side helpers, synchronized initial state, and invariant/ready predicates
- [x] Implement reader enter/exit with side-local counts and snapshot tickets
- [x] Implement writer publish and catch-up transitions with explicit quiescence gating

## Phase 2: Proofs and Tests

- [x] Prove side toggling, publish alternation, version advance, reader enter/exit roundtrip, quiescence restoration, and snapshot stability lemmas
- [x] Add unit tests and property tests covering enter/exit, publish, catch-up, and live-reader blocking
- [x] Export the new module from `src/lib.rs` and annotate public items with Tracey requirement markers

## Phase 3: Specs and Validation

- [x] Add `left-right` requirements to `docs/requirements.md`
- [x] Add the change spec and sync it to `openspec/specs/left-right/spec.md`
- [x] Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `tracey query uncovered`, `tracey query untested`, and `./scripts/verify.sh`
