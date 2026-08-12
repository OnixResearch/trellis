## Phase 1: Branch-table target model

- [x] [serial] Add finite branch-table target matching specs over listed depths and a default depth. r[wasm-validation.br-table]
- [x] [serial] Add helpers/tests for matching targets, out-of-range depths, and target-type mismatches. r[wasm-validation.br-table]

## Phase 2: Validation helper

- [x] [serial] Add reachable `br_table` validation requiring default target operands plus an `i32` selector. r[wasm-validation.br-table]
- [x] [serial] Add unreachable `br_table` validation that skips operand suffix checks but still rejects malformed target metadata. r[wasm-validation.br-table]
- [x] [serial] Add positive and negative runtime tests for accepted tables, stack mismatch, bad depth, and mismatched target types. r[wasm-validation.br-table]

## Phase 3: Spec/catalog sync and validation

- [x] [serial] Sync requirements and catalogs in `docs/requirements.md`, `cairn/specs/wasm/spec.md`, legacy `openspec/specs/wasm/spec.md`, README, and crate docs. r[wasm-validation.br-table]
- [x] [serial] Run focused Wasm tests and Verus proof checks, recording evidence. r[wasm-validation.br-table]
- [x] [serial] Run cargo/clippy/Tracey/ensures validation for the touched tree. r[wasm-validation.br-table]
- [x] [serial] Run Cairn validation/gates, archive this change, and commit completed evidence. r[wasm-validation.br-table]