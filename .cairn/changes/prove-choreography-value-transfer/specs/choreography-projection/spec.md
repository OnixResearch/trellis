# Choreography Owned-Transfer Projection Delta

## ADDED Requirements

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
