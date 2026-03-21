# Napkin — verified-logic

## Project State
- Infrastructure complete: Tracey config, requirements, module structure, verify script
- 10/10 requirements covered and verified (tests)
- Verus NOT in devshell — not packaged in nixpkgs. Installed separately.
- Tracey NOT in nixpkgs either — installed via cargo-dist system-wide
- Verus spec files are stubs — proofs not written yet (tests cover `verify` for now)

## Decisions
- Library crate: reusable verified logic for other projects
- Combined pattern preferred (exec fn with ensures), split pattern when spec is shared
- All public API gets verified contracts
- Tracey tracks requirement → code coverage
- Inline tests carry `r[verify]` annotations — sufficient for Tracey coverage
- Verus proofs will supplement tests once verus is wired up

## Mistakes & Corrections
- Tried to add `tracey` to flake.nix devshell — not in nixpkgs. Removed.
  Tracey is installed system-wide via cargo-dist.
- Cargo.toml had binary-crate deps from scaffold (tokio, clap, etc.) — stripped to zero deps.
- Needed `git add` + `Cargo.lock` before `nix develop` would work.
