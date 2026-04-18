## Context

This change ports only ChaosControl's pure value-level kernels into this repo's
normal `spec` / `exec` / `proof` / `test` shape. Scope is coverage math,
oracle verdicts, scheduler and fault-schedule kernels, exploration heuristics,
and fault-state predicates. Host timing, KVM/device code, and explorer shell
logic stay out of scope.

## Goals / Non-Goals

**Goals**
- Preserve exact upstream kernel semantics where they define replay or search behavior.
- Reuse existing generic lemmas when they already match the needed math.
- Land concrete `r[impl]` and `r[verify]` anchors for every new requirement.
- Keep exec code iterative in hot scheduler and fault-schedule paths.

**Non-Goals**
- KVM, VM, device, or wall-clock orchestration.
- Probabilistic policy-quality claims.
- End-to-end explorer verification.
- Stronger graph-theoretic liveness claims than the current ChaosControl helpers.

## Decisions

### 1. Use explicit ChaosControl-prefixed modules and split delta specs by domain

**Choice:** land `src/chaoscontrol_coverage.rs`, `chaoscontrol_oracle.rs`,
`chaoscontrol_scheduler.rs`, `chaoscontrol_fault_schedule.rs`,
`chaoscontrol_frontier.rs`, `chaoscontrol_input_tree.rs`, and
`chaoscontrol_fault_state.rs`, with cross-module lemmas in `src/properties.rs`.
Delta specs stay split by domain so each requirement family fits strict gate
review without per-file truncation.

**Rationale:** provenance stays obvious, names do not collide with existing
`bitmap` / `quorum`-style helpers, and stage gates can inspect complete specs.

### 1a. Fix one public validity surface per domain before implementation

**Choice:** public validity/API boundaries are fixed per module:
- `chaoscontrol_coverage`: constructor `new_coverage_bitmap(...)` establishes fixed `65536`-byte shape; validator `valid_coverage_bitmap(...)` and region helpers require it
- `chaoscontrol_oracle`: constructors `new_boolean_record(...)`, `new_reachable_record(...)`, and `new_unreachable_record(...)`, plus validator `valid_oracle_record(...)`, establish valid records; verdict/accounting helpers require them
- `chaoscontrol_scheduler`: constructor `new_scheduler_state(...)` and updater `apply_variant(...)` establish valid scheduler state; `tick(...)` / `advance(...)` require it
- `chaoscontrol_fault_schedule`: constructors `empty_fault_schedule(...)` and `new_fault_schedule(...)`, plus `add(...)` / `subset(...)`, produce normalized schedules; replay helpers require them; `add(...)` additionally requires `cursor = 0`
- `chaoscontrol_frontier`: public greedy selection helper `select_frontier_greedy(...)` is `requires`-guarded over non-empty stable-order inputs; no separate normalization constructor is promised
- `chaoscontrol_input_tree`: validator `valid_choice_batch(...)` and normalizer `normalize_choice_batch(...)` establish normalized batches; ranking/emission helpers require them
- `chaoscontrol_fault_state`: constructor `new_fault_state(...)` and validator `valid_fault_state(...)` establish valid states; quorum/healthy helpers require them

**Rationale:** review wanted the constructor-vs-`requires` boundary chosen before task decomposition so proof obligations and public API surface do not drift. The semantic contract is the validity boundary itself; the concrete helper names below are intended implementation spellings for reviewability, not separate delta-spec requirements unless later promoted into specs.

### 1b. Fix crate export policy before implementation

**Choice:** each new domain module (`chaoscontrol_coverage`, `chaoscontrol_oracle`,
`chaoscontrol_scheduler`, `chaoscontrol_fault_schedule`,
`chaoscontrol_frontier`, `chaoscontrol_input_tree`, and
`chaoscontrol_fault_state`) will be exported from `src/lib.rs` via `pub mod`.
Helper-only functions inside those modules remain private unless they carry an
intentional Tracey or public API surface. `src/properties.rs` stays in the
public crate graph as `pub mod properties;`, but the new ChaosControl additions
there are proof-only lemmas rather than a separate exec API.

