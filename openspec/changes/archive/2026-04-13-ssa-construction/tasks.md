## Phase 1: Implementation

- [x] Add `src/ssa_construction.rs` with raw/SSA IR models, iterated-dominance-frontier phi placement, SSA renaming, dominance checking, and phi elimination helpers.
- [x] Export `ssa_construction` from `src/lib.rs`.

## Phase 2: Requirements and docs

- [x] Add `ssa.*` requirements to `docs/requirements.md` and annotate the implementation/tests with Tracey tags.
- [x] Sync the SSA construction OpenSpec domain into `openspec/specs/ssa-construction/spec.md`.
- [x] Add `ssa_construction` to the README graph-algorithms inventory and refresh the README headline counts.
- [x] Refresh `openspec/specs/verification-gaps/spec.md` with the current live proof/test/Tracey counts.

## Phase 3: Verification

- [x] Keep unit tests and proptests covering IDF convergence, phi placement, SSA uniqueness, dominance-of-uses, and phi-elimination path semantics.
- [x] Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, direct `tracey query uncovered` / `tracey query untested`, and `./scripts/verify.sh`.
