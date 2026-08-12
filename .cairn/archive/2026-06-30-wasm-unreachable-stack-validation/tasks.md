## Phase 1: Validation-state model

- [x] [serial] Add Wasm validation state and type-stack suffix predicates. r[wasm-validation.unreachable-stack]
- [x] [serial] Add `unreachable` and `i32.add` validation helpers with reachable and unreachable tests. r[wasm-validation.i32-add]
- [x] [serial] Add `br` and `return` validation helpers with positive and negative tests. r[wasm-validation.control-transfer]

## Phase 2: Spec and catalog sync

- [x] [serial] Add canonical requirements and scenarios for unreachable-stack validation and control-transfer reachability. r[wasm-validation.unreachable-stack]
- [x] [serial] Update README and crate docs to include the validation-state slice. r[wasm-validation.control-transfer]

## Phase 3: Validation and archive

- [x] [serial] Run focused Wasm tests and Verus proof checks, recording evidence. r[wasm-validation.i32-add]
- [x] [serial] Run full cargo/clippy/Tracey/ensures validation for the touched requirements. r[wasm-validation.control-transfer]
- [x] [serial] Sync accepted specs, archive this change, and commit completed evidence. r[wasm-validation.unreachable-stack]
