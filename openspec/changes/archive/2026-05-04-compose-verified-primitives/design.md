## Context

The repository already has verified modules for the primitive facts needed by
this suite: jobs and durable replay, Raft, storage, database isolation, UCAN and
web-of-trust, concurrency, compiler passes, scheduling, accounting, seL4, Nix,
Nickel, and ChaosControl. Existing `properties.rs`, `concurrent_composition.rs`,
and `consensus_path.rs` show that cross-module proof composition is an accepted
repo pattern.

This change turns that pattern into public, named composition primitives. Each
new module should be small: pure spec predicates define the composition; exec
helpers orchestrate already-verified primitives; proof functions connect the
facade result to the underlying module contracts.

## Goals / Non-Goals

**Goals:**
- define all twenty requested composite primitives in one reviewable OpenSpec
  change
- keep every composite explicit about its underlying modules and input-validity
  boundary
- expose facade APIs only where the underlying primitives already support an
  exec-facing model
- add positive and negative tests for each facade
- add Verus proofs that the facade safety property follows from the composed
  primitives
- keep Tracey coverage at `0 uncovered` and `0 untested`

**Non-Goals:**
- replace or rewrite the underlying primitive modules
- prove new distributed, storage, compiler, or kernel algorithms unrelated to
  composition
- hide unverified assumptions behind public functions
- archive partial implementation slices as if the whole suite were complete

## Shared Facade Result Design

Each module may use a domain-specific public result enum, but it MUST expose a
pure mapping to the common outcome model:

- `Accept(value)`: admitted operation. The module's `<module>_accepted_spec(...)`
  predicate MUST hold.
- `Reject(reason)`: denied operation. The module's rejection spec MUST prove
  every mutable input state is unchanged, unless the requirement explicitly says
  the operation is read-only.
- `Wait(wait_state)`: valid blocking transition. Only `blocking_bounded_channel`
  may use `Wait` in this suite unless a later spec modification adds another
  explicit wait-capable requirement. `Wait` must satisfy
  `blocking_bounded_channel_wait_valid_spec(...)`.

Required naming per module:

- `<module>_accepted_spec(...)`: pure predicate for the accepted result, source
  facts, and at least one cross-module relation tying two or more source modules
  to the same token, index, resource, object, trace, graph, closure item, or
  state transition.
- `<module>_reject_preserves_spec(...)`: pure predicate proving every mutable
  input state is unchanged on reject, unless the operation is read-only.
- `<module>_refines_sources(...)`: proof that exec results map into
  `Accept`/`Reject`/`Wait` semantics, that `Accept` implies the exported
  safety theorem, and that `Wait` implies
  `blocking_bounded_channel_wait_valid_spec(...)` when present.

Domain-specific names such as allow/deny, install/reject, commit/abort,
schedule/not-selected, or reclaim/retain are presentation aliases only. The
implementation tasks must document their mapping to the common outcome model in
each module before proof work is checked off.

## Source Symbol Classification

In the change-local spec, `_spec` names are spec predicates/functions. The
following non-`_spec` names are intentionally classified so implementation tasks
can connect exec calls and proof obligations deterministically:

| Symbol | Kind | Use |
|---|---|---|
| `validate_envelope`, `encode_did_key` | exec API | UCAN/DID runtime shell inputs; proof obligations use paired spec facts |
| `topo_sort`, `has_cycle` | exec API | dependency-scheduler runtime graph checks; proofs use `is_topo_order_spec` / `has_cycle_spec` |
| `tick`, `apply_variant`, `drain_due`, `select_frontier_greedy` | exec API | ChaosControl runtime exploration shell inputs; proofs use listed `*_spec` predicates |
| `joint_quorum_intersection`, `dce_preserves_observable_trace` | proof/theorem | consumed directly as proof dependencies |
| `quorum_intersection_prop` | spec predicate | quorum-intersection condition used by read-gate proof |
| `wf` | spec predicate method | seL4 CDT/VSpace/Endpoint well-formedness predicates |
| `can_enqueue_sender`, `can_enqueue_receiver` | spec predicate methods | seL4 endpoint queue-admission predicates |
| `is_sorted_keys`, `well_formed_version`, `keys_unique`, `is_sorted_record`, `disjoint_keys` | spec predicates | Nix/Nickel validity boundaries whose historical names do not end in `_spec` |

Any future non-`_spec` source reference added to this change MUST be added to
this table before the related task can be marked complete.

## Exec-to-Spec Binding Rule

Every public exec facade MUST include a module-local binding table before its
proof task is checked off:

| Facade shell call | Existing source `pub fn` | Paired source spec/ensures | Used by accepted predicate |
|---|---|---|---|

