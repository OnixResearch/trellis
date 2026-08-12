# Graph Domain Primitives Specification

## Purpose

Adds reusable exact finite graph requirements for domain primitives. These
requirements cover graph adjacency, reachability, topological ordering,
component partitions, bounded shortest-path envelopes, and traversal
frontier/cut preservation. They MUST NOT claim heuristic optimality,
probabilistic graph properties, floating-point path accuracy, complexity bounds,
or external graph-library behavior.

## ADDED Requirements

### Requirement: Graph Adjacency [r[domain-primitives.graph-adjacency]]
The system MUST provide a finite graph-adjacency primitive for node-count
validity, edge endpoint admission, and adjacency lookup.

#### Scenario: Valid edge is admitted into adjacency [r[domain-primitives.graph-adjacency.scenario.valid-edge-is-admitted-into-adjacency]]

- GIVEN a finite node count and an edge whose source and destination are valid node indices
- WHEN adjacency is constructed or queried
- THEN the edge is accepted and appears only in the source node adjacency relation

#### Scenario: Malformed edge is rejected [r[domain-primitives.graph-adjacency.scenario.malformed-edge-is-rejected]]

- GIVEN an edge with a source or destination outside the finite node range
- WHEN graph adjacency admission is checked
- THEN the edge is rejected and the graph state is unchanged

### Requirement: Graph Reachability [r[domain-primitives.graph-reachability]]
The system MUST provide a finite graph-reachability primitive for path-witness
validity, reachability extension, and explicit unreachable results.

#### Scenario: Valid path witness proves reachability [r[domain-primitives.graph-reachability.scenario.valid-path-witness-proves-reachability]]

- GIVEN a valid finite graph and a path witness whose adjacent nodes follow admitted edges
- WHEN reachability for the path endpoints is checked
- THEN the destination is reachable from the source

#### Scenario: Invalid or absent path yields explicit non-reachability result [r[domain-primitives.graph-reachability.scenario.invalid-or-absent-path-yields-explicit-non-reachability-result]]

- GIVEN invalid endpoints, a malformed path witness, or a bounded search that finds no path
- WHEN reachability is checked
- THEN the primitive rejects the request or returns an explicit unreachable/empty result

### Requirement: Graph Topological Order [r[domain-primitives.graph-topological-order]]
The system MUST provide a graph topological-order primitive for DAG admission,
permutation validity, and edge-forward ordering.

#### Scenario: Valid topological order places every edge forward [r[domain-primitives.graph-topological-order.scenario.valid-topological-order-places-every-edge-forward]]

- GIVEN a finite directed graph and an order containing every node exactly once
- WHEN topological-order validity is checked
- THEN every admitted edge points from an earlier node to a later node in the order

#### Scenario: Cycle or malformed order is rejected [r[domain-primitives.graph-topological-order.scenario.cycle-or-malformed-order-is-rejected]]

- GIVEN a graph with a directed cycle, duplicate order entries, missing nodes, or out-of-range nodes
- WHEN topological-order admission is checked
- THEN the order is rejected

### Requirement: Graph Component Partition [r[domain-primitives.graph-component-partition]]
The system MUST provide an undirected component-partition primitive for complete,
disjoint grouping of finite graph nodes by connectivity.

#### Scenario: Partition covers each node exactly once [r[domain-primitives.graph-component-partition.scenario.partition-covers-each-node-exactly-once]]

- GIVEN a valid finite undirected graph and a component partition
- WHEN partition validity is checked
- THEN every node appears in exactly one component and all nodes inside a component are connected

#### Scenario: Invalid component partition is rejected [r[domain-primitives.graph-component-partition.scenario.invalid-component-partition-is-rejected]]

- GIVEN a partition with missing nodes, duplicate nodes, out-of-range nodes, or a component containing disconnected nodes
- WHEN component-partition admission is checked
- THEN the partition is rejected

### Requirement: Graph Shortest Path Envelope [r[domain-primitives.graph-shortest-path-envelope]]
The system MUST provide a bounded shortest-path-envelope primitive for
non-negative weights, path-cost accumulation, and candidate-cost comparison.

#### Scenario: Candidate path cost is bounded and comparable [r[domain-primitives.graph-shortest-path-envelope.scenario.candidate-path-cost-is-bounded-and-comparable]]

- GIVEN a valid finite weighted graph with non-negative bounded edge weights and a valid path witness
- WHEN the path cost is accumulated
- THEN the resulting cost is accepted, bounded, and comparable against other accepted candidate costs

#### Scenario: Invalid or overflowing path cost is rejected [r[domain-primitives.graph-shortest-path-envelope.scenario.invalid-or-overflowing-path-cost-is-rejected]]

- GIVEN a negative weight, malformed edge, invalid path witness, or path whose accumulated cost would overflow the bound
- WHEN path-cost admission is checked
- THEN the cost request is rejected

### Requirement: Graph Frontier Cut [r[domain-primitives.graph-frontier-cut]]
The system MUST provide a graph frontier/cut primitive for traversal state,
visited/frontier disjointness, and expansion preservation.

#### Scenario: Valid expansion preserves visited frontier cut invariant [r[domain-primitives.graph-frontier-cut.scenario.valid-expansion-preserves-visited-frontier-cut-invariant]]

- GIVEN a valid traversal state with disjoint visited and frontier sets
- WHEN one valid frontier node is expanded through admitted outgoing edges
- THEN the resulting visited and frontier sets remain within the node range and preserve the declared cut invariant

#### Scenario: Invalid frontier expansion is rejected without mutation [r[domain-primitives.graph-frontier-cut.scenario.invalid-frontier-expansion-is-rejected-without-mutation]]

- GIVEN a traversal state with an out-of-range frontier node, overlapping visited/frontier sets, or malformed outgoing edge
- WHEN expansion is requested
- THEN the operation is rejected and the traversal state is unchanged
