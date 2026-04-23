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
round-trip, key-message binding, and fixed-width key/signature outputs
at the primitive byte boundary consumed by later UCAN token-validation
changes.

## What Changes

A new module `src/ucan_signature.rs` providing:

**Data model:**
- `PublicKey` — 32-byte Ed25519 public key
- `SecretKey` — 32-byte Ed25519 seed
- `Signature` — 64-byte Ed25519 signature

**Exec functions (external_body for crypto):**
- `sign(secret_key, message) -> Signature`
- `verify(public_key, message, signature) -> bool`
- `public_from_secret(secret_key) -> PublicKey`

**Verified properties:**
- Round-trip: `verify(pk, msg, sign(sk, msg)) == true` when `pk = public_from_secret(sk)`
- Key binding: `verify(pk1, msg, sign(sk, msg)) == false` when `pk1 != public_from_secret(sk)`
- Message binding: `verify(pk, msg2, sign(sk, msg1)) == false` when `msg1 != msg2` (modeled)
- Determinism: repeated executions of `sign(sk, msg)` for the same `sk` and `msg` yield identical signature bytes
- Signature length: output is always exactly 64 bytes
- Public-key length: `public_from_secret(sk)` is always 32 bytes for valid 32-byte seed inputs

The crypto operations themselves stay `#[verifier::external_body]` —
we are not verifying Ed25519 internals. The proof surface is the
narrow primitive contract encoded by this change: honest-key round-trip,
wrong-key rejection, wrong-message rejection under the matched derived
public key, deterministic repeated signing, and fixed-width key/signature
outputs.

This change stops at the signature primitive boundary. Token-envelope
or token-validation callers consume `sign`, `verify`, and
`public_from_secret` over already-serialized payload bytes; wiring those
primitives into higher-level UCAN validation remains the responsibility
of the envelope and validation changes. Valid inputs are in-scope only
for well-formed fixed-width values: 32-byte seed secret keys, 32-byte
public keys, and 64-byte signatures. Reject/parse behavior for malformed
or non-canonical external byte strings remains out of scope for this
change.

## Capabilities

### New Capabilities
- `ucan-signature.round-trip`: sign then verify succeeds for matching keypair
- `ucan-signature.key-binding`: wrong public key rejects
- `ucan-signature.message-binding`: wrong message rejects
- `ucan-signature.deterministic`: same inputs produce same signature
- `ucan-signature.signature-length`: signatures are 64 bytes
- `ucan-signature.public-key-length`: derived public keys are 32 bytes

## Impact

- **Files**: `src/ucan_signature.rs` (new)
- **APIs**: ~5 pub fns (sign, verify, public_from_secret, plus spec helpers)
- **Dependencies**: ed25519-dalek or equivalent behind `#[cfg(not(verus_keep_ghost))]`
- **Testing**: Runtime tests cover round-trip, wrong-key rejection, wrong-message rejection, determinism, signature length, and derived public-key length; Verus proof functions cover the same structural contracts

## Acceptance Evidence

Archive readiness for this change requires fresh `openspec_gate`
results for proposal/design/tasks plus fresh validation transcripts for
`CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_signature`
and `./scripts/verus.sh src/lib.rs`, captured alongside the archive
closeout evidence.
