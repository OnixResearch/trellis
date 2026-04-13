# WebAssembly Primitives Specification

## Purpose

Defines requirements for a verified model of core WebAssembly execution
primitives: typed values, operand-stack discipline, local variable
access, linear memory, tables, indirect calls, structured control flow,
and validation rules for a supported Wasm instruction subset. Scope is
semantic behavior and preserved invariants, not binary decoding or full
engine implementation.

## Requirements

### Requirement: Typed Values

r[wasm-value.types]
WebAssembly values MUST carry exact value-type information. A value typed
as `i32`, `i64`, `f32`, or `f64` MUST NOT be accepted where a different
value type is required.

#### Scenario: Exact type match

- GIVEN a value tagged `i32`
- WHEN an instruction requires an `i32`
- THEN the value is accepted

#### Scenario: Type mismatch rejected

- GIVEN a value tagged `i32`
- WHEN an instruction requires an `i64`
- THEN validation or execution rejects the use as a type mismatch

### Requirement: Operand Stack Discipline

r[wasm-stack.discipline]
Each modeled instruction MUST consume exactly the operand types declared
by its typing rule and MUST produce exactly the result types declared by
that rule.

#### Scenario: Binary instruction consumes two operands

- GIVEN an operand stack ending in `[i32, i32]`
- WHEN a modeled `i32` binary instruction executes
- THEN those two operands are consumed and one `i32` result is pushed

#### Scenario: Instruction with insufficient operands is invalid

- GIVEN an operand stack with fewer operands than an instruction requires
- WHEN validation checks the instruction
- THEN the instruction sequence is rejected

### Requirement: Unreachable Stack Polymorphism

r[wasm-stack.polymorphic-unreachable]
Inside an unreachable control-flow region, validation MUST treat the
operand stack as polymorphic until control reaches the matching enclosing
label. This polymorphism MUST NOT escape the enclosing structured block.

#### Scenario: Unreachable region permits temporary stack mismatch

- GIVEN validation has entered an unreachable tail of a block
- WHEN a modeled instruction expects operands not present on the concrete stack
- THEN validation may continue in unreachable mode

#### Scenario: Block exit restores declared shape

- GIVEN an unreachable branch targets a label with result arity `k`
- WHEN control leaves the block
- THEN the stack shape at the target matches that label's declared result types

### Requirement: Local Access Typing

r[wasm-locals.access]
`local.get`, `local.set`, and `local.tee` MUST access only in-bounds local
indices and MUST preserve the declared type of each local slot.

#### Scenario: local.get returns declared type

- GIVEN local slot `j` has declared type `i64`
- WHEN `local.get j` executes
- THEN it pushes an `i64` value onto the operand stack

#### Scenario: local.set updates one slot

- GIVEN local slot `j` is in bounds and the stack top has the declared type for `j`
- WHEN `local.set j` executes
- THEN slot `j` is updated and all non-`j` locals are unchanged

### Requirement: Linear Memory Bounds

r[wasm-memory.bounds]
A load or store MUST succeed only when every addressed byte lies within
the current linear-memory bounds. Out-of-bounds access MUST trap.

#### Scenario: In-bounds load succeeds

- GIVEN linear memory length `n` and a load of width `w` at address `a`
- WHEN `a + w <= n`
- THEN the load succeeds and reads exactly bytes `[a, a + w)`

#### Scenario: Out-of-bounds store traps

- GIVEN linear memory length `n` and a store of width `w` at address `a`
- WHEN `a + w > n`
- THEN the store traps and memory is unchanged

### Requirement: Memory Growth Preserves Prefix

r[wasm-memory.grow]
`memory.grow` MUST increase linear memory only in whole 64KiB pages. On
success, the previous memory contents MUST remain unchanged as a prefix of
the grown memory. On failure, memory MUST remain unchanged.

#### Scenario: Grow appends zeroed pages

