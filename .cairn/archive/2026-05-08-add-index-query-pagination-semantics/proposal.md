## Why

Graph-query composition and storage/table primitives are drained, but bounded result windows and continuation cursors are not yet modeled as a compositional query facade.

## What Changes

- Specify finite page/window result semantics over index-backed and scan-backed query results.
- Model continuation cursor admission, monotone progress, and duplicate exclusion.
- Compose storage-index, table-relational, and graph-query result predicates.

## Scope

In scope: the requirement and task set in this change.

Out of scope:
- Database execution engine behavior.
- Performance, latency, or storage-layout claims.
- External query language conformance.

## Verification

Validate the active OpenSpec, implement only after the spec-foundation commit, capture evidence under this change, then sync/archive when all implementation tasks are complete.
