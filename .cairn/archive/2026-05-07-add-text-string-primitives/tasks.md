## Coverage Index

| Requirement ID | Future module |
|---|---|
| `text.finite-byte-string` | `src/text_bytes.rs` |
| `text.utf8-boundary-range` | `src/text_utf8.rs` |
| `text.token-span` | `src/text_span.rs` |
| `text.prefix-suffix` | `src/text_prefix.rs` |
| `text.substring-span` | `src/text_substring.rs` |
| `text.edit-distance-envelope` | `src/text_edit_distance.rs` |
| `text.normalization-envelope` | `src/text_normalization.rs` |
| `text.delimiter-split-join` | `src/text_delimiter.rs` |

## Phase 1: Spec foundation

- [x] Add focused text/string primitive OpenSpec with exact finite requirements.

## Phase 2: Byte, UTF-8, and span primitives

- [x] Implement `text_bytes` finite byte-string admission with checked constructors, no-mutation rejection tests, and Verus length/index proofs. `[covers=text.finite-byte-string]`
- [x] Implement `text_utf8` UTF-8 boundary/range metadata with rejection tests for invalid boundaries. `[covers=text.utf8-boundary-range]`
- [x] Implement `text_span` token-span metadata with byte/character correspondence proofs. `[covers=text.token-span]`

## Phase 3: Predicate and match primitives

- [x] Implement `text_prefix` prefix/suffix predicates with length and unchanged-span proofs. `[covers=text.prefix-suffix]`
- [x] Implement `text_substring` substring containment and match-span admission with deterministic result metadata. `[covers=text.substring-span]`
- [x] Implement `text_delimiter` delimiter split/join shape metadata and segment-bound proofs. `[covers=text.delimiter-split-join]`

## Phase 4: Envelope primitives

- [x] Implement `text_edit_distance` bounded edit-distance envelope metadata and rejection tests for overflowing budgets. `[covers=text.edit-distance-envelope]`
- [x] Implement `text_normalization` structural normalization envelopes without Unicode-table correctness claims. `[covers=text.normalization-envelope]`

## Phase 5: Evidence and archive

- [x] Add `docs/requirements.md`, `r[impl]`, `r[depends]`, and `r[verify]` coverage for every text/string primitive requirement.
- [x] Run focused cargo tests for text/string primitives and capture evidence.
- [x] Run `./scripts/verus.sh src/lib.rs` and capture evidence.
- [x] Run Tracey uncovered/untested checks and final repo validation.
- [x] Sync OpenSpec deltas and archive `add-text-string-primitives`.
