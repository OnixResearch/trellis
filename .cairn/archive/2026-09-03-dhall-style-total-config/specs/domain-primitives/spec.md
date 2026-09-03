# Typed Configuration Delta

## Purpose

Adds Dhall-inspired finite typed configuration normal-form primitives to Trellis without adopting Dhall syntax or replacing Nickel. The requirements define total typed expression admission, deterministic normal-form metadata, semantic digest input selection, safe import-closure admission, and explicit non-claim boundaries.

## ADDED Requirements

### Requirement: Typed Config Expression Envelope
r[domain-primitives.typed-config.expression-envelope] The system MUST define finite typed configuration expression envelopes for literals, records, variants/options, lists, references, and import metadata with explicit node identifiers, type identifiers, and bounded child references.

#### Scenario: Finite expression graph is admitted
r[domain-primitives.typed-config.expression-envelope.finite]
- GIVEN a finite expression envelope whose child references point to declared nodes and whose list/record metadata is representable
- WHEN expression-envelope admission is evaluated
- THEN the expression is accepted as a finite candidate for type admission.

#### Scenario: Dangling expression reference is rejected
r[domain-primitives.typed-config.expression-envelope.dangling-reference]
- GIVEN an expression envelope with a child, reference, or import node identifier absent from the finite expression set
- WHEN expression-envelope admission is evaluated
- THEN the expression is rejected with deterministic missing-node metadata before normalization is attempted.

### Requirement: Typed Config Type Admission
r[domain-primitives.typed-config.type-admission] The system MUST define type-admission predicates that accept only expressions whose value shape matches the declared finite schema/type envelope and reject malformed, absent, or unsupported values before normalization or downstream use.

#### Scenario: Required fields type-check before normalization
r[domain-primitives.typed-config.type-admission.required-fields]
- GIVEN a record expression containing every required field with values admitted by the corresponding field type
- WHEN type admission is evaluated
- THEN the record is accepted for normalization.

#### Scenario: Type mismatch is rejected before use
r[domain-primitives.typed-config.type-admission.type-mismatch]
- GIVEN an expression whose declared type tag requires one value shape but whose finite value envelope carries a different shape
- WHEN type admission is evaluated
- THEN the expression is rejected with deterministic type-mismatch metadata before normalization or digest input selection.

#### Scenario: Missing required field is rejected explicitly
r[domain-primitives.typed-config.type-admission.missing-field]
- GIVEN a schema requiring a field that is absent from the finite record expression
- WHEN type admission is evaluated
- THEN the expression is rejected with deterministic missing-field metadata.

### Requirement: Explicit Config Failure Metadata
r[domain-primitives.typed-config.explicit-failure] The system MUST represent absence, malformed values, unsupported expression forms, digest mismatches, and import-policy failures as explicit decision metadata rather than panics, unchecked unwraps, or implicit default success.

#### Scenario: Unsupported expression form fails closed
r[domain-primitives.typed-config.explicit-failure.unsupported-form]
- GIVEN a candidate expression whose kind is outside the supported finite typed-config subset
- WHEN admission is evaluated
- THEN the decision rejects the expression with unsupported-form metadata.

#### Scenario: Optional absence cannot satisfy required value
r[domain-primitives.typed-config.explicit-failure.optional-required]
- GIVEN an optional or absent value where the schema requires a present non-optional value
- WHEN type admission is evaluated
- THEN the decision rejects the expression instead of fabricating a default.

### Requirement: Deterministic Config Normal Form
r[domain-primitives.typed-config.normal-form] The system MUST define deterministic normal-form metadata for admitted typed configuration expressions, including canonical record-field ordering, canonical variant/optional representation, canonical list order preservation, and source-name/comment independence.

#### Scenario: Record normalizes to canonical field order
r[domain-primitives.typed-config.normal-form.record-order]
- GIVEN two admitted record expressions with the same typed fields presented in different source orders
- WHEN normal-form metadata is computed
- THEN both records produce the same canonical field sequence.

