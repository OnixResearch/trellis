## Why

Reusable verified byte/serialization logic currently gets specified ad hoc by
format-specific modules. A focused serialization/codec primitive family gives
future changes exact finite building blocks for byte buffers, framed records,
length-prefix metadata, schema/version admission, checksum-shape metadata,
encoder/decoder shape predicates, and bounded round-trip envelopes without
claiming generic codec correctness.

## What Changes

- Add a spec-only `add-serialization-codec-primitives` child OpenSpec under the
  domain-primitives umbrella.
- Define exact finite models for byte-buffer envelopes, frame envelopes,
  length-prefix envelopes, schema/version admission, checksum metadata shape,
  encoder/decoder shape predicates, and bounded round-trip envelopes.
- Leave implementation, Verus proofs, Tracey coverage, and evidence as future
  tasks.

## Capabilities

### New Capabilities

- `domain-primitives.serialization-codec.byte-buffer`: finite byte-buffer
  envelope and capacity admission.
- `domain-primitives.serialization-codec.frame`: finite framed-record envelope
  with header/payload/trailer ranges.
- `domain-primitives.serialization-codec.length-prefix`: checked length-prefix
  metadata tied to remaining payload bounds.
- `domain-primitives.serialization-codec.schema-version`: deterministic
  schema/version admission metadata.
- `domain-primitives.serialization-codec.checksum-shape`: checksum field shape
  and presence metadata without checksum-integrity claims.
- `domain-primitives.serialization-codec.encoder-shape`: deterministic encoder
  output-shape predicate.
- `domain-primitives.serialization-codec.decoder-shape`: deterministic decoder
  admission/rejection shape predicate.
- `domain-primitives.serialization-codec.round-trip`: bounded round-trip
  envelope over explicitly admitted finite models.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add
  `src/codec_*`, `src/serialize_*`, `src/deserialize_*`, `src/frame_*`,
  `docs/requirements.md`, tests, and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new dependencies.
- **Testing**: strict OpenSpec validation now; future implementation must run
  focused tests, Verus, Tracey, and final repo validation.

## Out of Scope

- implementing serialization or codec modules now
- arbitrary parser/codec conformance for external formats
- compression ratio, encryption, authentication, or security claims
- checksum collision resistance or corruption-detection guarantees
- streaming I/O, transport delivery, durability, or performance claims
- floating-point, locale, Unicode normalization, or semantic-equivalence claims
  beyond explicitly declared finite models
