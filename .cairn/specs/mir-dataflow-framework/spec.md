# Mir Dataflow Framework Specification

## Purpose

Defines the `mir-dataflow-framework` capability.

## Requirements

### Requirement: Dataflow Domain Envelope
r[mir-dataflow.domain] The system MUST define finite dataflow domains with declared element states, row widths, bottom state, top state, and finite-height metadata.

#### Scenario: Finite domain is admitted
r[mir-dataflow.domain.valid]
- GIVEN a finite domain descriptor with declared bottom, top, join, and height metadata
- WHEN domain admission is evaluated
- THEN the domain is accepted for analysis admission.

#### Scenario: Missing bound metadata is rejected
r[mir-dataflow.domain.missing-bound]
- GIVEN a domain descriptor without finite-height metadata
- WHEN domain admission is evaluated
- THEN the domain is rejected before fixed-point iteration is attempted.

### Requirement: Join-Semilattice Laws
r[mir-dataflow.lattice-laws] The system MUST admit only domains whose finite join operation satisfies idempotence, commutativity, associativity, bottom identity, and top absorption over the declared state space.

#### Scenario: Lawful join is accepted
r[mir-dataflow.lattice-laws.valid]
- GIVEN a finite join table satisfying every declared lattice law
- WHEN lattice-law admission is evaluated
- THEN the join table is accepted.

#### Scenario: Non-idempotent join is rejected
r[mir-dataflow.lattice-laws.non-idempotent]
- GIVEN a join table where joining a state with itself can change that state
- WHEN lattice-law admission is evaluated
- THEN the domain is rejected with deterministic lattice metadata.

### Requirement: Analysis Direction
r[mir-dataflow.direction] The system MUST distinguish forward and backward analyses and MUST use the declared direction when composing location effects into block transfer functions.

#### Scenario: Forward analysis follows block order
r[mir-dataflow.direction.forward]
- GIVEN a forward analysis over an admitted block
- WHEN block transfer is evaluated
- THEN effects are composed in forward MIR location order.

#### Scenario: Backward analysis follows reverse location order
r[mir-dataflow.direction.backward]
- GIVEN a backward analysis over an admitted block
- WHEN block transfer is evaluated
- THEN effects are composed in reverse MIR location order while preserving before/primary ordering at each location.

### Requirement: Before and Primary Effects
r[mir-dataflow.effects] Every admitted statement or terminator effect MUST define deterministic before and primary effects, and the before effect MUST be applied immediately before the primary effect regardless of analysis direction.

#### Scenario: Before effect precedes primary effect
r[mir-dataflow.effects.before-primary]
- GIVEN an admitted location with both effects
- WHEN the location transfer is evaluated
- THEN the before effect is applied before the primary effect.

#### Scenario: Malformed effect is rejected
r[mir-dataflow.effects.malformed]
- GIVEN an effect that produces a state outside the finite domain
- WHEN effect admission is evaluated
- THEN the analysis is rejected with deterministic effect metadata.

### Requirement: Per-Edge Effects
r[mir-dataflow.per-edge] The system MUST model terminator per-edge effects as deterministic transfers associated with in-range successor edges only.

#### Scenario: Edge-local transfer is applied
r[mir-dataflow.per-edge.valid]
- GIVEN an admitted terminator edge with a declared per-edge effect
- WHEN successor input state is computed
- THEN the edge-local effect is applied only to that successor edge.

#### Scenario: Invalid edge effect is rejected
r[mir-dataflow.per-edge.invalid-edge]
- GIVEN a per-edge effect naming a successor not present in the terminator successor set
- WHEN analysis admission is evaluated
- THEN the analysis is rejected with deterministic edge-effect metadata.

### Requirement: Monotone Transfer Admission
r[mir-dataflow.admission] The system MUST admit only transfer/effect tables that are monotone under the declared finite lattice order.

