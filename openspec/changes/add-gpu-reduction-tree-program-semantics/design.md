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

### 1. Shape before algebra

**Choice:** Verify tree/stage cardinality and bounds first, leaving operator laws to separate algebraic primitive slices.

**Rationale:** This prevents overclaiming correctness for floating-point or custom reductions.

### 2. Stage-local invariants

**Choice:** Represent each reduction stage by input count, output count, stride, and active lane bounds.

**Rationale:** Small stage contracts compose better in Verus than a monolithic tree proof.

### 3. Compose with block-reduction capacity

**Choice:** Use the existing block-reduction program admission as the outer envelope.

**Rationale:** This reuses already verified work/shared-memory capacity checks.

## Risks / Trade-offs

**Overclaiming GPU semantics** → Keep requirement wording finite and structural, and reject claims about runtime execution, memory consistency, or performance.

**Nonlinear arithmetic proof churn** → Use small products, explicit overflow assertions, and contracts that mirror executable checked arithmetic.

**Tracey drift** → Add `r[depends]`, `r[impl]`, and `r[verify]` IDs in the same implementation slice and run uncovered/untested checks before completion.
