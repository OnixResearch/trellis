## MODIFIED Requirements

### Requirement: Nix Version Modeled Compare Bridge

The system MUST compute admitted finite-model Nix version comparisons through verified executable helpers for separator skipping, component bounds, numeric parsing, component comparison, and lexicographic comparison. The modeled wrapper MUST return an ordering matching `compare_versions_spec` for admitted inputs, return no ordering claim for out-of-model inputs, and keep the full `compare_versions` external-body trusted boundary visible when the broader API remains trusted.

r[verification-gaps.nix-version-modeled-compare-bridge]

#### Scenario: Admitted modeled comparison is executable evidence

- GIVEN two admitted finite-model version byte sequences within the documented bounds
- WHEN `compare_versions_modeled` evaluates them
- THEN the admitted branch computes through verified executable helper paths for separator skipping, component bounds, numeric parsing, and lexicographic/component comparison
- AND the result is `Some(-1)`, `Some(0)`, or `Some(1)` matching `compare_versions_spec`

#### Scenario: Out-of-model input boundary remains explicit

- GIVEN either version byte sequence is outside the admitted finite model or configured bounds
- WHEN `compare_versions_modeled` evaluates admission
- THEN it returns `None` with no ordering claim
- AND no fallback trusted comparison result is exposed as verified evidence

#### Scenario: Trusted inventory remains visible

- GIVEN the full `compare_versions` exec/spec bridge remains trusted after this slice
- WHEN proof-gap inventory evidence is captured
- THEN the existing trusted-boundary count and site list still include `src/nix_version.rs::compare_versions`
- AND no replacement weakened marker or new external body is introduced
