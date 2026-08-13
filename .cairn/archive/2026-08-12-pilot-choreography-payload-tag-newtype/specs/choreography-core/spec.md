# Choreography Payload-Tag Newtype Pilot Delta

## ADDED Requirements

### Requirement: Typed Choreography Payload Tag

r[newtypes.choreography-payload.type]
Choreography payload positions MUST use a distinct `PayloadTag` structure over the complete `u32` value domain. Role and branch-label representations MUST remain unchanged during this pilot.

#### Scenario: Explicit payload tag is admitted

r[newtypes.choreography-payload.type.valid]
- GIVEN a `PayloadTag` contains any raw `u32` value
- WHEN a global or local communication node is constructed
- THEN the node MUST accept the value without an added numeric precondition.

#### Scenario: Label is not a payload tag

r[newtypes.choreography-payload.type.label-rejected]
- GIVEN a `LabelId` or raw `u32` value is not explicitly wrapped as `PayloadTag`
- WHEN it is supplied to a payload-tag position
- THEN Rust type checking MUST reject the program.

### Requirement: Existing Choreography Semantics Are Preserved

r[newtypes.choreography-payload.preservation]
The typed payload migration MUST preserve existing well-formedness, projection, merge, projectability, global-step, local-step, duality, choice-agreement, and step-correspondence results for equivalent raw payload values.

#### Scenario: Projection preserves typed payload

r[newtypes.choreography-payload.preservation.projection]
- GIVEN a projectable communication has one typed payload tag
- WHEN sender and receiver endpoints are projected
- THEN both local heads MUST contain the exact same `PayloadTag`.

#### Scenario: Mismatched payloads remain incompatible

r[newtypes.choreography-payload.preservation.mismatch]
- GIVEN two local heads differ only in their typed payload tags
- WHEN exact merge compatibility is evaluated
- THEN the heads MUST remain incompatible under the existing semantics.

### Requirement: Proof Strength Is Not Reduced

r[newtypes.choreography-payload.validation]
The pilot MUST pass full-library Verus verification without adding an external body, assumption, unverified public function, or proof-gap bridge.

#### Scenario: Full verification remains green

r[newtypes.choreography-payload.validation.verus]
- GIVEN the typed payload migration is complete
- WHEN the full Trellis Verus command runs
- THEN every prior choreography theorem MUST verify
- AND the trusted-boundary inventory MUST NOT increase.

#### Scenario: Proof repair cost is recorded

r[newtypes.choreography-payload.validation.cost]
- GIVEN fresh baseline and final evidence
- WHEN the pilot is reviewed
- THEN the evidence MUST record projection, trigger, equality, and proof repairs plus any exact blocker.

### Requirement: Payload-Tag Positive and Negative Evidence

r[newtypes.choreography-payload.evidence]
The pilot MUST include positive semantic tests, negative runtime tests, and compile-fail evidence for cross-domain payload construction.

#### Scenario: Evidence checks both guarantees

r[newtypes.choreography-payload.evidence.coverage]
- GIVEN the focused choreography evidence suite
- WHEN it runs
- THEN semantic-preservation cases MUST pass
- AND label-to-payload misuse MUST fail during compilation.

### Requirement: Payload-Tag Traceability

r[newtypes.choreography-payload.docs]
The pilot MUST update canonical requirements, implementation and verification annotations, public documentation, and accepted Cairn text for the typed payload boundary.

#### Scenario: New requirement remains linked

r[newtypes.choreography-payload.docs.traceable]
- GIVEN the pilot is complete
- WHEN Tracey and Cairn validation run
- THEN payload type, preservation, evidence, and validation requirements MUST have complete links.

### Requirement: Pilot Scope Boundary

r[newtypes.choreography-payload.scope]
The pilot MUST NOT migrate `RoleId` or `LabelId`, add payload validation, alter choreography semantics, or claim that all verified identifiers require newtypes.

#### Scenario: Later rollout requires separate review

r[newtypes.choreography-payload.scope.later]
- GIVEN the payload-tag pilot succeeds
- WHEN role or label migration is proposed
- THEN a separate Cairn change MUST record its proof cost.

## MODIFIED Requirements

### Requirement: Participant and Branch Well-Formedness

r[choreography.participants]
Finite choreography primitives MUST use exact `u32` role identifiers and branch labels plus typed `PayloadTag` metadata. A communication prefix MUST be well formed only when `from != to`. A choice MUST be well formed only when its branch list is non-empty and its branch labels are pairwise distinct in strict increasing order. `participants(choreo)` MUST contain exactly the roles that occur as communication endpoints or branch deciders.

#### Scenario: Self-send is rejected

- GIVEN a communication prefix has `from == to`
- WHEN global well-formedness is evaluated
- THEN the choreography MUST be rejected as not well formed.

#### Scenario: Duplicate branch labels are rejected

- GIVEN a choice has two branches with the same label
- WHEN global well-formedness is evaluated
- THEN the choreography MUST be rejected as not well formed.

#### Scenario: Participants are exact

- GIVEN a finite choreography names only roles `1`, `4`, and `9`
- WHEN `participants(choreo)` is evaluated
- THEN the result MUST contain exactly `1`, `4`, and `9`.
