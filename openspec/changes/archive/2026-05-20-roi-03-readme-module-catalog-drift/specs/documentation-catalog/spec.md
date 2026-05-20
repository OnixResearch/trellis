# Documentation Catalog Specification

## ADDED Requirements

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
