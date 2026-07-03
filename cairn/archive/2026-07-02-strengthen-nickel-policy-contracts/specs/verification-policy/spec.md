## MODIFIED Requirements

### Requirement: Nickel-Authored Verification Policy

Verification checker policy MUST be authored in Nickel when the policy is declarative configuration rather than parser/scanner logic. Nickel contracts MUST validate both field shape and policy-local reference integrity before generated JSON is accepted. Rust MUST remain the owner of source parsing, command execution, file reads, and scanner logic.

r[verification-policy.nickel-authored]

#### Scenario: Nickel policy typechecks

- GIVEN the Nickel policy source files
- WHEN the policy typecheck command runs in the Nix development environment
- THEN Nickel accepts the policy contracts and defaults

#### Scenario: Generated policy is fresh

- GIVEN the checked-in generated JSON policy artifact
- WHEN the policy export freshness check runs
- THEN the exported JSON matches the checked-in artifact byte-for-byte or with deterministic normalization

#### Scenario: Scanner logic remains in Rust

- GIVEN a checker requires source parsing or call-graph analysis
- WHEN the checker runs
- THEN Rust code performs the scanning while Nickel supplies only typed policy/configuration inputs

#### Scenario: Exemptions are bounded

- GIVEN the policy allows an exemption
- WHEN the exemption is authored
- THEN the exemption includes scope, owner, reason, and expiry or equivalent bounded-review metadata

### Requirement: Reusable Nickel Contract Library

The repository MUST provide reusable Nickel contracts for common policy data shapes, including non-empty strings, safe relative paths, supported enum values, non-empty arrays, unique identifiers, and BLAKE3 digest text. Policy-specific Nickel files SHOULD import these contracts instead of redefining equivalent local predicates.

r[verification-policy.reusable-nickel-contract-library]

#### Scenario: Shared contract is accepted

- GIVEN a valid policy imports the shared contract library
- WHEN Nickel typecheck and export run
- THEN the policy exports successfully and the generated artifact remains deterministic

#### Scenario: Unsafe relative path is rejected

- GIVEN a policy fixture contains an absolute path, parent-directory traversal, or backslash path separator in a path field governed by the shared safe-path contract
- WHEN the policy fixture is typechecked
- THEN Nickel rejects the fixture before JSON export

#### Scenario: Duplicate identifiers are rejected

- GIVEN a policy fixture declares two entries with the same ID in a field governed by a uniqueness contract
- WHEN the policy fixture is typechecked
- THEN Nickel rejects the fixture with a deterministic contract failure

### Requirement: Nickel Policy Reference Integrity

Nickel policy contracts MUST reject dangling cross-field references in policy data before export. The proof-review policy MUST validate that rule references, validation-gate references, and command-mode references point at declared entries. The Cairn policy MUST validate that replay groups, determinism surfaces, receipt schemas, and artifact dependencies reference declared entries. The verification-policy checker list MUST validate declared checker scripts, generated artifacts, severities, and exemption metadata against bounded contracts.

r[verification-policy.nickel-reference-integrity]

#### Scenario: Proof-review references are valid

- GIVEN a proof-review policy whose apply recipes reference declared rules and validation commands, and whose validation command modes reference declared loop modes
- WHEN the policy freshness check runs
- THEN Nickel accepts the policy and the generated JSON remains fresh

#### Scenario: Dangling validation gate is rejected

- GIVEN a proof-review policy fixture whose apply recipe names a validation gate that is not in `loop.validation_commands`
- WHEN Nickel typecheck runs
- THEN the fixture is rejected before export

#### Scenario: Cairn replay references are valid

- GIVEN a Cairn policy whose replay groups, determinism surfaces, and receipt schemas reference declared replay cases, groups, and receipt keys
- WHEN the Cairn policy freshness check runs
- THEN Nickel accepts the policy and the generated JSON remains fresh

#### Scenario: Dangling replay case is rejected

- GIVEN a Cairn policy fixture whose replay group references an undeclared replay case
- WHEN Nickel typecheck runs
- THEN the fixture is rejected before export

### Requirement: Nickel Policy Negative Fixtures

Every repository-owned Nickel policy checker MUST include positive and negative fixtures that demonstrate accepted policy data and representative rejected policy data. Negative fixtures MUST cover malformed shape, missing required fields, empty required arrays, unsupported enum values, unsafe paths, duplicate IDs, and at least one policy-local dangling reference.

r[verification-policy.nickel-negative-fixtures]

#### Scenario: Valid fixture passes

- GIVEN a minimal valid policy fixture for a repository-owned Nickel policy
- WHEN the policy fixture harness runs
- THEN the fixture is accepted and the expected exported form is deterministic

#### Scenario: Malformed fixture fails closed

- GIVEN a malformed policy fixture with a missing field, unsupported enum, duplicate ID, or dangling reference
- WHEN the policy fixture harness runs
- THEN the fixture fails with a diagnostic identifying the rejected policy family

#### Scenario: Policy checks are part of repo verification

- GIVEN the repo verification wrapper runs
- WHEN Nickel policy fixture and freshness checks are configured
- THEN the wrapper runs them before claiming full verification success
