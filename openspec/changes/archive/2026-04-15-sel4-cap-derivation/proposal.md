## Why

seL4 revocation and authority ancestry depend on a capability derivation tree. Splitting this out gives one place for copy/mint/move/revoke/recycle structure proofs without mixing in CSpace traversal or local rights algebra.

## What Changes

- Add a verified capability derivation tree with topologically ordered parent pointers and live-node tracking.
- Model copy, mint, move, revoke, and recycle transitions.
- Prove revocation removes exactly the live target subtree, including the target root, and recycle clears only a live leaf.

## Capabilities

### New Capabilities

- `sel4-cdt.structure`: Verified acyclic derivation ancestry for live capabilities
- `sel4-cdt.move`: Verified slot relocation that preserves ancestry and descendants
- `sel4-cdt.revoke`: Verified exact live-subtree revocation, including the target root
- `sel4-cdt.recycle`: Verified leaf-only recycle that clears exactly one live node

## Impact

- **Files**: New `src/sel4_cdt.rs` module, `src/lib.rs` export, `docs/requirements.md`, synced `openspec/specs/sel4-cdt/spec.md`, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for capability ancestry, revoke, and recycle
- **Dependencies**: Reuses existing seL4 capability modeling style; no new trusted boundaries
- **Testing**: Verus proofs, unit tests, and proptests for copy/mint, move, revoke, and recycle topologies

## Verification Expectations

- Prove the live derivation relation stays acyclic and parent-consistent across copy, mint, move, revoke, and recycle.
- Prove move preserves both ancestry and descendant membership.
- Prove revoke removes exactly the modeled live target subtree, including the target root.
- Prove recycle succeeds only on live leaves and clears exactly one live node.
- Keep all public verified functions on ordinary `ensures` contracts with no new `#[verifier::external_body]` sites.

## Out of Scope

- CNode path lookup
- Endpoint IPC
- Rights attenuation differences between copy and mint; this change treats them as the same structural child-creation step and leaves rights behavior to `sel4-cap-rights`
- Global kernel invariants beyond the CDT facts themselves
