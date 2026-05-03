## Why

Relational data handling often mixes table shape, column metadata, row identity,
keys, projections, filters, joins, grouping, and database-engine behavior. A
focused table/relational primitive family gives future changes exact finite
building blocks for tables, columns, rows, keys, joins, projections, filters,
grouping metadata, and deterministic relational admission without claiming SQL
conformance, database storage, transactions, indexes, query planning, or query
optimizer correctness.

## What Changes

- Add a spec-only `add-table-relational-primitives` child OpenSpec under the
  domain-primitives umbrella.
- Define exact finite models for table schemas, column metadata, row envelopes,
  key constraints, projection/filter shapes, join envelopes, grouping metadata,
  and deterministic relational admission.
- Leave implementation, Verus proofs, Tracey coverage, and evidence as future
  tasks.

## Capabilities

### New Capabilities

- `domain-primitives.table-relational.table-schema`: finite table/schema
  metadata envelopes.
- `domain-primitives.table-relational.column`: finite column metadata and type
  admission envelopes.
- `domain-primitives.table-relational.row`: finite row shape and cell-presence
  envelopes.
- `domain-primitives.table-relational.key`: finite key/uniqueness metadata.
- `domain-primitives.table-relational.projection-filter`: deterministic
  projection and filter-shape envelopes.
- `domain-primitives.table-relational.join`: finite join-shape metadata.
- `domain-primitives.table-relational.grouping`: finite grouping/aggregate-shape
  metadata.
- `domain-primitives.table-relational.admission`: deterministic relational
  admission/rejection envelopes.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add
  `src/table_*`, `src/relation_*`, `src/row_*`, `docs/requirements.md`, tests,
  and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new dependencies.
- **Testing**: strict OpenSpec validation now; future implementation must run
  focused tests, Verus, Tracey, and final repo validation.

## Out of Scope

- implementing table/relational modules now
- database engine behavior, SQL conformance, query planner/optimizer behavior,
  indexes, storage layout, transactions, isolation, durability, concurrency, or
  distributed database behavior
- external schema registry correctness, runtime ORM behavior, business-rule
  completeness, semantic query equivalence, performance, cost modeling, or
  materialized-view freshness
