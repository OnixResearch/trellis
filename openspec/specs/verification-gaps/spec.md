# Verification Gaps Specification

## Purpose

Tracks every trusted or weakened proof boundary in the current tree, not
just a historical subset of `#[verifier::external_body]` items. The goal
is to keep the trust surface explicit: every `external_body`,
`assume(...)` oracle, `@verified-logic:no-ensures` exemption, or
placeholder spec is a place where the proof story is weaker than the
fully verified `exec fn ensures result == spec(...)` pattern.

## Current Status

As of `2026-04-13`, backed by fresh reruns on the current tree, the live verification pipeline is green:

- `./scripts/verify.sh` passes
- `cargo clippy --all-targets -- -D warnings` passes
- `cargo test` passes (`3402 passed`)
- `./scripts/verus.sh src/lib.rs` passes (`6112 verified, 0 errors`)
- Tracey is green (`1345/1345 covered`, `1345/1345 verified`, `0 uncovered`, `0 untested`)

This green status does **not** mean the trust surface is empty. The
following inventory is the current audited boundary.

## Current Inventory

### `#[verifier::external_body]`

`53` external_body items remain across `11` files.

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
| `src/log_invariant_inductive.rs:174 leader_append_preserves_lmp` | Permanent | Sound only under the system-level Raft leader-chain invariant, not as a local lemma. | `src/log_invariant_inductive.rs` unit tests + proptest cover valid log-shape behavior. |
| `src/log_invariant_inductive.rs:223 leader_append_preserves_lmp_right` | Permanent | Same system-level leader-chain invariant gap as the left-extension lemma. | `src/log_invariant_inductive.rs` unit tests + proptest cover valid log-shape behavior. |
| `src/log_invariant_inductive.rs:275 leader_invariant_preserves_lmp` | Permanent | Same system-level leader-chain invariant gap as the append lemmas. | `src/log_invariant_inductive.rs` unit tests + proptest cover valid log-shape behavior. |
| `src/shamir_field.rs:25 valid_modulus` | Closeable | Prime-modulus validator is currently a trusted exec/spec bridge; runtime trial division rejects composites. | `src/shamir_field.rs` unit tests cover prime/composite acceptance directly. |
| `src/shamir_field.rs:104 field_add` | Closeable | Initial Shamir field-add exec/spec bridge is trusted while runtime tests cover modular arithmetic identities. | `src/shamir_field.rs` unit tests + proptest cover closure, identity, commutativity, subtraction, inverse search, and prime-modulus validation. |
| `src/shamir_field.rs:117 field_sub` | Closeable | Initial Shamir field-sub exec/spec bridge is trusted while runtime tests cover additive-inverse behavior. | `src/shamir_field.rs` unit tests + proptest cover closure, identity, commutativity, subtraction, inverse search, and prime-modulus validation. |
| `src/shamir_field.rs:133 field_mul` | Closeable | Initial Shamir field-mul exec/spec bridge is trusted while runtime tests cover modular multiplication laws. | `src/shamir_field.rs` unit tests + proptest cover closure, identity, commutativity, subtraction, inverse search, and prime-modulus validation. |
| `src/shamir_field.rs:145 field_inv` | Closeable | Extended-GCD inverse computation is currently a trusted exec/spec bridge. | `src/shamir_field.rs` unit tests + proptest cover inverse witnesses on small primes. |
| `src/shamir_field.rs:190 field_add_zero` | Closeable | Additive-identity proof stub is currently trusted. | `src/shamir_field.rs` unit tests + proptest cover additive identity directly. |
| `src/shamir_field.rs:201 field_mul_one` | Closeable | Multiplicative-identity proof stub is currently trusted. | `src/shamir_field.rs` unit tests + proptest cover multiplicative identity directly. |
| `src/shamir_field.rs:212 field_commutative` | Closeable | Commutativity proof stub is currently trusted. | `src/shamir_field.rs` unit tests + proptest cover commutativity directly. |
| `src/shamir_field.rs:223 field_closed` | Closeable | Closure proof stub is currently trusted. | `src/shamir_field.rs` unit tests + proptest cover result range bounds directly. |
| `src/shamir_field.rs:235 field_sub_add_roundtrip` | Closeable | Subtraction roundtrip proof stub is currently trusted. | `src/shamir_field.rs` unit tests + proptest cover `(a-b)+b == a` directly. |
| `src/shamir_field.rs:244 field_inv_from_spec_correct` | Closeable | Inverse-witness proof helper is currently trusted. | `src/shamir_field.rs` unit tests + proptest cover returned inverse witnesses directly. |
| `src/shamir_field.rs:259 field_inv_spec_correct` | Closeable | Inverse-spec correctness proof is currently trusted. | `src/shamir_field.rs` unit tests + proptest cover returned inverse witnesses directly. |
| `src/shamir_field.rs:268 zero_has_no_inverse` | Closeable | Zero-inverse proof stub is currently trusted. | `src/shamir_field.rs` unit tests cover `field_inv(0, p) == None`. |
| `src/shamir_field.rs:277 field_mul_zero` | Closeable | Zero-multiplication proof stub is currently trusted. | `src/shamir_field.rs` unit tests + proptest exercise zero cases. |
| `src/shamir_poly.rs:39 poly_eval_from` | Closeable | Recursive Horner helper is currently a trusted exec/spec bridge. | `src/shamir_poly.rs` unit tests + proptest compare Horner evaluation with direct runtime summation. |
| `src/shamir_poly.rs:59 poly_eval_horner` | Closeable | Public Horner evaluator is currently a trusted exec/spec bridge. | `src/shamir_poly.rs` unit tests + proptest compare Horner evaluation with direct runtime summation. |
| `src/shamir_poly.rs:72 poly_degree_bound` | Closeable | Degree-bound proof stub is currently trusted. | `src/shamir_poly.rs` unit tests cover degree-vs-length behavior. |
| `src/shamir_poly.rs:80 constant_poly_eval` | Closeable | Constant-polynomial proof stub is currently trusted. | `src/shamir_poly.rs` unit tests cover constant evaluation directly. |
| `src/shamir_poly.rs:89 linear_poly_eval` | Closeable | Linear-polynomial proof stub is currently trusted. | `src/shamir_poly.rs` unit tests cover linear evaluation directly. |
| `src/shamir_poly.rs:98 poly_at_zero_is_constant` | Closeable | Constant-term proof stub is currently trusted. | `src/shamir_poly.rs` unit tests + proptest cover evaluation at zero directly. |
| `src/shamir_lagrange.rs:86 basis_self_at_point` | Closeable | Basis self-evaluation proof stub is currently trusted. | `src/shamir_lagrange.rs` unit tests + proptest cover interpolation and reconstruction behavior. |
| `src/shamir_lagrange.rs:94 basis_self_from` | Closeable | Basis suffix proof helper is currently trusted. | `src/shamir_lagrange.rs` unit tests + proptest cover interpolation and reconstruction behavior. |
| `src/shamir_lagrange.rs:104 basis_other_at_point` | Closeable | Basis off-diagonal proof stub is currently trusted. | `src/shamir_lagrange.rs` unit tests + proptest cover interpolation and reconstruction behavior. |
| `src/shamir_lagrange.rs:111 basis_other_from` | Closeable | Basis off-diagonal suffix helper is currently trusted. | `src/shamir_lagrange.rs` unit tests + proptest cover interpolation and reconstruction behavior. |
| `src/shamir_lagrange.rs:126 basis_partition_of_unity_at_point` | Closeable | Partition-of-unity proof stub is currently trusted. | `src/shamir_lagrange.rs` unit tests + proptest cover interpolation and reconstruction behavior. |
| `src/shamir_lagrange.rs:149 basis_sum_at_point_from` | Closeable | Partition-of-unity suffix helper is currently trusted. | `src/shamir_lagrange.rs` unit tests + proptest cover interpolation and reconstruction behavior. |
| `src/shamir_lagrange.rs:159 lagrange_eval_at_point` | Closeable | Interpolation correctness proof stub is currently trusted. | `src/shamir_lagrange.rs` unit tests + proptest cover interpolation and reconstruction behavior. |
| `src/shamir_lagrange.rs:170 lagrange_eval_at_point_from` | Closeable | Interpolation suffix helper is currently trusted. | `src/shamir_lagrange.rs` unit tests + proptest cover interpolation and reconstruction behavior. |
| `src/shamir_lagrange.rs:186 lagrange_polynomials_unique` | Closeable | Polynomial-coefficient uniqueness theorem is currently trusted. | `src/shamir_lagrange.rs` unit tests cover distinct linear polynomials differing on two points. |
| `src/shamir_lagrange.rs:199 basis_at_zero_from` | Closeable | Runtime basis-at-zero helper is currently a trusted exec/spec bridge. | `src/shamir_lagrange.rs` unit tests + proptest cover interpolation and reconstruction behavior. |
| `src/shamir_lagrange.rs:230 lagrange_at_zero_from` | Closeable | Runtime interpolation-at-zero helper is currently a trusted exec/spec bridge. | `src/shamir_lagrange.rs` unit tests + proptest cover interpolation and reconstruction behavior. |
| `src/shamir_lagrange.rs:260 lagrange_at_zero` | Closeable | Public interpolation-at-zero evaluator is currently a trusted exec/spec bridge. | `src/shamir_lagrange.rs` unit tests + proptest cover interpolation and reconstruction behavior. |
| `src/shamir_split.rs:65 split_constant_term_preserved` | Closeable | Split constant-term proof stub is currently trusted. | `src/shamir_split.rs` unit tests + proptest cover secret-at-zero, prime-modulus rejection, and share evaluation. |
| `src/shamir_split.rs:79 split_count_and_xs` | Closeable | Split count/x-coordinate proof stub is currently trusted. | `src/shamir_split.rs` unit tests + proptest cover output count and x-coordinate preservation. |
| `src/shamir_split.rs:119 split` | Closeable | Public share-generation exec/spec bridge is currently trusted. | `src/shamir_split.rs` unit tests + proptest cover parameter validation and share generation. |
| `src/shamir_reconstruct.rs:31 reconstruct` | Closeable | Public reconstruction exec/spec bridge is currently trusted. | `src/shamir_reconstruct.rs` unit tests + proptest cover threshold round-trips and insufficient-share behavior. |
| `src/shamir_threshold.rs:25 threshold_completion_exists` | Closeable | Threshold-completion existence proof stub is currently trusted. | `src/shamir_threshold.rs` unit tests cover single-share completion to arbitrary secrets. |
| `src/shamir_threshold.rs:44 threshold_completion_unique` | Closeable | Threshold uniqueness proof stub is currently trusted. | `src/shamir_threshold.rs` unit tests cover threshold subsets fixing the secret. |

Summary:

- Permanent external-body items: `15`
- Closeable external-body items: `38`

### `assume(...)` soundness oracles

`0` soundness oracles remain in verified code.

No verified code in `src/` currently ends in an `assume(...)` soundness oracle.

### `@verified-logic:no-ensures` and placeholder surfaces

`0` weakened exec/spec boundaries remain.

No public verified functions in `src/` currently rely on `@verified-logic:no-ensures`, and no placeholder specs remain in the live tree.

## Requirements

### Requirement: Proof Audit Inventory Covers All Trusted Boundaries

The proof-gap inventory MUST track every trusted or weakened proof
boundary in the current tree, not only a historical subset of
`#[verifier::external_body]` items.

Tracked categories MUST include:

- `#[verifier::external_body]`
- `assume(...)` soundness oracles
- public functions exempted with `@verified-logic:no-ensures`
- public verified functions that actually lack an `ensures` clause
- placeholder or simplified specs that weaken exec/spec correspondence

#### Scenario: Mixed trust surfaces exist

- GIVEN the tree contains `external_body`, `assume(...)`, and
  `@verified-logic:no-ensures` sites
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
