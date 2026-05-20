## Why

The README module catalog has drifted from `src/lib.rs` in prior work. Manual updates are easy to miss in a repository with hundreds of public modules and frequent verified-primitive additions.

## What Changes

- Add a deterministic checker that compares public module declarations in `src/lib.rs` against the README module catalog or an explicit documented exclusion list.
- Add focused fixture or current-tree validation so catalog drift is caught by local checks.
- Keep README wording human-authored; the checker only detects missing or stale module entries.

## Scope

In scope:
- Module declaration extraction from `src/lib.rs`.
- README catalog extraction or explicit allowlist/exclusion handling.
- Nix/check integration and docs update.

Out of scope:
- Auto-generating the full README module descriptions.
- Changing module organization.
- Requiring every private/internal helper to appear in README.

## Impact

- **Files**: new script/checker, README, `flake.nix`, optional fixture files.
- **APIs**: no library API changes.
- **Testing**: checker pass on current tree plus negative fixture or synthetic README omission test.
