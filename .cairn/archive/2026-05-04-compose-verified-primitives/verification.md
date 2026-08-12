# Verification

## Completed scope

The `compose-verified-primitives` change now has twenty verified composite facades. Each facade owns paired accepted/reject predicates, a source-refinement proof bridge, public execution coverage where applicable, and runtime positive/negative evidence.

## Commands captured

```bash
mkdir -p openspec/changes/archive/2026-05-04-compose-verified-primitives/evidence/final
modules='durable_job_executor fenced_distributed_lock linearizable_read_gate safe_snapshot_replication crash_safe_page_cache verified_lsm_ingest serializable_txn_guard web_of_trust_ucan_authorizer policy_decision_engine blocking_bounded_channel safe_reclamation_domain cache_coherent_memory_region verified_compiler_pipeline backend_frame_allocator dependency_scheduler raft_membership_manager exactly_once_ledger_transfer sel4_invocation_step nix_eval_closure_policy chaoscontrol_exploration_core'
for m in $modules; do cargo test --quiet "$m"; done
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet
./scripts/verus.sh src/lib.rs
./scripts/check-ensures.sh
tracey query uncovered
tracey query untested
openspec validate compose-verified-primitives --strict --json
git diff --check
```

## Evidence map

- `evidence/review-index.md`
- `evidence/final/focused-module-tests.txt`
- `evidence/final/full-cargo-test.txt`
- `evidence/final/full-verus-lib.txt`
- `evidence/final/check-ensures.txt`
- `evidence/final/tracey-coverage.txt`
- `evidence/final/openspec-validate.txt`
- `evidence/final/git-diff-check.txt`

## Result summary

- Focused module tests: all twenty module filters passed.
- Full cargo tests: `4283 passed`; doc tests: `13 passed`.
- Verus: `8730 verified, 0 errors`.
- Tracey: `0 uncovered`; `0 untested`; `1772 of 1772` covered and verified.
- OpenSpec strict validation: passed.
- Whitespace check: passed.

## Explicit non-claims

The composite facades prove finite/source-composition properties over the repo-local models. They do not claim external system correctness, cryptographic strength beyond existing trusted primitives, scheduler fairness beyond modeled predicates, storage durability beyond modeled histories, runtime OS/hardware correctness, or production security.
