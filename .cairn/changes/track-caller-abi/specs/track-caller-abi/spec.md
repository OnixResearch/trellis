# Track Caller ABI Specification

## Purpose

Defines finite `#[track_caller]` ABI primitives for Trellis. The requirements cover attribute validity, caller-location metadata, direct-call propagation, tracked-chain forwarding, function-pointer reify shims, trait/default inheritance, const/codegen agreement, evidence, documentation, validation, and non-claim boundaries.

## Requirements

### Requirement: Track Caller Attribute Validity
r[track-caller.attribute-validity] The system MUST admit `track_caller` metadata only for functions whose finite ABI and item-kind metadata allow the attribute.

#### Scenario: Rust ABI function is admitted
r[track-caller.attribute-validity.valid]
- GIVEN a non-closure, non-naked function with the declared Rust ABI and track-caller metadata
- WHEN attribute admission is evaluated
- THEN the function is accepted as tracked.

#### Scenario: Invalid target is rejected
r[track-caller.attribute-validity.invalid]
- GIVEN a closure, naked function, non-Rust ABI function, or unsupported item kind marked tracked
- WHEN attribute admission is evaluated
- THEN the metadata is rejected with deterministic attribute metadata.

### Requirement: Caller Location Metadata
r[track-caller.location-metadata] The system MUST represent caller locations as finite deterministic metadata and MUST distinguish definition-site, call-site, and forwarded caller-location values.

#### Scenario: Call-site location is declared
r[track-caller.location-metadata.call-site]
- GIVEN an untracked caller invoking a tracked callee at a declared call site
- WHEN caller-location selection is evaluated
- THEN the selected location is the call-site metadata.

#### Scenario: Missing location metadata is rejected
r[track-caller.location-metadata.missing]
- GIVEN a tracked call edge without required caller-location metadata
- WHEN call-edge admission is evaluated
- THEN the edge is rejected.

### Requirement: Direct Tracked Call Propagation
r[track-caller.direct-call] A direct call to a tracked callee MUST supply the hidden caller-location metadata selected from the caller context.

#### Scenario: Untracked caller passes call-site location
r[track-caller.direct-call.untracked-caller]
- GIVEN an untracked caller directly calls a tracked callee
- WHEN direct-call propagation is evaluated
- THEN the callee receives the immediate call-site location metadata.

#### Scenario: Tracked caller forwards existing location
r[track-caller.direct-call.tracked-caller]
- GIVEN a tracked caller directly calls a tracked callee
- WHEN direct-call propagation is evaluated
- THEN the callee receives the caller's forwarded caller-location metadata.

### Requirement: Tracked Chain Propagation
r[track-caller.propagation] Across a finite chain of tracked calls, caller-location metadata MUST resolve to the nearest untracked caller boundary or declared definition-site fallback for shim cases.

#### Scenario: Chain forwards nearest untracked caller
r[track-caller.propagation.chain]
- GIVEN an untracked root calls a chain of tracked functions
- WHEN propagation is evaluated for the deepest tracked callee
- THEN the deepest callee receives the root call-site location metadata.

#### Scenario: Broken chain is rejected
r[track-caller.propagation.broken]
- GIVEN a tracked chain with an edge missing required forwarding metadata
- WHEN chain propagation is evaluated
- THEN the chain is rejected.

### Requirement: Reify Shim for Tracked Function Pointers
r[track-caller.reify-shim] Taking a pointer to a tracked function MUST require finite reify-shim metadata that adapts the erased function-pointer boundary without exposing a mismatched hidden-argument ABI.

#### Scenario: Tracked function pointer has shim
r[track-caller.reify-shim.valid]
- GIVEN a tracked function whose address is taken and a matching reify-shim entry is declared
- WHEN function-pointer admission is evaluated
- THEN the pointer use is accepted.

#### Scenario: Missing shim is rejected
r[track-caller.reify-shim.missing]
- GIVEN a tracked function whose address is taken without a declared shim
- WHEN function-pointer admission is evaluated
- THEN the use is rejected with deterministic shim metadata.

