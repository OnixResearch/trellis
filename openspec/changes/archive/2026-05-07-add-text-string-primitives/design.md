## Context

Many verified components eventually need text-like facts: a byte string has a
finite length, a UTF-8 range starts and ends on character boundaries, a token span
maps to a byte interval, a string has a prefix/suffix, a match span is in bounds,
an edit-distance budget is not exceeded, normalization has a conservative length
or span envelope, and split/join metadata preserves segment shape.

## Goals / Non-Goals

**Goals:**

- define exact finite text/string primitive requirements
- distinguish byte-level, UTF-8-boundary, character-range, and token-span models
- state checked-constructor versus `requires` validity boundaries
- avoid broad Unicode, regex, parser, NLP, or locale correctness claims

**Non-Goals:**

- implement text/string modules now
- prove full Unicode normalization, collation, regex, or parser correctness
- specify language grammars or NLP/model quality
- bind to external text-processing libraries

## Decisions

### 1. Model text as finite byte strings with optional UTF-8 admission

**Choice:** The foundation primitive is a finite byte sequence; UTF-8 boundary
and character-range primitives are separate admitted metadata layers.

**Rationale:** Byte-level length/index facts are broadly reusable, and UTF-8
validity should be explicit instead of assumed by every caller.

**Alternative:** Treat all strings as abstract characters only. Rejected because
many codec/protocol surfaces are byte-indexed and need byte-span proofs.

### 2. Keep spans explicit and half-open

**Choice:** Byte ranges, character ranges, token spans, and match spans use
finite half-open bounds plus correspondence metadata when crossing layers.

**Rationale:** Half-open spans compose cleanly with slicing, prefix/suffix,
substring, split, and join facts.

### 3. Treat edit distance as an envelope first

**Choice:** Bounded edit-distance primitives specify operation budgets and
admission envelopes; algorithmic optimality is a future proof task only if a
specific implementation claims it.

**Rationale:** Budget and no-overflow facts are useful without committing to a
particular dynamic-programming algorithm or proof cost.

### 4. Treat normalization as structural metadata first

**Choice:** Normalization envelope primitives specify source/output span,
length-growth bounds, idempotence tags when declared, and rejection of unsupported
forms without proving full Unicode normalization tables.

**Rationale:** This provides safe metadata for callers while avoiding unsupported
large Unicode-data correctness claims.

## Risks / Trade-offs

**Accidental Unicode correctness claims** → Requirements explicitly avoid full
normalization/collation correctness unless a later dedicated model owns the data
and proof obligations.

**Duplicate parser/format requirements** → This child owns reusable text/span
facts; future parser, format, protocol, or UI changes should reference these IDs
for shared string metadata.

**Byte/character ambiguity** → Requirements require explicit byte-vs-character
range metadata and checked UTF-8 boundary admission before character indexing.

## Validation Plan

- `openspec validate add-text-string-primitives --strict --json`
- validate every active primitive-family change after umbrella updates
- future implementation slices run focused cargo tests, `./scripts/verus.sh
  src/lib.rs`, Tracey uncovered/untested checks, and final repo validation
