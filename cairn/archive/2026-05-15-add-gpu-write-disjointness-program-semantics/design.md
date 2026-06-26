## Context

The repository already contains finite GPU launch, memory-layout, and program-shape admission predicates in `src/gpu_primitives.rs`. This change narrows the next program-oriented proof slice so implementation can add structural guarantees without claiming full GPU execution semantics.

## Goals / Non-Goals

**Goals:**
- Add small reusable finite contracts for this GPU program family.
- Keep executable checkers aligned with open specs and Tracey requirements.
- Provide positive and negative tests plus Verus proof functions for each public admission surface.

**Non-Goals:**
- Runtime CUDA API integration.
- Hardware scheduling, memory consistency, occupancy, or performance claims.
- Algebraic/numerical correctness unless a requirement explicitly states it.

## Decisions

### 1. Region-first model

**Choice:** Represent writes as finite byte ranges derived from admitted program coordinates rather than as device pointers.

**Rationale:** Verus proofs can compose range start/end, span, and disjointness facts without trusting runtime allocation state.

### 2. Pairwise disjointness only

**Choice:** Prove deterministic pairwise non-overlap over finite sets/windows first.

**Rationale:** Race freedom for arbitrary kernels needs memory-model and scheduler semantics outside this structural slice.

### 3. Reuse GPU layout primitives

**Choice:** Compose with existing pitched/tensor/tile admissions when a write region is projected from a layout.

**Rationale:** This keeps API boundaries local and avoids re-proving buffer-size facts.

## Risks / Trade-offs

**Overclaiming GPU semantics** → Keep requirement wording finite and structural, and reject claims about runtime execution, memory consistency, or performance.

**Nonlinear arithmetic proof churn** → Use small products, explicit overflow assertions, and contracts that mirror executable checked arithmetic.

**Tracey drift** → Add `r[depends]`, `r[impl]`, and `r[verify]` IDs in the same implementation slice and run uncovered/untested checks before completion.
