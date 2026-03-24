# KV Storage Primitives Specification

## Purpose

Defines correctness properties for KV storage primitives extracted from
aspen-redb-storage: entry version computation, snapshot integrity
verification, and constant-time hash comparison. These are pure decision
functions — no I/O, no database access, time and log index passed
explicitly.

## Requirements

### KV Entry Versioning

#### Requirement: New Key Initialization

When creating a new key (no existing version), `compute_kv_versions(None,
log_index)` MUST return `create_revision == log_index`, `mod_revision ==
log_index`, and `version == 1`.

##### Scenario: First write at log index 100

- GIVEN no existing entry for the key
- WHEN compute_kv_versions is called with log_index=100
- THEN create_revision MUST be 100, mod_revision MUST be 100, version MUST be 1

##### Scenario: First write at log index 0

- GIVEN no existing entry
- WHEN compute_kv_versions is called with log_index=0
- THEN create_revision MUST be 0, mod_revision MUST be 0, version MUST be 1

#### Requirement: Update Preserves Create Revision

When updating an existing key, `create_revision` MUST be preserved from
the existing entry. It MUST NOT change to the new log_index.

##### Scenario: Second write

- GIVEN an existing entry with create_revision=50, version=3
- WHEN compute_kv_versions is called with log_index=100
- THEN create_revision MUST be 50 (preserved), mod_revision MUST be 100

#### Requirement: Update Advances Mod Revision

On every update, `mod_revision` MUST equal the current `log_index`.
For new keys, `mod_revision` MUST also equal `log_index`.

##### Scenario: Update at index 200

- GIVEN an existing entry with create_revision=50
- WHEN compute_kv_versions is called with log_index=200
- THEN mod_revision MUST be 200

#### Requirement: Version Increments on Update

On every update to an existing key, `version` MUST increment by exactly
one (saturating at the maximum value). For new keys, version MUST be 1.

##### Scenario: Third update

- GIVEN an existing entry with version=3
- WHEN compute_kv_versions is called
- THEN version MUST be 4

##### Scenario: Overflow protection

- GIVEN an existing entry with version=i64::MAX
- WHEN compute_kv_versions is called
- THEN version MUST be i64::MAX (saturated, no wrap)

#### Requirement: Version Always Positive

`version` MUST be ≥ 1 for any output of `compute_kv_versions`. The
version field MUST never be zero or negative.

##### Scenario: New key

- GIVEN no existing entry
- WHEN compute_kv_versions is called
- THEN version MUST be 1

##### Scenario: After many updates

- GIVEN an existing entry with any valid version ≥ 1
- WHEN compute_kv_versions is called
- THEN version MUST be ≥ 2

#### Requirement: Deterministic

The same inputs MUST always produce the same output. The function MUST
be pure — no randomness, no clock, no side effects.

##### Scenario: Repeated calls

- GIVEN identical inputs (existing=(50, 3), log_index=100)
- WHEN compute_kv_versions is called twice
- THEN both results MUST be identical

#### Requirement: Mod Revision Monotonic

If updates are applied with strictly increasing log indices, `mod_revision`
MUST be strictly increasing across successive calls.

##### Scenario: Sequential updates

- GIVEN updates at log_index 10, 20, 30
- WHEN each update is applied in order
- THEN mod_revision values MUST be 10, 20, 30 (strictly increasing)

### Snapshot Integrity

#### Requirement: Compute-Verify Round Trip

`SnapshotIntegrity::compute(meta, data, chain_hash)` followed by
`verify(meta, data)` MUST always return true. A freshly computed
integrity value MUST pass its own verification.

##### Scenario: Basic round trip

- GIVEN meta_bytes and data_bytes
- WHEN integrity is computed and then verified with the same inputs
- THEN verify MUST return true

##### Scenario: Empty inputs

- GIVEN empty meta_bytes and empty data_bytes
- WHEN integrity is computed and verified
- THEN verify MUST return true

#### Requirement: Data Tamper Detection

If the `data` bytes are modified after `compute`, `verify` with the
modified data MUST return false.

##### Scenario: Modified data

- GIVEN integrity computed from original data
- WHEN verify is called with different data bytes
- THEN verify MUST return false

#### Requirement: Metadata Tamper Detection

If the `meta_bytes` are modified after `compute`, `verify` with the
modified metadata MUST return false.

##### Scenario: Modified metadata

- GIVEN integrity computed from original metadata
- WHEN verify is called with different meta bytes
- THEN verify MUST return false

#### Requirement: Chain Hash Continuity

When `verify_with_chain_hash` is called with an expected chain hash,
verification MUST fail if the stored `chain_hash_at_snapshot` does not
match the expected value.

##### Scenario: Matching chain hash

- GIVEN integrity computed with chain_hash=H
- WHEN verify_with_chain_hash is called with expected=H
- THEN verification MUST succeed (assuming data/meta unchanged)

##### Scenario: Mismatched chain hash

- GIVEN integrity computed with chain_hash=H1
- WHEN verify_with_chain_hash is called with expected=H2 where H1 ≠ H2
- THEN verification MUST fail

#### Requirement: Combined Hash Commits to Both Components

The `combined_hash` field MUST depend on both `data_hash` and
`meta_hash`. Changing either component MUST change the combined hash
(assuming collision resistance).

##### Scenario: Different data, same meta

- GIVEN two integrity values with different data but same metadata
- WHEN comparing combined_hash values
- THEN they MUST differ (under collision resistance)

#### Requirement: Hash Function Axioms

The hash function used for snapshot integrity MUST be deterministic.
The same input MUST always produce the same hash. Collision resistance
is assumed as a trusted axiom (same as `chain.rs`).

##### Scenario: Determinism

- GIVEN the same data bytes
- WHEN hashed twice
- THEN both hashes MUST be identical

### Constant-Time Comparison

#### Requirement: Equal Inputs Return True

When both 32-byte inputs are identical, `constant_time_eq` MUST return
true.

##### Scenario: All zeros

- GIVEN two arrays of 32 zero bytes
- WHEN constant_time_eq is called
- THEN result MUST be true

##### Scenario: All ones

- GIVEN two arrays of 32 0xFF bytes
- WHEN constant_time_eq is called
- THEN result MUST be true

#### Requirement: Any Difference Returns False

If the inputs differ at any byte position, `constant_time_eq` MUST
return false.

##### Scenario: Single bit difference

- GIVEN two arrays differing at exactly one byte
- WHEN constant_time_eq is called
- THEN result MUST be false

##### Scenario: All bytes different

- GIVEN two arrays where every byte differs
- WHEN constant_time_eq is called
- THEN result MUST be false

#### Requirement: Equivalence to Byte Equality

`constant_time_eq(a, b)` MUST return true if and only if `a == b`
(pointwise byte equality). The function MUST be a correct equality
check, not just a timing-safe approximation.

##### Scenario: Agreement with direct comparison

- GIVEN any two 32-byte arrays a, b
- WHEN constant_time_eq(a, b) and a == b are both evaluated
- THEN they MUST agree

#### Requirement: Fixed Iteration Count

The comparison MUST examine all 32 bytes regardless of where a mismatch
occurs. There MUST be no early-exit optimization. The XOR-fold pattern
(accumulate XOR differences, check accumulator == 0) MUST be used.

##### Scenario: Mismatch at first byte

- GIVEN arrays differing at byte 0
- WHEN constant_time_eq is called
- THEN all 32 bytes MUST still be examined (no short-circuit)

##### Scenario: Mismatch at last byte

- GIVEN arrays differing only at byte 31
- WHEN constant_time_eq is called
- THEN the function MUST detect the difference
