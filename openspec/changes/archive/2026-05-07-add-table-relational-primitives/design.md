## Context

The umbrella already includes storage/index, data-validation, serialization, and
constraint-solver families. Table/relational data needs a separate finite model
so future proofs can reason about row/column shape, keys, projections, filters,
joins, and grouping without inheriting database-engine or SQL semantics.

## Goals / Non-Goals

**Goals:**

- specify exact finite table/schema, column, row, and key metadata
- specify deterministic projection/filter, join, and grouping shape envelopes
- specify deterministic relational admission/rejection without implementation yet

**Non-Goals:**

- implementing runtime APIs in this spec-only change
- proving SQL conformance, database storage, transactions, isolation,
  concurrency, optimizer behavior, or external database correctness
- proving semantic equivalence of arbitrary relational algebra expressions
- claiming performance, cost, durability, or distributed database guarantees

## Decisions

### 1. Model relation data as finite envelopes

**Choice:** Table/relational primitives use finite schemas, rows, cells, key
metadata, and operation-shape envelopes rather than SQL or database execution.

**Rationale:** Finite envelopes compose with data-validation, storage/index, and
serialization primitives and keep proof obligations local.

### 2. Keep operation primitives shape-oriented

**Choice:** Projection, filter, join, and grouping primitives record admitted
operation shapes and metadata; they do not claim optimizer behavior or arbitrary
query semantics.

**Rationale:** Shape admission is a useful exact primitive that avoids complex
relational algebra completeness claims.

## Risks / Trade-offs

**Overlap with storage/index** → This child owns logical table/row/operation
shape; storage/index owns physical page, slot, cursor, segment, and version
metadata.

**SQL drift** → Requirements explicitly avoid SQL conformance, optimizer,
transaction, and engine claims unless a later focused model owns them.

## Validation Plan

- `openspec validate add-table-relational-primitives --strict --json`
- strict validation for every active primitive-family OpenSpec
- helper verification is expected to warn about future implementation tasks
- future implementation must add `docs/requirements.md`, `r[impl]`,
  `r[depends]`, `r[verify]`, tests, Verus proof evidence, and Tracey evidence
