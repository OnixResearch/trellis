## ADDED Requirements

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
