## Context

The local seL4 primitives become most valuable when they compose into a kernel-state invariant. This final change gives the repo a place to state the global invariant and prove primitive steps preserve it.

## Goals / Non-Goals

**Goals:**
- State a composed kernel invariant.
- Decompose it into named local obligations.
- Prove primitive steps preserve it.

**Non-Goals:**
- C correspondence proofs
- Liveness or timing proofs
- Driver or boot invariants outside the modeled primitives

## Decisions

### 1. Use an explicit aggregate `KernelState`

**Choice:** Model the composed invariant over
`KernelState { tcbs, ready_queues, endpoints, notification_count, reply_count, cdt, cspace, untypeds, asid_pool, vspaces, irqs }`.
**Rationale:** This reuses the existing seL4 module types directly and keeps cross-module references concrete instead of inventing a second object model.

### 2. Name the sub-invariants up front

**Choice:** The initial named sub-invariants are:
- `object_refs_valid_spec` — blocked-object references, endpoint waiters, and IRQ handlers point at valid object indices
- `cdt_wf_spec` — `state.cdt.wf()`
- `cspace_valid_spec` — root and child-slot references stay within `state.cspace.nodes`
- `tcb_legal_spec` — every `Tcb.state` satisfies `legal_state_spec`
- `scheduler_consistent_spec` — `consistent_ready_queues_spec(queue_view_spec(...), state.tcbs@)`
- `untyped_ownership_spec` — every untyped object is `wf()` and top-level untyped regions are pairwise disjoint
- `asid_pool_wf_spec` — `state.asid_pool.wf()`
- `vspace_consistent_spec` — every `VSpace` is `wf()`
- `irq_unique_spec` — IRQ numbers are pairwise distinct and bound handlers reference valid notification indices
**Rationale:** Review and implementation need the concrete proof targets pinned before any task can be scoped.

### 3. Add this change last

**Choice:** Add this change last
**Rationale:** The composition proof should depend on stable local modules rather than force early representation choices.

### 4. Pin the initial preservation-step set

**Choice:** The initial preservation-step set for this change is:
- `step_endpoint_send_block` — a running sender blocks and enters an endpoint sender queue when no receiver is waiting
- `step_endpoint_send_rendezvous` — a running sender matches the queued head receiver, where that receiver TCB is blocked on the same endpoint and has a priority representable in the ready-queue array, then wakes it onto the ready queue
- `step_untyped_revoke` — an untyped object revokes all of its children
**Rationale:** These steps exercise the scheduler/object-reference story and the untyped ownership story without pulling this change into every seL4 primitive at once, while keeping the first API surface explicit about which validity checks stay as wrapper `requires` clauses.

### 5. Keep verification artifacts local to `src/sel4_invariants.rs`

**Choice:** Put the defining spec fns, exec wrappers, and named `proof fn` artifacts in `src/sel4_invariants.rs`; only add `src/properties.rs` helpers if proof factoring later forces it.
**Rationale:** The proof surface is already cross-module because `KernelState` aggregates many modules. A second property layer should be optional, not mandatory.

### 6. Verification strategy

**Choice:** Use a public `kernel_invariant_spec(state)` conjunction, public named sub-invariant spec fns, public exec wrappers for the initial step set, and public named `proof fn` artifacts for decomposition and step preservation.
**Rationale:** This matches the repo pattern and makes the Tracey surface explicit. `docs/requirements.md` must carry `r[sel4-invariants.compose]`, `r[sel4-invariants.decomposition]`, and `r[sel4-invariants.step-preservation]` because Tracey reads that file rather than only the OpenSpec change docs.

## Risks / Trade-offs

**[Monolithic invariant proofs can become brittle]** Keep the invariant as a conjunction of named sub-invariants and prove preservation one sub-invariant at a time.
**[Dependency churn from earlier modules]** Land this change after local representations settle.
**[Cross-module references can overreach]** Keep notification/reply modeling abstract at first with counts/indices instead of forcing full composed object semantics before the step set needs them.
