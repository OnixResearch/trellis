## Why

Trellis is the authoritative proof corpus for `verified-logic`, but downstream consumers often see only crate bytes or UCAN's vendored snapshot. That makes it hard to tell which verified primitive, proof artifact, Verus command, trusted boundary, and non-claim a consumer is actually relying on.

This change adds a Trellis-owned verified primitive manifest so UCAN, Basalt, Aspen/Molten, Octet, Valence, and Cairn can consume explicit proof-artifact identity instead of inferring semantics from a vendored crate or a Cargo revision.

## What Changes

- Add a reviewed manifest family for exported verified primitives and proof artifacts.
- Record module/API identity, source BLAKE3, proof BLAKE3, Verus command, trusted-boundary classification, requirement IDs, and non-claims.
- Provide positive fixtures for exported primitives and negative fixtures for stale hashes, missing Verus commands, missing non-claims, and overbroad proof claims.
- Document that the manifest proves artifact identity and checked-local proof status only; downstream repos must still validate their own receipts and integration boundaries.

## Impact

- **Trellis** owns the primitive/proof manifest and its freshness gate.
- **UCAN** can bind vendored `verified-logic` imports to a Trellis manifest row.
- **Octet/Valence/Cairn** can distinguish formal proof, manual review, recorded-only, and trusted-boundary evidence without guessing from repo paths.
- **Testing** needs contract validation plus a fixture matrix covering positive and negative manifest rows.
