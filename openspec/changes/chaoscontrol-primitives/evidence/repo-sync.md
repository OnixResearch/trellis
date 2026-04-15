## Repo sync audit

Date: 2026-04-14

Goal: confirm `src/lib.rs`, `docs/requirements.md`, `README.md`, and `openspec/specs/`
do not already ship ChaosControl-specific primitives before new implementation.

### Commands

```text
$ rg -n "chaoscontrol|chaos control" src/lib.rs docs/requirements.md README.md openspec/specs -g "*.md" -g "*.rs" || true

$ find openspec/specs -maxdepth 2 -type f | sort
openspec/specs/beam/spec.md
openspec/specs/capability-auth/spec.md
openspec/specs/consensus/spec.md
openspec/specs/consistency-models/spec.md
openspec/specs/control-flow-graph/spec.md
openspec/specs/dead-code-elimination/spec.md
openspec/specs/distributed/spec.md
openspec/specs/dominator-tree/spec.md
openspec/specs/durable-execution/spec.md
openspec/specs/financial-accounting/spec.md
openspec/specs/hypervisor/spec.md
openspec/specs/job-runner/spec.md
openspec/specs/left-right/spec.md
openspec/specs/liveness-analysis/spec.md
openspec/specs/loop-detection/spec.md
openspec/specs/mmu/spec.md
openspec/specs/nickel/spec.md
openspec/specs/nix-derivation/spec.md
openspec/specs/nix/spec.md
openspec/specs/raft-replication/spec.md
openspec/specs/repo-index/spec.md
openspec/specs/secret-sharing/spec.md
openspec/specs/set-associative-lookup/spec.md
openspec/specs/ssa-construction/spec.md
openspec/specs/surelock/spec.md
openspec/specs/time/spec.md
openspec/specs/type-unification/spec.md
openspec/specs/verification-gaps/spec.md
openspec/specs/zircon-kernel/spec.md

$ rg -n "pub mod (bitmap|quorum|monotonic|seq_set)|chaoscontrol" src/lib.rs
317:pub mod bitmap;
318:pub mod quorum;
319:pub mod monotonic;
320:pub mod seq_set;
```

### Result

- `rg` returned no `chaoscontrol` or `chaos control` hits in `src/lib.rs`,
  `docs/requirements.md`, `README.md`, or `openspec/specs/**`.
- `openspec/specs/` currently has no ChaosControl domain spec.
- `src/lib.rs` exports reusable generic helpers such as `bitmap`, `quorum`,
  `monotonic`, and `seq_set`, but no ChaosControl-specific module.
- Manual comparison against those exported generic helpers and
  `evidence/reuse-plan.md` found reusable arithmetic/set facts only; no shipped
  module or main spec currently covers the combined
  coverage/oracle/scheduler/fault-schedule/frontier/input-tree/fault-state
  contract under another name.
