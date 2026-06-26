## ADDED Requirements

### Requirement: Nix Version Bridge Decomposition

The system MUST attempt a bounded structural decomposition of `src/nix_version.rs::compare_versions` that either narrows/removes the trusted bridge with verified finite comparison helpers or records why the bridge remains trusted without adding replacement weakened markers.

r[verification-gaps.nix-version-bridge-decomposition]

#### Scenario: Finite comparator agrees with modeled cases

- GIVEN admitted finite version byte sequences within the modeled token rules
- WHEN the verified comparator model compares them
- THEN the result matches the specified ordering relation

#### Scenario: Out-of-model input boundary is explicit

- GIVEN version bytes outside the admitted finite model
- WHEN comparison admission is evaluated
- THEN the boundary is rejected or delegated explicitly without claiming verified ordering

#### Scenario: Failed closeout preserves trust inventory

- GIVEN the bridge cannot be safely removed in this slice
- WHEN the decomposition attempt is closed
- THEN evidence records the blocker and no `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, `@verified-logic:exempt`, or new external body is introduced
