# Choreography Core Specification

## Purpose

Defines the finite global and local choreography primitives, participant /
well-formedness rules, and branch-merge semantics used by endpoint projection.

## Requirements

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

### Requirement: Communication Prefix Semantics
r[choreography.communication]

A global communication prefix
`Comm { from, to, label, payload_tag, next }` MUST denote one exact
point-to-point interaction from `from` to `to` carrying the exact symbolic
message metadata `(label, payload_tag)` and continuing with the exact stored
continuation `next`. This requirement is about the global syntax node itself.
The endpoint-projection specification defines the local projection shape.

#### Scenario: Communication metadata is exact

- GIVEN `Comm { from = A, to = B, label = L, payload_tag = P, next }`
- WHEN reading the global prefix fields
- THEN the prefix MUST expose exact `from = A`, `to = B`, `label = L`, and `payload_tag = P`

#### Scenario: Communication continuation is exact

- GIVEN `Comm { from = A, to = B, label = L, payload_tag = P, next }`
- WHEN reading the stored continuation
- THEN the continuation MUST be exactly `next`

### Requirement: Local Branch Merge Semantics
r[choreography.merge]

For a non-decider role, branch projections over exact local nodes
`End`, `Send { peer, label, payload_tag, next }`,
`Recv { peer, label, payload_tag, next }`,
`InternalChoice { branches }`, and `Offer { from, branches }` MUST merge by a
total helper `merge_local(decider, left, right) -> Option<LocalChoreo>`
parameterized by the exact choice decider. The recursive evaluation MUST
terminate by a structurally decreasing measure over remaining local syntax
size, with branch-list folds additionally decreasing on the remaining
normalized suffix length.
`End` merges only with `End`. Identical `Send` heads merge only when `peer`,
`label`, and `payload_tag` are all equal, and the result MUST keep that exact
`Send` head while recursively merging continuations. Identical `Recv` heads
merge only when `peer`, `label`, and `payload_tag` are all equal, and the
result MUST keep that exact `Recv` head while recursively merging
continuations. `InternalChoice` heads merge only when they carry the same
normalized label set; the result MUST be an `InternalChoice` with that same
ordered label set and recursively merged per-label continuations. Distinct
`Recv` heads from the exact `decider` with pairwise distinct labels merge into
one normalized `Offer` from that decider carrying the per-label continuations
in strict increasing label order. `Offer` heads from the same decider merge by
normalized label-set union: shared labels recursively merge, labels present in
only one input pass through unchanged, and the result keeps strict increasing
label order. Every other head conflict, including matching `peer` / `label`
with different `payload_tag` or `Offer` heads with different `from` roles,
MUST fail the merge.

#### Scenario: Identical prefixes merge recursively

- GIVEN two local choreographies with the same `Recv` head and different but
  mergeable continuations
- WHEN merging them
- THEN the result MUST keep that exact `Recv` head
- AND recursively merge the continuations

#### Scenario: Distinct decider receives form an offer

- GIVEN two local choreographies headed by `Recv` from the same decider with
  distinct labels `L1` and `L2`
- WHEN merging them
- THEN the result MUST be an `Offer` from that decider containing `L1` and `L2`

#### Scenario: Internal choices keep exact label set

- GIVEN two `InternalChoice` heads with the same ordered labels and mergeable
  per-label continuations
- WHEN merging them
- THEN the result MUST be an `InternalChoice` with that same exact ordered
  label set
- AND each matching-label continuation MUST be recursively merged

#### Scenario: Offer union preserves shared and unshared labels

- GIVEN two `Offer` heads from the same decider with shared label `L2` and
  unshared labels `L1` and `L3`
- WHEN merging them
- THEN the result MUST expose exactly `L1`, `L2`, and `L3` in strict
  increasing label order
- AND the `L2` continuation MUST be the recursive merge of the two `L2`
  branches
- AND the `L1` and `L3` branches MUST pass through unchanged

#### Scenario: Conflicting heads fail merge

- GIVEN one local choreography headed by `Send`
- AND another headed by `Recv` with a different shape
- WHEN merging them
- THEN the merge MUST fail

### Requirement: Projectability Is Exact
r[choreography.projectability]

A finite choreography MUST be projectable exactly when it is well formed and,
for every choice and every non-decider role appearing in at least one branch,
the role's branch projections admit the exact projection-side merge: preserve
any shared local `Send` / `Recv` prefix common to every branch in stored order,
and when behavior first diverges, expose an `Offer` from the decider keyed by
the stored global branch labels for the remaining branch-local continuations.
Projectability MUST fail on malformed syntax or on any non-mergeable branch
conflict.

#### Scenario: Common continuation is projectable

- GIVEN a well-formed choice whose branches project to the same local behavior
  for one non-decider role
- WHEN checking projectability
- THEN that role MUST not block projectability

#### Scenario: Non-mergeable branch conflict is not projectable

- GIVEN a well-formed choice whose branches project to incompatible local head
  actions for one non-decider role
- WHEN checking projectability
- THEN the choreography MUST be reported as not projectable

#### Scenario: Three-branch projectability preserves stored outer branch labels

- GIVEN a well-formed choice with stored branch labels `1 < 4 < 9`
- AND a non-decider role whose projections require more than one merge step
- WHEN checking projectability
- THEN successful branch projection MUST preserve the exact stored order `1`,
  then `4`, then `9`
- AND the first exposed `Offer`, if one appears, MUST use those stored outer
  branch labels exactly

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
