## Context

Monomorphization collection determines which concrete functions, statics, and shims need codegen. The guide also describes codegen-unit partitioning for stable non-generic code and volatile monomorphized instances.

Trellis can model the finite graph and partition invariants while leaving actual rustc type resolution and codegen out of scope.

## Decisions

### 1. Model mono items as finite identities

**Choice:** A mono item is a declared function/static/shim kind plus concrete instance metadata and a stable finite identity.

**Rationale:** Collection and partition correctness can be checked over item identity without executing generic substitution.

### 2. Use explicit roots and dependency edges

**Choice:** Collection operates over a finite set of roots and item-reference edges. An admitted collection contains exactly the finite reachability closure under those edges.

**Rationale:** This captures the collector's essential obligation in a Verus-friendly graph model.

### 3. Treat concrete instance validity as input metadata

**Choice:** The primitive validates declared concrete-type/const metadata for shape and equality, but does not solve Rust traits or normalize all types.

**Rationale:** Full trait solving is too broad. The mono primitive should consume already-declared instance facts.

### 4. Deduplicate by canonical item identity

**Choice:** Duplicate reachable occurrences must collapse to a single mono item in the collected set.

**Rationale:** Codegen item lists should be sets over canonical identity, not traversal histories.

### 5. Partition codegen units by disjoint coverage

**Choice:** CGU admission requires every collected item to appear in exactly one codegen unit, with stable/volatile classification metadata matching declared item kind.

**Rationale:** Partition facts are useful to release/evidence layers and can be proven without backend execution.

## Risks / Trade-offs

- Exact rustc partition heuristics are not stable enough for Trellis to copy. The model should specify Trellis-owned stable/volatile partition rules.
- Instance metadata can be simplified initially. Later primitives may refine type identity if needed.
