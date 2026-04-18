## Phase 0: Repo Sync

- [x] Compare this change against `src/lib.rs`, `docs/requirements.md`, `README.md`, and `openspec/specs/`; confirm no shipped ChaosControl-specific modules or specs overlap this scope
- [x] Record the repo-sync result in `openspec/changes/chaoscontrol-primitives/evidence/repo-sync.md`
- [x] Record which existing generic helpers (`bitmap`, `quorum`, `monotonic`, `seq_set`) will be reused in `openspec/changes/chaoscontrol-primitives/evidence/reuse-plan.md`
- [x] Record and keep current the upstream source mapping promised in `openspec/changes/chaoscontrol-primitives/evidence/upstream-provenance.md`
- [x] Before landing implementation, refresh `openspec/changes/chaoscontrol-primitives/evidence/repo-sync.md` against pinned pre-change baseline `acddde9851a3` (or the rebased equivalent) so current working-tree `docs/requirements.md` `chaoscontrol-*` anchors are not mistaken for shipped overlap
- [x] Confirm or add `pub mod properties;` in `src/lib.rs` before module work depends on the new cross-module proof lemmas in `src/properties.rs`
- [x] For each new `src/chaoscontrol_*.rs` file, add the matching `pub mod chaoscontrol_*;` line to `src/lib.rs` together with that module's first implementation task; keep helper-only functions private unless they intentionally carry public API / Tracey surface

## Phase 1: Coverage and Oracle

- [x] `src/chaoscontrol_coverage.rs`: land public `new_coverage_bitmap(...)` / `valid_coverage_bitmap(...)` and add `pub mod chaoscontrol_coverage;` to `src/lib.rs` with that first implementation (`r[chaoscontrol-coverage.regions]`)
- [x] `src/chaoscontrol_coverage.rs`: prove exact region cut points and require valid coverage for public region helpers (`r[chaoscontrol-coverage.regions]`)
- [x] `src/chaoscontrol_coverage.rs`: prove region-writer non-interference across code/assertion/schedule regions (`r[chaoscontrol-coverage.regions]`)
- [x] `src/chaoscontrol_coverage.rs`: pointwise-max merge, exact `u32` new-edge count, and zero/nonzero count facts (`r[chaoscontrol-coverage.union]`)
- [x] `src/chaoscontrol_coverage.rs`: exact AFL bucket map `0,1,2,3,4..7,8..15,16..31,32..127,128..255` with named proof artifacts for region/merge/classify soundness (`r[chaoscontrol-coverage.classify]`)
- [x] Coverage unit/property tests plus explicit code-side `r[impl]` / `r[verify]` anchors in `src/chaoscontrol_coverage.rs` and explicit `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `chaoscontrol-coverage.*`
- [x] `src/chaoscontrol_oracle.rs`: land public constructors `new_boolean_record(...)`, `new_reachable_record(...)`, `new_unreachable_record(...)`, validator `valid_oracle_record(...)`, and add `pub mod chaoscontrol_oracle;` to `src/lib.rs` with that first implementation (`r[chaoscontrol-oracle.accounting]`)
- [x] `src/chaoscontrol_oracle.rs`: shared validity helpers prove `runs_satisfied <= runs_hit` and `runs_hit == 0 <=> hit_count == 0` (`r[chaoscontrol-oracle.accounting]`)
- [x] `src/chaoscontrol_oracle.rs`: prove exact boolean and non-boolean record shapes on that public validity surface (`r[chaoscontrol-oracle.accounting]`)
- [x] `src/chaoscontrol_oracle.rs`: verdict helpers are total over valid inputs and implement exact pass/fail complements (`r[chaoscontrol-oracle.verdict]`)
- [x] `src/chaoscontrol_oracle.rs`: invalid boolean and non-boolean record shapes stay outside the verdict contract (`r[chaoscontrol-oracle.verdict]`)
- [x] Named oracle `proof fn` artifacts for accounting and verdict totality/exclusivity, plus explicit code-side `r[impl]` / `r[verify]` anchors in `src/chaoscontrol_oracle.rs`, oracle unit/property tests, and explicit `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `chaoscontrol-oracle.*`

## Phase 2: Scheduler and Fault Schedule

