## Why

Data validation logic often mixes schema admission, required-field checks,
range/enum constraints, cross-field predicates, normalization decisions, and
error reporting in ad hoc runtime code. A focused data-validation primitive
family gives future changes exact finite building blocks for schema predicates,
required/optional fields, enum and range constraints, cross-field predicates,
error accumulation, normalization decisions, and deterministic validation
admission without claiming external validator behavior or business-rule
completeness.

## What Changes

- Add a spec-only `add-data-validation-primitives` child OpenSpec under the
  domain-primitives umbrella.
- Define exact finite models for schema predicate envelopes, required/optional
  field metadata, enum constraints, range constraints, cross-field predicates,
  error accumulation, normalization decisions, and deterministic validation
  admission.
- Leave implementation, Verus proofs, Tracey coverage, and evidence as future
  tasks.

## Capabilities

### New Capabilities

- `domain-primitives.data-validation.schema-predicate`: finite schema predicate
  envelopes.
- `domain-primitives.data-validation.field-presence`: required/optional field
  metadata and admission.
- `domain-primitives.data-validation.enum-constraint`: finite enum membership
  constraints.
- `domain-primitives.data-validation.range-constraint`: bounded range
  constraints over admitted scalar envelopes.
- `domain-primitives.data-validation.cross-field`: finite cross-field predicate
  envelopes.
- `domain-primitives.data-validation.error-accumulation`: deterministic error
  accumulation metadata.
- `domain-primitives.data-validation.normalization-decision`: explicit
  normalization decision envelopes.
- `domain-primitives.data-validation.admission`: deterministic validation
  admission/rejection envelope.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add
  `src/schema_*`, `src/validate_*`, `docs/requirements.md`, tests, and
  evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new dependencies.
- **Testing**: strict OpenSpec validation now; future implementation must run
  focused tests, Verus, Tracey, and final repo validation.

## Out of Scope

- implementing data-validation modules now
- external validator, schema registry, database, web framework, or form-library
  correctness
- business-rule completeness, semantic domain correctness, or policy approval
  claims
- full language/schema conformance for JSON Schema, OpenAPI, Protobuf, SQL, or
  other external specifications
- automatic repair, lossy coercion correctness, internationalization, locale
  behavior, security, privacy, performance, or UI error-rendering claims
