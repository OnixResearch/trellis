# Verification Gaps Specification

## Purpose

Tracks every trusted or weakened proof boundary in the current tree, not
just a historical subset of `#[verifier::external_body]` items. The goal
is to keep the trust surface explicit: every `external_body`,
`assume_specification[...]` bridge, `assume(...)` oracle,
`@verified-logic:no-ensures` exemption, runtime-only public API exemption, or
placeholder/weakened spec is a place where the proof story is weaker than the
fully verified `exec fn ensures result == spec(...)` pattern.

## Current Status

As of `2026-04-15`, fresh reruns on the current tree show:

- `cargo clippy --all-targets -- -D warnings` passes
- `cargo test` passes (`3705 passed`)
- `./scripts/verus.sh src/lib.rs` passes (`6940 verified, 0 errors`)
- `tracey query uncovered` returns `0 uncovered out of 1491 rules`
- `tracey query untested` returns `0 untested out of 1491 rules`
- `./scripts/verify.sh` passes on this tree; direct Tracey query outputs remain archived as the coverage source of truth

These green direct checks do **not** mean the trust surface is empty. The
following inventory is the current audited boundary.

## Current Inventory

### `#[verifier::external_body]`

`33` external_body items remain across `14` files.

Each trusted site is listed individually below so the inventory matches the
actual tree and not just per-file totals.

