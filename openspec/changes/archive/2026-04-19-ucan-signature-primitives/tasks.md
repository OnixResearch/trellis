## Phase 1: Data Model and Spec Functions

- [x] Define `PublicKey`, `SecretKey`, `Signature` newtypes wrapping fixed-size byte arrays
- [x] Define spec functions: `sign_spec`, `verify_spec`, `public_from_secret_spec`
- [x] Add `r[ucan-signature.*]` requirements to `docs/requirements.md`

## Phase 2: External Body Exec Functions

- [x] Implement `sign`, `verify`, `keypair_generate`, `public_from_secret` as `#[verifier::external_body]` with postconditions
- [x] Add ed25519-dalek dependency behind `#[cfg(not(verus_keep_ghost))]`
- [x] Verify postcondition consistency: round-trip, key-binding, determinism, length

## Phase 3: Proofs

- [x] Prove `sign_verify_round_trip`: `verify(public_from_secret(sk), msg, sign(sk, msg)) == true`
- [x] Prove `wrong_key_rejects`: `pk != public_from_secret(sk) ==> verify(pk, msg, sign(sk, msg)) == false`
- [x] Prove `wrong_message_rejects`: `msg1 != msg2 ==> verify(pk, msg2, sign(sk, msg1)) == false`
- [x] Prove `sign_deterministic`: `sign(sk, msg) == sign(sk, msg)`
- [x] Prove `signature_length`: `sign(sk, msg).len() == 64`

## Phase 4: Tests and Validation

- [x] Property tests: random keypairs, round-trip sign/verify
- [x] Property tests: random message mutation rejects
- [x] Add `r[impl]` and `r[verify]` Tracey tags to all pub fns
- [x] Run `./scripts/verify.sh` green — 9 verified, 0 errors
- [x] Run `tracey query uncovered` — 0 uncovered out of 1649 rules
