# seL4 Invocation Decode Specification

## Purpose

Defines requirements for verified `sel4_invocation` semantics in this repo. Scope is kernel invocation typing behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Invocation Decode Is Sound

r[sel4-invocation.decode-soundness]
Invocation decode MUST return a typed kernel invocation only when the supplied message fields, looked-up capabilities, object types, and rights satisfy that invocation's preconditions. For this change's starter set, the modeled typed invocations are exactly endpoint send, endpoint receive, frame map, and untyped retype. For frame-map, authority is checked on the primary frame capability; the secondary VSpace capability is a typed root reference in this abstract decode surface.

#### Scenario: Well-typed authorized invocation decodes

- GIVEN a capability of the correct type with sufficient rights and well-formed arguments
- WHEN decode is performed
- THEN a typed invocation is returned

### Requirement: Malformed Or Under-Authorized Requests Are Rejected

r[sel4-invocation.reject]
Decode MUST fail or produce a fault when the object type is wrong, required rights are absent, or arguments are malformed. Decode-level malformed checks reject zero size / zero alignment in the starter set, while exact alignment and layout validity remain downstream obligations of the modeled operation modules.

#### Scenario: Wrong object type rejected

- GIVEN a capability whose object type does not match the requested operation
- WHEN decode is performed
- THEN decode fails

#### Scenario: Under-authorized invocation rejected

- GIVEN a capability lacking a required right
- WHEN decode is performed
- THEN decode fails

### Requirement: Decode Never Invents Authority

r[sel4-invocation.no-invented-authority]
Decode MUST NOT introduce authority not reachable from the supplied capability and lookup context. For this abstract decode surface, "reachable from context" means the decoded invocation may mention only object identifiers already carried by the supplied primary/secondary capability records.

#### Scenario: No extra authority appears

- GIVEN a decode context containing only the supplied looked-up capabilities
- WHEN decode succeeds
- THEN the resulting typed invocation uses only authority reachable from that context
