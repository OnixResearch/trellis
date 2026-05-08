# Verification Gaps Specification

## Purpose

This specification tracks source-derived proof-gap and trusted-boundary inventory requirements. It preserves the closed UCAN DID varint boundary requirement and records the current audit policy for remaining trusted cryptographic, codec, runtime, and solver-limit bridges.

## Requirements

### Requirement: UCAN DID Varint Decode Boundary Is Closed

`src/ucan_did.rs::varint_decode` MUST no longer be a trusted or weakened proof boundary. The implementation MUST remove its `#[verifier::external_body]` attribute and MUST NOT replace it with `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, or a runtime-only `@verified-logic:exempt` marker.

r[verification-gaps.close-ucan-did-varint-decode]

#### Scenario: Source audit finds no trusted varint decoder bridge

- GIVEN the implementation is complete
- WHEN `src/ucan_did.rs` is audited around `varint_decode`
- THEN `varint_decode` has a normal verified postcondition tying the result to `varint_decode_spec`
- AND no trusted or exempt marker appears on that function

#### Scenario: Decoder helper path has no replacement trust

- GIVEN the implementation is complete
- WHEN the `src/ucan_did.rs` decoder helper path is audited, including `varint_decode`, bounded decode helpers, and bounded decode lemmas
- THEN none of those items uses `#[verifier::external_body]`, `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, or `@verified-logic:exempt`

#### Scenario: Verus proves the decoder

- GIVEN the implementation is complete
- WHEN `./scripts/verus.sh src/ucan_did.rs` and `./scripts/verus.sh src/lib.rs` run
- THEN both verifier runs pass without a `varint_decode` external body

### Requirement: Proof-Gap Inventory Reflects The Closed Boundary

The repo-wide verification-gap inventory MUST remove the closeable row for `src/ucan_did.rs::varint_decode` and update all source-derived published totals. With the UCAN attenuation-chain checker, ability-subsumption checker, principal-link DID extraction bridge, SSA runtime-helper wrappers, instruction-selection public API surfaces, equal-input hash determinism / SHA-256 width facts, recursive `cv_fuel` proof lemmas, log-matching leader-append preservation lemmas, CID multibase round-trip bridge, and web-of-trust attestation/revocation signature-input checks verified, the current expected inventory is `16` total trusted/weakened/runtime-only boundaries: `16` `#[verifier::external_body]` sites, `0` `assume_specification[...]` bridges, and `0` instruction-selection runtime-only exemptions. No `@verified-logic:no-ensures` markers remain.

r[verification-gaps.inventory-after-ucan-did-varint]

#### Scenario: Counts are derived from source

- GIVEN the implementation is complete
- WHEN the count evidence runs `rg` audits for `external_body`, `assume_specification[...]`, `@verified-logic:no-ensures`, and instruction-selection runtime-only exemptions
- THEN `README.md`, `openspec/specs/verification-gaps/spec.md`, and captured evidence publish the same non-double-counted total
- AND the evidence lists any overlapping category sites as non-additive

#### Scenario: Tracey coverage stays green

- GIVEN `docs/requirements.md` and code annotations are updated for this change with the new or clarified `r[...]` IDs
- WHEN `tracey query status` starts or confirms the daemon and then `tracey query uncovered` and `tracey query untested` run sequentially
- THEN both coverage queries report zero missing coverage items

### Requirement: Source-Derived Proof-Gap Boundary Inventory

The system MUST maintain a source-derived inventory of remaining trusted or weakened proof boundaries before any proof-gap closeout attempt updates published counts.

r[verification-gaps.boundary-audit.inventory]

#### Scenario: Inventory lists every trusted boundary

- GIVEN the audit command scans `src/` for `#[verifier::external_body]`
- WHEN proof-gap inventory evidence is captured
- THEN the evidence lists every matching file, function, and total count
- AND it separately reports counts for `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, and runtime-only exemption markers

### Requirement: Permanent And Closeable Boundary Classification

The system MUST classify each remaining proof boundary as permanent trust or a closeable structural candidate before implementation work removes, rewrites, or re-counts the boundary.

r[verification-gaps.boundary-audit.classification]

#### Scenario: Cryptographic runtime trust is not overclaimed

- GIVEN a boundary represents runtime cryptography, digest implementation behavior, codec correctness, or collision resistance
- WHEN the boundary is classified
- THEN the classification records it as permanent trust unless a concrete structural wrapper proof is scoped without claiming cryptographic strength

#### Scenario: Structural candidates include proof plan

- GIVEN a boundary is classified as closeable
- WHEN implementation begins
- THEN the audit records the finite structural property to prove, the touched module, and the focused Verus/Tracey evidence required for closure

### Requirement: Bounded Closeout Slice

The system MUST drain at most one selected closeable boundary family per proof-gap audit closeout slice, and MUST NOT introduce `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, runtime-only exemption markers, or new `#[verifier::external_body]` sites as replacements.

