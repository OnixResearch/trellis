## Why

`verified-logic` now has hundreds of small verified primitives. The next useful
step is to expose composition primitives that downstream systems can adopt
without re-proving the same cross-module glue: durable job execution, fenced
locks, safe snapshot replication, cache/WAL recovery, compiler pipelines,
capability authorization, and similar end-to-end decisions.

This change specifies the full first suite of composition primitives from the
module families already present in the repository. The goal is not to replace
the underlying modules. The goal is to add small, reviewable facade modules whose
specs are conjunctions or refinements of existing primitive specs, with thin exec
surfaces where the underlying primitives already have runtime APIs.

## What Changes

- Add a `composite-primitives` OpenSpec domain covering these new primitives:
  - `durable_job_executor`
  - `fenced_distributed_lock`
  - `linearizable_read_gate`
  - `safe_snapshot_replication`
  - `crash_safe_page_cache`
  - `verified_lsm_ingest`
  - `serializable_txn_guard`
  - `web_of_trust_ucan_authorizer`
  - `policy_decision_engine`
  - `blocking_bounded_channel`
  - `safe_reclamation_domain`
  - `cache_coherent_memory_region`
  - `verified_compiler_pipeline`
  - `backend_frame_allocator`
  - `dependency_scheduler`
  - `raft_membership_manager`
  - `exactly_once_ledger_transfer`
  - `sel4_invocation_step`
  - `nix_eval_closure_policy`
  - `chaoscontrol_exploration_core`
- Require each composite to state exactly which primitive modules it composes,
  which input-validity boundary it uses, and which safety result it exports.
- Require positive and negative runtime tests for every exec facade.
- Require Verus composition proofs for every requirement before the change is
  archived.
- Require Tracey `r[impl]` / `r[verify]` coverage in `docs/requirements.md` once
  implementation begins.

## Scope and Milestones

This is one OpenSpec because the requested deliverable is the whole composition
suite, but implementation is intentionally sliced. Each composite gets its own
module, tests, proof evidence, and Tracey coverage. A phase may land locally only
when its focused tests and Verus checks pass, but the change is not archive-ready
until all twenty requirement IDs are implemented and validated.

Milestones:

1. shared facade/result conventions and requirement sync
2. highest-ROI composites: durable jobs, fenced locks, read gate, snapshots, and exactly-once transfers
3. storage, transaction, authorization, and channel composites
4. reclamation, coherence, compiler/backend/scheduler, and Raft membership composites
5. seL4, Nix/Nickel, ChaosControl, full docs/count sync, validation, and archive

## Exec Facade Boundary

Most composites expose public exec facades paired with spec/proof helpers. Mixed
spec-first surfaces stay explicit:

- `web_of_trust_ucan_authorizer` MUST NOT promise runtime DID or CID decode;
  existing DID decode and CID decode links remain spec/proof-only where the
  source modules are spec-first.
- `verified_compiler_pipeline` may expose only the checks and transforms already
  available at runtime; any whole-pipeline preservation theorem that depends on
  spec-only source facts remains proof-only.
- `nix_eval_closure_policy` may expose runtime merge/closure checks where source
  modules do, while closure reachability obligations may remain proof-only.

Positive and negative runtime tests are required for every public exec facade.
Spec/proof-only subfacts require Verus proof evidence instead of runtime tests.

Representative source runtime APIs already exist for each public facade:

