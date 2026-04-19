## Phase 1: Varint (Shared With DID)

- [x] Reuse or import varint encode/decode from `ucan_did.rs` if landed, otherwise implement here
- [x] Verify varint round-trip if not already proven

## Phase 2: Multihash

- [x] Define `Multihash` struct: code (u64), digest_len (u64), digest (Seq<u8>)
- [x] Implement `multihash_wrap_spec` and `multihash_wrap` exec
- [x] Implement `multihash_bytes_spec` — serialize to code varint + length varint + digest
- [x] Implement `multihash_decode_spec` — parse from bytes
- [x] Prove multihash digest length matches: `wrap(code, digest).digest_len == digest.len()`
- [x] Prove multihash round-trip: `decode(bytes(mh)) == Some(mh)`

## Phase 3: CID

- [x] Define `Cid` struct: version (u8), codec (u64), multihash (Multihash)
- [x] Implement `cid_from_bytes_spec` with `#[verifier::external_body]` hash function
- [x] Implement `cid_bytes_spec` — version byte + codec varint + multihash bytes
- [x] Prove deterministic: same inputs produce same CID
- [x] Prove version correct: version is always 1
- [x] Prove CID injectivity: equal bytes imply equal components
- [x] Add `r[ucan-cid.*]` requirements to `docs/requirements.md`

## Phase 4: Multibase and Validation

- [x] Implement `cid_encode` and `cid_decode` as `#[verifier::external_body]` with round-trip postcondition
- [x] Prove CID encode-decode round-trip: `decode(encode(cid, base)) == Some(cid)`
- [x] Property tests: random content produces deterministic CIDs
- [x] Property tests: CID encode/decode round-trip with random bytes
- [x] Add `r[impl]` and `r[verify]` Tracey tags
- [x] Run `./scripts/verify.sh` green — 18 verified, 0 errors
- [x] Run `tracey query uncovered` — 0 uncovered out of 1649 rules
