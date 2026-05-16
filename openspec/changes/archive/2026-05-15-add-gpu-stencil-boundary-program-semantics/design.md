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

### 1. Window admission, not arithmetic semantics

**Choice:** Model the set of input coordinates a stencil may read, not the computation performed on those reads.

**Rationale:** This yields reusable memory-safety facts for many kernels without committing to a numeric operator.

### 2. Guarded boundary modes

**Choice:** Start with explicit in-bounds guard admission and reject unguarded boundary reads.

**Rationale:** Padding, clamping, wraparound, and halo exchange policies should be later specialized slices.

### 3. 2D first

**Choice:** Target finite 2D stencil envelopes over existing tiled-kernel and layout primitives.

**Rationale:** 2D covers common image/grid kernels while keeping proof burden small.

## Risks / Trade-offs

**Overclaiming GPU semantics** → Keep requirement wording finite and structural, and reject claims about runtime execution, memory consistency, or performance.

**Nonlinear arithmetic proof churn** → Use small products, explicit overflow assertions, and contracts that mirror executable checked arithmetic.

**Tracey drift** → Add `r[depends]`, `r[impl]`, and `r[verify]` IDs in the same implementation slice and run uncovered/untested checks before completion.
