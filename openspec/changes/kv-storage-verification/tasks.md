## Phase 1: KV Version Computation

- [ ] Create `src/kv_version.rs` with `KvVersions` struct (u64 fields: create_revision, mod_revision, version)
- [ ] Write `compute_kv_versions_spec` and `compute_kv_versions` exec fn with ensures
- [ ] Prove version_always_positive: output version ≥ 1 for all inputs
- [ ] Prove create_revision_preserved: update case preserves create_revision from input
- [ ] Prove mod_revision_equals_log_index: mod_revision == log_index for both cases
- [ ] Prove version_increments: update version == old_version + 1 (or saturated)
- [ ] Prove version_saturates: version at u64::MAX stays at u64::MAX
- [ ] Prove deterministic: same inputs → same output
- [ ] Write proptest suite mirroring aspen-redb-storage's kv.rs unit tests
- [ ] Add `kv_version` to src/lib.rs

## Phase 2: Snapshot Integrity

- [ ] Create `src/snapshot_integrity.rs` with axiomatized hash function (same pattern as chain.rs)
- [ ] Define `SnapshotIntegrity` struct: data_hash, meta_hash, combined_hash, chain_hash_at_snapshot
- [ ] Write `compute_integrity_spec`/`compute_integrity` exec fn
- [ ] Write `verify_integrity_spec`/`verify_integrity` exec fn
- [ ] Write `verify_with_chain_hash_spec`/`verify_with_chain_hash` exec fn
- [ ] Prove round_trip: compute then verify always returns true
- [ ] Prove data_tamper_detected: different data → verify returns false (from collision resistance axiom)
- [ ] Prove meta_tamper_detected: different metadata → verify returns false
- [ ] Prove chain_hash_mismatch_detected: wrong chain hash → verify_with_chain_hash returns false
- [ ] Prove combined_hash_commits_to_both: combined_hash depends on both data_hash and meta_hash
- [ ] Write proptest suite with round-trip and tamper-detection properties
- [ ] Add `snapshot_integrity` to src/lib.rs

## Phase 3: Constant-Time Comparison

- [ ] Create `src/constant_time_eq.rs` with `xor_fold_spec` as a sequence fold over 32 bytes
- [ ] Write `constant_time_eq_spec` and `constant_time_eq` exec fn for [u8; 32]
- [ ] Prove equal_returns_true: identical inputs → xor_fold == 0 → true
- [ ] Prove any_difference_returns_false: any differing byte → xor_fold != 0 → false
- [ ] Prove equivalence_to_equality: constant_time_eq(a, b) <==> (a == b)
- [ ] Prove fold_visits_all_bytes: the fold covers indices 0..32 (structural property)
- [ ] Write proptest suite: reflexivity, symmetry, agreement with ==
- [ ] Add `constant_time_eq` to src/lib.rs

## Phase 4: Cross-Module Properties and Cleanup

- [ ] Add to properties.rs: kv_version mod_revision satisfies monotonic::is_advance
- [ ] Add to properties.rs: kv_version version field satisfies monotonic::is_advance (for updates with increasing log_index)
- [ ] Add to properties.rs: snapshot_integrity hash axioms consistent with chain.rs hash axioms
- [ ] Add to properties.rs: constant_time_eq agrees with chain.rs constant-time pattern
- [ ] Update delta spec if any requirements changed during implementation
- [ ] Run `verus` on all three modules — zero errors
- [ ] Run `cargo test` — all proptests pass
