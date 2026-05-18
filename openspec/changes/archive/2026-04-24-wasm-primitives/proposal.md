## Why

This repo still has no verified WebAssembly-specific core even though it already
contains many small verified state, memory, and stack primitives. The original
`wasm-primitives` draft tried to land a full semantic layer at once: values,
stacks, locals, memory, tables, indirect calls, control labels, and validation.
That scope is larger than the archiveable slices that have landed cleanly in
this repo.

To finish the remaining active change without cheating on the benchmark, this
change now lands the smallest self-contained Wasm primitive slice that matches
the repo's current proof style: exact value tags, operand-stack discipline for a
single modeled arithmetic instruction (`i32.add`), typed local access
(`local.get` / `local.set` / `local.tee`), and linear-memory bounds plus
`memory.grow` prefix preservation.

## What Changes

- Add `src/wasm_primitives.rs` with:
  - exact WebAssembly value tags for `i32`, `i64`, `f32`, and `f64`
  - operand-stack discipline for the modeled `i32.add` instruction
  - typed local frame construction plus `local.get`, `local.set`, and `local.tee`
  - bounded byte-window memory load/store and monotone page growth
- Add matching `r[wasm-*]` requirements to `docs/requirements.md`
- Add the canonical `openspec/specs/wasm/spec.md`
- Update `src/lib.rs` and `README.md` for the new module and live counts
- Archive this narrowed, implemented Wasm MVP slice with fresh evidence

## Capabilities

### New Capabilities

- `wasm-value.types`: Verified exact type tags for core Wasm values
- `wasm-stack.discipline`: Verified `i32.add` consumes two `i32`s and produces one `i32`
- `wasm-locals.access`: Verified typed `local.get`, `local.set`, and `local.tee`
- `wasm-memory.bounds`: Verified exact in-bounds byte-window memory access
- `wasm-memory.grow`: Verified monotone page growth with unchanged old prefix and zero-filled extension

## Impact

- **Files**: New `src/wasm_primitives.rs`, requirement updates, canonical Wasm spec sync, and archive evidence
- **APIs**: Public value-tag, local-frame, stack-effect, and memory helper surface for the Wasm MVP slice
- **Dependencies**: No new runtime dependencies
- **Testing / Verification**:
  - `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet wasm_`
  - `./scripts/verus.sh src/wasm_primitives.rs`
  - `./scripts/verus.sh src/lib.rs`
  - `./scripts/check-ensures.sh`
  - `tracey query uncovered`
  - `tracey query untested`
  - full `./autoresearch.sh` + `autoresearch.checks.sh`

## Out of Scope

- control labels, branches, returns, and unreachable-stack polymorphism
- validation judgments over instruction sequences
- tables and `call_indirect`
- binary decoding, module parsing, imports/exports, and instantiation
- floating-point arithmetic semantics beyond exact value tags
- threads, SIMD, exceptions, GC, and component-model features
