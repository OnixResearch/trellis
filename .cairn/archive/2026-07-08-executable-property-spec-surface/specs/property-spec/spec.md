## ADDED Requirements

### Requirement: Property specification index
r[trellis.property_spec] The repository MUST maintain a `docs/property-spec-index.md` file indexing property groups with source modules, property kinds, and approximate counts.

#### Scenario: Index covers major groups
r[trellis.property_spec.coverage]
- GIVEN `docs/property-spec-index.md`
- WHEN the indexed groups are reviewed
- THEN they include arithmetic, raft, concurrency, finance, memory, storage, cross-module, and R1CS

#### Scenario: Index references scope and gap docs
r[trellis.property_spec.references]
- GIVEN `docs/property-spec-index.md`
- WHEN the non-claims section is reviewed
- THEN it references `docs/module-scope.md` and `docs/proof-gap-sites.tsv`
