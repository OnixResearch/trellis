## Context

This repo verifies compact value-level models: exact `spec fn` rules,
exec wrappers with matching `ensures`, proof lemmas, and concrete or
property-style tests. A web-of-trust change fits that style, but only if
the identity, payload, revocation, and graph semantics are fixed before
code lands.

Fresh audit against pinned pre-change shipped surfaces `src/lib.rs`,
`docs/requirements.md`, `README.md`, and `openspec/specs/` showed this
was net-new work: no shipped `web_of_trust` modules, no shipped
`web-of-trust.*` requirements, and no main-spec web-of-trust domain.
The live working tree now contains landed Phase 1+2, the full Phase 3
chain/rooted-resolution/property slices, and the full Phase 4
threshold slice from this active change: `web_of_trust_principal`,
`web_of_trust_attestation`, `web_of_trust_revocation`,
`web_of_trust_chain`, `web_of_trust_resolution`,
`web_of_trust_policy`, plus cross-module web-of-trust lemmas in
`src/properties.rs`, along with the mirrored
`web-of-trust.principal-fingerprint`,
`web-of-trust.attestation-signature`,
`web-of-trust.attestation-window`, `web-of-trust.revocation`,
`web-of-trust.chain-validity`, `web-of-trust.rooted-resolution`,
`properties.web-of-trust.revocation-cuts-trust`,
`properties.web-of-trust.depth-monotone`, `web-of-trust.threshold`,
and `properties.web-of-trust.threshold-dedup` requirements.

## Goals / Non-Goals

**Goals**
- Canonical principal IDs from public-key bytes
- Exact attestation and revocation payload binding
- One operational trust role with explicit non-operational role handling
- Same-issuer exact/family revocation semantics
- Active-window semantics reusable across graph algorithms
- Exact rooted-resolution and threshold decisions over finite inputs
- Explicit `docs/requirements.md` and source-level Tracey coverage

**Non-Goals**
- Packet parsing, keyserver sync, or UI trust algebra
- Proving BLAKE3 or signature math internally
- Human identity proofing or directory lookup
- Additional non-`trust` role semantics beyond exact signed transport

## Decisions

### 1. Principal IDs

Use the full 32-byte BLAKE3 digest of canonical public-key bytes as
`PrincipalId`. Graph-level equality is by full 32-byte `PrincipalId`
equality.

### 2. Canonical byte grammar

This change fixes one byte grammar:
- variable byte strings = `u32` little-endian length + bytes
- times = `u64` little-endian
- `sequence_no` = `u64` little-endian
- role tags = `u8`
- target-kind tags = `u8`
- `trust` role tag = `0x01`
- `ExactPayload` target-kind tag = `0x01`
- `Family` target-kind tag = `0x02`
- any other `target_kind_tag` is non-canonical and rejected
- any variable-length field with `len > u32::MAX` is non-canonical and
  rejected

Canonical encodings:
- attestation payload = `(issuer_bytes, subject_bytes, role_tag,
  not_before, not_after, sequence_no)`
- revocation payload = `(revocation_issuer_bytes, target_kind_tag,
  target_encoding, effective_time)`
- family target encoding = `(issuer_bytes, subject_bytes, role_tag)`
- exact-payload target encoding = one length-prefixed canonical
  attestation payload byte string

Canonical public-key bytes are caller-supplied; this change does not
model external-format canonicalization.

### 3. Crypto boundaries are explicit and audited

If crypto helpers stay outside Verus, the only allowed trusted bridges
are:
- `blake3_digest_bridge`
- `attestation_signature_verify_bridge`
- `revocation_signature_verify_bridge`

Required contracts:
- `blake3_digest_bridge` returns the full 32-byte BLAKE3 digest of the
  exact canonical key-byte input
- `attestation_signature_verify_bridge` verifies against the attestation
  issuer's exact canonical key bytes and exact canonical attestation
  payload bytes
- `revocation_signature_verify_bridge` verifies against the revocation
  issuer's exact canonical key bytes and exact canonical revocation
  payload bytes

Any new or changed trusted bridge requires explicit human review before
implementation is considered complete. The review artifact at
`openspec/changes/web-of-trust-primitives/evidence/trusted-bridge-review.md`
must record the approved contracts for the allowed bridges, and later
completion checks must compare the implemented bridge definitions and
contracts against that artifact. Any assume-spec bridge must be
`#[cfg(verus_keep_ghost)]` gated.

