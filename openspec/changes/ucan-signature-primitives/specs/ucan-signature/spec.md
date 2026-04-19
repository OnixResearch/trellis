# UCAN Signature Primitives Specification

## Purpose

Defines correctness properties for Ed25519 signature creation and
verification as used by UCAN tokens. The cryptographic implementation
is trusted (external_body); this spec covers the contract between
sign and verify and their integration with token validation.

## Definitions

- **Public key**: A 32-byte Ed25519 public key.
- **Secret key**: A 32-byte Ed25519 seed (or 64-byte expanded key).
- **Signature**: A 64-byte Ed25519 signature over a message.
- **Keypair**: A matched (secret_key, public_key) where the public
  key is derived from the secret key.
- **Message**: An arbitrary byte sequence to be signed.

## Requirements

### Signature Round-Trip

#### Requirement: Sign-Verify Round-Trip

Signing a message with a secret key and verifying with the
corresponding public key MUST succeed.

`verify(public_from_secret(sk), msg, sign(sk, msg)) == true`

##### Scenario: Fresh keypair signs and verifies

- GIVEN a keypair (sk, pk) where pk = public_from_secret(sk)
- WHEN signing message m with sk
- THEN verify(pk, m, sign(sk, m)) returns true

### Key Binding

#### Requirement: Wrong Key Rejects

Verifying a signature with a public key that does not correspond
to the signing secret key MUST fail.

`pk != public_from_secret(sk) ==> verify(pk, msg, sign(sk, msg)) == false`

##### Scenario: Different keypair rejects

- GIVEN two keypairs (sk1, pk1) and (sk2, pk2) where pk1 != pk2
- WHEN signing with sk1 and verifying with pk2
- THEN verification fails

### Message Binding

#### Requirement: Wrong Message Rejects

Verifying a signature against a different message than the one
signed MUST fail.

`msg1 != msg2 ==> verify(pk, msg2, sign(sk, msg1)) == false`

##### Scenario: Altered message rejects

- GIVEN message m1 signed with sk producing sig
- WHEN verifying sig against different message m2
- THEN verification fails

### Determinism

#### Requirement: Signing Is Deterministic

Signing the same message with the same key MUST produce the
same signature.

`sign(sk, msg) == sign(sk, msg)`

##### Scenario: Repeated signing

- GIVEN secret key sk and message m
- WHEN signing m twice with sk
- THEN both signatures are identical

### Signature Format

#### Requirement: Signature Length Is 64 Bytes

An Ed25519 signature MUST always be exactly 64 bytes.

`sign(sk, msg).len() == 64`

##### Scenario: Any message produces 64-byte signature

- GIVEN any secret key and any message
- WHEN signing
- THEN the signature is 64 bytes

#### Requirement: Public Key Length Is 32 Bytes

An Ed25519 public key MUST be exactly 32 bytes.

`public_from_secret(sk).len() == 32`
