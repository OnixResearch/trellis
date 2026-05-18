# Composite Primitives Specification

## Purpose

Defines requirements for verified composition primitives built from existing
`verified-logic` modules. Each composite primitive MUST name the exact source
modules, MUST state the source validity predicates or constructor boundary, MUST
state whether the facade is public exec or spec/proof-only, and MUST export a
proof-backed cross-module safety statement. “Cross-module” means the accepted
predicate relates outputs or witnesses from at least two source modules over the
same facade input state, so the facade result is one atomic decision rather than
a bag of unrelated source calls.

## Common Facade Contract

Every composite facade MUST define disjoint outcomes:

- `Accept`: the requested transition/decision is admitted and the exported
  safety predicate holds.
- `Reject`: the request is denied and every mutable input state is unchanged,
  unless the requirement explicitly says the operation is read-only.
- `Wait`: only allowed for `blocking_bounded_channel` in this suite; this is an accepted
  state transition into a valid wait/backoff state, not a rejection. Any other
  wait-capable module requires a later spec modification. The blocking channel
  module MUST define `blocking_bounded_channel_wait_valid_spec(...)`.

Every composite module MUST define paired composed predicates named
`<module>_accepted_spec(...)` and `<module>_reject_preserves_spec(...)`, plus a
bridge proof named `<module>_refines_sources(...)`. The proof MUST establish that
an `Accept` result implies all named source predicates / validity boundaries plus
the composite's exported safety predicate, that `Reject` satisfies
`<module>_reject_preserves_spec(...)`, and that the blocking channel `Wait`
outcome satisfies `blocking_bounded_channel_wait_valid_spec(...)`. Each
`<module>_accepted_spec(...)` MUST contain at least one cross-module relation
that ties two or more source modules to the same token, index, resource, object,
trace, graph, closure item, or state transition.

Reference labels in every `Composes` list are determined as follows: names ending
in `_spec` are source predicates or spec functions. Every non-`_spec` source
item MUST appear in the classification table below as an exec API, spec
predicate, or proof/theorem dependency. Exec APIs are facade-shell inputs, and
their existing Verus `ensures` or module contracts must bridge them to paired
source specs before `*_refines_sources` can close. Exec APIs are not themselves
proof predicates.

| Non-`_spec` source item | Kind | Binding rule |
|---|---|---|
| `validate_envelope`, `encode_did_key` | exec API | UCAN/DID runtime shell inputs; proof obligations use paired spec facts |
| `topo_sort`, `has_cycle` | exec API | dependency-scheduler runtime graph checks; proofs use `is_topo_order_spec` / `has_cycle_spec` |
| `tick`, `apply_variant`, `drain_due`, `select_frontier_greedy` | exec API | ChaosControl runtime shell inputs; proofs use listed `*_spec` predicates |
| `joint_quorum_intersection`, `dce_preserves_observable_trace` | proof/theorem | consumed directly as proof dependencies |
| `quorum_intersection_prop` | spec predicate | quorum-intersection condition used by read-gate proof |
| `wf` | spec predicate method | seL4 CDT/VSpace/Endpoint well-formedness predicates |
| `can_enqueue_sender`, `can_enqueue_receiver` | spec predicate methods | seL4 endpoint queue-admission predicates |
| `is_sorted_keys`, `well_formed_version`, `keys_unique`, `is_sorted_record`, `disjoint_keys` | spec predicates | Nix/Nickel validity boundaries whose historical names do not end in `_spec` |

For every row in the table, the required reject-preservation predicate is the
mechanically named `<module>_reject_preserves_spec(...)`. For
`blocking_bounded_channel`, the additional wait predicate is
`blocking_bounded_channel_wait_valid_spec(...)`, proving occupancy remains
bounded, futex/backoff state is valid, and memory-order requirements still hold.

| Requirement ID | Module | Required bridge proof |
|---|---|---|
| `composite-primitives.durable-job-executor` | `durable_job_executor` | `durable_job_executor_refines_sources` |
| `composite-primitives.fenced-distributed-lock` | `fenced_distributed_lock` | `fenced_distributed_lock_refines_sources` |
| `composite-primitives.linearizable-read-gate` | `linearizable_read_gate` | `linearizable_read_gate_refines_sources` |
| `composite-primitives.safe-snapshot-replication` | `safe_snapshot_replication` | `safe_snapshot_replication_refines_sources` |
| `composite-primitives.crash-safe-page-cache` | `crash_safe_page_cache` | `crash_safe_page_cache_refines_sources` |
| `composite-primitives.verified-lsm-ingest` | `verified_lsm_ingest` | `verified_lsm_ingest_refines_sources` |
| `composite-primitives.serializable-txn-guard` | `serializable_txn_guard` | `serializable_txn_guard_refines_sources` |
| `composite-primitives.web-of-trust-ucan-authorizer` | `web_of_trust_ucan_authorizer` | `web_of_trust_ucan_authorizer_refines_sources` |
| `composite-primitives.policy-decision-engine` | `policy_decision_engine` | `policy_decision_engine_refines_sources` |
| `composite-primitives.blocking-bounded-channel` | `blocking_bounded_channel` | `blocking_bounded_channel_refines_sources` |
| `composite-primitives.safe-reclamation-domain` | `safe_reclamation_domain` | `safe_reclamation_domain_refines_sources` |
| `composite-primitives.cache-coherent-memory-region` | `cache_coherent_memory_region` | `cache_coherent_memory_region_refines_sources` |
| `composite-primitives.verified-compiler-pipeline` | `verified_compiler_pipeline` | `verified_compiler_pipeline_refines_sources` |
| `composite-primitives.backend-frame-allocator` | `backend_frame_allocator` | `backend_frame_allocator_refines_sources` |
| `composite-primitives.dependency-scheduler` | `dependency_scheduler` | `dependency_scheduler_refines_sources` |
| `composite-primitives.raft-membership-manager` | `raft_membership_manager` | `raft_membership_manager_refines_sources` |
| `composite-primitives.exactly-once-ledger-transfer` | `exactly_once_ledger_transfer` | `exactly_once_ledger_transfer_refines_sources` |
| `composite-primitives.sel4-invocation-step` | `sel4_invocation_step` | `sel4_invocation_step_refines_sources` |
| `composite-primitives.nix-eval-closure-policy` | `nix_eval_closure_policy` | `nix_eval_closure_policy_refines_sources` |
| `composite-primitives.chaoscontrol-exploration-core` | `chaoscontrol_exploration_core` | `chaoscontrol_exploration_core_refines_sources` |

