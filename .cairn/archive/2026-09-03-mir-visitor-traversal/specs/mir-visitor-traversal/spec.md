# MIR Visitor Traversal Delta

## Purpose

Defines finite MIR visitor and traversal primitives for Trellis. The requirements cover visit-location identity, visitor trace admission, coverage, traversal order, mutable shape preservation, evidence, documentation, validation, and non-claim boundaries.

## ADDED Requirements

### Requirement: MIR Visit Locations
r[mir-visitor.locations] The system MUST define finite visit-location identifiers for the MIR-core elements a visitor may inspect or mutate.

#### Scenario: Valid location is admitted
r[mir-visitor.locations.valid]
- GIVEN a visit location naming an element present in an admitted MIR-core body
- WHEN location admission is evaluated
- THEN the location is accepted.

#### Scenario: Invalid location is rejected
r[mir-visitor.locations.invalid]
- GIVEN a visit location naming a missing block, statement, terminator, local, or nested element
- WHEN location admission is evaluated
- THEN the location is rejected with deterministic location metadata.

### Requirement: Visitor Trace Admission
r[mir-visitor.trace] The system MUST model a visitor run as a finite trace of admitted visit locations with declared traversal mode and mutation metadata.

#### Scenario: Valid trace is accepted
r[mir-visitor.trace.valid]
- GIVEN a finite visitor trace whose entries are admitted locations and whose mode metadata is consistent
- WHEN trace admission is evaluated
- THEN the trace is accepted.

#### Scenario: Trace with malformed entry is rejected
r[mir-visitor.trace.malformed]
- GIVEN a visitor trace containing a malformed location or mode entry
- WHEN trace admission is evaluated
- THEN the trace is rejected before coverage claims are exported.

### Requirement: Visitor Coverage
r[mir-visitor.coverage] For exactly-once traversal modes, the admitted visitor trace MUST visit every requested MIR location exactly once and MUST NOT visit locations outside the requested set.

#### Scenario: Full coverage succeeds
r[mir-visitor.coverage.full]
- GIVEN an admitted body and a visitor trace containing every requested location once
- WHEN coverage admission is evaluated
- THEN the trace is accepted as covering the requested set.

#### Scenario: Missing visit is rejected
r[mir-visitor.coverage.missing]
- GIVEN an exactly-once visitor trace that omits a requested location
- WHEN coverage admission is evaluated
- THEN the trace is rejected with deterministic missing-visit metadata.

#### Scenario: Duplicate visit is rejected
r[mir-visitor.coverage.duplicate]
- GIVEN an exactly-once visitor trace that visits one requested location more than once
- WHEN coverage admission is evaluated
- THEN the trace is rejected with deterministic duplicate-visit metadata.

### Requirement: Traversal Order
r[mir-visitor.traversal-order] The system MUST define deterministic preorder and reverse-postorder traversal orders over admitted CFG edges and entry metadata.

#### Scenario: Preorder traversal starts at entry
r[mir-visitor.traversal-order.preorder]
- GIVEN an admitted CFG and entry block
- WHEN preorder traversal is evaluated
- THEN the first visited block is the entry and every later block is reachable under the declared traversal policy.

#### Scenario: Reverse postorder contains reachable blocks
r[mir-visitor.traversal-order.rpo]
- GIVEN an admitted CFG and entry block
- WHEN reverse-postorder traversal is evaluated
- THEN the result contains exactly the blocks reachable under the declared traversal policy in deterministic order.

### Requirement: Mutable Visitor Shape Preservation
r[mir-visitor.mut-shape] A mutable visitor trace MUST declare its permitted mutation class and MUST preserve all MIR-core shape invariants not explicitly allowed to change by that class.

#### Scenario: Shape-preserving rewrite is admitted
r[mir-visitor.mut-shape.valid]
- GIVEN a mutable visitor trace that edits only permitted fields and preserves declared body shape
- WHEN mutation-shape admission is evaluated
- THEN the trace is accepted.

#### Scenario: Invalid structural mutation is rejected
r[mir-visitor.mut-shape.invalid]
- GIVEN a mutable visitor trace that removes a terminator, creates an out-of-range local reference, or violates declared body shape
- WHEN mutation-shape admission is evaluated
- THEN the trace is rejected.

### Requirement: Visitor Admission Helpers
r[mir-visitor.admission] Runtime helpers MUST reject invalid locations, invalid traversal modes, duplicate visits, missing visits, and malformed mutation descriptors with deterministic metadata.

#### Scenario: First traversal failure is deterministic
r[mir-visitor.admission.first-failure]
- GIVEN a malformed visitor trace with multiple defects
- WHEN admission is evaluated repeatedly
- THEN the same first failure class and trace entry are reported.

### Requirement: Visitor Evidence Coverage
r[mir-visitor.evidence] The implementation MUST include positive and negative evidence for visit locations, trace admission, coverage, traversal orders, mutable shape preservation, and failure metadata.

#### Scenario: Positive fixtures cover traversal behavior
r[mir-visitor.evidence.positive.fixtures]
- GIVEN the MIR visitor traversal test suite
- WHEN positive fixtures run
- THEN immutable traversal, block-order traversal, and shape-preserving mutation examples are covered.

#### Scenario: Negative fixtures cover malformed traversal behavior
r[mir-visitor.evidence.negative.fixtures]
- GIVEN the MIR visitor traversal test suite
- WHEN negative fixtures run
- THEN invalid locations, missing visits, duplicate visits, traversal-policy mismatches, and invalid mutations are rejected deterministically.

### Requirement: Visitor Documentation Sync
r[mir-visitor.docs] The implementation MUST update Trellis requirements, source/test annotations, README/module-catalog text, and accepted Cairn spec text for public visitor/traversal helpers.

#### Scenario: Requirement IDs are traceable
r[mir-visitor.docs.traceable]
- GIVEN a public visitor/traversal helper lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in requirements, source annotations, tests or proofs, and Cairn spec documentation.

### Requirement: Visitor Validation Evidence
r[mir-visitor.final-validation] The change MUST capture focused Cargo, Verus, Tracey, and Cairn validation evidence before archive.

#### Scenario: Final validation names commands
r[mir-visitor.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names commands, outcomes, and limitations or blockers.

### Requirement: Visitor Non-Claim Boundary
r[mir-visitor.non-claims] The visitor/traversal primitive MUST NOT claim rustc visitor trait macro correctness, compiler borrow/lifetime behavior, pass semantic correctness, optimization correctness, or whole-compiler correctness.

#### Scenario: Traversal evidence is not pass correctness
r[mir-visitor.non-claims.coverage-only]
- GIVEN a consumer cites visitor coverage evidence for a pass
- WHEN the exported claim is reviewed
- THEN the claim is limited to admitted traversal coverage and shape preservation, not to semantic correctness of the pass's transformation.
