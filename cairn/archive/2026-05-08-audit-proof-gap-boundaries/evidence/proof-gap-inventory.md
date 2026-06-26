# Proof-gap boundary inventory evidence

Captured: 2026-05-08T00:23:52.123680+00:00

## Command

```bash
python - <<'PY'  # scans src/**/*.rs for proof-gap markers
...
PY
```

## Counts

- `external_body`: `16`
- `assume_specification`: `0`
- `no_ensures`: `0`
- `exempt`: `0`
- `assume_call`: `0`

## External-body sites

| # | File | Line | Function | Classification | Rationale |
|---:|---|---:|---|---|---|
| 1 | `src/chain.rs` | 70 | `pub fn compute_entry_hash(` | permanent trust | Runtime hash bridge; spec abstracts BLAKE3-style hash behavior. |
| 2 | `src/chain.rs` | 207 | `pub proof fn hash_collision_free(` | permanent trust | Cryptographic collision-resistance axiom over the abstract hash model. |
| 3 | `src/nix_version.rs` | 413 | `pub fn compare_versions(a: &Vec<u8>, b: &Vec<u8>) -> (result: i8)` | permanent/z3-limit trust | Recursive cv_fuel proof lemmas are internalized; exec/spec correspondence remains solver-limit bridge. |
| 4 | `src/snapshot_integrity.rs` | 54 | `pub proof fn dual_hash_collision_resistant(a1: u64, b1: u64, a2: u64, b2: u64)` | permanent trust | Cryptographic collision-resistance axiom for dual-hash model. |
| 5 | `src/snapshot_integrity.rs` | 63 | `pub fn dual_hash(a: u64, b: u64) -> (result: u64)` | permanent trust | Runtime hash/mix bridge to abstract dual_hash_spec. |
| 6 | `src/ucan_did.rs` | 623 | `pub proof fn axiom_base58btc_round_trip(bytes: Seq<u8>)` | permanent codec trust | Base58btc codec round-trip contract; lower codec algorithm is not verified here. |
| 7 | `src/ucan_did.rs` | 631 | `pub proof fn axiom_base58btc_injective(a: Seq<u8>, b: Seq<u8>)` | permanent codec trust | Base58btc injectivity contract; codec correctness stays trusted. |
| 8 | `src/ucan_did.rs` | 640 | `pub fn base58btc_encode(bytes: &Vec<u8>) -> (result: Vec<u8>)` | permanent runtime codec trust | Runtime base58btc encoder bridge to abstract codec spec. |
| 9 | `src/ucan_did.rs` | 649 | `pub fn base58btc_decode(encoded: &Vec<u8>) -> (result: Option<Vec<u8>>)` | permanent runtime codec trust | Runtime base58btc decoder bridge; verified structural screens compose around it. |
| 10 | `src/ucan_signature.rs` | 254 | `pub proof fn axiom_sign_verify_round_trip(sk: Seq<u8>, msg: Seq<u8>)` | permanent crypto trust | Ed25519 semantic round-trip axiom. |
| 11 | `src/ucan_signature.rs` | 263 | `pub proof fn axiom_wrong_key_rejects(sk: Seq<u8>, pk: Seq<u8>, msg: Seq<u8>)` | permanent crypto trust | Ed25519 key-binding rejection axiom. |
| 12 | `src/ucan_signature.rs` | 275 | `pub proof fn axiom_wrong_message_rejects(sk: Seq<u8>, msg1: Seq<u8>, msg2: Seq<u8>)` | permanent crypto trust | Ed25519 message-binding rejection axiom. |
| 13 | `src/ucan_signature.rs` | 289 | `pub fn sign(sk: &SecretKey, message: &Vec<u8>) -> (result: Ed25519Signature)` | permanent runtime crypto trust | Runtime Ed25519 signing bridge; byte-shape admission wrappers are already verified. |
| 14 | `src/ucan_signature.rs` | 303 | `pub fn verify(pk: &PublicKey, message: &Vec<u8>, sig: &Ed25519Signature) -> (result: bool)` | permanent runtime crypto trust | Runtime Ed25519 verification bridge. |
| 15 | `src/ucan_signature.rs` | 315 | `pub fn public_from_secret(sk: &SecretKey) -> (result: PublicKey)` | permanent runtime crypto trust | Runtime Ed25519 public-key derivation bridge. |
| 16 | `src/web_of_trust_principal.rs` | 103 | `pub fn blake3_digest_bridge(input: &[u8]) -> (result: Vec<u8>)` | permanent runtime crypto trust | Runtime BLAKE3 digest bridge; fixed-width admission/equality around PrincipalId is verified. |

## Zero-count weakened-boundary categories

- `assume_specification`: `0`
- `no_ensures`: `0`
- `exempt`: `0`
- `assume_call`: `0`

## Selected closeout candidate

No safe closeable boundary is selected in this audit slice. The live inventory is already classified as 16 permanent trust boundaries: cryptographic/hash runtime bridges, codec contracts, Ed25519 semantic/runtime contracts, and one documented Nix version exec/spec solver-limit bridge. The audit does not reduce published counts and does not introduce replacement weakened markers.
