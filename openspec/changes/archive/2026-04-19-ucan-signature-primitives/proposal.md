## Why

UCANs are anchored by cryptographic signatures — every token carries
an issuer signature over its payload, and verification of that signature
is the root trust check before any capability logic runs. The existing
capability-auth modules cover scope, subsumption, containment, and
delegation chain structure, but treat signatures as an opaque boolean.
Without verified sign/verify primitives, the authorization chain has
a trust gap at its foundation.

Ed25519 is the primary signature scheme for UCAN (via did:key with
multicodec prefix 0xed). The verification property is simple: valid
signatures round-trip, and no modification of the message or key
produces a valid signature. The interesting proof surface is not the
cryptographic hardness (that stays trusted/external_body) but the
structural properties: deterministic signing, verification
round-trip, key-message binding, and correct integration with
the token validation pipeline.

## What Changes

A new module `src/ucan_signature.rs` providing:

**Data model:**
- `PublicKey` — 32-byte Ed25519 public key
- `SecretKey` — 64-byte Ed25519 secret key (or 32-byte seed)
- `Signature` — 64-byte Ed25519 signature

**Exec functions (external_body for crypto):**
- `sign(secret_key, message) -> Signature`
- `verify(public_key, message, signature) -> bool`
- `keypair_generate() -> (SecretKey, PublicKey)`
- `public_from_secret(secret_key) -> PublicKey`

**Verified properties:**
- Round-trip: `verify(pk, msg, sign(sk, msg)) == true` when `pk = public_from_secret(sk)`
- Key binding: `verify(pk1, msg, sign(sk, msg)) == false` when `pk1 != public_from_secret(sk)`
- Message binding: `verify(pk, msg2, sign(sk, msg1)) == false` when `msg1 != msg2` (modeled)
- Determinism: `sign(sk, msg) == sign(sk, msg)` — same inputs, same output
- Signature length: output is always exactly 64 bytes

The crypto operations themselves stay `#[verifier::external_body]` —
we are not verifying Ed25519 internals. The proof surface is the
contract between sign and verify: that they form a consistent
pair with no false positives or false negatives for honest keys.

## Capabilities

### New Capabilities
- `ucan-signature.round-trip`: sign then verify succeeds for matching keypair
- `ucan-signature.key-binding`: wrong public key rejects
- `ucan-signature.message-binding`: wrong message rejects
- `ucan-signature.deterministic`: same inputs produce same signature
- `ucan-signature.length`: signatures are 64 bytes

## Impact

- **Files**: `src/ucan_signature.rs` (new)
- **APIs**: ~6 pub fns (sign, verify, keypair_generate, public_from_secret, plus spec helpers)
- **Dependencies**: ed25519-dalek or equivalent behind `#[cfg(not(verus_keep_ghost))]`
- **Testing**: Property tests for round-trip and rejection; Verus proofs for structural contracts
