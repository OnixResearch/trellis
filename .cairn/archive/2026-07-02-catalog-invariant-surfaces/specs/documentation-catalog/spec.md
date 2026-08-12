## ADDED Requirements

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
