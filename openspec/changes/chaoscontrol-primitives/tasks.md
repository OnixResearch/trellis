## Phase 0: Repo Sync

- [x] Compare this change against `src/lib.rs`, `docs/requirements.md`, `README.md`, and `openspec/specs/`; confirm no shipped ChaosControl-specific modules or specs overlap this scope
- [x] Record the repo-sync result in `openspec/changes/chaoscontrol-primitives/evidence/repo-sync.md`
- [x] Record which existing generic helpers (`bitmap`, `quorum`, `monotonic`, `seq_set`) will be reused in `openspec/changes/chaoscontrol-primitives/evidence/reuse-plan.md`

## Phase 1: Coverage and Oracle

- [ ] `src/chaoscontrol_coverage.rs`: fixed-`65536` constructors or `requires`-guarded validity, exact region cut points, and region-writer non-interference (`r[chaoscontrol-coverage.regions]`)
- [ ] `src/chaoscontrol_coverage.rs`: pointwise-max merge, exact `u32` new-edge count, and zero/nonzero count facts (`r[chaoscontrol-coverage.union]`)
- [ ] `src/chaoscontrol_coverage.rs`: exact AFL bucket map `0,1,2,3,4..7,8..15,16..31,32..127,128..255` with named proof artifacts for region/merge/classify soundness (`r[chaoscontrol-coverage.classify]`)
- [ ] Coverage unit/property tests plus `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `chaoscontrol-coverage.*`
- [ ] `src/chaoscontrol_oracle.rs`: shared validity helpers for `runs_satisfied <= runs_hit` and `runs_hit == 0 <=> hit_count == 0`, plus exact boolean and non-boolean record shapes (`r[chaoscontrol-oracle.accounting]`)
- [ ] `src/chaoscontrol_oracle.rs`: verdict helpers total over valid inputs, exact pass/fail complements, and invalid non-boolean shapes outside contract (`r[chaoscontrol-oracle.verdict]`)
- [ ] Named oracle proof artifacts for accounting and verdict totality/exclusivity, plus oracle unit/property tests and `docs/requirements.md` anchors for `chaoscontrol-oracle.*`

## Phase 2: Scheduler and Fault Schedule

- [ ] `src/chaoscontrol_scheduler.rs`: constructor canonicalization, randomized-state validity, `apply_variant` preconditions for invalid randomized overrides, and constructor / `apply_variant` RNG-consumption neutrality (`r[chaoscontrol-scheduler.turns]`)
- [ ] `src/chaoscontrol_scheduler.rs`: single-vCPU no-op rules, SMP `tick` positive-to-zero switch rule, explicit `tick == true -> next transition is advance` contract, round-robin `advance`, and exact randomized quantum assignment from caller `rng_word` (`r[chaoscontrol-scheduler.turns]`)
- [ ] `src/chaoscontrol_scheduler.rs`: exact fingerprint mix, tick neutrality, `apply_variant` persistence / `None`-keeps-current semantics, reset-to-zero epoch, and post-reset lower-inclusive / upper-exclusive randomized range behavior (`r[chaoscontrol-scheduler.fingerprint]`)
- [ ] Scheduler unit/property tests plus `docs/requirements.md` anchors for `chaoscontrol-scheduler.*`
- [ ] `src/chaoscontrol_fault_schedule.rs`: normalized schedule validity, fresh-ordinal allocation `0` else `1 + max(existing)`, and stable `(time_ns, ordinal)` normalization after `add` (`r[chaoscontrol-fault-schedule.replay]`)
- [ ] `src/chaoscontrol_fault_schedule.rs`: exact `drain_due(t)` selection, cursor monotonicity / bounds, preserved undrained suffix, and non-re-delivery (`r[chaoscontrol-fault-schedule.replay]`)
- [ ] `src/chaoscontrol_fault_schedule.rs`: subset first-occurrence selection over the full normalized source schedule, out-of-bounds ignore, carried source-index ordinals, `cursor = 0`, and re-normalization (`r[chaoscontrol-fault-schedule.subset]`)
- [ ] Fault-schedule unit/property tests plus `docs/requirements.md` anchors for `chaoscontrol-fault-schedule.*`
- [ ] `src/properties.rs`: `chaoscontrol_subset_replay_order(...)` proof artifact and tests for `r[properties.chaoscontrol-subset-replay]`
- [ ] `src/properties.rs`: `chaoscontrol_fingerprint_trace_eq(...)` proof artifact and tests for `r[properties.chaoscontrol-fingerprint-trace]`
- [ ] `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `properties.chaoscontrol-subset-replay` and `properties.chaoscontrol-fingerprint-trace`

