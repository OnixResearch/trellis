## MODIFIED Requirements

### Requirement: Nix Version Bridge Decomposition

The system MUST provide a bounded structural decomposition of `src/nix_version.rs::compare_versions` that narrows verified claims to an explicit finite admission wrapper. For admitted finite-model inputs, the modeled wrapper MUST compute the existing `compare_versions_spec` through verified executable logic rather than delegating the admitted case to the trusted `compare_versions` external body. The full `compare_versions` bridge MAY remain trusted for out-of-model or broader API use, but this slice MUST NOT add `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, `@verified-logic:exempt`, or new `#[verifier::external_body]` markers.

r[verification-gaps.nix-version-modeled-compare-bridge]

#### Scenario: Admitted modeled comparison is executable evidence

- GIVEN two version byte sequences admitted by the finite modeled token rules
- WHEN the modeled comparator evaluates them
- THEN it returns `Some(-1)`, `Some(0)`, or `Some(1)` matching `compare_versions_spec`
- AND the admitted branch does not call the trusted `compare_versions` external-body function

#### Scenario: Out-of-model input boundary remains explicit

- GIVEN either version byte sequence is outside the admitted finite model
- WHEN the modeled comparator evaluates admission
- THEN it returns `None` with no ordering claim

#### Scenario: Trusted inventory is not hidden

- GIVEN the full `compare_versions` exec/spec bridge remains trusted after this slice
- WHEN proof-gap inventory evidence is captured
- THEN the existing trusted-boundary count and site list still include `src/nix_version.rs::compare_versions`
- AND no replacement weakened marker or new external body is introduced
