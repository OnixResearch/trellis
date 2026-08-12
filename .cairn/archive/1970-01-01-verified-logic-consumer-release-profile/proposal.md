## Why

UCAN currently vendors a large `verified-logic` snapshot from Trellis so the UCAN std shell can bridge token, proof, caveat, replay, and Web-of-Trust facts into verified primitives. The vendored copy is almost entirely byte-identical to Trellis, but it is still a duplicated codebase with a manual sync script and a pinned revision file.

Trellis already owns the verified primitive corpus. This change defines a consumer release profile for `verified-logic` so UCAN and similar stack repos can depend on a reviewed crate/profile instead of copying the full source tree, while preserving the option to vendor only as an audited fallback.

## What Changes

- Define a `verified-logic` consumer release profile for stack repos that need Trellis verified primitives without local source duplication.
- Name the supported crate coordinates, feature set, verification evidence, and compatibility receipt shape for consumers such as UCAN.
- Add a vendored-snapshot compatibility report that compares a consumer snapshot to the Trellis release profile and distinguishes approved fallback vendoring from drift.
- Add positive fixtures for an aligned UCAN snapshot and direct dependency profile.
- Add negative fixtures for stale snapshots, missing revision metadata, unverified local patches, and overclaims that imported Trellis proofs certify downstream systems.

## Impact

- **Trellis** remains the proof owner for `verified-logic` primitives.
- **UCAN** can move from vendored snapshot to reviewed dependency when release constraints allow.
- **Consumers** get a bounded compatibility receipt rather than implicit trust in sibling paths or private snapshots.
