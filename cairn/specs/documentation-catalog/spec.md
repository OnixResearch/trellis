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

### Requirement: Runnable Example Coverage

The repository MUST provide checked runnable examples that demonstrate representative stable public APIs without requiring users to invoke Verus directly.

r[documentation-catalog.runnable-examples]

#### Scenario: Example compiles through normal cargo

- GIVEN a documented example under `examples/`
- WHEN the example validation command runs
- THEN the example compiles with normal cargo and does not require ghost/proof execution at runtime

#### Scenario: Admission example shows rejection

- GIVEN a public API whose useful behavior includes rejecting invalid input
- WHEN a representative example covers that API
- THEN the example or its companion test demonstrates both an accepted input and a rejected input

#### Scenario: README links remain accurate

- GIVEN README or guide text references a runnable example
- WHEN documentation catalog validation runs
- THEN every referenced example path exists and is part of the example validation set

### Requirement: Module Maturity Index

The repository MUST provide a deterministic module maturity index that exposes objective local facts about each public module's usage surface, evidence coverage, examples, and trusted-boundary adjacency.

r[documentation-catalog.module-maturity-index]

#### Scenario: Public module appears in index

- GIVEN `src/lib.rs` declares a public module that is not explicitly excluded
- WHEN the module maturity index checker runs
- THEN the index contains exactly one row for that module with its objective source facts

#### Scenario: Trusted-boundary adjacency is visible

- GIVEN a module owns or is adjacent to a tracked trusted/proof-gap boundary
- WHEN the module appears in the maturity index
- THEN the row marks that adjacency and links to the proof-gap inventory source

#### Scenario: Stale maturity row is rejected

- GIVEN the index contains a row for a module that is no longer public or has inconsistent source facts
- WHEN the maturity index checker runs
- THEN validation fails with a diagnostic identifying the stale or inconsistent row

### Requirement: Invariant Surface Catalog

The repository MUST provide a deterministic invariant-surface catalog that exposes objective local facts for public invariant predicates, well-formedness checks, preservation lemmas, soundness lemmas, admission-boundary predicates, evidence kind, and trusted-boundary adjacency.

r[documentation-catalog.invariant-surface-catalog]

#### Scenario: Invariant predicate appears in catalog

- GIVEN a public module defines or exports an invariant, well-formedness, preservation, soundness, or admission-boundary surface with a Tracey requirement marker
- WHEN the invariant catalog checker runs
- THEN the catalog contains exactly one row for that surface with its requirement ID, source predicate or proof artifact, module path, evidence kind, and classification label

#### Scenario: Test-only evidence is not overclaimed

- GIVEN an invariant surface has unit or property tests but no matching Verus proof artifact
- WHEN the catalog is rendered
- THEN the row distinguishes test evidence from proof evidence and does not claim the surface is proven

#### Scenario: Trusted-boundary adjacency remains visible

- GIVEN an invariant surface depends on, owns, or sits adjacent to a tracked trusted boundary
- WHEN the catalog is rendered
- THEN the row marks the trusted-boundary adjacency and identifies the local inventory source used for that fact

#### Scenario: Stale invariant row is rejected

- GIVEN the catalog contains a row for a requirement, predicate, or proof artifact that no longer exists in the local source facts
- WHEN the invariant catalog checker runs
- THEN validation fails with a diagnostic identifying the stale row