## Phase 3: Heuristics and Fault State

- [ ] `src/chaoscontrol_frontier.rs`: non-empty frontier validity, exact rational score `base_score / (1 + times_selected)`, and verified no-overflow `u64` cross-product comparator (`r[chaoscontrol-frontier.selection]`)
- [ ] `src/chaoscontrol_frontier.rs`: greedy maximality, monotone decay under re-selection, and lowest-input-index tie-break proofs (`r[chaoscontrol-frontier.selection]`)
- [ ] Frontier unit/property tests plus `docs/requirements.md` anchors for `chaoscontrol-frontier.selection`
- [ ] `src/chaoscontrol_input_tree.rs`: normalized choice-record validity and arithmetic bounds `sequence_id <= u64::MAX - 100` plus `n_options as u64 * (100 + sequence_id) <= u64::MAX` (`r[chaoscontrol-input-tree.alternatives]`)
- [ ] `src/chaoscontrol_input_tree.rs`: exact exec comparator where smaller denominator `n_options * (100 + sequence_id)` wins because the numerator is constant, with proof that exec order matches the mathematical score order (`r[chaoscontrol-input-tree.alternatives]`)
- [ ] `src/chaoscontrol_input_tree.rs`: lower `(vm_id, sequence_id)` tie-breaks, ascending distinct alternative emission excluding the original value, and exact budget prefix `min(budget, full_stream_len)` (`r[chaoscontrol-input-tree.alternatives]`)
- [ ] Input-tree unit/property tests plus `docs/requirements.md` anchors for `chaoscontrol-input-tree.alternatives`
- [ ] `src/chaoscontrol_fault_state.rs`: `2 <= num_nodes <= 64` validity, exact `num_nodes x num_nodes` shape, diagonal-false matrix, cleared high bits in `alive_mask`, and exact directed non-diagonal `partition_count` (`r[chaoscontrol-fault-state.quorum-health]`)
- [ ] `src/chaoscontrol_fault_state.rs`: majority threshold failure, exact bidirectional-pair `quorum_reachable` heuristic, and healthy gate `alive_count == num_nodes && quorum_reachable` (`r[chaoscontrol-fault-state.quorum-health]`, `r[chaoscontrol-fault-state.liveness-gate]`)
- [ ] Fault-state unit/property tests plus `docs/requirements.md` anchors for `chaoscontrol-fault-state.*`
- [ ] `src/properties.rs`: `chaoscontrol_quorum_gate_sound(...)` proof artifact, tests, and `docs/requirements.md` anchors for `r[properties.chaoscontrol-quorum-gate]`

## Phase 4: Integration and Validation

- [ ] Export the new modules from `src/lib.rs` and keep helper-only functions private unless they intentionally carry public API / Tracey surface
- [ ] Run the deterministic non-recursive exec audit for `src/chaoscontrol_scheduler.rs` and `src/chaoscontrol_fault_schedule.rs`: collect non-`spec fn` / non-`proof fn`, build the exec call graph by exact function-name calls, fail on any self-edge or cycle, and record the result in `openspec/changes/chaoscontrol-primitives/evidence/iterative-exec-audit.md`
- [ ] Run `./scripts/check-ensures.sh` and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-check-ensures.txt`
- [ ] Run `cargo test` and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-cargo-test.txt`
- [ ] Run `cargo clippy --all-targets -- -D warnings` and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-clippy.txt`
- [ ] Run `./scripts/verus.sh src/lib.rs` and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-verus.txt`
- [ ] Run `tracey query uncovered` and `tracey query untested`, require empty output or write reviewer-approved disposition in `openspec/changes/chaoscontrol-primitives/evidence/tracey-disposition.md`, and capture both transcripts
- [ ] Run `./scripts/verify.sh` and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-verify.txt`
