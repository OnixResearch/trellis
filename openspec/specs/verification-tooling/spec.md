# verification-tooling Specification

## Purpose
Defines deterministic repository verification tooling and fixture-backed regression coverage for checker scripts.
## Requirements
### Requirement: Checker Fixture Regressions
Verification checker scripts MUST have fixture-backed self-tests that cover both accepted inputs and representative rejected inputs for their enforced policy.

r[verification-tooling.checker-fixture-regressions]

#### Scenario: Valid fixtures pass

- GIVEN fixture source files that satisfy coverage, ensures, and iterative-exec policies
- WHEN the checker self-test harness runs
- THEN each checker accepts its valid fixture

#### Scenario: Missing coverage annotation is rejected

- GIVEN a fixture with a non-test public exec function missing an `r[impl]` annotation
- WHEN the coverage checker is run through the self-test harness
- THEN the harness observes a failing exit status and a diagnostic that identifies the missing annotation

#### Scenario: Missing Verus ensures clause is rejected

- GIVEN a fixture with a public exec function inside a `verus!` block that lacks an `ensures` clause
- WHEN the ensures checker is run through the self-test harness
- THEN the harness observes a failing exit status and a diagnostic that identifies the missing `ensures`

#### Scenario: Iterative exec recursion is rejected

- GIVEN a fixture with a self-edge, mutual cycle, or disallowed qualified intra-module exec call
- WHEN the iterative-exec checker is run through the self-test harness
- THEN the harness observes a failing exit status and a diagnostic that identifies the rejected graph shape

#### Scenario: Fixture checks are part of Nix verification

- GIVEN the repository's flake checks are evaluated
- WHEN the checker self-test check is selected
- THEN the fixture-backed self-tests run deterministically without requiring network access or the full repository test suite
