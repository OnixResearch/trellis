# Value Identity Representation

## Decision

`ValueId` is a transparent structure with one public `u32` field:

```rust
pub struct ValueId {
    pub raw: u32,
}
```

The wrapper preserves the complete numeric domain. It adds type separation, not value refinement.

`ValueId` remains distinct from `PayloadTag`. A payload tag names symbolic payload-type metadata. Multiple values can share one payload tag.

## Pilot input

The archived `pilot-choreography-payload-tag-newtype` change showed that a transparent wrapper can preserve the complete domain while separating choreography identities. It also showed that recursive syntax changes require broad match-arm and proof repair.

## Proof cost

The baseline had 10,720 verified checks. The final implementation has 10,752 verified checks. The change added 32 verified checks after extending production syntax, projection, merge, ownership updates, and correspondence.

Required repairs included recursive syntax views, clone functions and clone-view preservation, participant and size functions, well-formedness, projectability, projection shape, exact merge arms, step postconditions, lookup-update lemmas, and correspondence theorems.

No trusted bridge, assumption, or proof-gap marker was added.
