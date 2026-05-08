## Why

The primitive stack now includes serialization, table, storage, graph-query, and proof-gap evidence gates. A finite receipt/envelope layer would make evidence artifacts and validation windows compositional instead of ad hoc.

## What Changes

- Specify bounded receipt headers, artifact references, validation windows, and result/evidence envelopes.
- Compose serialization-codec, storage-index, table-relational, and graph-query predicates.
- Keep scope as finite receipt admission, not external storage or CI semantics.

## Scope

In scope: the requirement and task set in this change.

Out of scope:
- Runtime artifact storage guarantees.
- External CI/provider conformance.
- Cryptographic strength of hashes beyond existing trusted boundaries.

## Verification

Validate the active OpenSpec, implement only after the spec-foundation commit, capture evidence under this change, then sync/archive when all implementation tasks are complete.
