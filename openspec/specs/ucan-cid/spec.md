# UCAN CID Construction Specification

## Purpose

Defines correctness properties for Content Identifier (CID) construction
and verification as used by UCAN proof references. Covers multihash
wrapping, CID byte layout, multibase encoding, and round-trip correctness.

## Definitions

- **CID**: A Content Identifier — a self-describing content-addressed
  identifier: version + content codec + multihash.
- **Multihash**: A self-describing hash digest: hash function code
  (varint) + digest length (varint) + digest bytes.
- **Multicodec**: A varint code identifying the content type
  (0x71 for dag-cbor, 0x55 for raw).
- **Multibase**: A prefix character identifying the string encoding
  ('b' for base32lower, 'z' for base58btc).
- **Varint**: An unsigned variable-length integer (LEB128 encoding).

## Requirements

### CID Construction

#### Requirement: CID Construction Is Deterministic

Constructing a CID from the same content, codec, and hash function
MUST always produce the same CID.

`cid_from_bytes(codec, hash_code, content) == cid_from_bytes(codec, hash_code, content)`

##### Scenario: Repeated construction

- GIVEN content bytes, dag-cbor codec, SHA-256 hash
- WHEN constructing CID twice
- THEN both CIDs are identical

#### Requirement: CID Version Is Always 1

Constructed CIDs MUST always have version 1.

##### Scenario: Any construction

- GIVEN any content, codec, and hash function
- WHEN constructing a CID
- THEN the version field is 1

### CID Round-Trip

#### Requirement: CID Encode-Decode Round-Trip

Encoding a CID as a multibase string and decoding MUST recover
the original CID.

`cid_decode(cid_encode(cid, base)) == Some(cid)`

##### Scenario: Base32 round-trip

- GIVEN a CID with dag-cbor codec and SHA-256 multihash
- WHEN encoding as base32lower ('b' prefix)
- THEN decoding the string returns the original CID

##### Scenario: Base58btc round-trip

- GIVEN the same CID
- WHEN encoding as base58btc ('z' prefix)
- THEN decoding the string returns the original CID

### CID Injectivity

#### Requirement: Equal CID Bytes Imply Equal Components

If two CIDs have the same byte representation, their version,
codec, and multihash MUST be identical.

`cid_bytes(a) == cid_bytes(b) ==> a.version == b.version && a.codec == b.codec && a.multihash == b.multihash`

### Multihash

#### Requirement: Multihash Digest Length Matches

The digest length field in a multihash MUST equal the actual
length of the digest bytes.

`multihash_wrap(code, digest).digest_len == digest.len()`

##### Scenario: SHA-256 digest

- GIVEN a 32-byte SHA-256 digest
- WHEN wrapping as multihash with code 0x12
- THEN digest_len == 32

#### Requirement: Multihash Round-Trip

Encoding a multihash to bytes and parsing MUST recover the
original code and digest.

`multihash_decode(multihash_bytes(mh)) == Some(mh)`

### Varint

#### Requirement: Varint Encode-Decode Round-Trip

Encoding a non-negative integer as a varint and decoding MUST
recover the original value.

`varint_decode(varint_encode(n)) == Some((n, varint_encode(n).len()))`

##### Scenario: Single-byte value

- GIVEN n = 0x55 (raw codec)
- WHEN encoding as varint [0x55]
- THEN decoding returns (0x55, 1)

##### Scenario: Multi-byte value

- GIVEN n = 0x71 (dag-cbor codec)
- WHEN encoding as varint [0x71]
- THEN decoding returns (0x71, 1)

##### Scenario: Two-byte varint

- GIVEN n = 0x12 (SHA-256 hash code, which is < 128 so single byte)
- WHEN encoding as varint [0x12]
- THEN decoding returns (0x12, 1)

#### Requirement: Varint Decode Rejects Empty Input

Decoding a varint from an empty byte sequence MUST return None.
