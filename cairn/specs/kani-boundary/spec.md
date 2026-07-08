# Kani Boundary Specification

## Purpose

Defines the `kani-boundary` capability.

## Requirements

### Requirement: Kani as external Trellis evidence
r[trellis.kani_boundary.external_evidence] Trellis MAY reference Kani receipts as complementary external evidence for runtime wrappers, panic-freedom, and boundary regressions.

#### Scenario: Runtime-wrapper Kani evidence is referenced
r[trellis.kani_boundary.external_evidence.wrapper]
- GIVEN a runtime wrapper has a scoped Kani receipt for panic-freedom or boundary behavior
- WHEN Trellis documentation references that receipt
- THEN the reference MAY be recorded as complementary external evidence.

#### Scenario: Kani evidence is not required by default
r[trellis.kani_boundary.external_evidence.optional]
- GIVEN a Trellis module has no Kani receipt
- WHEN the default Trellis verification path runs
- THEN the absence of Kani evidence MUST NOT by itself fail Verus verification.

### Requirement: Verus remains Trellis proof authority
r[trellis.kani_boundary.verus_authority] Trellis MUST keep Verus proof artifacts as the authority for verified-logic claims and MUST NOT treat Kani receipts as replacements for Verus proofs.

#### Scenario: Verus proof claim requires Verus artifact
r[trellis.kani_boundary.verus_authority.proof]
- GIVEN a requirement claims a Trellis theorem or `ensures` obligation is verified
- WHEN the evidence is reviewed
- THEN the supporting artifact MUST be a Verus proof or accepted Trellis proof artifact rather than only a Kani receipt.

#### Scenario: Kani cannot close a trusted boundary by itself
r[trellis.kani_boundary.verus_authority.trust_boundary]
- GIVEN a trusted-boundary closure requires a Verus proof artifact
- WHEN a Kani receipt is presented as the only evidence
- THEN Trellis MUST reject the closure claim.

### Requirement: Kani reference scope identity
r[trellis.kani_boundary.scope_identity] Trellis Kani references MUST name source scope, receipt identity, property class, and non-claims.

#### Scenario: Complete Kani reference passes
r[trellis.kani_boundary.scope_identity.complete]
- GIVEN a Kani reference includes source scope, receipt identity, property class, and non-claims
- WHEN Trellis validates the reference
- THEN the reference MUST remain reviewable as external evidence.

#### Scenario: Missing non-claims fail
r[trellis.kani_boundary.scope_identity.missing]
- GIVEN a Kani reference omits non-claims
- WHEN Trellis validates the reference
- THEN validation MUST fail or mark the reference incomplete.

### Requirement: Kani boundary positive examples
r[trellis.kani_boundary.positive_examples] Trellis SHOULD include positive examples showing Kani receipts attached to runtime-wrapper or boundary evidence without replacing Verus proof.

#### Scenario: Positive example preserves roles
r[trellis.kani_boundary.positive_examples.valid]
- GIVEN a documentation example cites a Kani receipt and a Verus proof artifact
- WHEN the example is reviewed
- THEN the example SHOULD distinguish the Kani external-evidence role from the Verus proof role.

### Requirement: Kani boundary negative examples
r[trellis.kani_boundary.negative_examples] Trellis SHOULD include negative examples or checks for Kani-as-Verus-proof promotion and whole-module overclaims.

#### Scenario: Kani-as-proof promotion is rejected
r[trellis.kani_boundary.negative_examples.promotion]
- GIVEN an evidence reference treats a Kani receipt as a Verus proof artifact
- WHEN validation runs
- THEN validation SHOULD reject or flag the promotion.

#### Scenario: Whole-module overclaim is rejected
r[trellis.kani_boundary.negative_examples.whole_module]
- GIVEN a scoped Kani receipt is described as proving an entire Trellis module
- WHEN validation runs
- THEN validation SHOULD reject or flag the overclaim.

### Requirement: Kani consumer role documentation
r[trellis.kani_boundary.consumer_docs] Trellis documentation MUST explain how Octet and Valence should consume Trellis proof artifacts separately from Kani receipts.

#### Scenario: Consumer docs separate roles
r[trellis.kani_boundary.consumer_docs.visible]
- GIVEN a consumer reads Trellis evidence guidance
- WHEN Kani and Verus artifacts are described
- THEN the guidance MUST state that Kani receipts are external runtime evidence and Verus artifacts carry Trellis proof claims.
