# Durable Reference Identity Separation Specification

## Purpose

Defines distinct finite identity types for typed durable references. The requirements preserve malformed-input admission, existing Verus semantics, and the current storage and cryptographic non-claim boundaries.

## Requirements

### Requirement: Distinct Durable Reference Identity Types
r[newtypes.durable-identities.types] The typed durable reference model MUST use distinct structures for reference, value, type, codec, closure, and schema identities in every envelope, request, catalog, result, and conversion witness.

#### Scenario: Correct identity domains are accepted
r[newtypes.durable-identities.types.correct]
- GIVEN a durable reference whose fields use their declared identity types
- WHEN finite metadata and unpersist admission are evaluated
- THEN each comparison occurs only within the matching identity domain.

#### Scenario: Cross-domain substitution does not compile
r[newtypes.durable-identities.types.exchanged]
- GIVEN a program that supplies a closure identity as a type identity, or another mismatched domain
- WHEN Rust or Verus type checking evaluates the program
- THEN the program is rejected before semantic admission runs.

### Requirement: Invalid Raw Identities Remain Expressible
r[newtypes.durable-identities.invalid-inputs] Each identity structure MUST retain the complete raw `u64` value domain, including zero, so malformed finite metadata remains expressible and rejectable by explicit admission predicates.

#### Scenario: Zero identity is represented and rejected
r[newtypes.durable-identities.invalid-inputs.zero]
- GIVEN a durable reference with a zero raw value in a required identity field
- WHEN finite metadata admission is evaluated
- THEN the value is represented successfully and the reference is rejected by the existing validity rule.

#### Scenario: Constructor does not imply validity
r[newtypes.durable-identities.invalid-inputs.constructor]
- GIVEN any raw `u64` value
- WHEN its declared identity structure is constructed
- THEN construction alone makes no nonzero, cryptographic, storage, or migration claim.

### Requirement: Durable Reference Semantics Are Preserved
r[newtypes.durable-identities.preservation] For equivalent raw field values, the typed implementation MUST preserve finite metadata, closure availability, missing-closure reporting, unpersist type matching, and conversion-witness admission results.

#### Scenario: Matching type remains admitted
r[newtypes.durable-identities.preservation.type-match]
- GIVEN an admitted reference and request with equal `TypeHash` values and an available `ClosureHash`
- WHEN unpersist admission is evaluated
- THEN the request is accepted as before the migration.

#### Scenario: Incomplete witness remains rejected
r[newtypes.durable-identities.preservation.witness]
- GIVEN a conversion witness whose same-domain old or new identity differs from the referenced envelope
- WHEN migration admission is evaluated
- THEN the witness is rejected as before the migration.

### Requirement: Durable Identity Evidence
r[newtypes.durable-identities.evidence] The change MUST include positive semantic tests, negative malformed-input tests, and compile-fail evidence for representative cross-domain substitutions.

#### Scenario: Evidence covers semantic and structural behavior
r[newtypes.durable-identities.evidence.coverage]
- GIVEN the focused durable-reference evidence suite
- WHEN it runs
- THEN valid typed flows pass, malformed same-domain values fail admission, and mismatched domains fail to compile.

### Requirement: Durable Identity Traceability
r[newtypes.durable-identities.docs] The change MUST update canonical requirements, source and test annotations, public documentation, and accepted Cairn text for every new identity boundary.

#### Scenario: Identity requirements remain linked
r[newtypes.durable-identities.docs.traceable]
- GIVEN the completed migration
- WHEN Tracey and Cairn validation run
- THEN type, invalid-input, preservation, evidence, validation, and non-claim requirements have complete links.

### Requirement: Durable Identity Validation
r[newtypes.durable-identities.validation] The migration MUST pass focused and full Cargo tests, compile-fail checks, Clippy, full-library Verus, Tracey, proof-gap inventory, and Cairn validation without adding trusted proof boundaries.

#### Scenario: Full proof strength remains intact
r[newtypes.durable-identities.validation.verus]
- GIVEN the completed identity migration
- WHEN full-library Verus verification and proof-gap checks run
- THEN existing durable-reference properties verify and the trusted-boundary inventory does not increase.

### Requirement: Durable Identity Non-Claims
r[newtypes.durable-identities.non-claims] Identity wrappers MUST NOT be presented as cryptographic digests, collision-resistance evidence, persistence evidence, codec correctness, type-checker correctness, dependency retrieval, or migration execution.

#### Scenario: Wrapper claim stays structural
r[newtypes.durable-identities.non-claims.structural]
- GIVEN a consumer cites the typed identity evidence
- WHEN the claim is reviewed
- THEN the claim is limited to domain separation and preserved finite-model admission behavior.
