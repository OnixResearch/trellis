# HIR Attribute Parsing Verification

## Result

The finite inert-attribute implementation is complete. Strict validation and all three lifecycle gates pass. Sync created the accepted specification, and archive execution completed under `2026-09-02-hir-attribute-parsing`.

The public `hir_attribute_parsing` facade composes model, registry, rule, admission, agreement, proof, and test components under `src/hir/attribute/`. Every production file has fewer than 300 lines.

The functional core models:

- no-args, single, combine, and generic parser classes;
- non-empty supported target masks;
- argument templates and aggregate occurrences;
- reject, keep-first, keep-last, and combine duplicate rules;
- normal and unsafe-wrapper safety rules;
- stable and exact feature-gate rules;
- deterministic reason and input-index metadata; and
- limited-early versus later-HIR result agreement.

The core consumes typed finite facts and performs no I/O. A failed parse result is not comparable to a successful phase result.

## Baseline

The baseline source was `36f372f7fe5a87491dd61d62a40f22228975ddea`. Baseline Cargo tests, full-library Verus, and Tracey commands returned status `0`.

The first uncovered query printed the known Tracey daemon-start cancellation. Its direct rerun reported `0 uncovered out of 2072 rules`. See `evidence/baseline-2026-09-02/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 12 passed |
| Focused Verus | 48 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,839 Verus proofs; 4,889 tests; 2,079 requirements; 607 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,079 |
| Tracey untested | 0 of 2,079 |
| Octet | 3,867 accepted; 0 new; 0 stale |
| JSON artifact contracts | positive and negative fixtures passed |
| Consumer profile | 39 modules, 42 artifacts, and 10 fixtures passed |
| Cairn strict validation | passed |
| Cairn proposal, design, and tasks gates | passed |

The full wrapper ran inside the repository Nix development shell. A temporary `.worktrees/aspen` symlink restored the existing `../aspen` sibling relation. The command removed the link when it finished.

See `evidence/final-2026-09-02/`.

## Octet Ratchet

The new `src/hir/attribute/` files have zero Octet findings. The new public module changed only the existing `src/lib.rs` file-length identity.

The guarded migration replaced that one ID and preserved every unchanged finding record. The accepted count remains 3,867. It added no warning budget or HIR attribute finding.

See `evidence/octet/`.

## Generated and Bound Artifacts

The change refreshed these repository-owned artifacts after source identity changed:

- `docs/module-index.md`;
- `consumer-profile/generated/profile.json`, from its Nickel source;
- `docs/proof-evidence-bundle.json`; and
- `docs/proof-evidence-ir-chain.json`.

The consumer profile still selects the same 39 UCAN-facing modules. It updates only the measured crate-root artifact.

## Boundaries

This evidence proves finite inert parser and policy admission for the checked source and toolchain. It does not prove:

- active attribute or macro expansion;
- proc-macro behavior;
- token-stream parsing;
- rendered rustc diagnostics;
- Rust feature stabilization policy;
- rustc frontend correctness; or
- verifier soundness.
