## Coverage Index

| Requirement ID | Future module focus |
|---|---|
| `domain-primitives.knowledge-graph.term` | `src/knowledge_graph_*`, `src/kg_*` finite terms and identifiers |
| `domain-primitives.knowledge-graph.triple` | RDF-like fact triple envelopes |
| `domain-primitives.knowledge-graph.property` | property-graph labels and property metadata |
| `domain-primitives.knowledge-graph.ontology` | finite class/property/domain/range/subtype constraints |
| `domain-primitives.knowledge-graph.provenance` | source/confidence/timestamp/derivation metadata |
| `domain-primitives.knowledge-graph.alignment` | entity merge, alias, equivalence, and conflict metadata |
| `domain-primitives.knowledge-graph.query-shape` | traversal and graph-pattern query-shape envelopes |
| `domain-primitives.knowledge-graph.inference` | bounded rule-closure and inferred-fact witnesses |
| `domain-primitives.knowledge-graph.embedding-index` | embedding/vector index metadata envelopes |
| `domain-primitives.knowledge-graph.admission` | deterministic KG admission/rejection composition |

## Phase 1: Spec foundation

- [x] Create spec-only knowledge-graph primitive requirements from standards/research decomposition. `[covers=domain-primitives.knowledge-graph.term,domain-primitives.knowledge-graph.triple,domain-primitives.knowledge-graph.property,domain-primitives.knowledge-graph.ontology,domain-primitives.knowledge-graph.provenance,domain-primitives.knowledge-graph.alignment,domain-primitives.knowledge-graph.query-shape,domain-primitives.knowledge-graph.inference,domain-primitives.knowledge-graph.embedding-index,domain-primitives.knowledge-graph.admission]`

## Phase 2: Future implementation

- [x] Add docs/requirements entries and Tracey IDs for finite knowledge-graph terms.
- [x] Implement and verify RDF-like fact triple envelopes.
- [x] Implement and verify property-graph label/property metadata envelopes.
- [x] Implement and verify finite ontology/schema constraint metadata.
- [x] Implement and verify provenance metadata envelopes.
- [x] Implement and verify deterministic entity alignment and conflict metadata.
- [x] Implement and verify traversal and graph-pattern query-shape envelopes.
- [x] Implement and verify bounded rule-closure and inferred-fact witness metadata.
- [x] Implement and verify embedding/vector index metadata admission without quality claims.
- [x] Implement deterministic knowledge-graph admission/rejection composition.
- [x] Add focused positive and negative tests for each knowledge-graph primitive.
- [x] Run focused Verus, Tracey uncovered/untested, and strict OpenSpec validation.
- [x] Capture reviewable evidence and archive the change after all implementation tasks complete.
