## Phase 1: Varint and Multicodec

- [ ] Define `varint_encode_spec` and `varint_decode_spec` as recursive spec functions
- [ ] Implement `varint_encode` and `varint_decode` exec functions (fully internalized)
- [ ] Prove varint round-trip: `varint_decode(varint_encode(n)) == Some((n, len))`
- [ ] Prove varint rejects empty input

## Phase 2: DID Encoding

- [ ] Define `Did` and `DidKey` newtypes
- [ ] Define `encode_did_key_spec` and `decode_did_key_spec`
- [ ] Implement `base58btc_encode` and `base58btc_decode` as `#[verifier::external_body]` with round-trip postcondition
- [ ] Implement `encode_did_key` and `decode_did_key` exec functions
- [ ] Add `r[ucan-did.*]` requirements to `docs/requirements.md`

## Phase 3: Proofs

- [ ] Prove DID round-trip: `decode_did_key(encode_did_key(pk, codec)) == Some((pk, codec))`
- [ ] Prove injectivity: `pk1 != pk2 ==> encode_did_key(pk1, c) != encode_did_key(pk2, c)`
- [ ] Prove prefix validity: encoded DID starts with `"did:key:z"`
- [ ] Prove codec preservation through round-trip

## Phase 4: Tests and Validation

- [ ] Property tests: random 32-byte keys round-trip through did:key
- [ ] Property tests: varint round-trip with random u64 values
- [ ] Property tests: distinct random keys produce distinct DIDs
- [ ] Add `r[impl]` and `r[verify]` Tracey tags
- [ ] Run `./scripts/verify.sh` green
- [ ] Run `tracey query uncovered` — 0 uncovered
