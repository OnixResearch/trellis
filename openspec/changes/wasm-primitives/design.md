## Context

This repo verifies small primitives as value-level models: a `spec fn`
for the rule, an exec wrapper with matching ensures, proof lemmas for
invariants, and runtime tests for concrete or randomized scenarios. A
WebAssembly change should follow same pattern. Goal is not a full engine.
Goal is a compact verified core for typed values, operand/control stack
rules, locals, linear memory, tables, indirect calls, and validation for
a tractable instruction subset.

## Goals / Non-Goals

**Goals:**
- Model exact Wasm value types and typed stack elements
- Model operand-stack consume/produce rules plus unreachable-stack polymorphism
- Model local variable get/set/tee with index and type checks
- Model linear memory bounds, exact-byte effects, and 64KiB page growth
- Model table access and indirect-call signature checks
- Model structured control labels, branch-depth resolution, and return arity
- Model validation rules that prevent operand-stack underflow on the supported subset
- Add cross-module proofs connecting validation to memory, branch, and indirect-call safety
- Add matching Tracey requirements in `docs/requirements.md` when implementation starts

**Non-Goals:**
- Binary decoding or full module loader semantics
- Full numeric instruction set, especially exact floating-point/NaN behavior
- Imports/exports, host functions, or embedder state
- Threads, SIMD, exceptions, GC, and component-model features
- Whole-engine scheduling, JIT, or optimizer correctness

## Decisions

### 1. Scope stays at the Wasm MVP semantic core

**Choice:** Model the execution primitives other verified code composes
with: values, stacks, locals, memory, tables, indirect calls, control
labels, and validation. Leave parsing, linking, and host integration out.

**Rationale:** This matches repo style. It keeps the change useful for
sandbox and validator reasoning without dragging in a whole runtime.

### 2. Execution state stays value-level and decomposed

**Choice:** Represent state as plain structs and sequences: typed values,
an operand stack, a control-stack of labels, a local vector, linear
memory bytes, and a table of optional function references.

**Rationale:** Sequence-based models fit Verus well in this repo. They
also compose with existing collection and bounds proofs.

### 3. Unreachable stack polymorphism is explicit state

**Choice:** Model validation with a distinguished unreachable stack mode
instead of trying to encode polymorphism as ordinary stack contents.

**Rationale:** WebAssembly validation depends on this rule. Making it
explicit keeps branch/block typing proofs honest and avoids ad hoc
special cases in every instruction rule.

### 4. Linear memory is bytes plus page count

**Choice:** Represent memory as `Seq<u8>` with a page-size constant of
65536 bytes and expose load/store as bounded byte-window operations.
`memory.grow` returns the previous page count on success and preserves the
existing prefix.

**Rationale:** Byte sequences make exact affected-range proofs direct.
Page counts capture Wasm growth semantics better than raw byte lengths
alone.

### 5. Tables use optional function references plus a separate type map

**Choice:** Represent tables as `Seq<Option<FuncIdx>>` and maintain a
separate finite mapping from function indices to function signatures.
`call_indirect` checks both slot presence and exact type equality.

**Rationale:** This separates table occupancy from function typing. It is
small, deterministic, and enough to prove that validated indirect calls do
not jump to the wrong signature.

### 6. Validation and execution stay separate but linked

**Choice:** Give each modeled instruction both a validation rule and an
execution rule, then prove cross-module properties saying validated uses
preserve stack typing or trap in the specified ways.

**Rationale:** This mirrors WebAssembly's design: static validation rules
prevent malformed stack manipulations, while runtime still handles traps
like out-of-bounds memory or null table entries.

## Risks / Trade-offs

**[Exact float semantics omitted]** -> Accept. Initial value model can
carry float types as tagged values or opaque bit patterns later, without
promising IEEE/NaN corner-case proofs now.

**[Validation subset smaller than full Wasm]** -> Accept. The change is
still useful if it covers the primitive instructions other proofs depend
on first.

**[No full interpreter state machine]** -> Accept. The repo usually
verifies local primitives and composition lemmas, not monolithic machine
executions.
