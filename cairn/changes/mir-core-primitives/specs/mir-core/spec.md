# MIR Core Specification

## Purpose

Defines finite Rust-shaped MIR core primitives for Trellis. The requirements cover body/block shape, return-place admission, local/place/projection validity, operand and rvalue envelopes, terminator successor ranges, deterministic failure metadata, evidence, documentation, validation, and non-claim boundaries.

## Requirements

### Requirement: MIR Body Shape
r[mir-core.body-shape] The system MUST define a finite MIR body envelope with local declarations, basic blocks, and body-level metadata sufficient to validate references without reading compiler state.

#### Scenario: Finite body is admitted
r[mir-core.body-shape.valid]
- GIVEN a finite MIR body whose locals and blocks satisfy their admission predicates
- WHEN MIR body admission is evaluated
- THEN the body is accepted as a candidate for downstream MIR-core claims.

#### Scenario: Empty block table is rejected
r[mir-core.body-shape.empty-blocks]
- GIVEN a MIR body with no basic blocks
- WHEN body admission is evaluated
- THEN the body is rejected with deterministic block-table metadata.

### Requirement: MIR Return Place
r[mir-core.return-place] The system MUST model the distinguished MIR return place with a named constant and MUST require admitted bodies to contain that return local before any statement or terminator refers to it.

#### Scenario: Return place exists
r[mir-core.return-place.exists]
- GIVEN a body whose local table contains the named return place
- WHEN local-table admission is evaluated
- THEN the return place is available for return-value statements and terminators.

#### Scenario: Missing return place is rejected
r[mir-core.return-place.missing]
- GIVEN a body whose local table cannot supply the named return place
- WHEN body admission is evaluated
- THEN admission fails before downstream analyses can inspect return-place uses.

### Requirement: MIR Block Terminator Shape
r[mir-core.block-terminator] Each admitted MIR basic block MUST contain zero or more statements followed by exactly one final terminator.

#### Scenario: Final terminator is accepted
r[mir-core.block-terminator.valid]
- GIVEN a block with admitted statements and one admitted final terminator
- WHEN block-shape admission is evaluated
- THEN the block is accepted.

#### Scenario: Missing terminator is rejected
r[mir-core.block-terminator.missing]
- GIVEN a block without a final terminator
- WHEN block-shape admission is evaluated
- THEN the block is rejected with deterministic terminator metadata.

### Requirement: MIR Local References
r[mir-core.local-reference] Every admitted local reference in a place, operand, statement, or terminator MUST name an in-range local declaration in the containing body.

#### Scenario: In-range local is accepted
r[mir-core.local-reference.valid]
- GIVEN a local reference whose index names an existing local declaration
- WHEN local-reference admission is evaluated
- THEN the reference is accepted.

#### Scenario: Out-of-range local is rejected
r[mir-core.local-reference.out-of-range]
- GIVEN a local reference outside the body local-declaration range
- WHEN local-reference admission is evaluated
- THEN the reference is rejected with deterministic local metadata.

### Requirement: MIR Place Projection Chains
r[mir-core.place-projection] Every admitted place MUST have an admitted base local and a finite projection chain whose elements are declared by the Trellis MIR core model.

#### Scenario: Projected place is accepted
r[mir-core.place-projection.valid]
- GIVEN a place with an in-range base local and finite admitted projections
- WHEN place admission is evaluated
- THEN the place is accepted and its projection order is preserved.

#### Scenario: Unsupported projection is rejected
r[mir-core.place-projection.unsupported]
- GIVEN a place containing a projection kind outside the declared finite model
- WHEN place admission is evaluated
- THEN the place is rejected with deterministic projection metadata.

### Requirement: MIR Operand and Rvalue Shape
r[mir-core.rvalue-nonnesting] Admitted operands MUST be constants, copies, or moves of admitted places, and admitted rvalues MUST reference operands and places without nesting arbitrary rvalues inside rvalues.

#### Scenario: Simple rvalue is accepted
r[mir-core.rvalue-nonnesting.valid]
- GIVEN an rvalue whose operands are admitted constants, copies, or moves
- WHEN rvalue admission is evaluated
- THEN the rvalue is accepted for local transfer reasoning.

