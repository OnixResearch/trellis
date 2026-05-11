# Aspen Verified Logic Evidence Index Report

- status: pass
- rows: 8

## coordination-rate-limiter-refill-acquire
- source commit: `1d266aaac5118530b5cfcf8119768b6aaf961128`
- source paths: `crates/aspen-coordination/verus/rate_limiter_ops_spec.rs`
- local snapshot manifest: `-`
- boundary notes: Aspen runtime token accounting shell and remaining arithmetic helper trust boundaries stay outside this row.

## coordination-queue-explicit-nack
- source commit: `cc1f709f6f3a5990d1389827bfff3360d586e987`
- source paths: `crates/aspen-coordination/verus/queue_ack_spec.rs`
- local snapshot manifest: `-`
- boundary notes: Aspen queue runtime I/O, persistence, serialization, and remaining queue proof gaps stay outside this row.

## coordination-worker-expiration
- source commit: `b7246dc5f96d4a2560edc843a56b3ffacbf4071c`
- source paths: `crates/aspen-coordination/verus/worker_ops_spec.rs`
- local snapshot manifest: `-`
- boundary notes: Aspen worker runtime scheduling shell, lease time source, and remaining worker proof gaps stay outside this row.

## coordination-queue-ack-identity
- source commit: `e7a0bf8cd2fa1d1b8e43bdaaf8fc3068ef2c78a9`
- source paths: `crates/aspen-coordination/verus/queue_ack_spec.rs`
- local snapshot manifest: `-`
- boundary notes: Aspen queue runtime I/O, persistence, serialization, and remaining queue proof gaps stay outside this row.

## coordination-election-renewal-time
- source commit: `60a3ffec9bb9a5c7b152ecace2317a0ecc51854d`
- source paths: `crates/aspen-coordination/verus/election_state_spec.rs`
- local snapshot manifest: `-`
- boundary notes: Aspen election runtime clock source and compute_renewal_backoff trust boundary stay outside this row.

## coordination-registry-heartbeat-cleanup
- source commit: `8fd5a1436849d1bfaf66d24d13cb3ba87bda9020`
- source paths: `crates/aspen-coordination/verus/registry_ops_spec.rs`
- local snapshot manifest: `-`
- boundary notes: Aspen registry runtime health checks, storage shell, and remaining registry proof gaps stay outside this row.

## coordination-worker-completion
- source commit: `902fc415484e5ac9662fe0a247c411c32ef65ed9`
- source paths: `crates/aspen-coordination/verus/worker_ops_spec.rs`
- local snapshot manifest: `-`
- boundary notes: Aspen worker runtime scheduling shell, completion side effects, and remaining worker proof gaps stay outside this row.

## coordination-worker-pending-assignment
- source commit: `5861bf114f4ce6e7178668d7fb92b6361ac76397`
- source paths: `crates/aspen-coordination/verus/worker_ops_spec.rs`
- local snapshot manifest: `-`
- boundary notes: Aspen worker runtime scheduling shell, lease extension proof gaps, and remaining worker proof gaps stay outside this row.