**Rationale:** proposal scope already names an intended implementation surface, so the
`src/lib.rs` boundary should be fixed before implementation starts. This export plan is an implementation/tasking decision, not a separate semantic requirement beyond the validity surfaces and proof obligations already captured in the delta specs.

### 2. Pin coverage to the exact 64 KiB upstream model

**Choice:** model coverage as a fixed `65536`-byte `Vec<u8>` / `Seq<u8>` with
three exact regions: code `[0, 32768)`, assertion `[32768, 49152)`, and
schedule `[49152, 65536)`. Merge is exact pointwise `max`. New-edge count is
the exact `u32` count of `branch[i] > 0 && global[i] == 0`. Classification
uses the exact AFL intervals
`0 -> 0`, `1 -> 1`, `2 -> 2`, `3 -> 3`, `4..7 -> 4`, `8..15 -> 8`,
`16..31 -> 16`, `32..127 -> 32`, `128..255 -> 128`. Writers into one region
must prove non-interference with the other two.

**Rationale:** these are already stable ChaosControl semantics; weakening them
would reduce value and create review drift.

### 3. Make oracle verdicts total over exact valid aggregate records

**Choice:** preserve the four upstream verdict rules exactly: `always` passes
iff hit and never false; `sometimes` passes iff hit and at least one true;
`reachable` passes iff hit; `unreachable` passes iff never hit. The shared
valid-record surface includes `runs_satisfied <= runs_hit` and
`runs_hit == 0 <=> hit_count == 0`. Boolean records also satisfy
`hit_count == true_count + false_count`, `runs_hit <= hit_count`, and for
`sometimes` additionally `runs_satisfied <= true_count`. Non-boolean records
are exact: `reachable` requires `true_count == hit_count`, `false_count == 0`,
`runs_satisfied == runs_hit`; `unreachable` requires `true_count == 0`,
`false_count == hit_count`, `runs_satisfied == 0`. Verdict helpers are total
only over those valid inputs; invalid boolean or non-boolean shapes stay
outside contract.

**Rationale:** review wanted exact non-boolean validity, not a loose “depends on
hit_count only” contract.

### 4. Model scheduler state with exact turn, quantum, and variant rules

**Choice:** verified scheduler state is
`(num_vcpus, active, remaining, default_quantum, strategy, fingerprint)`.
Stored `default_quantum` is always `max(1, raw_default_quantum)` and is a
caller-provided scalar independent of strategy: it is accepted regardless of
whether the strategy is round-robin or randomized. Round-robin advance reloads
`remaining` from the stored `default_quantum`; randomized advance ignores
`default_quantum` and derives the reload from `rng_word` and the strategy's
`[min_quantum, max_quantum)` range. Valid randomized state requires
`1 <= min_quantum <= max_quantum`; invalid randomized construction or override
is a `requires` failure outside contract. Initial state is
`active = 0`, `remaining = default_quantum`, `fingerprint = 0`.
Single-vCPU `tick` and `advance` are no-ops and `tick` returns `false`.
SMP `tick` requires `remaining > 0`, decrements by one, and returns `true`
exactly on the positive-to-zero transition; after `true`, the next legal
transition is `advance`. SMP `advance` is otherwise still defined on any valid
state: if called early, it ends the current turn immediately and performs the
same next-vCPU / quantum-reload transition and fingerprint update. `advance`
moves to `(active + 1) % num_vcpus` and reloads `remaining` to the assigned
quantum. Round-robin uses
`default_quantum`. Randomized advance consumes a caller `rng_word: u64` and
uses the existing ChaosControl lower-inclusive, upper-exclusive rule: when
`min_quantum < max_quantum`, assigned quantum is
`min_quantum + (rng_word mod (max_quantum - min_quantum))`, so results lie in
`[min_quantum, max_quantum)` and `max_quantum` itself is not reached; when
`min_quantum == max_quantum`, assigned quantum is exactly `min_quantum`.
Constructor and `apply_variant` consume no RNG word.

**Rationale:** reproducibility depends on these exact kernel rules.

### 5. Keep fingerprint semantics separate and exact

