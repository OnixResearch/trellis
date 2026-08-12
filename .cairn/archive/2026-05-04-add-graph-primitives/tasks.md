## Coverage Index

| Requirement ID | Module task |
|---|---|
| `domain-primitives.graph-adjacency` | `graph_adjacency` |
| `domain-primitives.graph-reachability` | `graph_reachability` |
| `domain-primitives.graph-topological-order` | `graph_topological_order` |
| `domain-primitives.graph-component-partition` | `graph_component_partition` |
| `domain-primitives.graph-shortest-path-envelope` | `graph_shortest_path_envelope` |
| `domain-primitives.graph-frontier-cut` | `graph_frontier_cut` |

## Phase 1: Spec foundation

- [x] Create focused graph primitive OpenSpec with finite models, validity-boundary, and evidence expectations.

## Phase 2: Core finite graph primitives

- [x] Implement and prove `graph_adjacency`; cover node-count validity, endpoint admission, adjacency lookup, and malformed-edge rejection. `[covers=domain-primitives.graph-adjacency]`
- [x] Implement and prove `graph_reachability`; cover path witness validity, reachability extension, unreachable rejection/empty result, and bounded search admission. `[covers=domain-primitives.graph-reachability]`
- [x] Implement and prove `graph_topological_order`; cover DAG admission, edge-forward ordering, duplicate/missing-node rejection, and cycle rejection. `[covers=domain-primitives.graph-topological-order]`

## Phase 3: Partition, cost, and traversal primitives

- [x] Implement and prove `graph_component_partition`; cover undirected connectivity grouping, partition disjointness, coverage, and invalid-edge rejection. `[covers=domain-primitives.graph-component-partition]`
- [x] Implement and prove `graph_shortest_path_envelope`; cover non-negative bounded weights, path-cost accumulation, candidate-cost comparison, and overflow/malformed-weight rejection. `[covers=domain-primitives.graph-shortest-path-envelope]`
- [x] Implement and prove `graph_frontier_cut`; cover visited/frontier disjointness, cut preservation, expansion admission, and no-mutation-on-invalid-frontier. `[covers=domain-primitives.graph-frontier-cut]`

## Phase 4: Validation and archive

- [x] Sync completed requirement text to `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md`.
- [x] Run focused cargo/Verus/Tracey evidence for implemented modules.
- [x] Run OpenSpec gates after implementation evidence is synced.
- [x] Archive only after implementation and validation tasks complete.
