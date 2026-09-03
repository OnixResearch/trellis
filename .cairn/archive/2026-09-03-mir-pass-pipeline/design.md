## Context

The MIR pipeline described in the guide moves through named query phases from built MIR to optimized MIR. Some intermediate query results may be stolen by later phases to avoid cloning. Passes mutate MIR in place and are only valid for specific phases or optimization levels.

A Trellis primitive should model the finite pipeline trace and ordering obligations. It should not try to run rustc queries.

The implementation uses a functional core. Small model, registration, predicate, transition, proof, and test components remain independently reviewable. No component owns external effects.

## Decisions

### 1. Model phases as an explicit total order

**Choice:** The primitive defines a finite ordered phase set for built, const-prepared, promoted, drop-elaborated/const-checked, and optimized MIR dialects. A transition advances by one declared phase.

**Rationale:** Many pipeline bugs are phase-order bugs. A total finite order is easy to validate and reason about.

### 2. Represent passes as phase-compatible transformations

**Choice:** Each pass descriptor declares input phase, output phase or same-phase mutation, required validation predicates, and optional optimization-level gates.

**Rationale:** This lets Trellis reject misplaced passes without proving every pass's semantics.

### 3. Track artifact states for borrow and steal

**Choice:** Intermediate MIR artifacts have states such as available, borrowed, and stolen. A read after steal is invalid unless a prior dependency forced the read before stealing.

**Rationale:** The guide explicitly warns about stolen intermediate results. Modeling this ordering makes lifecycle evidence precise.

### 4. Keep mutation boundaries structural

**Choice:** In-place mutation is admitted only when the pass declares preserved structural properties or produces new validation metadata before the next phase.

**Rationale:** The pipeline primitive checks whether a trace is well staged; semantic preservation belongs to pass-specific proofs.

### 5. Make optimization-level gating explicit

**Choice:** Experimental or optimization-only passes require a declared level to be enabled. A trace that claims a disabled pass is rejected before state changes.

**Rationale:** This prevents accidental claims that optional passes always run.

## Risks / Trade-offs

- Modeling every rustc phase detail would be too broad. The first version should stick to finite query artifacts and pass descriptors.
- Some pass lists may change in rustc. Trellis should model a stable abstract pipeline, not exact upstream implementation order.
