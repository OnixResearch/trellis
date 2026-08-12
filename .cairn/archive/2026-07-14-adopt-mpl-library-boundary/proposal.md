## Why

Trellis is a reusable verified-logic library and proof corpus. Its current AGPL package expression forces strong combined-work licensing concerns onto every Rust consumer, which works against reuse of verified primitives and causes AGPL to propagate through otherwise separable protocol cores.

## What Changes

- License Trellis-owned `verified-logic` source under MPL-2.0.
- Ship the complete MPL text and document file-level copyleft and proof non-claims.
- Preserve third-party, verifier, and referenced artifacts under their own terms.
- Update consumer guidance so vendored snapshots preserve the MPL boundary.

## Impact

Modifications to Trellis source remain open at the file level, while larger linked applications may use their own license. Existing AGPL copies retain their prior grants.
