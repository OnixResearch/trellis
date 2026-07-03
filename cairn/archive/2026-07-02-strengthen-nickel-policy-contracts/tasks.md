## Phase 1: Shared Nickel contracts

- [x] [serial] Add a shared Nickel contract module for non-empty strings, safe relative paths, supported enums, non-empty arrays, unique IDs, and BLAKE3 digest text. r[verification-policy.reusable-nickel-contract-library]
- [x] [depends:shared-contracts] Refactor `verification-policy`, `proof-review`, and `cairn-policy` Nickel sources to import shared contracts where semantics match existing local predicates. r[verification-policy.reusable-nickel-contract-library]

## Phase 2: Cross-field policy integrity

- [x] [depends:shared-contracts] Strengthen `proof-review/policy.ncl` so mode references, recipe rule IDs, and recipe validation gates must point at declared policy entries. r[verification-policy.nickel-reference-integrity]
- [x] [depends:shared-contracts] Strengthen `cairn-policy/contracts.ncl` so artifact dependencies, replay groups, determinism surfaces, and receipt schema references must point at declared policy entries. r[verification-policy.nickel-reference-integrity]
- [x] [depends:shared-contracts] Strengthen `verification-policy/policy.ncl` with bounded checker script paths, generated artifact paths, severities, and exemption metadata. r[verification-policy.nickel-authored]

## Phase 3: Positive and negative fixtures

- [x] [depends:reference-integrity] Add valid policy fixtures plus invalid fixtures for dangling references, duplicate IDs, unsafe paths, empty required arrays, unsupported enum values, and missing fields. r[verification-policy.nickel-negative-fixtures]
- [x] [depends:fixtures] Add or extend policy fixture checker scripts so each invalid fixture is expected to fail closed and each valid fixture exports deterministically. r[verification-policy.nickel-negative-fixtures]

## Phase 4: Integration and validation

- [x] [depends:fixtures] Refresh generated policy JSON artifacts from Nickel and keep freshness checks deterministic. r[verification-policy.nickel-authored]
- [x] [depends:integration] Run Nickel typecheck/export/freshness, policy fixture checks, proof-review self-tests, Cairn validation/gates, and the repo verification wrapper; archive command output as evidence before completing the change. r[verification-policy.nickel-negative-fixtures]