#### Scenario: Nested rvalue is rejected
r[mir-core.rvalue-nonnesting.nested]
- GIVEN an rvalue candidate that embeds another rvalue where MIR core requires an operand or place
- WHEN rvalue admission is evaluated
- THEN the candidate is rejected with deterministic rvalue-shape metadata.

### Requirement: MIR Terminator Successors
r[mir-core.successors] Every admitted terminator successor MUST name an in-range basic block in the same body, and CFG edge extraction MUST return only those declared successor edges.

#### Scenario: Branch successors are in range
r[mir-core.successors.valid]
- GIVEN an admitted terminator whose successors are all in range
- WHEN CFG edge extraction is evaluated
- THEN the extracted edges target only valid basic blocks.

#### Scenario: Out-of-range successor is rejected
r[mir-core.successors.out-of-range]
- GIVEN a terminator whose successor names a missing block
- WHEN terminator admission is evaluated
- THEN the terminator is rejected with deterministic successor metadata.

### Requirement: MIR Core Admission Metadata
r[mir-core.failure-metadata] Runtime MIR-core helpers MUST report explicit success or deterministic failure metadata rather than panicking, indexing unchecked, or silently accepting malformed inputs.

#### Scenario: First failure is deterministic
r[mir-core.failure-metadata.first-failure]
- GIVEN a malformed body with multiple independent defects
- WHEN admission is evaluated repeatedly
- THEN the same deterministic failure class and location metadata are returned.

### Requirement: MIR Core Downstream Adapter
r[mir-core.downstream-adapter] The system MUST expose finite adapter facts for downstream Trellis analyses to consume admitted block counts, local counts, and successor edges without depending on rustc compiler data structures.

#### Scenario: Adapter preserves admitted shape
r[mir-core.downstream-adapter.shape]
- GIVEN an admitted MIR core body
- WHEN downstream adapter metadata is extracted
- THEN the metadata preserves the body block count, local count, and in-range successor relation.

### Requirement: MIR Core Evidence Coverage
r[mir-core.evidence] The implementation MUST include positive and negative evidence for every public MIR-core admission family.

#### Scenario: Positive fixtures cover accepted families
r[mir-core.evidence.positive.fixtures]
- GIVEN the MIR-core test suite
- WHEN positive fixtures run
- THEN valid body, block, local, place, operand, rvalue, statement, and terminator examples are accepted.

#### Scenario: Negative fixtures cover malformed families
r[mir-core.evidence.negative.fixtures]
- GIVEN the MIR-core test suite
- WHEN negative fixtures run
- THEN malformed body, block, local, place, operand, rvalue, statement, and terminator examples are rejected deterministically.

### Requirement: MIR Core Documentation Sync
r[mir-core.docs] The implementation MUST update Trellis requirements, source/test annotations, README/module-catalog text, and accepted Cairn spec text for any public MIR-core module that lands.

#### Scenario: Requirement IDs are traceable
r[mir-core.docs.traceable]
- GIVEN a public MIR-core helper lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in requirement text, source annotations, tests or proofs, and Cairn spec documentation.

### Requirement: MIR Core Validation Evidence
r[mir-core.final-validation] The change MUST capture focused Cargo, Verus, Tracey, and Cairn validation evidence before archive.

#### Scenario: Final validation names commands
r[mir-core.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names the commands run, outcomes, and any scoped limitations or blockers.

### Requirement: MIR Core Non-Claim Boundary
r[mir-core.non-claims] The MIR-core primitive MUST NOT claim full rustc MIR compatibility, Rust type checking, borrow checking, drop elaboration, optimization soundness, code generation correctness, parsing of rustc MIR text, or whole-compiler correctness.

#### Scenario: Shape evidence stays bounded
r[mir-core.non-claims.shape-only]
- GIVEN a consumer cites MIR-core admission evidence
- WHEN the exported claim is reviewed
- THEN the claim is limited to finite structural MIR-core shape and does not imply semantic correctness of Rust programs.
