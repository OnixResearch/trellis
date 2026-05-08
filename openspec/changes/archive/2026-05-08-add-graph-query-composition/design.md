## Context

The repository now has finite primitive modules for knowledge graphs, relational tables, storage indexes, parsing/format envelopes, serialization codecs, and text/string predicates. The missing layer is the exact composition contract that states when graph query answers are accepted and how those answers relate to the primitive predicates.

## Goals / Non-Goals

**Goals:**

- Model finite graph triples, triple patterns, variable bindings, and path witnesses.
- Compose graph facts with table-style joins/filters and storage-index lookup witnesses.
- Add a serialized query/result envelope shape that can be checked without trusting a parser or wire protocol.
- Keep proofs exact and local enough for Verus: length/bounds preservation, witness membership, join consistency, and lookup equivalence.

**Non-Goals:**

- SPARQL, Cypher, RDF, OWL/RDFS inference, property-graph engines, query planning, persistence, concurrency, or distributed graph execution.
- Text normalization beyond already admitted text/string primitives.
- Codec correctness beyond structural envelope composition and already bounded serialization predicates.

## Decisions

### 1. Finite witness-first semantics

**Choice:** Query answers are admitted by explicit finite witnesses: triple membership witnesses, variable-binding witnesses, path witnesses, and optional index-lookup witnesses.

**Rationale:** Witness-first semantics keeps the first implementation slice provable by scans and membership lemmas rather than requiring a complete query engine.

**Alternative:** Define a full recursive graph-query evaluator first. Rejected because it couples proof work to optimizer/evaluator choices and broadens the OpenSpec beyond a drainable slice.

**Implementation:** Add spec predicates for finite triples, patterns, bindings, paths, result rows, and lookup witnesses; add executable checkers only where the predicate can be checked by bounded scans.

### 2. Index equivalence is optional but exact

**Choice:** Index-backed lookup proofs are scoped to equivalence between an admitted index hit set and the corresponding finite scan result for the bounded predicate.

**Rationale:** This proves safety for index acceleration without requiring a physical index implementation or performance claims.

**Alternative:** Treat indexes as opaque hints. Rejected because it would not establish the core safety property: index-backed answers equal scan-backed answers.

**Implementation:** Compose storage-index predicates with triple-pattern predicates through a relation `lookup_equivalent_to_scan_spec(...)`.

### 3. Serialization stays structural

**Choice:** Serialized graph-query envelopes admit schema/version/header/payload/result-window shape only.

**Rationale:** The serialization codec primitives already separate finite envelope validity from codec/parser correctness.

**Alternative:** Claim parser or wire-protocol conformance. Rejected as out of scope for verified-logic primitives.

## Risks / Trade-offs

- **Overclaiming query semantics:** mitigate with explicit non-goals and finite-only wording.
- **Proof blow-up from nested joins:** start with bounded binary/chain joins and extract reusable binding-consistency lemmas.
- **Index semantics drift:** require scan-equivalence proof before any index-backed answer is admitted.
- **Archive clobber risk:** future archive must inspect `openspec/specs/composite-primitives/spec.md` after sync and preserve existing composite scenarios.