## ADDED Requirements

### Requirement: Durable Job Executor [r[composite-primitives.durable-job-executor]]
The system MUST provide a durable job executor primitive for idempotent,
resume-aware, retry-budgeted job admission.

- **Composes:** `src/job_state.rs` (`is_valid_state_spec`, `is_valid_transition_spec`), `src/job_idempotency.rs` (`try_execute_spec`, `is_consumed_spec`), `src/durable_history.rs` (`history_valid_spec`, `append_allowed_spec`), `src/durable_replay.rs` (`should_execute_effect_spec`, `replayable_event_at_spec`), `src/durable_resume.rs` (`checkpoint_valid_spec`, `resume_step_spec`), `src/job_retry.rs` (`should_retry_spec`, `attempts_remaining_spec`), and `src/job_timeout.rs` (`is_timed_out_spec`).
- **Validity boundary:** the facade MUST reject histories that fail `history_valid_spec` and caller checkpoints that fail `checkpoint_valid_spec`; duplicate token, timeout, and retry exhaustion are decision results, not preconditions.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** accepted execution MUST imply valid lifecycle transition, non-duplicate token admission, replay gate approval, non-rewound resume point, retry budget compliance, and not-timed-out status.

#### Scenario: Fresh token with valid resume point is accepted [r[composite-primitives.durable-job-executor.scenario.fresh-token-with-valid-resume-point-is-accepted]]

- GIVEN a valid durable history with a fresh idempotency token and a valid checkpoint
- WHEN the durable job executor computes the next step
- THEN the result consumes the token once and resumes from the checkpoint-derived step

#### Scenario: Duplicate or expired work is rejected [r[composite-primitives.durable-job-executor.scenario.duplicate-or-expired-work-is-rejected]]

- GIVEN a duplicate idempotency token, an exhausted retry budget, or a job timeout
- WHEN execution admission is requested
- THEN the request is rejected without appending a new accepted-execution event

### Requirement: Fenced Distributed Lock [r[composite-primitives.fenced-distributed-lock]]
The system MUST provide a fenced distributed lock primitive for stale-holder-safe
protected writes.

- **Composes:** `src/distributed_lock.rs` (`compute_next_fencing_token_spec`, `is_lock_expired_spec`), `src/lease_mutex.rs` (`can_acquire_spec`, `is_owner_spec`, `write_permitted_spec`), `src/fencing.rs` (`issue_token_spec`, `is_valid_write_spec`, `is_stale_spec`), `src/monotonic.rs` (`is_strict_advance_spec`), and `src/cas.rs` (`check_cas_spec`).
- **Validity boundary:** lock-state availability and ownership are checked by `lease_mutex` predicates; token freshness is checked by fencing/monotonic predicates; invalid owners and expired leases are rejected.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** accepted protected writes MUST imply current ownership and a fencing token strictly newer than the accepted fence, so stale holders cannot pass the facade.

#### Scenario: Current lease holder receives usable fence [r[composite-primitives.fenced-distributed-lock.scenario.current-lease-holder-receives-usable-fence]]

- GIVEN a live lock lease owned by a requester and a monotone next fencing token
- WHEN the requester acquires or renews the fenced lock
- THEN the returned token is greater than every previously accepted token for the resource

#### Scenario: Stale holder is rejected [r[composite-primitives.fenced-distributed-lock.scenario.stale-holder-is-rejected]]

- GIVEN a holder whose lease expired or whose fencing token is older than the accepted token
- WHEN the holder attempts a protected write
- THEN the write decision is rejected as stale

### Requirement: Linearizable Read Gate [r[composite-primitives.linearizable-read-gate]]
The system MUST provide a Raft read gate primitive for one-step linearizable read
admission.

- **Composes:** `src/read_index.rs` (`confirm_read_authority_spec`, `read_safe_at_spec`), `src/lease_read.rs` (`is_lease_valid_spec`, `lease_read_safe_spec`, `is_leader_for_lease_spec`), `src/check_quorum.rs` (`count_recent_contacts_spec`, `should_step_down_spec`), `src/term_authority.rs` (`apply_higher_term_spec`), and `src/quorum.rs` (`is_quorum_spec`, `quorum_intersection_prop`).
- **Validity boundary:** the facade validates leader state, current term authority, lease/read-index freshness, and quorum contact; any failed source predicate rejects the read.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** accepted reads MUST imply current leader authority plus quorum-backed read index or valid lease-read conditions at the returned committed index.

#### Scenario: Quorum-confirmed read is safe [r[composite-primitives.linearizable-read-gate.scenario.quorum-confirmed-read-is-safe]]

- GIVEN a leader term with current authority and a read-index acknowledged by a quorum
- WHEN the read gate is evaluated
- THEN the read is accepted as linearizable at the committed index

#### Scenario: Lost authority blocks reads [r[composite-primitives.linearizable-read-gate.scenario.lost-authority-blocks-reads]]