r[verification-gaps.boundary-audit.closeout-slice]

#### Scenario: Failed closeout is reclassified rather than hidden

- GIVEN the selected closeable boundary cannot be closed with a focused proof
- WHEN the closeout attempt is abandoned
- THEN the change records the blocker and classification update
- AND no replacement weakened marker is added

### Requirement: Published Inventory Synchronization

The system MUST synchronize README, requirements, and canonical OpenSpec proof-gap totals from source-derived count evidence whenever the trusted-boundary inventory changes.

r[verification-gaps.boundary-audit.inventory-sync]

#### Scenario: Published counts match source audit

- GIVEN a closeout changes the number of trusted or weakened proof boundaries
- WHEN the change is verified
- THEN README, `docs/requirements.md`, `openspec/specs/verification-gaps/spec.md`, and captured evidence report the same non-double-counted totals
- AND Tracey uncovered and untested queries remain green

### Requirement: Proof-Gap Inventory Drift Gate

The system MUST provide a Rust-script inventory gate that scans source files for trusted or weakened proof-gap markers and fails when live counts drift from the published expected inventory.

r[verification-gaps.inventory-drift-gate]

#### Scenario: Current source inventory passes

- GIVEN the current source tree and expected inventory counts
- WHEN `scripts/check-proof-gap-inventory.rs` scans `src/`
- THEN it reports 16 `#[verifier::external_body]` sites
- AND it reports zero `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, and `@verified-logic:exempt` markers
- AND it exits successfully

#### Scenario: Added trusted marker fails closed

- GIVEN the Rust inventory script runs in its controlled extra-marker fixture mode
- WHEN the simulated live inventory contains one additional `#[verifier::external_body]` marker
- THEN the script exits non-zero
- AND the evidence records the expected-count mismatch without mutating source files

#### Scenario: Full verification includes the drift gate

- GIVEN the repository verification script is run
- WHEN `./scripts/verify.sh` reaches the proof-gap inventory stage
- THEN it invokes the Rust inventory script before cargo, Verus, and Tracey checks claim success

### Requirement: Proof-Gap Site-List Drift Gate

The system MUST validate the exact current `#[verifier::external_body]` site list, not only the aggregate count, so same-count proof-gap substitutions fail closed.

r[verification-gaps.inventory-site-list-drift-gate]

#### Scenario: Current external-body site list passes

- GIVEN the current audited external-body baseline of 16 file/function sites
- WHEN `scripts/check-proof-gap-inventory.rs` scans `src/`
- THEN the script reports the expected 16 sites
- AND it exits successfully when every live site matches the expected file/function list

#### Scenario: Same-count site substitution fails closed

- GIVEN the Rust inventory script runs in a controlled same-count site-substitution fixture mode
- WHEN the simulated live inventory preserves the external-body count but replaces one expected site with an unexpected site
- THEN the script exits non-zero
- AND the evidence records the missing expected site and unexpected simulated site without mutating source files

#### Scenario: Full verification enforces exact sites

- GIVEN the repository verification script is run
- WHEN `./scripts/verify.sh` invokes the proof-gap inventory drift stage
- THEN the Rust checker validates both marker counts and the exact external-body site list before full verification can pass

### Requirement: Proof-Gap Classification Drift Gate

The system MUST validate the classification attached to every expected proof-gap site, so permanent trust and closeable-candidate labels cannot silently drift independently of the exact site list.

r[verification-gaps.inventory-classification-drift-gate]

#### Scenario: Current classifications pass

- GIVEN the current audited 16-site proof-gap baseline with classifications
- WHEN the Rust inventory checker validates classifications
- THEN every live site matches its expected classification and the checker exits successfully

#### Scenario: Classification substitution fails closed

- GIVEN a controlled fixture that preserves the exact site list but changes one classification
- WHEN the Rust checker evaluates classification drift
- THEN it exits non-zero and reports the changed classification without mutating source files

#### Scenario: Full verification enforces classifications

- GIVEN `./scripts/verify.sh` runs
- WHEN it invokes the proof-gap inventory drift stage
- THEN classification drift is checked before full verification can pass
