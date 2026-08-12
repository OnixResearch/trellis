# Upstream Verus corpus delta

## ADDED Requirements

### Requirement: Compatibility repairs preserve proof authority [r[trellis.upstream_verus_corpus.compatibility_repairs]]

A verifier-upgrade repair MUST preserve the checked theorem, source identity, and trusted-boundary inventory. Adaptations to pinned upstream sources MUST be separately identified and hashed, MUST execute the native candidate verifier boundary, and MUST NOT be reported as proof of the unmodified upstream revision.

#### Scenario: Local proof repair closes under both verifiers

- GIVEN the reviewed Trellis verifier accepts a theorem and the candidate rejects it
- WHEN the proof is repaired without changing its requires, ensures, source semantics, triggers, or trust markers
- THEN both complete-crate verifier runs MUST pass
- AND the candidate pass MAY replace only that blocker in the upgrade receipt

#### Scenario: Adapted upstream source passes

- GIVEN a pinned upstream source requires a compatibility adaptation
- WHEN the adapted source completes its native candidate proof command
- THEN the receipt MUST bind both the pinned source and adaptation identities
- AND MUST scope the result to the adapted source
- AND MUST NOT transfer the original upstream proof claim or authorize production adoption

#### Scenario: Proven provider revision is not yet published

- GIVEN every complete-crate and required-native proof passes against an exact committed local Octet revision
- WHEN that revision is not available from the durable declared flake source
- THEN the receipt MAY mark compatibility promotion eligible
- BUT the shipped Trellis verifier pin MUST remain unchanged
- AND the local Git override MUST NOT be committed as a product dependency

#### Scenario: Any required repair remains blocked

- GIVEN any complete-crate or required-native proof does not pass
- WHEN the decision is reduced
- THEN promotion MUST remain false
- AND the current verifier pin and Octet compatibility status MUST remain unchanged
