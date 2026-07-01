# documentation-catalog Specification

## Purpose
Defines requirements for keeping README module catalogs aligned with public module declarations.

## Requirements

### Requirement: README Module Catalog Drift Detection
The repository SHALL provide a deterministic check that detects drift between public module declarations in `src/lib.rs` and the README module catalog.

r[documentation-catalog.readme-module-drift]

#### Scenario: Declared module is documented

- GIVEN `src/lib.rs` declares a public module that is not explicitly excluded
- WHEN the module catalog drift checker runs
- THEN the checker requires the README catalog to contain an entry for that module

#### Scenario: Stale README module is rejected

- GIVEN the README catalog names a module that is no longer declared in `src/lib.rs` and is not explicitly exempted
- WHEN the module catalog drift checker runs
- THEN the checker fails with a diagnostic identifying the stale catalog entry

#### Scenario: Intentional exclusions are explicit

- GIVEN a public module is intentionally absent from the README catalog
- WHEN the checker evaluates exclusions
- THEN the exclusion includes a stable module name and rationale rather than being implicit

#### Scenario: Drift checker is locally gated

- GIVEN repository local verification checks run
- WHEN the documentation catalog check is selected
- THEN README/catalog drift is detected without requiring full Verus verification

### Requirement: Contributor Onboarding Guide

The repository MUST provide public contributor onboarding documentation that explains how to add or modify verified logic while preserving requirements coverage, proof obligations, positive and negative tests, and repository validation gates.

r[documentation-catalog.contributor-onboarding]

#### Scenario: New module workflow is documented

- GIVEN a contributor wants to add a verified module
- WHEN they read the contributor guide
- THEN the guide identifies the required requirement ID, spec function, exec function with `ensures`, proof or property evidence, and positive and negative tests

#### Scenario: Core change validation is explicit

- GIVEN a contributor changes core logic or verified library code
- WHEN they read the contributor guide
- THEN the guide tells them to run relevant baseline checks before the change and final checks after the change

#### Scenario: Untracked proof gap is rejected

- GIVEN a contributor introduces a trusted boundary or weakened proof marker without updating the proof-gap inventory
- WHEN repository validation runs
- THEN validation fails or the guide directs the contributor to the required inventory/spec update before completion
