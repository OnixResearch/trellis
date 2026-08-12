## Phase 1: Table and signature model

- [x] [serial] Add finite function-type and table-entry models with exact parameter/result matching. r[wasm-table.signature]
- [x] [serial] Add initialized-entry, out-of-range, and signature-mismatch tests. r[wasm-table.signature]

## Phase 2: `call_indirect` validation

- [x] [serial] Add reachable `call_indirect` validation that consumes params plus `i32` index and pushes results. r[wasm-validation.call-indirect]
- [x] [serial] Add unreachable `call_indirect` validation that keeps stack-polymorphic operand admission but still requires a matching table entry. r[wasm-validation.call-indirect]
- [x] [serial] Add positive and negative tests covering accepted calls, stack mismatch, uninitialized entries, out-of-range entries, and signature mismatch. r[wasm-validation.call-indirect]

## Phase 3: Spec/catalog sync and validation

- [x] [serial] Sync requirements and catalogs in `docs/requirements.md`, `cairn/specs/wasm/spec.md`, legacy `openspec/specs/wasm/spec.md`, README, and crate docs. r[wasm-validation.call-indirect]
- [x] [serial] Run focused Wasm tests and Verus proof checks, recording evidence. r[wasm-table.signature]
- [x] [serial] Run cargo/clippy/Tracey/ensures validation for the touched tree. r[wasm-validation.call-indirect]
- [x] [serial] Run Cairn validation/gates, archive this change, and commit completed evidence. r[wasm-table.signature]