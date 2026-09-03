## Context

The rustc-dev-guide explains that newly built MIR contains `Drop` and `DropAndReplace` terminators whose target may be initialized, uninitialized, conditionally initialized, or partially initialized. Drop elaboration uses dataflow facts and type drop obligations to decide whether to preserve, remove, guard, or expand drops.

This Trellis primitive should model the finite decision surface and shape-preserving transformation facts, not Rust destructor execution.

## Decisions

### 1. Model obligations over finite structural paths

**Choice:** A drop obligation is a finite structural path paired with declared drop-glue metadata. Obligations are established on initialization and released on moves from the path or descendants.

**Rationale:** Structural paths are the core abstraction in the guide and can be represented without full Rust type layout.

### 2. Treat type drop metadata as input

**Choice:** The primitive consumes finite metadata saying whether a type has direct drop glue, field drop obligations, or enum variant obligations.

**Rationale:** Computing `needs_drop` from all Rust types is outside scope. This keeps the proof surface bounded and reviewable.

### 3. Classify drops by maybe-init and maybe-uninit facts

**Choice:** Drop classification is a pure function over maybe-initialized and maybe-uninitialized memberships for the target and relevant descendants.

**Rationale:** This matches the rustc guide and composes naturally with a future MIR dataflow-framework primitive.

### 4. Preserve explicit `Open` drop expansion

**Choice:** `Open` drops expand recursively through fields or active enum variants until each required obligation is static, dead, or conditional.

**Rationale:** Partial initialization is the hard Rust-specific case and should be modeled directly rather than hidden behind a generic conditional drop.

### 5. Split `DropAndReplace` deterministically

**Choice:** `DropAndReplace` elaboration must produce a deterministic sequence: admitted drop behavior for the old target, followed by a write/initialization of the replacement.

**Rationale:** This gives tests a stable expected transformation and gives proofs a local preservation boundary.

### 6. Keep destructor effects out of scope

**Choice:** Elaborated drops represent calls or skipped calls as trace metadata only. They do not execute or verify destructor code.

**Rationale:** Trellis should prove drop-elaboration decisions, not arbitrary user `Drop` implementations.

## Risks / Trade-offs

- Open-drop recursion can be brittle in Verus. Use explicit fuel and path-depth budgets.
- Enum active-variant facts may need a simplified metadata model. That is acceptable if non-claims stay explicit.
- Later integration with dataflow may require adapters; this change may initially consume dataflow facts as caller-provided finite sets.
