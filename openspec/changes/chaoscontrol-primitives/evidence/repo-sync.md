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
```

### Result

- `rg` returned no `chaoscontrol` or `chaos control` hits in `src/lib.rs`,
  `docs/requirements.md`, `README.md`, or `openspec/specs/**`.
- `openspec/specs/` currently has no ChaosControl domain spec.
- Current shipped surfaces do not appear to already contain this change's
  ChaosControl-specific scope.