#### Scenario: Source-only refactor preserves normal form
r[domain-primitives.typed-config.normal-form.source-refactor]
- GIVEN two admitted expressions that differ only by local source names, comments, whitespace, or non-semantic ordering admitted by the normalizer
- WHEN normal-form metadata is computed
- THEN the normal-form metadata is equal.

#### Scenario: Normalization only follows type admission
r[domain-primitives.typed-config.normal-form.requires-type-admission]
- GIVEN a malformed expression that fails type admission
- WHEN normalization is requested
- THEN normalization returns explicit rejection metadata and no successful normal-form claim is exported.

### Requirement: Semantic Digest Input Selection
r[domain-primitives.typed-config.semantic-digest] The system MUST define semantic digest input selection over canonical normal-form metadata and declared dependency digest sequences, rather than source text, comments, whitespace, local names, or unresolved import syntax.

#### Scenario: Textual refactor keeps digest input stable
r[domain-primitives.typed-config.semantic-digest.refactor-stable]
- GIVEN two admitted expressions with equal canonical normal-form metadata and equal dependency digest sequences
- WHEN semantic digest input selection is evaluated
- THEN the selected digest input is equal for both expressions.

#### Scenario: Dependency digest drift changes digest input
r[domain-primitives.typed-config.semantic-digest.dependency-drift]
- GIVEN two admitted expressions with equal local normal-form metadata but different declared dependency digest sequences
- WHEN semantic digest input selection is evaluated
- THEN the selected digest input differs or reports dependency-drift metadata.

### Requirement: Typed Config Import Closure
r[domain-primitives.typed-config.import-closure] The system MUST define finite import-closure admission for frozen imports, declared semantic digest checks, no computed imports, no conditional import leakage, remote import attenuation, and same-origin header forwarding metadata.

#### Scenario: Frozen import admits matching digest
r[domain-primitives.typed-config.import-closure.frozen-match]
- GIVEN an import edge whose fetched normal-form digest metadata matches the declared frozen digest
- WHEN import-closure admission is evaluated
- THEN the import edge is accepted as digest-aligned within the finite model.

#### Scenario: Frozen import mismatch fails closed
r[domain-primitives.typed-config.import-closure.frozen-mismatch]
- GIVEN an import edge whose fetched normal-form digest metadata differs from the declared frozen digest
- WHEN import-closure admission is evaluated
- THEN the import edge is rejected with digest-mismatch metadata.

#### Scenario: Computed import is rejected
r[domain-primitives.typed-config.import-closure.no-computed-import]
- GIVEN an import target whose path, origin, or header scope depends on a normalized configuration value
- WHEN import-closure admission is evaluated
- THEN the import edge is rejected with computed-import metadata.

#### Scenario: Conditional import set cannot leak state
r[domain-primitives.typed-config.import-closure.no-conditional-leakage]
- GIVEN an expression that would fetch different import sets depending on a configuration branch or value
- WHEN import-closure admission is evaluated
- THEN the expression is rejected or the full static import set is required independently of branch selection.

#### Scenario: Remote import cannot access local or environment source
r[domain-primitives.typed-config.import-closure.remote-attenuation]
- GIVEN an expression imported from a remote origin whose transitive import edge targets a local file, home path, or environment variable source
- WHEN import-closure admission is evaluated
- THEN the transitive import is rejected with remote-to-local metadata.

#### Scenario: Headers forward only within same origin
r[domain-primitives.typed-config.import-closure.same-origin-headers]
- GIVEN an import edge with header metadata and a transitive relative import within the same origin
- WHEN import-closure admission is evaluated
- THEN header forwarding is admitted only for the same-origin relative target and rejected for cross-origin targets.

### Requirement: Typed Config Semantic Refactoring
r[domain-primitives.typed-config.semantic-refactoring] The system MUST define refactoring admission for typed config expressions where equal normal-form digest inputs admit behavior-preserving source refactors, and digest-changing refactors require explicit compatibility or unresolved-dependent metadata.