| Composite | Facade status | Representative existing `pub fn` sources |
|---|---|---|
| `durable_job_executor` | public exec | `try_execute`, `history_valid`, `should_execute_effect`, `resume_step`, `should_retry`, `is_timed_out` |
| `fenced_distributed_lock` | public exec | `try_acquire`, `is_owner`, `write_permitted`, `issue_token`, `is_valid_write`, `check_cas` |
| `linearizable_read_gate` | public exec | `confirm_read_authority`, `read_safe_at`, `lease_read_safe`, `count_recent_contacts`, `is_quorum` |
| `safe_snapshot_replication` | public exec | `is_chunk_valid`, `verify_integrity`, `should_install_snapshot`, `compact_log`, `is_safe_to_compact` |
| `crash_safe_page_cache` | public exec | `is_contiguous`, `is_valid_checksum`, `apply_entries`, `dirty_count`, `evict_line` |
| `verified_lsm_ingest` | public exec | `should_flush`, `needs_compaction`, `is_safe_to_compact`, `insert`, `is_valid_checksum` |
| `serializable_txn_guard` | public exec | `snapshot_visible`, `has_ww_conflict`, `has_dependency`, `transition` |
| `web_of_trust_ucan_authorizer` | mixed | `validate_envelope`, `ability_subsumes`, `caveat_set_satisfied`, `chain_valid_at`, `threshold_accepts`; DID/CID decode remains spec-only |
| `policy_decision_engine` | public exec | `authorizes`, `chain_valid`, `contains`, `covers`, `glob_match`, `match_pattern`, `can_access` |
| `blocking_bounded_channel` | public exec | `can_enqueue`, `can_send`, `try_wait`, `notify`, `advance`, `valid_for_store` |
| `safe_reclamation_domain` | public exec | `can_free_old_version`, `is_safe_to_reclaim`, `can_reclaim`, `should_deallocate` |
| `cache_coherent_memory_region` | public exec | `check_system_invariant`, `processor_transition`, `valid_directory_entry`, `enqueue`, `prefetch_read`, `write_back_store` |
| `verified_compiler_pipeline` | mixed | `check_well_formed`, `compute_dominators`, `find_back_edges`, `compute_liveness`, `construct_ssa`, `eliminate_dead`; whole-pipeline preservation may be proof-only |
| `backend_frame_allocator` | public exec | `valid_live_ranges`, `build_interference`, `allocate_greedy`, `compute_frame_size`, `assign_slot` |
| `dependency_scheduler` | public exec | `all_deps_satisfied`, `topo_sort`, `has_cycle`, `edf_select`, `select_min_vtime`, `try_acquire_slot` |
| `raft_membership_manager` | public exec | `is_joint_quorum`, `can_start_config_change`, `promote_voters`, `promotion_safe`, `can_start_transfer` |
| `exactly_once_ledger_transfer` | public exec | `execute_transfer`, `ledger_transfer`, `is_balanced`, `make_entry`, `try_execute`, `should_execute_effect` |
| `sel4_invocation_step` | public exec | bounded variants `CopyCap`, `MapPage`, `EndpointSend`, `BlockSenderOnEndpoint` using `lookup`, `can_send`, `copy_cap`, `map`, `send`, TCB blocked-state transitions as accepted kernel-state mutations, and invariant proof wrappers |
| `nix_eval_closure_policy` | mixed | `merge`, `concat_lists`, `compare_versions`, `generic_closure`, `record_merge`; reachability proof may be spec-only |
| `chaoscontrol_exploration_core` | public exec | `tick`, `new_fault_state`, `drain_due`, `select_frontier_greedy`, `record_code_hit`, oracle proof wrappers |

## Capabilities

### New Capabilities

- `composite-primitives.durable-job-executor`: durable, idempotent, retry-aware job execution.
- `composite-primitives.fenced-distributed-lock`: lease lock decisions guarded by fencing tokens.
- `composite-primitives.linearizable-read-gate`: single predicate for safe Raft linearizable reads.
- `composite-primitives.safe-snapshot-replication`: chunk transfer, integrity, install, and compaction safety.
- `composite-primitives.crash-safe-page-cache`: WAL-backed dirty-cache recovery with checksum validation.
- `composite-primitives.verified-lsm-ingest`: write-buffer flush and level-ingest safety.
- `composite-primitives.serializable-txn-guard`: transaction accept/reject guard from MVCC, dependency, and locking facts.
- `composite-primitives.web-of-trust-ucan-authorizer`: UCAN authorization gated by rooted web-of-trust policy.
- `composite-primitives.policy-decision-engine`: unified delegation, containment, scope, pattern, and trust-level decision.
- `composite-primitives.blocking-bounded-channel`: bounded channel state plus futex/spin parking discipline.
- `composite-primitives.safe-reclamation-domain`: combined RCU, epoch, hazard-pointer, and reference-count reclamation safety.
- `composite-primitives.cache-coherent-memory-region`: coherence, store-buffer, prefetch, and write-policy invariants.
- `composite-primitives.verified-compiler-pipeline`: CFG/SSA/optimization pipeline preserving observable behavior.
- `composite-primitives.backend-frame-allocator`: register/spill/stack-slot allocation with live-range conflict freedom.
- `composite-primitives.dependency-scheduler`: DAG-safe job readiness with deadlines, fairness, and capacity.
- `composite-primitives.raft-membership-manager`: safe membership changes, learner promotion, and leadership transfer.
- `composite-primitives.exactly-once-ledger-transfer`: replay-safe money movement with accounting conservation.
- `composite-primitives.sel4-invocation-step`: bounded `CopyCap` / `MapPage` / `EndpointSend` / `BlockSenderOnEndpoint` transition preserving seL4 invariants.
- `composite-primitives.nix-eval-closure-policy`: Nix/Nickel config merge plus closure reachability policy.
- `composite-primitives.chaoscontrol-exploration-core`: deterministic exploration state, frontier, oracle, and coverage composition.

## Impact

- **Files**: new `src/*` composition modules, `src/lib.rs`, `docs/requirements.md`, `README.md`, `openspec/specs/composite-primitives/spec.md`, tests, and archive evidence.
- **APIs**: new public facade helpers that compose existing primitives without weakening the underlying contracts.
- **Dependencies**: no new runtime dependencies planned.
- **Testing / Verification**:
  - focused positive and negative tests for each facade module
  - `cargo test --quiet <module-filter>` for each implemented slice
  - `./scripts/verus.sh src/lib.rs`
  - `./scripts/check-ensures.sh`
  - `tracey query uncovered`
  - `tracey query untested`
  - full validation evidence before archive

## Out of Scope

- changing semantics of existing primitive modules
- adding networking, storage, compiler, or kernel algorithms beyond the glue needed for composition
- unverified public facade functions
- archived completion before all requirement IDs have matching implementation and verification evidence
