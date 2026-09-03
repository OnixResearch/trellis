# MIR Drop Elaboration Verification

## Result

The finite destruction-decision implementation is complete. Strict validation
and the proposal, design, and tasks gates pass. Sync created the accepted
specification, and archive execution completed under `2026-09-03-mir-drop-elaboration`.

The public `mir_destruct` facade composes narrow model, path, fact-transition,
decision, proof, and test components. Every production file has fewer than 300
lines. The functional core performs no destructor, unwind, allocation,
filesystem, process, clock, network, or compiler effect.

The model provides:

- admitted MIR structural paths with parent and depth metadata;
- direct, field, enum-variant, and no-glue type shapes;
- finite maybe-initialized, maybe-uninitialized, and obligation sets;
- subtree initialization, move, and overwrite updates;
- static, dead, conditional, and open classifications;
- deterministic flag identities for conditional steps;
- bounded iterative field and active-variant expansion;
- inactive-variant and dead-path omission; and
- old-target call metadata before one replacement write.

No-glue targets emit no call. Missing facts, malformed paths, unsupported
metadata, invalid variants, and exhausted budgets fail before plan metadata
becomes available.

## Baseline

The baseline source was
`44226097b7e73add55e7a809fc7353c2645e4971`. Cargo, exact-source Nix Verus,
and Tracey passed before implementation. The baseline had 10,962 verified
obligations and 2,185 covered requirements.

See `evidence/baseline-2026-09-03/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 19 passed |
| Focused Verus | 7 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,969 Verus proofs; 5,101 tests; 2,196 requirements; 618 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,196 |
| Tracey untested | 0 of 2,196 |
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
Positive-predicate binding names removed both findings.

The final `src/destruct_paths/` tree has zero Octet findings. Its three-line
crate-root declaration changed only the existing `src/lib.rs` file-length
identity. The guarded migration replaced that ID and preserved every unchanged
finding record. It added no exception, warning budget, or destruction finding.

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

This evidence proves finite structural obligation, initializedness,
classification, expansion, and replacement-order metadata. It does not prove:

- user destructor behavior;
- panic or unwind cleanup correctness;
- rustc drop-shim generation;
- const-evaluation policy;
- unsafe-code soundness;
- whole-program memory safety; or
- verifier soundness.
