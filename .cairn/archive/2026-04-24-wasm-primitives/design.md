## Context

The landed slice follows the repo's normal pattern: one self-contained module
with value-level state, small exec wrappers, exact ensures, a few local proof
helpers, and runtime tests. The modeled Wasm instruction subset is intentionally
small and explicit:

- `i32.add`
- `local.get`
- `local.set`
- `local.tee`
- bounded byte-window memory load/store
- `memory.grow`

Everything else from the earlier broad draft is deferred out of scope for this
archive.

## Goals / Non-Goals

**Goals:**
- model exact value tags for `i32`, `i64`, `f32`, and `f64`
- model operand-stack discipline for `i32.add`
- model typed local frame construction and `local.get` / `local.set` / `local.tee`
- model linear memory as bytes plus page count with exact in-bounds window rules
- model `memory.grow` with monotone page count, unchanged prefix, and zero-filled extension
- sync requirements, README, canonical spec, and archive evidence to the landed slice

**Non-Goals:**
- control stack, branch-depth resolution, and return arity
- unreachable-stack polymorphism or whole-sequence validation
- tables and indirect calls
- binary decoding, linking, host embedding, or a full interpreter loop
- float arithmetic / NaN semantics beyond exact type tags

## Decisions

### 1. Land one module, not a family of `wasm_*` modules

**Choice:** implement `src/wasm_primitives.rs` as one small, reviewable MVP
module.

**Rationale:** the remaining active change must archive cleanly. One module keeps
proofs, tests, Tracey tags, and README/spec sync aligned with the repo's recent
successful archive pattern.

### 2. Value typing stays explicit and tag-based

**Choice:** represent core Wasm values as tagged enums:
`I32(u32) | I64(u64) | F32(u32) | F64(u64)`.

**Rationale:** exact tag matching is the core useful property for this slice,
and opaque bit-pattern floats avoid dragging in IEEE/NaN proof work.

### 3. Stack discipline is bounded to `i32.add`

**Choice:** the modeled arithmetic instruction subset is exactly one binary op:
`i32.add`.

**Rationale:** this is enough to prove the main operand-stack consume/produce
rule without opening a full validator or numeric opcode surface.

### 4. Locals use a typed frame with parallel type/value vectors

**Choice:** model locals as a `LocalFrame { types, values }` where well-formedness
means equal lengths and per-slot type agreement.

**Rationale:** this makes `local.get`, `local.set`, and `local.tee` precise:
index bounds, exact returned type, and unchanged non-target locals.

### 5. Linear memory is bytes plus page count

**Choice:** represent memory as `LinearMemory { bytes, pages }` with the invariant
`bytes.len() == pages * 65536` whenever that product fits in `usize`.

**Rationale:** exact byte-window load/store and prefix-preservation proofs are
most direct over byte sequences.

### 6. Use the canonical Wasm page cap with a representability guard

**Choice:** use `WASM_MAX_PAGES = 65536` and additionally require the computed
byte length to fit in `usize`.

**Rationale:** this keeps the model aligned with the Wasm 64KiB page abstraction
without pretending every host integer width can materialize the full 4GiB byte
array.

### 7. Public API stays exec-first, with specs as the paired proof surface

**Choice:** expose public exec helpers for value tags, local-frame operations,
`i32.add`, and memory operations. Keep the paired public spec predicates in the
same module as proof-facing companions to that exec surface rather than
promising a separate spec-only API layer.

**Rationale:** this matches the repo's usual archive shape: a small runtime
surface backed by exact paired specs and proofs. It also keeps README and
Tracey module descriptions aligned with the live exec API.

### 8. Verification strategy is explicit

**Choice:** verify this slice with:
- local proof helpers inside `src/wasm_primitives.rs`
- `r[impl]` annotations on every public exec function in the module
- unit tests for happy-path and reject-path cases
- proptests for memory round-trips and `memory.grow` prefix preservation
- `./scripts/verus.sh src/wasm_primitives.rs` for the focused module proof
- `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, and Tracey queries for integration
- `./autoresearch.sh` for the final benchmark + correctness gate

**Rationale:** this matches the repo's stable proof/test/archive workflow and
avoids the unrelated `./scripts/verify.sh` blocker in `src/instruction_selection.rs`.

## Risks / Trade-offs

**[Broader Wasm draft is deferred]** -> Accept. Archiving the self-contained MVP
slice is better than leaving one overly broad active change permanently open.

**[Only one arithmetic instruction is modeled]** -> Accept. The goal here is
operand-stack discipline, not arithmetic coverage.

**[No validator or control stack]** -> Accept. Those surfaces should land as
separate future changes once this minimal semantic core is in-tree.
