## Phase 1: Varint (Shared With DID)

- [ ] Reuse or import varint encode/decode from `ucan_did.rs` if landed, otherwise implement here
- [ ] Verify varint round-trip if not already proven

## Phase 2: Multihash

- [ ] Define `Multihash` struct: code (u64), digest_len (u64), digest (Seq<u8>)
- [ ] Implement `multihash_wrap_spec` and `multihash_wrap` exec
- [ ] Implement `multihash_bytes_spec` — serialize to code varint + length varint + digest
- [ ] Implement `multihash_decode_spec` — parse from bytes
- [ ] Prove multihash digest length matches: `wrap(code, digest).digest_len == digest.len()`
- [ ] Prove multihash round-trip: `decode(bytes(mh)) == Some(mh)`

## Phase 3: CID

- [ ] Define `Cid` struct: version (u8), codec (u64), multihash (Multihash)
- [ ] Implement `cid_from_bytes_spec` with `#[verifier::external_body]` hash function
- [ ] Implement `cid_bytes_spec` — version byte + codec varint + multihash bytes
- [ ] Prove deterministic: same inputs produce same CID
- [ ] Prove version correct: version is always 1
- [ ] Prove CID injectivity: equal bytes imply equal components
- [ ] Add `r[ucan-cid.*]` requirements to `docs/requirements.md`

## Phase 4: Multibase and Validation

- [ ] Implement `cid_encode` and `cid_decode` as `#[verifier::external_body]` with round-trip postcondition
- [ ] Prove CID encode-decode round-trip: `decode(encode(cid, base)) == Some(cid)`
- [ ] Property tests: random content produces deterministic CIDs
- [ ] Property tests: CID encode/decode round-trip with random bytes
- [ ] Add `r[impl]` and `r[verify]` Tracey tags
- [ ] Run `./scripts/verify.sh` green
- [ ] Run `tracey query uncovered` — 0 uncovered
