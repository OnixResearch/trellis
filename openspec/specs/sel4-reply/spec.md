# seL4 Reply Capability Specification

## Purpose

Defines requirements for verified `sel4_reply` semantics in this repo. Scope is linear IPC authority behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Reply Cap Is One Shot

r[sel4-reply.one-shot]
A reply capability created by a call MUST authorize at most one successful reply. After reply, the reply capability MUST NOT be reusable.

#### Scenario: First reply succeeds

- GIVEN a live reply capability connecting caller `C` and callee `S`
- WHEN `S` replies once
- THEN the reply succeeds and the reply capability is consumed

#### Scenario: Second reply denied

- GIVEN a consumed reply capability
- WHEN another reply is attempted
- THEN the operation is invalid

### Requirement: Reply Cap Preserves Caller/Callee Pairing

r[sel4-reply.pairing]
A live reply capability MUST remain associated with the caller and callee pair that created it. It MUST NOT be used to reply to a different caller.

#### Scenario: Cross-pair reply rejected

- GIVEN a reply capability created for caller `C1` and callee `S`
- WHEN `S` attempts to use it for caller `C2`
- THEN the operation is invalid

### Requirement: Revoke Or Cancel Invalidates Reply Cap

r[sel4-reply.invalidate]
Revocation or cancellation MUST invalidate a live reply capability so no later reply may succeed through it.

#### Scenario: Cancelled reply cap cannot reply

- GIVEN a live reply capability
- WHEN the call chain is cancelled
- THEN the reply capability becomes unusable