**Choice:** fingerprint update is the exact wrapping-`u64` rule
`fp' = fp.wrapping_mul(0x517c_c1b7_2722_0a95).bitxor(active as u64).bitxor(assigned_quantum as u64)`,
where `active` is the post-advance vCPU index and `assigned_quantum` is the
reload value chosen by that advance.
`tick` is fingerprint-neutral. Each SMP `advance` performs exactly one mix on
the post-advance `(active, assigned_quantum)` pair. `ScheduleVariant` is
`(strategy_override, quantum_override)`. `apply_variant` persists any present
`strategy_override`; when `quantum_override` is present, it updates stored
`default_quantum` to `max(1, raw_quantum_override)`, and when it is `None` the
stored `default_quantum` is unchanged. It then resets
`active = 0`, `remaining = default_quantum`, and `fingerprint = 0`. If the
persisted strategy after reset is randomized, later randomized advances still
use that strategy's lower-inclusive, upper-exclusive range
`[min_quantum, max_quantum)` when `min_quantum < max_quantum`, and exact
`min_quantum` when `min_quantum == max_quantum`; `quantum_override` changes
only the stored/reset scalar quantum.

**Rationale:** this is the stable replay contract for schedule fingerprints.

### 6. Make fault schedules exact normalized streams

**Choice:** a normalized schedule has unique ordinals, stable `(time_ns, ordinal)`
order, and `0 <= cursor <= len`. The public `add` helper is defined only on
undrained normalized schedules with `cursor = 0`; adding to partially drained
schedules stays outside the verified contract. On that input surface, `add`
allocates fresh ordinals as `0` for the first fault and otherwise
`1 + max(existing ordinals)`, including schedules produced by subset
extraction, then renormalizes by `(time_ns, ordinal)` and keeps result
`cursor = 0`. `drain_due(t)` returns exactly the not-yet-drained faults with
`time_ns <= t` starting at `cursor`, in normalized order, advances `cursor` by
exactly the number returned, preserves the undrained suffix, and never
re-delivers drained faults. `subset(indices)` ranges over the full normalized
source schedule, clones the first occurrence of each in-bounds source index,
ignores out-of-bounds indices and later duplicates, carries the source schedule
index as the result ordinal, resets result `cursor = 0`, and renormalizes by
`(time_ns, ordinal)`.

**Rationale:** replay and minimization both depend on this exact stream model.

### 7. Use exact rational ordering for frontier selection

**Choice:** frontier entries use `u32` `base_score` and `times_selected`.
Effective score is the rational `base_score / (1u64 + times_selected as u64)`.
The greedy helper is defined only for non-empty frontiers. Exec comparison uses
exact `u64` cross-products equivalent to the rational order; this is
overflow-safe because both multiplicands come from `u32`-bounded fields, so the
products stay below `u64::MAX`. Re-selection is monotone non-increasing,
greedy selection returns a maximal entry, and equal maximal scores break to the
lowest input index.

**Rationale:** proof needs exact ordering, not floating-point drift.

### 8. Keep input-tree generation deterministic at the primitive boundary

**Choice:** input-tree kernels operate on normalized tuples
`(vm_id: u32, sequence_id: u64, n_options: u32, original_value: u64)` with
`n_options >= 2`, `original_value < n_options`,
`sequence_id <= u64::MAX - 100`,
`n_options as u64 * (100 + sequence_id) <= u64::MAX`, and unique
`(vm_id, sequence_id)` pairs per batch. Ordering uses exact rational score
`100 / (n_options * (100 + sequence_id))`, with higher scores first. Because
that numerator is constant, exec ordering must compare the denominators
`n_options * (100 + sequence_id)` by verified no-overflow `u64` arithmetic;
smaller denominator wins, and the exec result must match the mathematical
ordering exactly. Equal scores break lexicographically by lower
`(vm_id, sequence_id)`, comparing `vm_id` first and `sequence_id` second. Each
choice contributes ascending distinct alternatives from `0 .. n_options`
excluding the original value. Output is the exact prefix of length
`min(budget, full_stream_len)`.

**Rationale:** gate review demanded exact order direction, exact exec
comparator, and exact budget truncation.

### 9. Keep fault-state health at the current pair-connected heuristic

