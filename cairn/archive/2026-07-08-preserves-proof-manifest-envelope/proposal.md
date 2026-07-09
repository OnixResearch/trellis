## Why

Trellis proof identity should stay in Trellis-owned manifests and Verus proof artifacts, but stack consumers may want canonical Preserves carriers for proof-manifest snapshots. A bounded envelope contract gives consumers a stable artifact without making Preserves part of proof semantics.

## What Changes

- Add an optional Preserves proof-manifest envelope contract.
- Wrap Trellis verified primitive/proof manifest rows into canonical Preserves envelopes linked to source manifest digests.
- Reject envelopes with stale source identity, missing primitive/proof rows, overbroad verified claims, or missing non-claims.

## Impact

- UCAN, Valence, Mantle, and Cairn can cite Trellis proof-manifest snapshots as `.preserves` carriers.
- Trellis keeps proof ownership in Verus/spec code and manifest rows.
