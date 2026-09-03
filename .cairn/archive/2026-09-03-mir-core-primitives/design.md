## Context

The rustc-dev-guide describes MIR as a control-flow graph made of basic blocks, statements, final terminators, indexed locals, places with projections, operands, and non-nested rvalues. Trellis has several downstream compiler primitives, but each currently carries its own simplified raw program shape.

The first useful primitive is a finite MIR envelope and admission layer. It should be small enough to verify directly and rich enough for later drop-elaboration, dataflow, pass-pipeline, and backend lowering changes.

## Decisions

### 1. Model MIR structure, not rustc internals

**Choice:** Define Trellis-owned MIR structs/enums for finite bodies, blocks, locals, places, projections, operands, rvalues, statements, and terminators.

**Rationale:** rustc types change over time and depend on large compiler contexts. Trellis needs stable proof input shapes, not an embedded compiler.

### 2. Keep `RETURN_PLACE` named and explicit

**Choice:** The return local must be represented by a named return-place constant in the model, and body admission must require that local to exist before normal locals or temporaries are referenced.

**Rationale:** MIR semantics depend on a distinguished return place. Naming the boundary avoids unexplained numeric indexing in proofs and tests.

### 3. Separate shape admission from type checking

**Choice:** MIR core admission checks finite structure, range validity, and declared type tags, but it does not prove full Rust type correctness.

**Rationale:** Shape validity is a reusable base. Full Rust typing needs separate HIR/THIR/type-system context and should not be smuggled into this core.

### 4. Enforce statement/terminator block shape

**Choice:** A basic block has zero or more statements and exactly one final terminator. Terminator successor references must be in range of the body block table.

**Rationale:** Existing Trellis CFG analyses assume finite successor graphs. MIR core should make that graph extraction safe and deterministic.

### 5. Preserve MIR's non-nested rvalue boundary

**Choice:** Rvalues may reference operands and places, but may not embed arbitrary nested rvalues. Temporary creation belongs to construction/lowering layers, not MIR core admission.

**Rationale:** This is one of MIR's simplifying invariants and gives later dataflow and codegen proofs a stable local-transfer shape.

### 6. Use deterministic rejection metadata

**Choice:** Runtime helpers should reject malformed bodies with explicit reason codes such as missing return place, invalid local, invalid projection, missing terminator, or out-of-range successor.

**Rationale:** Negative tests should prove fail-closed behavior instead of relying on panics or unchecked indexing.

## Risks / Trade-offs

- A small Trellis MIR will not capture all rustc variants initially. The benefit is a checkable proof surface with explicit non-claims.
- Place/projection typing can grow complex. The first implementation should admit declared finite projection tags and defer deep Rust type layout checks.
- Multiple existing raw-IR modules may need adapters later; this change only defines the shared core, not migration of every consumer.