| Site | Classification | Rationale | Test coverage |
|---|---|---|---|
| `src/chain.rs:42 hash_deterministic` | Permanent | Determinism axiom for closed `hash_spec`; this is the cryptographic assumption boundary. | `src/chain.rs` unit tests + proptest exercise the runtime hash wrapper and chain properties. |
| `src/chain.rs:70 compute_entry_hash` | Permanent | Exec wrapper is trusted because its `ensures` targets the closed `hash_spec`. | `src/chain.rs` unit tests + proptest exercise the runtime hash wrapper and chain properties. |
| `src/chain.rs:207 hash_collision_free` | Permanent | Collision-resistance axiom for the closed hash model. | `src/chain.rs` unit tests + proptest exercise the runtime hash wrapper and chain properties. |
| `src/log_invariant_inductive.rs:174 leader_append_preserves_lmp` | Permanent | Sound only under the system-level Raft leader-chain invariant, not as a local lemma. | `src/log_invariant_inductive.rs` unit tests + proptest cover valid log-shape behavior. |
| `src/log_invariant_inductive.rs:223 leader_append_preserves_lmp_right` | Permanent | Same system-level leader-chain invariant gap as the left-extension lemma. | `src/log_invariant_inductive.rs` unit tests + proptest cover valid log-shape behavior. |
| `src/log_invariant_inductive.rs:275 leader_invariant_preserves_lmp` | Permanent | Same system-level leader-chain invariant gap as the append lemmas. | `src/log_invariant_inductive.rs` unit tests + proptest cover valid log-shape behavior. |
| `src/nix_store_path.rs:27 sha256_output_len` | Permanent | SHA-256 output length is an external hash-model axiom. | `src/nix_store_path.rs` unit tests + proptest cover store-path roundtrips. |
| `src/nix_store_path.rs:33 sha256_deterministic` | Permanent | Hash determinism axiom for uninterpreted SHA-256 model. | `src/nix_store_path.rs` unit tests + proptest cover store-path roundtrips. |
| `src/nix_version.rs:207 cv_fuel_antisymmetric` | Permanent | Recursive `cv_fuel` unfolding still exceeds solver limits for antisymmetry. | `src/nix_version.rs` unit tests + proptest cover version ordering. |
| `src/nix_version.rs:267 cv_fuel_reflexive` | Permanent | Recursive `cv_fuel` unfolding still exceeds solver limits for reflexivity. | `src/nix_version.rs` unit tests + proptest cover version ordering. |
| `src/nix_version.rs:300 cv_fuel_range` | Permanent | Recursive `cv_fuel` unfolding still exceeds solver limits for range reasoning. | `src/nix_version.rs` unit tests + proptest cover version ordering. |
| `src/nix_version.rs:365 compare_versions` | Permanent | Iterative exec/spec bridge still hits the same `cv_fuel` unfolding limit. | `src/nix_version.rs` unit tests + proptest cover version ordering. |
| `src/snapshot_integrity.rs:43 dual_hash_deterministic` | Permanent | Determinism axiom for uninterpreted `dual_hash_spec`. | `src/snapshot_integrity.rs` unit tests + proptest exercise round-trip and tamper detection. |
| `src/snapshot_integrity.rs:54 dual_hash_collision_resistant` | Permanent | Collision-resistance axiom for the dual-hash model. | `src/snapshot_integrity.rs` unit tests + proptest exercise round-trip and tamper detection. |
| `src/snapshot_integrity.rs:63 dual_hash` | Permanent | Exec wrapper is trusted because its `ensures` targets uninterpreted `dual_hash_spec`. | `src/snapshot_integrity.rs` unit tests + proptest exercise round-trip and tamper detection. |
| `src/ucan_ability.rs:107 ability_subsumes` | Closeable | Runtime-only ability subsumption has no Verus postcondition; proofs currently target `ability_subsumes_spec`. | `ucan_ability` tests and proof lemmas cover wildcard, reflexive, and monotone containment behavior. |
| `src/ucan_cid.rs:314 axiom_cid_encode_round_trip` | Permanent | Multibase CID encode/decode is modeled as an uninterpreted external codec relation. | `ucan_cid` tests cover CID constants; proof lemmas cover CID byte construction and round-trip use of the axiom. |
| `src/ucan_did.rs:177 varint_decode` | Closeable | Runtime varint decoder is bridged to `varint_decode_spec` instead of proved internally. | `ucan_did` tests cover varint round-trip and truncated-continuation rejection. |
| `src/ucan_did.rs:269 axiom_base58btc_round_trip` | Permanent | Base58btc round-trip is an external codec-model axiom. | `ucan_did` tests cover DID encoding and base58btc round-trip behavior. |
| `src/ucan_did.rs:277 axiom_base58btc_injective` | Permanent | Base58btc injectivity is an external codec-model axiom. | `ucan_did` tests cover distinct DID/key encodings and proof lemmas consume the axiom. |
| `src/ucan_did.rs:286 base58btc_encode` | Permanent | Runtime base58btc encoder is trusted against the uninterpreted base58btc spec. | `ucan_did` tests cover DID encoding and base58btc round-trip behavior. |
| `src/ucan_did.rs:295 base58btc_decode` | Permanent | Runtime base58btc decoder is trusted against the uninterpreted base58btc spec. | `ucan_did` tests cover DID decoding specs and base58btc round-trip behavior. |
| `src/ucan_principal_link.rs:28 axiom_did_encode_extract_round_trip` | Closeable | Cross-module DID encode/extract relation is axiomatized instead of linked to the concrete DID spec. | `ucan_principal_link` proofs cover same-key principal derivation through the axiom. |
| `src/ucan_signature.rs:132 axiom_sign_verify_round_trip` | Permanent | Ed25519 sign/verify correctness is a cryptographic contract axiom. | `ucan_signature` tests/proofs cover key length, signature length, round-trip, key binding, and message binding. |
| `src/ucan_signature.rs:141 axiom_wrong_key_rejects` | Permanent | Wrong-key rejection is a cryptographic contract axiom. | `ucan_signature` tests/proofs cover wrong-key rejection. |
| `src/ucan_signature.rs:153 axiom_wrong_message_rejects` | Permanent | Wrong-message rejection is a cryptographic contract axiom. | `ucan_signature` tests/proofs cover wrong-message rejection. |
| `src/ucan_signature.rs:167 sign` | Permanent | Runtime Ed25519 signing implementation is trusted against the signature model. | `ucan_signature` tests/proofs cover sign/verify round-trip and output length. |
| `src/ucan_signature.rs:181 verify` | Permanent | Runtime Ed25519 verification implementation is trusted against the signature model. | `ucan_signature` tests/proofs cover round-trip, wrong-key, and wrong-message behavior. |
| `src/ucan_signature.rs:193 public_from_secret` | Permanent | Runtime public-key derivation is trusted against the signature model. | `ucan_signature` tests/proofs cover public-key length and signing round-trip. |
| `src/ucan_token.rs:180 check_attenuation_chain` | Closeable | Runtime iterator-based attenuation check is bridged to a quantified spec instead of proved internally. | `ucan_token` tests/proofs cover attenuation monotonicity through validation. |
| `src/web_of_trust_attestation.rs:307 attestation_signature_verify_bridge` | Permanent | Trusted runtime bridge for attestation signature verification over canonical payload bytes. | `web_of_trust_attestation` tests/proofs cover payload construction and signature validity behavior. |
| `src/web_of_trust_principal.rs:97 blake3_digest_bridge` | Permanent | Trusted runtime bridge from canonical public-key bytes to the uninterpreted full-length BLAKE3 principal-ID digest model. | `src/web_of_trust_principal.rs` unit tests + proptest cover principal-ID derivation and equality. |
| `src/web_of_trust_revocation.rs:379 revocation_signature_verify_bridge` | Permanent | Trusted runtime bridge for revocation signature verification over canonical payload bytes. | `web_of_trust_revocation` tests/proofs cover payload construction and revocation signature behavior. |