The binding table is mandatory evidence that the facade is thin glue over
existing primitives. If a source fact is spec/proof-only, the table row MUST say
`spec/proof-only` instead of naming a runtime call, and runtime tests are not
required for that subfact.

## Outcome Alias Matrix

| Module | `Accept` aliases | `Reject` aliases | `Wait` aliases |
|---|---|---|---|
| `durable_job_executor` | execute/resume | duplicate, expired, retry-exhausted | none |
| `fenced_distributed_lock` | grant, protected-write allowed | stale, expired, CAS-failed | none |
| `linearizable_read_gate` | safe/read allowed | lost authority, no quorum, expired lease | none |
| `safe_snapshot_replication` | install | corrupt, missing chunk, unsafe compaction | none |
| `crash_safe_page_cache` | recover/apply prefix | torn page, WAL gap, invalid checkpoint | none |
| `verified_lsm_ingest` | ingest/flush | unsafe target, invalid checksum | none |
| `serializable_txn_guard` | commit | abort/conflict/cycle/2PL violation | none |
| `web_of_trust_ucan_authorizer` | authorize/allow | deny/revoked/under-threshold | none |
| `policy_decision_engine` | allow | deny | none |
| `blocking_bounded_channel` | enqueue/dequeue | nonblocking full/closed invalid | park/backoff |
| `safe_reclamation_domain` | reclaim | retain/protected | none |
| `cache_coherent_memory_region` | grant/transition | coherence-invalid | none |
| `verified_compiler_pipeline` | optimize | invalid analysis/unsupported semantics | none |
| `backend_frame_allocator` | allocate | overlap conflict/invalid layout | none |
| `dependency_scheduler` | schedule/select | not-selected/blocked/cyclic/full | none |
| `raft_membership_manager` | apply membership change | unsafe promotion/transfer | none |
| `exactly_once_ledger_transfer` | post transfer | duplicate/replay/insufficient funds | none |
| `sel4_invocation_step` | `CopyCap`, `MapPage`, `EndpointSend`, `BlockSenderOnEndpoint` accepted kernel-state transitions | invalid capability/rights/object or any out-of-scope invocation | none |
| `nix_eval_closure_policy` | admit closure | invalid merge/missing dependency | none |
| `chaoscontrol_exploration_core` | advance path | oracle violation/invalid schedule returned as reject reason without mutating exploration state | none |

## Decisions

### 1. Use one domain spec and many small modules

**Choice:** keep one `composite-primitives` spec domain, but implement separate
modules such as `durable_job_executor.rs`, `fenced_distributed_lock.rs`, and
`linearizable_read_gate.rs`.

**Rationale:** the user asked for one OpenSpec covering the full set. Separate
modules keep proof and review packets small, while one domain gives reviewers a
single checklist for the suite.

**Implementation:** each module owns one requirement ID and may add helper proof
lemmas under that module or in `properties.rs` when cross-domain reuse matters.

### 2. Composition specs are conjunction/refinement surfaces

**Choice:** model each composite as a pure predicate or transition spec that
references the underlying primitive specs, then prove the exec facade refines
that predicate.

**Rationale:** this avoids duplicating existing algorithms. The new value is the
machine-checked glue: if the facade returns `Allow`, `Install`, `Commit`,
`Reclaim`, or `Schedule`, the named underlying facts all hold together.

**Implementation:** use functional-core / imperative-shell structure:
- pure spec/core helpers compute or state the composed decision
- exec wrappers call underlying primitives and return small enums/records
- proof functions bridge exec results to the composed predicate

### 3. Validity boundaries are caller-visible

**Choice:** every composite must state whether invalid input is rejected by a
constructor/checker or is excluded by a `requires` precondition.

**Rationale:** recent repo reviews reject vague validity boundaries. Composite
facades cross module families, so implicit validity assumptions would be hard to
audit.

**Implementation:** tasks require a boundary note per module before proof work is
marked complete.

### 4. Test both allow and reject paths

**Choice:** every exec facade gets at least one positive test and at least one
negative test matching its spec scenarios.

**Rationale:** repo policy requires happy-path and sad-path coverage. For these
facades, negative tests are especially important because most composite value is
fail-closed behavior.

**Implementation:** positive tests exercise valid composition. Negative tests
cover duplicate/stale/expired/corrupt/out-of-bounds/invalid-analysis cases as
appropriate for the primitive.

### 5. Land in dependency-friendly phases

