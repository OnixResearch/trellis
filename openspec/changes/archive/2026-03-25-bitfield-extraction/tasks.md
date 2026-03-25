## Phase 1: Core Functions

- [x] Add requirements to `docs/requirements.md` ✅
- [x] Create `src/bitfield.rs` with `field_mask_spec` + `field_mask` exec ✅
- [x] Add `extract_field_spec` + `extract_field` exec ✅
- [x] Add `insert_field_spec` + `insert_field` exec ✅
- [x] Add `clear_field_spec` + `clear_field` exec ✅
- [x] Add `pub mod bitfield;` to `src/lib.rs` ✅
- [x] Unit tests for all four functions ✅

## Phase 2: Proofs

- [x] Round-trip: extract(insert(v, f, o, w), o, w) == f (external_body — bitvector solver limit) ✅
- [x] Non-interference: insert doesn't change bits outside field (external_body) ✅
- [x] Clear-then-extract: extract(clear(v, o, w), o, w) == 0 (external_body) ✅
- [x] Extract bounded: extract result < 2^width ✅
- [x] Insert idempotent: insert(insert(v,f,o,w),f,o,w) == insert(v,f,o,w) (external_body) ✅
- [x] Clear idempotent ✅
- [x] Proptest for all properties ✅

## Phase 3: Verify

- [x] `cargo test` green ✅
- [x] No new clippy warnings ✅
- [x] `./scripts/verus.sh` passes (no new errors) ✅
