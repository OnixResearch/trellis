## Why

The Rust verification gate scripts are now part of the repository's trusted local quality surface. They currently prove that the live tree passes, but they do not yet prove that the scripts reject known-bad inputs or preserve tricky parser behavior across future edits.

## What Changes

- Add fixture-backed regression tests for `scripts/check-coverage.rs`, `scripts/check-ensures.rs`, and `scripts/check-iterative-exec.rs`.
- Cover both accepted and rejected fixtures for the checker rules that matter most.
- Wire the self-tests into Nix checks so checker parser drift is caught before full-tree verification claims.

## Scope

In scope:
- Synthetic fixture files and expected pass/fail assertions.
- A deterministic script self-test mode or a small Rust/Nix harness.
- Flake integration for the checker self-tests.

Out of scope:
- Rewriting the checker scripts into a crate.
- Changing checker policy semantics beyond what fixtures expose.
- Broad changes to `scripts/verify.rs` orchestration.

## Impact

- **Files**: `scripts/check-*.rs`, `tests/fixtures/**`, `flake.nix`, docs references if needed.
- **APIs**: no library API changes.
- **Testing**: targeted checker fixture runs, Nix checks, and full `./scripts/verify.rs` before archive.
