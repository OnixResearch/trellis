# Typed Configuration Normal Form Verification

## Result

The finite typed normal-form implementation is complete. Strict validation and
the proposal, design, and tasks gates pass. Sync updated the accepted
`domain-primitives` specification, and archive execution completed under
`2026-09-03-dhall-style-total-config`.

The public `total_config` facade composes narrow model, graph, schema, import,
normalization, refactor, proof, and test components. Every production file has
fewer than 300 lines. The functional core performs no filesystem, environment,
network, parser, evaluator, digest, or other external effect.

The model provides:

- finite typed scalar, list, record, variant, optional, reference, and import nodes;
- unique node keys, declared child references, and cycle rejection;
- required record fields with expected child types;
- canonical node-key and record-field order;
- source-label and comment independence;
- opaque fixed-width dependency digest metadata;
- deterministic normal-form and digest-input selection;
- static frozen imports with remote attenuation and same-origin header policy;
- digest-aware refactor compatibility; and
- deterministic first-failure table, index, and subject metadata.

The core selects digest input but computes no hash. Stack consumers can apply
BLAKE3 outside this proof boundary. Import success covers only supplied metadata
and does not perform an import.

## Baseline

The baseline source was
`891690467fcd40f989c520c590f8ceda5b4305ce`. Cargo, exact-source Nix Verus,
and Tracey passed before implementation. The baseline had 10,948 verified
obligations and 2,161 covered requirements.

See `evidence/baseline-2026-09-03/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 25 passed |
| Focused Verus | 7 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,955 Verus proofs; 5,060 tests; 2,173 requirements; 616 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,173 |
| Tracey untested | 0 of 2,173 |
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

The first Octet run found four new findings. Three applied to the new component.
The implementation removed them by merging narrow modules, splitting cycle
admission, and using a non-repeating graph path.

The final `src/normal_input/` tree has zero Octet findings. Its three-line
crate-root declaration changed only the existing `src/lib.rs` file-length
identity. The guarded migration replaced that ID and preserved every unchanged
finding record. It added no exception, warning budget, or normal-form finding.

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

This evidence proves finite type, graph, normal-form, digest-input, import-policy,
and refactor admission for supplied metadata. It does not prove:

- Dhall syntax or language compatibility;
- Nickel evaluation;
- parser or evaluator correctness;
- network, filesystem, or environment security;
- actual import fetching or cache behavior;
- BLAKE3 implementation or collision resistance;
- downstream system correctness; or
- verifier soundness.
