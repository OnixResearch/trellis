## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for this follow-up.

## Phase 2: Implementation

- [ ] [depends:spec] Inspect `src/nix_version.rs::compare_versions` and define the finite token/order model.
- [ ] [depends:spec] Implement verified parsing/comparison helpers or a narrowed wrapper around the existing bridge.
- [ ] [depends:spec] Add tests/proofs for modeled comparisons and explicit out-of-model behavior.
- [ ] [depends:spec] Update proof-gap inventory/count/site/classification evidence if the bridge changes.
- [ ] [depends:spec] Run cargo, clippy, Verus, Tracey, OpenSpec validation, and full verify evidence.

## Phase 3: Archive

- [ ] [depends:verification] Archive, commit, push, and verify clean state.
