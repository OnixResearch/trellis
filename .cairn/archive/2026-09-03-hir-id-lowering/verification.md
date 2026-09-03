# HIR ID Lowering Verification

## Result

The finite HIR ID-lowering implementation is complete. Strict validation and
the proposal, design, and tasks gates pass. Sync created the accepted
specification, and archive execution completed under `2026-09-03-hir-id-lowering`.

The public `hir_id_lowering` facade composes narrow model, source-catalog,
lifecycle, admission, proof, and test components. Every production file has
fewer than 300 lines. The functional core performs no filesystem, process,
clock, network, compiler, or other external effect.

The model provides:

- distinct owner, source, scoped HIR, and definition key types;
- a bound of `MAX_RECORDS` entries per finite table;
- complete source-to-HIR mappings;
- active-owner checks for every scoped key;
- unique source-mapped and generated targets;
- fresh generated targets with unique provenance;
- required-use and explicit-discard policy for created keys;
- deterministic definition correspondence; and
- stable first-failure table, index, and related-value metadata.

Mappings and generated nodes must name created keys with required-use policy.
Stale generated keys, unlowered sources, wrong owners, duplicate keys, and
missing definitions fail before count facts become available.

## Baseline

The baseline source was
`792e6900477e4af42a5792ef4cb43bc97da4843f`. Cargo, exact-source Nix Verus,
and Tracey passed before implementation. The baseline had 10,915 verified
obligations and 2,119 covered requirements.

See `evidence/baseline-2026-09-03/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 18 passed |
| Focused Verus | 10 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,925 Verus proofs; 4,972 tests; 2,129 requirements; 612 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,129 |
| Tracey untested | 0 of 2,129 |
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

The first Octet run found nine new findings. Eight applied to the new component.
The implementation removed all eight through path-local vocabulary that does
not repeat the public module path.

The final `src/frontend_lower/` tree has zero Octet findings. Its three-line
crate-root declaration changed only the existing `src/lib.rs` file-length
identity. The guarded migration replaced that ID and preserved every unchanged
finding record. It added no exception, warning budget, or HIR ID finding.

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

This evidence proves finite owner, mapping, freshness, use, uniqueness, and
definition-metadata admission for the checked source and toolchain. It does not
prove:

- full AST lowering correctness;
- AST-to-HIR semantic equivalence;
- parser or macro-expansion correctness;
- name resolution;
- HIR or THIR semantics;
- Rust type checking;
- whole-frontend correctness; or
- verifier soundness.
