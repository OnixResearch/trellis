## Phase 1: Varint and Multicodec

- [x] Define `varint_encode_spec` and `varint_decode_spec` as recursive spec functions
- [x] Implement `varint_encode` and `varint_decode` exec functions (fully internalized)
- [x] Prove varint round-trip: `varint_decode(varint_encode(n)) == Some((n, len))`
- [x] Prove varint rejects empty input

## Phase 2: DID Encoding

- [x] Define `Did` and `DidKey` newtypes
- [x] Define `encode_did_key_spec` and `decode_did_key_spec`
- [x] Implement `base58btc_encode` and `base58btc_decode` as `#[verifier::external_body]` with round-trip postcondition
- [x] Implement `encode_did_key` and `decode_did_key` exec functions
- [x] Add `r[ucan-did.*]` requirements to `docs/requirements.md`

## Phase 3: Proofs

- [x] Prove DID round-trip: `decode_did_key(encode_did_key(pk, codec)) == Some((pk, codec))`
- [x] Prove injectivity: `pk1 != pk2 ==> encode_did_key(pk1, c) != encode_did_key(pk2, c)`
- [x] Prove prefix validity: encoded DID starts with `"did:key:z"`
- [x] Prove codec preservation through round-trip

## Phase 4: Tests and Validation

- [x] Property tests: random 32-byte keys round-trip through did:key
- [x] Property tests: varint round-trip with random u64 values
- [x] Property tests: distinct random keys produce distinct DIDs
- [x] Add `r[impl]` and `r[verify]` Tracey tags
- [x] Run `./scripts/verify.sh` green — 20 verified, 0 errors
- [x] Run `tracey query uncovered` — 0 uncovered out of 1649 rules
