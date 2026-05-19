## Why

The Raft domain spec names a finite model envelope, but the implementation queue lacks a focused change for the node-universe, term/index bounds, command identifiers, message kinds, and admission metadata that every later Raft primitive should share.

## What Changes

- Define a copyable finite model metadata/envelope surface for Raft primitive admission.
- Add executable/spec predicates for admitted node ids, term bounds, log-index bounds, command ids, message kinds, and configuration epochs.
- Keep runtime networking, storage, liveness, and production cluster behavior explicitly out of scope.

## Capabilities

### Modified Capabilities
- `domain-primitives.raft-consensus.model`: Adds an implementation-ready OpenSpec slice for the currently missing verified surface.

## Impact

- **Files**: Expected implementation in `src/raft_consensus_core.rs`, requirement prose in `docs/requirements.md`, and Tracey annotations on spec/impl/proof/test surfaces.
- **APIs**: May add small copyable metadata structs plus spec/exec/proof functions; no production network/storage/client API is claimed.
- **Dependencies**: No new external dependencies expected.
- **Testing**: Focused Rust tests for positive/negative/mutation cases, `cargo test --quiet raft_consensus_core`, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested, strict OpenSpec validation, and `./scripts/verify.sh` before archive.