#### Scenario: Equal digest input admits source refactor
r[domain-primitives.typed-config.semantic-refactoring.equal-digest]
- GIVEN a refactor step whose before and after expressions have equal semantic digest input metadata
- WHEN refactor admission is evaluated
- THEN the step is admitted as behavior-preserving within the finite typed-config model.

#### Scenario: Digest-changing refactor requires compatibility metadata
r[domain-primitives.typed-config.semantic-refactoring.digest-change]
- GIVEN a refactor step whose before and after expressions have different semantic digest input metadata
- WHEN no compatibility witness or unresolved-dependent record is declared
- THEN the step is rejected with digest-changing-refactor metadata.

### Requirement: Typed Config Evidence Coverage
r[domain-primitives.typed-config.positive] The implementation MUST include positive evidence for typed record normalization, source refactor with stable digest input, matching frozen import, and same-origin relative import header metadata.

#### Scenario: Positive fixtures cover accepted behavior
r[domain-primitives.typed-config.positive.fixtures]
- GIVEN the typed-config primitive test suite
- WHEN positive fixtures run
- THEN they cover accepted type admission, canonical normalization, stable digest input, frozen-import match, and same-origin header forwarding.

### Requirement: Typed Config Negative Coverage
r[domain-primitives.typed-config.negative] The implementation MUST include negative evidence for type mismatch, missing required field, unsupported expression form, forbidden computed import, conditional import leakage, remote-to-local or remote-to-environment import, cross-origin header forwarding, frozen digest mismatch, and digest-changing refactor without compatibility.

#### Scenario: Negative fixtures cover failure modes
r[domain-primitives.typed-config.negative.fixtures]
- GIVEN the typed-config primitive test suite
- WHEN negative fixtures run
- THEN they reject every declared failure mode with deterministic metadata rather than panicking or silently accepting malformed input.

### Requirement: Typed Config Documentation Sync
r[domain-primitives.typed-config.docs] The implementation MUST update Trellis requirement, README/module-catalog, and accepted Cairn spec documentation for any public typed-config module or requirement IDs that land in source.

#### Scenario: Requirement IDs are traceable
r[domain-primitives.typed-config.docs.traceable]
- GIVEN the typed-config module lands in `src/`
- WHEN Tracey and documentation catalog checks run
- THEN the relevant `docs/requirements.md`, README/catalog, and Cairn spec entries expose matching requirement IDs and public module descriptions.

### Requirement: Typed Config Validation Evidence
r[domain-primitives.typed-config.final-validation] The change MUST capture focused validation evidence for Cargo tests, Verus verification as needed for the new module or full library, Tracey uncovered/untested checks, and Cairn validation/gates before archive.

#### Scenario: Final validation names commands and outcomes
r[domain-primitives.typed-config.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final validation evidence is inspected
- THEN it names the commands run, their outcomes, and any scoped limitations or blockers.

### Requirement: Typed Config Non-Claim Boundary
r[domain-primitives.typed-config.non-claims] The typed-config primitive MUST state that it does not prove Dhall parser correctness, full Dhall language compatibility, network security, filesystem security, environment secrecy, Nickel evaluation, cryptographic hash collision resistance, runtime import fetching, cache correctness, or downstream system correctness.

#### Scenario: Digest boundary is explicit
r[domain-primitives.typed-config.non-claims.digest-boundary]
- GIVEN semantic digest metadata is accepted by the typed-config primitive
- WHEN a reviewer reads the exported claim
- THEN the claim is limited to deterministic digest input selection and does not assert collision resistance or hash implementation correctness.

#### Scenario: Import policy is a finite model
r[domain-primitives.typed-config.non-claims.import-boundary]
- GIVEN import-closure admission succeeds in the typed-config primitive
- WHEN a reviewer reads the exported claim
- THEN the claim is limited to the declared finite import metadata and does not assert actual HTTP, filesystem, environment, CORS, credential, or cache behavior.
