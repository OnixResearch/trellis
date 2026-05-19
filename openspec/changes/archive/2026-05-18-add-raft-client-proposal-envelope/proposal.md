## Why

The Raft spec calls out client proposals, but the current verified logic has no focused finite envelope for leader-gated command admission before log append.

## What Changes

- Define finite client proposal metadata for command id, target node, term, dedupe/session hints if present, and leader-admission status.
- Admit proposals only through a node state admitted as leader for the proposal term and model.
- Keep client retry, transport, end-to-end linearizability, and external application semantics explicitly out of scope.

## Capabilities

### Modified Capabilities
- `domain-primitives.raft-consensus.client-proposal`: Adds an implementation-ready OpenSpec slice for the currently missing verified surface.

## Impact

- **Files**: Expected implementation in `src/raft_consensus_core.rs`, requirement prose in `docs/requirements.md`, and Tracey annotations on spec/impl/proof/test surfaces.
- **APIs**: May add small copyable metadata structs plus spec/exec/proof functions; no production network/storage/client API is claimed.
- **Dependencies**: No new external dependencies expected.
- **Testing**: Focused Rust tests for positive/negative/mutation cases, `cargo test --quiet raft_consensus_core`, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested, strict OpenSpec validation, and `./scripts/verify.sh` before archive.
