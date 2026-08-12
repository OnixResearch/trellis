## Why

The primitive taxonomy now includes finite data structures, but many verified
systems also need reusable graph primitives: adjacency admission, reachability,
topological ordering, component partitions, shortest-path envelopes, and bounded
cut/frontier reasoning. These are common across dependency graphs, workflow
schedulers, GIS routing/topology, ML computation graphs, and resource graphs.

This change adds a focused implemented graph primitive package. It uses exact
finite graph models first and avoids probabilistic, continuous, heuristic, or
algorithmic-optimality claims that would require separate cost/model specs.

## What Changes

- Add a focused `add-graph-primitives` OpenSpec for reusable finite graph modules:
  - `graph_adjacency`: node/edge admission, endpoint bounds, and adjacency lookup.
  - `graph_reachability`: bounded/existential reachability and path witnesses.
  - `graph_topological_order`: DAG admission and edge-forward ordering.
  - `graph_component_partition`: undirected component partition validity.
  - `graph_shortest_path_envelope`: bounded non-negative path-cost envelopes.
  - `graph_frontier_cut`: visited/frontier/cut preservation for graph traversals.
- Require public exec surfaces to reject malformed nodes, edges, weights, and
  invalid indices, or return explicit empty/error decisions.
- Keep implementation, evidence, and canonical spec sync as future tasks.

## Capabilities

### New Capabilities

- `domain-primitives.graph-adjacency`: finite node/edge and adjacency primitive.
- `domain-primitives.graph-reachability`: finite reachability/path-witness primitive.
- `domain-primitives.graph-topological-order`: DAG/topological-order primitive.
- `domain-primitives.graph-component-partition`: undirected component partition primitive.
- `domain-primitives.graph-shortest-path-envelope`: bounded non-negative path-cost primitive.
- `domain-primitives.graph-frontier-cut`: traversal frontier/cut preservation primitive.

## Impact

- **Files**: OpenSpec artifacts and implementation add
  `src/graph_adjacency.rs`, `src/graph_reachability.rs`,
  `src/graph_topological_order.rs`, `src/graph_component_partition.rs`,
  `src/graph_shortest_path_envelope.rs`, `src/graph_frontier_cut.rs`,
  `src/lib.rs`, `docs/requirements.md`, tests, and evidence.
- **APIs**: exact finite runtime admission APIs are added for the listed graph envelopes.
- **Dependencies**: no new runtime dependencies.
- **Testing**: this change validates with OpenSpec now; implementation evidence runs focused tests, Verus, Tracey, strict OpenSpec validation, and repo validation.

## Out of Scope

- negative-weight shortest paths, floating-point weights, heuristic search
  optimality, randomized graph algorithms, or complexity guarantees
- external graph libraries or large graph engines
- cross-family graph facades before the primitive modules exist