### 4. Role semantics

Only role tag `0x01` (`trust`) is operational. Other role-tag values are
valid signed payload contents but are non-operational in this change.
They never form trust edges, but they may still be matched exactly by
family revocations.

### 5. Revocation semantics

A revocation is effective only if it is:
- authenticated over issuer, target reference, and effective time
- same-issuer relative to the targeted attestation or family
- effective at or before evaluation time

Exact-payload revocation is by canonical attestation payload identity;
identical payload copies are one semantic exact-payload target and are
all disabled. Family revocation matches the exact `(issuer, subject,
role)` family.

### 6. Canonicality and record invariants at the API boundary

`Attestation` and `Revocation` records carry canonical issuer/subject
key bytes for signed payload construction and also carry derived
`PrincipalId` fields for graph comparisons. The record invariant is that
those derived IDs equal `principal_id_from_bytes(canonical_key_bytes)`.
If canonical payload construction fails or the bytes/ID invariant does
not hold, the record is treated as invalid and contributes no active
edge or effective revocation. `attestation_active_at` therefore means:
signature valid over canonical payload bytes, time inside the exact
half-open window `[not_before, not_after)`, and no effective same-issuer
revocation.

### 7. Rooted resolution

Public input model:
- `roots: &[PrincipalId]`
- `attestations: &[Attestation]`
- `revocations: &[Revocation]`

Root membership is set-style over `PrincipalId` equality. Duplicate
attestation records may appear as inputs. The exact decision is:

`resolves_trusted_at(...) == true` iff either:
- subject is already in `roots` and the witness is the empty chain, or
- there exists a valid trust chain from some root to the subject within
  the depth bound

The proof/exec witness shape is a finite sequence of attestation indices
into `attestations`. The checker uses bounded search over those finite
indices up to the supplied depth bound; soundness/completeness lemmas
connect that bounded search to the existential spec.

### 8. Threshold policy

`threshold_accepts(..., k, d, t)` rejects when `k == 0`. For `k > 0`, it
accepts iff at least `k` distinct introducers qualify, where each
qualifying introducer:
- resolves at the same time `t` under bound `d - 1`
- has an active direct `trust` edge to the subject at `t`
- preserves simple-chain validity when that final hop is appended

When `d == 0`, reject every subject. Root subjects get no threshold
special case. Threshold evaluation deduplicates by introducer
`PrincipalId` before comparing against `k`.

The checker enumerates finite introducer candidates from `attestations`,
uses bounded rooted-resolution checks for each candidate, and counts the
qualifying unique introducer IDs. Soundness/completeness lemmas connect
that finite enumeration to the threshold existential spec.

### 9. Module ownership

- `src/web_of_trust_principal.rs` — key bytes -> `PrincipalId`
- `src/web_of_trust_attestation.rs` — role model, attestation payloads,
  signature binding, activity helpers
- `src/web_of_trust_revocation.rs` — revocation payloads,
  same-issuer/effective matching
- `src/web_of_trust_chain.rs` — valid trust chains
- `src/web_of_trust_resolution.rs` — rooted resolution
- `src/web_of_trust_policy.rs` — threshold acceptance
- `src/properties.rs` — cross-module proof lemmas

### 10. Requirement coverage plan

Every `r[...]` in this change must be mirrored one-to-one into
`docs/requirements.md` before implementation is considered complete.

