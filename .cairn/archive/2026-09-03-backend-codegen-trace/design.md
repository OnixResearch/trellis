## Context

The guide describes backend-agnostic codegen traits and MIR lowering modules for blocks, statements, operands, places, and rvalues. A MIR basic block usually maps to one backend block, but calls and assertions can expand into multiple blocks. The builder interface creates values, branches, calls, and other operations.

A Trellis primitive should check a finite trace of builder operations and handle references. It should not attempt to verify LLVM or a concrete backend.

## Decisions

### 1. Model backend handles abstractly

**Choice:** Backend blocks, values, types, functions, and places are finite abstract handles with declaration/definition metadata.

**Rationale:** Handle validity is backend-agnostic and checkable. Concrete pointer lifetimes or LLVM objects are out of scope.

### 2. Record lowering as a deterministic trace

**Choice:** Lowering emits a finite trace tagged by MIR source location and builder operation kind.

**Rationale:** Trace evidence can connect MIR-core structure to backend operations without executing a backend.

### 3. Allow declared block expansion

**Choice:** A MIR block may map to one backend block or to a declared finite expansion when the terminator or statement requires it.

**Rationale:** The guide notes calls and assertions can introduce extra blocks. The primitive should not falsely require one-to-one mapping.

### 4. Check successor and handle consistency

**Choice:** Branch operations must target declared backend blocks derived from admitted MIR successors or declared expansion blocks. Value-using operations must reference previously declared compatible handles.

**Rationale:** These are the core structural safety facts that prevent dangling builder traces.

### 5. Keep ABI and instruction semantics out of scope

**Choice:** The primitive checks trace shape and builder-handle consistency, not target ABI lowering or exact instruction semantics.

**Rationale:** ABI and backend semantics require much larger models.

## Risks / Trade-offs

- A trace model may be verbose. Keep operation kinds small and focused on block/value/control-flow structure first.
- Later backend-specific adapters may need additional operation metadata; they can extend the finite operation set without changing the core non-claims.
