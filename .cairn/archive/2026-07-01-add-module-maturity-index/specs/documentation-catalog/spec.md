## ADDED Requirements

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
