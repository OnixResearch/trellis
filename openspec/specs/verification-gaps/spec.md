# Verification Gaps Specification

## Purpose

Tracks every trusted or weakened proof boundary in the current tree, not
just a historical subset of `#[verifier::external_body]` items. The goal
is to keep the trust surface explicit: every `external_body`,
`assume_specification[...]` bridge, `assume(...)` oracle,
`@verified-logic:no-ensures` exemption, or placeholder/weakened spec is a
place where the proof story is weaker than the fully verified
`exec fn ensures result == spec(...)` pattern.

## Current Status

As of `2026-04-15`, fresh reruns on the current tree show:

- `cargo clippy --all-targets -- -D warnings` passes
- `cargo test` passes (`3696 passed`)
- `./scripts/verus.sh src/lib.rs` passes (`6910 verified, 0 errors`)
- `tracey query uncovered` returns `0 uncovered out of 1488 rules`
- `tracey query untested` returns `0 untested out of 1488 rules`
- `./scripts/verify.sh` passes on this tree; direct Tracey query outputs remain archived as the coverage source of truth

These green direct checks do **not** mean the trust surface is empty. The
following inventory is the current audited boundary.

## Current Inventory

### `#[verifier::external_body]`

`16` external_body items remain across `6` files.

Each trusted site is listed individually below so the inventory matches the
actual tree and not just per-file totals.

| Site | Classification | Rationale | Test coverage |
|---|---|---|---|
| `src/chain.rs:42 hash_deterministic` | Permanent | Determinism axiom for closed `hash_spec`; this is the cryptographic assumption boundary. | `src/chain.rs` unit tests + proptest exercise the runtime hash wrapper and chain properties. |
| `src/chain.rs:70 compute_entry_hash` | Permanent | Exec wrapper is trusted because its `ensures` targets the closed `hash_spec`. | `src/chain.rs` unit tests + proptest exercise the runtime hash wrapper and chain properties. |
| `src/chain.rs:207 hash_collision_free` | Permanent | Collision-resistance axiom for the closed hash model. | `src/chain.rs` unit tests + proptest exercise the runtime hash wrapper and chain properties. |
| `src/snapshot_integrity.rs:43 dual_hash_deterministic` | Permanent | Determinism axiom for uninterpreted `dual_hash_spec`. | `src/snapshot_integrity.rs` unit tests + proptest exercise round-trip and tamper detection. |
| `src/snapshot_integrity.rs:54 dual_hash_collision_resistant` | Permanent | Collision-resistance axiom for the dual-hash model. | `src/snapshot_integrity.rs` unit tests + proptest exercise round-trip and tamper detection. |
| `src/snapshot_integrity.rs:63 dual_hash` | Permanent | Exec wrapper is trusted because its `ensures` targets uninterpreted `dual_hash_spec`. | `src/snapshot_integrity.rs` unit tests + proptest exercise round-trip and tamper detection. |
| `src/nix_version.rs:207 cv_fuel_antisymmetric` | Permanent | Recursive `cv_fuel` unfolding still exceeds solver limits for antisymmetry. | `src/nix_version.rs` unit tests + proptest cover version ordering. |
| `src/nix_version.rs:267 cv_fuel_reflexive` | Permanent | Recursive `cv_fuel` unfolding still exceeds solver limits for reflexivity. | `src/nix_version.rs` unit tests + proptest cover version ordering. |
| `src/nix_version.rs:300 cv_fuel_range` | Permanent | Recursive `cv_fuel` unfolding still exceeds solver limits for range reasoning. | `src/nix_version.rs` unit tests + proptest cover version ordering. |
| `src/nix_version.rs:365 compare_versions` | Permanent | Iterative exec/spec bridge still hits the same `cv_fuel` unfolding limit. | `src/nix_version.rs` unit tests + proptest cover version ordering. |
| `src/nix_store_path.rs:27 sha256_deterministic` | Permanent | Hash determinism axiom for uninterpreted SHA-256 model. | `src/nix_store_path.rs` unit tests + proptest cover store-path roundtrips. |
| `src/nix_store_path.rs:33 sha256_collision_resistant` | Permanent | Collision-resistance axiom for uninterpreted SHA-256 model. | `src/nix_store_path.rs` unit tests + proptest cover store-path roundtrips. |
| `src/web_of_trust_principal.rs:95 blake3_digest_bridge` | Permanent | Trusted runtime bridge from canonical public-key bytes to the uninterpreted full-length BLAKE3 principal-ID digest model. | `src/web_of_trust_principal.rs` unit tests + proptest cover principal-ID derivation and equality. |
| `src/log_invariant_inductive.rs:174 leader_append_preserves_lmp` | Permanent | Sound only under the system-level Raft leader-chain invariant, not as a local lemma. | `src/log_invariant_inductive.rs` unit tests + proptest cover valid log-shape behavior. |
| `src/log_invariant_inductive.rs:223 leader_append_preserves_lmp_right` | Permanent | Same system-level leader-chain invariant gap as the left-extension lemma. | `src/log_invariant_inductive.rs` unit tests + proptest cover valid log-shape behavior. |
| `src/log_invariant_inductive.rs:275 leader_invariant_preserves_lmp` | Permanent | Same system-level leader-chain invariant gap as the append lemmas. | `src/log_invariant_inductive.rs` unit tests + proptest cover valid log-shape behavior. |

Summary:

- Permanent external-body items: `16`
- Closeable external-body items: `0`

### `assume_specification[...]` exec bridges

`3` closeable `assume_specification[...]` bridges remain, all in
`src/ssa_construction.rs`.

