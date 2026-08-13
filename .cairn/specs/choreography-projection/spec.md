# Choreography Projection and Step Specification

## Purpose

Defines endpoint projection and one-step global / local semantics for finite,
projectable choreographies.

## Requirements

### Requirement: Endpoint Projection Has Exact Local Shape
r[choreography.projection]

For every projectable finite choreography `g` and role `r`,
`project_endpoint(r, g) -> LocalChoreo` MUST be a preconditioned-total helper:
it requires projectability and, on that surface, returns the exact local view
implied by the structure of `g`. That precondition depends only on `g`, so it
is equally admissible for roles outside `participants(g)`:
- `End` projects to `End`
- a communication prefix projects to `Send` for the sender, `Recv` for the
  receiver, and the projected continuation for every uninvolved role
- a choice projects to a normalized `InternalChoice` for the decider over the
  projected branches in branch-label order
- a choice projects for every non-decider role to the exact outer-label-
  preserving merge of the per-branch projections, preserving any shared local
  prefix and introducing an `Offer` from the decider whose branch labels are the
  stored global choice labels exactly at the first branch-dependent point
- a role outside `participants(g)` projects to `End`

#### Scenario: Sender projection produces send head

- GIVEN a projectable communication prefix from `A` to `B`
- WHEN projecting to `A`
- THEN the result MUST have a `Send` head to `B`

#### Scenario: Receiver projection produces receive head

- GIVEN a projectable communication prefix from `A` to `B`
- WHEN projecting to `B`
- THEN the result MUST have a `Recv` head from `A`

#### Scenario: Decider projection produces internal choice

- GIVEN a projectable choice with decider `D`
- WHEN projecting to `D`
- THEN the result MUST be `InternalChoice` over exactly the branch labels of
  the global choice

#### Scenario: Nonparticipant projects to end

- GIVEN a projectable choreography `g`
- AND a role `R` not in `participants(g)`
- WHEN projecting to `R`
- THEN the result MUST be `End`

#### Scenario: Common prefix is preserved before offer

- GIVEN a projectable choice whose non-decider branch projections share the
  same local prefix before differing on a later receive from the decider
- WHEN projecting to that non-decider role
- THEN the result MUST keep that exact shared prefix
- AND introduce an `Offer` only at the first branch-dependent point

### Requirement: One-Step Semantics Preserve Projected Continuations
r[choreography.steps]

A projectable finite choreography has exact one-step global and local semantics.
`global_comm_step(g) -> Option<GlobalChoreo>` MUST return `Some(next)` exactly
when `g` has a `Comm` head, and `None` otherwise. `global_choice_step(g, label)
-> Option<GlobalChoreo>` MUST return `Some(branch_l)` exactly when `g` has a
`Choice` head with exact branch label `label`, and `None` otherwise. Local
communication stepping is exposed by total helpers
`local_send_step(local) -> Option<LocalChoreo>` and
`local_recv_step(local) -> Option<LocalChoreo>`; local branch stepping is
exposed by total helpers
`local_internal_choice_step(local, label) -> Option<LocalChoreo>` and
`local_offer_step(local, label) -> Option<LocalChoreo>`. These local helpers
MUST return `Some(next)` only for the matching local head and exact chosen
label, and `None` on every other input. A global communication head reduces to
its continuation. A global choice head reduces to the branch selected by one
exact branch label. Every successful global head step result `Some(g')` MUST
remain on the projectable surface. Sender / receiver heads and any currently
exposed `InternalChoice` / `Offer` heads MUST reduce to the projections of the
same continuation or selected branch. Roles whose local view does not expose
such a head, including uninvolved roles and branch merges that preserve a
shared prefix, MUST already equal the corresponding endpoint projection after
the global step.

#### Scenario: Communication step matches projected continuations

- GIVEN a projectable communication head `g_before = Comm { from = A, to = B, ... next }`
- WHEN the global choreography takes one communication step
- THEN `A`'s projected `Send` head steps to `project_endpoint(A, next)`
- AND `B`'s projected `Recv` head steps to `project_endpoint(B, next)`
- AND every uninvolved role MUST already satisfy
  `project_endpoint(role, g_before) == project_endpoint(role, next)`

#### Scenario: Choice step matches projected branch continuations

- GIVEN a projectable choice with label set `L`
- WHEN the global choreography selects one branch label `l` in `L`
- THEN the decider's projected `InternalChoice` steps to the projection of
  branch `l`
- AND every non-decider projected head that already exposes an `Offer` for that
  choice steps on `l` to the projection of branch `l`
- AND every remaining non-decider projected view already equals the projection
  of branch `l` under its preserved shared prefix or identical behavior

#### Scenario: Missing branch label does not fabricate a step

- GIVEN an `InternalChoice` or `Offer` whose label set does not contain `l`
- WHEN calling its exact label-selection step helper with `l`
- THEN the result MUST be `None`

#### Scenario: Successful global head step preserves projectability

- GIVEN a projectable choreography whose global head-step helper returns
  `Some(g_after)`
- WHEN using `g_after` as the post-step choreography
- THEN `project_endpoint(role, g_after)` MUST remain admissible for every role

#### Scenario: Uninvolved role is unchanged by the head step

- GIVEN a projectable choreography `g_before` whose head step to `g_after`
  does not involve role `R`
- WHEN the head step occurs
- THEN `project_endpoint(R, g_before)` MUST equal `project_endpoint(R, g_after)`

### Requirement: Owned transfers project to exact local heads

r[choreography.value_transfer_projection.shape] For every projectable owned-transfer head, endpoint projection MUST produce a send-owned head for the source, a receive-owned head for the destination, and the projected continuation for every uninvolved role. The involved heads MUST preserve exact peers, label, payload tag, value identity, and continuation.

#### Scenario: Source projects to send-owned

r[choreography.value_transfer_projection.shape.sender]
- GIVEN a projectable owned-transfer head moves value `V` from `A` to `B`
- WHEN the choreography is projected to `A`
- THEN the result MUST have a send-owned head to `B`
- AND it MUST carry the exact label, payload tag, and value identity `V`.

#### Scenario: Destination projects to receive-owned

r[choreography.value_transfer_projection.shape.receiver]
- GIVEN a projectable owned-transfer head moves value `V` from `A` to `B`
- WHEN the choreography is projected to `B`
- THEN the result MUST have a receive-owned head from `A`
- AND it MUST carry the exact label, payload tag, and value identity `V`.

#### Scenario: Uninvolved role projects through transfer

r[choreography.value_transfer_projection.shape.uninvolved]
- GIVEN role `R` is neither source nor destination of a projectable owned-transfer head
- WHEN the choreography is projected to `R`
- THEN the result MUST equal the projection of the stored continuation.

### Requirement: Owned-transfer merge is exact

r[choreography.value_transfer_projection.merge] Local branch merge MUST preserve a shared send-owned or receive-owned prefix only when direction, peer, label, payload tag, and value identity match exactly. Every transfer-head mismatch MUST fail merge.

#### Scenario: Equal transfer heads merge

r[choreography.value_transfer_projection.merge.valid]
- GIVEN two local branch projections have equal send-owned heads and mergeable continuations
- WHEN local merge runs
- THEN the result MUST preserve the exact shared transfer head
- AND recursively merge the continuations.

#### Scenario: Value identities differ

r[choreography.value_transfer_projection.merge.value_mismatch]
- GIVEN two local transfer heads differ only in value identity
- WHEN local merge runs
- THEN the merge MUST fail without erasing the value distinction.
