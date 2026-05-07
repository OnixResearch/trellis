## Why

Knowledge-graph work often mixes reusable graph facts with semantic-web standards,
property-graph conveniences, ontology reasoning, entity resolution, provenance,
query engines, vector embeddings, and LLM/RAG pipelines. Research and standards
surface a small stable core: RDF-style triples use subject/predicate/object
statements over IRIs, blank nodes, and literals; property graphs attach labels and
properties to nodes and edges; construction pipelines need entity identity,
schema/ontology constraints, provenance, and deterministic admission before any
reasoning or retrieval claim is meaningful.

A focused knowledge-graph primitive family gives future changes exact finite
building blocks for fact triples, entity/relation identifiers, property metadata,
ontology constraint envelopes, provenance, merge/alignment witnesses, traversal
and query-shape admission, deterministic rule-closure envelopes, and embedding
index metadata without claiming RDF/OWL/SPARQL conformance, open-world semantics,
LLM extraction quality, graph database behavior, or vector-search correctness.

## What Changes

- Add a spec-only `add-knowledge-graph-primitives` child OpenSpec under the
  domain-primitives umbrella.
- Define exact finite models for knowledge-graph terms, triples, property graph
  metadata, ontology/schema constraints, provenance, entity alignment, traversal
  and query-shape envelopes, finite inference closure metadata, and deterministic
  knowledge-graph admission/rejection.
- Leave implementation, Verus proofs, Tracey coverage, and evidence as future
  tasks.

## Capabilities

### New Capabilities

- `domain-primitives.knowledge-graph.term`: finite entity, relation, literal,
  blank-node, label, and property-key metadata.
- `domain-primitives.knowledge-graph.triple`: RDF-like subject/predicate/object
  fact envelopes.
- `domain-primitives.knowledge-graph.property`: property-graph node/edge label
  and property metadata envelopes.
- `domain-primitives.knowledge-graph.ontology`: finite class, property,
  domain/range, subtype, and constraint metadata.
- `domain-primitives.knowledge-graph.provenance`: source, confidence, timestamp,
  and derivation metadata envelopes.
- `domain-primitives.knowledge-graph.alignment`: deterministic entity merge,
  alias, equivalence, and conflict metadata.
- `domain-primitives.knowledge-graph.query-shape`: finite traversal and
  graph-pattern query-shape envelopes.
- `domain-primitives.knowledge-graph.inference`: bounded rule-closure and
  inferred-fact witness metadata.
- `domain-primitives.knowledge-graph.embedding-index`: vector/embedding index
  metadata without vector-search quality claims.
- `domain-primitives.knowledge-graph.admission`: deterministic admission and
  rejection over finite knowledge-graph data.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add
  `src/knowledge_graph_*`, `src/kg_*`, `docs/requirements.md`, tests, and
  evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new dependencies.
- **Testing**: strict OpenSpec validation now; future implementation must run
  focused tests, Verus, Tracey, and final repo validation.

## Out of Scope

- implementing knowledge-graph modules now
- RDF/OWL/SPARQL conformance, open-world or description-logic completeness,
  graph database storage/query engines, transaction semantics, distributed graph
  behavior, ontology-learning quality, entity-resolution quality, LLM extraction
  quality, RAG answer quality, vector-search recall, embedding-model correctness,
  security, privacy, performance, or external knowledge-base correctness
