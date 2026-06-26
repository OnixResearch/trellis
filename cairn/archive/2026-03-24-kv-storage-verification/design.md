## Context

verified-logic already covers the foundations that aspen-redb-storage's
verified functions call: `expiration::expires_at` for TTL, `cas::check_cas`
for CAS conditions, `chain.rs` for log chain hashing. The remaining
unverified layer is the KV-specific composition: version bookkeeping,
snapshot integrity, and constant-time comparison.

The aspen-redb-storage functions are pure, take explicit parameters, and
use saturating arithmetic. They map directly to Verus exec functions with
ensures clauses.

## Goals / Non-Goals

**Goals:**
- Verify the three function groups from aspen-redb-storage that aren't
  already covered
- Prove the safety properties that matter at the call sites (version
  monotonicity, snapshot round-trip, timing-safe comparison)
- Keep the verified functions generic — no dependency on aspen types,
  redb, or Raft-specific structures

**Non-Goals:**
- Verifying the storage engine itself (redb transactions, disk I/O)
- Verifying serialization (serde, bincode)
- Replacing aspen-redb-storage — the verified functions here are
  building blocks that aspen can call or cross-reference

## Decisions

### 1. Model KV versions with u64 instead of i64

**Choice:** Use `u64` for version, create_revision, and mod_revision
internally. Provide a conversion note in docs for aspen's i64 API.

**Rationale:** Verus arithmetic works most naturally with unsigned types.
aspen uses i64 because etcd's protobuf API uses int64, but the values are
always non-negative. Modeling with u64 avoids negative-number edge cases
that don't exist in practice and lets the saturating-add proof be
straightforward.

**Alternative:** Use i64 directly. Rejected because it adds proof
complexity (must show values stay non-negative) with no practical benefit.

**Implementation:** `compute_kv_versions` takes `Option<(u64, u64)>` for
`(create_revision, version)` and returns a `KvVersions` struct with u64
fields. The spec fns and proofs operate on u64 throughout.

### 2. Axiomatize Blake3 for snapshot integrity (same as chain.rs)

**Choice:** Model the hash function as an uninterpreted `closed spec fn`
with determinism and collision-resistance axioms, identical to the approach
in `chain.rs`.

**Rationale:** Verus cannot verify cryptographic hash function internals.
The standard approach is to axiomatize the properties you rely on
(determinism, collision resistance) and mark the exec implementation as
`external_body`. This is the same pattern used in `chain.rs` and is
well-understood in the verification community.

**Alternative:** Skip verification of snapshot integrity entirely. Rejected
because the compute→verify round-trip is a high-value property — it's the
kind of thing that breaks silently when someone refactors the hash order.

**Implementation:** Reuse the `hash_spec` from `chain.rs` where possible.
For snapshot integrity, define a `dual_hash_spec` that hashes two inputs
and a `combined_hash_spec` that hashes two hashes. The round-trip proof
follows from determinism. Tamper detection follows from collision resistance.

### 3. Model constant-time comparison as a sequence fold

**Choice:** Specify constant_time_eq as a fold over byte positions that
accumulates XOR differences, then checks the accumulator.

**Rationale:** The spec must capture both correctness (equivalence to
equality) and the structural property (all bytes examined). A fold-based
spec naturally expresses "process every element" — the fold visits indices
0 through 31 regardless of intermediate values. The equivalence proof
shows the XOR accumulator is zero iff all bytes match.

**Alternative:** Specify as just `a == b` and trust the implementation
is constant-time. Rejected because the whole point of verifying this
function is to prove the structural property, not just correctness.

**Implementation:** Two-layer spec: `xor_fold_spec` computes the
accumulator, `constant_time_eq_spec` checks it equals zero. Proof that
`xor_fold_spec == 0 <==> pointwise_equal` connects the structural
definition to the semantic one.

### 4. Three separate modules, not one combined module

**Choice:** `kv_version.rs`, `snapshot_integrity.rs`, `constant_time_eq.rs`
as separate modules.

**Rationale:** Each module has independent concerns, independent test
suites, and independent proof obligations. Combining them into a single
`kv_storage.rs` would make the file large and conflate unrelated proof
contexts. The existing codebase follows one-concept-per-module (e.g.,
`cas.rs`, `chain.rs`, `expiration.rs`).

**Alternative:** Single `kv_storage.rs` mirroring aspen-redb-storage's
`verified/` layout. Rejected for the reasons above.

**Implementation:** Each module follows the standard pattern: spec fn →
exec fn with ensures → proof fns → proptest block.

### 5. Cross-module properties in properties.rs

**Choice:** Add properties linking the new modules to existing ones in
`src/properties.rs`.

**Rationale:** The interesting composition properties cross module
boundaries: "KV version's mod_revision satisfies monotonic::is_advance
when log indices increase," "snapshot integrity's hash axioms are
consistent with chain.rs's hash axioms." These belong in the existing
cross-module properties file.

**Implementation:**
- `kv_version` + `monotonic`: version field satisfies `is_advance`
- `kv_version` + `expiration`: TTL composition via `expires_at`
- `snapshot_integrity` + `chain`: shared hash axiom consistency
- `constant_time_eq` + `chain`: used by chain verification

## Risks / Trade-offs

**[Axiom consistency]** Adding more hash axioms (for snapshot integrity)
increases the trusted base. Mitigated by reusing the same axiom pattern
from `chain.rs` — no new axiom shapes, just new instances.

**[u64 vs i64 mismatch]** aspen uses i64 for version fields. Callers
need a cast. Mitigated by documenting the conversion and noting that
version values are always non-negative in practice.

**[Constant-time not machine-level]** Verus proves the algorithm visits
all bytes, but cannot prove the compiler doesn't optimize the XOR fold
into a short-circuiting comparison. This is a known limitation —
constant-time guarantees require assembly-level verification or compiler
barriers. The Verus proof still catches the most common bug class:
accidentally writing `==` instead of the XOR fold.
