# Design: Adopt Nickel 1.17 for proof configuration

## Context

Trellis uses Nickel for proof profiles, evidence contracts, and repository checks. The current package set provides Nickel `1.16.0`.

## Decisions

### Decision: Pin the exact upstream evaluator

**Choice:** Select Nickel `1.17.0` from commit `1320a983e6c3d1e2fb53dd2464b084b4903b1426`.

Use a narrow source input when the current Nix package set lacks this version.

**Rationale:** The change must not include unrelated proof-tool or Nixpkgs updates.

### Decision: Preserve proof authority

**Choice:** Nickel validates profile and evidence shape. Verus, Rust checks, and repository proof rails retain proof authority.

**Rationale:** Configuration acceptance does not prove a theorem.

### Decision: Run positive and negative fixtures

**Choice:** Run valid profiles and rejected malformed, missing-import, contract, unknown-field, and identity-mismatch fixtures.

Regenerate `flake.lock` only with Nix lock commands.

**Rationale:** Rejection behavior is part of the reviewed proof boundary.

### Decision: Record exact evaluator identity

**Choice:** Evidence that depends on Nickel records the version and source commit.

The record must preserve proof scope and non-claims.

## Risks and trade-offs

- New validation can reveal stale proof configuration.
- Diagnostic text can change without proof-statement changes.
- A passing profile does not prove downstream consumers correct.