### Requirement: Trait Track Caller Inheritance
r[track-caller.trait-inheritance] The system MUST model finite trait-method metadata so prototype annotations, default implementation annotations, and overrides produce deterministic tracked/untracked method status.

#### Scenario: Prototype annotation applies to implementation
r[track-caller.trait-inheritance.prototype]
- GIVEN a trait method prototype marked tracked and an implementation method without an explicit local marker
- WHEN inheritance admission is evaluated
- THEN the implementation method is treated as tracked.

#### Scenario: Default annotation applies to override
r[track-caller.trait-inheritance.default]
- GIVEN a default trait method marked tracked and an overriding implementation
- WHEN inheritance admission is evaluated
- THEN the override inherits tracked status according to the declared finite metadata.

### Requirement: Const and Codegen Agreement
r[track-caller.const-codegen-agreement] The system MUST define a finite agreement check requiring const-context and codegen-context caller-location traces to select the same abstract location for the same admitted call graph.

#### Scenario: Matching traces are accepted
r[track-caller.const-codegen-agreement.valid]
- GIVEN const and codegen propagation traces selecting the same location metadata for every tracked call
- WHEN agreement is evaluated
- THEN the traces are accepted.

#### Scenario: Divergent traces are rejected
r[track-caller.const-codegen-agreement.diverge]
- GIVEN const and codegen propagation traces that select different locations for an admitted tracked call
- WHEN agreement is evaluated
- THEN the traces are rejected.

### Requirement: Track Caller Admission Helpers
r[track-caller.admission] Runtime helpers MUST reject invalid attribute targets, missing location metadata, missing shims, broken propagation chains, and trace disagreement with deterministic metadata.

#### Scenario: First track-caller failure is deterministic
r[track-caller.admission.first-failure]
- GIVEN malformed track-caller metadata with multiple defects
- WHEN admission is evaluated repeatedly
- THEN the same first failure class and metadata location are reported.

### Requirement: Track Caller Evidence Coverage
r[track-caller.evidence] The implementation MUST include positive and negative evidence for attribute validity, direct calls, tracked chains, shims, trait inheritance, const/codegen agreement, and failure metadata.

#### Scenario: Positive fixtures cover propagation behavior
r[track-caller.evidence.positive.fixtures]
- GIVEN the track-caller ABI test suite
- WHEN positive fixtures run
- THEN valid tracked functions, direct calls, chains, shims, trait inheritance, and agreement traces are covered.

#### Scenario: Negative fixtures cover invalid propagation behavior
r[track-caller.evidence.negative.fixtures]
- GIVEN the track-caller ABI test suite
- WHEN negative fixtures run
- THEN invalid targets, missing locations, missing shims, broken chains, inheritance mismatches, and trace divergence are rejected deterministically.

### Requirement: Track Caller Documentation Sync
r[track-caller.docs] The implementation MUST update Trellis requirements, source/test annotations, README/module-catalog text, and accepted Cairn spec text for public track-caller ABI helpers.

#### Scenario: Requirement IDs are traceable
r[track-caller.docs.traceable]
- GIVEN a public track-caller helper lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in requirements, source annotations, tests or proofs, and Cairn spec documentation.

### Requirement: Track Caller Validation Evidence
r[track-caller.final-validation] The change MUST capture focused Cargo, Verus, Tracey, and Cairn validation evidence before archive.

#### Scenario: Final validation names commands
r[track-caller.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names commands, outcomes, and limitations or blockers.

### Requirement: Track Caller Non-Claim Boundary
r[track-caller.non-claims] The track-caller ABI primitive MUST NOT claim panic formatting correctness, runtime stack walking, actual `Location` allocation, concrete ABI lowering, rustc intrinsic implementation, platform debug-info correctness, or whole-codegen correctness.

#### Scenario: Location metadata is not runtime allocation
r[track-caller.non-claims.metadata-only]
- GIVEN a consumer cites track-caller evidence
- WHEN the exported claim is reviewed
- THEN the claim is limited to finite location metadata propagation and not to runtime `Location` allocation or panic output.
