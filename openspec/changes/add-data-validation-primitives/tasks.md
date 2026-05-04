## Coverage Index

| Requirement ID | Implemented module |
|---|---|
| `domain-primitives.data-validation.schema-predicate` | `src/data_validation_schema_predicate.rs` |
| `domain-primitives.data-validation.field-presence` | `src/data_validation_field_presence.rs` |
| `domain-primitives.data-validation.enum-constraint` | `src/data_validation_enum_constraint.rs` |
| `domain-primitives.data-validation.range-constraint` | `src/data_validation_range_constraint.rs` |
| `domain-primitives.data-validation.cross-field` | `src/data_validation_cross_field.rs` |
| `domain-primitives.data-validation.error-accumulation` | `src/data_validation_error_accumulation.rs` |
| `domain-primitives.data-validation.normalization-decision` | `src/data_validation_normalization_decision.rs` |
| `domain-primitives.data-validation.admission` | `src/data_validation_admission.rs` |

## Phase 1: Spec foundation

- [x] Create data-validation primitive requirements and umbrella taxonomy entry. `[covers=domain-primitives.data-validation.schema-predicate,domain-primitives.data-validation.field-presence,domain-primitives.data-validation.enum-constraint,domain-primitives.data-validation.range-constraint,domain-primitives.data-validation.cross-field,domain-primitives.data-validation.error-accumulation,domain-primitives.data-validation.normalization-decision,domain-primitives.data-validation.admission]`

## Phase 2: Implementation

- [x] Add docs/requirements entries and Tracey IDs for finite schema predicate envelopes.
- [x] Implement and verify required/optional field admission primitives.
- [x] Implement and verify finite enum membership constraints.
- [x] Implement and verify bounded range constraint primitives.
- [x] Implement and verify finite cross-field predicate envelopes.
- [x] Implement and verify deterministic error accumulation metadata.
- [x] Implement and verify normalization decision envelopes without broad repair claims.
- [x] Implement deterministic validation admission/rejection composition.
- [x] Add focused positive and negative tests for each validation primitive.
- [x] Run focused Verus, Tracey uncovered/untested, and strict OpenSpec validation.
- [x] Capture reviewable evidence and archive the change after all implementation tasks complete.
