# Boundary classification evidence

All 16 live `#[verifier::external_body]` sites are classified in `proof-gap-inventory.md`.

## Summary

- Permanent cryptographic/hash runtime or collision-resistance trust: 5 sites (`chain.rs`, `snapshot_integrity.rs`, `web_of_trust_principal.rs`).
- Permanent codec trust: 4 sites (`ucan_did.rs` base58btc axioms/runtime bridges).
- Permanent Ed25519 crypto trust: 6 sites (`ucan_signature.rs`).
- Permanent solver-limit exec/spec bridge: 1 site (`nix_version.rs::compare_versions`).
- Closeable structural candidates selected for this slice: 0. `src/proof_gap_boundary_audit.rs` records this as a verified audit constant; it does not replace or hide any trusted boundary.

## Rationale

The previous closeable structural items called out by the repository are already closed: equal-input hash determinism follows from pure spec equality, UCAN DID varint decoding is verified directly, CID multibase round-trip composes through the shared DID base58btc model, UCAN signature byte-length admission wrappers are verified before trusted crypto, and principal-ID digest admission/equality wrappers are verified around the remaining digest bridge.

This slice therefore records a no-closeout result rather than weakening the model or replacing any boundary with `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, `@verified-logic:exempt`, or a new `#[verifier::external_body]`.
