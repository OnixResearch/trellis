## Context

The repository already has focused primitive OpenSpecs for parsing/format and
text/string handling. Serialization/codec primitives are adjacent but distinct:
they model finite byte-level envelopes, frame metadata, version admission, and
shape-preserving encoder/decoder boundaries. They should not inherit broad
parser correctness or external codec conformance claims.

## Goals / Non-Goals

**Goals:**

- define exact finite byte and record-shape models
- keep validity boundaries explicit for checked constructors and proof-only specs
- support later verified composition of byte-buffer, frame, length, version, and
  round-trip predicates
- keep checksum metadata structural unless a later focused model proves a
  checksum algorithm

**Non-Goals:**

- implementing modules in this change
- proving generic codec correctness, external format conformance, compression,
  encryption, authentication, or checksum integrity
- modeling unbounded streaming, transport, filesystem, or performance behavior
- claiming semantic equivalence beyond declared finite encoder/decoder models

## Decisions

### 1. Model bytes and frames as finite envelopes

**Choice:** Byte buffers carry finite length/capacity metadata. Frames carry
header, payload, and optional trailer ranges that must be within the same finite
buffer.

**Rationale:** Bounds/range preservation is Verus-friendly and reusable across
format-specific children.

### 2. Keep length prefixes as checked metadata

**Choice:** Length-prefix primitives admit only prefixes whose decoded declared
length fits inside the remaining finite payload window.

**Rationale:** This proves structural admission without needing to verify every
future concrete varint or endian decoder in this family. Dedicated codecs can own
their exact length-decoding model later.

### 3. Separate checksum shape from checksum correctness

**Choice:** Checksum primitives describe field presence, byte range, algorithm ID
admission, and covered range metadata, but do not claim corruption detection or
cryptographic properties.

**Rationale:** Structural checksum placement is useful and verifiable; algorithm
strength is a separate trust/model boundary.

### 4. Bound round trips by admitted finite models

**Choice:** Round-trip envelopes are stated only for explicitly admitted values,
formats, schemas, versions, and finite output buffers.

**Rationale:** This keeps later proofs exact and prevents accidental generic
serializer/deserializer or semantic-equivalence claims.

## Risks / Trade-offs

**Overlap with parsing/format primitives** → Parsing/format owns grammar, token,
AST, parse-error, and semantic format-version envelopes; serialization/codec owns
byte buffers, frames, lengths, byte-level shape predicates, and bounded byte
round trips.

**Over-broad codec claims** → Requirements explicitly forbid compression,
security, streaming, external conformance, and performance guarantees unless a
later focused OpenSpec owns the exact model.

**Checksum ambiguity** → The initial primitive records checksum field shape and
algorithm metadata only; correctness of the algorithm or integrity guarantee is
out of scope.

## Validation Plan

- `openspec validate add-serialization-codec-primitives --strict --json`
- strict validation for every active primitive-family OpenSpec
- helper verification is expected to warn about incomplete future tasks
- future implementation slices must add Tracey/Verus evidence before archive
