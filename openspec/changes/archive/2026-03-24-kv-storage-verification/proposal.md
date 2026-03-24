## Why

aspen-redb-storage has a `verified/` module with pure functions for KV
entry versioning, snapshot integrity verification, and constant-time hash
comparison. These functions sit on the critical path of every Raft state
machine write — a bug in version computation corrupts the etcd-compatible
KV API, a bug in snapshot integrity lets corrupted snapshots install
silently, a bug in constant-time comparison leaks timing information.

The functions are tested with unit tests and follow Tiger Style (saturating
arithmetic, no I/O), but they lack machine-checked proofs. verified-logic
already covers the pieces these functions call — `expiration::expires_at`
for TTL calculation, `cas::check_cas` for CAS conditions, `chain.rs` for
log chain hashing — but the KV-specific composition layer is unverified.

Two areas have the most proof value:

1. **KV versioning** (`compute_kv_versions`): the version number, create
   revision, and mod revision fields that every etcd client depends on.
   Wrong version numbers break watches, transactions, and compaction.
   The properties are simple but load-bearing: version starts at 1 for
   new keys, increments on update, preserves create_revision across
   updates, and saturates instead of overflowing.

2. **Snapshot integrity** (`SnapshotIntegrity::compute`/`verify`): the
   dual-hash scheme that protects snapshot installation. A compute→verify
   round-trip must always succeed. Modifying either data or metadata must
   break verification. Chain hash continuity must be checked when provided.
   This extends the chain hash story already proven in `chain.rs` to cover
   the snapshot boundary.

3. **Constant-time comparison**: the XOR-fold pattern that prevents timing
   side channels on hash comparison. The property is that the function
   examines every byte regardless of where (or whether) a mismatch occurs.

## What Changes

Three new modules covering the pure decision logic from
aspen-redb-storage's `verified/` directory.

- **`kv_version`** — KV entry version computation: new key vs update,
  create_revision preservation, mod_revision assignment, version counter
  with saturating increment. Properties: version ≥ 1, create_revision
  stable across updates, mod_revision = log_index, monotonic version,
  saturation at i64::MAX.

- **`snapshot_integrity`** — Dual-hash snapshot verification with
  axiomatized Blake3 (same pattern as `chain.rs`). Properties:
  compute→verify round-trip, tamper detection on data and metadata
  independently, chain hash continuity check, combined hash commits
  to both components.

- **`constant_time_eq`** — Fixed-iteration XOR-fold comparison for
  32-byte hashes. Properties: equal inputs yield true, any single-byte
  difference yields false, result equivalent to byte-by-byte equality.

## Capabilities

### New Capabilities
- `kv-version`: Verified KV entry version computation with monotonicity
  and preservation proofs
- `snapshot-integrity`: Verified dual-hash snapshot verification with
  round-trip and tamper-detection proofs
- `constant-time-eq`: Verified fixed-iteration comparison with
  equivalence-to-equality proof

### Modified Capabilities
- `cross-module-properties`: Links from `kv_version` to `expiration`
  (TTL composition), from `snapshot_integrity` to `chain` (hash axioms),
  from `constant_time_eq` to `chain` (used by chain verification)

## Impact

- **Files**: 3 new src/ modules, src/lib.rs additions, src/properties.rs
  additions, delta spec
- **APIs**: ~15 new pub fns across 3 modules
- **Dependencies**: None — pure logic using existing vstd
- **Testing**: Verus verification + proptest property tests + model oracles
  against aspen-redb-storage's existing unit tests as reference
