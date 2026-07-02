## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for exportable proof-evidence bundles. r[proof-evidence-bundles.export]

## Phase 2: Implementation

- [x] [depends:spec] Define the proof-evidence bundle record shape, schema versioning, and explicit non-claim fields. Added `trellis-proof-evidence-v1` records with scope, evidence tier, scoped claims, and explicit non-claims. r[proof-evidence-bundles.export]
- [x] [depends:spec] Add deterministic generator and validator logic with pure bundle validation separated from filesystem/CLI shells. Added `src/bin/proof_evidence.rs` with pure generation/validation cores and filesystem-only CLI modes. r[proof-evidence-bundles.validation]
- [x] [depends:spec] Record BLAKE3 identities for source/proof inputs and validate freshness against local files. Bundle records include BLAKE3 content identities and validation rejects stale digests. r[proof-evidence-bundles.validation]
- [x] [depends:spec] Include requirement IDs, Tracey coverage status, verifier/toolchain identity, proof outcome, proof-gap references, and maturity facts when available. Generated records cover arithmetic, UCAN signature trusted boundaries, and the runnable proof example. r[proof-evidence-bundles.export]
- [x] [depends:spec] Add positive fixtures for complete valid bundles and negative fixtures for missing fields, stale hashes, unsupported schemas, missing gaps, missing coverage, and overbroad claims. Added unit tests and `self-test` fixtures covering all required failure modes. r[proof-evidence-bundles.validation]
- [x] [depends:spec] Document the downstream seam: Trellis exports proof evidence; Octet or another gate owns release admission. Added `docs/proof-evidence-bundles.md` and explicit bundle non-claims. r[proof-evidence-bundles.export]

## Phase 3: Validation

- [x] [depends:implementation] Run bundle generator/validator tests and fixture checks. Ran `cargo run --quiet --bin proof_evidence -- write`, `check`, `self-test`, and `cargo test --quiet --bin proof_evidence`. r[proof-evidence-bundles.validation]
- [x] [depends:implementation] Run Tracey uncovered/untested checks, Verus, cargo tests, clippy, Cairn validation/gates, and full repository verification as appropriate. Ran proof-evidence check/self-test, `cargo test --quiet --bin proof_evidence`, `cargo clippy --bin proof_evidence --tests -- -D warnings`, Tracey uncovered/untested (`0/1998`), and `./scripts/verus.sh src/lib.rs` (`10525 verified, 0 errors`). r[proof-evidence-bundles.validation]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. Synced accepted `proof-evidence-bundles` spec and validated the Cairn tree; archive execution follows this archive-ready checklist update. r[proof-evidence-bundles.export]
