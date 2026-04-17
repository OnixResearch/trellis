## Phase 0: Scope Lock and Audit

- [x] Record the current-tree audit in `proposal.md` and `design.md` against `src/lib.rs`, `docs/requirements.md`, `README.md`, and `openspec/specs/`
- [x] Keep `proposal.md`, `design.md`, and `openspec/changes/web-of-trust-primitives/specs/web-of-trust/spec.md` aligned on the operational `trust` role, same-issuer revocation authority, exact-vs-family revocation targets, authenticated revocation effective time, the canonical attestation/revocation byte grammar, the root depth-`0` base case, threshold `k = 0` / `d = 0` rejection, and root-subject non-special-casing
- [x] Get explicit human review/sign-off for any use of `blake3_digest_bridge`, `attestation_signature_verify_bridge`, or `revocation_signature_verify_bridge`, and record the exact approved contracts in `openspec/changes/web-of-trust-primitives/evidence/trusted-bridge-review.md`

## Phase 1: Principal IDs, Payload Bytes, and Crypto Bridge Shape

- [x] Add `src/web_of_trust_principal.rs` with public spec/exec principal-ID wrappers, matching `ensures`, and BLAKE3 fingerprint helpers plus proofs/tests for `web-of-trust.principal-fingerprint`
- [x] Add explicit `proof fn` artifacts `principal_id_deterministic`, `principal_id_full_eq`, and `principal_id_matches_key_bytes`
- [x] Establish the bytes-to-`PrincipalId` invariant (`principal_id_from_bytes(canonical_key_bytes)`) and regressions covering full-32-byte equality behavior
- [x] Add explicit unit/proptest coverage for the `web-of-trust.principal-fingerprint` scenarios in `openspec/changes/web-of-trust-primitives/specs/web-of-trust/spec.md`
- [x] Add a proptest model-oracle for principal fingerprint determinism/full-equality
- [x] Add `src/web_of_trust_attestation.rs` with public spec/exec attestation wrappers, matching `ensures`, canonical attestation payload-byte builders, and exact signature-binding helpers plus proofs/tests for `web-of-trust.attestation-signature`
- [x] Add explicit `proof fn` artifact `attestation_signature_binds_payload`
- [x] Add explicit unit/proptest coverage for the `web-of-trust.attestation-signature` scenarios in `openspec/changes/web-of-trust-primitives/specs/web-of-trust/spec.md`
- [x] Add a proptest model-oracle for attestation payload/signature binding
- [x] Add explicit `proof fn` artifacts `attestation_payload_field_order`, `attestation_payload_little_endian`, `attestation_payload_tag_encoding`, and `attestation_payload_overlong_rejected`
- [x] Restrict trusted crypto bridges in this change to `blake3_digest_bridge`, `attestation_signature_verify_bridge`, and `revocation_signature_verify_bridge`; do not introduce any additional trusted bridge
- [x] Make any BLAKE3/signature bridge explicit; if an assume-spec boundary is needed, gate it with `#[cfg(verus_keep_ghost)]` so normal cargo builds stay green
- [x] Add `docs/requirements.md` entries plus explicit `r[impl]` / `r[verify]` annotations on the defining source helpers, proof fns/lemmas, and tests for `web-of-trust.principal-fingerprint` and `web-of-trust.attestation-signature`

## Phase 2: Activation Windows and Revocation

- [x] Add `src/web_of_trust_revocation.rs` with public spec/exec revocation wrappers, matching `ensures`, canonical revocation payload-byte builders, revocation signature-validation helpers/bridge, same-issuer authentication checks, exact-payload targets, and family-target revocation helpers plus proofs/tests for `web-of-trust.revocation`
- [x] Require and verify derived-`PrincipalId` equality across same-issuer checks and family-target matching
- [x] Add explicit `proof fn` artifacts `revocation_record_ids_match_bytes`, `invalid_revocation_never_effective`, `revocation_monotone`, `revocation_exact_payload_correct`, and `revocation_family_target_correct`
- [x] Add explicit `proof fn` artifacts `revocation_payload_field_order`, `revocation_payload_little_endian`, `revocation_family_target_encoding`, `revocation_exact_payload_target_encoding`, `revocation_target_kind_tags`, `revocation_signature_binds_payload`, `revocation_invalid_kind_rejected`, and `revocation_overlong_rejected`
- [x] Add active-window helpers/proofs for `web-of-trust.attestation-window`, including exact-payload, family-target, and wrong-issuer revocation behavior
- [x] Add explicit `proof fn` artifacts `attestation_record_ids_match_bytes`, `invalid_attestation_is_inactive`, `attestation_active_requires_signature`, `attestation_active_half_open_window`, and `attestation_active_respects_revocation`
- [x] Prove and test that non-canonical or bytes/ID-mismatched attestations and revocations are ignored by `attestation_active_at`
- [x] Add explicit unit/proptest coverage for the `web-of-trust.attestation-window` scenarios in `openspec/changes/web-of-trust-primitives/specs/web-of-trust/spec.md`, including revoked-at-time and wrong-issuer revocation behavior
- [x] Prove and test the canonical byte grammar for revocation payloads, including field order, little-endian encoding, target-kind tag assignments, and rejection of variable-length inputs longer than `u32::MAX`
- [x] Add explicit unit/proptest coverage for authenticated revocation payload/signature binding and the `web-of-trust.revocation` scenarios in `openspec/changes/web-of-trust-primitives/specs/web-of-trust/spec.md`, including exact-payload-copy disablement, family-reissue disablement, invalid-kind, tampered-issuer, tampered-target, tampered-time, unrelated-attestation, and wrong-issuer cases
- [x] Add direct regression tests that exact-payload revocation disables identical canonical-payload copies and that family-target revocation disables reissued attestations in the same `(issuer, subject, role)` family
- [x] Add a proptest model-oracle for revocation matching/effectiveness
- [x] Add an explicit revocation monotonicity proof/test showing that once effective, a revocation remains effective at all later times
- [x] Add `docs/requirements.md` entries plus explicit `r[impl]` / `r[verify]` annotations on the defining source helpers, proof fns/lemmas, and tests for `web-of-trust.attestation-window` and `web-of-trust.revocation`

