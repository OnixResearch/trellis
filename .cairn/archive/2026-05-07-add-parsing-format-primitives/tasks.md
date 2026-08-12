## Coverage Index

| Requirement ID | Future module | Purpose |
|---|---|---|
| `domain-primitives.parsing-format.grammar-symbol` | `src/parse_grammar_symbol.rs` | finite terminal/nonterminal admission |
| `domain-primitives.parsing-format.token-stream` | `src/parse_token_stream.rs` | token sequence shape and monotone spans |
| `domain-primitives.parsing-format.source-span` | `src/parse_span.rs` | half-open source span predicates |
| `domain-primitives.parsing-format.ast-shape` | `src/parse_ast_shape.rs` | finite AST node/arity/child-range envelope |
| `domain-primitives.parsing-format.error-envelope` | `src/parse_error_envelope.rs` | deterministic parse error metadata |
| `domain-primitives.parsing-format.version-admission` | `src/format_version_admission.rs` | accepted format-version metadata |
| `domain-primitives.parsing-format.serialize-shape` | `src/format_serialize_shape.rs` | serializer output-shape envelope |
| `domain-primitives.parsing-format.deserialize-shape` | `src/format_deserialize_shape.rs` | deserializer input/result envelope |
| `domain-primitives.parsing-format.roundtrip-envelope` | `src/format_roundtrip_envelope.rs` | bounded canonical round-trip predicates |

## Phase 1: Spec foundation

- [x] Create parsing/format primitive OpenSpec with exact finite models and non-goals.

## Phase 2: Future implementation slices

- [x] Implement `parse_grammar_symbol` with terminal/nonterminal admission and proof coverage. `[covers=domain-primitives.parsing-format.grammar-symbol]`
- [x] Implement `parse_token_stream` with finite token sequence shape and monotone span checks. `[covers=domain-primitives.parsing-format.token-stream]`
- [x] Implement `parse_span` with half-open bounds, containment, overlap, and disjointness predicates. `[covers=domain-primitives.parsing-format.source-span]`
- [x] Implement `parse_ast_shape` with bounded node kind, arity, child-range, and acyclicity/tree-shape envelopes. `[covers=domain-primitives.parsing-format.ast-shape]`
- [x] Implement `parse_error_envelope` with deterministic error position/kind admission. `[covers=domain-primitives.parsing-format.error-envelope]`
- [x] Implement `format_version_admission` with accepted version/range metadata checks. `[covers=domain-primitives.parsing-format.version-admission]`
- [x] Implement `format_serialize_shape` with deterministic output-shape envelope checks. `[covers=domain-primitives.parsing-format.serialize-shape]`
- [x] Implement `format_deserialize_shape` with deterministic input-shape/result envelope checks. `[covers=domain-primitives.parsing-format.deserialize-shape]`
- [x] Implement `format_roundtrip_envelope` only for explicitly bounded canonical values. `[covers=domain-primitives.parsing-format.roundtrip-envelope]`

## Phase 3: Evidence and archive

- [x] Add `docs/requirements.md`, `r[impl]`, `r[depends]`, and `r[verify]` coverage for implemented parsing/format primitives.
- [x] Run focused tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and final repo validation.
- [x] Sync canonical specs and archive after implementation/evidence tasks complete.
