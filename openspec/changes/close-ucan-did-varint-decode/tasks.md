## Phase 0: Baseline and evidence plan

- [x] Run baseline `./scripts/verus.sh src/ucan_did.rs` and capture output in `evidence/baseline-verus-ucan-did.txt` [covers=ucan-did.varint-round-trip,ucan-did.varint-rejects-empty]
- [x] Run baseline focused runtime tests with a non-flaky target/cache setup and capture output in `evidence/baseline-cargo-test-ucan-did.txt`: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet ucan_did` [covers=ucan-did.varint-round-trip,ucan-did.varint-rejects-empty]
- [x] Capture pre-change proof-gap counts from source in `evidence/baseline-proof-gap-counts.txt` for `external_body`, `assume_specification[...]`, `@verified-logic:no-ensures`, instruction-selection runtime-only exemptions, and the local `assume\\s*\\(` audit, recording the expected baseline total `44` for the final `44 -> 43` delta [covers=verification-gaps.inventory-after-ucan-did-varint]

## Phase 1: Verified varint decode primitives

- [x] Add named u64 LEB128 decode constants inside the `verus!` block, including `VARINT_BASE`, byte budget, continuation bit, and payload mask [covers=ucan-did.varint-bounded-u64]
- [x] Refactor `varint_decode_spec` through bounded decoder primitives that reject empty, truncated, overlong, and overflowing inputs [covers=ucan-did.varint-bounded-u64,ucan-did.varint-rejects-empty]
- [x] Prove round-trip preservation lemmas for `varint_round_trip`, `varint_decode_prefix_free`, and multicodec round-trip consumers [covers=ucan-did.varint-round-trip]
- [x] Prove empty and truncated-continuation rejection lemmas, including `varint_decode_rejects_empty` and `varint_decode_rejects_single_continuation` [covers=ucan-did.varint-rejects-empty]
- [x] Prove bounded malformed-input lemmas for overlong rejection, final-byte overflow rejection, and non-minimal representable decode acceptance, including `varint_decode_rejects_overlong_continuations`, `varint_decode_rejects_overflowing_final_payload`, and `varint_decode_accepts_nonminimal_zero` [covers=ucan-did.varint-bounded-u64]
- [x] Replace the trusted `varint_decode` body with verified exec code tied to `varint_decode_spec` by postcondition [covers=ucan-did.varint-bounded-u64,ucan-did.varint-round-trip]
- [x] Remove `#[verifier::external_body]` from `ucan_did::varint_decode` and remove or quarantine any now-unused plain Rust runtime helper [covers=verification-gaps.close-ucan-did-varint-decode]

## Phase 2: Specs, requirements, and inventory

- [x] Update `docs/requirements.md` with any new or clarified `ucan-did.varint-*` and `verification-gaps.*` requirement IDs needed for Tracey coverage [covers=ucan-did.varint-bounded-u64,ucan-did.varint-round-trip,ucan-did.varint-rejects-empty,verification-gaps.close-ucan-did-varint-decode,verification-gaps.inventory-after-ucan-did-varint]
- [x] Add or update Tracey source/proof/test annotations for `ucan-did.varint-bounded-u64`, `ucan-did.varint-round-trip`, `ucan-did.varint-rejects-empty`, `verification-gaps.close-ucan-did-varint-decode`, and `verification-gaps.inventory-after-ucan-did-varint` [covers=ucan-did.varint-bounded-u64,ucan-did.varint-round-trip,ucan-did.varint-rejects-empty,verification-gaps.close-ucan-did-varint-decode,verification-gaps.inventory-after-ucan-did-varint]
- [x] Sync the UCAN DID checked-mul/add bounded-varint semantics into canonical `openspec/specs/ucan-did/spec.md` before archive and capture `evidence/final-semantic-reconciliation.txt` proving canonical spec, change-local spec, and `src/ucan_did.rs` agree [covers=ucan-did.varint-bounded-u64,ucan-did.varint-round-trip,ucan-did.varint-rejects-empty]
- [x] Update `openspec/specs/verification-gaps/spec.md` to remove the `ucan_did::varint_decode` closeable row and refresh source-derived counts [covers=verification-gaps.close-ucan-did-varint-decode,verification-gaps.inventory-after-ucan-did-varint]
- [x] Update `README.md` proof-gap headline/count prose to match the refreshed non-double-counted total [covers=verification-gaps.inventory-after-ucan-did-varint]
- [x] Capture count-audit evidence in `evidence/final-proof-gap-counts.txt` proving README, canonical verification-gap spec, and source-derived counts agree [covers=verification-gaps.inventory-after-ucan-did-varint]
- [x] Capture a source audit in `evidence/final-decoder-trust-audit.txt` proving the `ucan_did.rs` decoder helper path has no `#[verifier::external_body]`, `assume(...)` via `rg -n 'assume\\s*\\(' src/ucan_did.rs`, `assume_specification[...]`, `@verified-logic:no-ensures`, or `@verified-logic:exempt` replacement trust [covers=verification-gaps.close-ucan-did-varint-decode]

## Phase 3: Validation

- [x] Run `./scripts/verus.sh src/ucan_did.rs` and capture output in `evidence/final-verus-ucan-did.txt` showing `varint_decode` verifies without `external_body` [covers=verification-gaps.close-ucan-did-varint-decode,ucan-did.varint-bounded-u64]
- [x] Run `./scripts/verus.sh src/lib.rs` and capture output in `evidence/final-verus-lib.txt` [covers=verification-gaps.close-ucan-did-varint-decode]
- [x] Run `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet ucan_did` and capture output in `evidence/final-cargo-test-ucan-did.txt`, including focused positive/negative cases for Ed25519 two-byte decode, trailing-suffix consumed length, non-minimal accepted input, truncated-continuation rejection, and overflowing/overlong rejected input [covers=ucan-did.varint-round-trip,ucan-did.varint-rejects-empty,ucan-did.varint-bounded-u64]
- [x] Run `./scripts/check-ensures.sh` and capture output in `evidence/final-check-ensures.txt` [covers=verification-gaps.close-ucan-did-varint-decode]
- [x] Run `tracey query status` first, then run `tracey query uncovered` and `tracey query untested` sequentially and capture zero-count outputs in `evidence/final-tracey.txt` [covers=ucan-did.varint-bounded-u64,verification-gaps.inventory-after-ucan-did-varint]
- [x] Run `openspec_gate proposal close-ucan-did-varint-decode`, `openspec_gate design close-ucan-did-varint-decode`, and `openspec_gate tasks close-ucan-did-varint-decode`; capture final outputs in `evidence/final-openspec-gates.txt` before archive [covers=verification-gaps.inventory-after-ucan-did-varint]