**Choice:** fault-state helpers require `2 <= num_nodes <= 64`, an exact
`num_nodes x num_nodes` directed partition matrix, diagonal-false entries, and
cleared high bits in `alive_mask`. `alive_mask` sets bit `i` iff participant
`i` is alive. `partition_count` counts directed blocked non-diagonal links.
`quorum_reachable` is false when `alive_count < majority(n)` and otherwise true
exactly when some distinct alive pair has direct bidirectional connectivity.
The healthy-period gate is exactly
`alive_count == num_nodes && quorum_reachable(state)`.

**Rationale:** this matches the current ChaosControl helper; stronger liveness
claims stay out of scope.

### 10. Reuse existing lemmas and pin real Tracey anchors

**Choice:** reuse `bitmap` / `bitwise`, `quorum`, `monotonic`, and `seq_set`
where they fit. Every new requirement gets a concrete defining `r[impl]`
anchor. Every proof-style requirement, especially each `properties.*` rule,
also gets an explicit `r[verify]` proof artifact in code. Named cross-module
proofs land in `src/properties.rs`:
`chaoscontrol_subset_replay_order`, `chaoscontrol_fingerprint_trace_eq`, and
`chaoscontrol_quorum_gate_sound`.

**Upstream provenance map (pinned at `../chaoscontrol` revision `fa5123f04b017d5d3e260215eb71026e79033b3a`):**
- coverage -> `../chaoscontrol/crates/chaoscontrol-explore/src/coverage.rs`
- oracle/accounting aggregation -> `../chaoscontrol/crates/chaoscontrol-vmm/src/controller.rs` and `../chaoscontrol/crates/chaoscontrol-explore/src/report.rs`
- scheduler -> `../chaoscontrol/crates/chaoscontrol-vmm/src/scheduler.rs`
- fault schedule -> `../chaoscontrol/crates/chaoscontrol-fault/src/schedule.rs`
- frontier -> `../chaoscontrol/crates/chaoscontrol-explore/src/frontier.rs`
- input tree -> `../chaoscontrol/crates/chaoscontrol-explore/src/input_tree.rs`
- fault state / healthy-period heuristic -> `../chaoscontrol/crates/chaoscontrol-raft-guest/src/main.rs`

**Traceability matrix:**
- `r[chaoscontrol-coverage.regions]` -> `docs/requirements.md` + `src/chaoscontrol_coverage.rs` constructors/region helpers + named region/non-interference proofs
- `r[chaoscontrol-coverage.union]` -> `docs/requirements.md` + `src/chaoscontrol_coverage.rs` merge/new-edge helpers + merge/count proofs
- `r[chaoscontrol-coverage.classify]` -> `docs/requirements.md` + `src/chaoscontrol_coverage.rs` bucket helpers + classify proofs
- `r[chaoscontrol-oracle.verdict]` -> `docs/requirements.md` + `src/chaoscontrol_oracle.rs` verdict helpers + verdict totality/exclusivity proofs
- `r[chaoscontrol-oracle.accounting]` -> `docs/requirements.md` + `src/chaoscontrol_oracle.rs` validity/accounting helpers + accounting proofs
- `r[chaoscontrol-scheduler.turns]` -> `docs/requirements.md` + `src/chaoscontrol_scheduler.rs` constructor/tick/advance helpers + scheduler turn proofs
- `r[chaoscontrol-scheduler.fingerprint]` -> `docs/requirements.md` + `src/chaoscontrol_scheduler.rs` fingerprint/apply-variant helpers + fingerprint proofs
- `r[chaoscontrol-fault-schedule.replay]` -> `docs/requirements.md` + `src/chaoscontrol_fault_schedule.rs` add/drain/cursor helpers + replay/cursor proofs
- `r[chaoscontrol-fault-schedule.subset]` -> `docs/requirements.md` + `src/chaoscontrol_fault_schedule.rs` subset helpers + subset-order proofs
- `r[chaoscontrol-frontier.selection]` -> `docs/requirements.md` + `src/chaoscontrol_frontier.rs` score/select helpers + decay/tie-break proofs
- `r[chaoscontrol-input-tree.alternatives]` -> `docs/requirements.md` + `src/chaoscontrol_input_tree.rs` ranking/emission helpers + ordering/budget proofs
- `r[chaoscontrol-fault-state.quorum-health]` -> `docs/requirements.md` + `src/chaoscontrol_fault_state.rs` validity/alive-mask/quorum helpers + health proofs
- `r[chaoscontrol-fault-state.liveness-gate]` -> `docs/requirements.md` + `src/chaoscontrol_fault_state.rs` healthy-period helper + gate proofs
- `r[properties.chaoscontrol-subset-replay]` -> `docs/requirements.md` + `src/properties.rs::chaoscontrol_subset_replay_order` (`r[impl]`) + named `proof fn` artifact in code (`r[verify]`) + supplemental tests; proof hypothesis is exactly: replay `subset(source, indices)` from `cursor = 0` by any nondecreasing time sequence whose final drain time `t_final >= max(time_ns)` over all subset faults
- `r[properties.chaoscontrol-fingerprint-trace]` -> `docs/requirements.md` + `src/properties.rs::chaoscontrol_fingerprint_trace_eq` (`r[impl]`) + named `proof fn` artifact in code (`r[verify]`) + supplemental tests; proof hypothesis is exactly: same full scheduler state, or same full post-`apply_variant` reset state, plus the same post-advance `(active, assigned_quantum)` trace
- `r[properties.chaoscontrol-quorum-gate]` -> `docs/requirements.md` + `src/properties.rs::chaoscontrol_quorum_gate_sound` (`r[impl]`) + named `proof fn` artifact in code (`r[verify]`) + supplemental tests; proof hypothesis is exactly: valid fault state implies `healthy_period ==> quorum_reachable`
- `r[chaoscontrol-validation.iterative-exec]` -> `specs/chaoscontrol-validation/spec.md` + `openspec/changes/chaoscontrol-primitives/evidence/iterative-exec-audit.md` + deterministic audit task / transcript (change-local validation rule; not mirrored into `docs/requirements.md`)

