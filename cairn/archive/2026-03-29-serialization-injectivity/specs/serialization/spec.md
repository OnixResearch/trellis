# Serialization Injectivity Specification

## Purpose

Proves that canonical byte encodings of structures are injective:
equal byte sequences imply equal input values. Composes with
collision-resistance axioms (from chain.rs and snapshot_integrity.rs)
to give the full guarantee: same hash → same structure.

## Requirements

### Requirement: u64 Little-Endian Encoding

The system MUST provide `encode_u64_spec(v: u64) -> Seq<u8>` that
produces an 8-byte little-endian encoding.

The encoding MUST be injective: `encode_u64_spec(a) == encode_u64_spec(b) → a == b`.

The encoding MUST produce exactly 8 bytes.

#### Scenario: Roundtrip

- GIVEN any u64 value `v`
- WHEN `encode_u64_spec(v)` produces bytes `bs`
- THEN `decode_u64_spec(bs) == v`

#### Scenario: Injectivity

- GIVEN two u64 values `a` and `b`
- WHEN `encode_u64_spec(a) == encode_u64_spec(b)`
- THEN `a == b`

#### Scenario: Fixed length

- GIVEN any u64 value `v`
- WHEN `encode_u64_spec(v)` produces bytes `bs`
- THEN `bs.len() == 8`

### Requirement: Tag-Prefixed Encoding

The system MUST provide `encode_tagged_spec(tag: u8, payload: Seq<u8>) -> Seq<u8>`
that prepends a single tag byte to the payload.

Different tags MUST produce different encodings regardless of payload
content: if `tag1 != tag2`, then
`encode_tagged_spec(tag1, p1) != encode_tagged_spec(tag2, p2)`
for all payloads `p1`, `p2`.

Same tag with equal payloads MUST produce equal encodings (determinism).

#### Scenario: Tag disjointness

- GIVEN two different tag bytes `t1 != t2`
- WHEN any payloads `p1` and `p2` are encoded
- THEN `encode_tagged_spec(t1, p1) != encode_tagged_spec(t2, p2)`

#### Scenario: Same-tag injectivity

- GIVEN the same tag byte `t`
- WHEN `encode_tagged_spec(t, p1) == encode_tagged_spec(t, p2)`
- THEN `p1 == p2`

### Requirement: Length-Prefixed Encoding

The system MUST provide `encode_len_prefixed_spec(data: Seq<u8>) -> Seq<u8>`
that prepends an 8-byte little-endian length header to the data.

The encoding MUST be injective:
`encode_len_prefixed_spec(a) == encode_len_prefixed_spec(b) → a == b`.

#### Scenario: Injectivity

- GIVEN two byte sequences `a` and `b`
- WHEN `encode_len_prefixed_spec(a) == encode_len_prefixed_spec(b)`
- THEN `a == b`

#### Scenario: Length recovery

- GIVEN a byte sequence `data`
- WHEN `encode_len_prefixed_spec(data)` produces `encoded`
- THEN the first 8 bytes of `encoded` decode to `data.len()`

### Requirement: Pair Encoding

The system MUST provide `encode_pair_spec(a: u64, b: u64) -> Seq<u8>`
that concatenates two fixed-width u64 encodings.

The encoding MUST be injective:
`encode_pair_spec(a1, b1) == encode_pair_spec(a2, b2) → a1 == a2 && b1 == b2`.

The output MUST be exactly 16 bytes.

#### Scenario: Injectivity

- GIVEN two pairs `(a1, b1)` and `(a2, b2)`
- WHEN `encode_pair_spec(a1, b1) == encode_pair_spec(a2, b2)`
- THEN `a1 == a2` and `b1 == b2`

#### Scenario: Fixed length

- GIVEN any pair `(a, b)`
- WHEN encoded
- THEN output is 16 bytes

### Requirement: Quad Encoding

The system MUST provide `encode_quad_spec(a: u64, b: u64, c: u64, d: u64) -> Seq<u8>`
that concatenates four fixed-width u64 encodings.

The encoding MUST be injective:
`encode_quad_spec(a1,b1,c1,d1) == encode_quad_spec(a2,b2,c2,d2)`
implies all four fields are pairwise equal.

The output MUST be exactly 32 bytes.

#### Scenario: Injectivity

- GIVEN two quads `(a1,b1,c1,d1)` and `(a2,b2,c2,d2)`
- WHEN their encodings are equal
- THEN all four fields are pairwise equal

#### Scenario: Fixed length

- GIVEN any quad
- WHEN encoded
- THEN output is 32 bytes

### Requirement: Chain Hash Structure Injectivity

The system MUST prove that `hash_spec(prev1, idx1, term1, data1) ==
hash_spec(prev2, idx2, term2, data2)` implies all four fields are
equal, using encode_quad_injective composed with hash_collision_free.

This is the existing `hash_collision_free` axiom restated as a
composition of serialization injectivity and collision resistance,
showing the two-step reasoning chain explicitly.

#### Scenario: Composition

- GIVEN the chain hash of two entries is equal
- WHEN decomposed via encode_quad_injective + hash_collision_free
- THEN all four fields match (same conclusion as hash_collision_free,
  but via the explicit serialize → hash reasoning chain)

### Requirement: Dual Hash Structure Injectivity

The system MUST prove that `dual_hash_spec(a1, b1) == dual_hash_spec(a2, b2)`
implies `a1 == a2 && b1 == b2`, using encode_pair_injective composed
with dual_hash_collision_resistant.

#### Scenario: Composition

- GIVEN the dual hash of two pairs is equal
- WHEN decomposed via encode_pair_injective + dual_hash_collision_resistant
- THEN both fields match