- GIVEN a higher observed term, failed quorum check, or expired read lease
- WHEN the read gate is evaluated
- THEN the read is rejected

### Requirement: Safe Snapshot Replication [r[composite-primitives.safe-snapshot-replication]]
The system MUST provide a snapshot replication primitive for complete,
integrity-checked, compaction-safe installation.

- **Composes:** `src/snapshot_transfer.rs` (`is_chunk_valid_spec`, `next_expected_offset_spec`, `transfer_progress_spec`), `src/snapshot_integrity.rs` (`compute_integrity_spec`, `verify_integrity_spec`, `verify_with_chain_hash_spec`), `src/install_snapshot.rs` (`should_install_snapshot_spec`, `apply_snapshot_to_log_spec`), `src/log_compaction.rs` (`compact_log_spec`, `translate_index_spec`), and `src/compaction_fence.rs` (`is_safe_to_compact_spec`, `safe_compact_index_spec`).
- **Validity boundary:** chunks must pass transfer offset/size checks, integrity must verify, and compaction fences must approve the snapshot index; failure at any source predicate is rejection.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** installed snapshots MUST be complete, hash-valid, newer than the local snapshot boundary, and safe for log-prefix compaction.

#### Scenario: Complete verified snapshot installs safely [r[composite-primitives.safe-snapshot-replication.scenario.complete-verified-snapshot-installs-safely]]

- GIVEN all snapshot chunks arrive in order, match the expected integrity hash, and pass the compaction fence
- WHEN installation is requested
- THEN the snapshot is installed and the compacted log prefix is safe to discard

#### Scenario: Corrupt or unsafe snapshot is rejected [r[composite-primitives.safe-snapshot-replication.scenario.corrupt-or-unsafe-snapshot-is-rejected]]

- GIVEN a missing chunk, hash mismatch, stale snapshot index, or unsafe compaction fence
- WHEN installation is requested
- THEN installation is rejected and log state is unchanged

### Requirement: Crash-Safe Page Cache [r[composite-primitives.crash-safe-page-cache]]
The system MUST provide a crash-safe page-cache primitive for WAL-backed dirty
page recovery.

- **Composes:** `src/wal_segment.rs` (`is_contiguous_spec`, `is_checkpoint_valid_spec`, `is_truncate_safe_spec`), `src/wal_recovery.rs` (`replay_range_spec`, `replay_entries_spec`, `apply_all_spec`), `src/page_checksum.rs` (`checksum_spec`, `is_valid_checksum_spec`), `src/block_cache.rs` (`dirty_count_spec`, `write_back_spec`, `evict_spec`), and `src/write_policy.rs` (`flushable_cache_spec`, `flush_all_memory_spec`, `evict_line_spec`).
- **Validity boundary:** recovery first computes the maximal contiguous, checksum-valid WAL prefix from a valid checkpoint. `Accept` may recover that prefix and ignore an invalid suffix. `Reject` is reserved for invalid checkpoints, torn checkpoint pages, or inputs where no safe prefix can be identified; reject leaves mutable state unchanged.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** accepted recovered state MUST equal replay of exactly the maximal valid WAL prefix over checksum-valid pages; invalid suffix effects MUST be ignored. Rejected recovery MUST leave mutable state unchanged.

#### Scenario: Recovery replays valid dirty updates [r[composite-primitives.crash-safe-page-cache.scenario.recovery-replays-valid-dirty-updates]]

- GIVEN a WAL prefix with contiguous valid entries and cached dirty pages with matching checksums
- WHEN recovery runs
- THEN the recovered page state reflects exactly the replayable WAL prefix

#### Scenario: Invalid suffix is ignored during prefix recovery [r[composite-primitives.crash-safe-page-cache.scenario.invalid-suffix-is-ignored-during-prefix-recovery]]

- GIVEN a valid WAL prefix followed by a torn checksum, missing sequence, or invalid truncation point
- WHEN recovery runs
- THEN recovery applies only the valid prefix and does not apply the invalid suffix

#### Scenario: Invalid checkpoint rejects without mutation [r[composite-primitives.crash-safe-page-cache.scenario.invalid-checkpoint-rejects-without-mutation]]

- GIVEN an invalid checkpoint or torn checkpoint page from which no safe prefix can be identified
- WHEN recovery runs
- THEN recovery is rejected and mutable page-cache state is unchanged

### Requirement: Verified LSM Ingest [r[composite-primitives.verified-lsm-ingest]]
The system MUST provide an LSM ingest primitive for flush and compaction-safe run
admission.

- **Composes:** `src/write_buffer.rs` (`should_flush_spec`, `swap_spec`, `flush_complete_spec`), `src/lsm_level.rs` (`is_valid_size_ratio_spec`, `level_target_size_spec`, `needs_compaction_spec`), `src/compaction_fence.rs` (`is_safe_to_compact_spec`), `src/block_cache.rs` (`insert_spec`, `mark_dirty_spec`), and `src/page_checksum.rs` (`is_valid_checksum_spec`).
- **Validity boundary:** flush threshold, target-level size ratio, compaction fence, and block checksum predicates are checked; unsafe targets reject without discarding buffered data.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** accepted ingests MUST produce checksum-valid runs within level budget and rejected ingests MUST preserve buffered writes.

#### Scenario: Flush produces level-safe run [r[composite-primitives.verified-lsm-ingest.scenario.flush-produces-level-safe-run]]

- GIVEN a write buffer at flush threshold and a target level with available size budget
- WHEN ingest executes
- THEN the produced run respects level sizing and has checksum-valid blocks

#### Scenario: Unsafe compaction target is rejected [r[composite-primitives.verified-lsm-ingest.scenario.unsafe-compaction-target-is-rejected]]

