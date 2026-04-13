## Why

`../chaoscontrol/` already contains several pure kernels that would fit this
repo's model well: small deterministic functions with clear algebraic
properties, high test leverage, and thin imperative shells around them. The
highest-value ones are not the KVM or device shells themselves. They are the
search and coordination kernels underneath them: coverage bitmap math,
assertion-oracle verdicts, SMP scheduler transitions and fingerprints, fault
schedule replay/subsetting, frontier heuristics, input-tree alternative
selection, and fault-state predicates used to gate liveness and state
coverage.

This repo already has the right verification style for such code: `spec fn`
for the mathematical rule, executable wrappers that match the spec, proof
lemmas for invariants, and runtime tests for concrete examples. Adding these
ChaosControl-derived primitives here would turn exploration-critical logic
from "tested utility code" into reusable verified building blocks.

## What Changes

- Add verified primitives for AFL-style coverage bitmaps with disjoint code /
  assertion / schedule regions and algebraic merge/new-edge queries.
- Add verified primitives for cross-run assertion-oracle verdict evaluation
  and run-aggregation accounting.
- Add verified primitives for deterministic vCPU scheduling: countdown,
  turn advance, schedule-variant application, and rolling interleaving
  fingerprint updates.
- Add verified primitives for fault schedule ordering, due-fault draining,
  cursor monotonicity, and subset extraction used by schedule minimization.
- Add verified primitives for exploration heuristics: frontier effective
  scoring/selection, dedup-key helpers, and input-tree alternative generation
  under a fixed budget.
- Add verified predicates for ChaosControl-style fault-state health checks,
  including alive-mask encoding, partition counting, and quorum-reachability
  gates.

## Capabilities

### New Capabilities
- `chaoscontrol-coverage.regions`: Verified coverage bitmap regions and merge/new-edge algebra.
- `chaoscontrol-oracle.verdict`: Verified assertion-oracle verdict and run-accounting rules.
- `chaoscontrol-scheduler.turns`: Verified deterministic scheduler advance and quantum semantics.
- `chaoscontrol-scheduler.fingerprint`: Verified rolling schedule-fingerprint update semantics.
- `chaoscontrol-fault-schedule.replay`: Verified due-fault draining, ordering, and cursor monotonicity.
- `chaoscontrol-fault-schedule.subset`: Verified subset extraction semantics for minimization/replay.
- `chaoscontrol-frontier.selection`: Verified frontier effective-score and selection rules.
- `chaoscontrol-input-tree.alternatives`: Verified budgeted alternative generation for recorded choice points.
- `chaoscontrol-fault-state.quorum-health`: Verified alive-mask, partition-count, and quorum-health predicates.

## Impact

- **Files**: new `src/chaoscontrol_*.rs` modules, `src/properties.rs` cross-module lemmas, `docs/requirements.md` entries, and this OpenSpec change
- **APIs**: public spec/exec/proof functions for coverage, oracle verdicts, scheduling, fault schedules, frontier/input-tree heuristics, and fault-state predicates
- **Dependencies**: no new runtime deps expected; may reuse existing local modules such as `bitmap`, `quorum`, `monotonic`, and `seq_set` where helpful
- **Testing**: Verus proofs, unit tests, and property tests for ranking and subset/replay behaviors

## Out of Scope

- KVM ioctls, VM snapshot side effects, guest memory I/O, or full replay orchestration
- eBPF event collection and trace serialization
- Host wall-clock timing and profiling instrumentation
- Full explorer state-machine verification end to end
- Guest-specific Raft protocol semantics beyond the shared fault-state helpers
