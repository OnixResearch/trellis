# UCAN DID Model Specification

## Purpose

Defines correctness properties for DID (did:key) encoding and decoding
as used by UCAN token issuer and audience fields. Covers multicodec
varint prefixing, multibase (base58btc) encoding, and the `did:key:`
URI scheme.

## Definitions

- **DID**: A Decentralized Identifier, a URI of the form `did:<method>:<method-specific-id>`.
- **did:key**: A DID method where the method-specific identifier is a
  multibase-encoded multicodec-prefixed public key.
- **Multicodec**: A numeric codec value identifying the key type
  (for example Ed25519 = `0xed`), serialized with varint/LEB128 bytes
  (for Ed25519, `[0xed, 0x01]`).
- **Multibase**: A prefix character identifying the base encoding
  ('z' for base58btc).
- **Varint**: An unsigned variable-length integer encoding (LEB128).

## Requirements

### DID Round-Trip

#### Requirement: Encode-Decode Round-Trip
`r[ucan-did.round-trip]`

Encoding a public key as a did:key and decoding the result MUST
recover the original key and codec for the supported codec set of this
change.

`decode_did_key(encode_did_key(pk, codec)) == Some((pk, codec))`

##### Scenario: Ed25519 key round-trips
`r[ucan-did.round-trip.scenario-ed25519]`

- GIVEN a 32-byte Ed25519 public key pk
- WHEN encoding as did:key with Ed25519 codec
- THEN decoding the result returns (pk, Ed25519)

### DID Injectivity

#### Requirement: Distinct Keys Produce Distinct DIDs
`r[ucan-did.injective]`

If two public keys are different, their DID encodings MUST differ.

`pk1 != pk2 ==> encode_did_key(pk1, c) != encode_did_key(pk2, c)`

##### Scenario: Two different Ed25519 keys
`r[ucan-did.injective.scenario-distinct-keys]`

- GIVEN pk1 != pk2, both 32 bytes
- WHEN encoding both as did:key with Ed25519 codec
- THEN the encoded strings differ

### DID Format

#### Requirement: DID Prefix Is Valid
`r[ucan-did.prefix-valid]`

An encoded did:key for the supported codec set of this change MUST
always start with the string `"did:key:z"`.

##### Scenario: Any key encoding
`r[ucan-did.prefix-valid.scenario-any-key]`

- GIVEN any public key and supported codec
- WHEN encoding as did:key
- THEN the result starts with "did:key:z"

#### Requirement: Codec Is Preserved
`r[ucan-did.codec-preserved]`

The codec recovered from decoding MUST match the codec used for encoding
within the supported codec set of this change.

`decode_did_key(encode_did_key(pk, c)) == Some((_, c))`

### Multicodec

#### Requirement: Multicodec Varint Round-Trip
`r[ucan-did.varint-round-trip]`

Encoding a codec as a varint and decoding MUST recover the
original codec value.

`varint_decode(varint_encode(code)) == Some((code, varint_encode(code).len()))`

##### Scenario: Ed25519 codec 0xed
`r[ucan-did.varint-round-trip.scenario-ed25519]`

- GIVEN codec value 0xed
- WHEN encoding as varint [0xed, 0x01]
- THEN decoding returns (0xed, 2)

#### Requirement: Varint Decode Rejects Truncated Input
`r[ucan-did.varint-rejects-empty]`

Decoding a varint from an empty or truncated byte sequence
MUST return None.

##### Scenario: Empty varint input rejects
`r[ucan-did.varint-rejects-empty.scenario-empty]`

- GIVEN an empty byte sequence
- WHEN decoding as a varint
- THEN the result is None

##### Scenario: Truncated continuation rejects
`r[ucan-did.varint-rejects-empty.scenario-truncated]`

- GIVEN a non-empty byte sequence ending in a continuation byte
- WHEN decoding as a varint
- THEN the result is None

### Multibase

#### Requirement: Base58btc Round-Trip
`r[ucan-did.base58btc-round-trip]`

Encoding bytes as base58btc and decoding MUST recover the
original bytes.

`base58btc_decode(base58btc_encode(bytes)) == Some(bytes)`

#### Requirement: Base58btc Decode Rejects Invalid Characters
`r[ucan-did.base58btc-invalid-char-rejects]`

Decoding a string containing characters outside the base58
alphabet MUST return None.

##### Scenario: Invalid base58 character rejects
`r[ucan-did.base58btc-invalid-char-rejects.scenario-invalid-char]`

- GIVEN a string containing a character outside the base58 alphabet
- WHEN decoding as base58btc
- THEN the result is None

#### Requirement: DID Decode Rejects Wrong Prefix
`r[ucan-did.wrong-prefix-rejects]`

Decoding a byte string that does not start with `"did:key:z"` MUST
return None.

##### Scenario: Wrong DID prefix rejects
`r[ucan-did.wrong-prefix-rejects.scenario-wrong-prefix]`

- GIVEN a byte string that does not start with `"did:key:z"`
- WHEN decoding as did:key
- THEN the result is None

#### Requirement: DID Decode Rejects Unsupported Payload Structure
`r[ucan-did.malformed-payload-rejects]`

Decoding a `did:key` whose multibase payload does not decode, whose
multicodec framing does not parse, or whose codec is outside the
supported codec set of this change MUST return None.

##### Scenario: Malformed payload rejects
`r[ucan-did.malformed-payload-rejects.scenario-malformed-payload]`

- GIVEN a `did:key` string whose payload is malformed after the prefix
- WHEN decoding as did:key
- THEN the result is None
