## Why

Invocation decode is the point where raw requests become typed kernel operations. A standalone change keeps the soundness proof focused: decode should accept only well-typed, rights-authorized operations and never invent authority.

## What Changes

- Add abstract decode input with an invocation label, argument count/words, and an explicit decode context carrying the looked-up capabilities.
- Start with one concrete starter-set invocation per nearby seL4 surface in this repo: endpoint send, endpoint receive, frame map, and untyped retype.
- Model decode using primary/secondary capability kinds, rights, and message-shape checks.
- Prove decode soundness, rejection of malformed or under-authorized inputs, and structural no-invented-authority.

## Capabilities

### New Capabilities

- `sel4-invocation.decode-soundness`: Verified decode returns only well-typed authorized invocations
- `sel4-invocation.reject`: Verified malformed or under-authorized requests are rejected
- `sel4-invocation.no-invented-authority`: Verified decode never invents authority

## Impact

- **Files**: New `src/sel4_invocation.rs` module, `src/lib.rs` export, `docs/requirements.md` anchors, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for abstract decode over a small typed invocation enum
- **Dependencies**: Reuses `sel4_cap_rights`; starter-set variants reference nearby endpoint, vspace/frame, and untyped object kinds without introducing a shared global cap table
- **Cross-module theorems**: None required for this starter set; the proof surface stays local to `src/sel4_invocation.rs`
- **Testing**: Verus proofs, unit tests, and proptests for decode success/failure cases

## Out of Scope

- Raw syscall number/register ABI
- Execution of the decoded invocation
- Global invariant preservation of every invocation step
