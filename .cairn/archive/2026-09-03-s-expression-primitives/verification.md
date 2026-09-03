# S-expression Primitives Verification

## Result

The finite canonical-tree implementation is complete. Strict validation and the
proposal, design, and tasks gates pass. Sync created the accepted specification,
and archive execution completed under `2026-09-03-s-expression-primitives`.

The public `s_expression` facade composes narrow model, admission, rendering,
reader, roundtrip, proof, and test components. Every production file has fewer
than 300 lines. The functional core performs no filesystem, process, clock,
network, evaluator, or other external effect.

The model provides:

- dialect-neutral symbol and quoted-text atoms;
- ordered atom-or-list trees;
- named maximum node, depth, and UTF-8 byte bounds;
- checked node, depth, and canonical-byte measures;
- canonical parentheses, one-space separators, and text escapes;
- canonical-only bounded parsing with byte offsets;
- deterministic atom, escape, delimiter, spacing, trailing, and budget failures;
- iterative bounded measurement and rendering stacks; and
- mechanical canonical serialize/parse roundtrip checks.

The reader accepts no comments, quote sugar, dotted pairs, labels, or external
dialect extensions. Raw controls, unknown escapes, non-canonical spaces,
unbalanced delimiters, trailing input, and exhausted budgets fail before a tree
identity becomes available.

## Baseline

The baseline source was
`2a4529bcefdbeef07d4bf8cd4528a1b7e902a2e6`. Cargo, exact-source Nix Verus,
and Tracey passed before implementation. The baseline had 10,942 verified
obligations and 2,150 covered requirements.

See `evidence/baseline-2026-09-03/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 23 passed |
| Focused Verus | 6 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,948 Verus proofs; 5,035 tests; 2,161 requirements; 615 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,161 |
| Tracey untested | 0 of 2,161 |
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

The first Octet run found eight new findings. Seven applied to the new component.
The implementation removed all seven through explicit bounded stacks, narrow
output state, character-returning escape decoding, and path-local names.

The final `src/paren_tree/` tree has zero Octet findings. Its three-line
crate-root declaration changed only the existing `src/lib.rs` file-length
identity. The guarded migration replaced that ID and preserved every unchanged
finding record. It added no exception, warning budget, or canonical-tree finding.

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

This evidence proves finite tree admission, canonical rendering, bounded parse
admission, and checked roundtrip behavior for the supplied grammar. It does not
prove:

- evaluator semantics;
- macros, quote sugar, or symbol hygiene;
- interning or module loading;
- bytecode safety;
- filesystem, network, or process behavior;
- Steel, Scheme, Common Lisp, or SMT-LIB compatibility;
- parser completeness for an external dialect;
- whole-consumer correctness; or
- verifier soundness.
