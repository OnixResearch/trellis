## Why

Trellis checks several generated or checked-in JSON artifacts, but the consumer-facing schema expectations are scattered across Rust code, prose, and ad hoc validation. Nickel contracts can make these artifact shapes explicit without replacing the Rust generators or the semantic validators.

## What Changes

- Add Nickel contracts for repository-owned JSON evidence and policy artifacts where shape stability matters to downstream consumers.
- Start with `docs/proof-evidence-bundle.json`, `octet-baseline.json`, and generated policy JSON artifacts that are already checked in.
- Add contract-admission checks that run alongside existing Rust semantic validation.
- Add positive and negative fixtures for missing fields, unsupported schema versions, invalid digest formats, unsafe paths, overbroad claim fields, and malformed finding records.

## Scope

In scope:
- Consumer-facing shape contracts for generated JSON evidence artifacts.
- Deterministic Nickel typecheck/export or import-based validation wrappers.
- Positive and negative fixture coverage.
- Wiring into existing evidence/policy verification checks.

Out of scope:
- Replacing Rust generators or semantic validation logic.
- Treating shape validation as release approval or third-party conformance.
- Changing artifact schemas except where necessary to make existing expectations explicit.

## Impact

- **Files**: Nickel JSON artifact contracts, fixture files, evidence validation scripts, `docs/proof-evidence-bundle.json` checks, `octet-baseline.json` checks, and verification wrapper wiring.
- **Risks**: some existing artifacts may need normalization or explicit schema fields before contract validation is stable.
- **Testing**: contract fixture checks, existing proof-evidence validation, Octet baseline validation if available, policy freshness, Cairn validation/gates, and repo verification wrapper.
