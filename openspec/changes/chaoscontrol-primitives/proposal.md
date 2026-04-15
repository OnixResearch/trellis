## Why

`../chaoscontrol/` already contains several pure kernels that fit this repo's
model well: small policy kernels with clear algebraic properties,
high test leverage, and thin imperative shells around them. Highest-value ones
are not KVM or device shells themselves. They are search and coordination
kernels underneath them: coverage bitmap math, assertion-oracle verdicts, SMP
scheduler transitions and fingerprints, fault schedule replay/subsetting,
frontier scoring, input-tree alternative generation, and fault-state predicates
used to gate liveness checks.

This repo already has right verification style for such code: `spec fn` for
mathematical rule, executable wrappers that match spec, proof lemmas for
invariants, and runtime tests for concrete examples. Adding ChaosControl-derived
primitives here would turn exploration-critical logic from tested utility code
into reusable verified building blocks.

## What Changes

- Add verified coverage primitives for the existing ChaosControl 64 KiB
  AFL-style bitmap layout: exact region boundaries, AFL bucket
  classification, pointwise-max merge, and exact new-edge count queries.
- Add verified primitives for cross-run assertion-oracle verdict evaluation
  and basic run-aggregation accounting invariants.
- Add verified primitives for deterministic vCPU scheduling: countdown,
  round-robin turn advance, randomized quantum assignment from caller-supplied
  `rng_word` values in the existing ChaosControl range model,
  schedule-variant reset, and rolling interleaving fingerprint updates.
- Add verified primitives for fault schedule ordering, due-fault draining,
  cursor monotonicity, and subset extraction used by replay and minimization.
- Add verified primitives for exploration heuristics: frontier effective
  scoring/selection and input-tree alternative generation under a fixed budget.
- Add verified predicates for ChaosControl-style fault-state health checks,
  including `u64` alive-mask encoding for `2 <= n <= 64`, directed partition
  counting, the current raft-guest majority-alive + bidirectional-pair health
  predicate, and the derived healthy-period liveness gate used by the guest
  loop.

## Capabilities

### New Capabilities
- `chaoscontrol-coverage.regions`: Verified 64 KiB coverage bitmap regions with exact cut points.
- `chaoscontrol-coverage.union`: Verified pointwise-max merge and new-edge counting.
- `chaoscontrol-coverage.classify`: Verified AFL bucket classification for hit counters.
- `chaoscontrol-oracle.verdict`: Verified assertion-oracle verdict rules.
- `chaoscontrol-oracle.accounting`: Verified run-accounting invariants.
- `chaoscontrol-scheduler.turns`: Verified deterministic scheduler advance and quantum semantics.
- `chaoscontrol-scheduler.fingerprint`: Verified rolling schedule-fingerprint update semantics.
- `chaoscontrol-fault-schedule.replay`: Verified due-fault draining, ordering, and cursor monotonicity.
- `chaoscontrol-fault-schedule.subset`: Verified subset extraction semantics for minimization and replay.
- `chaoscontrol-frontier.selection`: Verified frontier effective-score and greedy-selection rules.
- `chaoscontrol-input-tree.alternatives`: Verified budgeted alternative generation for recorded choice points.
- `chaoscontrol-fault-state.quorum-health`: Verified `u64` alive-mask, directed partition-count, and majority-alive + bidirectional-pair health predicates.
- `chaoscontrol-fault-state.liveness-gate`: Verified healthy-period gate derived from all-alive state plus the current pair-connected health predicate.

## Impact

- **Files**: new `src/chaoscontrol_*.rs` modules, `src/properties.rs` cross-module lemmas, `docs/requirements.md` entries, and this OpenSpec change
- **APIs**: public spec/exec/proof functions for coverage, oracle verdicts, scheduling, fault schedules, frontier/input-tree heuristics, and fault-state predicates
- **Dependencies**: no new runtime deps expected; may reuse existing local modules such as `bitmap`, `quorum`, `monotonic`, and `seq_set` where helpful
- **Testing**: Verus proofs, unit tests, property tests, fresh `./scripts/check-ensures.sh`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, direct Tracey queries, and fresh `./scripts/verify.sh`
- **Repo sync**: before implementation, compare against `src/lib.rs`, `docs/requirements.md`, `README.md`, and `openspec/specs/`. Current preflight summary:
  - `rg -n "chaoscontrol|chaos control" src/lib.rs docs/requirements.md README.md openspec/specs -g "*.md" -g "*.rs"` -> no output
  - `rg -n "pub mod (bitmap|quorum|monotonic|seq_set)|chaoscontrol" src/lib.rs` -> only generic helper exports, no ChaosControl modules
- **Traceability**: every new requirement and `properties.*` rule in this change will map to a concrete defining `r[impl]` artifact; every proof-style rule, especially each `properties.*` requirement, will also land explicit `r[verify]` proof artifacts; tests alone are not enough

## Out of Scope

- KVM ioctls, VM snapshot side effects, guest memory I/O, or full replay orchestration
- eBPF event collection and trace serialization
- Host wall-clock timing and profiling instrumentation
- Full explorer state-machine verification end to end
- Probabilistic shell policies outside the pure kernels, such as epsilon-random frontier sampling
- Guest-specific Raft protocol semantics beyond the shared fault-state helpers
