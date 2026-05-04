# Configuration Feature Flag Primitives Specification

## Purpose

Defines implemented requirements for reusable finite configuration/feature-flag
primitives. These primitives MUST use exact finite config keys, typed value
envelopes, default metadata, override precedence, feature-gate predicates,
rollout bucket metadata, validation decisions, and deterministic config
admission/rejection envelopes. They MUST avoid runtime config-store, database,
filesystem, environment-variable, secret-manager, distributed rollout,
consistency, availability, randomization-quality, A/B-test validity, business
policy, security, hot-reload, caching, performance, or external flag-service
claims unless a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Config Key Envelope [r[domain-primitives.configuration-feature-flag.key]]
The system MUST provide finite config-key envelopes that record key identifiers,
optional namespaces, and deterministic known/unknown-key admission metadata.

#### Scenario: Unknown key rejected explicitly [r[domain-primitives.configuration-feature-flag.key.scenario.unknown-key-rejected-explicitly]]

- GIVEN a finite admitted config-key set
- WHEN a candidate key is not in the admitted set
- THEN the config-key primitive returns an explicit unknown-key decision without consulting an external store

### Requirement: Typed Value Envelope [r[domain-primitives.configuration-feature-flag.typed-value]]
The system MUST provide finite typed-value envelopes that bind declared type tags
to admitted value shapes and reject type/value mismatches deterministically.

#### Scenario: Type mismatch rejected before use [r[domain-primitives.configuration-feature-flag.typed-value.scenario.type-mismatch-rejected-before-use]]

- GIVEN a declared config value type and candidate finite value envelope
- WHEN the candidate value shape does not match the declared type tag
- THEN the typed-value primitive rejects the value or exposes an explicit type-mismatch decision

### Requirement: Default Value Metadata [r[domain-primitives.configuration-feature-flag.default]]
The system MUST provide default-value metadata that ties each default to a
declared finite key envelope and compatible typed-value envelope.

#### Scenario: Default must match declared key and type [r[domain-primitives.configuration-feature-flag.default.scenario.default-must-match-declared-key-and-type]]

- GIVEN default metadata for a config key
- WHEN the key is unknown or the default value does not match the declared type
- THEN the default primitive rejects the metadata without mutating the candidate config set

### Requirement: Override Precedence Envelope [r[domain-primitives.configuration-feature-flag.override-precedence]]
The system MUST provide deterministic finite override-precedence envelopes for
admitted configuration sources and explicit tie/duplicate-source decisions.

#### Scenario: Highest admitted precedence selected deterministically [r[domain-primitives.configuration-feature-flag.override-precedence.scenario.highest-admitted-precedence-selected-deterministically]]

- GIVEN finite override candidates for the same admitted key
- WHEN the precedence envelope orders the candidate sources without ties
- THEN the override-precedence primitive selects the highest-precedence admitted candidate deterministically

### Requirement: Feature Gate Predicate Envelope [r[domain-primitives.configuration-feature-flag.gate]]
The system MUST provide finite feature-gate predicate envelopes that map admitted
keys, subjects, and typed values to explicit enabled/disabled/rejected decisions.

#### Scenario: Gate rejects unknown subject metadata [r[domain-primitives.configuration-feature-flag.gate.scenario.gate-rejects-unknown-subject-metadata]]

- GIVEN a feature-gate predicate over finite admitted subject metadata
- WHEN a candidate subject lacks the required admitted metadata
- THEN the feature-gate primitive returns an explicit rejection decision rather than claiming runtime rollout behavior

### Requirement: Rollout Bucket Envelope [r[domain-primitives.configuration-feature-flag.rollout-bucket]]
The system MUST provide deterministic rollout-bucket envelopes over finite
subject identifiers, bucket ranges, and declared membership predicates without
claiming randomness quality or distributed rollout consistency.

#### Scenario: Bucket range must be admitted [r[domain-primitives.configuration-feature-flag.rollout-bucket.scenario.bucket-range-must-be-admitted]]

- GIVEN finite rollout bucket metadata and a candidate subject bucket
- WHEN the bucket lies outside the admitted bucket range
- THEN the rollout-bucket primitive rejects the candidate or exposes an explicit out-of-range decision

### Requirement: Validation Decision Metadata [r[domain-primitives.configuration-feature-flag.validation-decision]]
The system MUST provide validation decision metadata that records accepted,
rejected, and reason-coded invalid configuration entries over finite admitted
keys and typed values.

#### Scenario: Invalid entry preserves decision reason [r[domain-primitives.configuration-feature-flag.validation-decision.scenario.invalid-entry-preserves-decision-reason]]

- GIVEN a finite configuration entry with unknown key, mismatched type, or invalid precedence metadata
- WHEN validation is performed by the primitive
- THEN the validation decision records the rejection reason without mutating the input entry

### Requirement: Config Admission Envelope [r[domain-primitives.configuration-feature-flag.admission]]
The system MUST provide deterministic configuration admission/rejection envelopes
that compose key, typed-value, default, override, gate, rollout, and validation
metadata over finite inputs.

#### Scenario: Admission requires all finite checks [r[domain-primitives.configuration-feature-flag.admission.scenario.admission-requires-all-finite-checks]]

- GIVEN finite configuration and feature-flag metadata
- WHEN any required key, type, default, override, gate, rollout, or validation check rejects
- THEN the config-admission primitive returns an explicit rejection decision before any runtime store or distributed flag-service claim is made
