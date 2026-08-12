# Serialization Codec Primitives Specification

## Purpose

Defines spec-only requirements for reusable finite serialization/codec
primitives. These primitives MUST use exact finite byte buffers, framed-record
envelopes, checked length-prefix metadata, schema/version admission, checksum
shape metadata, encoder/decoder shape predicates, and bounded round-trip
envelopes. They MUST avoid arbitrary external codec conformance, streaming I/O,
compression ratio, encryption, authentication, checksum integrity, corruption
detection, transport delivery, storage durability, performance, or semantic
equivalence claims unless a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Byte Buffer Envelope [r[domain-primitives.serialization-codec.byte-buffer]]
The system MUST provide a finite byte-buffer envelope that records byte length,
capacity bounds, and admitted byte ranges without claiming ownership of any
external buffer implementation.

#### Scenario: Buffer range must fit finite length [r[domain-primitives.serialization-codec.byte-buffer.scenario.buffer-range-must-fit-finite-length]]

- GIVEN a finite byte buffer envelope and a candidate half-open byte range
- WHEN the range end exceeds the admitted buffer length
- THEN the byte-buffer primitive rejects the range or requires a proven bound before use

### Requirement: Framed Record Envelope [r[domain-primitives.serialization-codec.frame]]
The system MUST provide framed-record envelopes whose header, payload, and
optional trailer ranges are finite, nonnegative, and contained within the
admitted byte-buffer envelope.

#### Scenario: Frame sections must stay within buffer [r[domain-primitives.serialization-codec.frame.scenario.frame-sections-must-stay-within-buffer]]

- GIVEN a frame envelope with header, payload, and trailer metadata
- WHEN any section range falls outside the admitted buffer length
- THEN the frame primitive rejects the envelope or requires a proof that the section is in bounds

### Requirement: Length Prefix Envelope [r[domain-primitives.serialization-codec.length-prefix]]
The system MUST provide checked length-prefix metadata that ties a declared
finite payload length to the remaining admitted byte window.

#### Scenario: Declared length must fit remaining payload [r[domain-primitives.serialization-codec.length-prefix.scenario.declared-length-must-fit-remaining-payload]]

- GIVEN a length prefix and a remaining finite payload window
- WHEN the declared length is greater than the remaining window length
- THEN the length-prefix primitive rejects the envelope or exposes an explicit invalid-length decision

### Requirement: Schema Version Admission [r[domain-primitives.serialization-codec.schema-version]]
The system MUST provide deterministic schema/version admission metadata that
records finite schema identifiers, supported version ranges, and explicit
unknown-version rejection.

#### Scenario: Unknown version rejected deterministically [r[domain-primitives.serialization-codec.schema-version.scenario.unknown-version-rejected-deterministically]]

- GIVEN finite supported schema/version metadata
- WHEN a record declares an unknown schema or unsupported version
- THEN the schema-version primitive returns an explicit rejection decision without mutating the input envelope

### Requirement: Checksum Shape Metadata [r[domain-primitives.serialization-codec.checksum-shape]]
The system MUST provide checksum shape metadata for algorithm identifiers,
checksum byte ranges, and covered byte ranges without claiming checksum
integrity, corruption detection, collision resistance, or cryptographic
security.

#### Scenario: Checksum field range must be structural only [r[domain-primitives.serialization-codec.checksum-shape.scenario.checksum-field-range-must-be-structural-only]]

- GIVEN checksum metadata for an admitted finite buffer
- WHEN the checksum field range or covered range is outside the buffer
- THEN the checksum-shape primitive rejects the metadata without claiming whether any checksum value is correct

### Requirement: Encoder Shape Predicate [r[domain-primitives.serialization-codec.encoder-shape]]
The system MUST provide deterministic encoder shape predicates that relate an
admitted finite model value to finite output-buffer shape metadata.

#### Scenario: Encoder output must satisfy declared shape [r[domain-primitives.serialization-codec.encoder-shape.scenario.encoder-output-must-satisfy-declared-shape]]

- GIVEN an admitted finite model value and encoder shape predicate
- WHEN an encoder output length or section layout violates the declared shape
- THEN the encoder-shape primitive rejects the output metadata or requires a proof of shape preservation

### Requirement: Decoder Shape Predicate [r[domain-primitives.serialization-codec.decoder-shape]]
The system MUST provide deterministic decoder shape predicates that accept or
reject finite byte buffers based on declared byte-level envelopes before any
format-specific semantic claim is made.

#### Scenario: Decoder rejects malformed finite buffer shape [r[domain-primitives.serialization-codec.decoder-shape.scenario.decoder-rejects-malformed-finite-buffer-shape]]

- GIVEN a finite byte buffer and decoder shape predicate
- WHEN required frame, length, schema, or section metadata is malformed
- THEN the decoder-shape primitive returns an explicit rejection decision without claiming external format conformance

### Requirement: Bounded Round-Trip Envelope [r[domain-primitives.serialization-codec.round-trip]]
The system MUST provide bounded round-trip envelopes only for explicitly
admitted finite model values, schemas, versions, output buffers, and decoder
shape predicates.

#### Scenario: Round trip requires admitted finite model [r[domain-primitives.serialization-codec.round-trip.scenario.round-trip-requires-admitted-finite-model]]

- GIVEN a candidate model value and serialization/codec round-trip envelope
- WHEN the value, schema, version, or output shape is not admitted by the finite model
- THEN the round-trip primitive rejects the claim or requires the missing admission proof before asserting a bounded round trip
