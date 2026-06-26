# Knowledge Graph Primitives Specification

## Purpose

Defines spec-only requirements for reusable finite knowledge-graph primitives.
These primitives MUST use exact finite term metadata, RDF-like fact triple
envelopes, property-graph label/property metadata, ontology/schema constraint
envelopes, provenance metadata, entity-alignment metadata, traversal and
query-shape envelopes, bounded inference-witness metadata, embedding-index
metadata, and deterministic knowledge-graph admission/rejection. They MUST avoid
RDF/OWL/SPARQL/SHACL/Cypher/Gremlin conformance, open-world or
description-logic completeness, graph database behavior, runtime query engines,
LLM extraction quality, RAG answer quality, vector-search recall, embedding-model
correctness, storage/transaction guarantees, distributed graph behavior,
security, privacy, performance, or external knowledge-base correctness unless a
later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Knowledge Graph Term Metadata [r[domain-primitives.knowledge-graph.term]]
The system MUST provide finite knowledge-graph term metadata envelopes for entity
identifiers, relation identifiers, literal values, blank-node-like local
identifiers, labels, property keys, namespace scopes, and deterministic term
admission results.

#### Scenario: Unknown predicate term is rejected [r[domain-primitives.knowledge-graph.term.scenario.unknown-predicate-term-is-rejected]]

- GIVEN a finite term registry and a candidate fact referencing a predicate absent from that registry
- WHEN term admission is evaluated
- THEN the term primitive returns explicit unknown-predicate rejection metadata

### Requirement: RDF-Like Fact Triple Envelope [r[domain-primitives.knowledge-graph.triple]]
The system MUST provide finite subject/predicate/object fact triple envelopes
that admit only declared subject terms, predicate terms, and object terms or
literal envelopes.

#### Scenario: Fact triple requires admitted endpoints [r[domain-primitives.knowledge-graph.triple.scenario.fact-triple-requires-admitted-endpoints]]

- GIVEN finite term metadata and a candidate subject/predicate/object fact
- WHEN the subject or object is outside the admitted term set
- THEN the triple primitive rejects the fact with endpoint metadata before any RDF semantics claim is made

### Requirement: Property Graph Metadata Envelope [r[domain-primitives.knowledge-graph.property]]
The system MUST provide finite property-graph metadata envelopes for node labels,
edge labels, node properties, edge properties, property key admission, and
property value envelopes.

#### Scenario: Edge property key must be declared [r[domain-primitives.knowledge-graph.property.scenario.edge-property-key-must-be-declared]]

- GIVEN finite property metadata and a candidate edge property
- WHEN the property key is outside the declared property-key envelope
- THEN the property primitive rejects the edge property with unknown-property-key metadata

### Requirement: Ontology Constraint Envelope [r[domain-primitives.knowledge-graph.ontology]]
The system MUST provide finite ontology/schema constraint envelopes for classes,
relations, domains, ranges, subtype edges, cardinality metadata, and deterministic
constraint admission results.

#### Scenario: Relation range violation is rejected [r[domain-primitives.knowledge-graph.ontology.scenario.relation-range-violation-is-rejected]]

- GIVEN finite ontology metadata declaring a relation range class
- WHEN a fact uses an object term outside the admitted range envelope
- THEN the ontology primitive rejects the fact with range-violation metadata

### Requirement: Provenance Metadata Envelope [r[domain-primitives.knowledge-graph.provenance]]
The system MUST provide finite provenance metadata envelopes for source
identifiers, extraction method identifiers, confidence envelopes, timestamps or
version counters, derivation links, and deterministic provenance admission.

#### Scenario: Fact without required source is rejected [r[domain-primitives.knowledge-graph.provenance.scenario.fact-without-required-source-is-rejected]]

- GIVEN a graph admission policy requiring source provenance
- WHEN a candidate fact lacks admitted source metadata
- THEN the provenance primitive rejects the fact with missing-source metadata

### Requirement: Entity Alignment Envelope [r[domain-primitives.knowledge-graph.alignment]]
The system MUST provide deterministic entity-alignment envelopes for aliases,
equivalence candidates, merge witnesses, canonical-term selection, conflict
metadata, and non-merge decisions.

#### Scenario: Conflicting merge witness prevents canonicalization [r[domain-primitives.knowledge-graph.alignment.scenario.conflicting-merge-witness-prevents-canonicalization]]

- GIVEN two entity terms with incompatible declared class or key metadata
- WHEN entity alignment is evaluated
- THEN the alignment primitive rejects the merge and returns explicit conflict metadata

### Requirement: Traversal and Query Shape Envelope [r[domain-primitives.knowledge-graph.query-shape]]
The system MUST provide finite traversal and graph-pattern query-shape envelopes
for start terms, edge or predicate filters, path-length bounds, variable slots,
projection slots, and deterministic query-shape admission.

#### Scenario: Query shape with unbounded path is rejected [r[domain-primitives.knowledge-graph.query-shape.scenario.query-shape-with-unbounded-path-is-rejected]]

- GIVEN finite query-shape metadata requiring explicit traversal bounds
- WHEN a candidate graph pattern contains an unbounded path expression
- THEN the query-shape primitive rejects the pattern before any query-engine or SPARQL/Cypher claim is made

### Requirement: Bounded Inference Witness Envelope [r[domain-primitives.knowledge-graph.inference]]
The system MUST provide bounded inference-witness envelopes for declared rules,
input fact identifiers, derived fact identifiers, closure-depth bounds, and
explicit inferred-fact admission metadata.

#### Scenario: Derived fact requires admitted rule witness [r[domain-primitives.knowledge-graph.inference.scenario.derived-fact-requires-admitted-rule-witness]]

- GIVEN finite rule metadata and candidate inferred fact metadata
- WHEN the inferred fact lacks an admitted rule witness or exceeds the declared closure-depth bound
- THEN the inference primitive rejects the fact with missing-or-overbound witness metadata

### Requirement: Embedding Index Metadata Envelope [r[domain-primitives.knowledge-graph.embedding-index]]
The system MUST provide finite embedding-index metadata envelopes for embedding
identifiers, vector dimension bounds, indexed term or fact identifiers, distance
metric identifiers, version metadata, and deterministic index admission without
claiming vector-search quality.

#### Scenario: Embedding dimension mismatch is rejected [r[domain-primitives.knowledge-graph.embedding-index.scenario.embedding-dimension-mismatch-is-rejected]]

- GIVEN embedding-index metadata declaring a vector dimension
- WHEN a candidate embedding vector envelope has a different dimension
- THEN the embedding-index primitive rejects the vector with dimension-mismatch metadata

### Requirement: Knowledge Graph Admission Envelope [r[domain-primitives.knowledge-graph.admission]]
The system MUST provide deterministic knowledge-graph admission/rejection
envelopes that compose term, triple, property, ontology, provenance, alignment,
query-shape, inference-witness, and embedding-index metadata over finite
knowledge-graph data.

#### Scenario: Admission requires all finite KG checks [r[domain-primitives.knowledge-graph.admission.scenario.admission-requires-all-finite-kg-checks]]

- GIVEN finite knowledge-graph metadata and a candidate fact, entity merge, query shape, inferred fact, or embedding record
- WHEN any term, triple, property, ontology, provenance, alignment, query-shape, inference, or embedding-index check rejects
- THEN the knowledge-graph-admission primitive returns explicit rejection before any graph database, standards-conformance, reasoning-completeness, LLM, RAG, or vector-search claim is made
