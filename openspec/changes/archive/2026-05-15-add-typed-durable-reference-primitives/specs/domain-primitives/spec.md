## ADDED Requirements

### Requirement: Typed Durable Reference Envelope

The system MUST define finite typed durable reference envelopes containing value identity, expected type identity, schema/version metadata, and dependency-closure identity, and MUST reject references whose declared metadata is inconsistent or outside finite model bounds.

r[domain-primitives.typed-durable-reference.envelope]

#### Scenario: Well-formed typed reference is admitted

- GIVEN a durable reference with admitted value hash, type hash, schema version, and dependency closure identity
- WHEN typed durable reference admission is evaluated
- THEN the reference is admitted

#### Scenario: Missing dependency closure is rejected

- GIVEN a durable reference whose declared dependency closure identity is absent from the finite codebase model
- WHEN typed durable reference admission is evaluated
- THEN the reference is rejected with missing-closure metadata

### Requirement: Unpersist Type Match Admission

The system MUST define unpersist admission that succeeds only when the caller's expected type identity equals the persisted reference's declared type identity and the referenced dependency closure is available, without claiming runtime byte decoding or storage retrieval correctness.

r[domain-primitives.typed-durable-reference.unpersist-type-match]

#### Scenario: Matching type identity admits unpersist

- GIVEN an admitted durable reference with type hash `T`
- WHEN a caller requests unpersist with expected type hash `T` and the dependency closure is available
- THEN unpersist admission accepts the request

#### Scenario: Mismatched type identity rejects unpersist

- GIVEN an admitted durable reference with type hash `T`
- WHEN a caller requests unpersist with a different expected type hash `U`
- THEN unpersist admission rejects the request with type-mismatch metadata

#### Scenario: Available type is not enough without closure

- GIVEN matching type identity but an unavailable dependency closure
- WHEN unpersist admission is evaluated
- THEN the request is rejected with missing-closure metadata

### Requirement: Explicit Durable Reference Migration Boundary

The system MUST define schema/type migration admission as an explicit conversion witness from an old typed durable reference identity to a new typed durable reference identity, and MUST reject implicit migration when no conversion witness is declared.

r[domain-primitives.typed-durable-reference.migration-boundary]

#### Scenario: Explicit conversion admits migration

- GIVEN an old typed reference and a declared conversion witness to a new type/schema identity
- WHEN migration admission is evaluated
- THEN the migrated reference identity is admitted according to the witness metadata

#### Scenario: Implicit type change is rejected

- GIVEN a persisted reference whose type identity differs from the caller's expected type
- WHEN no conversion witness is declared
- THEN migration admission rejects the request
