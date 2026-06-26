# Verification Gaps Specification

## Purpose

This change closes one closeable proof boundary by internalizing
`src/ucan_did.rs::varint_decode`. It also requires the published proof-gap
inventory to stay source-derived and non-double-counted after the boundary is
removed.

## Requirements

### Requirement: UCAN DID Varint Decode Boundary Is Closed [r[verification-gaps.close-ucan-did-varint-decode]]
`src/ucan_did.rs::varint_decode` MUST no longer be a trusted or weakened proof
boundary. The implementation MUST remove its `#[verifier::external_body]`
attribute and MUST NOT replace it with `assume(...)`,
`assume_specification[...]`, `@verified-logic:no-ensures`, or a runtime-only
`@verified-logic:exempt` marker.

#### Scenario: Source audit finds no trusted varint decoder bridge

- GIVEN the implementation is complete
- WHEN `src/ucan_did.rs` is audited around `varint_decode`
- THEN `varint_decode` has a normal verified postcondition tying the result to
  `varint_decode_spec`
- AND no trusted or exempt marker appears on that function

#### Scenario: Decoder helper path has no replacement trust

- GIVEN the implementation is complete
- WHEN the `src/ucan_did.rs` decoder helper path is audited, including
  `varint_decode`, bounded decode helpers, and bounded decode lemmas
- THEN none of those items uses `#[verifier::external_body]`, `assume(...)`,
  `assume_specification[...]`, `@verified-logic:no-ensures`, or
  `@verified-logic:exempt`

#### Scenario: Verus proves the decoder

- GIVEN the implementation is complete
- WHEN `./scripts/verus.sh src/ucan_did.rs` and `./scripts/verus.sh src/lib.rs`
  run
- THEN both verifier runs pass without a `varint_decode` external body

### Requirement: Proof-Gap Inventory Reflects The Closed Boundary [r[verification-gaps.inventory-after-ucan-did-varint]]
The repo-wide verification-gap inventory MUST remove the closeable row for
`src/ucan_did.rs::varint_decode` and update all source-derived published totals.
With no unrelated concurrent proof-gap changes, the expected inventory after
this change is `43` total trusted/weakened/runtime-only boundaries: `32`
`#[verifier::external_body]` sites, `3` `assume_specification[...]` bridges,
`8` instruction-selection runtime-only exemptions, and the single
`@verified-logic:no-ensures` marker reported as co-located with an already
counted external body rather than added twice.

#### Scenario: Counts are derived from source

- GIVEN the implementation is complete
- WHEN the count evidence runs `rg` audits for `external_body`,
  `assume_specification[...]`, `@verified-logic:no-ensures`, and
  instruction-selection runtime-only exemptions
- THEN `README.md`, `openspec/specs/verification-gaps/spec.md`, and captured
  evidence publish the same non-double-counted total
- AND the evidence lists any overlapping category sites as non-additive

#### Scenario: Tracey coverage stays green

- GIVEN `docs/requirements.md` and code annotations are updated for this change
  with the new or clarified `r[...]` IDs
- WHEN `tracey query status` starts or confirms the daemon and then
  `tracey query uncovered` and `tracey query untested` run sequentially
- THEN both coverage queries report zero missing coverage items
