# WebAssembly MVP Primitives Specification

## Purpose

Defines the landed verified WebAssembly MVP semantic slice: exact value tags,
operand-stack discipline for `i32.add`, typed local-variable access,
linear-memory bounds plus page growth, typed control labels for branch/return
arity, unreachable-stack polymorphism, and structured control-frame stack-height
restoration. This canonical spec intentionally excludes whole module validation,
tables, indirect calls, and binary decoding.

## Requirements

### Requirement: Typed Values

r[wasm-value.types]
Core Wasm values MUST carry exact value-type information. A value tagged `i32`,
`i64`, `f32`, or `f64` MUST be accepted only when that exact type is required.

#### Scenario: Exact type match

- GIVEN a value tagged `i32`
- WHEN a helper requires an `i32`
- THEN the value is accepted

#### Scenario: Type mismatch rejected

- GIVEN a value tagged `i32`
- WHEN a helper requires an `i64`
- THEN the value is rejected as a type mismatch

### Requirement: Operand Stack Discipline

r[wasm-stack.discipline]
The modeled `i32.add` instruction MUST consume exactly two `i32` operands from
the operand-stack suffix and MUST produce exactly one `i32` result.

#### Scenario: Two i32 operands produce one i32 result

- GIVEN an operand stack ending in `[i32, i32]`
- WHEN `i32.add` executes
- THEN those two operands are consumed and one `i32` result is pushed

#### Scenario: Underflow or mismatch is rejected

- GIVEN an operand stack with fewer than two operands or a non-`i32` suffix
- WHEN `i32.add` executes
- THEN the instruction is rejected and the stack is unchanged

### Requirement: Local Access Typing

r[wasm-locals.access]
`local.get`, `local.set`, and `local.tee` MUST access only in-bounds local
indices and MUST preserve the declared type of each local slot.

#### Scenario: local.get returns declared type

- GIVEN local slot `j` has declared type `f64`
- WHEN `local.get j` executes
- THEN it returns an `f64` value

#### Scenario: local.set updates one slot

- GIVEN local slot `j` is in bounds and a replacement value has the declared type for `j`
- WHEN `local.set j` executes
- THEN slot `j` is updated and all non-`j` locals are unchanged

#### Scenario: local.tee returns the written value

- GIVEN local slot `j` is in bounds and a replacement value has the declared type for `j`
- WHEN `local.tee j` executes
- THEN slot `j` is updated and that same value is returned

#### Scenario: Out-of-bounds or wrong-type update is rejected

- GIVEN local slot `j` is out of bounds or the replacement value has the wrong type
- WHEN `local.set j` or `local.tee j` executes
- THEN the operation is rejected and the frame is unchanged

### Requirement: Linear Memory Bounds

r[wasm-memory.bounds]
A memory load or store MUST succeed only when every addressed byte lies within
linear-memory bounds. Out-of-bounds access MUST fail without modifying memory.

#### Scenario: In-bounds load returns exact byte window

- GIVEN linear memory length `n` and a load of width `w` at address `a`
- WHEN `a + w <= n`
- THEN the load succeeds and returns exactly bytes `[a, a + w)`

#### Scenario: Out-of-bounds store is a no-op

- GIVEN linear memory length `n` and a store of width `w` at address `a`
- WHEN `a + w > n`
- THEN the store fails and memory is unchanged

### Requirement: Memory Growth Preserves Prefix

r[wasm-memory.grow]
`memory.grow` MUST increase linear memory only in whole 64KiB pages. On
success, old memory contents MUST remain unchanged as a prefix of the grown
memory and the appended bytes MUST be zero. On failure, memory MUST remain
unchanged.

#### Scenario: Successful grow preserves prefix

- GIVEN memory with `p` pages
- WHEN `memory.grow delta` succeeds
- THEN the result reports previous size `p`, new size is `p + delta`, and the old bytes remain unchanged as a prefix

#### Scenario: Successful grow zero-fills extension

- GIVEN a successful `memory.grow delta`
- WHEN the new suffix is inspected
- THEN every newly appended byte is zero

#### Scenario: Failed grow is a no-op

- GIVEN a memory state for which growth is rejected
- WHEN `memory.grow` fails
- THEN the page count and byte contents are unchanged

### Requirement: Control Labels

r[wasm-control.labels]
Wasm control labels MUST distinguish function, block, and loop targets. A
branch to a block or function label MUST use that label's result types as the
expected operand suffix. A branch to a loop label MUST use that label's
parameter types as the expected operand suffix.

#### Scenario: Block label expects results

- GIVEN a block label with result type `i32`
- WHEN branch target types are selected for that label
- THEN the expected suffix is `[i32]`

#### Scenario: Loop label expects parameters

- GIVEN a loop label with parameter type `i64` and result type `i32`
- WHEN branch target types are selected for that label
- THEN the expected suffix is `[i64]`

### Requirement: Branch Arity

r[wasm-control.branch-arity]
A modeled branch MUST be admitted only when its depth names an in-bounds control
label and the operand-stack suffix exactly matches the target label types. Depth
`0` MUST name the innermost label, represented by the last control-stack entry.

