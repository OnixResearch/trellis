## Coverage Index

| Requirement ID | Future module focus |
|---|---|
| `domain-primitives.data-validation.schema-predicate` | `src/schema_*`, `src/validate_*` finite schema predicate envelopes |
| `domain-primitives.data-validation.field-presence` | required/optional field metadata and admission |
| `domain-primitives.data-validation.enum-constraint` | finite enum membership constraints |
| `domain-primitives.data-validation.range-constraint` | bounded range constraints over finite scalar envelopes |
| `domain-primitives.data-validation.cross-field` | finite cross-field predicate envelopes |
| `domain-primitives.data-validation.error-accumulation` | deterministic validation error accumulation metadata |
| `domain-primitives.data-validation.normalization-decision` | explicit normalization decision envelopes |
| `domain-primitives.data-validation.admission` | deterministic validation admission/rejection |

## Phase 1: Spec foundation

- [x] Create spec-only data-validation primitive requirements and umbrella taxonomy entry. `[covers=domain-primitives.data-validation.schema-predicate,domain-primitives.data-validation.field-presence,domain-primitives.data-validation.enum-constraint,domain-primitives.data-validation.range-constraint,domain-primitives.data-validation.cross-field,domain-primitives.data-validation.error-accumulation,domain-primitives.data-validation.normalization-decision,domain-primitives.data-validation.admission]`

## Phase 2: Future implementation

- [ ] Add docs/requirements entries and Tracey IDs for finite schema predicate envelopes.
- [ ] Implement and verify required/optional field admission primitives.
- [ ] Implement and verify finite enum membership constraints.
- [ ] Implement and verify bounded range constraint primitives.
- [ ] Implement and verify finite cross-field predicate envelopes.
- [ ] Implement and verify deterministic error accumulation metadata.
- [ ] Implement and verify normalization decision envelopes without broad repair claims.
- [ ] Implement deterministic validation admission/rejection composition.
- [ ] Add focused positive and negative tests for each validation primitive.
- [ ] Run focused Verus, Tracey uncovered/untested, and strict OpenSpec validation.
- [ ] Capture reviewable evidence and archive the change after all implementation tasks complete.
