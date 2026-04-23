## Why

The landed `src/ucan_cid.rs` module is a proof-layer CID framing model,
not the larger runtime CID API described by the older change artifacts.
The code already contains verified varint framing, multihash byte layout,
CID header layout, byte-level multibase round-trip, and SHA-256-based
CID determinism over uninterpreted digest bytes. What it does *not*
contain is a public exec `cid_encode(...)`, `cid_decode(...)`,
`multihash_wrap(...)`, `cid_from_bytes(...)`, or a runtime `CidString`
type.

This change should therefore reconcile the OpenSpec artifacts to the
module that actually shipped so the proposal, design, tasks, canonical
spec, and evidence match the verified code already in `src/ucan_cid.rs`.

## What Changes

A landed module `src/ucan_cid.rs` providing:

**Runtime carrier structs:**
- `Multihash { code: u64, digest: Vec<u8> }`
- `Cid { version: u8, codec: u64, multihash: Multihash }`
- constants `CID_VERSION`, `SHA256_CODE`, `SHA256_DIGEST_LEN`,
  `DAG_CBOR_CODEC`, and `RAW_CODEC`

**Spec / proof surface:**
- `varint_encode_spec`, `varint_decode_spec`
- `multihash_wrap_spec`, `multihash_bytes_spec`, `multihash_decode_spec`
- `cid_bytes_spec`, `cid_from_multihash_spec`, `cid_from_bytes_spec`
- `cid_encode_spec`, `cid_decode_spec` as uninterpreted multibase specs
- `sha256_digest_spec` as an uninterpreted digest model
- proofs for varint round-trip, empty-input rejection, multihash digest
  length, multihash round-trip, CID version correctness, CID
  determinism, CID header injectivity, byte-level multibase round-trip,
  and deterministic CID construction from content bytes

**Verified properties:**
- varint encode/decode round-trip and empty-input rejection
- multihash digest length matches the digest bytes
- multihash byte encoding round-trips for representable digest lengths
- constructed CIDs always use version 1
- CID construction is deterministic for fixed codec and content
- equal CID byte sequences imply equal CID headers (`version`, `codec`)
- multibase encode/decode round-trips canonical CID byte sequences

## Non-Goals

- a public exec `cid_encode(...)` / `cid_decode(...)` API
- a runtime `CidString` type
- runtime SHA-256 or BLAKE3 hashing helpers
- structured CID parsing from multibase strings back into `Cid`
- CID resolution, storage lookup, or proof-chain traversal by CID
- IPLD selectors, CIDv0 compatibility, or alternate hash families
- property-test scaffolding beyond the focused constant tests already in
  the landed module

## Capabilities

### Landed Capabilities
- `ucan-cid.cid-deterministic`
- `ucan-cid.cid-version`
- `ucan-cid.cid-round-trip`
- `ucan-cid.cid-injectivity`
- `ucan-cid.multihash-digest-length`
- `ucan-cid.multihash-round-trip`
- `ucan-cid.varint-round-trip`
- `ucan-cid.varint-rejects-empty`

## Impact

- **Files**: `src/ucan_cid.rs`, `docs/requirements.md`, and synchronized OpenSpec artifacts
- **APIs**: proof/spec CID framing surface plus runtime carrier structs and constants
- **Dependencies**: none beyond the existing Verus/vstd environment; hashing stays uninterpreted
- **Testing**: focused runtime constant tests plus full-lib Verus verification

## Acceptance Evidence

Archive readiness for this change requires fresh `openspec_gate`
results for `proposal`, `design`, and `tasks`, plus fresh validation
transcripts for:

- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_cid`
- `./scripts/verus.sh src/lib.rs`

The resulting gate/runtime/proof evidence must be captured alongside the
archive closeout and post-archive active-change recount.
