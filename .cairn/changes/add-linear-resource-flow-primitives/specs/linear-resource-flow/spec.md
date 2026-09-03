# Linear Resource Flow Delta

## ADDED Requirements

### Requirement: Trellis defines finite symbolic resource rows

r[trellis.linear_resource_flow.model] Trellis MUST define separate resource entries, row expressions, closed rows, substitutions, arrows, witnesses, and admission decisions.

#### Scenario: A consumer supplies a closed row

- GIVEN every entry has normalized kind, state, and occurrence identities
- WHEN Trellis evaluates the row
- THEN the result MUST use only the supplied symbolic facts
- AND it MUST perform no host effect

#### Scenario: A consumer supplies a row tail variable

- GIVEN a finite row expression contains an optional tail variable
- WHEN Trellis admits the expression
- THEN the variable MUST remain distinct from concrete resource entries

### Requirement: Closed rows require unique occurrences

r[trellis.linear_resource_flow.well_formed] A well-formed closed row MUST contain each resource occurrence at most once.

#### Scenario: Every occurrence is unique

- GIVEN a closed row has distinct occurrence identities
- WHEN well-formedness admission runs
- THEN admission MUST accept the occurrence shape

#### Scenario: One occurrence appears twice

- GIVEN a closed row repeats an occurrence identity
- WHEN well-formedness admission runs
- THEN admission MUST reject the row

### Requirement: Row unification is bounded and sound

r[trellis.linear_resource_flow.unification] Successful row unification MUST produce a substitution that makes both admitted row expressions equal.

#### Scenario: A tail variable matches a closed suffix

- GIVEN one row expression ends in a variable
- AND the other row supplies a finite compatible suffix
- WHEN unification runs within its declared bound
- THEN the returned substitution MUST make both rows equal

#### Scenario: A variable contains itself

- GIVEN a proposed substitution places a row variable inside its own expansion
- WHEN the occurs check runs
- THEN unification MUST reject the substitution

#### Scenario: The solver exhausts its bound

- GIVEN row unification cannot finish within the declared bound
- WHEN the solver reaches exhaustion
- THEN it MUST return an explicit bounded failure
- AND it MUST NOT claim incompatibility as a proved result

### Requirement: Sequential arrows compose through row unification

r[trellis.linear_resource_flow.sequence] Sequential composition MUST require the first output row and second input row to unify.

#### Scenario: Adjacent rows unify

- GIVEN the first arrow output unifies with the second arrow input
- WHEN composition runs
- THEN the result MUST apply the substitution to the outer input and output rows

#### Scenario: Adjacent rows disagree

- GIVEN adjacent rows contain incompatible kind, state, occurrence, or order facts
- WHEN composition runs
- THEN composition MUST reject

### Requirement: Graph operations preserve linear occurrences

r[trellis.linear_resource_flow.graph] Fork, conditional join, and terminal decisions MUST preserve every linear occurrence unless an explicit export or disposal applies.

#### Scenario: A fork uses a complete disjoint partition

- GIVEN branch rows are pairwise disjoint
- AND their combined occurrences equal the input occurrences
- WHEN fork admission runs
- THEN admission MUST accept the partition

#### Scenario: A fork duplicates one occurrence

- GIVEN two branch rows contain the same occurrence
- WHEN fork admission runs
- THEN admission MUST reject the partition

#### Scenario: A fork loses one occurrence

- GIVEN the branch rows omit an input occurrence
- WHEN fork admission runs
- THEN admission MUST reject the partition

#### Scenario: Conditional branches agree

- GIVEN every possible branch returns the same well-formed closed row
- WHEN conditional join admission runs
- THEN admission MUST accept the join

#### Scenario: Conditional branches disagree

- GIVEN possible branches return different closed rows
- WHEN conditional join admission runs
- THEN admission MUST reject the join

#### Scenario: A terminal row is discharged

- GIVEN every terminal occurrence has an exact export or disposal fact
- WHEN terminal admission runs
- THEN admission MUST accept the terminal row

#### Scenario: A terminal row loses an occurrence