- GIVEN a target level or compaction fence that would violate size or ordering constraints
- WHEN ingest is requested
- THEN the ingest is rejected without losing buffered writes

### Requirement: Serializable Transaction Guard [r[composite-primitives.serializable-txn-guard]]
The system MUST provide a transaction guard for commit admission from isolation,
dependency, and lock-discipline facts.

- **Composes:** `src/mvcc.rs` (`snapshot_visible_spec`, `is_committed_spec`), `src/snapshot_isolation.rs` (`si_allows_commit_spec`, `has_ww_conflict_spec`, `rw_overlap_spec`), `src/txn_dependency.rs` (`has_dependency_spec`, `has_self_dep_spec`, `has_two_cycle_spec`, `ts_ordered_spec`), and `src/two_phase_lock.rs` (`can_acquire_spec`, `would_violate_spec`, `transition_spec`, `is_complete_spec`).
- **Validity boundary:** transaction read/write sets, dependency edges, timestamps, and lock phase are validated by source predicates; conflict/cycle/phase failures reject commit.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** accepted commits MUST be MVCC-visible, SI conflict-free, dependency-cycle-free for the modeled edges, and two-phase-locking compliant.

#### Scenario: Conflict-free transaction commits [r[composite-primitives.serializable-txn-guard.scenario.conflict-free-transaction-commits]]

- GIVEN a transaction whose reads are visible, writes do not conflict, dependencies are acyclic, and locks remain in the growing phase until commit
- WHEN the guard evaluates commit admission
- THEN commit is accepted

#### Scenario: Serialization risk aborts [r[composite-primitives.serializable-txn-guard.scenario.serialization-risk-aborts]]

- GIVEN a write-write conflict, dependency cycle, or two-phase-locking violation
- WHEN the guard evaluates commit admission
- THEN commit is rejected

### Requirement: Web-of-Trust UCAN Authorizer [r[composite-primitives.web-of-trust-ucan-authorizer]]
The system MUST provide a UCAN authorizer that combines token validity,
attenuation, caveats, DID/CID byte models, and rooted web-of-trust policy.

- **Composes:** `src/ucan_token.rs` (`envelope_valid_spec`, `validate_envelope`, `temporal_valid_spec`, `attenuation_monotone_spec`), `src/ucan_ability.rs` (`ability_contains_spec`, `ability_subsumes_spec`), `src/ucan_caveat.rs` (`caveat_set_satisfied_spec`, `caveat_set_contains_spec`), `src/ucan_did.rs` (`encode_did_key`, `decode_did_key_spec`, `varint_decode_spec`), `src/ucan_cid.rs` (`cid_bytes_spec`, `multihash_decode_spec`), `src/web_of_trust_chain.rs` (`chain_valid_at_spec`), `src/web_of_trust_resolution.rs` (`resolves_trusted_at_spec`), and `src/web_of_trust_policy.rs` (`threshold_accepts_spec`).
- **Validity boundary:** token envelope, temporal validity, ability/caveat attenuation, and web-of-trust resolution are checked; DID decode and CID decode remain spec-only where existing modules are spec-first, while runtime validation may only use existing public exec surfaces.
- **Facade status:** mixed facade: public exec for existing UCAN and web-of-trust runtime checks; DID/CID decode linkage is spec/proof-only and MUST NOT promise a public runtime DID/CID decode API.
- **Exported safety:** allowed actions MUST imply valid token envelope, ability containment, satisfied caveats, valid DID/CID spec linkage, rooted non-revoked trust, and threshold acceptance.

#### Scenario: Rooted threshold token authorizes [r[composite-primitives.web-of-trust-ucan-authorizer.scenario.rooted-threshold-token-authorizes]]

- GIVEN a syntactically valid token with allowed ability, satisfied caveats, valid DID/CID linkage, and rooted non-revoked threshold trust
- WHEN authorization is evaluated
- THEN the requested action is authorized

#### Scenario: Revoked or under-threshold token is denied [r[composite-primitives.web-of-trust-ucan-authorizer.scenario.revoked-or-under-threshold-token-is-denied]]

- GIVEN a valid token whose trust path is revoked or whose rooted threshold is not met
- WHEN authorization is evaluated
- THEN the requested action is denied

### Requirement: Policy Decision Engine [r[composite-primitives.policy-decision-engine]]
The system MUST provide a policy decision engine for one allow/deny result from
capability and trust primitives.

- **Composes:** `src/authorize.rs` (`authorizes_spec`, `authorizes_compound_spec`), `src/delegation.rs` (`level_valid_spec`, `chain_fully_valid_spec`, `can_delegate_spec`), `src/containment.rs` (`contains_spec`, `contains_compound_spec`), `src/scope.rs` (`covers_spec`, `compound_covers_spec`), `src/glob.rs` (`glob_match_spec`), `src/pattern.rs` (`match_pattern_spec`, `pattern_contains_spec`), and `src/trust_level.rs` (`resolve_trust_spec`, `can_access_spec`).
- **Validity boundary:** delegation chains must satisfy `level_valid_spec` and `chain_fully_valid_spec`; containment/scope inputs are checked by `contains_spec`, `contains_compound_spec`, `covers_spec`, and `compound_covers_spec`; glob/pattern inputs are checked by `glob_match_spec`, `match_pattern_spec`, and `pattern_contains_spec`; trust inputs are checked by `resolve_trust_spec` and `can_access_spec`. Malformed chains are rejected; scope/pattern/trust mismatches are ordinary deny results.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** allow decisions MUST imply valid delegation, containment, scope/pattern match, authorization, and sufficient trust level.

#### Scenario: Contained delegation allows scoped action [r[composite-primitives.policy-decision-engine.scenario.contained-delegation-allows-scoped-action]]

