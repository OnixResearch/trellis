## Context

MIR visitors are a convenience layer for walking bodies, blocks, statements, terminators, places, operands, and locals. Traversal helpers walk CFGs in standard orders such as preorder or reverse postorder. Future Trellis compiler passes can benefit from reusable coverage and order facts.

## Decisions

### 1. Define visitable locations explicitly

**Choice:** The primitive defines finite location identifiers for body, block, statement, terminator, local, place, operand, rvalue, and successor-edge visits.

**Rationale:** Explicit locations make coverage and no-duplicate properties checkable without relying on callback machinery.

### 2. Separate immutable and mutable visitor claims

**Choice:** Immutable visitor traces prove coverage and order. Mutable visitor traces additionally declare which structural fields may change and which body-shape invariants must be preserved.

**Rationale:** A visitor that mutates MIR needs stronger obligations than one that inspects MIR.

### 3. Keep traversal orders pure

**Choice:** Preorder and reverse-postorder helpers are pure functions over admitted CFG edges and entry metadata.

**Rationale:** This reuses existing graph primitives and avoids compiler object lifetimes.

### 4. Reject invalid or duplicate visits deterministically

**Choice:** A visitor trace must reject locations outside the body and duplicate visits for traversal modes that promise exactly-once coverage.

**Rationale:** Pass proofs need to know whether all relevant nodes were seen exactly once.

### 5. Keep pass semantics out of scope

**Choice:** The primitive proves traversal coverage and structural preservation only; it does not prove that a pass's rewrite is semantically correct.

**Rationale:** Pass semantics are pass-specific.

## Risks / Trade-offs

- Some real rustc visitors intentionally skip or revisit nodes. This primitive should support declared traversal modes rather than claiming every visitor behaves identically.
- Mutable visitor shape-preservation may need small, explicit mutation descriptors to remain Verus-friendly.
