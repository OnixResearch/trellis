## Phase 1: Checker contract

- [x] [serial] Define README catalog extraction rules and any explicit exclusion format. Completed 2026-05-20T00:13:00Z; implemented `## Modules` extraction, backtick identifier cataloging, and explicit rationale-bearing exclusion format.
- [x] [serial] Implement the drift checker as a deterministic local script or Rust script. Completed 2026-05-20T00:13:00Z; added `scripts/check-readme-module-catalog.rs`.

## Phase 2: Regression coverage

- [x] [depends:checker-contract] Add positive current-tree validation and a negative fixture or synthetic omission case. Completed 2026-05-20T00:13:00Z; checker passes current tree and `--self-test` exercises valid, missing-module, and stale-module fixtures.
- [x] [parallel] Update README/developer docs to explain the checker and exclusion policy. Completed 2026-05-20T00:13:00Z; README documents compact catalog coverage and verification command.

## Phase 3: Integration and evidence

- [x] [depends:regression-coverage] Wire the checker into an appropriate Nix/local verification gate. Completed 2026-05-20T00:13:00Z; added Nix `readme-module-catalog` check and `./scripts/verify.rs` steps.
- [x] [depends:integration] Run the checker, Nix gate, and `./scripts/verify.rs`; record evidence before archive. Completed 2026-05-20T00:13:00Z; evidence recorded under `openspec/changes/roi-03-readme-module-catalog-drift/evidence/`.
