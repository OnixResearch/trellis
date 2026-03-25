# Bitfield Specification

## Purpose

Verified multi-bit field extraction and insertion within u64 words.
Generalizes single-bit ops (bitwise.rs) to arbitrary-width fields.

## Requirements

### Requirement: Field Mask

The system MUST compute a bitmask covering bits [offset, offset+width).
The mask MUST have exactly `width` consecutive 1-bits starting at position
`offset`. When width is 0 the mask MUST be 0.

#### Scenario: Basic mask
- GIVEN offset=4, width=3
- WHEN field_mask is called
- THEN result is 0b0111_0000 (0x70)

#### Scenario: Zero width
- GIVEN any offset, width=0
- WHEN field_mask is called
- THEN result is 0

### Requirement: Extract Field

The system MUST extract bits [offset, offset+width) from a word, right-shifted
to the least-significant position. The result MUST be less than 2^width.

#### Scenario: Extract middle bits
- GIVEN value=0xFF, offset=4, width=4
- WHEN extract_field is called
- THEN result is 0xF

#### Scenario: Extract zero-width
- GIVEN any value, any offset, width=0
- WHEN extract_field is called
- THEN result is 0

### Requirement: Insert Field

The system MUST replace bits [offset, offset+width) of a word with the
given field value, leaving all other bits unchanged. The field value
MUST fit within `width` bits (field < 2^width).

#### Scenario: Insert into cleared word
- GIVEN value=0, field=0xA, offset=4, width=4
- WHEN insert_field is called
- THEN result is 0xA0

#### Scenario: Insert overwrites existing
- GIVEN value=0xFF, field=0x5, offset=4, width=4
- WHEN insert_field is called
- THEN result is 0x5F

### Requirement: Clear Field

The system MUST zero bits [offset, offset+width) of a word, leaving all
other bits unchanged.

#### Scenario: Clear middle bits
- GIVEN value=0xFF, offset=4, width=4
- WHEN clear_field is called
- THEN result is 0x0F

### Requirement: Round-trip

Extracting a field after inserting it MUST return the original field value.
extract_field(insert_field(v, f, o, w), o, w) == f for any f < 2^width.

### Requirement: Non-interference

Inserting a field MUST NOT change bits outside [offset, offset+width).
For any bit position p outside the field, bit_at(insert_field(...), p) ==
bit_at(original, p).

### Requirement: Clear-then-extract

Extracting from a cleared field MUST return 0.
extract_field(clear_field(v, o, w), o, w) == 0.

### Requirement: Extract bounded

The result of extract_field MUST always be less than 2^width (fits in
`width` bits).
