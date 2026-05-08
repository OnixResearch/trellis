## Context

Graph-query composition and storage/table primitives are drained, but bounded result windows and continuation cursors are not yet modeled as a compositional query facade.

## Goals

- Specify finite page/window result semantics over index-backed and scan-backed query results.
- Model continuation cursor admission, monotone progress, and duplicate exclusion.
- Compose storage-index, table-relational, and graph-query result predicates.

## Non-Goals

- Database execution engine behavior.
- Performance, latency, or storage-layout claims.
- External query language conformance.

## Decisions

### Bounded finite semantics

The change MUST stay finite, deterministic, and mechanically checkable. Runtime/product claims and external-system conformance are excluded unless a checked predicate in this repo proves the property.

### Evidence boundary

Implementation tasks MUST capture positive and negative evidence before any task is marked complete. The final task remains open until sync, archive, commit, push, and clean-state verification are done.
