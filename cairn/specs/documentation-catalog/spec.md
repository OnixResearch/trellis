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

### Requirement: Intent-Oriented Module Guide

The repository MUST provide a user-facing module guide that maps common implementation and modeling goals to the relevant Trellis modules, examples, and requirement anchors.

r[documentation-catalog.intent-module-guide]

#### Scenario: User goal maps to a starting module

- GIVEN a documented user goal such as bounded arithmetic, quorum admission, authorization, memory layout, or compiler analysis
- WHEN a user reads the module guide
- THEN the guide identifies at least one starting module and explains why it is relevant

#### Scenario: Caveats are visible

- GIVEN a guide entry for a model-only, facade, or trusted-boundary-adjacent surface
- WHEN the entry is rendered in the guide
- THEN the entry states the caveat before directing users to depend on the API

#### Scenario: Stale module reference is rejected

- GIVEN the guide references a module that is no longer declared in `src/lib.rs`
- WHEN documentation catalog validation runs
- THEN validation fails with a diagnostic identifying the stale reference

### Requirement: Cookbook Recipes

The repository MUST provide cookbook documentation that describes common user and contributor workflows with clear success criteria, validation commands, and failure-mode guidance.

r[documentation-catalog.cookbook-recipes]

#### Scenario: Recipe states observable success

- GIVEN a cookbook recipe for using or adding Trellis logic
- WHEN a reader follows the recipe
- THEN the recipe states the expected output, validation command, or file change that demonstrates completion

#### Scenario: Recipe includes a negative path

- GIVEN a recipe that introduces tests, requirements, examples, or validation checks
- WHEN the recipe describes its workflow
- THEN it includes at least one representative failure mode or rejection case

#### Scenario: Checked snippet remains current

- GIVEN a cookbook snippet is marked executable or normative
- WHEN cookbook validation runs
- THEN the snippet's referenced files and commands are checked or the validation fails with a diagnostic
