# MIR Dataflow Framework Verification

## Result

The finite analysis implementation is complete. Strict validation and the
proposal, design, and tasks gates pass. Sync created the accepted specification,
and archive execution completed under `2026-09-03-mir-dataflow-framework`.

The public `mir_dataflow_framework` facade composes narrow model, lattice,
table, engine, cursor, proof, and test components. Every production file has
fewer than 300 lines. The functional core performs no filesystem, process,
clock, network, compiler, or other external effect.

The model provides:

- bounded finite state domains with bottom, top, and height;
- exhaustive join-table shape and semilattice-law checks;
- exhaustive transfer-table range and monotonicity checks;
- forward and backward location order;
- before effects applied immediately before primary effects;
- unique, admitted CFG edge effects;
- a named height-and-work-item convergence budget;
- deterministic bounded fixed-point execution;
- direction-relative block entry and exit states; and
- pure location and edge cursors with exactly-once requested inspection.

The engine returns results only after the next block-entry vector equals the
current vector. It rejects an insufficient or exhausted budget. Framework
admission does not establish soundness for a concrete Rust analysis.

## Baseline

The baseline source was
`8c3655903c51e32e3ae7cfc3f547ef03870e389c`. Cargo, exact-source Nix Verus,
and Tracey passed before implementation. The baseline had 10,955 verified
obligations and 2,173 covered requirements.

See `evidence/baseline-2026-09-03/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 22 passed |
| Focused Verus | 7 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,962 Verus proofs; 5,082 tests; 2,185 requirements; 617 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,185 |
| Tracey untested | 0 of 2,185 |
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

The first Octet run found three new findings. Two applied to the new component.
Moving transfer helpers to a non-repeating table module removed both findings.

The final `src/finite_flow/` tree has zero Octet findings. Its three-line
crate-root declaration changed only the existing `src/lib.rs` file-length
identity. The guarded migration replaced that ID and preserved every unchanged
finding record. It added no exception, warning budget, or finite-analysis finding.

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

This evidence proves finite domain, lattice, transfer, edge, convergence, and
result-inspection admission for supplied tables. It does not prove:

- a borrow checker or move checker;
- initializedness or liveness analysis soundness;
- MIR execution semantics;
- rustc dataflow-engine behavior;
- pass correctness or optimization soundness;
- whole-compiler correctness; or
- verifier soundness.
