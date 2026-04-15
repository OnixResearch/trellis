## Repo sync audit

Date: 2026-04-15

Goal: confirm `src/lib.rs`, `docs/requirements.md`, `README.md`, and `openspec/specs/`
do not already ship ChaosControl-specific primitives before new implementation.

### Commands

Baseline commit before the ChaosControl change stack: `acddde9851a3`.
Current working tree now contains active-change `docs/requirements.md`
anchors, so the overlap audit must read that baseline commit rather than the
working tree.

```text
$ git grep -n -i "chaoscontrol\|chaos control" acddde9851a3 -- src/lib.rs docs/requirements.md README.md openspec/specs '*.md' '*.rs' || true

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

$ git show acddde9851a3:src/lib.rs | rg -n "pub mod (bitmap|quorum|monotonic|seq_set|properties)|chaoscontrol"
301:pub mod bitmap;
449:pub mod monotonic;
492:pub mod properties;
494:pub mod quorum;
516:pub mod seq_set;
```

### Result

- Baseline commit `acddde9851a3` returned no `chaoscontrol` or
  `chaos control` hits in `src/lib.rs`, `docs/requirements.md`, `README.md`,
  or `openspec/specs/**`.
- `openspec/specs/` currently has no shipped ChaosControl domain spec.
- Baseline `src/lib.rs` exports reusable generic helpers such as `bitmap`,
  `monotonic`, `properties`, `quorum`, and `seq_set`, but no
  ChaosControl-specific module.
- Current working-tree `docs/requirements.md` does contain active-change
  `properties.chaoscontrol-*` anchors; those are change-local additions, not
  evidence of an already shipped overlapping domain.
- Manual comparison against those exported generic helpers and
  `evidence/reuse-plan.md` found reusable arithmetic/set facts only; no shipped
  module or main spec currently covers the combined
  coverage/oracle/scheduler/fault-schedule/frontier/input-tree/fault-state
  contract under another name.
