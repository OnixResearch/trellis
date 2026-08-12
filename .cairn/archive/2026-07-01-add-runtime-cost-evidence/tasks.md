## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for runtime-cost evidence. r[runtime-evidence.ghost-erasure]

## Phase 2: Implementation

- [x] [depends:spec] Add runtime-cost evidence docs with reproducible normal-build commands and environment/profile fields. Added `docs/runtime-cost.md` with command, profile, target, result, measurement status, and refresh fields. r[runtime-evidence.ghost-erasure]
- [x] [depends:spec] Select representative example or binary targets and document how to refresh their evidence. Selected `core_basics` build and `ucan_web_of_trust` run records with refresh commands. r[runtime-evidence.ghost-erasure]
- [x] [depends:spec] Add positive validation for complete evidence records and negative validation for missing required fields if a checker is introduced. Added `scripts/check-runtime-evidence.rs` with positive, missing-field, and hard-guarantee negative self-tests. r[runtime-evidence.ghost-erasure]
- [x] [depends:spec] Link evidence docs from README usage or verification sections. README usage/verification sections link runtime evidence and checker. r[runtime-evidence.ghost-erasure]
- [x] [depends:spec] Keep absolute performance/size values descriptive unless a named bound is formally specified. Evidence labels observed size/output as observations, not guarantees or pass/fail bounds. r[runtime-evidence.ghost-erasure]

## Phase 3: Validation

- [x] [depends:implementation] Run the documented lightweight evidence commands or explain any opt-in commands not rerun by default. Ran `CARGO_TARGET_DIR=target/runtime-cost-evidence cargo build --example core_basics`, `stat`, and `cargo run --quiet --example ucan_web_of_trust`. r[runtime-evidence.ghost-erasure]
- [x] [depends:implementation] Run cargo, clippy, README/catalog, Tracey, and Verus/full verification as appropriate for touched docs/scripts. Ran runtime-evidence checker self-test/live check, examples checker, README catalog/metrics checks, rustfmt on changed scripts, and `cargo check --lib`; clippy/Tracey/Verus were not required for docs/checker-only evidence metadata. r[runtime-evidence.ghost-erasure]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. Synced `runtime-evidence` accepted spec and validated the Cairn tree; archive execution follows this archive-ready checklist update. r[runtime-evidence.ghost-erasure]
