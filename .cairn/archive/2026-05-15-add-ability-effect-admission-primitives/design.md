## Context

Unison represents effects as abilities in types and uses handlers to provide or discharge them. This change models that concept as finite sets and subset/discharge predicates suitable for Verus.

## Goals / Non-Goals

**Goals:**
- Define deterministic finite ability-set admission.
- Prove subset and handler-discharge properties.
- Provide sandbox attenuation primitives for later remote-computation admission.

**Non-Goals:**
- Implement algebraic effect handlers or continuations.
- Claim runtime effect safety, I/O behavior, or FFI behavior.
- Replace UCAN/capability authorization primitives.

## Decisions

### 1. Finite set model

**Choice:** Represent abilities as bounded identifiers in finite vectors/sets.

**Rationale:** Requirement containment and missing-ability witnesses are easy to verify structurally.

### 2. Handler discharge is syntactic/structural

**Choice:** A handler removes only the declared handled ability from the modeled requirement set.

**Rationale:** This avoids claiming semantic correctness of handler bodies or continuations.

### 3. Sandbox as attenuation

**Choice:** A sandbox-provided ability set must be a subset of the parent-provided set.

**Rationale:** This aligns with least-privilege remote execution checks and composes with existing authorization primitives later.

## Risks / Trade-offs

**Effect-system overclaiming** → Keep this slice finite and structural; do not claim real language typechecking.

**Overlap with capability modules** → Treat abilities as typed effect requirements, not principals/resources/actions.

**Set proof instability** → Prefer bounded vector scans with exact membership contracts and small subset lemmas.
