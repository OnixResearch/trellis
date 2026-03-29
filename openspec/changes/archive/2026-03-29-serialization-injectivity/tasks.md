## Phase 1: Core encoding primitives

- [x] Add `src/serialize_inj.rs` with module doc and crate import ✅
- [x] `encode_u64_spec` / `decode_u64_spec` — spec fns for 8-byte LE encoding ✅
- [x] `encode_u64_len` — proof that output is exactly 8 bytes ✅
- [x] `encode_u64_roundtrip` — proof that decode(encode(v)) == v ✅
- [x] `encode_u64_injective` — proof that equal encodings → equal values ✅
- [x] `encode_u64` — exec wrapper with ensures linking to spec ✅

## Phase 2: Tagged and length-prefixed encoding

- [x] `encode_tagged_spec` — spec fn: tag byte ∥ payload ✅
- [x] `tag_disjoint` — proof: different tags → different encodings ✅
- [x] `encode_tagged_injective` — proof: same tag, equal bytes → equal payloads ✅
- [x] `encode_len_prefixed_spec` — spec fn: 8-byte length ∥ data ✅
- [x] `encode_len_prefixed_injective` — proof: equal encodings → equal data ✅
- [x] `encode_len_prefixed_length_recovery` — proof: first 8 bytes decode to data.len() ✅

## Phase 3: Struct encodings

- [x] `encode_pair_spec` — spec fn: encode_u64(a) ∥ encode_u64(b) ✅
- [x] `encode_pair_len` — proof: output is 16 bytes ✅
- [x] `encode_pair_injective` — proof: equal encodings → both fields equal ✅
- [x] `encode_quad_spec` — spec fn: four concatenated u64 encodings ✅
- [x] `encode_quad_len` — proof: output is 32 bytes ✅
- [x] `encode_quad_injective` — proof: equal encodings → all four fields equal ✅

## Phase 4: Cross-module composition

- [x] `chain_hash_struct_injective` — proof composing encode_quad_injective with chain::hash_collision_free ✅
- [x] `dual_hash_struct_injective` — proof composing encode_pair_injective with snapshot_integrity::dual_hash_collision_resistant ✅
- [x] Add cross-module proofs to `src/properties.rs` linking serialize_inj to chain and snapshot_integrity ✅

## Phase 5: Module wiring and tests

- [x] Add `pub mod serialize_inj;` to `src/lib.rs` ✅
- [x] Add tracey `r[impl]`/`r[depends]`/`r[verify]` annotations to all pub fns ✅
- [x] Proptest: roundtrip encoding for u64, pairs, quads ✅
- [x] Proptest: injectivity oracle — random pairs, encoding equality ↔ value equality ✅
- [x] Proptest: tag disjointness — random tags and payloads ✅
- [x] Proptest: length-prefixed roundtrip with random byte sequences ✅

## Phase 6: Verification gates

- [x] `cargo test` green — 20 serialize_inj + 3 cross-module tests pass ✅
- [x] `cargo clippy -- -D warnings` clean ✅
- [x] `verus` passes — 4016 verified, 0 errors ✅
- [x] `check-coverage.sh` — all pub fns have r[impl] ✅
- [x] `check-ensures.sh` — all pub fns in verus! have ensures ✅
