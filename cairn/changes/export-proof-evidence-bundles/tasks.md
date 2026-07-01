## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for exportable proof-evidence bundles. r[proof-evidence-bundles.export]

## Phase 2: Implementation

- [ ] [depends:spec] Define the proof-evidence bundle record shape, schema versioning, and explicit non-claim fields.
- [ ] [depends:spec] Add deterministic generator and validator logic with pure bundle validation separated from filesystem/CLI shells.
- [ ] [depends:spec] Record BLAKE3 identities for source/proof inputs and validate freshness against local files.
- [ ] [depends:spec] Include requirement IDs, Tracey coverage status, verifier/toolchain identity, proof outcome, proof-gap references, and maturity facts when available.
- [ ] [depends:spec] Add positive fixtures for complete valid bundles and negative fixtures for missing fields, stale hashes, unsupported schemas, missing gaps, missing coverage, and overbroad claims.
- [ ] [depends:spec] Document the downstream seam: Trellis exports proof evidence; Octet or another gate owns release admission.

## Phase 3: Validation

- [ ] [depends:implementation] Run bundle generator/validator tests and fixture checks.
- [ ] [depends:implementation] Run Tracey uncovered/untested checks, Verus, cargo tests, clippy, Cairn validation/gates, and full repository verification as appropriate.
- [ ] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence.
