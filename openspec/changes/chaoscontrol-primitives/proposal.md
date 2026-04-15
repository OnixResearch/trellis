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
  AFL-style bitmap layout: exact region boundaries, non-interference for
  region-local hit-recording helpers, AFL bucket classification,
  pointwise-max merge, and exact new-edge count queries.
- Add verified helpers for cross-run assertion-oracle verdict evaluation over
  pre-aggregated records, plus exact run-accounting invariants for those
  records; record-folding/update mechanics themselves stay out of scope.
- Add verified primitives for deterministic vCPU scheduling: countdown,
  round-robin turn advance, randomized quantum assignment from caller-supplied
  `rng_word` values in the existing ChaosControl range model,
  schedule-variant reset, and rolling interleaving fingerprint updates.
- Add verified primitives for normalized fault-schedule `add`, due-fault
  draining, cursor monotonicity, and subset extraction used by replay and
  minimization.
- Add verified primitives for exploration heuristics: frontier effective
  scoring/selection and input-tree alternative generation under a fixed budget.
- Add verified predicates for ChaosControl-style fault-state health checks,
  including `u64` alive-mask encoding for `2 <= n <= 64`, directed partition
  counting, the current raft-guest majority-alive + bidirectional-pair health
  predicate, and the derived healthy-period liveness gate used by the guest
  loop.

## Capabilities

### New Capabilities
- `chaoscontrol-coverage.regions`: Verified 64 KiB coverage bitmap regions with exact cut points and isolated region-local hit recording.
- `chaoscontrol-coverage.union`: Verified pointwise-max merge and new-edge counting.
- `chaoscontrol-coverage.classify`: Verified AFL bucket classification for hit counters.
- `chaoscontrol-oracle.verdict`: Verified assertion-oracle verdict rules.
- `chaoscontrol-oracle.accounting`: Verified run-accounting invariants.
- `chaoscontrol-scheduler.turns`: Verified deterministic scheduler advance and quantum semantics.
- `chaoscontrol-scheduler.fingerprint`: Verified rolling schedule-fingerprint update semantics.
- `chaoscontrol-fault-schedule.replay`: Verified normalized fault-schedule `add`, due-fault draining, ordering, and cursor monotonicity.
- `chaoscontrol-fault-schedule.subset`: Verified subset extraction semantics for minimization and replay.
- `chaoscontrol-frontier.selection`: Verified frontier effective-score and greedy-selection rules.
- `chaoscontrol-input-tree.alternatives`: Verified budgeted alternative generation for recorded choice points.
- `chaoscontrol-fault-state.quorum-health`: Verified `u64` alive-mask, directed partition-count, and majority-alive + bidirectional-pair health predicates.
- `chaoscontrol-fault-state.liveness-gate`: Verified healthy-period gate derived from all-alive state plus the current pair-connected health predicate.

## Impact

- **Files**: new `src/chaoscontrol_*.rs` modules, `src/properties.rs` cross-module lemmas, `docs/requirements.md` entries, and this OpenSpec change
- **APIs**: intended implementation surface is the core exec/spec/proof helpers for coverage, oracle verdict/accounting, scheduler transitions/fingerprint, fault-schedule `add`/replay/subset, frontier selection, input-tree alternative generation, and fault-state quorum/healthy-gate logic, each over explicitly stated valid or normalized inputs. Proposal-level validity summary: coverage uses fixed-shape bitmaps, oracle uses valid aggregate records, scheduler uses valid scheduler state, fault-schedule helpers use normalized schedules, frontier selection uses non-empty stable-order inputs, input-tree helpers use normalized choice batches, and fault-state helpers use valid fault states. Each domain fixes its semantic validity surface in design; helpers are total only over those surfaces. Exact helper spellings and `pub mod` exports stay design/task decisions unless a delta spec later promotes them into change-level requirements. `src/properties.rs` exposes proof-only cross-module lemmas, not a separate exec API surface
- **Dependencies**: no new runtime deps expected; may reuse existing local modules such as `bitmap`, `quorum`, `monotonic`, and `seq_set` where helpful
- **Testing**: implementation-completion validation plan uses Verus proofs, unit tests, property tests, fresh `./scripts/check-ensures.sh`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, direct Tracey queries, and fresh `./scripts/verify.sh`; these command captures are completion evidence, not separate proposal-stage semantic requirements, except for the deterministic iterative-exec rule tracked in `specs/chaoscontrol-validation/spec.md`
- **Repo sync**: preflight comparison against `src/lib.rs`, `docs/requirements.md`, `README.md`, and `openspec/specs/` found no shipped ChaosControl-specific module or spec. Fresh evidence is recorded in `openspec/changes/chaoscontrol-primitives/evidence/repo-sync.md` and currently pins pre-change baseline `acddde9851a3`; that audit found no ChaosControl hits in the shipped baseline and notes that current working-tree `docs/requirements.md` `properties.chaoscontrol-*` anchors are change-local additions, not shipped overlap. Existing generic helpers such as `bitmap`, `quorum`, `monotonic`, and `seq_set` remain reusable, but no current shipped surface covers the combined coverage/oracle/scheduler/fault-schedule/frontier/input-tree/fault-state contract under another name. Implementation tasks keep that repo-sync check explicit so overlap is rechecked before code lands.
- **Traceability**: `docs/requirements.md` will mirror the code-traceable requirement set in this change: `chaoscontrol-coverage.regions`, `chaoscontrol-coverage.union`, `chaoscontrol-coverage.classify`, `chaoscontrol-oracle.verdict`, `chaoscontrol-oracle.accounting`, `chaoscontrol-scheduler.turns`, `chaoscontrol-scheduler.fingerprint`, `chaoscontrol-fault-schedule.replay`, `chaoscontrol-fault-schedule.subset`, `chaoscontrol-frontier.selection`, `chaoscontrol-input-tree.alternatives`, `chaoscontrol-fault-state.quorum-health`, `chaoscontrol-fault-state.liveness-gate`, `properties.chaoscontrol-subset-replay`, `properties.chaoscontrol-fingerprint-trace`, and `properties.chaoscontrol-quorum-gate`. The validation-only requirement `chaoscontrol-validation.iterative-exec` stays change-local in `specs/chaoscontrol-validation/spec.md` and its evidence artifact. Every mirrored requirement and `properties.*` rule will map to a concrete defining `r[impl]` artifact; every proof-style rule, especially each `properties.*` requirement, will also land explicit `r[verify]` proof artifacts; tests alone are not enough

## Out of Scope

- KVM ioctls, VM snapshot side effects, guest memory I/O, or full replay orchestration
- eBPF event collection and trace serialization
- Host wall-clock timing and profiling instrumentation
- Full explorer state-machine verification end to end
- Probabilistic shell policies outside the pure kernels, such as epsilon-random frontier sampling
- Guest-specific Raft protocol semantics beyond the shared fault-state helpers
