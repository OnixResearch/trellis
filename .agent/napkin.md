# Napkin — verified-logic

## Project State
- 10/10 requirements covered (impl) and verified (verify) per Tracey
- 22 Verus proofs pass across 4 spec files
- 33 cargo tests pass: 11 unit + 22 property-based (proptest)
- Property tests bridge the spec-implementation gap: each `_ref` function mirrors
  the Verus spec, proptest fuzzes `impl(a,b) == ref(a,b)` for random inputs
- Full pipeline: clippy → tests → verus → tracey, all strict (exit 1 on failure)

## Decisions
- Library crate: reusable verified logic for other projects
- Split pattern used: `src/` has runtime code with `r[impl]`, `verus/` has specs/proofs with `r[depends]`/`r[verify]`
- Tests in `src/` carry `r[verify]` annotations for Tracey coverage
- Verus proofs in `verus/` also carry `r[verify]` annotations — both test and proof verify the same requirement
- `spec fn` definitions carry `r[depends]` — they formally depend on the requirement text
- Hand-implemented `saturating_add` and `saturating_sub` (no stdlib delegation) so Verus
  can verify the full logic chain
- `checked_mul` still delegates to stdlib — its internals are compiler intrinsics that
  can't be hand-implemented without unsafe

## Verus Toolchain
- verus-root: `/nix/store/7hr3z8ss29pxl29pllv6kchlk9kbckys-verus-root-0.2026.01.30`
- rust toolchain (for verus): `/nix/store/27f9852zp2qlg34zmqjz27cg6sq680sx-rust-default-1.93.0`
- Version: 0.2026.01.30 (needs rustc 1.93.0 libs + z3 bundled in verus-root)
- Wrapper: `scripts/verus.sh` sets LD_LIBRARY_PATH, VERUS_Z3_PATH, extern flags

## Clippy Policy
- Correctness + suspicious lints: deny
- Pedantic lints: warn
- `manual_abs_diff`, `manual_map`, `manual_saturating_arithmetic`, `manual_unwrap_or`,
  `implicit_saturating_sub`: allow project-wide (hand-written verified logic)
- Cast lints (truncation, sign loss, wrap): deny
- `unsafe_code`: forbid
- `integer_division`: allow in test modules only (proptest range construction)
- Config lives in `[lints.clippy]` section of Cargo.toml

## Verification Layers
1. **Verus proofs** — mathematical properties of spec functions (22 proofs)
2. **Property-based conformance tests** — fuzz that `impl == spec_ref` for random inputs (22 proptests)
3. **Unit tests** — spot checks at boundaries and edge cases (11 tests)
4. **Clippy** — static analysis with strict correctness/suspicious deny
5. **Tracey** — requirement-to-code traceability, strict uncovered/untested checks

## Mistakes & Corrections
- Tried to add `tracey` to flake.nix devshell — not in nixpkgs. Removed.
- Verus wrapper script (`verus`) needs rustup — doesn't work in nix. Used `rust_verify`
  directly with LD_LIBRARY_PATH pointing at the matching 1.93.0 rustc libs.
- Recursive spec fn needs `decreases` clause (sorted_insert_spec).
- Clippy `manual_abs_diff` and `manual_map` fire on verified implementations — allow project-wide.
- Hand-implementing saturating_add/sub triggered `manual_unwrap_or` and `implicit_saturating_sub`
  lints — added to project-wide allows.
- Test reference functions using `as u128` triggered `cast_lossless` — use `u128::from()`.
- Test reference function `wide as u64` triggered `cast_possible_truncation` — use `try_from().unwrap()`.
- `prop_saturating_add_bounded` was trivially true (`u64 <= u64::MAX`) — removed.
- `v.sort()` on u64 → use `v.sort_unstable()`.
- `u64::MAX / 2` in proptest ranges triggered `integer_division` — allow in test module.