Summary:

- Permanent external-body items: `29`
- Closeable external-body items: `4`

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

### Runtime-only public API exemptions

`8` closeable runtime-only public API exemptions remain, all in
`src/instruction_selection.rs`.

These public APIs are intentionally outside the verified `r[impl]` surface for
now. They are closeable proof debt, not permanent trust boundaries: the module
can later be converted to the normal `spec fn` plus verified `exec fn` pattern.

| Site | Classification | Rationale | Test coverage |
|---|---|---|---|
| `src/instruction_selection.rs:83 is_well_formed_tree` | Closeable runtime-only | Plain-Rust tree-shape predicate has runtime recursion and no Verus spec/exec correspondence yet. | Focused `instruction_selection` unit tests cover malformed trees, overflow rejection, tiling, coverage, and soundness checks. |
| `src/instruction_selection.rs:94 is_valid_tile_pattern` | Closeable runtime-only | Plain-Rust tile-pattern validator has no Verus spec/exec correspondence yet. | Focused `instruction_selection` unit tests cover valid exact/fused tiles and invalid root `Any` rejection. |
| `src/instruction_selection.rs:119 subtree_eval` | Closeable runtime-only | Plain-Rust recursive expression evaluator has checked arithmetic but no Verus spec/exec correspondence yet. | Focused `instruction_selection` unit tests cover reference arithmetic and overflow rejection. |
| `src/instruction_selection.rs:147 tiling_cost` | Closeable runtime-only | Plain-Rust tiling-cost projection is guarded by runtime well-formedness but has no Verus contract. | Focused `instruction_selection` unit tests cover cost projection and malformed-tree rejection through the public surface. |
| `src/instruction_selection.rs:155 tile_matches` | Closeable runtime-only | Plain-Rust tile matcher and uncovered-root computation have no Verus spec/exec correspondence yet. | Focused `instruction_selection` unit tests cover single-node matches, deep tiles, mismatches, arity errors, and literal mismatches. |
| `src/instruction_selection.rs:201 optimal_tiling` | Closeable runtime-only | Plain-Rust dynamic-programming tiler has no Verus proof of optimality, coverage, or tie-break determinism yet. | Focused `instruction_selection` unit tests cover missing tiles, lower-cost choice, tie-breaking, and repeated-run determinism. |
| `src/instruction_selection.rs:211 check_coverage` | Closeable runtime-only | Plain-Rust tiling coverage checker has no Verus spec/exec correspondence yet. | Focused `instruction_selection` unit tests cover complete coverage acceptance and duplicate/path failure behavior through the runtime checker. |
| `src/instruction_selection.rs:238 check_soundness` | Closeable runtime-only | Plain-Rust tiling soundness checker has no Verus proof tying tile semantics to expression semantics yet. | Focused `instruction_selection` unit tests cover soundness acceptance and arithmetic surface behavior. |

Summary:

- Permanent runtime-only public API exemptions: `0`
- Closeable runtime-only public API exemptions: `8`

Overall live inventory: `44` trusted/weakened/runtime-only proof boundaries = `29` permanent + `15` closeable.

### `assume(...)` soundness oracles

`0` soundness oracles remain in verified code.

No verified code in `src/` currently ends in an `assume(...)` soundness oracle.

### `@verified-logic:no-ensures` and placeholder surfaces

`1` weakened exec/spec boundary remains in those categories, already counted
under `#[verifier::external_body]` above.

`src/ucan_ability.rs:107 ability_subsumes` carries
`@verified-logic:no-ensures` because proofs currently target
`ability_subsumes_spec` while the runtime helper remains external-body
runtime-only. No placeholder specs remain in the live tree.

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
- runtime-only public functions exempted with `@verified-logic:exempt`
- public verified functions that actually lack an `ensures` clause
- placeholder or simplified specs that weaken exec/spec correspondence

#### Scenario: Mixed trust surfaces exist

- GIVEN the tree contains `external_body`, `assume_specification[...]`,
  `assume(...)`, `@verified-logic:no-ensures`, or runtime-only
  `@verified-logic:exempt` sites
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
