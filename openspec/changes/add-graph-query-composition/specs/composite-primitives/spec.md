## ADDED Requirements

### Requirement: Bounded Graph Query Model [r[composite-primitives.graph-query.model]]
The system MUST define a finite graph-query model over admitted graph nodes, edge labels, triples, variable identifiers, literal atoms, query bounds, and explicit validity predicates before any query answer or witness references them.

#### Scenario: Query reference outside the model is rejected [r[composite-primitives.graph-query.model.scenario.out-of-model-reference-rejected]]

- GIVEN a query pattern or witness referencing a node, label, literal, or variable outside the finite model
- WHEN graph-query admission is evaluated
- THEN the reference is rejected with out-of-model metadata

### Requirement: Triple Pattern Match Predicate [r[composite-primitives.graph-query.triple-pattern]]
The system MUST provide a triple-pattern predicate that admits a graph triple against constants and variables only when every constant equals the corresponding triple field and every repeated variable is assigned consistently.

#### Scenario: Repeated variable must bind consistently [r[composite-primitives.graph-query.triple-pattern.scenario.repeated-variable-consistent]]

- GIVEN a pattern where the subject and object positions use the same variable
- WHEN a candidate triple is matched
- THEN the match is accepted only if the triple subject and object are equal under the admitted binding

### Requirement: Binding Extension and Compatibility [r[composite-primitives.graph-query.binding-extension]]
The system MUST define binding-extension predicates that preserve existing variable assignments, add only admitted variable/value pairs, and reject conflicting assignments during pattern matching or joins.

#### Scenario: Conflicting join assignment is rejected [r[composite-primitives.graph-query.binding-extension.scenario.conflicting-join-assignment-rejected]]

- GIVEN two partial bindings that assign different values to the same variable
- WHEN graph-query join compatibility is evaluated
- THEN the joined binding is rejected as conflicting

### Requirement: Path Witness Validity [r[composite-primitives.graph-query.path-witness]]
The system MUST admit a path witness only when every consecutive step is backed by an admitted graph triple, the path endpoints match the requested pattern, and the path length stays within the finite query bound.

#### Scenario: Broken path step is rejected [r[composite-primitives.graph-query.path-witness.scenario.broken-step-rejected]]

- GIVEN a path witness containing adjacent nodes with no admitted edge triple between them
- WHEN path-witness admission is evaluated
- THEN the witness is rejected with broken-step metadata

### Requirement: Join and Filter Composition [r[composite-primitives.graph-query.join-filter]]
The system MUST compose triple-pattern result rows with table-style joins and filters so accepted rows preserve source-pattern witnesses, binding compatibility, row bounds, and filter predicate truth.

#### Scenario: Accepted joined row preserves source witnesses [r[composite-primitives.graph-query.join-filter.scenario.joined-row-preserves-witnesses]]

- GIVEN two pattern result rows with compatible bindings and admitted source witnesses
- WHEN the graph-query join is evaluated
- THEN the accepted joined row carries bindings from both rows and preserves the source witness references

### Requirement: Index Lookup Scan Equivalence [r[composite-primitives.graph-query.index-equivalence]]
The system MUST admit an index-backed graph-query result only when the index hit set is equivalent to the finite scan result for the same triple-pattern predicate and query bounds.

#### Scenario: Missing scan hit rejects index result [r[composite-primitives.graph-query.index-equivalence.scenario.missing-scan-hit-rejects-index-result]]

- GIVEN an index hit set that omits a triple accepted by the finite scan predicate
- WHEN index-backed result admission is evaluated
- THEN the index-backed result is rejected as not scan-equivalent

### Requirement: Serialized Query Envelope [r[composite-primitives.graph-query.serialized-envelope]]
The system MUST admit serialized graph-query requests and result windows only when the serialization envelope has an admitted schema version, bounded header/payload/trailer ranges, result-window metadata, and a payload shape that references only admitted finite query-model elements.

#### Scenario: Unsupported query schema is rejected [r[composite-primitives.graph-query.serialized-envelope.scenario.unsupported-schema-rejected]]

- GIVEN a serialized graph-query envelope with an unsupported schema version or out-of-bounds payload length
- WHEN serialized envelope admission is evaluated
- THEN the envelope is rejected before any query-result witness is accepted

### Requirement: Query Result Soundness Facade [r[composite-primitives.graph-query.result-soundness]]
The system MUST expose a composite graph-query result facade whose accepted result implies finite model validity, pattern-match correctness, binding compatibility, path-witness validity where requested, filter truth, optional index-scan equivalence, and serialized envelope validity where supplied.

#### Scenario: Accepted result carries all required proofs [r[composite-primitives.graph-query.result-soundness.scenario.accepted-result-carries-proofs]]

- GIVEN a graph-query result with pattern witnesses, bindings, optional path witnesses, optional index evidence, and optional serialized envelope metadata
- WHEN the result facade accepts the result
- THEN every exported result row is backed by the corresponding finite primitive witnesses and no rejected predicate is bypassed
