## Why

State-machine safety exists, but the spec still lacks a focused applied-prefix primitive that gates application of committed log entries in order.

## What Changes

- Define finite applied-prefix metadata and admission predicates.
- Admit apply steps only for committed contiguous prefixes, never beyond commit index, local log, or snapshot boundary.
- Prove monotone last-applied advancement and no skipping of committed prefix entries.

## Capabilities

### Modified Capabilities
- `domain-primitives.raft-consensus.applied-prefix`: Adds an implementation-ready OpenSpec slice for the currently missing verified surface.

## Impact

- **Files**: Expected implementation in `src/raft_consensus_core.rs`, requirement prose in `docs/requirements.md`, and Tracey annotations on spec/impl/proof/test surfaces.
- **APIs**: May add small copyable metadata structs plus spec/exec/proof functions; no production network/storage/client API is claimed.
- **Dependencies**: No new external dependencies expected.
- **Testing**: Focused Rust tests for positive/negative/mutation cases, `cargo test --quiet raft_consensus_core`, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested, strict OpenSpec validation, and `./scripts/verify.sh` before archive.
