## Coverage Index

| Requirement ID | Future module focus |
|---|---|
| `domain-primitives.table-relational.table-schema` | `src/table_*`, `src/relation_*` finite table/schema metadata |
| `domain-primitives.table-relational.column` | column metadata and type admission |
| `domain-primitives.table-relational.row` | row shape and cell-presence envelopes |
| `domain-primitives.table-relational.key` | key/uniqueness metadata |
| `domain-primitives.table-relational.projection-filter` | projection and filter shape envelopes |
| `domain-primitives.table-relational.join` | finite join-shape metadata |
| `domain-primitives.table-relational.grouping` | grouping and aggregate-shape metadata |
| `domain-primitives.table-relational.admission` | deterministic relational admission/rejection |

## Phase 1: Spec foundation

- [x] Create spec-only table/relational primitive requirements and umbrella taxonomy entry. `[covers=domain-primitives.table-relational.table-schema,domain-primitives.table-relational.column,domain-primitives.table-relational.row,domain-primitives.table-relational.key,domain-primitives.table-relational.projection-filter,domain-primitives.table-relational.join,domain-primitives.table-relational.grouping,domain-primitives.table-relational.admission]`

## Phase 2: Future implementation

- [x] Add docs/requirements entries and Tracey IDs for finite table/schema metadata.
- [x] Implement and verify column metadata and type admission primitives.
- [x] Implement and verify row shape and cell-presence envelopes.
- [x] Implement and verify finite key/uniqueness metadata.
- [x] Implement and verify projection/filter shape envelopes.
- [x] Implement and verify finite join-shape metadata.
- [x] Implement and verify grouping and aggregate-shape metadata.
- [x] Implement deterministic relational admission/rejection composition.
- [x] Add focused positive and negative tests for each table/relational primitive.
- [x] Run focused Verus, Tracey uncovered/untested, and strict OpenSpec validation.
- [x] Capture reviewable evidence and archive the change after all implementation tasks complete.
