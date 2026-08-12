## ADDED Requirements

### Requirement: Nickel-Owned Lifecycle Configuration

Trellis lifecycle configuration MUST be authored from a typed Nickel source when the configuration is declarative project metadata or lifecycle guidance. Checked-in YAML or JSON lifecycle config files MAY remain as compatibility artifacts, but they MUST be generated from the Nickel source and freshness-checked before repository verification claims success.

r[verification-policy.nickel-owned-lifecycle-config]

#### Scenario: Nickel lifecycle config typechecks

- GIVEN the Nickel lifecycle config source
- WHEN the lifecycle config check runs
- THEN Nickel accepts the project metadata, domain list, context text, schema name, and rule sections under the declared contract

#### Scenario: Generated compatibility config is fresh

- GIVEN checked-in `cairn/config.yaml` and legacy compatibility config artifacts
- WHEN lifecycle config freshness runs
- THEN the generated output from Nickel matches the checked-in compatibility artifacts deterministically

#### Scenario: YAML is compatibility output

- GIVEN a tool still consumes YAML lifecycle config
- WHEN the repo updates lifecycle configuration
- THEN the source edit occurs in Nickel and the YAML artifact is regenerated rather than edited as an independent source of truth

### Requirement: Lifecycle Config Negative Fixtures

The lifecycle config checker MUST include positive and negative fixtures for the Nickel lifecycle config contract. Negative fixtures MUST reject missing project metadata, empty domain lists, empty rule lists, unsupported lifecycle schemas, malformed rule section values, and unsafe generated artifact paths.

r[verification-policy.lifecycle-config-negative-fixtures]

#### Scenario: Valid lifecycle fixture passes

- GIVEN a minimal valid lifecycle config fixture with project metadata, one or more domains, context text, a supported schema, and proposal/task rules
- WHEN the lifecycle config fixture checker runs
- THEN the fixture typechecks and exports deterministically

#### Scenario: Empty domain list is rejected

- GIVEN a lifecycle config fixture with no domains
- WHEN Nickel typecheck runs
- THEN the fixture is rejected before any generated compatibility artifact is accepted

#### Scenario: Unsupported schema is rejected

- GIVEN a lifecycle config fixture with a schema name outside the supported lifecycle schema set
- WHEN Nickel typecheck runs
- THEN the fixture is rejected before export

#### Scenario: Malformed rules are rejected

- GIVEN a lifecycle config fixture whose proposal or task rules are missing, empty, or not strings
- WHEN Nickel typecheck runs
- THEN the fixture is rejected with a deterministic contract failure
