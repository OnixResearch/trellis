## Coverage Index

| Requirement ID | Module focus |
|---|---|
| `domain-primitives.identifier-namespace.identifier` | `src/id_*`, `src/namespace_*` finite identifier envelopes |
| `domain-primitives.identifier-namespace.namespace` | finite namespace scope metadata |
| `domain-primitives.identifier-namespace.scoped-uniqueness` | scoped uniqueness over finite registries |
| `domain-primitives.identifier-namespace.alias-map` | finite alias maps and target admission |
| `domain-primitives.identifier-namespace.canonicalization` | explicit canonicalization decisions |
| `domain-primitives.identifier-namespace.collision` | deterministic collision metadata |
| `domain-primitives.identifier-namespace.lookup` | deterministic lookup result envelopes |
| `domain-primitives.identifier-namespace.registration` | deterministic registration admission/rejection |

## Phase 1: Spec foundation

- [x] Create spec-only identifier/namespace primitive requirements and umbrella taxonomy entry. `[covers=domain-primitives.identifier-namespace.identifier,domain-primitives.identifier-namespace.namespace,domain-primitives.identifier-namespace.scoped-uniqueness,domain-primitives.identifier-namespace.alias-map,domain-primitives.identifier-namespace.canonicalization,domain-primitives.identifier-namespace.collision,domain-primitives.identifier-namespace.lookup,domain-primitives.identifier-namespace.registration]`

## Phase 2: Future implementation

- [x] Add docs/requirements entries and Tracey IDs for finite identifier envelopes.
- [x] Implement and verify namespace-scope admission primitives.
- [x] Implement and verify scoped uniqueness predicates over finite registries.
- [x] Implement and verify finite alias-map admission and lookup helpers.
- [x] Implement and verify canonicalization decision envelopes without broad semantic-equivalence claims.
- [x] Implement and verify deterministic collision metadata.
- [x] Implement and verify deterministic lookup result envelopes.
- [x] Implement deterministic registration admission/rejection composition.
- [x] Add focused positive and negative tests for each identifier/namespace primitive.
- [x] Run focused Verus, Tracey uncovered/untested, and strict OpenSpec validation.
- [x] Capture reviewable evidence and archive the change after all implementation tasks complete.
