## Phase 1: Values, Stack, and Locals

- [ ] Add `wasm_value` with value types, typed values, and exact-type predicates
- [ ] Add `wasm_stack` with push/pop helpers, stack-effect specs, and unreachable-stack state
- [ ] Add `wasm_locals` with `local.get`, `local.set`, and `local.tee` semantics
- [ ] Prove value typing, stack discipline, polymorphic-unreachable, and local-access invariants

## Phase 2: Memory, Tables, and Indirect Calls

- [ ] Add `wasm_memory` with bounded load/store and 64KiB page-growth semantics
- [ ] Add `wasm_table` with bounded get/set over optional function references
- [ ] Add `wasm_call_indirect` with table-entry presence and exact-signature checks
- [ ] Prove bounds, exact affected-range, prefix-preservation, and signature-match properties

## Phase 3: Control Flow and Validation

- [ ] Add `wasm_control` with label stack, branch-depth resolution, and return-arity rules
- [ ] Add `wasm_validation` for the modeled instruction subset
- [ ] Prove validated instruction sequences cannot underflow the operand stack
- [ ] Prove cross-module properties for validated memory ops, indirect calls, and branch stack restoration

## Phase 4: Requirement Coverage and Tests

- [ ] Add `wasm-*` and `properties.wasm-*` requirements to `docs/requirements.md`
- [ ] Add Tracey annotations across the new `src/wasm_*.rs` modules and property proofs
- [ ] Add unit tests for representative Wasm scenarios and proptest model oracles
- [ ] Verify the tree, sync the change, and archive it after implementation completes