- GIVEN a delegation chain whose scopes contain the requested action and whose trust level is sufficient
- WHEN the policy engine evaluates the request
- THEN the request is allowed

#### Scenario: Scope or trust mismatch denies [r[composite-primitives.policy-decision-engine.scenario.scope-or-trust-mismatch-denies]]

- GIVEN a delegation with insufficient containment, unmatched pattern, or insufficient trust level
- WHEN the policy engine evaluates the request
- THEN the request is denied

### Requirement: Blocking Bounded Channel [r[composite-primitives.blocking-bounded-channel]]
The system MUST provide a blocking bounded channel primitive that combines queue
capacity with wait/wake and backoff state.

- **Composes:** `src/bounded_mpmc.rs` (`occupancy_spec`, `can_enqueue_spec`, `can_dequeue_spec`), `src/mpsc_bounds.rs` (`can_send_spec`, `can_receive_spec`, `is_drained_spec`), `src/futex_state.rs` (`should_park_spec`, `try_wait_spec`, `notify_spec`), `src/spin_backoff.rs` (`is_spinning_spec`, `is_yielding_spec`, `advance_spec`), and `src/memory_order.rs` (`valid_for_load_spec`, `valid_for_store_spec`, `at_least_as_strong_spec`).
- **Validity boundary:** capacity, closed/drained state, futex state, backoff counters, and memory order are checked; full non-blocking sends reject and full blocking sends enter a valid wait/backoff state.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** channel transitions MUST preserve occupancy bounds and keep parking/wake decisions consistent with queue state and memory-order requirements.

#### Scenario: Available slot sends without parking [r[composite-primitives.blocking-bounded-channel.scenario.available-slot-sends-without-parking]]

- GIVEN a bounded channel with free capacity and valid memory-ordering preconditions
- WHEN a sender attempts to enqueue
- THEN the message is enqueued and no futex wait is required

#### Scenario: Full channel parks or rejects correctly [r[composite-primitives.blocking-bounded-channel.scenario.full-channel-parks-or-rejects-correctly]]

- GIVEN a full bounded channel
- WHEN a sender attempts to enqueue under blocking mode or non-blocking mode
- THEN blocking mode enters a valid wait/backoff state and non-blocking mode rejects without changing occupancy

### Requirement: Safe Reclamation Domain [r[composite-primitives.safe-reclamation-domain]]
The system MUST provide a safe reclamation domain from RCU, epoch, hazard, and
reference-count protections.

- **Composes:** `src/rcu.rs` (`is_grace_period_complete_spec`, `can_free_old_version_spec`, `all_threads_quiescent_spec`), `src/epoch_reclaim.rs` (`is_safe_to_reclaim_spec`, `can_advance_spec`, `count_active_spec`), `src/hazard_pointer.rs` (`is_protected_spec`, `can_reclaim_spec`, `is_slot_active_spec`, `all_slots_empty_spec`), and `src/ref_count.rs` (`should_deallocate_spec`, `can_upgrade_weak_spec`, `total_refs_spec`).
- **Validity boundary:** RCU reader counters are evaluated through `all_threads_quiescent_spec` / `is_grace_period_complete_spec`; epoch activity is evaluated through `count_active_spec`; hazard slots are evaluated through `is_protected_spec`, `is_slot_active_spec`, and `all_slots_empty_spec`; reference counts are evaluated through `should_deallocate_spec`. Active readers, pinned epochs, hazard slots, or strong references cause rejection.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** reclaim decisions MUST imply completed grace/epoch safety, no hazard protection, and no strong reference preventing deallocation.

#### Scenario: Unprotected retired object is reclaimable [r[composite-primitives.safe-reclamation-domain.scenario.unprotected-retired-object-is-reclaimable]]

- GIVEN an object retired before a completed grace period, with no active hazard pointer and no strong reference
- WHEN reclamation is scanned
- THEN the object may be reclaimed

#### Scenario: Protected object is retained [r[composite-primitives.safe-reclamation-domain.scenario.protected-object-is-retained]]

- GIVEN an active reader epoch, hazard pointer, or strong reference covering an object
- WHEN reclamation is scanned
- THEN the object is not reclaimed

### Requirement: Cache-Coherent Memory Region [r[composite-primitives.cache-coherent-memory-region]]
The system MUST provide a cache-coherent memory-region primitive combining cache
protocol, directory, store-buffer, prefetch, and write-policy facts.

- **Composes:** `src/mesi.rs` (`system_invariant_spec`, `processor_write_spec`), `src/moesi.rs` (`single_writer_spec`, `memory_consistent_spec`, `read_fresh_spec`), `src/directory_coherence.rs` (`valid_directory_entry_spec`, `directory_cache_consistent_spec`), `src/store_buffer.rs` (`buffer_valid_spec`, `can_forward_spec`, `is_fence_satisfied_spec`), `src/cache_prefetch.rs` (`valid_cache_vector_spec`, `prefetch_read_cache_spec`, `prefetch_write_cache_spec`), and `src/write_policy.rs` (`flushable_cache_spec`, `write_back_store_spec`, `write_through_store_mem_spec`).
- **Validity boundary:** cache vectors, directory entries, store-buffer entries, prefetch hints, and write-policy lines must satisfy source validity predicates; invalid shapes are rejected before coherence claims.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** accepted region transitions MUST preserve unique writable ownership, directory/cache consistency, store-buffer forwarding validity, prefetch architectural invisibility, and dirty-line policy invariants.

#### Scenario: Exclusive writer remains unique [r[composite-primitives.cache-coherent-memory-region.scenario.exclusive-writer-remains-unique]]

- GIVEN a region with a directory owner and coherent per-cache line states
- WHEN a write permission grant is evaluated
- THEN at most one cache owns writable permission for the line

