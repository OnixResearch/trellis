# Verification Gaps Specification

## Purpose

Tracks the trusted or weakened proof boundaries in the current tree.
This change narrows that inventory by removing the closeable Zircon
exec/spec bridges while keeping the remaining permanent and Shamir
boundaries explicit.

## Current Status

After this change, the Zircon model no longer contributes any
`#[verifier::external_body]` sites. The remaining trust inventory is:

- `53` total `#[verifier::external_body]` items across `11` files
- `15` permanent boundaries
- `38` closeable Shamir-secret-sharing boundaries
- `0` `assume(...)` soundness oracles
- `0` `@verified-logic:no-ensures` / placeholder-spec surfaces

## Requirements

### Requirement: Zircon Trust Bridges Are Internalized

The verification-gap inventory MUST NOT list closeable Zircon
`#[verifier::external_body]` bridges after this change.

#### Scenario: Queue and buffer bridges removed

- GIVEN the main source tree after this change
- WHEN the trusted inventory is regenerated from `src/`
- THEN `src/zircon_channel.rs`, `src/zircon_fifo.rs`, and `src/zircon_socket.rs`
  contribute zero `#[verifier::external_body]` sites

#### Scenario: Tree and port bridges removed

- GIVEN the main source tree after this change
- WHEN the trusted inventory is regenerated from `src/`
- THEN `src/zircon_job.rs` and `src/zircon_port.rs` contribute zero
  `#[verifier::external_body]` sites

### Requirement: Published Trust Counts Match The Reduced Inventory

The published proof-posture summary MUST reflect the reduced trust
inventory after the Zircon bridges are internalized.

#### Scenario: Main inventory refreshed

- GIVEN the reduced trusted-site scan result
- WHEN `openspec/specs/verification-gaps/spec.md` is updated
- THEN it reports `53` total `#[verifier::external_body]` items,
  `15` permanent boundaries, and `38` closeable boundaries

#### Scenario: README refreshed

- GIVEN the reduced trusted-site scan result
- WHEN `README.md` is updated
- THEN its trusted-boundary summary matches the same `53 / 15 / 38` counts
