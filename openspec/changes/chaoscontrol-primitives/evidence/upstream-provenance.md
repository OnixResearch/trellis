# Upstream provenance map

Pinned upstream revision: `fa5123f04b017d5d3e260215eb71026e79033b3a`
(`../chaoscontrol`, captured 2026-04-15).

These upstream files define the current ChaosControl semantics this change is
trying to preserve exactly at the pure-kernel boundary.

- coverage bitmap, merge, new-edge counting, AFL buckets:
  `../chaoscontrol/crates/chaoscontrol-explore/src/coverage.rs`
- aggregated assertion counters and report-side verdict shapes:
  `../chaoscontrol/crates/chaoscontrol-vmm/src/controller.rs`
  `../chaoscontrol/crates/chaoscontrol-explore/src/report.rs`
- deterministic scheduler transitions, randomized quantum rule, fingerprint, variant reset:
  `../chaoscontrol/crates/chaoscontrol-vmm/src/scheduler.rs`
- fault schedule add/drain/reset/subset behavior:
  `../chaoscontrol/crates/chaoscontrol-fault/src/schedule.rs`
- frontier scoring and `times_selected` decay:
  `../chaoscontrol/crates/chaoscontrol-explore/src/frontier.rs`
- input-tree choice ranking and alternative generation:
  `../chaoscontrol/crates/chaoscontrol-explore/src/input_tree.rs`
- fault-state alive-mask, partition counting, quorum-reachable heuristic, healthy window:
  `../chaoscontrol/crates/chaoscontrol-raft-guest/src/main.rs`