- GIVEN a terminal occurrence has no export or disposal fact
- WHEN terminal admission runs
- THEN admission MUST reject the row

### Requirement: Non-linear operations require exact witnesses

r[trellis.linear_resource_flow.witness] Duplication and disposal MUST require an admitted witness for the exact resource kind and operation identity.

#### Scenario: An exact duplication witness exists

- GIVEN a duplication witness matches the resource kind and operation
- WHEN duplication admission runs
- THEN the decision MUST permit the declared duplicate outputs

#### Scenario: A duplication witness is absent or stale

- GIVEN no exact duplication witness matches the resource kind and operation
- WHEN duplication admission runs
- THEN the decision MUST reject duplication

#### Scenario: An exact disposal witness exists

- GIVEN a disposal witness matches the resource kind and operation
- WHEN disposal admission runs
- THEN the decision MUST permit the declared disposal

#### Scenario: A disposal witness is absent or stale

- GIVEN no exact disposal witness matches the resource kind and operation
- WHEN disposal admission runs
- THEN the decision MUST reject disposal

### Requirement: Rejection preserves modeled inputs

r[trellis.linear_resource_flow.rejection] Every rejected resource-flow decision MUST preserve all supplied modeled rows and MUST return no admitted transition.

#### Scenario: Any public admission rejects

- GIVEN a public resource-flow decision rejects its input
- WHEN the caller inspects the result
- THEN every returned modeled input MUST equal the supplied value
- AND no transition MUST be admitted

### Requirement: The module remains a pure proof component

r[trellis.linear_resource_flow.boundary] The module MUST remain deterministic and `no_std + alloc` compatible. It MUST NOT perform I/O, hashing, serialization, authorization, or execution.

#### Scenario: A direct unit test calls the module

- GIVEN ordinary in-memory model values
- WHEN a test calls any public decision
- THEN the decision MUST need no filesystem, network, process, clock, credential, or runtime service

#### Scenario: An implementation adds ambient state

- GIVEN the module reads an environment value, clock, file, network, process, or mutable global
- WHEN the architecture guard runs
- THEN the guard MUST reject the implementation

### Requirement: Consumers bind an immutable proof profile

r[trellis.linear_resource_flow.consumer] Trellis MUST publish the exact model, bounds, functions, proofs, assumptions, source revision, and non-claims for consumers.

#### Scenario: Choregraph adopts the profile

- GIVEN Trellis has published an immutable resource-flow profile
- WHEN Choregraph maps graph artifacts into the model
- THEN Choregraph MUST bind the exact Trellis revision and profile identity
- AND it MUST run local positive and negative projection evidence

#### Scenario: A consumer uses a stale profile

- GIVEN a consumer projection names a different model, bound, proof, or revision
- WHEN consumer-profile admission runs
- THEN admission MUST reject the projection

### Requirement: Mirth remains an external reference

r[trellis.linear_resource_flow.reference] Trellis MUST record the reviewed Mirth revision and adapted concepts. It MUST NOT treat Mirth execution as proof evidence.

#### Scenario: Documentation cites the resource design

- GIVEN Trellis documentation cites the Mirth resource model
- WHEN provenance review runs
- THEN it MUST bind revision `b180112a547cb803e3bf5720a0bb08f8bffa9742`
- AND it MUST state that no source or runtime was imported

#### Scenario: Evidence includes a successful Mirth build

- GIVEN the reviewed Mirth build and golden suite passed
- WHEN Trellis evidence classifies that result
- THEN it MUST classify the result as external reference evidence only

### Requirement: Evidence covers acceptance and rejection

r[trellis.linear_resource_flow.evidence] The change MUST include positive and negative runtime tests, proof functions, Tracey links, and exact verifier evidence.

#### Scenario: The proof portfolio is complete

- GIVEN every public decision has a matching specification and stated proofs
- WHEN the full verification rail runs
- THEN the evidence MUST record the exact source, verifier, assumptions, and results

#### Scenario: Only positive examples exist

- GIVEN the change lacks required mismatch, duplication, loss, occurs-check, witness, or rejection-preservation cases
- WHEN readiness review runs
- THEN readiness MUST remain blocked
