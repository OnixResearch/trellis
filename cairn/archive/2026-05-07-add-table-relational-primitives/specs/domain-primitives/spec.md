# Table Relational Primitives Specification

## Purpose

Defines spec-only requirements for reusable finite table/relational primitives.
These primitives MUST use exact finite table/schema metadata, column metadata,
row shape envelopes, key/uniqueness metadata, projection/filter shape envelopes,
join-shape metadata, grouping/aggregate-shape metadata, and deterministic
relational admission/rejection. They MUST avoid database engine behavior, SQL
conformance, query planning or optimization, indexes, storage layout,
transactions, isolation, durability, concurrency, distributed database behavior,
ORM behavior, semantic query equivalence, performance, or external database
correctness unless a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Table Schema Metadata [r[domain-primitives.table-relational.table-schema]]
The system MUST provide finite table/schema metadata envelopes that record table
identifiers, declared columns, row-count bounds, schema-version metadata, and
deterministic table admission results.

#### Scenario: Unknown table schema is rejected explicitly [r[domain-primitives.table-relational.table-schema.scenario.unknown-table-schema-is-rejected-explicitly]]

- GIVEN finite table/schema metadata and a declared schema registry envelope
- WHEN a candidate table references an unknown schema identifier
- THEN the table-schema primitive returns explicit rejection metadata without consulting a database engine

### Requirement: Column Metadata Envelope [r[domain-primitives.table-relational.column]]
The system MUST provide finite column metadata envelopes for column identifiers,
declared value envelopes, nullability or presence policy, ordering position, and
deterministic column admission results.

#### Scenario: Column type mismatch is rejected before row admission [r[domain-primitives.table-relational.column.scenario.column-type-mismatch-is-rejected-before-row-admission]]

- GIVEN finite column metadata and a candidate cell value envelope
- WHEN the value envelope does not match the declared column envelope
- THEN the column primitive rejects the cell before row admission is composed

### Requirement: Row Shape Envelope [r[domain-primitives.table-relational.row]]
The system MUST provide finite row shape envelopes that record row identifiers,
cell positions, required/optional cell presence, and deterministic row admission
results.

#### Scenario: Missing required cell is reported [r[domain-primitives.table-relational.row.scenario.missing-required-cell-is-reported]]

- GIVEN finite row shape metadata and required column positions
- WHEN a required cell is absent
- THEN the row primitive returns explicit missing-cell metadata

### Requirement: Key Metadata Envelope [r[domain-primitives.table-relational.key]]
The system MUST provide finite key metadata envelopes for primary-key-like,
foreign-key-like, compound-key, and uniqueness predicates over declared row and
column metadata without claiming database enforcement.

#### Scenario: Duplicate finite key is rejected [r[domain-primitives.table-relational.key.scenario.duplicate-finite-key-is-rejected]]

- GIVEN finite key metadata and admitted row envelopes
- WHEN two rows violate a declared finite uniqueness predicate
- THEN the key primitive returns explicit duplicate-key metadata without claiming database uniqueness enforcement

### Requirement: Projection Filter Shape [r[domain-primitives.table-relational.projection-filter]]
The system MUST provide deterministic projection/filter shape envelopes over
finite column sets, row predicates, and admitted table metadata.

#### Scenario: Projection references admitted columns only [r[domain-primitives.table-relational.projection-filter.scenario.projection-references-admitted-columns-only]]

- GIVEN finite table metadata and a projection envelope
- WHEN the projection references a column outside the admitted schema
- THEN the projection/filter primitive rejects the operation shape

### Requirement: Join Shape Metadata [r[domain-primitives.table-relational.join]]
The system MUST provide finite join-shape metadata for declared input tables,
join-key metadata, cardinality bounds, unmatched-row policy, and deterministic
join-shape admission results.

#### Scenario: Join key must be present in both inputs [r[domain-primitives.table-relational.join.scenario.join-key-must-be-present-in-both-inputs]]

- GIVEN finite join metadata and two admitted table schemas
- WHEN a declared join key is absent from either input schema
- THEN the join primitive returns explicit join-key rejection metadata

### Requirement: Grouping Metadata Envelope [r[domain-primitives.table-relational.grouping]]
The system MUST provide finite grouping/aggregate-shape metadata for group keys,
aggregate input columns, aggregate result metadata, and deterministic grouping
admission without claiming aggregate numeric accuracy beyond declared exact
models.

#### Scenario: Grouping references admitted columns only [r[domain-primitives.table-relational.grouping.scenario.grouping-references-admitted-columns-only]]

- GIVEN finite grouping metadata and an admitted table schema
- WHEN a group key or aggregate input references an unknown column
- THEN the grouping primitive returns explicit unknown-column rejection metadata

### Requirement: Relational Admission Envelope [r[domain-primitives.table-relational.admission]]
The system MUST provide deterministic relational admission/rejection envelopes
that compose table/schema, column, row, key, projection/filter, join, and grouping
metadata over finite relational data.

#### Scenario: Admission requires all finite relational checks [r[domain-primitives.table-relational.admission.scenario.admission-requires-all-finite-relational-checks]]

- GIVEN finite relational metadata and candidate table or operation shape
- WHEN any schema, column, row, key, projection/filter, join, or grouping check rejects
- THEN the relational-admission primitive returns an explicit rejection before any database engine, SQL, transaction, or query-planner claim is made
