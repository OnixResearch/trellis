# Verification Gaps Specification

## Purpose

Tracks the trusted or weakened proof boundaries in the current tree.
This change narrows that inventory by removing the closeable Shamir
exec/spec bridges while keeping the remaining permanent boundaries
explicit.

## Current Status

After this change, the Shamir model no longer contributes any
`#[verifier::external_body]` sites. The remaining trust inventory is:

- `15` total `#[verifier::external_body]` items across `5` files
- `15` permanent boundaries
- `0` closeable boundaries
- `0` `assume(...)` soundness oracles
- `0` `@verified-logic:no-ensures` / placeholder-spec surfaces

## Requirements

### Requirement: Shamir Trust Bridges Are Internalized

The verification-gap inventory MUST NOT list closeable Shamir
`#[verifier::external_body]` bridges after this change.

#### Scenario: Field and polynomial bridges removed

- GIVEN the main source tree after this change
- WHEN the trusted inventory is regenerated from `src/`
- THEN `src/shamir_field.rs` and `src/shamir_poly.rs` contribute zero
  `#[verifier::external_body]` sites

#### Scenario: Interpolation bridges removed

- GIVEN the main source tree after this change
- WHEN the trusted inventory is regenerated from `src/`
- THEN `src/shamir_lagrange.rs` contributes zero
  `#[verifier::external_body]` sites

#### Scenario: Split, reconstruct, and threshold bridges removed

- GIVEN the main source tree after this change
- WHEN the trusted inventory is regenerated from `src/`
- THEN `src/shamir_split.rs`, `src/shamir_reconstruct.rs`, and
  `src/shamir_threshold.rs` contribute zero `#[verifier::external_body]` sites

### Requirement: Secret-Sharing Verification Stays Fully Internal

Shamir secret-sharing requirements MUST stay green after the trusted
bridges are removed.

#### Scenario: Existing requirement coverage preserved

- GIVEN the current Tracey requirement IDs for the Shamir modules
- WHEN the Shamir proof bridges are internalized
- THEN `shamir-field.*`, `shamir-poly.*`, `shamir-lagrange.*`,
  `shamir-split.*`, `shamir-reconstruct.*`, and `shamir-threshold.*`
  remain covered and verified

#### Scenario: Verification pipeline remains green

- GIVEN the source tree after this change
- WHEN `cargo test`, `cargo clippy --all-targets -- -D warnings`,
  `./scripts/verus.sh src/lib.rs`, and `./scripts/verify.sh` run
- THEN all four commands succeed

### Requirement: Published Trust Counts Match The Reduced Inventory

The published proof-posture summary MUST reflect the reduced trust
inventory after the Shamir bridges are internalized.

#### Scenario: Main inventory refreshed

- GIVEN the reduced trusted-site scan result
- WHEN `openspec/specs/verification-gaps/spec.md` is updated
- THEN it reports `15` total `#[verifier::external_body]` items,
  `15` permanent boundaries, and `0` closeable boundaries

#### Scenario: README refreshed

- GIVEN the reduced trusted-site scan result
- WHEN `README.md` is updated
- THEN its trusted-boundary summary matches the same `15 / 15 / 0` counts
