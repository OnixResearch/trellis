## Context

This repo is best at value-level models with crisp invariants. ChaosControl has
several such kernels already, but they currently live as ordinary Rust logic
inside an exploration system. The target here is not the full hypervisor or
explorer. The target is the pure logic that decides:

- whether a branch found new coverage
- whether an assertion passed, failed, or stayed unexercised
- whether two SMP interleavings are distinct
- which scheduled faults are still pending or preserved by a subset
- which frontier entries and input-tree alternatives are worth trying next
- whether a fault state represents a quorum-healthy period

Those decisions are small enough to verify and important enough that a bug
changes global search behavior.

## Goals / Non-Goals

**Goals:**
- Translate the pure ChaosControl kernels into this repo's normal
  `spec/exec/proof/test` pattern.
- Capture algebraic properties and search-policy invariants, not only
  example-by-example behavior.
- Keep module boundaries reusable outside ChaosControl.
- Reuse existing generic primitives in this repo where they already match the
  needed abstraction.

**Non-Goals:**
- Porting KVM/device code or full multi-VM controller logic.
- Proving probabilistic exploration optimality.
- Reproducing every current heuristic constant from ChaosControl if a simpler,
  stable mathematical core suffices.
- Replacing integration tests in ChaosControl; this repo only provides the
  verified primitives.

## Decisions

### 1. Use repo-local module names with ChaosControl-prefixed requirements

**Choice:** add modules like `chaoscontrol_coverage`, `chaoscontrol_oracle`,
`chaoscontrol_scheduler`, `chaoscontrol_fault_schedule`,
`chaoscontrol_frontier`, `chaoscontrol_input_tree`, and
`chaoscontrol_fault_state`.

**Rationale:** this avoids collisions with existing generic modules such as
`bitmap` and `quorum`, while keeping provenance obvious.

### 2. Reuse existing generic lemmas where possible

**Choice:** when an existing module already models the needed math, reuse it
instead of re-proving the same fact under a new name. Examples:
- `bitmap` / `bitwise` for mask-style reasoning
- `quorum` for majority properties
- `monotonic` for countdown/advance monotonicity
- `seq_set` for subset/filter relationships

**Rationale:** the new value is in the ChaosControl-specific composition, not in
cloning already-verified arithmetic.

### 3. Separate policy kernels from imperative shells

**Choice:** only port pure decision kernels. For example:
- coverage region layout, merge, classify, and new-edge queries
- oracle verdict tables and run-accounting transitions
- scheduler state transitions and fingerprint update step
- schedule drain/subset helpers
- frontier effective-score computations and bounded alternative generation
- fault-state predicates over alive/partition bitmaps

**Rationale:** this repo verifies value transformations well. Pulling in host
I/O or mutation-heavy orchestration would add proof cost without adding much
reusable logic.

### 4. Verify search-policy invariants directly

**Choice:** treat exploration heuristics as real semantics. The proofs should
state things like:
- merge is associative/commutative/idempotent
- new-coverage count is zero iff the bitmap adds no new hit positions
- frontier greedy choice maximizes effective score among candidates
- generated alternatives never equal the original value and never exceed the
  budget
- fault schedule draining never re-delivers a fault

**Rationale:** search bugs are often silent policy drift, not panics.

### 5. Keep scheduler and fault-schedule exec code iterative

**Choice:** prefer iterative exec implementations with recursive or algebraic
proof helpers only where needed.

**Rationale:** this matches repo guidance from other modules: proofs may be
recursive, runtime paths should stay efficient and unsurprising.

## Risks / Trade-offs

**[Duplicate abstractions]** -> A ChaosControl-specific wrapper can overlap an
existing generic primitive. Mitigation: only keep the wrapper when the domain
shape matters; otherwise compose existing modules.

**[Heuristic churn]** -> Frontier and input-tree policies can evolve. Mitigation:
verify stable semantic obligations (budget bounds, ordering relations,
monotonicity) instead of every tuning knob.

**[Overfitting to one upstream repo]** -> Names and behavior may look too
ChaosControl-specific. Mitigation: keep specs mathematical and document the
source system only as motivation.

## Migration Plan

1. Add coverage and oracle modules first; they have the highest correctness
   leverage and the clearest specs.
2. Add scheduler and fault-schedule modules next; they support replay and SMP
   exploration semantics.
3. Add frontier, input-tree, and fault-state modules after the lower-level
   kernels are stable.
4. Add cross-module properties in `src/properties.rs` and corresponding
   `docs/requirements.md` entries.
5. Validate with Verus, tests, clippy, and Tracey coverage.
