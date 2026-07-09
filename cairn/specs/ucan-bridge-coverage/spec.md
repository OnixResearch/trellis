# UCAN Bridge Coverage Specification

## Purpose

Defines Trellis-owned coverage mapping for UCAN verified bridge helpers.

## Requirements

### Requirement: UCAN bridge coverage contract
r[trellis.ucan_bridge_coverage.contract] Trellis MUST support a coverage manifest that maps UCAN verified bridge helpers to Trellis primitive IDs, proof artifacts, evidence tiers, and non-claim boundaries.

#### Scenario: Covered bridge row passes
r[trellis.ucan_bridge_coverage.fixtures.positive]
- GIVEN a coverage row names a UCAN helper, Trellis primitive ID, source/proof identity, vendored path, evidence tier, and required non-claims
- WHEN coverage validation runs
- THEN the row MUST pass and preserve the helper-to-primitive mapping.

#### Scenario: Missing primitive row fails
r[trellis.ucan_bridge_coverage.fixtures.negative]
- GIVEN a UCAN helper row references an unknown Trellis primitive ID, stale vendored path, missing non-claim, or unsupported tier promotion
- WHEN coverage validation runs
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Coverage validation
r[trellis.ucan_bridge_coverage.validation] Trellis MUST validate bridge coverage over parsed Trellis primitive metadata and UCAN vendor metadata without performing UCAN token orchestration.

#### Scenario: Validation reports coverage only
r[trellis.ucan_bridge_coverage.validation.boundary]
- GIVEN a UCAN helper maps to a Trellis primitive
- WHEN validation passes
- THEN the receipt MUST state that bridge coverage does not prove UCAN parser correctness, resolver truth, replay-store durability, or consumer authorization completeness.

### Requirement: Ownership docs
r[trellis.ucan_bridge_coverage.docs] Documentation MUST state that Trellis owns primitive proof identity and UCAN owns runtime bridge orchestration.

#### Scenario: Boundary is visible
r[trellis.ucan_bridge_coverage.docs.boundary]
- GIVEN a reviewer reads the bridge coverage report
- WHEN ownership is described
- THEN it MUST name Trellis as primitive/proof owner and UCAN as token/shell owner.

### Requirement: Final validation
r[trellis.ucan_bridge_coverage.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers coverage health
r[trellis.ucan_bridge_coverage.final_validation.fixtures]
- GIVEN covered, stale, missing-primitive, missing-non-claim, and tier-promotion fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.
