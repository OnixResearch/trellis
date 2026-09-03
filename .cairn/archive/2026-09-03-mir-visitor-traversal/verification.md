# MIR Visitor Traversal Verification

## Result

The finite MIR visitor implementation is complete. Strict validation and the
proposal, design, and tasks gates pass. Sync created the accepted specification,
and archive execution completed under `2026-09-03-mir-visitor-traversal`.

The public `mir_visitor_traversal` facade composes narrow location, traversal,
coverage, mutation, proof, and test components. Every production file has fewer
than 300 lines. The functional core performs no filesystem, process, clock,
network, compiler, or other external effect.

The model provides:

- explicit body, local, block, statement, terminator, place, operand, rvalue,
  and successor locations;
- exactly-once and permissive coverage modes;
- deterministic iterative preorder and reverse postorder;
- reachable-only and all-block policies;
- immutable and value-only mutation classes;
- admitted-body shape checks before mutable facts become available; and
- deterministic first-failure classes and trace locations.

Traversal vectors reserve the admitted body block count before bounded growth.
Mutable traces validate both bodies and preserve local, block, statement,
terminator, and successor counts.

## Baseline

The baseline source was
`41cc039ce8c72e494cba4bfb598132cae1083790`. Cargo, exact-source Nix Verus,
and Tracey passed before implementation. The baseline had 10,898 verified
obligations and 2,109 covered requirements.

See `evidence/baseline-2026-09-03/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 20 passed |
| Focused Verus | 17 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,915 Verus proofs; 4,954 tests; 2,119 requirements; 611 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,119 |
| Tracey untested | 0 of 2,119 |
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

The first Octet run found six new findings. Five applied to the new component.
The implementation removed all five through exhaustive matching, bounded
allocation, positive Boolean naming, and non-repeating file paths.

The final `src/mir_walk/` tree has zero Octet findings. Its three-line crate-root
declaration changed only the existing `src/lib.rs` file-length identity. The
guarded migration replaced that ID and preserved every unchanged finding
record. It added no exception, warning budget, or visitor finding.

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

This evidence proves finite location, coverage, order, reachability, and shape
admission for the checked source and toolchain. It does not prove:

- rustc visitor-trait or macro correctness;
- compiler borrow or lifetime behavior;
- a pass transformation's semantics;
- optimization preservation or soundness;
- callback execution or fairness;
- whole-compiler correctness; or
- verifier soundness.
