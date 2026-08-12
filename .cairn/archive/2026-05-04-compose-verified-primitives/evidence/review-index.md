# Composite Primitive Evidence Index

## Scope

This evidence package closes the `compose-verified-primitives` change for twenty public composite facades:

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

## Evidence files

- `final/focused-module-tests.txt` — targeted positive/negative runtime evidence for all twenty module filters.
- `final/full-cargo-test.txt` — full `cargo test --quiet` evidence with shared target directory.
- `final/full-verus-lib.txt` — full-library Verus proof evidence.
- `final/check-ensures.txt` — public facade ensures gate evidence.
- `final/tracey-coverage.txt` — repo-wide Tracey uncovered/untested evidence.
- `final/openspec-validate.txt` — strict OpenSpec validation for this change.
- `final/git-diff-check.txt` — whitespace diff check.

## Fresh results

- Focused module tests: all twenty filters passed.
- Full cargo tests: `4283 passed` plus `13` doc tests.
- Verus: `8730 verified, 0 errors`.
- Tracey: `0 uncovered` and `0 untested` out of `1772` requirements.
- OpenSpec: `compose-verified-primitives` strict validation passed.
