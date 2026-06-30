## Phase 1: Control-label model

- [x] [serial] Add Wasm control-label kinds and target-type specs. r[wasm-control.labels]
- [x] [serial] Add branch-depth and branch-arity exec helpers with positive and negative tests. r[wasm-control.branch-arity]
- [x] [serial] Add return-arity exec helper with positive and negative tests. r[wasm-control.return-arity]

## Phase 2: Spec and catalog sync

- [x] [serial] Add canonical requirement prose and scenarios for Wasm control labels, branch arity, and return arity. r[wasm-control.labels]
- [x] [serial] Update README and crate docs to include the expanded Wasm slice. r[wasm-control.branch-arity]

## Phase 3: Validation and archive

- [x] [serial] Run focused Wasm tests and available Verus/fallback checks, recording any blocked tool. r[wasm-control.return-arity]
- [x] [serial] Run Tracey coverage and repo checks needed for the touched requirements. r[wasm-control.branch-arity]
- [x] [serial] Sync accepted specs, archive this change, and commit completed evidence. r[wasm-control.labels]
