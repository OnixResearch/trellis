# Octet rail receipt for bounds.clamp, 2026-09-15

Command: cargo-octet prove --policy-artifacts-dir <patched> --source src/bounds.rs --rail trellis.aeneas_lean_coverage.clamp#rail-0

Exit: 0. Receipt identity b3:63da4db595096c8ea13e6713ec63b7c1d8ece0d2a581282fa0f36c6df10b8d43.
cargo-octet artifact verify reports every artifact verified with evidence_tier formal_proof.

The receipt is an `octet-formal-proof-envelope/v1` with a six-stage chain: rust
source, Charon extraction, Aeneas translation, generated Lean, theorem source,
and Lean check. The provider profile identity is the reviewed
`b3:cd11b49d...`, Charon is pinned to `cb50ff16...`, and Lean is 4.31.0.

## Working conditions and deviations

1. The pinned Octet CLI was built from a worktree at revision 890f424c after
   `nix flake lock` refreshed the `dylint-src` and `crane` narHashes. The pinned
   source content is unchanged; GitHub served different tarball hashes.
2. The rail entry is a Trellis-authored copy of the Octet external-consumer
   policy rail, under policy_id `trellis.aeneas_lean_coverage.clamp`.
3. The source root carried a consumer-side adapter: `[package.metadata.charon]`
   `start_from = ["crate::bounds::clamp"]`, `src/bin` removed, a vendored cargo
   source replacement, and the theorem copied to `Theorem.lean`. The receipt
   records the reviewed extraction policy `start_from = crate`, so the item
   selection came from crate metadata rather than from the reviewed policy.
4. `OCTET_AENEAS_LEAN_LIBRARY` pointed at a local build of the same provider
   library revision, to avoid a multi-hour in-sandbox mathlib build.
5. The receipt `trust_boundaries` text still names the external-consumer
   fixture, and `claim.requirement_id` is the Octet policy field rather than a
   Trellis requirement ID.

## Non-claims

The receipt proves the named theorem over the exact bound source and generated
model. It does not prove Aeneas, Charon, Lean, or rustc soundness, semantic
equivalence, whole-program correctness, or release eligibility.
