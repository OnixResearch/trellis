# Proof Artifact Manifest

Trellis publishes `docs/proof-artifact-manifest.json` as a reviewed boundary for exported verified primitives. The manifest is intentionally narrow: it records stable primitive IDs, source/proof artifact BLAKE3 identities, byte counts, Verus command identity, trusted-boundary classification, recommended evidence tier, requirement IDs, and non-claims.

## Contract

The Nickel contract is `docs/proof-artifact-manifest.contract.ncl`. The checker is `scripts/check-verified-primitive-manifest.rs`.

Each row must include:

- a stable `trellis.*` primitive ID;
- the module path and public API names exported by that source file;
- source artifact BLAKE3 and byte count;
- proof artifact BLAKE3 and byte count for `formal_proof` rows;
- verifier identity and argv;
- trusted-boundary classification;
- evidence-tier recommendation;
- downstream non-claims.

## Preserves carrier

`docs/proof-artifact-manifest.preserves-envelope.json` records optional Preserves carrier metadata for `docs/proof-artifact-manifest.preserves`. The carrier exists for interchange only: proof ownership stays with Verus/spec artifacts and Trellis manifest rows. The carrier is not a proof artifact, not downstream authorization semantics, and not verifier soundness evidence.

## Downstream consumption

UCAN vendor sync can record a consumed manifest digest next to vendored `verified-logic` revision metadata. If UCAN does not use the manifest, the sync record should say so explicitly rather than implying proof certification from vendored bytes.

Octet proof imports may reference Trellis primitive IDs, but imported rows stay external/manual-review evidence until an Octet-owned verifier rail reruns and emits its own receipt.

Valence may use primitive IDs and artifact identities as linkage/provenance metadata only. Valence does not turn a Trellis manifest row into behavioral correctness.

Cairn gates may require presence, freshness, and non-claim visibility for a consumed manifest row. Cairn does not prove source-code correctness or release readiness from the row alone.

Basalt, Aspen, and Molten may bind adapter or UCAN-facing evidence to primitive IDs, but they must keep integration checks and runtime boundary claims local to their own receipts.

## Non-claims

A manifest row does not prove downstream integration correctness, consumer release eligibility, or verifier soundness outside the declared proof boundary. Trusted-boundary, proof-gap, manual-review, and recorded-only rows must not be promoted to `formal_proof` unless a downstream-owned verifier receipt supports that exact property.

A Preserves envelope is a carrier for proof-manifest identity only. It does not change proof ownership, replace Verus/spec artifacts, prove downstream authorization semantics, or certify release eligibility.

## Refresh

After changing exported primitives or proof files, refresh the row identity and run:

```bash
./scripts/check-verified-primitive-manifest.rs
./scripts/check-verified-primitive-manifest.rs --self-test
```
