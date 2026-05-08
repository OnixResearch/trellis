## Context

A fresh audit on 2026-05-08 found these remaining trusted boundaries:

- `src/chain.rs`: `compute_entry_hash`, `hash_collision_free`
- `src/nix_version.rs`: `compare_versions`
- `src/snapshot_integrity.rs`: `dual_hash_collision_resistant`, `dual_hash`
- `src/ucan_did.rs`: `axiom_base58btc_round_trip`, `axiom_base58btc_injective`, `base58btc_encode`, `base58btc_decode`
- `src/ucan_signature.rs`: `axiom_sign_verify_round_trip`, `axiom_wrong_key_rejects`, `axiom_wrong_message_rejects`, `sign`, `verify`, `public_from_secret`
- `src/web_of_trust_principal.rs`: `blake3_digest_bridge`

No `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, or runtime-only `@verified-logic:exempt` markers were found in `src/`.

## Decisions

### 1. Source-derived inventory first

**Choice:** The drain starts by capturing an exact source-derived inventory and classifying every boundary.

**Rationale:** Published trust counts have historically drifted when counts were edited manually or a category was double-counted. A source-derived inventory makes the implementation slice reviewable and prevents accidental overclaims.

### 2. Permanent-vs-closeable classification

**Choice:** Boundaries that encode cryptographic strength, runtime hash/codec implementations, or third-party crypto behavior are permanent unless a later change introduces a verified structural wrapper that does not claim cryptographic strength. Boundaries that only encode deterministic structural behavior, byte/length shape, ordering, or duplicate facts already available in another verified module are closeable candidates.

**Rationale:** The repo should reduce trusted assumptions where mechanical proof is realistic, without spending the narrow slice attacking genuine cryptographic/runtime trust.

### 3. One closeable boundary per drain slice

**Choice:** The implementation phase attempts at most one narrow closeable family before archive.

**Rationale:** External-body removals are proof-heavy. Limiting the slice keeps evidence small and makes revert/retry decisions cheap if the chosen boundary turns out permanent or solver-limited.

## Risks

- **False closeable classification:** mitigate by requiring a concrete proof plan before editing and allowing the drain to reclassify a boundary as permanent with evidence rather than forcing a failed proof.
- **Inventory drift:** mitigate with source-derived count scripts and canonical README/spec/docs synchronization in the same commit as any count change.
- **Overclaiming crypto:** mitigate by explicitly excluding cryptographic strength, collision resistance beyond modeled axioms, and runtime codec correctness unless separately verified.
