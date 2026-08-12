## Why

Trellis has many verified serialization and language-adjacent primitives, but it does not yet have a small reusable S-expression data layer. Several future proof surfaces could benefit from a canonical tree format for proof manifests, finite interpreter metadata, test fixtures, and human-readable evidence without importing a whole Scheme reader or evaluator.

This change carves out a narrow Trellis-owned S-expression primitive: finite atoms and lists, canonical serialization, bounded parsing/admission metadata, and roundtrip facts. It is intentionally smaller than the active Scheme-logic work. Scheme-facing modules may later build on it, but this change does not claim Steel compatibility, Scheme standard compliance, macro behavior, evaluation semantics, or runtime reader completeness.

## What Changes

- Add a new `s-expression` Cairn spec for finite canonical S-expression primitives.
- Plan a `src/s_expression.rs` Verus module with pure AST/spec predicates and checked exec helpers.
- Define atom/list well-formedness, canonical serialization shape, bounded parse/admission results, size/depth budget handling, and deterministic error metadata.
- Require positive and negative tests for canonical values, roundtrips, malformed delimiters, invalid atoms, non-canonical spacing/escaping, and budget rejection.
- Require `docs/requirements.md`, source annotations, README/module catalog text, Tracey checks, Verus checks, and Cairn validation evidence before archive.

## Impact

- **Files**: new Cairn spec package now; later implementation should touch `src/s_expression.rs`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and focused tests.
- **Testing**: implementation must include positive and negative tests plus fresh Verus, Tracey, and Cairn validation evidence.
- **Boundaries**: no evaluator, no macro expander, no global interner, no filesystem/network/process behavior, and no compatibility claim for Steel, Scheme, Lisp, SMT-LIB, or any external S-expression dialect.
