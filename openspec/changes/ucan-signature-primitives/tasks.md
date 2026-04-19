## Phase 1: Data Model and Spec Functions

- [ ] Define `PublicKey`, `SecretKey`, `Signature` newtypes wrapping fixed-size byte arrays
- [ ] Define spec functions: `sign_spec`, `verify_spec`, `public_from_secret_spec`
- [ ] Add `r[ucan-signature.*]` requirements to `docs/requirements.md`

## Phase 2: External Body Exec Functions

- [ ] Implement `sign`, `verify`, `keypair_generate`, `public_from_secret` as `#[verifier::external_body]` with postconditions
- [ ] Add ed25519-dalek dependency behind `#[cfg(not(verus_keep_ghost))]`
- [ ] Verify postcondition consistency: round-trip, key-binding, determinism, length

## Phase 3: Proofs

- [ ] Prove `sign_verify_round_trip`: `verify(public_from_secret(sk), msg, sign(sk, msg)) == true`
- [ ] Prove `wrong_key_rejects`: `pk != public_from_secret(sk) ==> verify(pk, msg, sign(sk, msg)) == false`
- [ ] Prove `wrong_message_rejects`: `msg1 != msg2 ==> verify(pk, msg2, sign(sk, msg1)) == false`
- [ ] Prove `sign_deterministic`: `sign(sk, msg) == sign(sk, msg)`
- [ ] Prove `signature_length`: `sign(sk, msg).len() == 64`

## Phase 4: Tests and Validation

- [ ] Property tests: random keypairs, round-trip sign/verify
- [ ] Property tests: random message mutation rejects
- [ ] Add `r[impl]` and `r[verify]` Tracey tags to all pub fns
- [ ] Run `./scripts/verify.sh` green
- [ ] Run `tracey query uncovered` — 0 uncovered