#### Scenario: In-bounds branch with matching suffix is admitted

- GIVEN a control stack ending in a block label with result type `i32`
- WHEN a branch of depth `0` is checked against an operand stack ending in `i32`
- THEN the branch is admitted

#### Scenario: Out-of-bounds branch is rejected

- GIVEN a control stack with one label
- WHEN a branch of depth `1` is checked
- THEN the branch is rejected

#### Scenario: Branch suffix mismatch is rejected

- GIVEN a target label expecting `i32`
- WHEN a branch is checked against an operand stack ending in `i64`
- THEN the branch is rejected

### Requirement: Return Arity

r[wasm-control.return-arity]
A modeled return MUST be admitted only when the control stack has a root
function label and the operand-stack suffix exactly matches that function
label's result types. Empty control stacks or non-function roots MUST be
rejected.

#### Scenario: Matching function result suffix is admitted

- GIVEN a root function label with result types `[i32, f64]`
- WHEN return arity is checked against an operand stack ending in `[i32, f64]`
- THEN the return is admitted

#### Scenario: Missing function label is rejected

- GIVEN an empty control stack or a control stack whose root is not a function label
- WHEN return arity is checked
- THEN the return is rejected

#### Scenario: Return suffix mismatch is rejected

- GIVEN a root function label with result type `i32`
- WHEN return arity is checked against an operand stack ending in `i64`
- THEN the return is rejected

### Requirement: Unreachable Stack Validation State

r[wasm-validation.unreachable-stack]
The Wasm validation model MUST track an operand-type stack, a control-label
stack, and whether the current path is reachable. When the path is reachable,
input-pop checks MUST require the operand-type stack suffix to exactly match
expected types. When the path is unreachable, input-pop checks MUST be admitted
without requiring those suffix types.

#### Scenario: Reachable suffix mismatch is rejected

- GIVEN a reachable validation state with an operand-type stack ending in `i64`
- WHEN an instruction requires an `i32` suffix
- THEN the input-pop check is rejected

#### Scenario: Unreachable suffix mismatch is admitted

- GIVEN an unreachable validation state with an empty operand-type stack
- WHEN an instruction requires an `i32` suffix
- THEN the input-pop check is admitted

### Requirement: I32 Add Validation

r[wasm-validation.i32-add]
The Wasm validation model MUST admit `i32.add` only when reachable state has two
trailing `i32` operand types, or when the current path is unreachable. On
success, validation MUST record one produced `i32` result type.

#### Scenario: Reachable i32.add consumes two i32 types

- GIVEN a reachable validation state whose operand-type stack ends in `[i32, i32]`
- WHEN `i32.add` validation succeeds
- THEN those two input types are replaced by one `i32` result type

#### Scenario: Unreachable i32.add is stack-polymorphic

- GIVEN an unreachable validation state with no operand types
- WHEN `i32.add` validation runs
- THEN validation succeeds and records one `i32` result type

### Requirement: Control Transfer Validation

r[wasm-validation.control-transfer]
The Wasm validation model MUST validate `br` and `return` against the existing
control-label arity rules while the current path is reachable. Successful `br`,
`return`, and `unreachable` validation MUST mark the current path unreachable.
Malformed branch depths, empty control stacks, non-function return roots, and
reachable suffix mismatches MUST be rejected.

#### Scenario: Branch validation marks path unreachable

- GIVEN a reachable validation state with a branch target whose expected suffix matches the operand-type stack
- WHEN branch validation succeeds
- THEN the validation state becomes unreachable

#### Scenario: Return validation marks path unreachable

- GIVEN a reachable validation state with a function root whose result suffix matches the operand-type stack
- WHEN return validation succeeds
- THEN the validation state becomes unreachable

#### Scenario: Malformed control transfer is rejected

- GIVEN an out-of-range branch depth or a return state without a function root
- WHEN control-transfer validation runs
- THEN validation is rejected and reachability is unchanged

### Requirement: Control Frame State

r[wasm-validation.control-frame]
The Wasm validation model MUST record structured control frames with label kind,
parameter types, result types, entry operand-stack height, and entry
reachability. The control-frame stack MUST stay aligned with the branch-label
stack for helpers that enter or end modeled blocks and loops.

#### Scenario: Entering a block records a frame

- GIVEN a validation state with a reachable operand-type stack
- WHEN a block is entered with matching parameter types
- THEN a control frame and branch label are pushed with the post-parameter-pop entry height

#### Scenario: Ending a frame removes aligned metadata

- GIVEN a validation state with one innermost control frame and matching label
- WHEN the frame ends successfully
- THEN the innermost frame and label are removed

### Requirement: Block and Loop Entry Validation

r[wasm-validation.block-entry]
The Wasm validation model MUST admit block and loop entry only when reachable
state has a matching parameter suffix, or when the current path is unreachable.
Successful entry MUST remove the consumed parameter types, push an aligned
control frame and branch label, and keep the current reachability unchanged.

