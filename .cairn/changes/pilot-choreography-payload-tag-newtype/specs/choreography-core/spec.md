# Choreography Payload-Tag Newtype Pilot Specification

## Purpose

Defines a bounded Verus pilot that separates payload tags from raw choreography labels while preserving all current global, local, projection, merge, step, and property semantics.

## Requirements

### Requirement: Typed Choreography Payload Tag
r[newtypes.choreography-payload.type] Choreography payload positions MUST use a distinct `PayloadTag` structure over the complete `u32` value domain, while role and branch-label representations remain unchanged during this pilot.

#### Scenario: Explicit payload tag is admitted
r[newtypes.choreography-payload.type.valid]
- GIVEN a `PayloadTag` containing any raw `u32` value
- WHEN a global or local communication node is constructed
- THEN the value is accepted in the payload-tag position without an added numeric precondition.

#### Scenario: Label is not a payload tag
r[newtypes.choreography-payload.type.label-rejected]
- GIVEN a `LabelId` or raw `u32` value that has not been explicitly wrapped as `PayloadTag`
- WHEN it is supplied to a payload-tag position
- THEN Rust type checking rejects the program.

### Requirement: Existing Choreography Semantics Are Preserved
r[newtypes.choreography-payload.preservation] The typed payload migration MUST preserve existing well-formedness, projection, merge, projectability, global step, local step, duality, choice-agreement, and step-correspondence results for equivalent raw payload values.

#### Scenario: Projection preserves typed payload
r[newtypes.choreography-payload.preservation.projection]
- GIVEN a projectable communication with one typed payload tag
- WHEN sender and receiver endpoints are projected
- THEN both local heads contain the exact same `PayloadTag`.

#### Scenario: Mismatched payloads remain incompatible
r[newtypes.choreography-payload.preservation.mismatch]
- GIVEN two local heads that differ only in their typed payload tags
- WHEN exact merge compatibility is evaluated
- THEN they remain incompatible under the existing semantics.

### Requirement: Proof Strength Is Not Reduced
r[newtypes.choreography-payload.validation] The pilot MUST pass full-library Verus verification without adding an external body, assumption, unverified public function, or proof-gap bridge.

#### Scenario: Full verification remains green
r[newtypes.choreography-payload.validation.verus]
- GIVEN the completed typed payload migration
- WHEN the full Trellis Verus command runs
- THEN every prior choreography theorem verifies and the trusted-boundary inventory does not increase.

#### Scenario: Proof repair cost is recorded
r[newtypes.choreography-payload.validation.cost]
- GIVEN fresh baseline and post-change evidence
- WHEN the pilot is reviewed
- THEN the evidence records required projection, trigger, equality, and proof repairs plus any exact blocker.

### Requirement: Payload-Tag Positive and Negative Evidence
r[newtypes.choreography-payload.evidence] The pilot MUST include positive semantic tests, negative runtime tests, and compile-fail evidence for cross-domain payload construction.

#### Scenario: Evidence checks both guarantees
r[newtypes.choreography-payload.evidence.coverage]
- GIVEN the focused choreography evidence suite
- WHEN it runs
- THEN semantic preservation passes and label-to-payload misuse fails during compilation.

### Requirement: Payload-Tag Traceability
r[newtypes.choreography-payload.docs] The pilot MUST update canonical requirements, implementation and verification annotations, public documentation, and accepted Cairn text for the typed payload boundary.

#### Scenario: New requirement remains linked
r[newtypes.choreography-payload.docs.traceable]
- GIVEN the completed pilot
- WHEN Tracey and Cairn validation run
- THEN the payload type, preservation, evidence, and validation requirements have complete links.

### Requirement: Pilot Scope Boundary
r[newtypes.choreography-payload.scope] The pilot MUST NOT migrate `RoleId` or `LabelId`, add payload validation, alter choreography semantics, or claim that all verified identifiers require newtypes.

#### Scenario: Later rollout requires separate review
r[newtypes.choreography-payload.scope.later]
- GIVEN a successful payload-tag pilot
- WHEN role or label migration is proposed
- THEN that migration is handled by a separate Cairn change with its own proof-cost evidence.
