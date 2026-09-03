# Rustc Monomorphization Verification

## Result

The finite concrete-item collection implementation is complete. Strict
validation and the proposal, design, and tasks gates pass. Sync created the
accepted specification, and archive execution completed under
`2026-09-03-rustc-monomorphization`.

The public `rustc_monomorphization` facade composes narrow model, collection,
partition, admission, proof, and test components. Every production file has
fewer than 300 lines. The functional core performs no filesystem, process,
clock, network, compiler, linker, or other external effect.

The model provides:

- finite function, static, and shim keys;
- plain, generic, specialized, static, and shim instance shapes;
- explicit type and const arguments, origins, and shim sources;
- a bound of `MAX_ITEMS` rows per finite table;
- deterministic root and edge-order reachability;
- canonical collapse of repeated roots and paths;
- stable and volatile classification;
- disjoint, complete output-unit partitions; and
- deterministic first-failure table, index, and subject metadata.

Generic or specialized instances require concrete argument metadata. Shim
sources and reference endpoints must exist. Collection and partition facts are
withheld for missing reachability, duplicate output, overlap, omission, or class
drift.

## Baseline

The baseline source was
`5b9538e5088a1221a41aa29e974a871873804ff4`. Cargo, exact-source Nix Verus,
and Tracey passed before implementation. The baseline had 10,937 verified
obligations and 2,140 covered requirements.

See `evidence/baseline-2026-09-03/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 19 passed |
| Focused Verus | 5 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,942 Verus proofs; 5,012 tests; 2,150 requirements; 614 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,150 |
| Tracey untested | 0 of 2,150 |
| Octet | 3,867 accepted; 0 new; 0 stale |
| JSON artifact contracts | positive and negative fixtures passed |
| Consumer profile | 39 modules, 42 artifacts, and 10 fixtures passed |
| Cairn strict validation | passed |
| Cairn proposal, design, and tasks gates | passed |

The full wrapper ran inside the repository Nix development shell. A temporary
`.worktrees/aspen` symlink restored the existing `../aspen` sibling relation.
The command removed the link after completion.

See `evidence/final-2026-09-03/`.

## Octet Ratchet

The first Octet run found no issue in `src/instance_partition/`. It reported
only the existing `src/lib.rs` file-length identity after the three-line module
declaration.

The guarded migration replaced that ID and preserved every unchanged finding
record. It added no exception, warning budget, or item-partition finding.

See `evidence/octet/`.

## Generated and Bound Artifacts

The change refreshed these repository-owned artifacts after source identity
changed:

- `docs/module-index.md`;
- `consumer-profile/generated/profile.json`, from its Nickel source;
- `docs/proof-evidence-bundle.json`; and
- `docs/proof-evidence-ir-chain.json`.

The consumer profile keeps the same 39 UCAN-facing modules. It updates only the
measured crate-root artifact.

## Boundaries

This evidence proves finite instance-shape, reachability, deduplication,
classification, and partition admission for the checked source and toolchain.
It does not prove:

- trait solving;
- full generic substitution;
- rustc instance resolution;
- generated object or machine code;
- linker or LTO behavior;
- binary-size optimization;
- compile-time performance;
- whole-backend correctness; or
- verifier soundness.
