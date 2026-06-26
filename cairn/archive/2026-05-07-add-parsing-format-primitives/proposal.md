## Why

Finite text/string primitives define byte ranges and spans, but reusable parsing
and data-format reasoning needs its own focused family. Token streams, grammar
symbol metadata, parse spans, AST shape envelopes, serializer/deserializer shape
checks, and format-version admission are common proof building blocks across
configuration, codecs, protocols, query languages, and document formats.

This change keeps those primitives spec-only for now. It records exact finite
models that later Verus slices can implement without claiming full language
parsing, parser-generator completeness, schema-language conformance, wire-format
performance, or semantic equivalence for arbitrary external formats.

## What Changes

- Add focused `add-parsing-format-primitives` requirements for finite parsing
  and data-format primitives.
- Scope the first implementation plan to exact sequence/tree/metadata models:
  - `parse_grammar_symbol`: finite terminal/nonterminal symbol admission.
  - `parse_token_stream`: token sequence shape, offsets, and monotone spans.
  - `parse_span`: half-open source spans and containment/overlap predicates.
  - `parse_ast_shape`: finite AST node kind/arity/child-range envelopes.
  - `parse_error_envelope`: deterministic error position/kind admission.
  - `format_version_admission`: accepted version/range metadata.
  - `format_serialize_shape`: serializer output-shape envelope.
  - `format_deserialize_shape`: deserializer input-shape and structural result envelope.
  - `format_roundtrip_envelope`: explicitly bounded canonical round-trip predicates.
- Update the `add-domain-primitives` umbrella so this family is tracked as a
  separate child change.

## Capabilities

### New Capabilities

- `domain-primitives.parsing-format-family`: reusable finite parsing/format
  primitive taxonomy and required child-change scope.
- `parse_grammar_symbol`: exact terminal/nonterminal symbol admission.
- `parse_token_stream`: finite token-stream metadata and monotone span checks.
- `parse_span`: half-open source span predicates.
- `parse_ast_shape`: finite AST shape/arity/child-range envelope.
- `parse_error_envelope`: deterministic parse error metadata envelope.
- `format_version_admission`: accepted format-version metadata.
- `format_serialize_shape`: serializer output-shape envelope.
- `format_deserialize_shape`: deserializer input-shape/result envelope.
- `format_roundtrip_envelope`: bounded canonical round-trip predicate envelope.

## Impact

- **Files**: future implementation may add `src/parse_*`, `src/format_*`,
  `src/lib.rs`, `docs/requirements.md`, tests, and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new runtime dependencies.
- **Testing**: current verification is strict OpenSpec validation. Future
  implementation tasks must add focused unit/property tests, Verus proof checks,
  Tracey coverage, and final repo validation evidence.

## Out of Scope

- implementing parsing or format primitives in this change
- proving full parser-generator correctness, grammar completeness, ambiguity
  resolution, semantic analysis, or language conformance
- claiming full Unicode normalization, locale collation, regex-engine behavior,
  NLP quality, compression ratio, cryptographic security, or external codec
  compatibility
- proving arbitrary serializer/deserializer equivalence outside explicitly
  bounded canonical envelopes
