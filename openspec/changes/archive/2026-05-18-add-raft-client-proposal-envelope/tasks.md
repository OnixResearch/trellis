## Phase 1: Implementation Surface

- [x] [serial] Define the finite metadata/spec boundary for `domain-primitives.raft-consensus.client-proposal` in `src/raft_consensus_core.rs`, including constructor-vs-`requires` validity behavior and explicit non-claims. ✅ completed: 2026-05-19T00:15:09Z
- [x] [depends:domain-primitives.raft-consensus.client-proposal] Add executable admission/check functions with `ensures` clauses and `r[impl domain-primitives.raft-consensus.client-proposal]` annotations. ✅ completed: 2026-05-19T00:15:09Z
- [x] [depends:domain-primitives.raft-consensus.client-proposal] Add focused proof lemmas with `r[verify domain-primitives.raft-consensus.client-proposal]` coverage for positive and rejection/no-mutation facts. ✅ completed: 2026-05-19T00:15:09Z

## Phase 2: Evidence and Integration

- [x] [depends:domain-primitives.raft-consensus.client-proposal] Add positive, negative, and mutation-style Rust tests tied to `r[verify domain-primitives.raft-consensus.client-proposal]`. ✅ completed: 2026-05-19T00:15:09Z
- [x] [depends:domain-primitives.raft-consensus.client-proposal] Sync `docs/requirements.md` and canonical domain-primitives requirement prose with the implemented validity boundary. ✅ completed: 2026-05-19T00:15:09Z
- [x] [serial] Run and capture focused evidence: `cargo test --quiet raft_consensus_core`, `./scripts/verus.sh src/lib.rs`, `tracey query uncovered`, `tracey query untested`, and strict OpenSpec validation for this change. ✅ completed: 2026-05-19T00:15:09Z
- [x] [serial] Run final full-tree verification with `./scripts/verify.sh`, reconcile any transient Tracey output, then archive the change when all tasks are complete. ✅ completed: 2026-05-19T00:15:09Z