#### Scenario: Prefetch or store forwarding does not violate coherence [r[composite-primitives.cache-coherent-memory-region.scenario.prefetch-or-store-forwarding-does-not-violate-coherence]]

- GIVEN a prefetch hint or store-buffer forwarded read
- WHEN the memory-region invariant is checked
- THEN coherence state and architectural read/write equivalence remain valid

### Requirement: Verified Compiler Pipeline [r[composite-primitives.verified-compiler-pipeline]]
The system MUST provide a bounded compiler-pipeline primitive for the existing raw
IR model.

- **Composes:** `src/control_flow_graph.rs` (`valid_program_spec`, `well_formed_cfg_spec`), `src/dominator_tree.rs` (`valid_graph_spec`, `dominates_matrix_spec`), `src/loop_detection.rs` (`back_edges_spec`, `single_entry_spec`, `loop_exits_spec`), `src/liveness_analysis.rs` (`valid_liveness_input_spec`, `equations_hold_spec`, `live_range_spec`, `sequential_live_summary_spec`), `src/ssa_construction.rs` (`raw_program_valid_spec`, `ssa_program_valid_spec`), `src/constant_folding.rs` (`constprop_sets_shape_spec`, `top_free_env_spec`), and `src/dead_code_elimination.rs` (`live_rows_compatible_spec`, `eliminate_block_spec`, `observable_trace_spec`, `observable_trace_after_dce_spec`, `dce_preserves_observable_trace`).
- **Validity boundary:** this first slice is limited to the existing `RawProgram` straight-line block-order observable-trace model from `dead_code_elimination` (`observable_trace_spec` over stores, calls, and returns). Inputs must satisfy `valid_program_spec`, `well_formed_cfg_spec`, `raw_program_valid_spec`, `valid_liveness_input_spec`, `sequential_live_summary_spec`, and `live_rows_compatible_spec`; malformed CFG targets, invalid analysis shapes, non-sequential path semantics, or unsupported instruction semantics are rejected instead of assumed.
- **Facade status:** public exec facade for the pipeline checks that existing modules expose; proof-only bridge for any preservation theorem whose source module remains spec-only.
- **Additional bridge obligations:** the composite MUST add `verified_compiler_pipeline_ssa_preserves_observable_trace(...)` and `verified_compiler_pipeline_constfold_preserves_observable_trace(...)` before the full pipeline can be accepted; `dce_preserves_observable_trace` remains the named existing DCE theorem.
- **Exported safety:** accepted optimized output MUST preserve the modeled observable trace using the SSA bridge, constant-folding bridge, and `observable_trace_spec` / `observable_trace_after_dce_spec` for the supported raw-IR slice; it MUST not claim preservation for invalid analyses.

#### Scenario: Optimized program preserves observable behavior [r[composite-primitives.verified-compiler-pipeline.scenario.optimized-program-preserves-observable-behavior]]

- GIVEN a valid raw program and analyses satisfying their validity boundaries
- WHEN the pipeline runs SSA construction, constant folding, and dead-code elimination
- THEN the optimized program preserves the specified observable trace

#### Scenario: Invalid analysis boundary is rejected [r[composite-primitives.verified-compiler-pipeline.scenario.invalid-analysis-boundary-is-rejected]]

- GIVEN malformed CFG targets, invalid dominance facts, or liveness shape mismatch
- WHEN the pipeline is requested
- THEN the pipeline rejects the input instead of claiming preservation

### Requirement: Backend Frame Allocator [r[composite-primitives.backend-frame-allocator]]
The system MUST provide a backend frame allocator for the existing live-range,
register, and local/spill stack-slot model.

- **Composes:** `src/liveness_analysis.rs` (`live_range_spec`, `valid_liveness_input_spec`), `src/register_allocation.rs` (`valid_live_ranges_spec`, `valid_graph_spec`, `valid_coloring_spec`, `allocation_well_formed_spec`), and `src/stack_frame_layout.rs` (`live_range_valid_spec`, `can_reuse_slot_spec`, `non_overlap_spec`, `frame_size_spec`, `layout_valid_spec`).
- **Validity boundary:** this first slice is limited to local/spill slots already modeled by `stack_frame_layout`; live inputs must satisfy `valid_liveness_input_spec` / `live_range_spec`, register inputs must satisfy `valid_live_ranges_spec`, `valid_graph_spec`, `valid_coloring_spec`, and `allocation_well_formed_spec`, and stack-slot inputs must satisfy `live_range_valid_spec`, `can_reuse_slot_spec`, `non_overlap_spec`, `frame_size_spec`, and `layout_valid_spec`. Callee-save and ABI argument placement remain out of scope.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** accepted allocations MUST assign overlapping live ranges to non-conflicting registers or stack slots and may reuse stack slots only when live ranges do not overlap.

#### Scenario: Non-overlapping ranges reuse a slot [r[composite-primitives.backend-frame-allocator.scenario.non-overlapping-ranges-reuse-a-slot]]

- GIVEN two spill ranges that do not overlap and share compatible layout constraints
- WHEN stack slots are assigned
- THEN the ranges may reuse one slot without conflict

#### Scenario: Overlapping ranges are separated [r[composite-primitives.backend-frame-allocator.scenario.overlapping-ranges-are-separated]]

- GIVEN overlapping live ranges assigned to stack or register locations
- WHEN allocation is validated
- THEN the ranges are assigned distinct non-conflicting resources or allocation is rejected

### Requirement: Dependency Scheduler [r[composite-primitives.dependency-scheduler]]
The system MUST provide a dependency scheduler that admits only ready,
capacity-safe work from graph and scheduling facts.

