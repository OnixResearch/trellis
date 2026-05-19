## Why

The Raft spec requires follower/candidate/leader node-state admission, but role/term/log/vote/commit/applied metadata is not yet tied together as one verified surface.

## What Changes

- Define finite node-state metadata for follower, candidate, and leader roles.
- Admit node states only when role, current term, vote metadata, log metadata, commit index, and applied index are mutually consistent with the finite model.
- Separate constructor-normalized metadata from predicates that require an already-valid finite model.

## Capabilities

### Modified Capabilities
- `domain-primitives.raft-consensus.node-state`: Adds an implementation-ready OpenSpec slice for the currently missing verified surface.

## Impact

- **Files**: Expected implementation in `src/raft_consensus_core.rs`, requirement prose in `docs/requirements.md`, and Tracey annotations on spec/impl/proof/test surfaces.
- **APIs**: May add small copyable metadata structs plus spec/exec/proof functions; no production network/storage/client API is claimed.
- **Dependencies**: No new external dependencies expected.
- **Testing**: Focused Rust tests for positive/negative/mutation cases, `cargo test --quiet raft_consensus_core`, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested, strict OpenSpec validation, and `./scripts/verify.sh` before archive.
