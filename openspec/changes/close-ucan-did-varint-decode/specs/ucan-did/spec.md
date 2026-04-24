# UCAN DID Model Specification

## Purpose

This change hardens the `did:key` multicodec varint layer by replacing the
trusted runtime decoder bridge with verified bounded u64 LEB128 decode
primitives. Base58btc remains a separate trusted codec model.

## Requirements

### Requirement: Varint Decode Uses Bounded U64 Semantics

r[ucan-did.varint-bounded-u64]
`varint_decode_spec` and `varint_decode` MUST model the same bounded u64
LEB128 decoder. The decoder MUST use named limits for the u64 byte budget,
shift width, continuation bit, and payload mask. The concrete formulas are:
`VARINT_GROUP_BITS = 7`, `VARINT_CONTINUATION_BIT = 1 << VARINT_GROUP_BITS`,
`VARINT_PAYLOAD_MASK = VARINT_CONTINUATION_BIT - 1`, `U64_BIT_WIDTH = 64`,
`MAX_U64_VARINT_BYTES = (U64_BIT_WIDTH + VARINT_GROUP_BITS - 1) /
VARINT_GROUP_BITS`, `MAX_U64_VARINT_SHIFT = VARINT_GROUP_BITS *
(MAX_U64_VARINT_BYTES - 1)`, and `max_payload_for_shift(shift) = u64::MAX >>
shift`. It MUST return `Some((value, consumed))` only when a terminating byte is
found within `MAX_U64_VARINT_BYTES` and the accumulated value is representable as
`u64`. Any `Some` result MUST satisfy `1 <= consumed <= input.len()` and
`consumed <= MAX_U64_VARINT_BYTES`. It MUST return `None` for empty input,
continuation-only input, overlong encodings, and any terminating byte whose
payload exceeds `max_payload_for_shift(current_shift)`. For this change,
"overlong" means "no terminating byte appears before
`MAX_U64_VARINT_BYTES` is exhausted"; non-minimal encodings that terminate
within the budget and remain representable are accepted.

#### Scenario: Terminating byte returns value and consumed length

- GIVEN a byte sequence whose first byte has no continuation bit
- WHEN decoding it as a varint
- THEN the result is `Some((first_byte, 1))`

#### Scenario: Ed25519 codec decodes from two bytes

- GIVEN the byte sequence `[0xed, 0x01]`
- WHEN decoding it as a varint
- THEN the result is `Some((0xed, 2))`

#### Scenario: Trailing bytes are not consumed

- GIVEN `varint_encode_spec(code) ++ suffix`
- WHEN decoding it as a varint
- THEN the result is `Some((code, varint_encode_spec(code).len()))`

#### Scenario: Overlong or overflowing input rejects

- GIVEN a byte sequence with no terminating byte before the named u64 varint
  byte budget is exhausted, or whose final payload exceeds the remaining u64
  bits
- WHEN decoding it as a varint
- THEN the result is `None`

#### Scenario: Non-minimal representable input is accepted

- GIVEN a non-minimal byte sequence that terminates within the named u64 varint
  byte budget and represents a `u64` value
- WHEN decoding it as a varint
- THEN the result is `Some((value, consumed))` with consumed bounded by the input
  length and the named u64 varint byte budget

### Requirement: Multicodec Varint Round-Trip Remains Verified

r[ucan-did.varint-round-trip]
Encoding any `u64` value as LEB128 and then decoding it with the bounded decoder
MUST recover the original value and the encoded length.

`varint_decode(varint_encode(code)) == Some((code, varint_encode(code).len()))`

#### Scenario: Any encoded u64 round-trips

- GIVEN any `u64` codec value
- WHEN it is encoded with `varint_encode` and decoded with `varint_decode`
- THEN the decoded value equals the original codec and the consumed length equals
  the encoded length

### Requirement: Varint Decode Rejects Empty Or Truncated Input

r[ucan-did.varint-rejects-empty]
Decoding a varint from an empty byte sequence or from a sequence that ends
before a terminating byte MUST return `None`.

#### Scenario: Empty varint input rejects

- GIVEN an empty byte sequence
- WHEN decoding as a varint
- THEN the result is `None`

#### Scenario: Truncated continuation rejects

- GIVEN a non-empty byte sequence ending in a continuation byte before any
  terminating byte
- WHEN decoding as a varint
- THEN the result is `None`
