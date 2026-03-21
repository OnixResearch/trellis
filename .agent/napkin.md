# Napkin — verified-logic

## Project State
- Verus specs embedded in `src/` — no separate `verus/` directory
- Every exec fn has `ensures result == *_spec(args)` — Verus verifies the actual implementation
- Drift between spec and impl is structurally impossible
- 21 Verus proofs + 25 cargo tests (10 unit + 15 property) = all gates green
- `vstd` is a cargo dependency — `verus!` blocks compile normally (ghost code erased)
- `rust_verify` runs on `src/lib.rs` — verifies the whole crate

## Recent Changes
- Slice module added: chunk_count, prefix_eq, suffix_eq, all_equal, mismatch_index
  - chunk_count proofs need nonlinear_arith for multiplication commutativity:
    fundamental div-mod gives `d * (n/d)` but ensures needs `(n/d) * d`
  - suffix_eq quantifier needs explicit #![trigger] — Verus can't auto-infer for
    complex index expressions like `a[a.len() - n + i]`
  - suffix_eq loop invariant needs `a_start + n == a.len()` to prove no arithmetic overflow
    on `a_start + i` (instead of `a_start as int == a@.len() - n as int`)
  - mismatch_none_implies_prefix_eq uses inductive helper proof fn
  - Seq::len() returns nat, not int — must cast with `as int` for int return types
- Conversion module added: narrow_u32, narrow_u16, narrow_u8, signed_to_unsigned, unsigned_to_signed
  - Cast lints (truncation, sign_loss, wrap) allowed at module level — verified casts are the point
  - clippy::checked_conversions also allowed — can't use try_from inside verus! blocks
  - Roundtrip proofs (i64→u64→i64, u64→i64→u64) verify directly — Z3 handles cast chains
  - Narrowing chain proof (u8 fits → u16 fits → u32 fits) trivially automatic
  - Option<u32>, Option<u16>, Option<u8>, Option<i64> all work in Verus ensures
- Cross-module properties added: all_equal⟹count, chunk_count↔div_ceil, narrow↔in_range, all_equal⟹partitioned
  - chunk_count_div_ceil needs by(nonlinear_arith) for quotient uniqueness:
    Z3 can't derive (n-1)/k == n/k from fundamental_div_mod alone, needs
    case split on n%k > 0 vs == 0 with nonlinear_arith per branch
  - all_equal_implies_count needs inductive helper (count_from == len - from)
  - all_equal_implies_partitioned and narrow_iff_in_range are automatic (Z3 instant)
- Ring buffer added: verified FIFO queue with power-of-two capacity
  - Struct fields must be `pub` for `pub open spec fn` — Verus treats struct as opaque
    outside the module when any field is private
  - `is_power_of_two_spec` import needs #[cfg(verus_keep_ghost)] — spec fns are erased
    in normal builds, so top-level `use` fails without the gate
  - Pop overflow fix: capture `head` and `cap` as locals before mutation, so Verus
    retains `head < cap` for the `(head + 1)` arithmetic
  - `&mut self` methods work with `old(self)` for pre-state references in ensures
  - Vec::set(i, v) is the Verus way to mutate vec elements in-place
  - Constructor loop builds Vec one element at a time (no vec![0; n] in Verus)
- Total: 232 Verus proofs, 172 cargo tests, 80 requirements

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
