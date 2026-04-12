## Why

Invocation decode is the point where raw requests become typed kernel operations. A standalone change keeps the soundness proof focused: decode should accept only well-typed, rights-authorized operations and never invent authority.

## What Changes

- Add abstract syscall/message input and typed invocation results.
- Model decode using looked-up capabilities, object types, and argument checks.
- Prove decode soundness and rejection of malformed or under-authorized inputs.

## Capabilities

### New Capabilities

- `sel4-invocation.decode-soundness`: Verified decode returns only well-typed authorized invocations
- `sel4-invocation.reject`: Verified malformed or under-authorized requests are rejected
- `sel4-invocation.no-invented-authority`: Verified decode never invents authority

## Impact

- **Files**: New `src/sel4_invocation.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for kernel invocation typing
- **Dependencies**: Reuses `sel4_cap_rights`, `sel4_cspace`, and object-type modules
- **Testing**: Verus proofs, unit tests, and proptests for decode success/failure cases

## Out of Scope

- Raw syscall number/register ABI
- Execution of the decoded invocation
- Global invariant preservation of every invocation step
