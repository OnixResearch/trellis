## Why

`verified-logic` has many byte-oriented codecs and metadata checks, but reusable
text/string facts should be specified as exact finite primitives before parsing,
format, protocol, search, or UI-facing changes duplicate them. Finite strings,
byte ranges, character ranges, token spans, prefix/suffix predicates, bounded
edit distance, and normalization envelopes are common proof building blocks.

This change adds a focused spec-only package for text/string primitives. It uses
finite byte sequences, explicit encoding/admission metadata, and deterministic
span predicates. It does not claim full Unicode normalization correctness,
locale-aware collation, regex-engine behavior, language parsing, or NLP quality.

## What Changes

- Add `text_*` domain-primitive requirements for:
  - finite byte string admission and length bounds
  - UTF-8 character boundary/range metadata
  - token spans over finite byte/character ranges
  - prefix and suffix predicates over finite sequences
  - substring containment and match span admission
  - bounded edit-distance envelope metadata
  - normalization envelopes that track source/output spans without claiming full
    Unicode normalization correctness
  - delimiter split/join shape metadata
- Leave implementation, proofs, Tracey tags, tests, and evidence as future
  tasks.
- Update the `add-domain-primitives` umbrella taxonomy so reusable text/string
  requirements are not duplicated by future parser, format, protocol, ML, or UI
  child changes.

## Capabilities

### New Capabilities

- `domain-primitives.text-string-family`: reusable finite text/string taxonomy
  and required child-change scope.
- `text.finite-byte-string`: finite byte sequence length/admission metadata.
- `text.utf8-boundary-range`: UTF-8 boundary and character-range metadata over a
  finite byte string.
- `text.token-span`: finite token span metadata with byte/character range
  correspondence.
- `text.prefix-suffix`: prefix/suffix predicates and length bounds.
- `text.substring-span`: substring containment and match span admission.
- `text.edit-distance-envelope`: bounded edit-operation count metadata without
  algorithmic optimality claims unless proved later.
- `text.normalization-envelope`: source/output span and length envelope metadata
  without full Unicode normalization correctness claims.
- `text.delimiter-split-join`: delimiter-driven split/join shape metadata over
  finite segments.

## Impact

- **Files**: future implementation may add `src/text_*`, `src/lib.rs`,
  `docs/requirements.md`, tests, README entries, and OpenSpec evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new runtime dependencies.
- **Testing**: this package validates through `openspec validate`; future
  implementation slices must add focused tests, Verus proofs, Tracey coverage,
  and final repo validation.

## Out of Scope

- implementing modules in this change
- full Unicode normalization/collation correctness, locale-aware behavior,
  regex-engine correctness, language parsing, grammar validation, or NLP quality
- bindings to external text/regex/parser libraries
- cross-family parser/format/protocol layers before reusable text primitives land
