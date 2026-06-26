## Why

Shamir secret sharing is already in tree, already covered by requirements,
and already backed by unit tests and proptests. But it still dominates the
remaining closeable trust surface: `38` of the live `53`
`#[verifier::external_body]` items sit in the six Shamir modules.

That makes Shamir trust reduction highest-value next proof step. Closing
those bridges would remove every non-permanent trusted boundary from the
repo, leaving only the intentionally permanent architectural assumptions
(hash models, Nix version solver limits, store-path hashing, and the Raft
leader-chain invariant lemmas).

## What We Know

- Current tree is green: `cargo test`, `cargo clippy --all-targets -- -D warnings`,
  `./scripts/verus.sh src/lib.rs`, and `./scripts/verify.sh` pass.
- Live trust inventory is `53` `#[verifier::external_body]` items across `11` files.
- Permanent subset is `15` items across `5` files:
  `chain`, `snapshot_integrity`, `nix_version`, `nix_store_path`,
  `log_invariant_inductive`.
- Closeable subset is the Shamir stack:
  - `src/shamir_field.rs`: `14`
  - `src/shamir_lagrange.rs`: `12`
  - `src/shamir_poly.rs`: `6`
  - `src/shamir_split.rs`: `3`
  - `src/shamir_threshold.rs`: `2`
  - `src/shamir_reconstruct.rs`: `1`
- Existing requirement IDs already cover the behavior in `docs/requirements.md`:
  `shamir-field.*`, `shamir-poly.*`, `shamir-lagrange.*`,
  `shamir-split.*`, `shamir-reconstruct.*`, and `shamir-threshold.*`.

## What Changes

- Internalize Shamir field exec/spec bridges and proof stubs in
  `src/shamir_field.rs`.
- Internalize Horner evaluation and polynomial helper bridges in
  `src/shamir_poly.rs`.
- Internalize Lagrange basis, interpolation, and uniqueness bridges in
  `src/shamir_lagrange.rs`.
- Internalize split, reconstruct, and threshold witnesses in
  `src/shamir_split.rs`, `src/shamir_reconstruct.rs`, and
  `src/shamir_threshold.rs`.
- Refresh `README.md` and `openspec/specs/verification-gaps/spec.md` after
  the Shamir bridges disappear.
- Sync `openspec/specs/secret-sharing/spec.md` so the uniqueness prose
  matches the verified Lagrange requirement already tracked in
  `docs/requirements.md`.

## Scope

- **In scope**: removing all `38` closeable Shamir `external_body` sites
  while preserving current public behavior, requirement coverage, and green
  verification.
- **Out of scope**: changing the permanent trust boundaries, expanding the
  Shamir API surface, or changing requirement prose beyond the narrowly
  scoped uniqueness-wording sync needed to match the verified Lagrange
  model already tracked in `docs/requirements.md`.

## Evidence

Starting inventory snapshot:

- `53` total `#[verifier::external_body]` items across `11` files
- `15` permanent boundaries
- `38` closeable Shamir boundaries
