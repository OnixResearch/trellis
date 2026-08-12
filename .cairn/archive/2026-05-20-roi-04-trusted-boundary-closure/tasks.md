## Phase 1: Target selection

- [x] [serial] Run/read the proof-gap inventory and select exactly one closeable or narrowable trusted boundary target. Completed 2026-05-20T00:32:23Z; selected `src/nix_version.rs::compare_versions` broad trusted bridge for narrowing.
- [x] [serial] Update this change's design notes with the selected target, current classification, and closure strategy. Completed 2026-05-20T00:32:23Z; design records public-API narrowing and private fallback inventory target.

## Phase 2: Implementation

- [x] [depends:target-selection] Implement the verified replacement, checked wrapper, or narrower trust contract for the selected boundary. Completed 2026-05-20T00:32:23Z; admitted finite-model inputs now route through verified executable helpers, while only out-of-model/overflow inputs call `compare_versions_unmodeled_trusted`.
- [x] [depends:target-selection] Add focused positive and negative tests if the selected boundary has runtime behavior. Completed 2026-05-20T00:32:23Z; added focused admitted-path and unmodeled-fallback test coverage.
- [x] [depends:implementation] Update `docs/proof-gap-sites.tsv`, README counts, and requirement/spec references to match the actual source result. Completed 2026-05-20T00:32:23Z; inventory still reports 16 external bodies with the Nix site moved to the private fallback.

## Phase 3: Validation

- [x] [depends:implementation] Run focused tests, `./scripts/verus.sh src/lib.rs`, proof-gap inventory checker, Tracey uncovered/untested, and record evidence. Completed 2026-05-20T00:32:23Z; evidence recorded under `openspec/changes/roi-04-trusted-boundary-closure/evidence/`.
- [x] [depends:validation] Run `./scripts/verify.rs` and archive only if the raw log is non-contradictory and supports the updated count. Completed 2026-05-20T00:32:23Z; final verify rerun used `TMPDIR=/tmp` to avoid transient rustdoc temp-dir deletion and ended `=== ALL CHECKS PASSED ===`.
