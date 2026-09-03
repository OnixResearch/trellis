# MIR Core Verification

## Result

The finite MIR-core implementation is complete. Strict validation and the
proposal, design, and tasks gates pass. Sync created the accepted specification,
and archive execution completed under `2026-09-03-mir-core-primitives`.

The public `mir_core` facade composes narrow model, admission, proof,
downstream-fact, and test components. Every production file has fewer than
300 lines. The functional core performs no filesystem, process, clock, network,
compiler, or other external effect.

The model provides:

- `u32`-indexed local and block identities;
- a named `RETURN_PLACE`;
- finite local declarations, places, projections, operands, and shallow rvalues;
- basic blocks with zero or more statements and one final terminator;
- deterministic first-failure classes and body-relative locations;
- checked successor ranges; and
- validated block counts, local counts, and ordered CFG edges for downstream analyses.

Oversized host tables fail with `TableTooWide`. Unsupported projections,
nested-rvalue markers, missing terminators, invalid local references, and
out-of-range successors fail before adapter facts become available.

## Baseline

The baseline source was
`b957d58f7c55f1d3d77c1043b4927d9e64863e25`. Cargo tests passed before the
core change. The first direct full Verus run hit two resource limits in the
pre-existing `nix_drv_serialize.rs` module. The exact-source Nix proof check
then passed with 10,877 verified obligations and zero errors.

See `evidence/baseline-2026-09-03/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 15 passed |
| Focused Verus | 21 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,898 Verus proofs; 4,934 tests; 2,109 requirements; 610 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,109 |
| Tracey untested | 0 of 2,109 |
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

The first Octet run found 14 new findings. Thirteen applied to the new module.
The implementation removed all thirteen through fixed-width public indices,
owned edge buffers, and a non-repeating module path.

The final `src/mir_core/` tree has zero Octet findings. Its three-line
crate-root declaration changed only the existing `src/lib.rs` file-length
identity. The guarded migration replaced that ID and preserved every unchanged
finding record. It added no exception, warning budget, or MIR-core finding.

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

This evidence proves finite structural MIR-core admission for the checked
source and toolchain. It does not prove:

- full rustc MIR compatibility or parsing;
- Rust type or lifetime correctness;
- borrow checking or drop elaboration;
- MIR execution semantics;
- optimization preservation or soundness;
- linker or code-generation correctness;
- whole-compiler correctness; or
- verifier soundness.