- **Composes:** `src/job_dag.rs` (`all_deps_satisfied_spec`, `unsatisfied_count_spec`), `src/topo_sort.rs` (`is_topo_order_spec`, `topo_sort`), `src/cycle_detect.rs` (`has_cycle_spec`, `has_cycle`), `src/deadline_queue.rs` (`edf_select_spec`, `is_earlier_spec`), `src/fair_share.rs` (`select_min_vtime_spec`, `fair_alloc_spec`), and `src/job_slot.rs` (`has_available_slot_spec`, `try_acquire_slot_spec`).
- **Validity boundary:** job graphs must be acyclic and topologically valid, dependencies must be satisfied, and slot capacity must be available; blocked jobs are rejected.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** scheduled jobs MUST be dependency-ready, cycle-free under the modeled graph, deadline/fairness eligible, and within slot capacity.

#### Scenario: Ready acyclic job is scheduled within capacity [r[composite-primitives.dependency-scheduler.scenario.ready-acyclic-job-is-scheduled-within-capacity]]

- GIVEN an acyclic job graph with all dependencies satisfied for a candidate job and an available slot
- WHEN scheduling chooses the next job
- THEN the selected job is ready and capacity remains within bounds

#### Scenario: Cyclic or blocked job is not scheduled [r[composite-primitives.dependency-scheduler.scenario.cyclic-or-blocked-job-is-not-scheduled]]

- GIVEN a cycle, unsatisfied dependency, missed capacity bound, or unfair selection state
- WHEN scheduling chooses the next job
- THEN the blocked job is not selected

### Requirement: Raft Membership Manager [r[composite-primitives.raft-membership-manager]]
The system MUST provide a Raft membership manager for safe configuration changes,
learner promotion, and leadership transfer.

- **Composes:** `src/joint_consensus.rs` (`is_joint_quorum_spec`, `joint_quorum_intersection`), `src/config_change.rs` (`can_start_config_change_spec`, `can_leave_joint_spec`, `leader_in_new_config_spec`), `src/voter_set.rs` (`promote_voters_spec`, `all_distinct_spec`), `src/learner_change.rs` (`can_add_learner_spec`, `can_remove_learner_spec`, `quorum_unchanged_spec`), `src/learner_readiness.rs` (`is_caught_up_spec`, `promotion_safe_spec`), and `src/leadership_transfer.rs` (`can_start_transfer_spec`, `is_target_caught_up_spec`, `is_transfer_expired_spec`).
- **Validity boundary:** voter sets must remain distinct, learners must be caught up before promotion, and leadership transfer targets must be eligible; unsafe transitions reject.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** accepted membership changes MUST preserve quorum intersection, valid config-change phase, learner readiness, and leadership-transfer eligibility.

#### Scenario: Learner promotion through joint consensus is safe [r[composite-primitives.raft-membership-manager.scenario.learner-promotion-through-joint-consensus-is-safe]]

- GIVEN a caught-up learner and a valid joint-consensus transition
- WHEN membership promotion is applied
- THEN quorum safety and configuration-change invariants hold

#### Scenario: Unsafe transfer or promotion is rejected [r[composite-primitives.raft-membership-manager.scenario.unsafe-transfer-or-promotion-is-rejected]]

- GIVEN a stale learner, invalid voter set, or leadership transfer target that is not eligible
- WHEN membership change is requested
- THEN the change is rejected

### Requirement: Exactly-Once Ledger Transfer [r[composite-primitives.exactly-once-ledger-transfer]]
The system MUST provide an exactly-once ledger transfer primitive for replay-safe
money movement.

- **Composes:** `src/transfer.rs` (`validate_transfer_spec`, `transfer_valid_spec`, `is_duplicate_spec`), `src/ledger.rs` (`can_transfer_spec`, `transfer_result_spec`, `total_balance_spec`), `src/double_entry.rs` (`is_balanced_spec`, `trial_balance_spec`), `src/journal.rs` (`journal_valid_spec`, `can_append_spec`, `final_balance_spec`), `src/job_idempotency.rs` (`try_execute_spec`), and `src/durable_replay.rs` (`should_execute_effect_spec`, `effect_completed_spec`).
- **Validity boundary:** balances, journal appendability, transfer amount, duplicate token state, and replay gate are checked; duplicate/replayed transfers reject without a second posting.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** accepted transfers MUST post balanced debit/credit entries once, preserve ledger total, append valid journal evidence, and reject duplicate replay.

#### Scenario: New transfer posts once [r[composite-primitives.exactly-once-ledger-transfer.scenario.new-transfer-posts-once]]

- GIVEN sufficient funds, a fresh idempotency token, and an appendable journal
- WHEN the transfer is applied
- THEN debit and credit entries are balanced and total ledger value is conserved

#### Scenario: Replayed transfer does not double-post [r[composite-primitives.exactly-once-ledger-transfer.scenario.replayed-transfer-does-not-double-post]]

- GIVEN a duplicate idempotency token or replayed durable event
- WHEN the transfer is applied again
- THEN no second debit or credit is posted

### Requirement: seL4 Invocation Step [r[composite-primitives.sel4-invocation-step]]
The system MUST provide a seL4 invocation-step primitive that preserves kernel
invariants across a bounded modeled capability invocation slice: `CopyCap`,
`MapPage`, `EndpointSend`, and `BlockSenderOnEndpoint`. IRQ, notification,
ASID, untyped, reply, receive, scheduling, and deletion/revocation invocations
are out of scope for this composite.

