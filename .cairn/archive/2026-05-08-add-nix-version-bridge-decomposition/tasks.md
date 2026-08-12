## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for this follow-up.

## Phase 2: Implementation

- [x] [depends:spec] Inspect `src/nix_version.rs::compare_versions` and define the finite token/order model.
- [x] [depends:spec] Implement verified parsing/comparison helpers or a narrowed wrapper around the existing bridge.
- [x] [depends:spec] Add tests/proofs for modeled comparisons and explicit out-of-model behavior.
- [x] [depends:spec] Update proof-gap inventory/count/site/classification evidence if the bridge changes.
- [x] [depends:spec] Run cargo, clippy, Verus, Tracey, OpenSpec validation, and full verify evidence.

## Phase 3: Archive

- [x] [depends:verification] Archive, commit, push, and verify clean state.
