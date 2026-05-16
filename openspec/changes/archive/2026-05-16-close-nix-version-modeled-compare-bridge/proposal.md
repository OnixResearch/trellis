## Why

The remaining trusted-boundary inventory is stable at 16 sites, and most are permanent cryptographic or codec trust. The highest-ROI structural seam is `src/nix_version.rs::compare_versions_modeled`: it currently narrows input admission but still delegates admitted inputs through the trusted `compare_versions` external body. A verified modeled comparator would turn the finite admitted wrapper into real executable evidence without claiming the full Nix comparator bridge is closed.

## What Changes

- Specify a verified admitted-input comparator path for the existing finite Nix version model.
- Keep the full `compare_versions` external body in the trusted inventory unless the broader bridge is separately closed.
- Add focused runtime tests and Verus proof coverage showing admitted modeled inputs compute `compare_versions_spec` without calling the trusted wrapper.

## Scope

In scope:
- `src/nix_version.rs` finite admitted-model comparator helpers/wrapper.
- `docs/requirements.md` and `openspec/specs/verification-gaps/spec.md` synchronization for the narrowed claim.
- Focused Verus/tests, Tracey uncovered/untested, and proof-gap inventory checks.

Out of scope:
- Removing the full `compare_versions` external body.
- Proving arbitrary unbounded Nix version strings or changing Nix version ordering semantics.
- Reclassifying cryptographic or base58/Ed25519 trusted boundaries.

## Verification

Validate this OpenSpec package, then implement the modeled comparator with `r[impl]` / `r[verify]` coverage. Run `./scripts/verus.sh src/nix_version.rs`, focused Nix-version tests, `tracey query uncovered`, `tracey query untested`, the proof-gap inventory gate, and the full repository verification gate before archive.
