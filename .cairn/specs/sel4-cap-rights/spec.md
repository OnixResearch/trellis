# seL4 Capability Rights Specification

## Purpose

Defines requirements for verified `sel4_cap_rights` semantics in this repo. Scope is authority behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Capability Rights Attenuation
r[sel4-cap-rights.attenuation]

A derived capability MUST NOT grant any right absent from its source capability. Copy MAY preserve rights and masking MAY drop rights, but derivation MUST NOT add authority.

#### Scenario: Copy preserves rights

- GIVEN a capability with rights set `R`
- WHEN the capability is copied unchanged
- THEN the derived capability has rights `R`

#### Scenario: Masking drops rights only

- GIVEN a capability with rights set `R`
- WHEN a derived capability requests `R'` where `R' ⊆ R`
- THEN the result has rights `R'`

### Requirement: Capability Rights Gate Operations
r[sel4-cap-rights.operation-gates]

Kernel operations MUST require the corresponding authority on the acting capability. Send requires send authority, receive requires receive authority, grant-like transfer requires grant authority, mapping requires mapping authority, and control operations require control authority.

#### Scenario: Send denied without send right

- GIVEN an endpoint capability lacking send authority
- WHEN send is attempted
- THEN the operation is invalid

#### Scenario: Grant denied without grant right

- GIVEN a capability lacking grant authority
- WHEN grant-like transfer is attempted
- THEN the operation is invalid

#### Scenario: Map denied without map right

- GIVEN a frame capability lacking mapping authority
- WHEN map is attempted
- THEN the operation is invalid

### Requirement: Rights Masking Laws
r[sel4-cap-rights.masking]

Masking rights MUST be idempotent and monotone. Applying the same mask twice MUST equal applying it once, and masking with a smaller mask MUST NOT produce a larger rights set.

#### Scenario: Masking is idempotent

- GIVEN rights `R` and mask `M`
- WHEN masking is applied twice
- THEN the result equals masking once
