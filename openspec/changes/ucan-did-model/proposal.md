## Why

UCANs identify issuer and audience by DID, not raw key bytes. The
canonical form for key-based identity is `did:key:z...` — a multibase-
encoded multicodec-prefixed public key. Every UCAN token field that
references an identity uses this encoding, and two tokens that name
the same key with different encodings are semantically identical but
byte-different. Without a verified DID model, the token envelope
cannot canonically identify principals, and chain validation cannot
match issuer-to-audience across delegation hops.

The encode/decode logic is small but fiddly: multicodec varint
prefixing of the Ed25519 codec value `0xed` as bytes `[0xed, 0x01]`,
multibase encoding (base58btc with 'z' prefix), and round-trip
correctness. A bug here means tokens that should chain don't, or worse,
two different keys appear to be the same identity.

## What Changes

A new module `src/ucan_did.rs` providing:

**Data model:**
- `DidKey` — a validated `did:key` value carrying the encoded bytes plus decoded public key bytes and codec for the supported codec set in this change

**Encode/decode:**
- `encode_did_key(public_key, codec) -> DidKey` — multicodec prefix + base58btc + `did:key:z` prefix
- `decode_did_key(did_string) -> Option<(PublicKey, Codec)>` — parse and extract raw decoded parts from an externally supplied did:key string
- `multicodec_prefix(codec) -> Seq<u8>` — varint encoding for codec values such as Ed25519 (`0xed` encoded as `[0xed, 0x01]`)
- `base58btc_encode(bytes) -> Seq<u8>` / `base58btc_decode(chars) -> Option<Seq<u8>>`

**Verified properties:**
- Round-trip: `decode_did_key(encode_did_key(pk, codec)) == Some((pk, codec))`
- Canonical: `encode_did_key(pk1, c) == encode_did_key(pk2, c) ==> pk1 == pk2` (injective)
- Prefix: encoded DID always starts with `"did:key:z"`
- Codec preservation: decoded codec matches encoded codec
- Multicodec round-trip: varint prefix encodes/decodes correctly

Base58btc encode/decode stays `#[verifier::external_body]` (arithmetic-
heavy division loop). The proof surface is the composition: multicodec
framing is correct, base58 round-trips, the `did:key:` prefix is
always present, and decode rejects malformed inputs at the byte/string
boundary for this narrowed `did:key` surface.

## Non-Goals

- Supporting DID methods other than `did:key`
- Full generic DID parsing, normalization, or DID document resolution
- Unsupported codec families beyond the codec values explicitly modeled by this change
- Higher-level UCAN proof-chain integration beyond returning validated `did:key` data

## Capabilities

### New Capabilities
- `ucan-did.round-trip`: encode then decode recovers original key and codec
- `ucan-did.injective`: distinct keys produce distinct DIDs
- `ucan-did.prefix-valid`: output always starts with `did:key:z`
- `ucan-did.codec-preserved`: codec survives round-trip
- `ucan-did.varint-round-trip`: codec values encode/decode cleanly through varint framing

## Impact

- **Files**: `src/ucan_did.rs` (new)
- **APIs**: ~8 pub fns (encode/decode for DID, multicodec, base58)
- **Dependencies**: None for verification; optional bs58 behind cfg gate for runtime
- **Testing**: Property tests for round-trip with random 32-byte keys; Verus proofs for structural properties
