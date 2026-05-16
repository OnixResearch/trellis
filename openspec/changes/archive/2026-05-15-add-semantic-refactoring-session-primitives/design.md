## Context

Unison's codebase model allows names to move between immutable definition hashes and uses structured refactoring to propagate compatible replacements while leaving incompatible dependents for explicit resolution. This change models that structurally with finite tables.

## Goals / Non-Goals

**Goals:**
- Define name binding movement over immutable definition identities.
- Prove compatible propagation and unresolved-dependent tracking facts.
- Keep codebase/session validity explicit even mid-refactor.

**Non-Goals:**
- Real compiler typechecking or compatibility inference.
- Textual rewrite, merge, or IDE behavior.
- Dependency closure or remote sync, covered by separate changes.

## Decisions

### 1. Compatibility is declared metadata

**Choice:** Use explicit type/interface compatibility metadata rather than modeling type inference.

**Rationale:** This keeps the slice finite and verifiable while preserving the important admission boundary.

### 2. Old definitions remain in the codebase table

**Choice:** Refactoring steps add/move bindings but do not remove old definitions.

**Rationale:** This captures immutability and history while avoiding prune/garbage-collection semantics.

### 3. Unresolved dependents are first-class

**Choice:** Incompatible propagation is admitted only if unresolved dependents are recorded.

**Rationale:** This lets a session remain valid without pretending every dependent was rewritten.

## Risks / Trade-offs

**Type-system overclaiming** → Treat compatibility as explicit input metadata; do not prove real type equivalence.

**Too broad refactoring model** → Start with single old/new replacement steps and finite dependent sets.

**Name/dependency overlap** → Reuse content-addressed code primitives later, but keep this slice implementable with opaque hashes.
