## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for this follow-up.

## Phase 2: Implementation

- [x] [depends:spec] Design and add the minimal checked proof-gap site manifest format.
- [x] [depends:spec] Update the Rust checker to parse and validate the manifest without adding Python or shell gate logic.
- [x] [depends:spec] Capture positive, malformed, and stale-manifest negative evidence.
- [x] [depends:spec] Sync canonical verification-gaps requirements/spec and run strict validation.
- [x] [depends:spec] Run Rust checker, focused Verus/Tracey, and full verify evidence.

## Phase 3: Archive

- [x] [depends:verification] Archive, commit, push, and verify clean state.
