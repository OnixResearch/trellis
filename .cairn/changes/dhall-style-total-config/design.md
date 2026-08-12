## Design

The change introduces a finite typed configuration normal-form primitive. It adapts Dhall's safety model into Trellis's usual `spec fn` plus checked `exec fn` style: pure data in, deterministic admission or rejection out, and no ambient I/O in the verified core.

### Decisions

1. **Model Dhall-style semantics, not Dhall compatibility.** The primitive is inspired by Dhall's total evaluation, canonical normal forms, semantic hashes, frozen imports, and safe import policy. It does not parse Dhall syntax, claim interoperability with Dhall's standard, or replace Nickel as the default stack configuration language.

2. **Finite expression envelope.** Expressions are finite values with explicit node/type/import identifiers. The first implementation slice should avoid unrestricted recursion and keep fold-like behavior bounded by input list length and Verus decreases arguments.

3. **Type admission comes before normalization.** A configuration expression is normalized only after type admission succeeds. Missing required fields, type/value mismatches, unknown constructors, and unsupported expression forms produce deterministic rejection metadata.

4. **Normal form is the semantic identity input.** Digest metadata is computed from the canonical normal-form envelope and declared dependency digest sequence, not source text, comments, whitespace, or local names. The verified surface proves only that the digest input is selected deterministically; cryptographic hash collision resistance remains an explicit trusted/runtime boundary.

5. **Imports are capabilities with policy.** Import metadata is modeled as finite edges with origin, target, declared digest, optional header scope, and kind tags. The checked policy rejects computed imports, conditional import leakage, remote-to-local/environment import edges, cross-origin header forwarding, and digest mismatches.

6. **Semantic refactoring is digest-aware.** Refactors that preserve the normal-form digest are admitted as behavior-preserving within this finite model. Refactors that change the digest require explicit compatibility or unresolved-dependent metadata through the existing semantic-refactoring pattern.

7. **Non-claims stay explicit.** Passing the primitive does not prove parser correctness, network security, filesystem security, environment secrecy, complete Dhall language semantics, Nickel evaluation, BLAKE3 implementation correctness, or downstream system correctness.

### Functional core and shell boundary

The implementation should keep the core as pure Verus-visible functions over in-memory values:

- expression/type/import envelopes;
- admission predicates;
- first-failure metadata helpers;
- normalization constructors;
- digest-input selection;
- import-policy predicates.

Any future parser, Nickel/Dhall importer, file reader, HTTP fetcher, cache manager, environment lookup, or digest implementation belongs in a thin runtime shell and must feed explicit finite inputs into the verified core.

### Validation shape

Positive evidence should cover:

- a typed record with required fields normalizing to a canonical field order;
- a source-name/comment-preserving refactor that keeps the same normal-form digest input;
- a frozen import whose declared digest matches the imported normal form;
- same-origin relative import header forwarding metadata that stays within the origin.

Negative evidence should cover:

- type mismatch before normalization;
- missing required field;
- forbidden computed or conditional import;
- remote import attempting local or environment access;
- cross-origin header forwarding;
- frozen import digest mismatch;
- digest-changing refactor without compatibility or unresolved-dependent metadata.

### Documentation and lifecycle sync

The implementation should add matching `docs/requirements.md` entries because Tracey coverage in Trellis is driven by that file, then sync the accepted Cairn spec after implementation and archive the change with captured validation evidence.
