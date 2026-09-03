# Backend Codegen Trace Specification

## Purpose

Defines the `backend-codegen-trace` capability.

## Requirements

### Requirement: Backend Handles
r[backend-trace.handles] The system MUST define finite abstract handles for backend blocks, values, functions, types, places, and builder operations.

#### Scenario: Declared handle is admitted
r[backend-trace.handles.valid]
- GIVEN a backend handle declared before use in the finite trace
- WHEN handle-use admission is evaluated
- THEN the handle is accepted for that operation.

#### Scenario: Dangling handle is rejected
r[backend-trace.handles.dangling]
- GIVEN a builder operation that references a handle absent from the declaration set
- WHEN trace admission is evaluated
- THEN the operation is rejected with deterministic handle metadata.

### Requirement: MIR to Backend Block Mapping
r[backend-trace.block-mapping] The system MUST map each admitted MIR basic block to at least one declared backend block and MUST admit additional expansion blocks only when declared by the lowering metadata.

#### Scenario: Direct block mapping is accepted
r[backend-trace.block-mapping.direct]
- GIVEN a MIR block mapped to one declared backend block
- WHEN block-mapping admission is evaluated
- THEN the mapping is accepted.

#### Scenario: Declared expansion is accepted
r[backend-trace.block-mapping.expansion]
- GIVEN a MIR block whose lowering metadata declares finite expansion backend blocks
- WHEN block-mapping admission is evaluated
- THEN those expansion blocks are accepted as part of the MIR block's lowering trace.

#### Scenario: Undeclared expansion is rejected
r[backend-trace.block-mapping.undeclared]
- GIVEN a backend block emitted for a MIR block without declaration in direct or expansion metadata
- WHEN block-mapping admission is evaluated
- THEN the trace is rejected.

### Requirement: Lowering Trace
r[backend-trace.lowering-trace] The system MUST model statement, operand, place, rvalue, and terminator lowering as deterministic finite trace entries tied to admitted MIR-core locations.

#### Scenario: Statement trace is tied to source location
r[backend-trace.lowering-trace.statement]
- GIVEN a trace entry for a MIR statement location admitted by MIR core
- WHEN trace admission is evaluated
- THEN the entry is associated with that source location.

#### Scenario: Trace for invalid location is rejected
r[backend-trace.lowering-trace.invalid-location]
- GIVEN a trace entry whose source MIR location is not admitted
- WHEN trace admission is evaluated
- THEN the entry is rejected with deterministic source-location metadata.

### Requirement: Handle Validity
r[backend-trace.handle-validity] Every builder operation in an admitted trace MUST reference only declared handles that are valid at that point in the trace.

#### Scenario: Value use follows definition
r[backend-trace.handle-validity.defined-before-use]
- GIVEN a builder operation that uses a value handle defined earlier in the trace
- WHEN operation admission is evaluated
- THEN the use is accepted.

#### Scenario: Value use before definition is rejected
r[backend-trace.handle-validity.use-before-def]
- GIVEN a builder operation that uses a value handle before any declaration or definition of that handle
- WHEN operation admission is evaluated
- THEN the trace is rejected.

### Requirement: Terminator Lowering
r[backend-trace.terminators] An admitted terminator trace MUST emit control-flow operations whose targets are declared backend blocks corresponding to MIR successors, call return paths, unwind paths, or declared expansion blocks.

#### Scenario: Branch targets mapped successors
r[backend-trace.terminators.branch]
- GIVEN a MIR branch terminator with admitted successors and a trace branch operation
- WHEN terminator trace admission is evaluated
- THEN the backend branch targets correspond to declared backend blocks for those successors.

#### Scenario: Missing terminator operation is rejected
r[backend-trace.terminators.missing]
- GIVEN a lowered MIR block trace without an admitted terminator control-flow operation
- WHEN terminator trace admission is evaluated
- THEN the trace is rejected.

### Requirement: SSA-Like Local Metadata
r[backend-trace.ssa-like-locals] The system MUST define finite metadata identifying locals that may be lowered directly as backend values versus locals that remain memory-backed, without relying on backend-specific optimization passes.

#### Scenario: SSA-like local is direct-valued
r[backend-trace.ssa-like-locals.direct]
- GIVEN a local classified as SSA-like by admitted metadata
- WHEN local lowering admission is evaluated
- THEN direct-value lowering is accepted for that local.

#### Scenario: Conflicting local classification is rejected
r[backend-trace.ssa-like-locals.conflict]
- GIVEN a local classified both direct-valued and memory-backed in incompatible metadata
- WHEN local-classification admission is evaluated
- THEN the classification is rejected.

### Requirement: Backend Trace Admission
r[backend-trace.admission] Runtime helpers MUST reject malformed backend traces with deterministic metadata rather than panicking, indexing unchecked, or silently accepting unsupported operation kinds.

#### Scenario: First trace failure is deterministic
r[backend-trace.admission.first-failure]
- GIVEN a backend trace with multiple defects
- WHEN admission is evaluated repeatedly
- THEN the same first failure class and trace entry are reported.

### Requirement: Backend Trace Evidence Coverage
r[backend-trace.evidence] The implementation MUST include positive and negative evidence for handle use, block mapping, lowering trace shape, terminator lowering, SSA-like local metadata, and failure behavior.

#### Scenario: Positive fixtures cover valid trace behavior
r[backend-trace.evidence.positive.fixtures]
- GIVEN the backend-codegen trace test suite
- WHEN positive fixtures run
- THEN direct block, expanded block, operand, place, rvalue, statement, terminator, and SSA-like local traces are covered.

#### Scenario: Negative fixtures cover malformed trace behavior
r[backend-trace.evidence.negative.fixtures]
- GIVEN the backend-codegen trace test suite
- WHEN negative fixtures run
- THEN dangling handles, invalid mapping, invalid locations, missing terminators, use-before-definition, and unsupported operations are rejected deterministically.

### Requirement: Backend Trace Documentation Sync
r[backend-trace.docs] The implementation MUST update Trellis requirements, source/test annotations, README/module-catalog text, and accepted Cairn spec text for public backend-codegen trace helpers.

#### Scenario: Requirement IDs are traceable
r[backend-trace.docs.traceable]
- GIVEN a public backend trace helper lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in requirements, source annotations, tests or proofs, and Cairn spec documentation.

### Requirement: Backend Trace Validation Evidence
r[backend-trace.final-validation] The change MUST capture focused Cargo, Verus, Tracey, and Cairn validation evidence before archive.

#### Scenario: Final validation names commands
r[backend-trace.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names commands, outcomes, and limitations or blockers.

### Requirement: Backend Trace Non-Claim Boundary
r[backend-trace.non-claims] The backend-codegen trace primitive MUST NOT claim LLVM IR correctness, concrete backend correctness, target ABI correctness, machine-code correctness, linker behavior, LTO behavior, debug-info correctness, or backend performance.

#### Scenario: Trace evidence is backend-agnostic
r[backend-trace.non-claims.backend-agnostic]
- GIVEN a consumer cites backend trace evidence
- WHEN the exported claim is reviewed
- THEN the claim is limited to finite backend-agnostic trace shape and handle validity.
