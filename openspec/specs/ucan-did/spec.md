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
- **Multicodec**: A varint prefix identifying the key type
  (0xed01 for Ed25519, 0x1200 for P-256).
- **Multibase**: A prefix character identifying the base encoding
  ('z' for base58btc).
- **Varint**: An unsigned variable-length integer encoding (LEB128).

## Requirements

### DID Round-Trip

#### Requirement: Encode-Decode Round-Trip

Encoding a public key as a did:key and decoding the result MUST
recover the original key and codec.

`decode_did_key(encode_did_key(pk, codec)) == Some((pk, codec))`

##### Scenario: Ed25519 key round-trips

- GIVEN a 32-byte Ed25519 public key pk
- WHEN encoding as did:key with Ed25519 codec
- THEN decoding the result returns (pk, Ed25519)

### DID Injectivity

#### Requirement: Distinct Keys Produce Distinct DIDs

If two public keys are different, their DID encodings MUST differ.

`pk1 != pk2 ==> encode_did_key(pk1, c) != encode_did_key(pk2, c)`

##### Scenario: Two different Ed25519 keys

- GIVEN pk1 != pk2, both 32 bytes
- WHEN encoding both as did:key with Ed25519 codec
- THEN the encoded strings differ

### DID Format

#### Requirement: DID Prefix Is Valid

An encoded did:key MUST always start with the string `"did:key:z"`.

##### Scenario: Any key encoding

- GIVEN any public key and codec
- WHEN encoding as did:key
- THEN the result starts with "did:key:z"

#### Requirement: Codec Is Preserved

The codec recovered from decoding MUST match the codec used for encoding.

`decode_did_key(encode_did_key(pk, c)) == Some((_, c))`

### Multicodec

#### Requirement: Multicodec Varint Round-Trip

Encoding a codec as a varint and decoding MUST recover the
original codec value.

`varint_decode(varint_encode(code)) == Some((code, varint_encode(code).len()))`

##### Scenario: Ed25519 codec 0xed

- GIVEN codec value 0xed
- WHEN encoding as varint [0xed, 0x01]
- THEN decoding returns (0xed, 2)

#### Requirement: Varint Decode Rejects Truncated Input

Decoding a varint from an empty or truncated byte sequence
MUST return None.

### Multibase

#### Requirement: Base58btc Round-Trip

Encoding bytes as base58btc and decoding MUST recover the
original bytes.

`base58btc_decode(base58btc_encode(bytes)) == Some(bytes)`

#### Requirement: Base58btc Decode Rejects Invalid Characters

Decoding a string containing characters outside the base58
alphabet MUST return None.
