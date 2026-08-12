## Context

Physics primitive proofs should first use exact discrete envelopes: dimension
vectors, bounded scalar/vector components, finite time horizons, conservative
AABB-style collision candidates, bounded contact records, and non-negative
energy/momentum budgets. This gives later physics code reusable safety checks
without pretending to verify a full simulator.

## Goals / Non-Goals

**Goals:** specify exact-model physics primitive contracts and future evidence tasks.

**Non-Goals:** implement modules in this pass; prove floating-point numerical
accuracy, continuous dynamics, or solver convergence.

## Decisions

### 1. Dimension and bounds before dynamics

**Choice:** `physics_units`, `physics_vector_bounds`, and `physics_time_horizon`
precede integration/collision/budget primitives.

**Rationale:** Dynamics proofs need dimensional compatibility, representable
state, and bounded time arithmetic.

### 2. Conservative envelopes instead of exact collision physics

**Choice:** Collision and contact primitives specify conservative admission and
record validity, not exact physical contact resolution.

**Rationale:** Conservative checks are useful and proof-friendly; exact physics is
not in scope.

## Validation Plan

- `openspec validate add-physics-primitives --strict --json`
- later implementation evidence per task: focused cargo tests, Verus, Tracey