#### Scenario: Monotone transfer is accepted
r[mir-dataflow.admission.monotone]
- GIVEN a transfer table that preserves the lattice order for every finite input pair
- WHEN analysis admission is evaluated
- THEN the transfer table is accepted.

#### Scenario: Non-monotone transfer is rejected
r[mir-dataflow.admission.non-monotone]
- GIVEN a transfer table that maps ordered input states to unordered output states
- WHEN analysis admission is evaluated
- THEN the analysis is rejected before fixed-point iteration.

### Requirement: Fixed-Point Convergence
r[mir-dataflow.convergence] For admitted finite analyses, the system MUST prove or mechanically check that iterative dataflow reaches a stable fixed point within the declared convergence budget.

#### Scenario: Stable point satisfies equations
r[mir-dataflow.convergence.fixed]
- GIVEN an admitted finite analysis and MIR-core graph
- WHEN fixed-point iteration stabilizes within budget
- THEN every block entry and exit state satisfies the framework dataflow equations.

#### Scenario: Budget exhaustion rejects result claim
r[mir-dataflow.convergence.budget]
- GIVEN an iteration attempt that does not stabilize within the declared budget
- WHEN a result claim is requested
- THEN the claim is rejected with deterministic convergence metadata.

### Requirement: Results Inspection
r[mir-dataflow.results] The system MUST define pure result-inspection helpers for block entry, block exit, before-effect, primary-effect, and per-edge states derived from fixed-point results.

#### Scenario: Cursor reads admitted location
r[mir-dataflow.results.cursor]
- GIVEN fixed-point results and an admitted MIR location
- WHEN cursor inspection is evaluated for that location
- THEN the returned state corresponds to the declared before or primary effect point.

#### Scenario: Visitor covers requested locations
r[mir-dataflow.results.visitor]
- GIVEN fixed-point results and an admitted body
- WHEN visitor inspection is evaluated
- THEN the visitor observes every requested location state exactly once in the declared traversal order.

### Requirement: Dataflow Evidence Coverage
r[mir-dataflow.evidence] The implementation MUST include positive and negative evidence for domain, lattice, transfer, per-edge, convergence, and result-inspection behavior.

#### Scenario: Positive fixtures cover framework behavior
r[mir-dataflow.evidence.positive.fixtures]
- GIVEN the MIR dataflow framework test suite
- WHEN positive fixtures run
- THEN admitted domains, directions, effects, per-edge effects, convergence, and result inspection are covered.

#### Scenario: Negative fixtures cover malformed analyses
r[mir-dataflow.evidence.negative.fixtures]
- GIVEN the MIR dataflow framework test suite
- WHEN negative fixtures run
- THEN malformed lattice laws, non-monotone transfers, invalid edge effects, and budget failures are rejected deterministically.

### Requirement: Dataflow Documentation Sync
r[mir-dataflow.docs] The implementation MUST update Trellis requirements, source/test annotations, README/module-catalog text, and accepted Cairn spec text for public dataflow-framework helpers.

#### Scenario: Requirement IDs are traceable
r[mir-dataflow.docs.traceable]
- GIVEN a public dataflow-framework helper lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in requirements, source annotations, tests or proofs, and Cairn spec documentation.

### Requirement: Dataflow Validation Evidence
r[mir-dataflow.final-validation] The change MUST capture focused Cargo, Verus, Tracey, and Cairn validation evidence before archive.

#### Scenario: Final validation names commands
r[mir-dataflow.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names commands, outcomes, and any limitations or blockers.

### Requirement: Dataflow Non-Claim Boundary
r[mir-dataflow.non-claims] The dataflow framework MUST NOT claim soundness of any particular Rust analysis, borrow checker, move checker, initializedness checker, or rustc engine unless a separate instantiated primitive proves that analysis.

#### Scenario: Framework claim stays generic
r[mir-dataflow.non-claims.generic]
- GIVEN a consumer cites framework convergence evidence
- WHEN the exported claim is reviewed
- THEN the claim is limited to admitted finite analyses and does not imply semantic correctness of a specific compiler pass.
