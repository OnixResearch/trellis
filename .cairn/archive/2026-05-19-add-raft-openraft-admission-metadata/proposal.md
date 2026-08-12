## Why

The Raft/OpenRaft-inspired operational primitives currently expose boolean predicates for log-pointer ordering, I/O progress, vote/log I/O ordering, effective membership, replication-session progress, and read-linearizer readiness. The previous Raft envelope slices proved deterministic admission metadata valuable for review and Tracey coverage. The OpenRaft-inspired surfaces should get the same small deterministic admission envelope so callers can distinguish first-failure causes without overclaiming storage, quorum, lease, or production OpenRaft behavior.

## What Changes

- Add deterministic admission metadata/result codes for the OpenRaft-inspired operational primitive family.
- Provide spec and executable helpers that return the first finite failure for log pointers, I/O progress updates, vote/log I/O ordering, membership rollback/admission, replication progress, and read-linearizer readiness.
- Preserve existing boolean predicates as the simple validity core and keep durability, crash recovery, RPC quorum, lease timing, and end-to-end linearizability explicitly out of scope.

## Capabilities

### Modified Capabilities
- `domain-primitives.raft-consensus.openraft-log-pointers`: Add first-failure admission metadata for pointer lattice violations.
- `domain-primitives.raft-consensus.openraft-io-progress`: Add first-failure admission metadata for invalid or regressing cursor updates.
- `domain-primitives.raft-consensus.openraft-vote-log-io`: Add first-failure admission metadata for log writes without prior term/vote ordering.
- `domain-primitives.raft-consensus.openraft-effective-membership`: Add deterministic metadata for effective-membership validity and rollback admission.
- `domain-primitives.raft-consensus.openraft-replication-session`: Add first-failure admission metadata for stale sessions, backwards matches, and leader-log-bound violations.
- `domain-primitives.raft-consensus.openraft-read-linearizer`: Add deterministic readiness metadata for finite read-log catch-up.

## Impact

- **Files**: Expected implementation in `src/raft_consensus_core.rs`, synced requirement prose in `docs/requirements.md` and `openspec/specs/domain-primitives/spec.md`, and archived evidence under this change.
- **APIs**: May add small copyable result structs/constants plus spec/exec/proof helpers; existing boolean helpers should remain compatible.
- **Dependencies**: No new external dependencies expected.
- **Testing**: Focused Rust tests for positive and negative first-failure cases, `nix develop -c cargo test --quiet raft_consensus_core`, `nix develop -c ./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, strict OpenSpec validation, and final `nix develop -c ./scripts/verify.sh` before archive.
