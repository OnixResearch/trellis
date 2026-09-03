# Rustc Monomorphization Delta

## Purpose

Defines finite monomorphization and codegen-unit partitioning primitives for Trellis. The requirements cover mono-item identity, concrete instance metadata, root reachability collection, deduplication, stable/volatile classification, CGU partition disjointness/completeness, evidence, documentation, validation, and non-claim boundaries.

## ADDED Requirements

### Requirement: Mono Item Envelope
r[mono.item-envelope] The system MUST define finite mono-item identities for function instances, statics, and shims with declared concrete instance metadata.

#### Scenario: Concrete function instance is admitted
r[mono.item-envelope.function]
- GIVEN a function mono item with admitted concrete instance metadata
- WHEN mono-item admission is evaluated
- THEN the item is accepted for collection.

#### Scenario: Malformed instance metadata is rejected
r[mono.item-envelope.malformed]
- GIVEN a mono item whose declared kind and instance metadata are inconsistent
- WHEN mono-item admission is evaluated
- THEN the item is rejected with deterministic instance metadata.

### Requirement: Mono Collection Closure
r[mono.collection] The system MUST define collection as the finite reachability closure from declared root items through admitted item-reference edges.

#### Scenario: Reachable item is collected
r[mono.collection.reachable]
- GIVEN a mono item reachable from a root by admitted reference edges
- WHEN collection admission is evaluated
- THEN the item appears in the collected mono-item set.

#### Scenario: Missing reachable item is rejected
r[mono.collection.missing]
- GIVEN a collection output that omits an admitted reachable item
- WHEN collection admission is evaluated
- THEN the output is rejected with deterministic missing-item metadata.

#### Scenario: Dangling reference edge is rejected
r[mono.collection.dangling]
- GIVEN a reference edge whose target mono item is absent from the finite item universe
- WHEN collection admission is evaluated
- THEN the input graph is rejected.

### Requirement: Mono Deduplication
r[mono.dedup] The system MUST canonicalize reachable mono-item occurrences so each canonical mono-item identity appears at most once in the collected set.

#### Scenario: Duplicate occurrence collapses
r[mono.dedup.collapse]
- GIVEN two reachability paths to the same canonical mono-item identity
- WHEN collection is evaluated
- THEN the collected set contains one item for that identity.

#### Scenario: Duplicate output is rejected
r[mono.dedup.duplicate-output]
- GIVEN a collection output that lists the same canonical mono-item identity more than once
- WHEN collection admission is evaluated
- THEN the output is rejected with deterministic duplicate metadata.

### Requirement: CGU Classification
r[mono.cgu-classification] The system MUST classify collected mono items into declared stable or volatile codegen-unit classes according to finite item-kind and generic-instance metadata.

#### Scenario: Non-generic local item is stable
r[mono.cgu-classification.stable]
- GIVEN a collected item declared non-generic and local to its source module
- WHEN CGU classification is evaluated
- THEN the item is assigned to the declared stable class.

#### Scenario: Generic instance is volatile
r[mono.cgu-classification.volatile]
- GIVEN a collected generic or specialized instance
- WHEN CGU classification is evaluated
- THEN the item is assigned to the declared volatile class.

### Requirement: CGU Partition
r[mono.cgu-partition] The system MUST admit only codegen-unit partitions that are disjoint and complete over the collected mono-item set.

#### Scenario: Complete disjoint partition is accepted
r[mono.cgu-partition.valid]
- GIVEN a set of CGUs whose item sets are pairwise disjoint and whose union equals the collected mono-item set
- WHEN partition admission is evaluated
- THEN the partition is accepted.

#### Scenario: Overlapping partition is rejected
r[mono.cgu-partition.overlap]
- GIVEN two CGUs that contain the same canonical mono-item identity
- WHEN partition admission is evaluated
- THEN the partition is rejected with deterministic overlap metadata.

#### Scenario: Partition omission is rejected
r[mono.cgu-partition.omission]
- GIVEN a collected mono item absent from every CGU
- WHEN partition admission is evaluated
- THEN the partition is rejected with deterministic omission metadata.

### Requirement: Mono Admission Helpers
r[mono.admission] Runtime helpers MUST reject malformed mono-item graphs, invalid collection outputs, and invalid partitions with deterministic metadata rather than panicking or silently accepting unsupported inputs.

#### Scenario: First mono failure is deterministic
r[mono.admission.first-failure]
- GIVEN malformed mono metadata with multiple defects
- WHEN admission is evaluated repeatedly
- THEN the same first failure class and item or partition location are reported.

### Requirement: Mono Evidence Coverage
r[mono.evidence] The implementation MUST include positive and negative evidence for mono-item admission, collection closure, deduplication, classification, partitioning, and failure metadata.

#### Scenario: Positive fixtures cover collection behavior
r[mono.evidence.positive.fixtures]
- GIVEN the monomorphization test suite
- WHEN positive fixtures run
- THEN reachable concrete functions, statics, shims, duplicate collapse, and valid partitions are covered.

#### Scenario: Negative fixtures cover malformed collection behavior
r[mono.evidence.negative.fixtures]
- GIVEN the monomorphization test suite
- WHEN negative fixtures run
- THEN missing items, dangling references, malformed instances, duplicate output, overlapping partitions, and omissions are rejected deterministically.

### Requirement: Mono Documentation Sync
r[mono.docs] The implementation MUST update Trellis requirements, source/test annotations, README/module-catalog text, and accepted Cairn spec text for public monomorphization helpers.

#### Scenario: Requirement IDs are traceable
r[mono.docs.traceable]
- GIVEN a public monomorphization helper lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in requirements, source annotations, tests or proofs, and Cairn spec documentation.

### Requirement: Mono Validation Evidence
r[mono.final-validation] The change MUST capture focused Cargo, Verus, Tracey, and Cairn validation evidence before archive.

#### Scenario: Final validation names commands
r[mono.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names commands, outcomes, and limitations or blockers.

### Requirement: Mono Non-Claim Boundary
r[mono.non-claims] The monomorphization primitive MUST NOT claim trait solving, full generic substitution, rustc instance resolution, linker behavior, LTO behavior, machine-code correctness, binary-size optimization, compile-time performance, or whole-backend correctness.

#### Scenario: Collection evidence is not codegen evidence
r[mono.non-claims.no-codegen]
- GIVEN a consumer cites mono collection evidence
- WHEN the exported claim is reviewed
- THEN the claim is limited to finite collection and partition metadata, not generated object code.
