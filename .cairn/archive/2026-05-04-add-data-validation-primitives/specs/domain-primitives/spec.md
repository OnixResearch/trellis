# Data Validation Primitives Specification

## Purpose

Defines implemented requirements for reusable finite data-validation primitives.
These primitives MUST use exact finite schema predicate envelopes,
required/optional field metadata, enum constraints, range constraints,
cross-field predicate envelopes, deterministic error accumulation,
normalization decisions, and deterministic validation admission/rejection
envelopes. They MUST avoid external validator, schema registry, runtime
framework, database, business-rule completeness, semantic repair, security,
privacy, performance, UI rendering, or broad external schema-language
conformance claims unless a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Schema Predicate Envelope [r[domain-primitives.data-validation.schema-predicate]]
The system MUST provide finite schema predicate envelopes that declare admitted
field identifiers, value-shape predicates, and deterministic predicate IDs over
finite candidate records.

#### Scenario: Unknown schema predicate rejected explicitly [r[domain-primitives.data-validation.schema-predicate.scenario.unknown-schema-predicate-rejected-explicitly]]

- GIVEN a finite admitted schema predicate set
- WHEN a candidate predicate ID is absent from that set
- THEN the schema-predicate primitive returns an explicit unknown-predicate decision without invoking an external validator

### Requirement: Field Presence Envelope [r[domain-primitives.data-validation.field-presence]]
The system MUST provide required/optional field-presence envelopes that check
finite candidate records against declared field metadata and expose explicit
missing, present, or duplicate-field decisions.

#### Scenario: Required field absence preserves input record [r[domain-primitives.data-validation.field-presence.scenario.required-field-absence-preserves-input-record]]

- GIVEN a finite candidate record and required field metadata
- WHEN a required field is absent
- THEN the field-presence primitive returns a missing-field decision without mutating the candidate record

### Requirement: Enum Constraint Envelope [r[domain-primitives.data-validation.enum-constraint]]
The system MUST provide finite enum-constraint envelopes that admit only values
contained in a declared finite member set and expose explicit out-of-enum
rejection decisions.

#### Scenario: Enum candidate must be declared [r[domain-primitives.data-validation.enum-constraint.scenario.enum-candidate-must-be-declared]]

- GIVEN a finite enum member set and candidate value
- WHEN the candidate value is not a member of the declared set
- THEN the enum-constraint primitive rejects the value with explicit out-of-enum metadata

### Requirement: Range Constraint Envelope [r[domain-primitives.data-validation.range-constraint]]
The system MUST provide bounded range-constraint envelopes over admitted scalar
values, inclusive/exclusive bound metadata, and deterministic out-of-range
rejection decisions.

#### Scenario: Range bounds are checked before scalar use [r[domain-primitives.data-validation.range-constraint.scenario.range-bounds-are-checked-before-scalar-use]]

- GIVEN finite scalar metadata and declared lower/upper range bounds
- WHEN the candidate scalar lies outside the admitted bound envelope
- THEN the range-constraint primitive rejects the scalar before any downstream validation composition uses it

### Requirement: Cross-Field Predicate Envelope [r[domain-primitives.data-validation.cross-field]]
The system MUST provide finite cross-field predicate envelopes that relate a
bounded set of admitted fields and return explicit satisfied, unsatisfied, or
inapplicable decisions.

#### Scenario: Cross-field predicate rejects missing dependency [r[domain-primitives.data-validation.cross-field.scenario.cross-field-predicate-rejects-missing-dependency]]

- GIVEN a cross-field predicate requiring two admitted fields
- WHEN either required field is absent from the candidate record
- THEN the cross-field primitive returns an explicit inapplicable or rejected decision rather than claiming business-rule completeness

### Requirement: Error Accumulation Envelope [r[domain-primitives.data-validation.error-accumulation]]
The system MUST provide deterministic error-accumulation envelopes that collect
finite validation error codes, field references, and stable ordering metadata.

#### Scenario: Error accumulation order is deterministic [r[domain-primitives.data-validation.error-accumulation.scenario.error-accumulation-order-is-deterministic]]

- GIVEN a finite set or sequence of validation decisions
- WHEN multiple decisions reject
- THEN the error-accumulation primitive records the resulting error metadata in a deterministic order declared by the envelope

### Requirement: Normalization Decision Envelope [r[domain-primitives.data-validation.normalization-decision]]
The system MUST provide normalization decision envelopes that record accepted,
rejected, unchanged, or transformed shape metadata for deterministic finite
normalization steps without claiming semantic repair correctness.

#### Scenario: Normalization rejection keeps original value metadata [r[domain-primitives.data-validation.normalization-decision.scenario.normalization-rejection-keeps-original-value-metadata]]

- GIVEN a finite value envelope and deterministic normalization metadata
- WHEN normalization is not admitted
- THEN the normalization-decision primitive records rejection metadata while preserving the original value envelope

### Requirement: Validation Admission Envelope [r[domain-primitives.data-validation.admission]]
The system MUST provide deterministic validation admission/rejection envelopes
that compose schema predicate, field presence, enum, range, cross-field, error
accumulation, and normalization decisions over finite candidate records.

#### Scenario: Admission requires all finite validation checks [r[domain-primitives.data-validation.admission.scenario.admission-requires-all-finite-validation-checks]]

- GIVEN finite validation metadata and a candidate record
- WHEN any schema predicate, field-presence, enum, range, cross-field, error, or normalization check rejects
- THEN the validation-admission primitive returns an explicit rejection decision before any external-validator or business-rule claim is made