- GIVEN memory with `p` pages
- WHEN `memory.grow` by `delta > 0` succeeds
- THEN the result reports previous size `p`, new size is `p + delta`, and the old bytes remain unchanged as a prefix

#### Scenario: Failed grow is no-op

- GIVEN a memory state for which growth is rejected
- WHEN `memory.grow` fails
- THEN the page count and byte contents are unchanged

### Requirement: Table Bounds

r[wasm-table.bounds]
Table access and update MUST use only in-bounds table indices. An
out-of-bounds `table.get` or `table.set` MUST trap or fail without
modifying other slots.

#### Scenario: In-bounds set updates one slot

- GIVEN a table of length `n` and index `i < n`
- WHEN `table.set i ref` executes
- THEN slot `i` becomes `ref` and all other slots are unchanged

#### Scenario: Out-of-bounds get traps

- GIVEN a table of length `n` and index `i >= n`
- WHEN `table.get i` executes
- THEN the access traps

### Requirement: Indirect Call Signature Match

r[wasm-call-indirect.signature]
`call_indirect` MUST succeed only if the selected table entry is present
and the referenced function's signature exactly matches the expected type
at the call site. Missing entries or mismatched signatures MUST trap.

#### Scenario: Matching entry dispatches

- GIVEN table slot `i` contains function `f`
- AND `f` has the exact expected function type
- WHEN `call_indirect i` executes
- THEN control transfers to `f`

#### Scenario: Mismatched signature traps

- GIVEN table slot `i` contains function `f`
- AND `f`'s signature differs from the expected type
- WHEN `call_indirect i` executes
- THEN the instruction traps

### Requirement: Branch Target Arity

r[wasm-control.labels]
A branch with depth `d` MUST target an existing control label at depth `d`
and MUST supply exactly that label's declared result arity.

#### Scenario: Branch resolves valid label

- GIVEN a control stack with at least `d + 1` labels
- WHEN `br d` executes
- THEN it targets the label at depth `d`

#### Scenario: Branch with wrong arity is invalid

- GIVEN a label expecting `k` result values
- WHEN validation checks a branch that provides a different number of values
- THEN the branch is rejected

### Requirement: Function Return Arity

r[wasm-control.return]
A `return` MUST consume exactly the function's declared result values from
the operand stack and leave the caller-visible result sequence in that
order.

#### Scenario: Single-result return

- GIVEN a function declared to return one `i32`
- WHEN `return` executes with an `i32` on top of the operand stack
- THEN that value becomes the function result

#### Scenario: Missing result is invalid

- GIVEN a function declared to return `k` values
- WHEN validation checks a `return` with fewer than `k` values available
- THEN the function body is rejected

### Requirement: Validation Prevents Stack Underflow

r[wasm-validation.stack-safety]
For the supported instruction subset, validation MUST reject any
instruction sequence that could underflow the operand stack at runtime.

#### Scenario: Well-typed sequence validates

- GIVEN an instruction sequence whose stack effects compose correctly
- WHEN validation runs
- THEN the sequence is accepted

#### Scenario: Underflowing sequence rejected

- GIVEN an instruction that requires two operands after a sequence that leaves one
- WHEN validation runs
- THEN the sequence is rejected

## Cross-Module Properties

### Requirement: Validated Memory Operations Are Safe

r[properties.wasm-validated-memory]
A validated memory instruction MUST either execute with an in-bounds byte
window and preserve operand typing or trap according to
`wasm-memory.bounds`; it MUST NOT read or write outside linear memory.

### Requirement: Validated Indirect Calls Match Signatures

r[properties.wasm-indirect-call-safe]
A validated indirect call site MUST either dispatch to a present table
entry whose function type exactly matches the expected signature or trap;
it MUST NOT dispatch to a function with a different signature.

### Requirement: Branches Restore Target Stack Shape

r[properties.wasm-branch-stack]
For any validated branch or return, the values transferred to the target
label or function exit MUST match that target's declared result types and
arity exactly.