**Rationale:** this repo repeatedly hits Tracey drift when proof-only rules lack
concrete `r[impl]` anchors.

### 11. Make the iterative-exec audit deterministic

**Choice:** `src/chaoscontrol_scheduler.rs` and
`src/chaoscontrol_fault_schedule.rs` must keep exec code acyclic. This design
choice is also tracked by `r[chaoscontrol-validation.iterative-exec]`. The audit
runs as concrete command
`python3 scripts/check-iterative-exec.py src/chaoscontrol_scheduler.rs src/chaoscontrol_fault_schedule.rs`,
records evidence in `openspec/changes/chaoscontrol-primitives/evidence/iterative-exec-audit.md`,
and applies one rule: scan those two files, collect functions that are not
`spec fn` or `proof fn`, build the module-local call graph among those exec
functions by exact bare local function-name calls, and fail on any self-edge or
call-graph cycle of length greater than one. To keep that deterministic scan
sound, intra-module exec helper calls in those two files must use bare local
function names rather than qualified/module-prefixed aliases. Spec/proof
recursion and standard library method calls are exempt.

**Rationale:** review wanted a reproducible rule that catches direct and mutual
exec recursion, not just ad hoc eyeballing.

## Migration Plan

1. Keep repo-sync evidence current in `openspec/changes/chaoscontrol-primitives/evidence/repo-sync.md` and reuse existing generic helpers where they already match. Current pinned baseline is `acddde9851a3`; that audit found no shipped ChaosControl-specific module/spec overlap and treats current working-tree `properties.chaoscontrol-*` anchors in `docs/requirements.md` as change-local additions rather than shipped surfaces.
2. Land coverage and oracle modules first, including tests and `docs/requirements.md` anchors.
3. Land scheduler and fault-schedule modules next, preserving exact replay and fingerprint rules.
4. Land frontier, input-tree, and fault-state modules after lower-level kernels are stable.
5. Land cross-module proofs in `src/properties.rs` with matching `r[impl]` / `r[verify]` anchors.
6. Run the deterministic iterative-exec audit and record `evidence/iterative-exec-audit.md`.
7. Validate with `./scripts/check-ensures.sh`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, exact Tracey commands `tracey query uncovered` and `tracey query untested`, and `./scripts/verify.sh`.
8. Treat non-empty output from `tracey query uncovered` or `tracey query untested` as failure unless reviewer-approved disposition is recorded in `evidence/tracey-disposition.md`.