## Phase 3: Chains, Rooted Resolution, and Cross-Module Properties

- [x] Add `src/web_of_trust_chain.rs` with public spec/exec chain-validity wrappers, matching `ensures`, and active simple-path chain validity proofs/tests for `web-of-trust.chain-validity`
- [x] Require and verify derived-`PrincipalId` equality across root membership and chain repetition checks
- [x] Prove and test that duplicate `roots` entries are semantically ignored by rooted resolution and therefore by threshold callers that reuse rooted-resolution results
- [x] Add explicit `proof fn` artifacts `chain_valid_root_issuer`, `chain_valid_adjacency`, `chain_valid_simple_path`, `chain_valid_role_filtered`, and `chain_valid_requires_active_edges`
- [x] Add explicit `proof fn` artifact `invalid_records_have_no_chain_edge`
- [x] Prove and test that non-canonical or bytes/ID-mismatched attestations/revocations contribute no valid chain edge
- [x] Add explicit unit/proptest coverage for the `web-of-trust.chain-validity` scenarios in `openspec/changes/web-of-trust-primitives/specs/web-of-trust/spec.md`, including root-issuer, adjacency, and inactive-edge cases
- [x] Add a proptest model-oracle for chain validity
- [x] Add `src/web_of_trust_resolution.rs` with public spec/exec rooted-resolution wrappers, matching `ensures`, exact rooted-trust decision helpers, including the empty-chain root base case, plus proofs/tests for `web-of-trust.rooted-resolution`
- [x] Implement rooted-resolution witnesses as finite sequences of attestation indices and use bounded search over those indices
- [x] Add explicit `proof fn` artifacts `resolves_trusted_at_sound`, `resolves_trusted_at_complete`, `duplicate_roots_ignored`, and `invalid_records_ignored_by_resolution`
- [x] Prove and test that non-canonical or bytes/ID-mismatched attestations/revocations are ignored by rooted resolution
- [x] Add explicit unit/proptest coverage for the `web-of-trust.rooted-resolution` scenarios in `openspec/changes/web-of-trust-primitives/specs/web-of-trust/spec.md`, including direct-root-cert depth-1 cases
- [x] Add a proptest model-oracle for rooted resolution
- [x] Add/extend `src/properties.rs` with explicit `proof fn` artifact `revocation_cuts_trust_witness` for `properties.web-of-trust.revocation-cuts-trust`
- [x] Add witness-based regression tests for `properties.web-of-trust.revocation-cuts-trust`
- [x] Add/extend `src/properties.rs` with explicit `proof fn` artifact `depth_monotone_resolution` for `properties.web-of-trust.depth-monotone`
- [x] Add depth-regression tests for `properties.web-of-trust.depth-monotone`
- [x] Add `docs/requirements.md` entries plus explicit `r[impl]` / `r[verify]` annotations on the defining source helpers, lemmas, and named regression tests for `web-of-trust.chain-validity`, `web-of-trust.rooted-resolution`, `properties.web-of-trust.revocation-cuts-trust`, and `properties.web-of-trust.depth-monotone`

## Phase 4: Threshold Policy and Integration

