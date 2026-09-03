## Why

The rustc-dev-guide backend chapters identify monomorphization as the first backend step: collect concrete instances and statics, deduplicate them, and partition them into codegen units. Trellis has backend-frame and instruction-selection primitives, but no finite primitive for mono-item collection or CGU partitioning.

This change defines bounded monomorphization metadata and partition-admission facts without claiming actual machine-code generation or full rustc instance resolution.

## What Changes

- Add a `rustc-monomorphization` Cairn spec.
- Add a `src/instance_partition/` component with finite item, reachability, instantiation, and output-unit partition specs.
- Define root items, call/reference edges, concrete instance metadata, statics, shims, deduplication, and stable/volatile CGU partitioning.
- Require positive and negative tests for reachable generic instances, duplicate elimination, missing reachable items, wrong type-argument metadata, and invalid partition overlaps.

## Impact

- **Files**: implementation touches `src/instance_partition/`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and tests.
- **Testing**: implementation must include positive and negative finite graph/partition fixtures.
- **Boundaries**: no claim about trait solving, full generic substitution, linker behavior, LTO, machine code, binary size, or compile-time performance.
