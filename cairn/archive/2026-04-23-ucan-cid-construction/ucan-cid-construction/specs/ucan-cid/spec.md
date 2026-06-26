# UCAN CID Specification

## Purpose

Defines the landed CID framing model used by `src/ucan_cid.rs`. In the
landed module, CID construction is modeled at the byte/spec layer:
varints are internalized, multihash and CID byte layouts are explicit,
multibase round-trip is stated over canonical CID byte sequences, and
runtime carrier structs exist without a public exec encode/decode API.

## Definitions

- **Multihash**: A self-describing hash framing value with byte layout
  `varint(code) ++ varint(digest_len) ++ digest`.
- **CID**: A CIDv1 framing value with byte layout
  `varint(version) ++ varint(codec) ++ multihash_bytes`.
- **Canonical CID bytes**: `cid_bytes_spec(cid)`.
- **Multibase round-trip**: `cid_decode_spec(cid_encode_spec(bytes, base))`.
- **Representable digest length**: A digest length that fits in `u64`.

## Requirements

### CID Construction

#### Requirement: CID Construction Is Deterministic
`r[ucan-cid.cid-deterministic]`

Constructing a CID from the same content and codec MUST always produce
the same CID.

`cid_from_bytes_spec(codec, content) == cid_from_bytes_spec(codec, content)`

##### Scenario: Repeated construction is identical
`r[ucan-cid.cid-deterministic.scenario-repeated-construction]`

- GIVEN content bytes and a fixed codec
- WHEN constructing the CID twice
- THEN the two CID values are equal

#### Requirement: CID Version Is Always 1
`r[ucan-cid.cid-version]`

Constructed CIDs MUST always have version 1.

##### Scenario: Constructed CID uses version 1
`r[ucan-cid.cid-version.scenario-version-one]`

- GIVEN any codec and multihash
- WHEN constructing `cid_from_multihash_spec(codec, mh)`
- THEN the resulting `version` field is `1`

### CID Round-Trip

#### Requirement: CID Encode-Decode Round-Trip
`r[ucan-cid.cid-round-trip]`

Encoding canonical CID bytes as a multibase string and decoding MUST
recover the original CID byte sequence.

`cid_decode_spec(cid_encode_spec(cid_bytes_spec(cid), base)) == Some(cid_bytes_spec(cid))`

##### Scenario: Base32 round-trip preserves CID bytes
`r[ucan-cid.cid-round-trip.scenario-base32-bytes]`

- GIVEN a CID value and a multibase selector
- WHEN encoding `cid_bytes_spec(cid)` and decoding the result
- THEN the decoded bytes equal `cid_bytes_spec(cid)`

### CID Injectivity

#### Requirement: Equal CID Bytes Imply Equal Components
`r[ucan-cid.cid-injectivity]`

If two CIDs have the same byte representation, their `version` and
`codec` fields MUST be identical.

`cid_bytes_spec(a) == cid_bytes_spec(b) ==> a.version == b.version && a.codec == b.codec`

##### Scenario: Equal bytes imply equal header
`r[ucan-cid.cid-injectivity.scenario-equal-bytes-equal-header]`

- GIVEN CIDs `a` and `b` with equal canonical CID bytes
- WHEN comparing their headers
- THEN `a.version == b.version` and `a.codec == b.codec`

### Multihash

#### Requirement: Multihash Digest Length Matches
`r[ucan-cid.multihash-digest-length]`

The digest in a wrapped multihash MUST have the same length as the input
digest.

`multihash_wrap_spec(code, digest).digest.len() == digest.len()`

##### Scenario: SHA-256 digest length stays 32
`r[ucan-cid.multihash-digest-length.scenario-sha256-digest]`

- GIVEN a 32-byte digest and SHA-256 code `0x12`
- WHEN wrapping it as a multihash
- THEN the wrapped digest length is `32`

#### Requirement: Multihash Round-Trip
`r[ucan-cid.multihash-round-trip]`

Encoding a multihash to bytes and parsing MUST recover the original code
and digest for representable digest lengths.

`multihash_decode_spec(multihash_bytes_spec(mh)) == Some((mh, multihash_bytes_spec(mh).len()))`

##### Scenario: Wrap then decode recovers the multihash
`r[ucan-cid.multihash-round-trip.scenario-wrap-then-decode]`

- GIVEN a multihash with digest length that fits in `u64`
- WHEN encoding it to bytes and decoding those bytes
- THEN the decoded multihash equals the original input

### Varint

#### Requirement: Varint Encode-Decode Round-Trip
`r[ucan-cid.varint-round-trip]`

Encoding a value as a varint and decoding MUST recover the original
value.

`varint_decode_spec(varint_encode_spec(n)) == Some((n, varint_encode_spec(n).len()))`

##### Scenario: Single-byte raw codec round-trip
`r[ucan-cid.varint-round-trip.scenario-single-byte-raw]`

- GIVEN `n = 0x55`
- WHEN encoding and decoding the varint
- THEN the result is `Some((0x55, 1))`

##### Scenario: Two-byte varint round-trip
`r[ucan-cid.varint-round-trip.scenario-two-byte-300]`

- GIVEN `n = 300`
- WHEN encoding and decoding the varint
- THEN the result is `Some((300, 2))`

#### Requirement: Varint Decode Rejects Empty Input
`r[ucan-cid.varint-rejects-empty]`

Decoding a varint from an empty byte sequence MUST return `None`.

##### Scenario: Empty input returns none
`r[ucan-cid.varint-rejects-empty.scenario-empty-none]`

- GIVEN an empty byte sequence
- WHEN decoding a varint
- THEN the result is `None`
