# Verification Gaps Specification

## MODIFIED Requirements

### Requirement: Nix Version Bridge Decomposition
The system MUST provide a bounded structural decomposition of `src/nix_version.rs::compare_versions` that routes admitted finite-model inputs through verified executable comparison helpers while keeping out-of-model inputs on an explicit fallback trusted boundary.

r[verification-gaps.nix-version-bridge-decomposition]

#### Scenario: Finite comparator agrees with modeled cases

- GIVEN admitted finite version byte sequences within the modeled token rules
- WHEN the modeled comparison wrapper compares them
- THEN the result is delegated through the existing compare specification with an explicit admitted-input precondition

#### Scenario: Out-of-model input boundary is explicit

- GIVEN version bytes outside the admitted finite model
- WHEN comparison admission is evaluated
- THEN the wrapper returns no ordering claim instead of pretending the input was verified by the finite model

#### Scenario: Public API narrows the trusted bridge

- GIVEN admitted finite-model inputs within the public `compare_versions` API preconditions
- WHEN `compare_versions` evaluates them
- THEN it uses the verified modeled executable path and the trusted inventory retains only the unmodeled fallback boundary

### Requirement: Nix Version Modeled Compare Bridge
The system MUST compute admitted finite-model Nix version comparisons through verified executable helpers for separator skipping, component bounds, numeric parsing, and lexicographic comparison. The modeled wrapper MUST return an ordering matching `compare_versions_spec` for admitted inputs, return no ordering claim for out-of-model inputs, and keep only the unmodeled fallback external-body trusted boundary visible for the broader API.

r[verification-gaps.nix-version-modeled-compare-bridge]

#### Scenario: Admitted modeled comparison is executable evidence

- GIVEN two admitted finite-model version byte sequences
- WHEN `compare_versions_modeled` evaluates them
- THEN the admitted branch computes through the verified modeled comparator path
- AND the result is `Some(-1)`, `Some(0)`, or `Some(1)` matching `compare_versions_spec`

#### Scenario: Out-of-model input boundary remains explicit

- GIVEN either version byte sequence is outside the admitted finite model
- WHEN `compare_versions_modeled` evaluates admission
- THEN it returns `None` with no ordering claim

#### Scenario: Trusted inventory remains visible

- GIVEN the public `compare_versions` bridge has been narrowed to admitted finite-model correctness
- WHEN proof-gap inventory evidence is captured
- THEN the trusted-boundary count is unchanged and the site list includes `src/nix_version.rs::compare_versions_unmodeled_trusted`
- AND no replacement weakened marker or additional external body is introduced

## ADDED Requirements

### Requirement: Single Trusted Boundary Closure Slice
A trusted-boundary closure change MUST select one tracked proof-gap site or one tightly related site family, justify why it is closeable or narrowable, and update source, inventory, and evidence together.

r[verification-gaps.single-boundary-closure]

#### Scenario: Target is selected from current inventory

- GIVEN the current proof-gap inventory lists trusted boundaries
- WHEN this change begins implementation
- THEN it records the selected target site, current classification, and reason it is suitable for closure or narrowing

#### Scenario: Closure removes or narrows trust honestly

- GIVEN a selected trusted boundary
- WHEN implementation changes are complete
- THEN the source either removes the trusted marker with verified replacement logic or narrows the trusted contract without hiding an equivalent new trust boundary elsewhere

#### Scenario: Inventory matches source

- GIVEN source proof-gap markers and the proof-gap manifest
- WHEN the proof-gap inventory checker runs
- THEN expected counts, site names, and classifications match the source tree

#### Scenario: Verification evidence supports the count

- GIVEN the closure or narrowing is claimed complete
- WHEN focused tests, Verus, Tracey coverage, and full verification run
- THEN the evidence supports the updated trusted-boundary count and no uncovered or untested requirements remain