- [x] `src/chaoscontrol_scheduler.rs`: land `new_scheduler_state(...)`, enforce constructor canonicalization and randomized-state validity, and add `pub mod chaoscontrol_scheduler;` to `src/lib.rs` with that first implementation (`r[chaoscontrol-scheduler.turns]`)
- [x] `src/chaoscontrol_scheduler.rs`: prove `apply_variant` preconditions for invalid randomized overrides plus constructor / `apply_variant` RNG-consumption neutrality (`r[chaoscontrol-scheduler.turns]`)
- [x] `src/chaoscontrol_scheduler.rs`: single-vCPU no-op rules, SMP `tick` positive-to-zero switch rule, explicit `tick == true -> next transition is advance` contract, round-robin `advance`, and exact randomized quantum assignment from caller `rng_word` (`r[chaoscontrol-scheduler.turns]`)
- [x] `src/chaoscontrol_scheduler.rs`: exact fingerprint mix, tick neutrality, `apply_variant` persistence / `None`-keeps-current semantics, reset-to-zero epoch, and post-reset lower-inclusive / upper-exclusive randomized range behavior (`r[chaoscontrol-scheduler.fingerprint]`)
- [x] Add explicit scheduler proof artifacts in `src/chaoscontrol_scheduler.rs` for turn semantics and fingerprint semantics, with code-side `r[impl]` / `r[verify]` anchors for `chaoscontrol-scheduler.turns` and `chaoscontrol-scheduler.fingerprint`
- [x] Scheduler unit/property tests plus explicit `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `chaoscontrol-scheduler.*`
- [x] `src/chaoscontrol_fault_schedule.rs`: land public constructors `empty_fault_schedule(...)` and `new_fault_schedule(...)`, and add `pub mod chaoscontrol_fault_schedule;` to `src/lib.rs` with that first implementation (`r[chaoscontrol-fault-schedule.replay]`)
- [x] `src/chaoscontrol_fault_schedule.rs`: prove normalized schedule validity plus public `add` precondition `cursor = 0` (`r[chaoscontrol-fault-schedule.replay]`)
- [x] `src/chaoscontrol_fault_schedule.rs`: prove `add` fresh-ordinal allocation `0` else `1 + max(existing)` and stable `(time_ns, ordinal)` normalization with result `cursor = 0` (`r[chaoscontrol-fault-schedule.replay]`)
- [x] `src/chaoscontrol_fault_schedule.rs`: exact `drain_due(t)` selection, cursor monotonicity / bounds, preserved undrained suffix, and non-re-delivery (`r[chaoscontrol-fault-schedule.replay]`)
- [x] Add explicit replay/cursor `proof fn` artifacts in `src/chaoscontrol_fault_schedule.rs` for `r[chaoscontrol-fault-schedule.replay]`, with code-side `r[verify]` anchors
- [x] `src/chaoscontrol_fault_schedule.rs`: subset first-occurrence selection over the full normalized source schedule, out-of-bounds ignore, carried source-index ordinals, `cursor = 0`, and re-normalization (`r[chaoscontrol-fault-schedule.subset]`)
- [x] Add explicit subset-order `proof fn` artifacts in `src/chaoscontrol_fault_schedule.rs` for `r[chaoscontrol-fault-schedule.subset]`, with code-side `r[verify]` anchors
- [x] Fault-schedule unit/property tests plus explicit code-side `r[impl]` / `r[verify]` anchors in `src/chaoscontrol_fault_schedule.rs` and explicit `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `chaoscontrol-fault-schedule.*`
- [x] `src/properties.rs`: `proof fn chaoscontrol_subset_replay_order(...)` defining artifact with explicit code-side `r[impl]` / `r[verify]` anchors and tests for `r[properties.chaoscontrol-subset-replay]`
- [x] `src/properties.rs`: `proof fn chaoscontrol_fingerprint_trace_eq(...)` defining artifact with explicit code-side `r[impl]` / `r[verify]` anchors and tests for `r[properties.chaoscontrol-fingerprint-trace]`
- [x] `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `properties.chaoscontrol-subset-replay` and `properties.chaoscontrol-fingerprint-trace`

## Phase 3: Heuristics and Fault State

- [x] `src/chaoscontrol_frontier.rs`: non-empty frontier validity, exact rational score `base_score / (1 + times_selected)`, verified no-overflow `u64` cross-product comparator, and `src/lib.rs` gains `pub mod chaoscontrol_frontier;` with that first implementation (`r[chaoscontrol-frontier.selection]`)
- [x] `src/chaoscontrol_frontier.rs`: greedy maximality, monotone decay under re-selection, and lowest-input-index tie-break proofs (`r[chaoscontrol-frontier.selection]`)
- [x] Add named frontier `proof fn` artifacts in `src/chaoscontrol_frontier.rs` for effective-score comparison, decay, maximality, and tie-break behavior, with code-side `r[verify]` anchors for `r[chaoscontrol-frontier.selection]`
- [x] Frontier unit/property tests plus explicit code-side `r[impl]` / `r[verify]` anchors in `src/chaoscontrol_frontier.rs` and explicit `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `chaoscontrol-frontier.selection`
- [x] `src/chaoscontrol_input_tree.rs`: land public validator `valid_choice_batch(...)`, normalizer `normalize_choice_batch(...)`, and add `pub mod chaoscontrol_input_tree;` to `src/lib.rs` with that first implementation (`r[chaoscontrol-input-tree.alternatives]`)
- [x] `src/chaoscontrol_input_tree.rs`: prove normalized choice-record validity, arithmetic bounds `sequence_id <= u64::MAX - 100` plus `n_options as u64 * (100 + sequence_id) <= u64::MAX`, and unique `(vm_id, sequence_id)` pairs within each normalized input batch (`r[chaoscontrol-input-tree.alternatives]`)
- [x] `src/chaoscontrol_input_tree.rs`: exact exec comparator where smaller denominator `n_options * (100 + sequence_id)` wins because the numerator is constant, with proof that exec order matches the mathematical score order (`r[chaoscontrol-input-tree.alternatives]`)
- [x] `src/chaoscontrol_input_tree.rs`: lexicographic `(vm_id, sequence_id)` tie-breaks with `vm_id` first and `sequence_id` second, and emitted output identity stays `(vm_id, sequence_id, alternative_value)` (`r[chaoscontrol-input-tree.alternatives]`)
- [x] `src/chaoscontrol_input_tree.rs`: emit ascending distinct alternatives excluding the original value and truncate to the exact prefix `min(budget, full_stream_len)` (`r[chaoscontrol-input-tree.alternatives]`)
- [x] Add named input-tree `proof fn` artifacts in `src/chaoscontrol_input_tree.rs` for comparator equivalence, tie-break correctness, identity preservation, and exact budget-prefix behavior, with code-side `r[verify]` anchors for `r[chaoscontrol-input-tree.alternatives]`
- [x] Input-tree unit/property tests plus explicit code-side `r[impl]` / `r[verify]` anchors in `src/chaoscontrol_input_tree.rs` and explicit `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `chaoscontrol-input-tree.alternatives`
- [x] `src/chaoscontrol_fault_state.rs`: land public constructor `new_fault_state(...)`, validator `valid_fault_state(...)`, and add `pub mod chaoscontrol_fault_state;` to `src/lib.rs` with that first implementation (`r[chaoscontrol-fault-state.quorum-health]`)
- [x] `src/chaoscontrol_fault_state.rs`: prove `2 <= num_nodes <= 64` validity, exact `num_nodes x num_nodes` shape, diagonal-false matrix, exact alive-mask bit encoding, and cleared high bits in `alive_mask` (`r[chaoscontrol-fault-state.quorum-health]`)
- [x] `src/chaoscontrol_fault_state.rs`: prove exact directed non-diagonal `partition_count` semantics (`r[chaoscontrol-fault-state.quorum-health]`)
- [x] `src/chaoscontrol_fault_state.rs`: majority threshold failure, exact bidirectional-pair `quorum_reachable` heuristic, and healthy gate `alive_count == num_nodes && quorum_reachable` (`r[chaoscontrol-fault-state.quorum-health]`, `r[chaoscontrol-fault-state.liveness-gate]`)
- [x] Add explicit fault-state proof artifacts in `src/chaoscontrol_fault_state.rs` for quorum-health and liveness-gate semantics, with code-side `r[impl]` / `r[verify]` anchors for `chaoscontrol-fault-state.*`
- [x] Fault-state unit/property tests plus explicit `docs/requirements.md` `r[impl]` / `r[verify]` anchors for `chaoscontrol-fault-state.*`
- [x] `src/properties.rs`: `proof fn chaoscontrol_quorum_gate_sound(...)` defining artifact with explicit code-side `r[impl]` / `r[verify]` anchors, tests, and `docs/requirements.md` anchors for `r[properties.chaoscontrol-quorum-gate]`

## Phase 4: Integration and Validation

- [x] `src/chaoscontrol_scheduler.rs` and `src/chaoscontrol_fault_schedule.rs`: keep intra-module exec helper calls bare-local only; no `self::...` or module-qualified aliases for local helper calls (`r[chaoscontrol-validation.iterative-exec]`)
- [x] Add or verify `scripts/check-iterative-exec.py` so it implements the specified deterministic audit semantics for bare local exec-call cycle detection used by `r[chaoscontrol-validation.iterative-exec]`
- [x] Run `python3 scripts/check-iterative-exec.py src/chaoscontrol_scheduler.rs src/chaoscontrol_fault_schedule.rs`: collect non-`spec fn` / non-`proof fn`, require intra-module exec helper calls to use bare local function names, build the exec call graph by exact bare local function-name calls, fail on any self-edge or cycle, and record the result in `openspec/changes/chaoscontrol-primitives/evidence/iterative-exec-audit.md` (`r[chaoscontrol-validation.iterative-exec]`)
- [x] Run `./scripts/check-ensures.sh` and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-check-ensures.txt`
- [x] Run `cargo test` and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-cargo-test.txt`
- [x] Run `cargo clippy --all-targets -- -D warnings` and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-clippy.txt`
- [x] Run `./scripts/verus.sh src/lib.rs` and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-verus.txt`
- [x] Run `tracey query uncovered`, require empty output or write reviewer-approved disposition in `openspec/changes/chaoscontrol-primitives/evidence/tracey-disposition.md`, and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-tracey-uncovered.txt`
- [x] Run `tracey query untested`, require empty output or write reviewer-approved disposition in `openspec/changes/chaoscontrol-primitives/evidence/tracey-disposition.md`, and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-tracey-untested.txt`
- [x] Run `./scripts/verify.sh` and capture output in `openspec/changes/chaoscontrol-primitives/evidence/validation-verify.txt`
