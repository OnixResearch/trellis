# Text String Primitives Delta Specification

## ADDED Requirements

### Requirement: Finite Byte String [r[text.finite-byte-string]]
The system MUST define a finite byte-string primitive with explicit length
bounds, valid byte-index admission, and no-mutation rejection semantics for
checked public operations.

#### Scenario: Checked byte string admits finite length and indices [r[text.finite-byte-string.scenario.checked-byte-string-admits-finite-length-and-indices]]

- GIVEN a finite byte sequence and representable length metadata
- WHEN a checked byte-string constructor or checker evaluates the sequence
- THEN it admits only byte indices within the finite length and exposes the length as representable metadata

#### Scenario: Invalid byte string metadata is rejected without mutation [r[text.finite-byte-string.scenario.invalid-byte-string-metadata-rejected-without-mutation]]

- GIVEN a previously valid byte-string value
- WHEN a public checked operation receives overflowing length, invalid slice, or out-of-bounds index metadata
- THEN it rejects the input explicitly and leaves the previously valid value unchanged

### Requirement: UTF-8 Boundary Range [r[text.utf8-boundary-range]]
The system MUST define UTF-8 boundary/range metadata over finite byte strings for
admitted character-boundary indices and half-open character/byte spans.

#### Scenario: UTF-8 boundary range admits only character boundaries [r[text.utf8-boundary-range.scenario.utf8-boundary-range-admits-only-character-boundaries]]

- GIVEN a finite byte string and a candidate byte range
- WHEN the UTF-8 boundary-range primitive admits the range
- THEN the range start and end are in bounds and occur on declared UTF-8 character boundaries

#### Scenario: Invalid UTF-8 range is rejected [r[text.utf8-boundary-range.scenario.invalid-utf8-range-rejected]]

- GIVEN a byte range that is out of bounds, reversed, or not aligned to admitted UTF-8 character boundaries
- WHEN a checked UTF-8 range operation evaluates the range
- THEN it rejects the range without exposing character-index metadata for it

### Requirement: Token Span [r[text.token-span]]
The system MUST define token-span metadata over finite text with token index,
byte span, optional character span, and deterministic span-order constraints.

#### Scenario: Token span maps token to admitted byte and character ranges [r[text.token-span.scenario.token-span-maps-token-to-admitted-byte-and-character-ranges]]

- GIVEN finite text, token metadata, and an admitted token index
- WHEN the token-span primitive evaluates the token
- THEN it exposes a finite half-open byte span and, when UTF-8 metadata is present, a corresponding admitted character span

#### Scenario: Token spans are ordered or explicitly overlapping by policy [r[text.token-span.scenario.token-spans-are-ordered-or-explicitly-overlapping-by-policy]]

- GIVEN multiple token spans in finite text
- WHEN the token-span metadata declares a non-overlap policy
- THEN each later token span starts at or after the previous token span end

### Requirement: Prefix and Suffix Predicate [r[text.prefix-suffix]]
The system MUST define finite prefix and suffix predicates with length bounds,
byte/character layer declaration, and exact sequence equality over admitted
positions.

#### Scenario: Prefix predicate compares only admitted prefix positions [r[text.prefix-suffix.scenario.prefix-predicate-compares-only-admitted-prefix-positions]]

- GIVEN finite haystack and prefix sequences with admitted byte or character layer metadata
- WHEN the prefix predicate accepts the prefix
- THEN the prefix length is less than or equal to the haystack length and every admitted prefix position equals the corresponding haystack position

#### Scenario: Suffix predicate compares only admitted suffix positions [r[text.prefix-suffix.scenario.suffix-predicate-compares-only-admitted-suffix-positions]]

- GIVEN finite haystack and suffix sequences with admitted byte or character layer metadata
- WHEN the suffix predicate accepts the suffix
- THEN the suffix length is less than or equal to the haystack length and every admitted suffix position equals the corresponding haystack position at the declared offset

### Requirement: Substring Span [r[text.substring-span]]
The system MUST define substring containment and match-span admission over finite
text with deterministic span metadata for accepted matches.

#### Scenario: Substring match span is finite and in bounds [r[text.substring-span.scenario.substring-match-span-is-finite-and-in-bounds]]

- GIVEN finite haystack and needle sequences plus a declared byte or character layer
- WHEN the substring-span primitive admits a match
- THEN the match span is within the haystack bounds and every admitted needle position equals the corresponding haystack position inside that span

#### Scenario: Substring absence does not expose match span [r[text.substring-span.scenario.substring-absence-does-not-expose-match-span]]

- GIVEN finite haystack and needle sequences
- WHEN the substring-span primitive reports no admitted match
- THEN it exposes absence metadata without constructing an out-of-bounds or inconsistent match span

### Requirement: Edit Distance Envelope [r[text.edit-distance-envelope]]
The system MUST define a bounded edit-distance envelope primitive for finite
source/target lengths, operation-budget metadata, and conservative lower/upper
bounds on accepted edit operations.

#### Scenario: Edit-distance envelope admits only within declared budget [r[text.edit-distance-envelope.scenario.edit-distance-envelope-admits-only-within-declared-budget]]

- GIVEN finite source and target lengths plus an edit-operation budget
- WHEN the edit-distance envelope accepts a candidate edit path or summary
- THEN the accepted operation count is representable, within the declared budget, and consistent with the finite source/target length envelope

#### Scenario: Edit-distance envelope avoids unproved optimality claims [r[text.edit-distance-envelope.scenario.edit-distance-envelope-avoids-unproved-optimality-claims]]

- GIVEN an edit-distance envelope requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim algorithmic optimality or Levenshtein distance equality unless a dedicated proof for that algorithm is also specified

### Requirement: Normalization Envelope [r[text.normalization-envelope]]
The system MUST define normalization envelope metadata for finite source/output
spans, length-growth bounds, supported normalization form tags, and explicit
unsupported-form rejection.

#### Scenario: Normalization envelope bounds output span metadata [r[text.normalization-envelope.scenario.normalization-envelope-bounds-output-span-metadata]]

- GIVEN finite source text, a supported normalization form tag, and declared output length bounds
- WHEN the normalization-envelope primitive admits normalized output metadata
- THEN the output span is finite, representable, and within the declared conservative length-growth envelope

#### Scenario: Normalization envelope avoids Unicode table correctness claims [r[text.normalization-envelope.scenario.normalization-envelope-avoids-unicode-table-correctness-claims]]

- GIVEN a normalization-envelope requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim full Unicode normalization table correctness, locale collation, or semantic text equivalence unless a dedicated model owns those claims

### Requirement: Delimiter Split Join Shape [r[text.delimiter-split-join]]
The system MUST define delimiter split/join shape metadata over finite text,
finite delimiter spans, segment bounds, and deterministic empty-segment policy.

#### Scenario: Delimiter split exposes finite ordered segments [r[text.delimiter-split-join.scenario.delimiter-split-exposes-finite-ordered-segments]]

- GIVEN finite text, finite delimiter metadata, and a declared empty-segment policy
- WHEN the delimiter split-shape primitive accepts the metadata
- THEN it exposes finite ordered segment spans that are in bounds and consistent with the delimiter positions and empty-segment policy

#### Scenario: Join shape accounts for delimiters between segments [r[text.delimiter-split-join.scenario.join-shape-accounts-for-delimiters-between-segments]]

- GIVEN finite segment lengths and finite delimiter length metadata
- WHEN the delimiter join-shape primitive computes or validates output length metadata
- THEN the output length envelope accounts for every segment and every inserted delimiter between adjacent segments without overflow
