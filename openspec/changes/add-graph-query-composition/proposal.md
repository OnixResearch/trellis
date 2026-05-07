## Why

The domain-primitive drain added finite knowledge-graph, table/relational, storage-index, parsing, serialization, and text/string primitives. The next useful layer is a bounded composition contract that proves graph-query semantics by connecting those primitive families instead of adding another isolated primitive family.

## What Changes

- Add a composite-primitives OpenSpec for bounded graph query semantics.
- Define path-witness, triple-pattern, join/filter, index-lookup equivalence, and serialized query/result envelope requirements.
- Keep the surface finite and structural: no SPARQL conformance, query optimizer, persistence engine, distributed query execution, inference engine, or external graph database claims.

## Capabilities

### New Capabilities

- `composite-primitives.graph-query-composition`: bounded graph pattern/query admission over finite knowledge-graph facts and primitive relational/index/serialization surfaces.

### Modified Capabilities

- `composite-primitives`: gains a new active composite family above the already archived primitive modules.

## Impact

- **Files:** active OpenSpec package under `openspec/changes/add-graph-query-composition/`.
- **APIs:** future implementation is expected to add a focused `src/graph_query_composition.rs` module and `src/lib.rs` export.
- **Dependencies:** no new third-party dependencies are expected.
- **Testing:** validate the OpenSpec now; future drain runs focused Rust tests, Verus, Tracey prefix checks, and `./scripts/verify.sh`.
