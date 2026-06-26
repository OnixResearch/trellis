# Draft → Landed Scope Mapping

The original `wasm-primitives` draft covered a much broader semantic surface:
values, stacks, locals, memory, tables, indirect calls, control labels, and
validation. The landed archive narrows that to the smallest self-contained Wasm
MVP slice that matches this repo's current proof style.

## Landed now

- exact Wasm value tags for `i32`, `i64`, `f32`, and `f64`
- operand-stack discipline for the modeled `i32.add` instruction
- typed local-frame construction plus `local.get`, `local.set`, and `local.tee`
- bounded byte-window linear-memory load/store helpers
- `memory.grow` with 64KiB pages, unchanged old prefix, and zero-filled extension
- synced requirements, canonical spec, README/lib.rs updates, and archive evidence

## Explicitly deferred from the original draft

- control labels, branch-depth resolution, and return arity
- unreachable-stack polymorphism and instruction-sequence validation judgments
- tables and `call_indirect`
- binary decoding, module parsing, imports/exports, and instantiation
- floating-point arithmetic semantics beyond exact value tags

## Why the scope changed

The full draft was larger than the archiveable slices that have landed cleanly in
this repo. The narrowed slice still adds a meaningful verified Wasm semantic
core without pretending to finish tables, control flow, and validation in the
same change.
