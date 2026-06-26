## Context

seL4 revocation and authority ancestry depend on a capability derivation tree. This change isolates that model so copy/mint/move/revoke/recycle rules can close without mixing in CSpace traversal or rights algebra.

## Goals / Non-Goals

**Goals:**
- Model live capability ancestry as an acyclic parent-pointer tree.
- Preserve ancestry under move.
- Remove exactly one live subtree under revoke.
- Allow recycle only for live leaves, clearing exactly one node.

**Non-Goals:**
- Slot lookup
- Reply-cap lifecycle
- Untyped memory reclaim

## Decisions

### 1. Represent derivation as a topologically ordered parent-pointer tree

**Choice:** Store one parent pointer per capability with the invariant `parent < child`, and implement ancestry/subtree checks as recursive parent walks over that topological order.
**Rationale:** This makes acyclicity structural and gives small proof-friendly recursive helpers without a visited-set model.

### 2. Track liveness separately from structural ancestry

**Choice:** Keep `parents` stable and use a parallel `present` vector for live capabilities.
**Rationale:** Revoke and recycle become exact liveness updates without rewriting the structural tree.

### 3. Model move as slot relocation only

**Choice:** Move updates only the slot vector.
**Rationale:** seL4 move should preserve ancestry and descendant relations.

### 4. Make revoke and recycle exact liveness transforms

**Choice:** Revoke clears the entire live target subtree, including the target root itself, via `revoke_present_spec(target)`. Recycle clears `present.update(node, false)` only when the node is a live leaf.
**Rationale:** This keeps the runtime shape simple and pins the model's exact liveness update instead of leaving target-vs-descendants behavior ambiguous.

## Risks / Trade-offs

**[Revocation proofs can touch every node]** Use an explicit recursive subtree-membership helper plus a helper lemma that subtree membership extends from a live parent to its live child, then prove the resulting live set still satisfies the live-parent invariant.
**[Recycle can accidentally clear non-leaves]** Check for live children first and prove the final update preserves the live-parent invariant.
**[Move vs copy confusion]** Keep copy/mint append-child semantics separate from move's slot-only semantics.