- **Composes:** `src/sel4_cspace.rs` (`lookup_spec`, `lookup_from_spec`), `src/sel4_cap_rights.rs` (`rights_subset_spec`, `can_send_spec`, `can_receive_spec`, `can_map_spec`, `can_control_spec`), `src/sel4_cdt.rs` (`wf`, `strict_ancestor_spec`, `subtree_member_spec`), `src/sel4_vspace.rs` (`wf`, `map_enabled_spec`, `map_transition_spec`), `src/sel4_endpoint.rs` (`wf`, `can_enqueue_sender`, `can_enqueue_receiver`), `src/sel4_tcb.rs` (`legal_state_spec`, `legal_transition_spec`), and `src/sel4_invariants.rs` (`kernel_invariant_spec`).
- **Validity boundary:** the facade first validates CSpace lookup, cap rights, CDT/VSpace/endpoint/TCB well-formedness, and kernel invariant prestate for exactly the `CopyCap`, `MapPage`, `EndpointSend`, and `BlockSenderOnEndpoint` variants; invalid capability, object mismatch, or any out-of-scope invocation rejects without mutation.
- **Facade status:** public exec facade for modeled invocations with paired spec and proof helpers.
- **Exported safety:** accepted invocation steps MUST mutate only the object allowed by the resolved capability and MUST preserve `kernel_invariant_spec`. Modeled blocked-TCB state transitions are `Accept` kernel-state mutations, not suite-level `Wait` outcomes.

#### Scenario: Valid invocation preserves invariants [r[composite-primitives.sel4-invocation-step.scenario.valid-invocation-preserves-invariants]]

- GIVEN a valid capability lookup, sufficient rights, and one of `CopyCap`, `MapPage`, `EndpointSend`, or `BlockSenderOnEndpoint` matching the object type
- WHEN the invocation step executes
- THEN the corresponding kernel object state changes and global invariants are preserved

#### Scenario: Invalid capability blocks mutation [r[composite-primitives.sel4-invocation-step.scenario.invalid-capability-blocks-mutation]]

- GIVEN a missing slot, insufficient rights, invalid derivation, or mismatched invocation type
- WHEN the invocation step is requested
- THEN the kernel state is unchanged

### Requirement: Nix Eval Closure Policy [r[composite-primitives.nix-eval-closure-policy]]
The system MUST provide a Nix/Nickel closure-policy primitive for merged config
and reachable package closure admission.

- **Composes:** `src/nix_attrset.rs` (`is_sorted_keys`, `get_attr_spec`, `merge_spec`), `src/nix_list.rs` (`length_spec`, `elem_spec`, `concat_lists_spec`), `src/nix_string.rs` (`substring_spec`, `replace_strings_spec`), `src/nix_version.rs` (`well_formed_version`, `compare_versions_spec`), `src/nix_closure.rs` (`generic_closure_spec`, `reachable_key_spec`, `keys_unique`), and `src/nickel_record_merge.rs` (`is_sorted_record`, `record_merge_spec`, `disjoint_keys`).
- **Validity boundary:** attrsets/records must be sorted and merge-valid, version strings must be well formed, and closure operator tables must be finite/key-unique; invalid merge or unreachable dependency rejects.
- **Facade status:** public exec facade where source modules expose exec operations; closure-policy reachability proof may use spec-only helpers.
- **Exported safety:** accepted configs MUST include every reachable closure item required by the Nickel-merged Nix package selection.

#### Scenario: Merged config admits reachable package closure [r[composite-primitives.nix-eval-closure-policy.scenario.merged-config-admits-reachable-package-closure]]

- GIVEN a Nickel-merged configuration selecting Nix packages with valid versions and reachable dependencies
- WHEN closure policy is evaluated
- THEN every required closure item is included

#### Scenario: Missing dependency or rejected merge blocks closure [r[composite-primitives.nix-eval-closure-policy.scenario.missing-dependency-or-rejected-merge-blocks-closure]]

- GIVEN an invalid merge, missing attribute, incompatible version, or unreachable dependency
- WHEN closure policy is evaluated
- THEN the policy rejects the result

### Requirement: ChaosControl Exploration Core [r[composite-primitives.chaoscontrol-exploration-core]]
The system MUST provide a ChaosControl exploration core for deterministic,
coverage-accounted, oracle-checked exploration steps.

- **Composes:** `src/chaoscontrol_scheduler.rs` (`valid_scheduler_spec`, `tick`, `apply_variant`), `src/chaoscontrol_fault_state.rs` (`valid_fault_state_spec`, `quorum_reachable_spec`, `healthy_period_spec`), `src/chaoscontrol_fault_schedule.rs` (`valid_schedule_spec`, `drain_due`), `src/chaoscontrol_frontier.rs` (`score_ge_spec`, `select_frontier_greedy`), `src/chaoscontrol_oracle.rs` (`valid_oracle_record_spec`, `verdict_spec`), and `src/chaoscontrol_coverage.rs` (`valid_coverage_bitmap_spec`, `merge_spec`, `new_edge_count_spec`).
- **Validity boundary:** scheduler, fault state, schedule, frontier entries, oracle records, and coverage bitmaps must satisfy source validity predicates; oracle violation or invalid schedule rejects the path.
- **Facade status:** public exec facade with paired spec and proof helpers.
- **Exported safety:** accepted exploration steps MUST be deterministic for the selected state, apply only valid due faults, and update coverage consistently. Oracle failure MUST be returned as a `Reject` reason without mutating exploration state.

#### Scenario: Frontier step is deterministic and covered [r[composite-primitives.chaoscontrol-exploration-core.scenario.frontier-step-is-deterministic-and-covered]]

- GIVEN a scheduler state, selected frontier node, and enabled fault schedule
- WHEN exploration advances one step
- THEN the next state is deterministic and coverage accounting records the explored rule

#### Scenario: Oracle violation stops exploration path [r[composite-primitives.chaoscontrol-exploration-core.scenario.oracle-violation-stops-exploration-path]]

- GIVEN an oracle violation or invalid fault schedule
- WHEN exploration attempts to continue the path
- THEN that path is rejected and the violation is returned in the reject reason without mutating exploration state
