## Context

Generated JSON artifacts are a public boundary for downstream tooling. Rust generators and validators already own semantic checks such as stale BLAKE3 identities, coverage status, and proof-gap references. Nickel is still useful as an admission layer for stable shape contracts: supported schema versions, required fields, path hygiene, digest text formats, bounded enum values, and explicit non-claim fields.

## Decisions

### 1. Nickel validates shape, Rust validates semantics

**Choice:** Use Nickel contracts to reject malformed JSON artifact shape before or alongside existing Rust semantic validators. Keep content identity recomputation, Tracey coverage checks, proof-gap reconciliation, and Octet-specific semantics in Rust tools.

**Rationale:** This keeps the functional core clear: Nickel admits typed data, Rust checks repository-specific facts that require filesystem and tool execution.

### 2. Start with checked-in artifacts that downstream consumers can see

**Choice:** Cover:
- `docs/proof-evidence-bundle.json`
- `octet-baseline.json`
- `proof-review/generated/policy.json`
- `verification-policy/generated/policy.json`
- `cairn-policy/generated/cairn-policy.json`

**Rationale:** These files are already reviewable artifacts, and malformed shape would create downstream confusion even if internal generators are correct.

### 3. Contracts expose non-claims and digest/path hygiene

**Choice:** Contracts require supported schema identifiers, explicit non-claims where relevant, BLAKE3 digest shape for digest fields, safe relative paths for path fields, and bounded enum values for severity/status/outcome fields.

**Rationale:** The strongest recurring risks are overbroad claims, stale or malformed identity text, unsafe paths, and silent enum drift.

### 4. Fixtures prove both acceptance and rejection

**Choice:** Add minimal valid fixtures and rejected fixtures for missing required fields, unsupported schemas, malformed BLAKE3 digests, unsafe paths, unknown severity/outcome values, overbroad claim fields, and malformed Octet finding records.

**Rationale:** The contract is not complete until negative evidence shows bad artifacts fail closed.

## Risks / Trade-offs

- Some artifacts are large. Keep fixtures small and deterministic rather than duplicating full real artifacts.
- Contract validation should not duplicate every semantic Rust check. The line is shape and consumer admission in Nickel; repository fact validation stays in Rust.
- Generated policy artifacts may already be covered by policy freshness. Shape-contract checks should compose with, not replace, freshness.