#### Scenario: Reachable block entry consumes parameters

- GIVEN a reachable validation state whose operand-type stack ends in the declared block parameters
- WHEN the block is entered
- THEN those parameter types are removed and the frame records the resulting stack height

#### Scenario: Reachable loop entry rejects parameter mismatch

- GIVEN a reachable validation state whose operand-type stack does not match the declared loop parameters
- WHEN the loop is entered
- THEN entry is rejected and the validation state is unchanged

#### Scenario: Unreachable entry is stack-polymorphic

- GIVEN an unreachable validation state without the declared parameters
- WHEN a block is entered
- THEN entry succeeds and records the current stack height

### Requirement: Block End Validation

r[wasm-validation.block-end]
The Wasm validation model MUST admit ending the innermost frame only when
reachable state has a result-type suffix matching that frame, or when the current
path is unreachable. Successful end MUST pop the frame and aligned label,
restore the operand-type stack to the frame entry height, append the frame result
types, and restore reachability to the frame's entry reachability.

#### Scenario: Reachable block end restores height plus results

- GIVEN an innermost block frame with entry height `h` and result type `i32`
- WHEN the reachable operand stack ends in `i32` and the frame ends
- THEN the operand stack is restored to the first `h` types followed by `i32`

#### Scenario: Reachable block end rejects result mismatch

- GIVEN an innermost block frame expecting result type `i32`
- WHEN the reachable operand stack ends in `i64`
- THEN the end is rejected and validation state is unchanged

#### Scenario: Unreachable block end synthesizes results

- GIVEN an unreachable state inside a block frame with result type `i32`
- WHEN the frame ends
- THEN validation succeeds, restores the frame entry height, appends `i32`, and restores entry reachability

### Requirement: Table Signature Admission

r[wasm-table.signature]
The Wasm table model MUST represent a finite sequence of optional function signatures. A table entry MUST be admitted for indirect-call use only when the selected index is in bounds, initialized, and its parameter and result type sequences exactly match the expected function type.

#### Scenario: Matching initialized table entry is admitted

- GIVEN a finite table with an initialized function entry
- WHEN the entry's parameter and result types exactly match the expected function type
- THEN table-entry signature admission succeeds

#### Scenario: Invalid table entry is rejected

- GIVEN an out-of-range table index, an uninitialized table slot, or a function entry with mismatched parameter or result types
- WHEN table-entry signature admission is checked
- THEN admission is rejected without claiming callee execution behavior

### Requirement: Call Indirect Validation

r[wasm-validation.call-indirect]
The Wasm validation model MUST admit modeled `call_indirect` only when table-entry signature admission succeeds and, while reachable, the operand-type stack suffix exactly matches the expected function parameters followed by an `i32` table-index operand. Successful reachable validation MUST consume that suffix and push the expected result types. Successful unreachable validation MUST keep operand pops stack-polymorphic but still require a matching table entry and synthesize the expected result types.

#### Scenario: Reachable indirect call consumes arguments and index

- GIVEN a reachable validation state whose operand-type stack ends with expected parameters followed by `i32`
- WHEN `call_indirect` validation uses a matching initialized table entry
- THEN validation succeeds, consumes the parameter/index suffix, and appends the expected result types

#### Scenario: Reachable indirect call rejects stack mismatch

- GIVEN a reachable validation state whose operand-type stack does not end with expected parameters followed by `i32`
- WHEN `call_indirect` validation is checked
- THEN validation is rejected and the validation state is unchanged

#### Scenario: Unreachable indirect call still checks the table

- GIVEN an unreachable validation state and a finite table entry
- WHEN `call_indirect` validation is checked
- THEN operand suffix checks are skipped, but out-of-range, uninitialized, or signature-mismatched table entries are still rejected

### Requirement: Branch Table Validation

r[wasm-validation.br-table]
The Wasm validation model MUST admit modeled `br_table` only when the default branch depth is in bounds, every listed branch depth is in bounds, every listed target's branch-label types exactly match the default target's branch-label types, and, while reachable, the operand-type stack suffix exactly matches the default target types followed by an `i32` selector. Successful validation MUST mark the current path unreachable. Unreachable validation MUST keep operand pops stack-polymorphic but still reject malformed depths or mismatched target types.

#### Scenario: Matching branch table targets are admitted

- GIVEN a validation state with block and loop labels whose branch-label types match the default target
- WHEN `br_table` validation checks listed depths and the default depth
- THEN validation succeeds and marks the current path unreachable

#### Scenario: Branch table rejects malformed targets

- GIVEN a branch table with an out-of-range listed depth, out-of-range default depth, or a listed target whose branch-label types differ from the default target
- WHEN `br_table` validation is checked
- THEN validation is rejected and the validation state is unchanged

#### Scenario: Unreachable branch table still checks targets

- GIVEN an unreachable validation state and a branch table candidate
- WHEN `br_table` validation is checked
- THEN operand suffix checks are skipped, but malformed target depths or mismatched target types are still rejected
