# Rustc Metadata Identity Verification

## Result

The finite crate-artifact compatibility implementation is complete. Strict validation and all three lifecycle gates pass. Sync created the accepted specification, and archive execution completed under `2026-09-03-rustc-metadata-identity`.

The public `rustc_metadata_identity` facade composes model, admission, resolver, proof, and test components under `src/rustc/metadata/`. Every production file has fewer than 300 lines.

The functional core models:

- rlib, dylib, and rmeta capability boundaries;
- declared compiler version, strict version hash, and stable crate ID fields;
- check, documentation, pipeline, link, and codegen usage modes;
- optional encoded MIR requirements;
- deterministic compatibility failures; and
- bounded unique, missing, ambiguous, and excessive candidate selection.

`MAX_CANDIDATES` bounds each scan. The core returns the first unique candidate or first conflicting pair. It performs no filesystem, archive, registry, linker, or codegen I/O.

## Baseline

The baseline source was `470a0db819c1fd72527c34484fb6ba6688983cb9`. Baseline Cargo tests, full-library Verus, and Tracey commands returned status `0`.

The first uncovered query printed the known Tracey daemon-start cancellation. Its direct rerun reported `0 uncovered out of 2079 rules`. See `evidence/baseline-2026-09-02/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 13 passed |
| Focused Verus | 27 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,866 Verus proofs; 4,902 tests; 2,087 requirements; 608 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,087 |
| Tracey untested | 0 of 2,087 |
| Octet | 3,867 accepted; 0 new; 0 stale |
| JSON artifact contracts | positive and negative fixtures passed |
| Consumer profile | 39 modules, 42 artifacts, and 10 fixtures passed |
| Cairn strict validation | passed |
| Cairn proposal, design, and tasks gates | passed |

The full wrapper ran inside the repository Nix development shell. A temporary `.worktrees/aspen` symlink restored the existing `../aspen` sibling relation. The command removed the link when it finished.

See `evidence/final-2026-09-03/`.

## Octet Ratchet

The new `src/rustc/metadata/` files have zero Octet findings. The new public module changed only the existing `src/lib.rs` file-length identity.

The guarded migration replaced that one ID and preserved every unchanged finding record. The accepted count remains 3,867. It added no warning budget or rustc metadata finding.

See `evidence/octet/`.

## Generated and Bound Artifacts

The change refreshed these repository-owned artifacts after source identity changed:

- `docs/module-index.md`;
- `consumer-profile/generated/profile.json`, from its Nickel source;
- `docs/proof-evidence-bundle.json`; and
- `docs/proof-evidence-ir-chain.json`.

The consumer profile still selects the same 39 UCAN-facing modules. It updates only the measured crate-root artifact.

## Boundaries

This evidence proves compatibility of declared finite rustc metadata fields for the checked source and toolchain. It does not prove:

- archive or section parsing;
- rustc hash construction;
- hash collision resistance;
- linker or object-code correctness;
- LTO correctness;
- metadata compression correctness;
- whole build-system correctness; or
- verifier soundness.

The model preserves rustc's mandated SVH and stable-crate-ID fields. It does not replace them with BLAKE3.
