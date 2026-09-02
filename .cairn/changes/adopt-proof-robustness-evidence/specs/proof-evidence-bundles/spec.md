# Proof Evidence Bundles Specification

## Purpose

Extend Trellis proof evidence with an Octet-owned robustness receipt for selected Verus proofs.

## Requirements

### Requirement: Proof robustness uses the accepted Octet profile

r[trellis.proof_evidence_bundles.proof_robustness.profile]

Trellis MUST consume an immutable accepted Octet proof-robustness package and MUST bind its profile, schema, provider, toolchain, command, and receipt identities.

#### Scenario: Accepted provider contract is available

- GIVEN the selected Octet revision publishes the required profile and receipt contract
- WHEN Trellis preflight validates the dependency
- THEN the pilot MAY execute with the exact selected identities

#### Scenario: Provider contract is absent or different

- GIVEN the Octet package, profile, schema, provider, toolchain, command, or receipt identity is missing or stale
- WHEN Trellis preflight runs
- THEN the pilot MUST report a blocked dependency
- AND it MUST NOT construct a local substitute schema

### Requirement: Pilot corpus covers distinct proof classes

r[trellis.proof_evidence_bundles.proof_robustness.corpus]

The first pilot profile MUST select `src/arithmetic.rs` as a trust-free class, `src/chain.rs` as an accepted-boundary class, and `src/choreography_projection.rs` as a recursive proof-complexity class. Each member MUST retain its exact source and proof scope identity, selection reason, limits, and expected trust mode.

#### Scenario: Every pilot member completes

- GIVEN each selected member passes preflight under its declared trust mode
- WHEN the Octet robustness profile runs
- THEN every member MUST have one current proof receipt and one linked robustness receipt
- AND the pilot summary MUST preserve individual member classifications

#### Scenario: A pilot member cannot run as declared

- GIVEN one selected module cannot run under its declared scope, trust mode, provider, or limit
- WHEN preflight or execution runs
- THEN that member MUST be `blocked`, `unsupported`, or failed with a named reason
- AND the profile MUST NOT silently replace or remove it

### Requirement: Trust modes preserve accepted assumption boundaries

r[trellis.proof_evidence_bundles.proof_robustness.assumptions]

Trust-free pilot members MUST use strict no-cheating mode. Members with accepted trust boundaries MUST use audited-boundary mode and bind every declared boundary to the current proof-gap inventory.

#### Scenario: Trust-free proof remains trust-free

- GIVEN a pilot member declares strict no-cheating mode and has no accepted proof gap
- WHEN proof and audit validation run
- THEN the receipt MUST record strict mode and an empty declared-boundary set

#### Scenario: Trust expands or mode changes

- GIVEN a new source boundary appears, an accepted boundary is missing, strict mode is disabled, or audited mode lacks an exact proof-gap reference
- WHEN audit validation runs
- THEN the pilot member MUST fail closed with a deterministic assumption diagnostic

### Requirement: Verus proof-use data remains partial

r[trellis.proof_evidence_bundles.proof_robustness.proof_use]

Trellis MUST preserve supported Verus `-V axiom-usage-info` output as `axiom-usage-only` proof-use evidence. It MUST NOT claim complete dependency, redundancy, contradiction, or vacuity analysis from that output.

#### Scenario: Axiom-use output is available

- GIVEN Verus emits supported axiom-use facts for a selected member
- WHEN Trellis links proof-use evidence
- THEN the facts MUST retain source, proof, verifier, child-receipt, and robustness-receipt identities
- AND coverage MUST remain `axiom-usage-only`

#### Scenario: Complete dependency status is requested

- GIVEN a consumer requests complete proof dependency or unused-assumption status
- WHEN only Verus axiom-use facts are available
- THEN the request MUST be unsupported
- AND Trellis MUST NOT infer an empty or complete dependency set

### Requirement: Robustness evidence extends the proof evidence chain

r[trellis.proof_evidence_bundles.proof_robustness.handoff]

Each robustness receipt MUST link to the matching Trellis source envelope, proof evidence IR, verifier receipt, property and requirement IDs, assumptions, proof gaps, and non-claims before stack handoff.

#### Scenario: Complete linked handoff validates

- GIVEN all source, proof, verifier, assumption, proof-gap, child-receipt, robustness, and non-claim identities match
- WHEN Trellis validates the proof evidence chain
- THEN the chain MUST preserve the robustness receipt as supplemental recorded evidence

#### Scenario: Robustness receipt is swapped or stale

- GIVEN a robustness receipt refers to another source, proof, verifier, property, assumption set, or child receipt
- WHEN chain validation runs
- THEN validation MUST fail with a deterministic linkage diagnostic

### Requirement: Robustness evidence does not own theorem status

r[trellis.proof_evidence_bundles.proof_robustness.boundary]

A stable proof-robustness classification MUST NOT replace the Verus proof result or support verifier soundness, complete dependency, semantic equivalence, whole-system correctness, or release eligibility claims.

#### Scenario: Stable failed proof is supplied

- GIVEN all seeds produce the same failed proof outcome
- WHEN Trellis classifies the evidence chain
- THEN the robustness result MAY be stable
- AND the theorem MUST remain unverified

#### Scenario: Robustness evidence is promoted

- GIVEN a bundle treats seed stability, resource stability, or axiom-use facts as formal proof
- WHEN claim-boundary validation runs
- THEN validation MUST fail

### Requirement: Proof robustness adoption has adversarial validation

r[trellis.proof_evidence_bundles.proof_robustness.validation]

The pilot MUST include positive and negative coverage for dependency identity, corpus membership, trust modes, assumption linkage, proof-use coverage, child receipts, classifications, limits, handoff roles, and non-claims.

#### Scenario: Pilot fixture matrix runs

- GIVEN clean, accepted-boundary, complex, unstable, stale, malformed, unsupported, and overclaiming fixtures
- WHEN focused pilot validation runs
- THEN every positive fixture MUST pass deterministically
- AND every negative fixture MUST fail at its declared boundary
