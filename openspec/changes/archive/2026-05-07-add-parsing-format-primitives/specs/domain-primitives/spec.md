# Parsing and Format Primitive Specification

## Purpose

Defines spec-only requirements for reusable finite parsing and data-format
primitives. These primitives MUST use exact sequence, span, tree-shape, and
metadata models and MUST avoid broad parser-generator, language-semantics,
external-codec, performance, compression, or security claims unless a later
focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Grammar Symbol Admission [r[domain-primitives.parsing-format.grammar-symbol]]
The system MUST define finite grammar-symbol admission for terminal and
nonterminal identifiers without claiming grammar completeness or recognition of
an external language.

#### Scenario: Symbol admission rejects malformed symbol metadata [r[domain-primitives.parsing-format.grammar-symbol.scenario.rejects-malformed-symbol-metadata]]

- GIVEN a grammar symbol with a kind tag and bounded identifier metadata
- WHEN the kind tag or identifier bounds are invalid
- THEN the checked admission predicate rejects the symbol without mutating caller-owned token or grammar state

### Requirement: Token Stream Shape [r[domain-primitives.parsing-format.token-stream]]
The system MUST define finite token-stream shape checks for token kinds, source
spans, and monotone ordering over a finite token sequence.

#### Scenario: Token stream admits monotone non-overlapping spans [r[domain-primitives.parsing-format.token-stream.scenario.admits-monotone-nonoverlapping-spans]]

- GIVEN a finite token sequence with half-open source spans
- WHEN every token span is valid and ordered after the prior token span
- THEN the stream shape predicate admits the sequence as structurally well-formed

### Requirement: Source Span Predicate [r[domain-primitives.parsing-format.source-span]]
The system MUST define half-open source span predicates for bounded start/end
offsets, containment, overlap, and disjointness over a finite source length.

#### Scenario: Source span rejects out-of-bounds ranges [r[domain-primitives.parsing-format.source-span.scenario.rejects-out-of-bounds-ranges]]

- GIVEN a finite source length and a candidate half-open span
- WHEN the start is greater than the end or the end exceeds the source length
- THEN the span predicate rejects the candidate

### Requirement: AST Shape Envelope [r[domain-primitives.parsing-format.ast-shape]]
The system MUST define finite AST shape envelopes for node kind metadata, child
range bounds, arity limits, root admission, and acyclic child references.

#### Scenario: AST shape rejects dangling child ranges [r[domain-primitives.parsing-format.ast-shape.scenario.rejects-dangling-child-ranges]]

- GIVEN a finite AST node table and a node with a declared child range
- WHEN the child range exceeds the node table bounds or violates the arity limit
- THEN the AST shape envelope rejects the node table

### Requirement: Parse Error Envelope [r[domain-primitives.parsing-format.error-envelope]]
The system MUST define deterministic parse-error metadata envelopes containing a
bounded source position, error kind, and optional expected-symbol metadata.

#### Scenario: Error envelope admits bounded diagnostic metadata [r[domain-primitives.parsing-format.error-envelope.scenario.admits-bounded-diagnostic-metadata]]

- GIVEN a finite source length and deterministic parser decision metadata
- WHEN the error position is within bounds and the error kind is admitted
- THEN the error envelope admits the diagnostic metadata without claiming user-facing message quality

### Requirement: Format Version Admission [r[domain-primitives.parsing-format.version-admission]]
The system MUST define exact format-version admission for finite version tags,
accepted version sets, and closed-open supported version ranges.

#### Scenario: Version admission rejects unsupported versions [r[domain-primitives.parsing-format.version-admission.scenario.rejects-unsupported-versions]]

- GIVEN a finite accepted version set or supported version range
- WHEN an input version is outside the accepted metadata
- THEN the version admission predicate rejects the input before format-specific decoding proceeds

### Requirement: Serializer Shape Envelope [r[domain-primitives.parsing-format.serialize-shape]]
The system MUST define serializer output-shape envelopes for finite emitted
segments, length bounds, ordering metadata, and deterministic failure admission.

#### Scenario: Serializer shape preserves declared segment order [r[domain-primitives.parsing-format.serialize-shape.scenario.preserves-declared-segment-order]]

- GIVEN a finite list of serializer output segments with declared offsets and lengths
- WHEN the segment metadata is admitted
- THEN the output-shape envelope states that segment offsets are monotone and within the declared output length

### Requirement: Deserializer Shape Envelope [r[domain-primitives.parsing-format.deserialize-shape]]
The system MUST define deserializer input-shape and structural result envelopes
for finite fields, consumed ranges, remaining ranges, and deterministic
admission or rejection.

#### Scenario: Deserializer shape separates consumed and remaining input [r[domain-primitives.parsing-format.deserialize-shape.scenario.separates-consumed-and-remaining-input]]

- GIVEN a finite input and a structural deserializer result
- WHEN the result is admitted
- THEN the consumed range and remaining range are bounded, non-overlapping, and cover only admitted portions of the input

### Requirement: Bounded Round-Trip Envelope [r[domain-primitives.parsing-format.roundtrip-envelope]]
The system MUST define bounded canonical round-trip predicates only for
explicitly admitted finite values, versions, and shape envelopes.

#### Scenario: Round-trip envelope stays canonical and bounded [r[domain-primitives.parsing-format.roundtrip-envelope.scenario.stays-canonical-and-bounded]]

- GIVEN an admitted finite value, an accepted format version, and a declared canonical serializer/deserializer envelope
- WHEN a later implementation claims round-trip preservation
- THEN the claim is scoped to the admitted canonical value set and does not imply arbitrary external-format compatibility
