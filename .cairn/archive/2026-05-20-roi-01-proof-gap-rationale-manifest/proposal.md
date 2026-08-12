## Why

The proof-gap inventory now checks counts, exact sites, classifications, and README headline freshness, but the checked manifest still records only path, function signature, and classification. Reviewers must read README/OpenSpec prose to understand why each permanent trusted boundary is acceptable. That weakens future proof-gap closure work because rationale can drift independently from the source-derived manifest.

## What Changes

- Extend the proof-gap manifest contract so every tracked trusted boundary carries a concise rationale/description field.
- Teach the Rust proof-gap inventory checker to reject malformed, empty, duplicate, unsorted, or stale rationale-bearing manifest rows.
- Add deterministic fixture/self-test coverage for missing or malformed rationale fields.
- Wire the stronger checker through local verification and Nix evidence.

## Scope

In scope:
- `docs/proof-gap-sites.tsv` schema and rows.
- `scripts/check-proof-gap-inventory.rs` validation logic and controlled fixture modes.
- `scripts/verify.rs`, `flake.nix`, README/checker documentation, and verification evidence.
- Synced `verification-gaps` OpenSpec requirements.

Out of scope:
- Closing or narrowing any trusted boundary in this change.
- Changing cryptographic trust assumptions.
- Broad README proof-gap prose rewrites beyond linking the manifest as the rationale source of truth.

## Impact

- **Files**: `docs/proof-gap-sites.tsv`, `scripts/check-proof-gap-inventory.rs`, `scripts/verify.rs`, `flake.nix`, README/docs, and `openspec/specs/verification-gaps/spec.md` after archive.
- **APIs**: no public Rust API changes.
- **Testing**: proof-gap inventory checker, controlled negative fixture/self-test modes, Nix check, `TMPDIR=/tmp ./scripts/verify.rs`, and Tracey coverage if requirement/docs IDs change.
