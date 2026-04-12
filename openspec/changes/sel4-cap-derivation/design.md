## Context

seL4 revocation and authority ancestry depend on a capability derivation tree. Splitting this out gives one place for copy/mint/move/revoke structure proofs without mixing in CSpace traversal or local rights algebra.

## Goals / Non-Goals

**Goals:**
- Model ancestry as an acyclic tree.
- Preserve ancestry under move.
- Remove exactly descendants on revoke.

**Non-Goals:**
- Slot lookup
- Reply-cap lifecycle
- Untyped memory reclaim

## Decisions

### 1. Represent derivation as a tree relation

**Choice:** Represent derivation as a tree relation
**Rationale:** Revocation is naturally subtree removal, so tree structure is the right abstraction.

### 2. Keep rights amplification out of scope

**Choice:** Keep rights amplification out of scope
**Rationale:** Rights algebra belongs in `sel4_cap_rights`; this change focuses on ancestry and revoke shape.

### 3. Model move as slot relocation only

**Choice:** Model move as slot relocation only
**Rationale:** Move should preserve derivation ancestry even when CSpace placement changes.

## Risks / Trade-offs

**[Revocation proofs can touch many nodes]** Use explicit subtree membership predicates and sibling-preservation lemmas.
**[Move vs copy confusion]** Keep copy/mint create-child and move preserve-node semantics separate in the model.
