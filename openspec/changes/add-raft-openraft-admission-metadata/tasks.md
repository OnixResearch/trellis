## Phase 1: Admission Metadata Surface

- [ ] [serial] Define deterministic result codes and a copyable result envelope for OpenRaft-inspired operational admission in `src/raft_consensus_core.rs`, including explicit finite-model non-claims.
- [ ] [depends:domain-primitives.raft-consensus.openraft-log-pointers] Add spec/exec classifiers for log-pointer lattice violations with stable first-failure ordering and `r[impl]` coverage.
- [ ] [depends:domain-primitives.raft-consensus.openraft-io-progress] Add spec/exec classifiers for I/O progress validity and monotone update violations with `r[impl]` coverage.
- [ ] [depends:domain-primitives.raft-consensus.openraft-vote-log-io] Add spec/exec classifiers for vote/log I/O ordering and effective-membership validity/rollback admission with `r[impl]` coverage.
- [ ] [depends:domain-primitives.raft-consensus.openraft-replication-session] Add spec/exec classifiers for replication-session mismatch, backwards match movement, leader-log bounds, and read-linearizer readiness with `r[impl]` coverage.

## Phase 2: Proofs, Tests, and Spec Sync

- [ ] [depends:openraft-admission-metadata] Add proof lemmas with `r[verify]` coverage showing OK results imply the existing boolean predicates and representative rejection codes preserve/no-mutate relevant metadata.
- [ ] [depends:openraft-admission-metadata] Add focused positive and negative Rust tests for stable first-failure ordering across the OpenRaft-inspired operational family.
- [ ] [depends:openraft-admission-metadata] Sync `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md` wording to the implemented admission-result boundary.
- [ ] [serial] Run and capture focused evidence: `cargo test --quiet raft_consensus_core`, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested, and strict OpenSpec validation for this change.
- [ ] [serial] Run final full-tree verification with `./scripts/verify.sh`, reconcile any transient Tracey output, then sync/archive the change when all tasks are complete.
