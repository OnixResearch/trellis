## Context

Lattice/Salt frames several useful verification UX ideas: type facts can discharge redundant checks, call-site assumptions can imply callee preconditions, slice views can carry bound certificates, arena provenance can be tracked by scope depth, and async paths can be admitted against named progress budgets. Trellis should adapt these as Verus-checked finite models rather than importing the Lattice compiler/runtime architecture.

## Goals

- Provide a pure verification-frontier model for contract implication and check-elision metadata.
- Reuse Trellis's existing style: spec predicates, exec helpers with `ensures`, proof bridge lemmas, and explicit positive/negative tests.
- Keep all adapted logic deterministic and small enough to validate without external solvers at runtime.
- Document the Lattice reference as inspiration only.

## Non-Goals

- Do not introduce a new trusted SMT backend into Trellis core.
- Do not emit or reason about real machine code, MLIR, LLVM, or kernel scheduling behavior.
- Do not turn `Unknown` into a runtime assertion policy unless a later, separately specified adapter owns that shell behavior.
- Do not add broad benchmark or performance claims from the external repository.

## Decisions

### 1. Model outcomes explicitly

**Choice:** Every adapted verifier primitive SHALL return or define an explicit admission outcome: `Proven`, `Rejected`, or `Unknown` where uncertainty is possible.

**Rationale:** Lattice's UX emphasizes proof-or-fallback. Trellis needs a stronger proof boundary: `Proven` may erase or accept a check in a finite model, `Rejected` records a modeled violation, and `Unknown` remains non-proof evidence until a caller handles it explicitly.

### 2. Keep solver-like reasoning finite

**Choice:** Type facts, caller facts, path facts, slice facts, and progress costs SHALL be represented as finite in-memory data and Verus predicates.

**Rationale:** Trellis can verify these adapters directly. A future executable shell may translate external compiler facts into the finite model, but the core proof does not depend on an external solver.

### 3. Split functional core from shell

**Choice:** Core modules SHALL expose pure specs and thin exec helpers. Any future file scanning, report rendering, or external-reference documentation lives outside the core.

**Rationale:** The adapted ideas are easiest to test and prove when the core takes typed facts and returns deterministic metadata.

### 4. Require bounded negative evidence

**Choice:** Each primitive SHALL include negative tests or proof cases: violated preconditions for contract implication, inverted/out-of-bounds slices, local arena escapes, stale scope stores, and over-budget or no-yield progress paths.

**Rationale:** These patterns are most useful when they reject bad evidence cleanly rather than silently accepting incomplete facts.

### 5. Treat Lattice as a reference, not a dependency

**Choice:** Documentation SHALL cite Lattice/Salt as an external design reference only. No implementation task may treat Lattice behavior, benchmarks, or code as authoritative validation for Trellis.

**Rationale:** Trellis validation remains Verus, tests, Tracey, and repo-local evidence.
