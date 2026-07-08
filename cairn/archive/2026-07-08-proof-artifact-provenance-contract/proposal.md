# Change: Proof artifact provenance contract

## Why

Trellis proof artifacts are useful downstream only when their identity, checked
property, and claim boundary are explicit. Octet and Valence can reference
Trellis proofs as review inputs, but Trellis must make the local proof claim
machine-readable without implying downstream Rust, release, or whole-stack
correctness.

## What Changes

- Define a Trellis-owned proof artifact provenance contract with canonical
  BLAKE3 identities for proof inputs, proof outputs, verifier receipts, and
  declared local properties.
- Require every exportable proof artifact to state its checked local property,
  verification status, verifier/toolchain identity, input digests, and explicit
  non-claims.
- Add positive and negative fixtures for valid proof artifact metadata, stale
  digests, missing local-property scope, weakened non-claims, and downstream
  overclaim attempts.
- Document the handoff boundary: exported Trellis artifacts are manual-review or
  reference inputs until a downstream repo reruns and validates its own receipt.

## Impact

- **Files**: proof-artifact provenance docs/contracts, fixture metadata, and
  verification tests.
- **Consumers**: Octet and Valence get stable Trellis artifact identities and
  claim boundaries without inheriting Trellis-local proof authority.
- **Non-claims**: This does not certify downstream Octet code, Mantle release
  bundles, or Cairn release gates. It proves only the declared Trellis-local
  proof fact for the exact content and verifier identity recorded.
