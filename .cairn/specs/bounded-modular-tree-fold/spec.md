# Bounded Modular Tree Fold Specification

## Purpose

Defines the `bounded-modular-tree-fold` capability.

## Requirements

### Requirement: External projects remain outside the proof boundary

r[bounded-modular-tree-fold.boundary]
Trellis MUST treat Bend and HVM2 as pinned external design and conformance references only. Trellis MUST NOT import their source into the verified core or claim that a Trellis-local model proves their syntax, typing, frontend, code generation, interaction system, scheduler, runtime, or generated artifacts correct.

#### Scenario: Pinned external references are recorded

- GIVEN documentation names Bend or HVM2 as motivation
- WHEN reference metadata is validated
- THEN it MUST include the exact reviewed revision, version, license, bounded role, and non-claim.

#### Scenario: External behavior is promoted to a proved fact

- GIVEN evidence states that a local Trellis theorem proves Bend or HVM2 behavior
- WHEN claim validation runs
- THEN the evidence MUST be rejected as outside the proof boundary.

### Requirement: The `u24` domain is explicit and closed

r[bounded-modular-tree-fold.u24-domain]
The verified module MUST define named constants for the `u24` width, modulus, maximum, mask, and conversion bounds. It MUST define validity and normalization specifications, and executable normalization MUST return a valid value equal to the mathematical input modulo the declared modulus.

#### Scenario: Arbitrary input is normalized

- GIVEN any representable runtime input
- WHEN `u24` normalization runs
- THEN the result MUST be in the declared domain and equal the mathematical modulo result.

#### Scenario: Valid input is normalized again

- GIVEN an input already in the declared domain
- WHEN normalization runs one or more times
- THEN the value MUST remain unchanged.

### Requirement: `u24` wrapping addition refines modulo addition

r[bounded-modular-tree-fold.u24-add]
Executable `u24` addition MUST accept valid operands, MUST return a valid result, and MUST equal mathematical addition modulo the named `u24` modulus. The implementation MUST avoid unproved runtime overflow and MUST NOT use ambient state.

#### Scenario: Addition stays below the modulus

- GIVEN two valid operands whose mathematical sum is in the domain
- WHEN wrapping addition runs
- THEN the result MUST equal their ordinary sum.

#### Scenario: Addition crosses the modulus

- GIVEN two valid operands whose mathematical sum reaches or exceeds the modulus
- WHEN wrapping addition runs
- THEN the result MUST equal the mathematical sum modulo the modulus.

### Requirement: Concrete `u24` addition satisfies the required algebra

r[bounded-modular-tree-fold.algebra]
Trellis MUST provide verified proof functions for zero identity, commutativity, and associativity of concrete `u24` wrapping addition. Generic algebraic predicates MAY be used only after the concrete operation is proved to satisfy them.

#### Scenario: Reduction grouping changes

- GIVEN three valid `u24` values
- WHEN the left-associated and right-associated wrapping sums are compared
- THEN the results MUST be equal.

#### Scenario: Operand order changes

- GIVEN two valid `u24` values
- WHEN their wrapping sums are evaluated in both operand orders
- THEN the results MUST be equal.

### Requirement: Balanced reduction input is admitted explicitly

r[bounded-modular-tree-fold.admission]
The versioned reduction profile MUST accept only a non-empty ordered leaf vector whose length is a power of two, whose length does not exceed a nonzero caller-provided limit, whose leaves are valid `u24` values, and whose accounting values are representable. Every rejected input MUST return a typed deterministic reason without mutating the input.

#### Scenario: Valid balanced input is admitted

- GIVEN a non-empty power-of-two vector of valid leaves within the declared limit
- WHEN profile admission runs
- THEN the input MUST be admitted with its exact profile identity and bounds.

#### Scenario: Input violates the profile

- GIVEN empty input, a zero limit, an over-limit vector, a non-power-of-two length, an invalid leaf, or unrepresentable accounting
- WHEN profile admission runs
- THEN the input MUST be rejected with the matching typed reason before reduction.

### Requirement: Both executable reducers refine one ordered sum

r[bounded-modular-tree-fold.refinement]
Trellis MUST provide an optimized iterative sequential oracle and an adjacent-pair balanced reference reducer. For admitted input, each public executable path MUST ensure equality to one ordered mathematical `u24` modular-sum specification.

#### Scenario: Sequential oracle evaluates admitted leaves

- GIVEN admitted leaves
- WHEN the optimized sequential oracle runs
- THEN its result MUST equal the declared ordered modular-sum specification.

#### Scenario: Balanced reference evaluates admitted leaves

- GIVEN the same admitted leaves
- WHEN repeated adjacent-pair reduction runs
- THEN its final result MUST equal the declared balanced-reduction specification.

### Requirement: Balanced reduction equals sequential reduction

r[bounded-modular-tree-fold.equivalence]
Trellis MUST prove that one adjacent-pair round preserves the ordered modular sum and that repeated admitted balanced reduction returns the same value as the optimized sequential oracle.

#### Scenario: One balanced round completes

