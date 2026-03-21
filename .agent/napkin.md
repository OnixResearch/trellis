# Napkin — verified-logic

## Project State
- Verus specs embedded in `src/` — no separate `verus/` directory
- Every exec fn has `ensures result == *_spec(args)` — Verus verifies the actual implementation
- Drift between spec and impl is structurally impossible
- 21 Verus proofs + 25 cargo tests (10 unit + 15 property) = all gates green
- `vstd` is a cargo dependency — `verus!` blocks compile normally (ghost code erased)
- `rust_verify` runs on `src/lib.rs` — verifies the whole crate

## Architecture
- `spec fn *_spec(...)` — mathematical definition (ghost, erased at compile time)
- `exec fn *(...)` with `ensures result == *_spec(args)` — runtime code, verified to match spec
- `proof fn` — proves properties of the spec (symmetry, idempotent, bounded, etc.)
- `#[test]` + proptest — runtime verification + fuzz testing

## Verus Toolchain
- verus-root: `/nix/store/7hr3z8ss29pxl29pllv6kchlk9kbckys-verus-root-0.2026.01.30`
- rust toolchain (for verus): `/nix/store/27f9852zp2qlg34zmqjz27cg6sq680sx-rust-default-1.93.0`
- vstd crates.io version: `=0.0.0-2026-01-25-0057`
- Wrapper: `scripts/verus.sh` runs rust_verify on src/lib.rs with correct extern paths

## Clippy Policy
- Correctness + suspicious: deny
- Pedantic: warn
- Manual impl lints allowed: `manual_abs_diff`, `manual_map`, `manual_saturating_arithmetic`,
  `manual_unwrap_or`, `implicit_saturating_sub`
- Cast lints: deny
- `unsafe_code`: forbid
- `integer_division`: allow in test modules

## Tracey Annotations
- `r[depends]` on `spec fn` — formal dependency on requirement text
- `r[impl]` on `exec fn` — the runtime implementation
- `r[verify]` on `proof fn` and `#[test]` — evidence of correctness
- Config scans `src/**/*.rs` only (no separate verus/ directory)

## Enforcement Scripts
- `check-coverage.sh` — scans for `pub fn` without `r[impl]`, exits 1 on violation
- `check-ensures.sh` — scans for `pub fn` inside `verus!` without `ensures`, exits 1
- Both support exemption markers (`@verified-logic:exempt`, `@verified-logic:no-ensures`)
- Lookback window: 10 lines for r[impl], 5 lines for exemptions
- Stops scanning at `#[cfg(test)]` (test fns don't need r[impl])
- Skips `spec fn`, `proof fn`, private `fn`

## Cross-Module Pattern
- Spec fns from other modules are ghost code — erased in normal builds
- Cross-module proofs must go in a `#[cfg(verus_keep_ghost)] mod verus_proofs { }`
- Inside that module: `use crate::other_module::*_spec` works because Verus sets the cfg
- Tests go outside the cfg-gated module (they use exec fns which exist in all builds)
- Verus verifies the whole crate including cfg(verus_keep_ghost) modules

## Mistakes & Corrections
- Tried `cargo-verus` — needs rustup, doesn't work in nix. Used `rust_verify` directly.
- `proof fn` can't call `exec fn` in ensures — must reference `spec fn` instead.
- `FnSpec` deprecated — use `spec_fn`.
- Ghost closures can't have requires/ensures — restructured option module.
- Removed `verus/` directory — all specs embedded in `src/`.
