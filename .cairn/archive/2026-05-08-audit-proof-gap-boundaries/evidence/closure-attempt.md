# Closeout attempt evidence

## Decision

No safe closeable boundary family is selected for implementation in this audit slice.

## Audit result

The only remaining `external_body` sites are classified as permanent cryptographic/runtime/codec trust or as the documented `nix_version::compare_versions` exec/spec solver-limit bridge. Removing any of these in a narrow pass would either overclaim cryptographic/codec correctness or require a larger redesign of the Nix version exec/spec proof.

## Replacement-marker check

The source-derived inventory in `proof-gap-inventory.md` reports:

- `assume_specification`: 0
- `@verified-logic:no-ensures`: 0
- `@verified-logic:exempt`: 0
- textual `assume(` under `src/`: 0

A small verified audit-constant module (`src/proof_gap_boundary_audit.rs`) was added to give Tracey-covered constants and proof functions for the source-derived counts, classification result, no-replacement-marker policy, and unchanged published count. No trusted boundary was removed, so published trusted-boundary counts remain unchanged at 16.
