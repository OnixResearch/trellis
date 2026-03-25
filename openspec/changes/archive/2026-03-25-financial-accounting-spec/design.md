## Context

The `balance`, `transfer`, `double_entry`, and `journal` modules verify
fundamental accounting invariants. Each module follows the standard
verified-logic pattern: spec fn → exec fn → proof fn → tests. But the
requirements live only in code comments and ensures clauses — no
authoritative spec document ties them together.

## Goals / Non-Goals

**Goals:** Capture every verifiable property across the four modules in
a single spec. Make the requirements tracey-trackable.

**Non-Goals:** Adding new functions or proofs. This change is
specification and annotation only.

## Decisions

### 1. Single domain, not four

**Choice:** One `financial-accounting` spec covering all four modules.
**Rationale:** The modules are tightly coupled — transfer uses balance,
journal uses apply_entry which bridges to balance semantics, double_entry's
conservation connects to transfer's conservation. A single spec captures
cross-cutting properties like "conservation of value" once.
**Alternative:** Per-module specs. Rejected — would duplicate the
conservation requirement across balance, transfer, and double-entry.

### 2. Requirement IDs match existing annotation style

**Choice:** IDs like `balance.credit`, `transfer.conservation`,
`journal.valid` — matching the existing `r[depends ...]` tags already in
source files.
**Rationale:** Minimal annotation churn. Most tags already exist and just
need a spec document to land against.

### 3. Spec covers properties, not implementation details

**Choice:** Requirements state what holds (conservation, monotonicity,
roundtrip), not how (loop invariant structure, overflow checks).
**Rationale:** Implementation may change (different overflow detection
strategy, different loop structure) but the properties are permanent.

## Risks / Trade-offs

**Annotation drift** → The spec is the source of truth. If a requirement
ID changes in the spec, the corresponding `r[...]` tags must update.
Tracey catches this.