These are trusted bridge declarations from thin verified wrappers to
plain-Rust SSA runtime helpers. They are closeable proof debt, not
architectural trust boundaries.

| Site | Classification | Rationale | Test coverage |
|---|---|---|---|
| `src/ssa_construction.rs:195 runtime_place_phis` | Closeable | The verified wrapper only preserves block/edge lengths. Exact phi placement and predecessor coverage are checked only by executable oracles. | `test_place_phis_diamond`, `prop_place_phis_hits_manual_idf`. |
| `src/ssa_construction.rs:197 runtime_construct_ssa` | Closeable | The verified wrapper only preserves block/edge lengths. Single-assignment and rename correctness stay in runtime checks/tests. | `test_construct_ssa_simple_branch`, `prop_constructed_program_is_ssa`. |
| `src/ssa_construction.rs:199 runtime_check_dominance_of_uses_with_dom` | Closeable | Dominance-of-use checking is delegated to a plain-Rust predicate instead of a proved Verus argument. | `test_dominance_of_uses_branch_phi`, `prop_constructed_program_has_dominated_uses`. |

Summary:

- Permanent `assume_specification[...]` bridges: `0`
- Closeable `assume_specification[...]` bridges: `3`

Overall live inventory: `19` trusted/weakened boundaries = `16` permanent + `3` closeable.

### `assume(...)` soundness oracles

`0` soundness oracles remain in verified code.

No verified code in `src/` currently ends in an `assume(...)` soundness oracle.

### `@verified-logic:no-ensures` and placeholder surfaces

`0` weakened exec/spec boundaries remain in those categories.

No public verified functions in `src/` currently rely on
`@verified-logic:no-ensures`, and no placeholder specs remain in the live
tree.

## Requirements

### Requirement: Proof Audit Inventory Covers All Trusted Boundaries

The proof-gap inventory MUST track every trusted or weakened proof
boundary in the current tree, not only a historical subset of
`#[verifier::external_body]` items.

Tracked categories MUST include:

- `#[verifier::external_body]`
- `assume_specification[...]` exec bridges
- `assume(...)` soundness oracles
- public functions exempted with `@verified-logic:no-ensures`
- public verified functions that actually lack an `ensures` clause
- placeholder or simplified specs that weaken exec/spec correspondence

#### Scenario: Mixed trust surfaces exist

- GIVEN the tree contains `external_body`, `assume_specification[...]`,
  `assume(...)`, or `@verified-logic:no-ensures` sites
- WHEN the proof-gap inventory is refreshed
- THEN every site appears in the inventory with file path,
  classification, and rationale

#### Scenario: Inventory totals are published

- GIVEN the inventory has been regenerated from the current tree
- WHEN `README.md` or the main `verification-gaps` spec reports totals
- THEN the published totals match the regenerated inventory

### Requirement: Proof Status Reports Are Evidence Backed

The project MUST NOT claim a fully green proof posture unless the live
verification commands support that claim.

#### Scenario: Verification pipeline is failing

- GIVEN `./scripts/verify.sh` reports failing Clippy, Verus, or Tracey gates
- WHEN the proof status is documented
- THEN the documentation reports the failing gates and affected modules
  instead of claiming all checks are green

#### Scenario: Verification pipeline is green

- GIVEN `./scripts/verify.sh` passes and Tracey reports zero uncovered
  and zero untested rules
- WHEN the proof status is documented
- THEN the documentation may describe the proof posture as green while
  still reporting the audited trust surface

### Requirement: Proof Audit Tracks Tracey Coverage Drift

Tracey uncovered and untested rules discovered during a proof audit MUST
be treated as remediation items until cleared.

#### Scenario: Uncovered rules are present

- GIVEN `tracey query uncovered` returns one or more rule IDs
- WHEN the proof audit is recorded
- THEN the remediation plan lists those rule IDs and the missing
  implementation references to add

#### Scenario: Untested rules are present

- GIVEN `tracey query untested` returns one or more rule IDs
- WHEN the proof audit is recorded
- THEN the remediation plan lists those rule IDs and the missing
  verification references to add

#### Scenario: Tracey is green again

- GIVEN the missing `r[impl]` and `r[verify]` references have been added
- WHEN Tracey is re-run
- THEN the audit records `0 uncovered` and `0 untested`

### Requirement: Ensure Checker Distinguishes Real Gaps From Parser Limits

The contract-checking tooling MUST report real missing postconditions
without flagging long valid signatures as missing `ensures`.

#### Scenario: Long verified signature

- GIVEN a public verified function whose `ensures` clause appears many
  lines after the `pub fn`
- WHEN `scripts/check-ensures.sh` runs
- THEN the function is accepted as having an `ensures` clause

#### Scenario: Real missing ensures

- GIVEN a public verified function with no `ensures` clause and no
  exemption marker
- WHEN `scripts/check-ensures.sh` runs
- THEN the function is reported as a real contract gap

### Requirement: Permanent Trust Boundaries Stay Separable From Closeable Ones

Permanent architectural trust boundaries MUST be distinguished from
closeable proof debt.

#### Scenario: Permanent item is inventoried

- GIVEN an item exists because of cryptographic assumptions, solver
  limits, or a system-level invariant that is intentionally modeled as trusted
- WHEN it is listed in the gap inventory
- THEN it is classified as permanent with a rationale

#### Scenario: Closeable item is inventoried

- GIVEN an item exists because a proof is incomplete, a loop/spec bridge
  is still trusted, or a placeholder spec remains in the tree
- WHEN it is listed in the gap inventory
- THEN it is classified as closeable with a rationale
