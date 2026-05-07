## Context

The active primitive families already include graph, table/relational,
identifier/namespace, data-validation, serialization, text/string, ML, and
state-machine/workflow slices. Knowledge graphs need a separate finite semantic
fact model because their useful core is neither just generic graph reachability
nor a database/query-engine claim. Current standards and research separate
RDF-style triples, property graphs, ontologies/schema constraints, construction
provenance, entity alignment, reasoning, and embeddings; this change captures
only the exact finite envelopes that are appropriate for verified primitives.

## Goals / Non-Goals

**Goals:**

- specify exact finite term, triple, and property-graph metadata envelopes
- specify finite ontology/schema constraint, provenance, alignment, traversal,
  query-shape, inference-witness, and embedding-index metadata
- specify deterministic knowledge-graph admission/rejection without
  implementation yet

**Non-Goals:**

- implementing runtime APIs in this spec-only change
- proving RDF, RDFS, OWL, SPARQL, SHACL, Cypher, Gremlin, LPG, or RDF-star
  conformance
- proving open-world semantics, description-logic completeness, graph database
  behavior, graph query execution, LLM extraction quality, RAG answer quality,
  entity-resolution quality, embedding quality, vector-search recall, or
  external knowledge-base correctness
- claiming performance, scalability, privacy, security, distributed behavior, or
  storage/transaction guarantees

## Decisions

### 1. Model knowledge as finite admitted facts

**Choice:** Knowledge-graph primitives use finite term registries, fact triples,
node/edge/property metadata, provenance records, and deterministic admission
results instead of external semantic-web or graph-database execution.

**Rationale:** Finite envelopes let Verus proofs reason about identity,
well-formed facts, and metadata composition while avoiding broad standards and
runtime system claims.

### 2. Treat RDF and property graphs as shape inputs, not standards claims

**Choice:** RDF-like triples and property-graph-style labels/properties are
modeled as compatible finite shapes. The primitive family does not claim RDF
model-theoretic semantics, OWL entailment, SPARQL evaluation, or property-graph
query-language conformance.

**Rationale:** Both models are common knowledge-graph representations, but their
standards and engines carry semantics too broad for a reusable primitive family.

### 3. Keep reasoning and embeddings witness-oriented

**Choice:** Inference primitives record bounded rule-closure envelopes and
witness metadata; embedding primitives record vector/index metadata and admission
bounds only.

**Rationale:** This keeps useful hooks for later KG construction, validation, and
retrieval systems without claiming reasoning completeness, model quality, or
nearest-neighbor correctness.

## Risks / Trade-offs

**Overlap with generic graph primitives** → This child owns semantic fact terms,
triples, properties, ontologies, provenance, and KG-specific admission; generic
graph primitives own graph topology, paths, reachability, cuts, and graph-shape
logic.

**Overlap with identifier/namespace and table/relational primitives** → This
child can reuse finite identifiers and relation-like records, but owns KG entity
alias/equivalence, predicate metadata, and fact provenance.

**Standards drift** → Requirements explicitly avoid RDF/OWL/SPARQL/SHACL/Cypher
or property-graph conformance unless a later focused standards OpenSpec owns the
exact subset and test corpus.

**AI/RAG overclaiming** → Requirements explicitly avoid LLM extraction quality,
embedding quality, vector-search recall, and answer correctness; only finite
metadata/admission envelopes are in scope.

## Validation Plan

- `openspec validate add-knowledge-graph-primitives --strict --json`
- strict validation for every active primitive-family OpenSpec
- helper verification is expected to warn about future implementation tasks
- future implementation must add `docs/requirements.md`, `r[impl]`,
  `r[depends]`, `r[verify]`, tests, Verus proof evidence, and Tracey evidence
