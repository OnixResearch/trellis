## ADDED Requirements

### Requirement: Content-Addressed Definition Identity

The system MUST define finite content-addressed definition identity envelopes where a definition hash is admitted only for a declared normalized definition kind, dependency-hash sequence, and metadata boundary, while names remain separate aliases and do not affect definition identity.

r[domain-primitives.content-addressed-code.definition-identity]

#### Scenario: Name change preserves definition identity

- GIVEN the same normalized definition and dependency hashes with two different human-readable names
- WHEN definition identity admission is evaluated
- THEN both aliases point at the same admitted definition hash envelope

#### Scenario: Dependency change changes identity input

- GIVEN two otherwise equal definition envelopes with different dependency-hash sequences
- WHEN definition identity admission is evaluated
- THEN the envelopes are distinguished by dependency identity metadata

#### Scenario: Unknown definition hash is rejected

- GIVEN a name binding that points to a hash absent from the finite codebase definition set
- WHEN name-binding admission is evaluated
- THEN the binding is rejected with missing-definition metadata

### Requirement: Dependency Closure Admission

The system MUST define finite dependency-closure admission for a root definition hash, reporting whether every transitively referenced dependency hash is present in the declared codebase and producing deterministic missing-dependency metadata without claiming runtime linking or execution.

r[domain-primitives.content-addressed-code.dependency-closure]

#### Scenario: Complete closure is admitted

- GIVEN a root definition whose transitive dependency hashes are all present in the finite codebase
- WHEN dependency-closure admission is evaluated
- THEN the root is admitted as closure-complete

#### Scenario: Missing dependency is reported

- GIVEN a root definition whose transitive dependency closure references an absent hash
- WHEN dependency-closure admission is evaluated
- THEN the root is rejected with the missing dependency hash metadata

#### Scenario: Adding missing dependency can close the root

- GIVEN a rejected root whose only missing dependency hash is added to the finite codebase with its own complete closure
- WHEN dependency-closure admission is reevaluated
- THEN the root is admitted as closure-complete

### Requirement: Recursive Definition Group Canonical Envelope

The system MUST define finite recursive definition group envelopes whose members are ordered canonically by declared member identity, whose intra-group references use stable member positions, and whose group hash metadata is independent of source-file order.

r[domain-primitives.content-addressed-code.recursive-group]

#### Scenario: Source order does not affect group envelope

- GIVEN two recursive group declarations with the same members in different source order
- WHEN canonical group admission is evaluated
- THEN both declarations produce the same admitted member-order metadata

#### Scenario: Out-of-range recursive reference is rejected

- GIVEN a recursive group member referencing a member position outside the canonical group bounds
- WHEN group admission is evaluated
- THEN the group is rejected with out-of-range-recursive-reference metadata
