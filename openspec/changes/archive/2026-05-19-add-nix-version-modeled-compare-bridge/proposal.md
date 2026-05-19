## Why

`src/nix_version.rs::compare_versions` remains an explicit trusted bridge for the full Nix version comparison API. The canonical verification-gaps spec already calls out a bounded modeled-compare bridge as the next structural decomposition: compute admitted finite-model comparisons through verified executable helpers while preserving the broader trusted boundary for out-of-model inputs.

## What Changes

- Add a bounded finite-model admission surface for Nix version byte sequences.
- Add verified executable helpers for separator skipping, component bounds, numeric parsing, and component/lexicographic comparison over admitted inputs.
- Add `compare_versions_modeled` as the explicit wrapper that returns `Some(-1)`, `Some(0)`, or `Some(1)` for admitted inputs and `None` for out-of-model inputs.
- Keep the existing `compare_versions` trusted bridge visible; this change narrows claims instead of proving all Nix comparator behavior.

## Capabilities

### Modified Capabilities
- `verification-gaps.nix-version-modeled-compare-bridge`: turns the existing canonical requirement into an implemented, verified, evidence-backed structural wrapper.
- `verification-gaps.boundary-audit.inventory`: preserves the trusted-boundary inventory and prevents replacement weakened markers.

## Impact

- **Files**: Expected implementation in `src/nix_version.rs`; possible requirement/evidence wording updates in `docs/requirements.md`, `openspec/specs/verification-gaps/spec.md`, and archived evidence under this change.
- **APIs**: Adds bounded modeled/admission helpers and tests. Existing `compare_versions` behavior remains compatible and trusted.
- **Dependencies**: No new external dependencies expected.
- **Testing**: Focused `nix_version` Rust tests, `nix develop -c ./scripts/verus.sh src/lib.rs`, proof-gap inventory gate, Tracey uncovered/untested, strict OpenSpec validation, and final maintained `./scripts/verify.sh` before archive.
