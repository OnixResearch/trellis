## Context

The repository now has text/string primitives for finite byte strings and spans.
Parsing and format primitives should build on those concepts but remain separate:
text primitives state what a byte/string span is, while parsing/format primitives
state how finite token streams, syntax trees, version metadata, and structural
serializer/deserializer envelopes are admitted.

## Goals / Non-Goals

**Goals:**

- define exact finite parsing and data-format primitive requirements
- keep validity boundaries explicit for token streams, spans, AST shapes, and
  format metadata
- support later Verus proofs using bounded sequences, ranges, and tree-shape
  envelopes
- avoid broad runtime parser/codec/library claims

**Non-Goals:**

- implementing modules in this change
- proving parser-generator completeness or arbitrary grammar recognition
- proving external format conformance, compression, performance, or security
- modeling unbounded parse forests, probabilistic parsers, or language semantics

## Decisions

### 1. Model parsing as finite metadata, not full language semantics

**Choice:** Requirements describe terminals/nonterminals, token streams, source
spans, AST node kinds, arity, child ranges, and error metadata as finite
sequences and bounded integer fields.

**Rationale:** These contracts are reusable and Verus-friendly. They let later
parsers prove structural safety without needing to prove a full language
semantics in the first slice.

**Alternative:** Start with a grammar recognizer or parser-combinator API.
Rejected because grammar completeness, ambiguity, and semantic actions are too
broad for a reusable primitive baseline.

### 2. Keep serializer/deserializer claims structural and bounded

**Choice:** Format primitives admit version metadata, output/input shape
envelopes, deterministic structural decisions, and explicitly bounded canonical
round-trip predicates.

**Rationale:** Shape and admission facts are useful across codecs and formats.
Full external-format compatibility and semantic equivalence require dedicated
format-specific OpenSpecs.

**Alternative:** Promise generic codec correctness. Rejected because different
formats have incompatible canonicalization, ordering, error, and extension
semantics.

### 3. Separate parsing/format primitives from text/string primitives

**Choice:** This child change owns `parse_*` and `format_*` primitives. Existing
`add-text-string-primitives` owns byte strings, UTF-8 admission, spans, prefix/
suffix predicates, edit envelopes, normalization envelopes, and delimiter shapes.

**Rationale:** Parsing/format primitives can reference text/string requirements
later without duplicating low-level string facts.

## Risks / Trade-offs

**Over-claiming parser correctness** → Requirements stay limited to finite
metadata admission, deterministic structural envelopes, and explicit bounded
predicates.

**Duplicating text/string requirements** → This change uses text/string spans as
future dependencies but owns only parse/format structures.

**Codec/security drift** → Format requirements exclude cryptographic security,
compression ratio, wire compatibility, and external library behavior unless a
later focused change adds exact format-specific models.

## Validation Plan

- `openspec validate add-parsing-format-primitives --strict --json`
- strict validation for all active primitive-family OpenSpecs
- future implementation: focused cargo tests, Verus checks, Tracey uncovered/
  untested checks, and final repo validation before archive
