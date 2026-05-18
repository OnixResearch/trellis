# WebAssembly MVP Primitives Specification

## Purpose

Defines requirements for a verified WebAssembly MVP semantic slice: exact value
tags, operand-stack discipline for `i32.add`, typed local-variable access, and
linear-memory bounds plus page growth. This specification intentionally excludes
control labels, validation judgments, tables, indirect calls, and binary
decoding.

## Requirements

### Requirement: Typed Values [r[wasm-value.types]]
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

### Requirement: Operand Stack Discipline [r[wasm-stack.discipline]]
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

### Requirement: Local Access Typing [r[wasm-locals.access]]
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

### Requirement: Linear Memory Bounds [r[wasm-memory.bounds]]
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

### Requirement: Memory Growth Preserves Prefix [r[wasm-memory.grow]]
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
