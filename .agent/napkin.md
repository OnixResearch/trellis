# Napkin — verified-logic

## Project State
- 10/10 requirements covered (impl) and verified (verify) per Tracey
- 22 Verus proofs pass across 4 spec files
- 11 cargo tests pass across 4 modules
- Verus runs via `scripts/verus.sh` wrapper (nix store paths for rust_verify + z3 + rustc libs)
- Combined check: `scripts/verify.sh` runs verus + tracey in sequence

## Decisions
- Library crate: reusable verified logic for other projects
- Split pattern used: `src/` has runtime code with `r[impl]`, `verus/` has specs/proofs with `r[depends]`/`r[verify]`
- Tests in `src/` carry `r[verify]` annotations for Tracey coverage
- Verus proofs in `verus/` also carry `r[verify]` annotations — both test and proof verify the same requirement
- `spec fn` definitions carry `r[depends]` — they formally depend on the requirement text

## Verus Toolchain
- verus-root: `/nix/store/7hr3z8ss29pxl29pllv6kchlk9kbckys-verus-root-0.2026.01.30`
- rust toolchain (for verus): `/nix/store/27f9852zp2qlg34zmqjz27cg6sq680sx-rust-default-1.93.0`
- Version: 0.2026.01.30 (needs rustc 1.93.0 libs + z3 bundled in verus-root)
- Wrapper: `scripts/verus.sh` sets LD_LIBRARY_PATH, VERUS_Z3_PATH, extern flags

## Mistakes & Corrections
- Tried to add `tracey` to flake.nix devshell — not in nixpkgs. Removed.
- Verus wrapper script (`verus`) needs rustup — doesn't work in nix. Used `rust_verify`
  directly with LD_LIBRARY_PATH pointing at the matching 1.93.0 rustc libs.
- Recursive spec fn needs `decreases` clause (sorted_insert_spec).
