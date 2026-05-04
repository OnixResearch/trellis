## Context

The repository already contains some graph-shaped proof patterns, and future
primitive families will need them again. A focused graph OpenSpec lets later
implementation reuse one set of finite graph validity boundaries and proof
contracts instead of reauthoring reachability, DAG, component, or frontier rules
inside each domain change.

## Goals / Non-Goals

**Goals:**

- define exact finite graph models for nodes, edges, adjacency, and paths
- specify validity boundaries and invalid-input behavior before implementation
- keep each graph primitive small enough for independent Verus contracts and
  Tracey coverage
- support later composition by data-structure, GIS, ML, scheduling, and resource
  graph changes

**Non-Goals:**

- prove asymptotic complexity, allocation behavior, or hardware/cache behavior
- model heuristic quality, probabilistic graph properties, or floating-point path
  costs
- replace existing specialized graph code before matching these requirement IDs

## Decisions

### 1. Use finite node-indexed graph models

**Choice:** Requirements are stated over finite node counts, bounded node indices,
finite edge sequences, optional adjacency lists, and explicit path witnesses.

**Rationale:** Index and endpoint bounds are Verus-friendly, compose with bounded
array/map primitives, and avoid ambiguous object-identity or allocator models.

**Alternative:** Specify generic graph libraries with arbitrary node payloads and
iterator-heavy APIs. Rejected because the first verified slice should prove
structural graph facts before ergonomic wrappers.

### 2. Separate reachability, topological order, components, and costs

**Choice:** Reachability, DAG/topological ordering, undirected component
partitioning, and non-negative path-cost envelopes get separate requirements.

**Rationale:** Each has a different proof shape. Splitting them avoids one
brittle mega-proof and lets later implementation land one primitive at a time.

### 3. Conservative shortest-path envelope before full algorithms

**Choice:** The shortest-path primitive specifies bounded non-negative path-cost
admission, candidate-cost comparison, and rejection of malformed/overflowing
weights. It does not require a particular algorithm or complexity guarantee.

**Rationale:** Cost bounds and witness validity are reusable and verifiable; full
algorithmic optimality can be added later if needed with dedicated evidence.

## Risks / Trade-offs

**Overlap with existing reachability/topological code** → Later implementation
should either reuse existing verified modules or add thin requirement-aligned
wrappers after auditing current contracts.

**Too many graph variants** → This first graph family covers directed finite
edges, undirected components, and non-negative weighted envelopes only. Other
models need separate OpenSpecs.

**Shortest-path overclaiming** → Keep wording to candidate/witness envelopes and
non-negative bounded costs unless a later proof establishes full optimality.

## Validation Plan

- `openspec validate add-graph-primitives --strict --json`
- umbrella validation after taxonomy updates
- implementation evidence: focused tests, `./scripts/verus.sh src/lib.rs`,
  Tracey uncovered/untested checks, and final repo validation before archive
