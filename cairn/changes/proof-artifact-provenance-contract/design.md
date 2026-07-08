# Design: Proof artifact provenance contract

## Approach

Trellis records proof artifact provenance as deterministic metadata over
already-produced local proof artifacts. The pure validation core should accept
in-memory metadata and byte digests, then return a validation report. Any CLI or
script wrapper owns file reads, verifier execution, and report persistence.

## Canonical identity

Each exportable artifact records:

- stable artifact ID;
- BLAKE3 digest over canonical metadata bytes;
- source/proof input BLAKE3 digests;
- verifier/toolchain identity and normalized invocation;
- declared local property ID and local scope;
- verification status and failure reason when not passing;
- required non-claims.

Paths and display names are review metadata only. They must not be used as the
primary identity.

## Boundary model

The contract distinguishes Trellis-local proof facts from downstream use:

- `trellis_local_property`: the proof was checked for the declared Trellis
  property and exact content;
- `manual_review_input`: another repo may cite the artifact identity for review;
- `downstream_certified`: forbidden unless the downstream repo records its own
  verifier receipt.

The validator fails closed when metadata attempts to promote a Trellis-local
proof into a downstream certification claim.

## Fixture model

Positive fixtures cover a minimal passing proof artifact and a proof artifact
with a failed status plus failure reason. Negative fixtures cover stale digest,
missing property scope, missing verifier identity, missing non-claims, unsupported
claim role, and downstream overclaim.

## What is explicitly not provided

- No proof of downstream Octet, Mantle, Valence, or Cairn behavior.
- No verifier soundness proof beyond the verifier identity recorded in the
  artifact.
- No release eligibility or whole-program correctness claim.
