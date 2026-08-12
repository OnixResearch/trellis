## Why

UCAN exposes many verified bridge helpers that delegate to vendored Trellis `verified-logic` primitives. The relationship is documented, but there is no Trellis-owned coverage manifest that maps each UCAN-facing bridge to an exported primitive/proof row and detects stale vendor coverage.

## What Changes

- Add a UCAN bridge coverage manifest in Trellis.
- Map UCAN public helper names to Trellis primitive IDs, proof artifacts, trusted boundaries, and non-claims.
- Add positive and negative fixtures for covered bridges, stale vendored rows, missing primitive IDs, and overbroad verified claims.

## Impact

- UCAN can verify vendored bridge coverage against Trellis-owned primitive identity.
- Trellis keeps proof ownership while UCAN owns token orchestration.
- Downstream consumers can see which UCAN helper is backed by which Trellis primitive.
