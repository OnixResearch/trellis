## Phase 1: Contract surfaces

- [x] [serial] Define Nickel JSON artifact contracts for proof-evidence bundles, Octet baseline findings, and generated policy artifacts without replacing existing Rust semantic validators. r[verification-tooling.json-artifact-contract-admission]
- [x] [depends:contract-surfaces] Add BLAKE3 digest text, safe relative path, supported schema, bounded enum, required-field, and explicit non-claim checks for the covered artifact families. r[verification-tooling.json-artifact-contract-admission]

## Phase 2: Checker integration

- [x] [depends:contract-surfaces] Add a JSON artifact contract checker that runs Nickel contract admission against checked-in artifacts and reports deterministic diagnostics. r[verification-tooling.json-artifact-contract-admission]
- [x] [depends:checker] Compose contract admission with existing `proof_evidence` validation so shape failures are reported before or alongside stale-identity and coverage failures. r[proof-evidence-bundles.validation]

## Phase 3: Positive and negative fixtures

- [x] [depends:checker] Add minimal valid fixtures for proof-evidence bundles, Octet baselines, and generated policy artifacts. r[verification-tooling.json-artifact-contract-fixtures]
- [x] [depends:fixtures] Add negative fixtures for missing required fields, unsupported schema versions, malformed digest fields, unsafe paths, unsupported enum values, overbroad claim fields, and malformed finding records. r[verification-tooling.json-artifact-contract-fixtures]
- [x] [depends:fixtures] Cover the JSON artifact checker with positive and negative self-tests. r[verification-tooling.json-artifact-contract-fixtures]

## Phase 4: Integration and validation

- [x] [depends:self-tests] Integrate JSON artifact contract admission into the repo verification wrapper or the smallest existing evidence/policy check that runs during full verification. r[verification-tooling.json-artifact-contract-admission]
- [x] [depends:integration] Run contract fixtures, proof-evidence validation, Octet baseline validation when available, policy freshness, Cairn validation/gates, and the repo verification wrapper; archive command output before completing the change. r[verification-tooling.json-artifact-contract-fixtures]
