# Verification: add-bounded-modular-tree-fold-primitives

r[verify bounded-modular-tree-fold.validation]

## Result

The change applies to parent `e72045fffa9237d66c7b662b828723100e263291`.

The current tree passes the focused tests, full verifier, traceability, strict Clippy, consumer profile, and pinned Octet gate.

## Proof and test evidence

- `nix develop -c cargo test --quiet bounded_modular_tree_fold --lib` passes 6 tests.
- `nix develop -c cargo test --quiet u24 --lib` passes 5 tests.
- `nix develop -c cargo run --quiet --example bounded_modular_tree_fold` passes.
- `nix develop -c ./scripts/verus.sh src/lib.rs` reports 11,098 verified obligations and 0 errors.
- `nix develop -c ./scripts/verify.rs` passes all checks and 5,216 library tests.
- `nix develop -c cargo clippy --all-targets -- -D warnings` passes.
- Focused `rustfmt --check` passes for all changed Rust files except the inherited crate root.

The repository-wide format command reports unrelated current-main differences. This change does not modify those files.

## Traceability and evidence

- Tracey reports 0 uncovered rules out of 2,252 rules.
- Tracey reports 0 untested rules out of 2,252 rules.
- The module index covers 624 public modules.
- The invariant catalog covers 14 required rows.
- The proof-evidence bundle and IR chain each validate 7 records.
- The primitive manifest validates 17 primitives, 1 Preserves envelope, and 11 fixtures.
- The proof-gap inventory contains the expected 16 external bodies and no unexpected gaps.
- The consumer profile validates 39 modules, 42 artifacts, and 10 fixtures.
- The README rail reports 11,098 proofs, 5,216 tests, 2,252 requirements, 624 modules, and 16 boundaries.

## Octet evidence

The guarded migration replaces only the accepted `src/lib.rs` file-length identity.

- stale identity: `b3:370738e60362a2d8`
- new identity: `b3:69dd8f7e6f7922b2`
- final result: 3,867 accepted, 3,867 current accepted, 0 new, and 0 stale

The files under `src/bounded_modular_tree_fold/` and `src/u24/` have zero accepted findings.

## Nix boundary

`nix build .#checks.x86_64-linux.octet --no-link -L` passes with local builders.

The broad `nix flake check -L` stops at the known immutable consumer-release pin.

That pin expects the old 47,826-byte `src/lib.rs` and old proof-manifest identity. Current-tree validation and the focused Nix gate pass.

## Lifecycle audit

An initial sync returned exit status zero with `"blocked": true`. The first archive was uncommitted and was reversed.

The superseded logs remain under `evidence/superseded-before-sync-audit-2026-09-04/`.

The corrected sync preview reports `"blocked": false`, `"changed": true`, no diagnostics, and no reasons.

The corrected sync execution reports `"mutated": true`. The accepted specification has BLAKE3 `b2eeb72e6752c2849908cd8f14a1e15419b9ab8cd2a3778902d184c3e2c14711`.

Strict post-sync validation passes.

## Claim boundary

The evidence covers the local finite `u24` model, balanced reduction, equivalence, admission, and abstract work counts.

It does not prove Bend, HVM2, generated C, host compilers, runtime performance, arbitrary trees, or downstream decoding.