- [x] Add `src/web_of_trust_policy.rs` with public spec/exec threshold wrappers, matching `ensures`, exact threshold accept/reject helpers, distinct-introducer dedup by introducer principal, `d - 1` introducer depth accounting, repeated-final-hop rejection including the `I = S` self-introduction case, and explicit `k = 0` / `d = 0` reject behavior, and proofs/tests for `web-of-trust.threshold`
- [x] Implement threshold checking by enumerating finite introducer candidates from `attestations`
- [x] Require and verify derived-`PrincipalId` equality across threshold introducer dedup
- [x] Add explicit `proof fn` artifacts `threshold_accepts_sound`, `threshold_accepts_complete`, and `invalid_records_ignored_by_threshold` for the threshold checker/spec equivalence
- [x] Prove and test that non-canonical or bytes/ID-mismatched attestations/revocations are ignored by threshold evaluation
- [x] Add explicit unit/proptest coverage for the `web-of-trust.threshold` scenarios in `openspec/changes/web-of-trust-primitives/specs/web-of-trust/spec.md`, including `k = 0` reject, `d = 0` reject, root-subject behavior, indirect-introducer rejection, repeated-final-hop rejection including the `I = S` self-introduction case, and duplicate-attestation dedup
- [x] Add a proptest model-oracle for threshold acceptance
- [x] Add/extend `src/properties.rs` with explicit `proof fn` artifacts `threshold_dedup_preserves_result` and `threshold_dedup_preserves_count` for `properties.web-of-trust.threshold-dedup`
- [x] Add duplicate-attestation regression tests for `properties.web-of-trust.threshold-dedup`
- [x] Add `docs/requirements.md` entries plus explicit `r[impl]` / `r[verify]` annotations on the defining source helpers, lemmas, and named regression tests for `web-of-trust.threshold`
- [x] Add remaining `docs/requirements.md` entries plus explicit `r[impl]` / `r[verify]` annotations on the defining source helpers, lemmas, and named regression tests for `properties.web-of-trust.threshold-dedup`
- [x] Export any still-pending new modules from `src/lib.rs`, notably `src/web_of_trust_policy.rs`
- [x] Add required `r[impl]` anchors to every public function introduced or changed by this change, including public functions outside the specifically named helper/proof lists
- [x] Run a deterministic mirror check that compares every new `r[...]` ID in `openspec/changes/web-of-trust-primitives/specs/web-of-trust/spec.md` against `docs/requirements.md`, fails on any missing or mismatched mirror entry, and saves output to `openspec/changes/web-of-trust-primitives/evidence/mirror-check.txt`
- [x] Run a deterministic anchor check that every defining source helper, checker, test, and lemma named in the design carries the required `r[impl]` / `r[verify]` anchors, and save output to `openspec/changes/web-of-trust-primitives/evidence/anchor-check.txt`
- [x] Run a deterministic public-function coverage check that every public function introduced or changed by this change carries required `r[impl]` coverage aligned with `./scripts/verify.sh`, and save output to `openspec/changes/web-of-trust-primitives/evidence/public-fn-anchor-check.txt`
- [x] Run a deterministic bridge audit that the only trusted crypto bridges in the change are `blake3_digest_bridge`, `attestation_signature_verify_bridge`, and `revocation_signature_verify_bridge`, that any assume-spec bridge is `#[cfg(verus_keep_ghost)]` gated, and save output to `openspec/changes/web-of-trust-primitives/evidence/bridge-audit.txt`
- [x] Run a deterministic bridge-contract audit that compares the implemented definitions/contracts of `blake3_digest_bridge`, `attestation_signature_verify_bridge`, and `revocation_signature_verify_bridge` against `openspec/changes/web-of-trust-primitives/evidence/trusted-bridge-review.md`, fails on any drift, and saves output to `openspec/changes/web-of-trust-primitives/evidence/bridge-contract-audit.txt`
- [x] Treat earlier ad-hoc reruns only as developer checks; do not claim final validation green for this change until the evidence files below exist and are fresh for the current tree
- [x] Run `cargo test` with `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic` and save fresh command output to `openspec/changes/web-of-trust-primitives/evidence/cargo-test.txt`
- [x] Run `cargo clippy --all-targets -- -D warnings` with `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic` and save fresh command output to `openspec/changes/web-of-trust-primitives/evidence/clippy.txt`
- [x] Run `./scripts/verus.sh src/lib.rs` with `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic` in the environment and save fresh command output to `openspec/changes/web-of-trust-primitives/evidence/verus.txt`
- [x] Run `./scripts/check-ensures.sh` and save fresh command output to `openspec/changes/web-of-trust-primitives/evidence/check-ensures.txt`
- [x] Run `tracey query uncovered`, fail if the result is not `0 uncovered`, and save fresh command output to `openspec/changes/web-of-trust-primitives/evidence/tracey-uncovered.txt`
- [x] Run `tracey query untested`, fail if the result is not `0 untested`, and save fresh command output to `openspec/changes/web-of-trust-primitives/evidence/tracey-untested.txt`
- [x] Run `./scripts/verify.sh` and save fresh command output to `openspec/changes/web-of-trust-primitives/evidence/verify.txt`
