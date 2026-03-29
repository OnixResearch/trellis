## Phase 1: Core typed_state module

- [x] Add `src/typed_state.rs` with module doc ✅
- [x] `Unverified` and `Verified` marker types (zero-sized, public names, private Stamped fields) ✅
- [x] `Stamped<S, T>` struct with private fields ✅
- [x] `stamp_spec` / `stamp` — wrap raw data as unverified ✅
- [x] `verify_with_spec` / `verify_with` — predicate-gated transition to verified ✅
- [x] `unwrap_verified_spec` / `unwrap_verified` — extract inner value from verified ✅
- [x] `inner_spec` / `inner` — read access in any state ✅

## Phase 2: Verus proofs

- [x] `stamp_preserves_value` — stamp(v).inner() == v ✅
- [x] `verify_roundtrip` — stamp → verify(true) → unwrap == original ✅
- [x] `verify_none_on_false` — verify(false predicate) returns None ✅
- [x] `verify_implies_predicate` — Some result means predicate held ✅

## Phase 3: Concrete snapshot instantiation

- [x] `verify_snapshot` in properties.rs — composes typed_state + snapshot_integrity ✅
- [x] Proof: verify_snapshot returning Some implies verify_integrity_spec holds ✅
- [x] Proptest: snapshot roundtrip through stamp → verify_snapshot → unwrap ✅

## Phase 4: Module wiring and tests

- [x] Add `pub mod typed_state;` to `src/lib.rs` ✅
- [x] Add tracey annotations to all pub fns ✅
- [x] Add requirements to `docs/requirements.md` ✅
- [x] Proptest: stamp/verify/unwrap roundtrip with u64 values ✅
- [x] Proptest: verify rejects when predicate fails ✅
- [x] Proptest: inner returns same value for both states ✅

## Phase 5: Verification gates

- [x] `cargo test` green — 13 typed_state + 2 cross-module tests pass ✅
- [x] `cargo clippy -- -D warnings` clean ✅
- [x] `verus` passes — 4024 verified, 0 errors ✅
- [x] `check-coverage.sh` clean ✅
- [x] `check-ensures.sh` clean ✅