| Requirement | `r[impl]` anchor | `r[verify]` / proof plan | Tests |
| --- | --- | --- | --- |
| `web-of-trust.principal-fingerprint` | `principal_id_from_bytes` | `blake3_digest_bridge`, `principal_id_deterministic`, `principal_id_full_eq`, `principal_id_matches_key_bytes` | equal/different bytes + proptest oracle |
| `web-of-trust.attestation-signature` | `attestation_payload_bytes`, `attestation_signature_valid` | `attestation_signature_verify_bridge`, `attestation_signature_binds_payload`, `attestation_payload_field_order`, `attestation_payload_little_endian`, `attestation_payload_tag_encoding`, `attestation_payload_overlong_rejected` | exact verify, tamper, overlong reject + proptest oracle |
| `web-of-trust.attestation-window` | `attestation_active_at` | `attestation_record_ids_match_bytes`, `invalid_attestation_is_inactive`, `attestation_active_requires_signature`, `attestation_active_half_open_window`, `attestation_active_respects_revocation` | in-window, expired, future, revoked-at-time, wrong-issuer |
| `web-of-trust.revocation` | `revocation_payload_bytes`, `revocation_matches`, `revocation_effective_at` | `revocation_payload_field_order`, `revocation_payload_little_endian`, `revocation_family_target_encoding`, `revocation_exact_payload_target_encoding`, `revocation_target_kind_tags`, `revocation_record_ids_match_bytes`, `invalid_revocation_never_effective`, `revocation_signature_verify_bridge`, `revocation_signature_binds_payload`, `revocation_monotone`, `revocation_exact_payload_correct`, `revocation_family_target_correct`, `revocation_invalid_kind_rejected`, `revocation_overlong_rejected` | exact, family, invalid-kind, tampered-issuer, tampered-target, tampered-time, overlong, unrelated, wrong-issuer + proptest oracle |
| `web-of-trust.chain-validity` | `chain_valid_at` | `chain_valid_root_issuer`, `chain_valid_adjacency`, `chain_valid_simple_path`, `chain_valid_role_filtered`, `chain_valid_requires_active_edges`, `invalid_records_have_no_chain_edge` | valid path, root issuer, adjacency, cycle, inactive, non-`trust` + proptest oracle |
| `web-of-trust.rooted-resolution` | `resolves_trusted_at` | `resolves_trusted_at_sound`, `resolves_trusted_at_complete`, `duplicate_roots_ignored`, `invalid_records_ignored_by_resolution` | root depth 0, direct root cert depth 1, existing chain, too-small bound + proptest oracle |
| `web-of-trust.threshold` | `threshold_accepts` | `threshold_accepts_sound`, `threshold_accepts_complete`, `invalid_records_ignored_by_threshold` | roots depth 1, `k=0`, `d=0`, root-subject case, indirect introducer reject, repeated-final-hop, duplicate-edge + proptest oracle |
| `properties.web-of-trust.revocation-cuts-trust` | `revocation_cuts_trust_witness` | `revocation_cuts_trust_witness` | witness regression tests |
| `properties.web-of-trust.depth-monotone` | `depth_monotone_resolution` | `depth_monotone_resolution` | depth regressions |
| `properties.web-of-trust.threshold-dedup` | `threshold_dedup_preserves_result`, `threshold_dedup_preserves_count` | `threshold_dedup_preserves_result`, `threshold_dedup_preserves_count` | duplicate-attestation regressions |

Property witness assumptions used by the cross-module proofs:
- `revocation_cuts_trust_witness` takes a fixed attestation-index chain
  witness and a designated revoked attestation occurrence on that same
  witness.
- `depth_monotone_resolution` reuses the same successful attestation-index
  witness while increasing only the depth bound.
- `threshold_dedup_preserves_result` and
  `threshold_dedup_preserves_count` compare original vs deduplicated
  final-hop edge multisets for a fixed set of qualifying introducer
  principals.

### 11. Deterministic completion checks

Before implementation is considered complete or green to claim:
- every new `r[...]` ID must have a one-to-one `docs/requirements.md`
  mirror
- every defining source artifact named above must carry its planned
  source-level `r[impl]` / `r[verify]` anchors
- every public function introduced or changed by this change must carry
  required `r[impl]` coverage aligned with the repo-wide `./scripts/verify.sh`
  public-function coverage gate, not only the specifically named helpers
  above
- `openspec/changes/web-of-trust-primitives/evidence/trusted-bridge-review.md`
  must exist before completion and must record explicit human approval
  of the reviewed contracts for `blake3_digest_bridge`,
  `attestation_signature_verify_bridge`, and
  `revocation_signature_verify_bridge`
- deterministic bridge audits must fail on any drift between those
  reviewed contracts and the implemented bridge definitions/contracts,
  and must also confirm any assume-spec bridge is
  `#[cfg(verus_keep_ghost)]` gated
- validation expectations remain the repo command set used later in
  tasks, and any green claim must capture fresh output for: `cargo test`,
  `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh
  src/lib.rs`, `./scripts/check-ensures.sh`, `tracey query uncovered`,
  `tracey query untested`, and `./scripts/verify.sh`

## Risks / Trade-offs

- **Abstract crypto boundary**: accepted; verify exact use, not crypto internals.
- **Single operational role**: accepted; smaller model, later extension possible.
- **No human-identity normalization**: accepted; graph logic only.
- **Narrower than full PGP trust**: accepted; better than a vague model.