- GIVEN an admitted even-length intermediate vector
- WHEN adjacent pairs are combined in order
- THEN the round output MUST have half the length and the same ordered modular sum.

#### Scenario: All balanced rounds complete

- GIVEN an admitted power-of-two leaf vector
- WHEN balanced reduction reaches one value
- THEN that value MUST equal the optimized sequential oracle result.

### Requirement: Abstract reduction work is exact and bounded

r[bounded-modular-tree-fold.work]
For an admitted vector with `n` leaves, Trellis MUST prove that the balanced schedule performs exactly `n - 1` abstract combines, uses the exact number of halving rounds, decreases intermediate vector length on each nonterminal round, and terminates at one value.

#### Scenario: Singleton input is evaluated

- GIVEN one admitted leaf
- WHEN work accounting runs
- THEN combine and round counts MUST be zero and the result MUST equal that leaf.

#### Scenario: Multi-round input is evaluated

- GIVEN an admitted vector with more than one leaf
- WHEN work accounting runs
- THEN every round MUST shrink the vector and total combines MUST equal the leaf count minus one.

### Requirement: Proof evidence binds the downstream oracle seam

r[bounded-modular-tree-fold.evidence]
The proof-evidence bundle MUST bind the algorithm profile, executable oracle symbol, source files, requirements, proof functions, verifier and toolchain, proof-gap references, BLAKE3 content identities, result status, claims, and non-claims. A downstream verified role MUST require exact artifact pinning and a downstream validation or rerun receipt.

#### Scenario: Exact proof artifact is consumed

- GIVEN a consumer pins the complete passing artifact and records its own validation or rerun receipt
- WHEN the proof role is evaluated
- THEN the consumer MAY cite the Trellis-local oracle refinement and algebraic laws.

#### Scenario: Proof handoff is stale or incomplete

- GIVEN the artifact identity, source, verifier, algorithm profile, executable symbol, non-claim, or downstream receipt is missing or different
- WHEN the proof role is evaluated
- THEN verified status MUST be denied.

### Requirement: Positive and negative fixtures cover the boundary

r[bounded-modular-tree-fold.fixtures]
The change MUST include positive unit and property tests for normalization, wrapping, algebraic laws, singleton reduction, multi-round reduction, sequential and balanced equality, and exact work accounting. It MUST include negative tests for every admission and evidence failure.

#### Scenario: Generated admitted inputs are checked

- GIVEN generated admitted vectors across declared boundary values and sizes
- WHEN property tests run
- THEN the optimized oracle, balanced reference, and independent test-only mathematical oracle MUST agree.

#### Scenario: Invalid fixture reaches the wrong boundary

- GIVEN a negative fixture declares one expected admission or evidence failure
- WHEN the fixture suite runs
- THEN the suite MUST fail if the fixture passes or produces a different rejection reason.

### Requirement: Claims remain local and bounded

r[bounded-modular-tree-fold.non-claims]
Documentation and evidence MUST state that the proofs do not establish Bend or HVM2 correctness, source effect absence, compiler correctness, generated-C equivalence, host-compiler correctness, arbitrary-tree equivalence, benchmark performance, resource behavior, HVM interaction counts, release eligibility, or downstream input-decoding correctness.

#### Scenario: Consumer reports matching external output

- GIVEN Bend or generated C returns the same value as the verified oracle for tested input
- WHEN evidence is rendered
- THEN the result MUST be classified as differential conformance evidence for that run, not external semantic proof.

#### Scenario: Abstract combines are labeled as HVM interactions

- GIVEN a report maps the proved abstract combine count directly to HVM interactions
- WHEN claim validation runs
- THEN the report MUST be rejected as an unsupported metric transfer.

### Requirement: Documentation is traceable

r[bounded-modular-tree-fold.docs]
The change MUST add canonical requirements, public module documentation, exports, implementation and verification links, README and module-catalog entries, examples, proof-evidence metadata, and explicit non-claims for every public surface.

#### Scenario: Coverage is queried

- GIVEN the accepted bounded modular tree-fold requirements
- WHEN Tracey coverage runs
- THEN every requirement MUST have implementation and verification evidence or an exact bounded rationale.

#### Scenario: A public helper lacks a contract

- GIVEN a new public executable helper lacks a matching requirement, specification, ensures refinement, test, or proof link
- WHEN repository validation runs
- THEN validation MUST fail.

### Requirement: Final validation requires proof and runtime evidence

r[bounded-modular-tree-fold.validation]
The change MUST run focused positive and negative Cargo tests, property tests, full-library Verus verification, Tracey uncovered and untested checks, proof-evidence validation, proof-gap inventory, module and README drift checks, Clippy, Octet policy, Cairn validation and gates, and relevant Nix checks before synchronization and archive.

#### Scenario: All required rails pass

- GIVEN the implementation, proofs, tests, documentation, and evidence are complete
- WHEN final validation runs
- THEN the change MAY synchronize and archive with exact command and result evidence.

#### Scenario: A promised theorem or rail is absent

- GIVEN one promised proof function, public refinement, negative test, traceability link, evidence check, or validation rail is absent or failing
- WHEN closeout runs
- THEN the change MUST remain active.
