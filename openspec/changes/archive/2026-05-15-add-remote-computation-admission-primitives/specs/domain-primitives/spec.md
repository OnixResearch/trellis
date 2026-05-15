## ADDED Requirements

### Requirement: Remote Computation Envelope

The system MUST define finite remote computation envelopes containing a computation identity, dependency-closure requirement, ability requirement set, target node identity, and declared resource bounds, without claiming real network transport or execution.

r[domain-primitives.remote-computation.envelope]

#### Scenario: Well-formed envelope is admitted

- GIVEN a remote computation envelope whose computation, target node, dependency set, ability set, and resource bounds are all declared in the finite model
- WHEN envelope admission is evaluated
- THEN the envelope is admitted

#### Scenario: Unknown target node is rejected

- GIVEN a remote computation envelope targeting a node absent from the finite node set
- WHEN envelope admission is evaluated
- THEN the envelope is rejected with unknown-target metadata

### Requirement: Remote Dependency Availability Admission

The system MUST define remote dependency availability admission that accepts a computation only when every dependency hash is already present on the target node or is declared fetchable, and deterministically reports missing or unfetchable dependency hashes.

r[domain-primitives.remote-computation.dependency-availability]

#### Scenario: Present closure admits dependency availability

- GIVEN a target node whose cache contains every dependency hash required by the computation
- WHEN dependency availability is evaluated
- THEN the computation is admitted as dependency-ready

#### Scenario: Fetchable missing dependency is pending

- GIVEN a required dependency hash absent from the target node but declared fetchable from the source
- WHEN dependency availability is evaluated
- THEN the computation is admitted only as fetch-pending with the missing dependency metadata

#### Scenario: Unfetchable missing dependency is rejected

- GIVEN a required dependency hash absent from the target node and not declared fetchable
- WHEN dependency availability is evaluated
- THEN remote admission rejects the computation with unfetchable-dependency metadata

### Requirement: Remote Ability And Resource Admission

The system MUST define remote ability and resource admission where a target node may run a computation only when the sandbox-provided abilities cover the computation's required abilities and declared resource bounds fit within the target's finite resource limits.

r[domain-primitives.remote-computation.ability-resource]

#### Scenario: Ability and resource bounds admit execution eligibility

- GIVEN a computation whose required abilities are provided by the target sandbox and whose declared resource bounds fit the target limits
- WHEN remote execution eligibility is evaluated
- THEN the computation is admitted as eligible in the finite model

#### Scenario: Missing remote ability rejects execution eligibility

- GIVEN a computation requiring `Remote` or `IO`
- WHEN the target sandbox does not provide that ability
- THEN remote execution eligibility rejects the computation with missing-ability metadata

#### Scenario: Resource bound overflow rejects execution eligibility

- GIVEN a computation declaring memory or step bounds above the target's finite limits
- WHEN remote execution eligibility is evaluated
- THEN the computation is rejected with resource-bound metadata