**Choice:** implement the suite in phases that minimize proof coupling:
1. shared facade/result patterns
2. highest-ROI composites: `durable_job_executor`, `fenced_distributed_lock`, `linearizable_read_gate`, `safe_snapshot_replication`, and `exactly_once_ledger_transfer`
3. storage, transaction, authorization, and channel composites: `crash_safe_page_cache`, `verified_lsm_ingest`, `serializable_txn_guard`, `web_of_trust_ucan_authorizer`, `policy_decision_engine`, and `blocking_bounded_channel`
4. reclamation, coherence, compiler/backend/scheduler, and Raft membership composites: `safe_reclamation_domain`, `cache_coherent_memory_region`, `verified_compiler_pipeline`, `backend_frame_allocator`, `dependency_scheduler`, and `raft_membership_manager`
5. seL4, Nix/Nickel, ChaosControl, full docs/spec/Tracey/README sync, validation, and archive evidence

**Rationale:** this lets each phase validate independently while preserving one
final archive boundary for the complete requested suite.

## Composite Map

| Composite | Underlying modules | Primary safety result |
|---|---|---|
| `durable_job_executor` | `job_state`, `job_idempotency`, `durable_history`, `durable_replay`, `durable_resume`, `job_retry`, `job_timeout` | accepted execution is idempotent, resumable, retry-budgeted, and not timed out |
| `fenced_distributed_lock` | `distributed_lock`, `lease_mutex`, `fencing`, `monotonic`, `cas` | accepted writes carry fresh fencing authority |
| `linearizable_read_gate` | `read_index`, `lease_read`, `check_quorum`, `term_authority`, `quorum` | accepted reads have current leader authority and quorum backing |
| `safe_snapshot_replication` | `snapshot_transfer`, `snapshot_integrity`, `install_snapshot`, `log_compaction`, `compaction_fence` | installed snapshots are complete, intact, and compaction-safe |
| `crash_safe_page_cache` | `wal_segment`, `wal_recovery`, `page_checksum`, `block_cache`, `write_policy` | recovery applies only contiguous checksum-valid WAL effects |
| `verified_lsm_ingest` | `write_buffer`, `lsm_level`, `compaction_fence`, `block_cache`, `page_checksum` | ingested runs respect level budget and preserve buffered writes on rejection |
| `serializable_txn_guard` | `mvcc`, `snapshot_isolation`, `txn_dependency`, `two_phase_lock` | commits are conflict-free and serialization-safe |
| `web_of_trust_ucan_authorizer` | `ucan_token`, `ucan_ability`, `ucan_caveat`, `ucan_did`, `ucan_cid`, `web_of_trust_*` | authorization requires token validity plus rooted non-revoked threshold trust |
| `policy_decision_engine` | `authorize`, `delegation`, `containment`, `scope`, `glob`, `pattern`, `trust_level` | allow decisions imply contained delegation, scope match, and sufficient trust |
| `blocking_bounded_channel` | `bounded_mpmc`, `mpsc_bounds`, `futex_state`, `spin_backoff`, `memory_order` | channel occupancy and wait/wake state remain consistent |
| `safe_reclamation_domain` | `rcu`, `epoch_reclaim`, `hazard_pointer`, `ref_count` | reclaimed objects are not protected by readers, hazards, or strong references |
| `cache_coherent_memory_region` | `mesi`, `moesi`, `directory_coherence`, `store_buffer`, `cache_prefetch`, `write_policy` | writable ownership is unique and invisible effects preserve coherence |
| `verified_compiler_pipeline` | `control_flow_graph`, `dominator_tree`, `loop_detection`, `liveness_analysis`, `ssa_construction`, `constant_folding`, `dead_code_elimination` | optimized output preserves the modeled observable behavior |
| `backend_frame_allocator` | `liveness_analysis`, `register_allocation`, `stack_frame_layout` | overlapping live ranges never share conflicting locations |
| `dependency_scheduler` | `job_dag`, `topo_sort`, `cycle_detect`, `deadline_queue`, `fair_share`, `job_slot` | scheduled jobs are dependency-ready and capacity-safe |
| `raft_membership_manager` | `joint_consensus`, `config_change`, `voter_set`, `learner_change`, `learner_readiness`, `leadership_transfer` | membership changes preserve quorum and eligibility invariants |
| `exactly_once_ledger_transfer` | `transfer`, `ledger`, `double_entry`, `journal`, `job_idempotency`, `durable_replay` | replay-safe transfers preserve accounting conservation |
| `sel4_invocation_step` | `sel4_cspace`, `sel4_cap_rights`, `sel4_cdt`, `sel4_vspace`, `sel4_endpoint`, `sel4_tcb`, `sel4_invariants` | `CopyCap`, `MapPage`, `EndpointSend`, and `BlockSenderOnEndpoint` mutate only allowed kernel state and preserve invariants |
| `nix_eval_closure_policy` | `nix_attrset`, `nix_list`, `nix_string`, `nix_version`, `nix_closure`, `nickel_record_merge` | accepted configs include all reachable closure items |
| `chaoscontrol_exploration_core` | `chaoscontrol_scheduler`, `chaoscontrol_fault_state`, `chaoscontrol_fault_schedule`, `chaoscontrol_frontier`, `chaoscontrol_oracle`, `chaoscontrol_coverage` | exploration steps are deterministic, covered, and oracle-checked |

