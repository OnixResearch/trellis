# Choreography Core Value-Transfer Delta

## ADDED Requirements

### Requirement: Choreography value identities are distinct

r[choreography.value_ownership.identity] Owned choreography values MUST use a distinct value-identity domain that cannot be confused with role identifiers, branch labels, or payload tags.

#### Scenario: Value identity is used in a transfer

r[choreography.value_ownership.identity.valid]
- GIVEN a well-formed value identity and an admitted ownership state
- WHEN an owned-transfer node names that value
- THEN the node MUST preserve the exact value identity separately from its payload tag.

#### Scenario: Payload tag is supplied as a value identity

r[choreography.value_ownership.identity.invalid]
- GIVEN a role identifier, branch label, or payload tag without explicit value-identity construction
- WHEN it is supplied to a value-identity position
- THEN the typed boundary MUST reject the construction.

### Requirement: Ownership states are normalized and unique

r[choreography.value_ownership.state] A finite ownership state MUST contain entries in strict value-identity order and MUST contain exactly one owner entry for every represented value identity.

#### Scenario: Normalized ownership state is admitted

r[choreography.value_ownership.state.valid]
- GIVEN a finite sequence of distinct values in strict identity order with one role owner per value
- WHEN ownership admission runs
- THEN the state MUST be admitted with the same value-to-owner mapping.

#### Scenario: Value has two owner entries

r[choreography.value_ownership.state.duplicate]
- GIVEN two ownership entries have the same value identity
- WHEN ownership admission runs
- THEN admission MUST fail without choosing one owner.

### Requirement: Owned transfer changes one exact owner

r[choreography.value_ownership.transfer] A global owned-transfer step MUST succeed only when source and destination differ and the source currently owns the selected value. Success MUST advance to the exact continuation and atomically replace that value owner with the destination.

#### Scenario: Current owner transfers a value

r[choreography.value_ownership.transfer.valid]
- GIVEN role `A` owns value `V` and an owned-transfer head selects `A`, destination `B`, and value `V`
- WHEN the global owned-transfer step runs
- THEN the result MUST advance to the stored continuation
- AND the result ownership state MUST record `B` as the owner of `V`.

#### Scenario: Former owner transfers the value again

r[choreography.value_ownership.transfer.stale]
- GIVEN a prior successful transfer moved value `V` from `A` to `B`
- WHEN a later owned-transfer step selects `A` as the source of `V`
- THEN the step MUST fail without a replacement choreography or ownership state.

#### Scenario: Role transfers a value to itself

r[choreography.value_ownership.transfer.self]
- GIVEN an owned-transfer head has equal source and destination roles
- WHEN transfer admission runs
- THEN the transfer MUST be rejected as not well formed.

### Requirement: Owned transfer preserves unrelated ownership

r[choreography.value_ownership.conservation] Every successful owned-transfer step MUST preserve ownership-state normalization and entry count, change only the selected value owner, and preserve every unrelated value owner exactly.

#### Scenario: One value moves among several values

r[choreography.value_ownership.conservation.valid]
- GIVEN a normalized state contains selected value `V` and unrelated values
- WHEN one admitted transfer moves `V` from `A` to `B`
- THEN `B` MUST own `V`
- AND `A` MUST no longer own `V`
- AND every unrelated value owner MUST remain unchanged.

### Requirement: Value ownership claims remain semantic and bounded

r[choreography.value_ownership.boundary] Owned-transfer evidence MUST describe only atomic semantic ownership over supplied finite facts and MUST NOT claim transport delivery, persistence, memory reclamation, authorization, exactly-once execution, or liveness under faults.

#### Scenario: Semantic transfer is reported as network delivery

r[choreography.value_ownership.boundary.overclaim]
- GIVEN the ownership step and all Trellis proofs pass
- WHEN evidence is presented as proof that network bytes reached a remote process
- THEN the claim MUST be rejected as outside the verified boundary.

### Requirement: Value ownership has positive and negative evidence

r[choreography.value_ownership.evidence] The owned-transfer core MUST include positive, negative, compile-fail, and property evidence for identity separation, ownership admission, transfer decisions, conservation, and stale-owner rejection.

#### Scenario: Ownership evidence suite runs

r[choreography.value_ownership.evidence.coverage]
- GIVEN the maintained owned-transfer evidence suite
- WHEN focused checks run
- THEN valid ownership and transfer cases MUST pass
- AND malformed, duplicate, stale, mismatched, and cross-domain cases MUST fail as specified.

### Requirement: Value ownership verification remains production linked

r[choreography.value_ownership.validation] Owned-transfer proofs MUST constrain the production ownership admission and transfer implementation, preserve existing choreography theorems, and add no trusted bridge, assumption, or proof-gap boundary.

#### Scenario: Proof uses another transfer implementation

r[choreography.value_ownership.validation.duplicate]
- GIVEN a proof model does not refine the production owned-transfer step
- WHEN verification evidence is reviewed
- THEN that proof MUST NOT satisfy the change.
