## Why

AppendEntries admission exists, but the missing central Raft transition is the follower successor-log predicate: rejected appends leave logs unchanged; admitted appends preserve the matching prefix, truncate only conflicting suffixes, and append contiguous entries.

## What Changes

- Define follower log-update candidate/result metadata for AppendEntries.
- Add verified predicates for no-mutation on rejection, matching-prefix preservation, conflict suffix truncation, contiguous append, and successor length bounds.
- Compose with existing log-prefix, AppendEntries, and log-matching primitives without claiming storage durability or network delivery.

## Capabilities

### Modified Capabilities
- `domain-primitives.raft-consensus.follower-log-update`: Adds an implementation-ready OpenSpec slice for the currently missing verified surface.

## Impact

- **Files**: Expected implementation in `src/raft_consensus_core.rs`, requirement prose in `docs/requirements.md`, and Tracey annotations on spec/impl/proof/test surfaces.
- **APIs**: May add small copyable metadata structs plus spec/exec/proof functions; no production network/storage/client API is claimed.
- **Dependencies**: No new external dependencies expected.
- **Testing**: Focused Rust tests for positive/negative/mutation cases, `cargo test --quiet raft_consensus_core`, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested, strict OpenSpec validation, and `./scripts/verify.sh` before archive.
