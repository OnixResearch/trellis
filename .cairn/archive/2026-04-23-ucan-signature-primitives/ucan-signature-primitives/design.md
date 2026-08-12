## Context

The capability-auth modules treat signatures as opaque booleans.
UCAN tokens need real sign/verify operations with provable contracts.
Ed25519 is the primary scheme (multicodec 0xed in did:key).

## Goals / Non-Goals

**Goals:** Verified contracts for sign/verify round-trip, key binding,
message binding, deterministic signing, 64-byte signature outputs, and
32-byte derived public keys. Provide the byte-level signature primitive
surface consumed by higher-level UCAN envelope and validation code.

**Non-Goals:** Verifying Ed25519 internals (modular arithmetic, curve
operations). Supporting non-Ed25519 schemes in this change. Implementing
higher-level token parsing, envelope validation, or delegation-chain
resolution in this change.

## Decisions

### 1. Crypto Stays External

**Choice:** `sign` and `verify` are `#[verifier::external_body]` with
assumed postconditions modeling the sign/verify contract.

**Rationale:** Ed25519 internals are not expressible in Verus's decidable
fragment. The security guarantee comes from the library (ed25519-dalek),
not from the proof. What we prove is that the rest of the system uses
the contract correctly.

**Implementation:** Runtime uses ed25519-dalek behind
`#[cfg(not(verus_keep_ghost))]`. Verus sees spec-level stubs with
postconditions.

### 2. Byte-Sequence Message Model

**Choice:** Messages are `Seq<u8>` in spec, `&[u8]` in exec.

**Rationale:** Token payloads are serialized to bytes before signing.
Keeping the signature interface at the byte level avoids coupling to
any serialization format.

### 3. Key Types Are Opaque Wrappers

**Choice:** `SecretKey` is modeled as a 32-byte Ed25519 seed,
`PublicKey` as 32 bytes, and `Signature` as 64 bytes.

**Rationale:** The landed runtime implementation derives the verifying
key from a 32-byte `ed25519_dalek::SigningKey` seed and emits standard
32-byte public keys plus 64-byte signatures. Fixing the representation
at these sizes removes the earlier 32-byte/64-byte ambiguity and makes
length obligations explicit in both spec and exec postconditions.

### 4. Length and Determinism Stay in the Primitive Contract

**Choice:** Determinism and fixed-length guarantees are first-class
signature-module obligations, not deferred integration concerns.

**Rationale:** The token-validation pipeline only needs a narrow
primitive contract over serialized payload bytes: repeated signing with
the same secret key and message yields the same signature bytes,
`sign(...)` always yields 64 bytes, and `public_from_secret(...)` always
yields 32 bytes. The signature module proves those facts directly so
callers can treat signature bytes and key bytes as stable opaque values.

**Implementation:** `sign_spec` and `public_from_secret_spec` build
fixed-length sequences, the exec wrappers carry matching postconditions,
and proof functions cover determinism plus both length properties.

### 5. Integration Boundary Stops at Serialized Payload Bytes

**Choice:** The integration point for token validation is a byte-level
primitive boundary, not an in-change token parser.

**Rationale:** UCAN envelope and validation code serialize payloads and
then call `verify(pk, payload_bytes, sig)` or `sign(sk, payload_bytes)`.
This change only guarantees the signature primitive contract over those
bytes. It deliberately does not model token-field extraction, signature
placement inside a token envelope, or delegation-chain traversal.

**Verification surface:** Runtime tests exercise honest-key round-trip,
wrong-key rejection, wrong-message rejection under the matched derived
public key, determinism, signature length, and derived public-key
length. Proof functions map directly to the requirement set: the
round-trip axiom and `sign_verify_round_trip` cover
`r[ucan-signature.round-trip]`, `wrong_key_rejects` covers
`r[ucan-signature.key-binding]`, `wrong_message_rejects` covers
`r[ucan-signature.message-binding]`, `sign_deterministic` covers
`r[ucan-signature.deterministic]`, `signature_length` covers
`r[ucan-signature.signature-length]`, and `public_key_length` covers
`r[ucan-signature.public-key-length]`. Archive-ready validation also
captures the concrete checks promised in the proposal:
the `openspec_gate` tool for proposal/design/tasks plus
`CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_signature`
and `./scripts/verus.sh src/lib.rs`.

## Risks / Trade-offs

**[Assumed contract mismatch]** → The assumed postconditions on
sign/verify must faithfully model ed25519-dalek behavior. Mitigated
by property tests that exercise the runtime path and compare against
the spec contract.
