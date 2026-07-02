## Context

Trellis can already prove many reusable logic modules. Downstream adoption improves when those proof facts are available as stable, machine-checkable evidence rather than prose or raw verifier logs. A proof-evidence bundle should be a deterministic summary of local facts: what requirement was checked, where the source/proof lives, which command/toolchain produced the outcome, what content was identified, what coverage exists, and what trust boundaries remain.

The bundle is not a release approval. It is an input to a downstream policy engine or release gate.

## Goals

- Export a deterministic bundle of proof evidence for selected modules, examples, or the whole repository.
- Make bundle records self-describing enough for downstream consumers to validate scope and freshness.
- Use BLAKE3 for new content identities unless an existing interoperability contract requires another algorithm.
- Fail closed on missing fields, stale identities, unsupported schemas, untracked proof gaps, and unsupported proof claims.
- Keep generation deterministic and local: no network access, wall-clock dependency, or external service state.

## Non-Goals

- Do not sign or publish bundles to a remote registry in this change.
- Do not change Verus proof semantics or Tracey coverage rules.
- Do not make Octet depend on this bundle until a separate Octet change accepts the contract.
- Do not claim a bundle proves whole-program correctness, production safety, or third-party conformance.

## Decisions

### Bundle record shape

Each bundle record SHALL include a schema version, bundle scope, requirement IDs, module/source paths, proof or verifier command, verifier/toolchain identity when available, proof outcome, Tracey coverage status, trusted-boundary/proof-gap references, maturity or usage-surface facts when available, BLAKE3 content identities, and explicit non-claims.

### Validation behavior

Validation SHALL fail closed when required fields are absent, content identities do not match local files, schema versions are unsupported, requirement IDs are uncovered/untested, proof-gap references are missing, or claim text exceeds the recorded evidence tier.

### Data ownership

If the bundle manifest is human-authored policy/configuration, it SHOULD be authored in Nickel with typed contracts. If the bundle is emitted by a Rust tool after verification, the DTO and validation logic SHOULD be Rust-owned and deterministic. Runtime generation SHALL not require Nickel evaluation unless a later change explicitly scopes that behavior.

### Downstream seam

The bundle MAY be consumed by Octet or other gates, but Trellis only vouches for the exported proof facts and boundaries. A downstream release gate owns admission policy, artifact provenance, and release approval.

## Risks / Trade-offs

- Bundles can go stale. Mitigate with BLAKE3 identity checks and fail-closed validation.
- Raw verifier logs can be too large. Mitigate by summarizing required facts and linking bounded logs or identities.
- Consumers may overinterpret bundles. Mitigate with explicit non-claims and evidence-tier labels.
- Schema design can expand too quickly. Mitigate by starting with a minimal record shape and versioning from the first implementation.

## Validation

- Positive fixture validates a complete bundle for at least one representative proof-backed module or example.
- Negative fixtures reject missing requirement IDs, stale BLAKE3 identities, unsupported schema versions, missing proof-gap references, missing coverage, and overbroad claims.
- Generator/checker tests keep pure parsing/validation separate from filesystem and command execution shells.
- Tracey uncovered/untested checks and Verus verification remain green for any introduced requirement IDs.
- Cairn validation and gates pass for this package.
