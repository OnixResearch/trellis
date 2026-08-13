## Why

Trellis proves finite choreography projection, branch agreement, communication duality, and step correspondence. The current model carries symbolic payload tags, but it does not identify a value or record which role owns that value.

A role-located authoring model needs a smaller verified fact: one admitted value has one owner at each semantic boundary, and a successful transfer changes only that owner. Without this layer, downstream syntax such as a value at a role can become an unchecked annotation.

Wyzer at revision `7f6acf0f60cf34357e06de50112eb2b1ed833a0c` is an MIT-licensed design reference for role-located values, explicit transfer, and negative moved-value examples. The project operator reports direct MIT confirmation from the author. This change can reuse suitable MIT material after source review, but it selects Trellis-owned specifications and proofs as authority.

## What Changes

- Add a distinct value identity and a normalized finite ownership state.
- Add an explicit global transfer node and exact local send-owned and receive-owned projections.
- Admit a transfer only when the source is the current owner and source and destination differ.
- Make a successful global transfer atomically replace only the selected value owner.
- Prove ownership uniqueness, transfer conservation, old-owner exclusion, unrelated-value preservation, projection duality, and step correspondence.
- Add positive, negative, and property evidence for successful transfer, stale owner, duplicate value, self-transfer, wrong peer, wrong value, and use after transfer.
- Preserve existing symbolic communication semantics and all current choreography theorems.

## Success Criteria

- Every admitted ownership state contains one entry per value identity.
- A successful transfer moves one value from the exact source to the exact destination.
- Every unrelated value and every non-ownership choreography field remains unchanged.
- A second transfer from the former owner fails unless ownership moved back through another admitted transfer.
- Sender and receiver projections carry the same value, label, payload tag, peers, and continuation.
- Full-library Verus passes without a new trusted boundary, assumption, or proof-gap bridge.

## False Completion

A payload tag that only names a type is not ownership evidence. A runtime move check without a mathematical ownership state is not completion.

Tests without production-linked proofs are not completion. A proof over syntax that does not constrain the executable transfer step is not completion.

## Impact

- **Core**: `src/choreography_global.rs`, `src/choreography_local.rs`, and one focused ownership module if separation reduces proof churn.
- **Projection**: `src/choreography_projection.rs` and exact merge behavior for transfer heads.
- **Steps and properties**: `src/choreography_step.rs`, `src/properties.rs`, and production-linked Verus proofs.
- **Requirements**: canonical choreography requirements and Tracey links.
- **Compatibility**: existing `Comm`, `Send`, and `Recv` behavior remains valid. New owned-transfer constructors are additive.

## Dependencies

- Review the result of `pilot-choreography-payload-tag-newtype` before selecting the `ValueId` representation.
- The value identity must remain distinct from role, label, and payload-tag domains.
- This change does not require role or label newtypes.

## Out of Scope

- Transport, serialization, retries, clocks, persistence, authorization, or external effect execution.
- Proving delivery, deadlock freedom under faults, exactly-once transfer, or byte ownership.
- Adopting Wyzer networking, Perceus, FBIP, compile-time execution, or the C ABI.
- Replacing Rust ownership or memory management.
