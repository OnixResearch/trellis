## Context

The capability-auth modules treat signatures as opaque booleans.
UCAN tokens need real sign/verify operations with provable contracts.
Ed25519 is the primary scheme (multicodec 0xed in did:key).

## Goals / Non-Goals

**Goals:** Verified contracts for sign/verify round-trip, key binding,
message binding. Integration point for token validation pipeline.

**Non-Goals:** Verifying Ed25519 internals (modular arithmetic, curve
operations). Supporting non-Ed25519 schemes in this change.

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

**Choice:** `PublicKey` and `SecretKey` wrap fixed-size byte arrays
with no public field access in exec.

**Rationale:** Prevents accidental key material exposure. Spec-level
view functions expose the bytes for proof purposes only.

## Risks / Trade-offs

**[Assumed contract mismatch]** → The assumed postconditions on
sign/verify must faithfully model ed25519-dalek behavior. Mitigated
by property tests that exercise the runtime path and compare against
the spec contract.
