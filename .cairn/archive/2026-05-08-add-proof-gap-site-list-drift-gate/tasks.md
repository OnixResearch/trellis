## Phase 1: Specification

- [x] [serial] Define exact proof-gap site-list drift requirements and design scope.

## Phase 2: Rust gate implementation

- [x] [serial] Extend `scripts/check-proof-gap-inventory.rs` with exact external-body site-list validation and a same-count negative fixture.
- [x] [depends:rust-gate] Add Tracey-covered audit constants/proofs for the site-list gate.
- [x] [depends:rust-gate] Capture positive and same-count negative fixture evidence.

## Phase 3: Verification and archive

- [x] [depends:evidence] Sync canonical requirements/spec text and run strict OpenSpec validation.
- [x] [depends:validation] Run Rust checker, focused Verus, Tracey uncovered/untested, and full `./scripts/verify.sh` evidence.
- [x] [depends:verification] Archive, commit, push, and verify clean state.
