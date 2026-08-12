# MIR Pass Pipeline Specification

## Purpose

Defines finite MIR pass-pipeline primitives for Trellis. The requirements cover phase order, pass compatibility, artifact read/borrow/steal state, validation gates, optimization-level gating, deterministic failure metadata, evidence, documentation, validation, and non-claim boundaries.

## Requirements

### Requirement: MIR Phase Order
r[mir-pass.phase-order] The system MUST define a finite ordered set of MIR pipeline phases and admit only traces whose phase transitions follow that order.

#### Scenario: Ordered phases are accepted
r[mir-pass.phase-order.valid]
- GIVEN a trace that moves from built MIR through later declared phases without reversal
- WHEN phase-order admission is evaluated
- THEN the phase sequence is accepted.

#### Scenario: Phase reversal is rejected
r[mir-pass.phase-order.reversal]
- GIVEN a trace that returns to an earlier MIR phase after a later phase is reached
- WHEN phase-order admission is evaluated
- THEN the trace is rejected with deterministic phase metadata.

### Requirement: MIR Artifact State
r[mir-pass.artifact-state] The system MUST model each intermediate MIR artifact as finite state metadata that records whether it is available for read, borrowed for a dependent query, or stolen by a later phase.

#### Scenario: Available artifact can be read
r[mir-pass.artifact-state.read]
- GIVEN an artifact marked available
- WHEN a read transition is evaluated
- THEN the read is admitted and the artifact remains available unless another transition changes it.

#### Scenario: Stolen artifact cannot be read
r[mir-pass.artifact-state.stolen-read]
- GIVEN an artifact marked stolen
- WHEN a later read transition is evaluated
- THEN the read is rejected with deterministic artifact-state metadata.

### Requirement: Pass Compatibility
r[mir-pass.pass-compatibility] Every admitted pass descriptor MUST declare the MIR phase or phases in which it may run, whether it mutates in place, and which validation predicate must hold before the next phase.

#### Scenario: Compatible pass is accepted
r[mir-pass.pass-compatibility.valid]
- GIVEN a pass descriptor whose input phase matches the current artifact phase
- WHEN pass admission is evaluated
- THEN the pass is accepted for the trace.

#### Scenario: Wrong-phase pass is rejected
r[mir-pass.pass-compatibility.wrong-phase]
- GIVEN a pass descriptor run against an artifact phase outside its declared compatibility set
- WHEN pass admission is evaluated
- THEN the pass is rejected with deterministic pass metadata.

### Requirement: Pipeline Admission
r[mir-pass.admission] Runtime helpers MUST accept only pipeline traces whose phase transitions, pass descriptors, artifact states, validation gates, and optimization gates are all admitted.

#### Scenario: Valid pipeline trace is admitted
r[mir-pass.admission.valid]
- GIVEN a finite trace satisfying phase, pass, artifact, validation, and gate predicates
- WHEN pipeline admission is evaluated
- THEN the trace is accepted.

#### Scenario: Invalid trace fails closed
r[mir-pass.admission.invalid]
- GIVEN a finite trace with a malformed phase, pass, artifact, validation, or gate step
- WHEN pipeline admission is evaluated
- THEN the trace is rejected before exporting a pipeline claim.

### Requirement: Steal Ordering
r[mir-pass.steal-order] The system MUST reject any trace that reads an artifact after it has been stolen unless the required read was recorded before the steal transition.

#### Scenario: Borrow before steal is accepted
r[mir-pass.steal-order.borrow-before]
- GIVEN a dependent query read recorded before a later steal of the same artifact
- WHEN steal-order admission is evaluated
- THEN the ordering is accepted.

#### Scenario: Read after steal is rejected
r[mir-pass.steal-order.read-after]
- GIVEN a read event ordered after a steal event for the same artifact
- WHEN steal-order admission is evaluated
- THEN the trace is rejected with deterministic steal-order metadata.

### Requirement: Optimization Gating
r[mir-pass.optimization-gating] Passes marked as optimization-level gated MUST run only when the finite pipeline settings admit their declared gate.

#### Scenario: Enabled optimization pass runs
r[mir-pass.optimization-gating.enabled]
- GIVEN a pass whose optimization gate is enabled by the pipeline settings
- WHEN pass admission is evaluated
- THEN the pass may appear in the trace.

#### Scenario: Disabled optimization pass is rejected
r[mir-pass.optimization-gating.disabled]
- GIVEN a pass whose optimization gate is disabled
- WHEN pass admission is evaluated for a trace that runs it
- THEN the trace is rejected.

### Requirement: Pipeline Failure Metadata
r[mir-pass.failure-metadata] Pipeline admission MUST report deterministic failure metadata for phase-order, pass-compatibility, missing-validation, optimization-gate, and steal-order failures.

#### Scenario: Deterministic first failure is reported
r[mir-pass.failure-metadata.first]
- GIVEN a malformed pipeline trace with multiple defects
- WHEN admission is evaluated repeatedly
- THEN the same first failure class and trace step are reported.

### Requirement: Pass Pipeline Evidence Coverage
r[mir-pass.evidence] The implementation MUST include positive and negative evidence for phase order, pass compatibility, artifact state, steal ordering, optimization gating, and failure metadata.

#### Scenario: Positive fixtures cover valid traces
r[mir-pass.evidence.positive.fixtures]
- GIVEN the MIR pass-pipeline test suite
- WHEN positive fixtures run
- THEN valid phase, pass, validation, gate, and steal-order traces are covered.

#### Scenario: Negative fixtures cover invalid traces
r[mir-pass.evidence.negative.fixtures]
- GIVEN the MIR pass-pipeline test suite
- WHEN negative fixtures run
- THEN malformed phase, pass, validation, gate, and steal-order traces are rejected deterministically.

### Requirement: Pass Pipeline Documentation Sync
r[mir-pass.docs] The implementation MUST update Trellis requirements, source/test annotations, README/module-catalog text, and accepted Cairn spec text for public pass-pipeline helpers.

#### Scenario: Requirement IDs are traceable
r[mir-pass.docs.traceable]
- GIVEN a public pass-pipeline helper lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in requirements, source annotations, tests or proofs, and Cairn spec documentation.

### Requirement: Pass Pipeline Validation Evidence
r[mir-pass.final-validation] The change MUST capture focused Cargo, Verus, Tracey, and Cairn validation evidence before archive.

#### Scenario: Final validation names commands
r[mir-pass.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names commands, outcomes, and limitations or blockers.

### Requirement: Pass Pipeline Non-Claim Boundary
r[mir-pass.non-claims] The pass-pipeline primitive MUST NOT claim actual rustc query execution, incremental-compilation correctness, pass semantic correctness, optimization soundness, borrow-checker correctness, or whole-compiler correctness.

#### Scenario: Pipeline claim is structural
r[mir-pass.non-claims.structural]
- GIVEN a consumer cites pass-pipeline evidence
- WHEN the exported claim is reviewed
- THEN the claim is limited to finite trace ordering and admission, not to correctness of the transformations in the trace.
