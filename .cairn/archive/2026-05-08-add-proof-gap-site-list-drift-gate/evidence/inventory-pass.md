# Proof-Gap Inventory Report

Source root: `src/`

## Summary

| marker | expected | actual | status |
| --- | ---: | ---: | --- |
| `external_body` | 16 | 16 | ok |
| `assume_specification` | 0 | 0 | ok |
| `no_ensures` | 0 | 0 | ok |
| `runtime_exempt` | 0 | 0 | ok |
| `assume_call` | 0 | 0 | ok |

## External Body Site List

Status: `ok`

Expected sites: 16

Actual sites: 16

Missing expected sites: none.

Unexpected sites: none.

## Hits

### `external_body`

- `src/chain.rs:70` `#[verifier::external_body]` → `pub fn compute_entry_hash(`
- `src/chain.rs:207` `#[verifier::external_body]` → `pub proof fn hash_collision_free(`
- `src/nix_version.rs:413` `#[verifier::external_body]` → `pub fn compare_versions(a: &Vec<u8>, b: &Vec<u8>) -> (result: i8)`
- `src/snapshot_integrity.rs:54` `#[verifier::external_body]` → `pub proof fn dual_hash_collision_resistant(a1: u64, b1: u64, a2: u64, b2: u64)`
- `src/snapshot_integrity.rs:63` `#[verifier::external_body]` → `pub fn dual_hash(a: u64, b: u64) -> (result: u64)`
- `src/ucan_did.rs:623` `#[verifier::external_body]` → `pub proof fn axiom_base58btc_round_trip(bytes: Seq<u8>)`
- `src/ucan_did.rs:631` `#[verifier::external_body]` → `pub proof fn axiom_base58btc_injective(a: Seq<u8>, b: Seq<u8>)`
- `src/ucan_did.rs:640` `#[verifier::external_body]` → `pub fn base58btc_encode(bytes: &Vec<u8>) -> (result: Vec<u8>)`
- `src/ucan_did.rs:649` `#[verifier::external_body]` → `pub fn base58btc_decode(encoded: &Vec<u8>) -> (result: Option<Vec<u8>>)`
- `src/ucan_signature.rs:254` `#[verifier::external_body]` → `pub proof fn axiom_sign_verify_round_trip(sk: Seq<u8>, msg: Seq<u8>)`
- `src/ucan_signature.rs:263` `#[verifier::external_body]` → `pub proof fn axiom_wrong_key_rejects(sk: Seq<u8>, pk: Seq<u8>, msg: Seq<u8>)`
- `src/ucan_signature.rs:275` `#[verifier::external_body]` → `pub proof fn axiom_wrong_message_rejects(sk: Seq<u8>, msg1: Seq<u8>, msg2: Seq<u8>)`
- `src/ucan_signature.rs:289` `#[verifier::external_body]` → `pub fn sign(sk: &SecretKey, message: &Vec<u8>) -> (result: Ed25519Signature)`
- `src/ucan_signature.rs:303` `#[verifier::external_body]` → `pub fn verify(pk: &PublicKey, message: &Vec<u8>, sig: &Ed25519Signature) -> (result: bool)`
- `src/ucan_signature.rs:315` `#[verifier::external_body]` → `pub fn public_from_secret(sk: &SecretKey) -> (result: PublicKey)`
- `src/web_of_trust_principal.rs:103` `#[verifier::external_body]` → `pub fn blake3_digest_bridge(input: &[u8]) -> (result: Vec<u8>)`

### `assume_specification`

No hits.

### `no_ensures`

No hits.

### `runtime_exempt`

No hits.

### `assume_call`

No hits.
