# Parking-Lot Identity Separation Specification

## Purpose

Defines the Rust type boundary between parking-lot keys and thread identities. The requirements preserve the existing finite queue and raw-mutex semantics while making cross-domain calls fail during compilation.

## Requirements

### Requirement: Distinct Parking-Lot Identity Types
r[newtypes.parking-lot-identities] The parking-lot core MUST represent queue keys as `ParkKey` and waiter or owner identities as `ThreadId`, with no implicit conversion between those domains.

#### Scenario: Correct identities are admitted
r[newtypes.parking-lot-identities.correct]
- GIVEN a queue operation with a `ParkKey` and a waiter with a `ThreadId`
- WHEN the operation is compiled and executed
- THEN each value is accepted only in its declared identity position.

#### Scenario: Exchanged identities do not compile
r[newtypes.parking-lot-identities.exchanged]
- GIVEN a call that supplies a `ParkKey` where `ThreadId` is required, or the reverse
- WHEN the Rust type checker evaluates the call
- THEN compilation fails before the operation can execute.

### Requirement: Typed Parking-Lot Public API
r[newtypes.parking-lot-api] Public parking-lot core and raw-mutex functions MUST use `ParkKey` for queue positions and `ThreadId` for waiter or owner positions instead of accepting those identities as raw `u64` values.

#### Scenario: Park uses both domains
r[newtypes.parking-lot-api.park]
- GIVEN a valid table, one `ParkKey`, and one absent `ThreadId`
- WHEN the waiter is parked
- THEN the resulting queue is selected by the key and contains the exact thread identity.

#### Scenario: Raw mutex records typed ownership
r[newtypes.parking-lot-api.owner]
- GIVEN an unlocked raw mutex and one `ThreadId`
- WHEN fast-path acquisition succeeds
- THEN the owner field contains that `ThreadId`, not a queue key or raw integer.

### Requirement: Existing Parking-Lot Semantics Are Preserved
r[newtypes.parking-lot-semantics] For equivalent raw identity values, the typed implementation MUST preserve existing queue validity, waiter uniqueness, park, unpark, requeue, waiter-count, and raw-mutex transition results.

#### Scenario: Requeue preserves identity and count
r[newtypes.parking-lot-semantics.requeue]
- GIVEN a valid source queue and a distinct destination `ParkKey`
- WHEN wake-and-requeue executes
- THEN moved waiters retain their `ThreadId` values and the existing waiter-count relation holds.

#### Scenario: Invalid table remains rejected
r[newtypes.parking-lot-semantics.invalid]
- GIVEN a table with a duplicate `ThreadId` or a waiter whose recorded `ParkKey` differs from its queue key
- WHEN table validity is evaluated
- THEN the table is rejected as before the type migration.

### Requirement: Parking-Lot Identity Evidence
r[newtypes.parking-lot-evidence] The change MUST include positive runtime tests, negative runtime tests, and compile-fail evidence for both directions of thread/key exchange.

#### Scenario: Evidence covers the type boundary
r[newtypes.parking-lot-evidence.coverage]
- GIVEN the focused parking-lot evidence suite
- WHEN all evidence runs
- THEN valid typed operations pass, invalid runtime states fail, and exchanged identity programs fail to compile.

### Requirement: Parking-Lot Identity Documentation
r[newtypes.parking-lot-docs] The change MUST update canonical requirements, source and test annotations, public API documentation, and accepted Cairn text for the typed identity boundary.

#### Scenario: Identity requirements remain traceable
r[newtypes.parking-lot-docs.traceable]
- GIVEN the completed type migration
- WHEN Tracey and Cairn validation run
- THEN each new requirement has implementation and positive or negative evidence links.

### Requirement: Parking-Lot Identity Non-Claims
r[newtypes.parking-lot-non-claims] The type separation MUST NOT be presented as a Verus proof, scheduler proof, blocking correctness proof, fairness proof, or operating-system synchronization proof.

#### Scenario: Type evidence stays bounded
r[newtypes.parking-lot-non-claims.bounded]
- GIVEN a consumer cites the parking-lot identity evidence
- WHEN the claim is reviewed
- THEN the claim is limited to Rust type separation and preserved finite-model behavior.

### Requirement: Parking-Lot Identity Validation
r[newtypes.parking-lot-validation] The change MUST capture focused and full Cargo tests, compile-fail checks, Clippy, Tracey, and Cairn validation before archive.

#### Scenario: Final evidence is reviewable
r[newtypes.parking-lot-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final evidence is inspected
- THEN it names each command, outcome, and remaining limitation or blocker.
