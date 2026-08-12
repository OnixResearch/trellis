## Phase 1: Control-frame model

- [x] [serial] Add control-frame data and frame-stack specs to validation state. r[wasm-validation.control-frame]
- [x] [serial] Add block and loop entry helpers with positive and negative tests. r[wasm-validation.block-entry]
- [x] [serial] Add control-frame end helper with reachable and unreachable tests. r[wasm-validation.block-end]

## Phase 2: Spec and catalog sync

- [x] [serial] Add canonical requirements and scenarios for control frames, block entry, and block end. r[wasm-validation.control-frame]
- [x] [serial] Update README and crate docs to include control-frame validation. r[wasm-validation.block-end]

## Phase 3: Validation and archive

- [x] [serial] Run focused Wasm tests and Verus proof checks, recording evidence. r[wasm-validation.block-entry]
- [x] [serial] Run full cargo/clippy/Tracey/ensures validation for the touched requirements. r[wasm-validation.block-end]
- [x] [serial] Sync accepted specs, archive this change, and commit completed evidence. r[wasm-validation.control-frame]
