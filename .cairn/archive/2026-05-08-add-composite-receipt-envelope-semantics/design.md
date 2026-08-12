## Context

The primitive stack now includes serialization, table, storage, graph-query, and proof-gap evidence gates. A finite receipt/envelope layer would make evidence artifacts and validation windows compositional instead of ad hoc.

## Goals

- Specify bounded receipt headers, artifact references, validation windows, and result/evidence envelopes.
- Compose serialization-codec, storage-index, table-relational, and graph-query predicates.
- Keep scope as finite receipt admission, not external storage or CI semantics.

## Non-Goals

- Runtime artifact storage guarantees.
- External CI/provider conformance.
- Cryptographic strength of hashes beyond existing trusted boundaries.

## Decisions

### Bounded finite semantics

The change MUST stay finite, deterministic, and mechanically checkable. Runtime/product claims and external-system conformance are excluded unless a checked predicate in this repo proves the property.

### Evidence boundary

Implementation tasks MUST capture positive and negative evidence before any task is marked complete. The final task remains open until sync, archive, commit, push, and clean-state verification are done.
