## Why

UCAN issuer and audience fields are `did:key` identifiers, not raw key
bytes. The canonical form is the `did:key:z...` URI: a multibase-encoded
multicodec prefix followed by the raw key bytes. Without a verified
framing model here, later UCAN work has to trust that the same key bytes
always become the same DID bytes and that the codec/payload boundary is
preserved through the encode/decode model.

The landed module in this repo is intentionally byte-oriented and
narrower than a full DID library. It models generic `did:key` framing
for any `u64` multicodec value and any opaque key-byte sequence, with
Ed25519 (`0xed`, encoded as `[0xed, 0x01]`) and 32-byte public keys as
the canonical UCAN example exercised by tests. It does not implement a
full generic `Did` type, codec-registry validation, or codec-specific
key-length rejection.

## What Changes

A landed module `src/ucan_did.rs` providing:

**Data model:**
- `DidKey` — an encode-side carrier that stores the exact public-key
  bytes, codec, and encoded `did:key` bytes produced by `encode_did_key`

**Public exec surface:**
- `encode_did_key(public_key_bytes, codec) -> DidKey` — varint-prefix the
  codec, append the raw key bytes, base58btc-encode the payload, and add
  the `did:key:z` prefix
- `varint_encode(code) -> Vec<u8>` / `varint_decode(bytes) -> Option<(u64, usize)>`
- `base58btc_encode(bytes) -> Vec<u8>` / `base58btc_decode(bytes) -> Option<Vec<u8>>`

**Spec/proof surface:**
- `decode_did_key_spec(did_bytes) -> Option<(public_key_bytes, codec)>`
  — spec-level decode model used for round-trip and codec-preservation
  proofs; the landed module does not expose a public exec decode API
- `multicodec_prefix_spec(codec, key_bytes)` / `multicodec_parse_spec(...)`

**Verified properties:**
- Round-trip: `decode_did_key_spec(encode_did_key_spec(pk, codec)) == Some((pk, codec))`
- Injective: `pk1 != pk2 ==> encode_did_key_spec(pk1, codec) != encode_did_key_spec(pk2, codec)`
- Prefix valid: encoded output always starts with `did:key:z`
- Codec preserved: decoding the encoded DID recovers the original codec
- Varint round-trip: `varint_decode(varint_encode(code))` recovers `code`
- Varint empty-input rejection: decoding an empty or truncated varint returns `None`
- Base58btc round-trip: `base58btc_decode(base58btc_encode(bytes)) == Some(bytes)`

Base58btc encode/decode stays `#[verifier::external_body]`.
The proof surface for this change is the structural framing contract:
varint correctness, did:key prefix construction, exact byte preservation,
and spec-level encode/decode round-trip. Codec-specific validity and key
length checks remain the responsibility of higher-level key-specific
modules such as Ed25519 signature handling.

## Non-Goals

- Supporting DID methods other than `did:key`
- A generic `Did` type, DID document resolution, or DID normalization
- Codec-registry membership checks or unsupported-codec rejection
- Codec-specific key-length validation or rejection
- A public runtime `decode_did_key` API in this change
- Higher-level UCAN proof-chain integration beyond preserving canonical
  `did:key` framing for later modules

## Capabilities

### New Capabilities
- `ucan-did.round-trip`: spec-level encode/decode round-trip preserves key bytes and codec
- `ucan-did.injective`: distinct key bytes produce distinct DIDs for the same codec
- `ucan-did.prefix-valid`: encoded output always starts with `did:key:z`
- `ucan-did.codec-preserved`: codec survives spec-level round-trip unchanged
- `ucan-did.varint-round-trip`: codec values encode/decode cleanly through varint framing
- `ucan-did.varint-rejects-empty`: empty or truncated varints reject
- `ucan-did.base58btc-round-trip`: base58btc round-trip preserves bytes

## Impact

- **Files**: `src/ucan_did.rs` (landed), `docs/requirements.md`, and the UCAN DID OpenSpec artifacts
- **APIs**: public exec encode + varint/base58 helpers, plus spec-level DID decode helpers
- **Dependencies**: `bs58` behind `#[cfg(not(verus_keep_ghost))]` for runtime only
- **Testing**: Runtime tests cover the Ed25519 example path, varint round-trip/empty rejection, base58 round-trip, and distinct-key separation; Verus proofs cover the structural did:key framing contract

## Acceptance Evidence

Archive readiness for this change requires fresh `openspec_gate`
results for proposal/design/tasks plus fresh validation transcripts for
`CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_did`,
`CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib test_varint_decode_truncated_continuation`,
and `./scripts/verus.sh src/lib.rs`, captured alongside the archive
closeout evidence.