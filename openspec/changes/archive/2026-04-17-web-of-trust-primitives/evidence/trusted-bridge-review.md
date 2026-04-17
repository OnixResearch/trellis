# Trusted Bridge Review — web-of-trust-primitives

Status: approved

This artifact records the exact reviewed contracts for the only allowed
trusted crypto bridges in `web-of-trust-primitives`.

## Scope

Allowed trusted bridges in this change:
- `blake3_digest_bridge`
- `attestation_signature_verify_bridge`
- `revocation_signature_verify_bridge`

No additional trusted crypto bridge is allowed without a new human review.
Any assume-spec bridge in this change must be `#[cfg(verus_keep_ghost)]`
gated.

## Reviewed Contracts

### `blake3_digest_bridge`

Reviewed contract:
- input: exact canonical public-key byte string supplied by caller
- output: full 32-byte BLAKE3 digest of that exact input byte string
- no truncation, normalization, re-encoding, or alternate byte grammar is
  allowed inside the bridge contract

### `attestation_signature_verify_bridge`

Reviewed contract:
- input key bytes: exact canonical attestation issuer public-key bytes
- input payload bytes: exact canonical attestation payload bytes for
  `(issuer_bytes, subject_bytes, role_tag, not_before, not_after, sequence_no)`
- result: signature verification is performed against that exact issuer
  key and that exact canonical payload byte string
- the bridge contract must not accept verification against reordered
  fields, alternate encodings, or substituted issuer bytes

### `revocation_signature_verify_bridge`

Reviewed contract:
- input key bytes: exact canonical revocation issuer public-key bytes
- input payload bytes: exact canonical revocation payload bytes for
  `(revocation_issuer_bytes, target_kind_tag, target_encoding, effective_time)`
- result: signature verification is performed against that exact issuer
  key and that exact canonical payload byte string
- the bridge contract must not accept verification against reordered
  fields, alternate encodings, or substituted issuer/target/effective-time
  bytes

## Human Sign-Off

- reviewer: user
- date: 2026-04-14
- decision: approve
- notes: Approved in-session. Later bridge implementations must match the reviewed contracts above exactly.
