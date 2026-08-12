# UCAN DID Model Specification

## Purpose

Defines the byte-oriented `did:key` framing model used by UCAN token
issuer and audience fields. Covers multicodec varint prefixing,
base58btc multibase payload encoding, and the `did:key:z` prefix.
This change models generic framing for any `u64` codec value and any
opaque key-byte sequence, with Ed25519 (`0xed`) and 32-byte public keys
as the canonical UCAN example exercised by tests.

## Definitions

- **did:key**: A DID method whose method-specific identifier is a
  multibase-encoded multicodec-prefixed byte sequence.
- **Codec**: A numeric multicodec value serialized as a varint/LEB128
  byte sequence.
- **Key bytes**: The opaque byte payload carried after the multicodec
  varint prefix.
- **DidKey**: The encode-side carrier returned by `encode_did_key`,
  storing the exact key bytes, codec, and encoded `did:key` bytes.
- **Validity boundary**: This module preserves arbitrary codec values and
  arbitrary key bytes through did:key framing. It does not enforce
  codec-registry membership or codec-specific key lengths.
- **Trusted proof helper**: The proof of `r[ucan-did.injective]` relies on
  a trusted base58btc injectivity helper in addition to the user-facing
  round-trip requirement `r[ucan-did.base58btc-round-trip]`.

## Requirements

### DID Round-Trip

#### Requirement: Encode-Decode Round-Trip
`r[ucan-did.round-trip]`

Encoding key bytes as a `did:key` MUST use the canonical byte layout
`did:key:z ++ base58btc_encode(varint_encode(codec) ++ pk)`. The public
`DidKey` carrier returned by `encode_did_key` MUST preserve the exact
input key bytes and codec, and its `encoded` bytes MUST equal that
canonical encoding. Spec-level decode of those canonical bytes MUST
recover the original key bytes and codec.

`encode_did_key_spec(pk, codec) == did_key_prefix_spec() ++ base58btc_encode_spec(varint_encode_spec(codec) ++ pk)`

`decode_did_key_spec(encode_did_key_spec(pk, codec)) == Some((pk, codec))`

##### Scenario: Ed25519 key round-trips
`r[ucan-did.round-trip.scenario-ed25519]`

- GIVEN a 32-byte Ed25519 public key `pk`
- WHEN encoding it as `did:key` with codec `0xed`
- THEN spec-level decode returns `(pk, 0xed)`

##### Scenario: Encode carrier preserves inputs and bytes
`r[ucan-did.round-trip.scenario-carrier]`

- GIVEN any key bytes `pk` and any codec value `codec`
- WHEN `encode_did_key(pk, codec)` produces a `DidKey`
- THEN `DidKey.public_key_bytes == pk`, `DidKey.codec == codec`, and `DidKey.encoded == encode_did_key_spec(pk, codec)`

### DID Injectivity

#### Requirement: Distinct Keys Produce Distinct DIDs
`r[ucan-did.injective]`

If two key-byte payloads are different, their `did:key` encodings MUST
differ for the same codec.

`pk1 != pk2 ==> encode_did_key_spec(pk1, codec) != encode_did_key_spec(pk2, codec)`

##### Scenario: Two different Ed25519 keys
`r[ucan-did.injective.scenario-distinct-keys]`

- GIVEN `pk1 != pk2`, both 32-byte Ed25519 public keys
- WHEN encoding both as `did:key` with codec `0xed`
- THEN the encoded byte strings differ

### DID Format

#### Requirement: DID Prefix Is Valid
`r[ucan-did.prefix-valid]`

An encoded `did:key` MUST always start with the byte string `did:key:z`.

##### Scenario: Any encoded key has the prefix
`r[ucan-did.prefix-valid.scenario-any-key]`

- GIVEN any key bytes and any codec value
- WHEN encoding as `did:key`
- THEN the result starts with `did:key:z`

#### Requirement: Codec Is Preserved
`r[ucan-did.codec-preserved]`

The codec recovered from spec-level decoding MUST match the codec used
for encoding.

`decode_did_key_spec(encode_did_key_spec(pk, codec)) == Some((_, codec))`

##### Scenario: Ed25519 codec survives round-trip
`r[ucan-did.codec-preserved.scenario-ed25519]`

- GIVEN a 32-byte Ed25519 public key `pk`
- WHEN encoding it as `did:key` with codec `0xed`
- THEN spec-level decode recovers codec `0xed`

### Multicodec

#### Requirement: Multicodec Varint Round-Trip
`r[ucan-did.varint-round-trip]`

Encoding a codec as a varint and then decoding MUST recover the original
codec value and consumed length.

`varint_decode(varint_encode(code)) == Some((code, varint_encode(code).len()))`

##### Scenario: Ed25519 codec 0xed
`r[ucan-did.varint-round-trip.scenario-ed25519]`

- GIVEN codec value `0xed`
- WHEN encoding it as a varint
- THEN decoding returns `(0xed, 2)`

#### Requirement: Varint Decode Rejects Empty Or Truncated Input
`r[ucan-did.varint-rejects-empty]`

Decoding a varint from an empty byte sequence or a truncated continuation
sequence MUST return `None`.

##### Scenario: Empty varint input rejects
`r[ucan-did.varint-rejects-empty.scenario-empty]`

- GIVEN an empty byte sequence
- WHEN decoding as a varint
- THEN the result is `None`

##### Scenario: Truncated continuation rejects
`r[ucan-did.varint-rejects-empty.scenario-truncated]`

- GIVEN a non-empty byte sequence ending in a continuation byte
- WHEN decoding as a varint
- THEN the result is `None`

### Base58btc

#### Requirement: Base58btc Round-Trip
`r[ucan-did.base58btc-round-trip]`

Encoding bytes as base58btc and then decoding MUST recover the original
bytes.

`base58btc_decode(base58btc_encode(bytes)) == Some(bytes)`

##### Scenario: Multicodec-prefixed Ed25519 payload round-trips
`r[ucan-did.base58btc-round-trip.scenario-ed25519-payload]`

- GIVEN the bytes `[0xed, 0x01] ++ pk` for a 32-byte Ed25519 public key `pk`
- WHEN encoding and then decoding with base58btc
- THEN the original bytes are recovered