## Evidence and Coverage Matrix

Each row below is required before archive. Runtime tests mean at least one
positive scenario and one negative scenario from the spec. Mixed rows must add
runtime tests only for public exec surfaces and Verus proof evidence for the
spec-only subfacts. Every row also requires the matching `<module>_accepted_spec`
and `<module>_reject_preserves_spec` predicates from the shared result design
before the listed `*_refines_sources` proof can be checked off.

| Requirement ID | Runtime evidence | Proof evidence | Tracey evidence |
|---|---|---|---|
| `composite-primitives.durable-job-executor` | positive resume, negative duplicate/expired | `durable_job_executor_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.fenced-distributed-lock` | positive holder, negative stale holder | `fenced_distributed_lock_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.linearizable-read-gate` | positive quorum read, negative lost authority | `linearizable_read_gate_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.safe-snapshot-replication` | positive install, negative corrupt/unsafe | `safe_snapshot_replication_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.crash-safe-page-cache` | positive replay, invalid-suffix prefix recovery, and invalid-checkpoint rejection | `crash_safe_page_cache_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.verified-lsm-ingest` | positive flush, negative unsafe target | `verified_lsm_ingest_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.serializable-txn-guard` | positive commit, negative conflict/cycle | `serializable_txn_guard_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.web-of-trust-ucan-authorizer` | runtime token/trust allow/deny where public APIs exist | `web_of_trust_ucan_authorizer_refines_sources` for DID/CID spec links | `r[impl]` + `r[verify]` |
| `composite-primitives.policy-decision-engine` | positive allow, negative deny | `policy_decision_engine_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.blocking-bounded-channel` | positive enqueue, negative full/nonblocking and wait path | `blocking_bounded_channel_refines_sources` plus `blocking_bounded_channel_wait_valid_spec` | `r[impl]` + `r[verify]` |
| `composite-primitives.safe-reclamation-domain` | positive reclaim, negative protected retain | `safe_reclamation_domain_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.cache-coherent-memory-region` | positive unique writer, negative coherence violation | `cache_coherent_memory_region_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.verified-compiler-pipeline` | runtime supported-slice preservation and invalid-analysis rejection | `verified_compiler_pipeline_refines_sources`, `verified_compiler_pipeline_ssa_preserves_observable_trace`, and `verified_compiler_pipeline_constfold_preserves_observable_trace` for proof-only preservation links | `r[impl]` + `r[verify]` |
| `composite-primitives.backend-frame-allocator` | positive reuse, negative overlap | `backend_frame_allocator_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.dependency-scheduler` | positive ready schedule, negative cycle/blocked | `dependency_scheduler_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.raft-membership-manager` | positive learner promotion, negative unsafe transfer | `raft_membership_manager_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.exactly-once-ledger-transfer` | positive post once, negative replay | `exactly_once_ledger_transfer_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.sel4-invocation-step` | positive invocation, negative invalid cap | `sel4_invocation_step_refines_sources` | `r[impl]` + `r[verify]` |
| `composite-primitives.nix-eval-closure-policy` | runtime merge/closure allow/deny where public APIs exist | `nix_eval_closure_policy_refines_sources` for closure reachability links | `r[impl]` + `r[verify]` |
| `composite-primitives.chaoscontrol-exploration-core` | positive deterministic step, negative oracle violation | `chaoscontrol_exploration_core_refines_sources` | `r[impl]` + `r[verify]` |

Final validation evidence MUST include `./scripts/verus.sh src/lib.rs`,
`./scripts/check-ensures.sh`, `tracey query uncovered`, and
`tracey query untested` with repo-wide zero uncovered and zero untested rules.

## Risks / Trade-offs

**[Large suite]** -> Mitigation: one requirement and one module per composite,
with focused tests and proof evidence per module.

**[Cross-module proof brittleness]** -> Mitigation: keep facade specs shallow,
reuse existing lemmas, and add local bridge lemmas instead of rewriting primitive
proofs.

**[Exec surfaces are uneven]** -> Mitigation: use spec-only composition where an
underlying domain is intentionally spec-first, and document that boundary in the
module design notes.

**[Tracey drift]** -> Mitigation: add `docs/requirements.md` IDs and `r[impl]` /
`r[verify]` tags as each module lands, then run uncovered/untested queries before
archive.
