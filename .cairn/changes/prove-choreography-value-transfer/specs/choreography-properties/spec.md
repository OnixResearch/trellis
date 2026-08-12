# Choreography Owned-Transfer Properties Delta

## ADDED Requirements

### Requirement: Owned-transfer projection and steps correspond

r[properties.choreography_transfer_correspondence] For every projectable owned-transfer head admitted by a normalized ownership state, the successful global transfer step and matching projected local heads MUST agree on source, destination, label, payload tag, value identity, and continuation. The global result MUST satisfy ownership conservation.

#### Scenario: Sender and receiver agree with global transfer

r[properties.choreography_transfer_correspondence.valid]
- GIVEN role `A` owns value `V` and one projectable transfer moves `V` from `A` to `B`
- WHEN the global step and the matching local send-owned and receive-owned steps run
- THEN both local steps MUST reach the endpoint projections of the global continuation
- AND both local heads MUST identify the same exact transfer
- AND the global ownership state MUST record `B` as the owner of `V`.

#### Scenario: Global ownership rejects the projected action

r[properties.choreography_transfer_correspondence.rejected]
- GIVEN projected transfer heads name source `A` and value `V`
- AND the current normalized ownership state does not record `A` as owner of `V`
- WHEN global transfer correspondence is evaluated
- THEN no successful global transfer correspondence claim MUST be produced.

### Requirement: Successful transfer excludes stale source reuse

r[properties.choreography_transfer_exclusion] After a successful owned-transfer step moves value `V` from `A` to `B`, a later step with unchanged ownership facts MUST NOT admit `A` as the source of `V`.

#### Scenario: Former owner attempts immediate reuse

r[properties.choreography_transfer_exclusion.stale]
- GIVEN a successful transfer changed the owner of `V` from `A` to `B`
- WHEN the next transfer attempt names `A` as source of `V`
- THEN the attempt MUST fail
- AND the result MUST NOT fabricate another ownership state.
