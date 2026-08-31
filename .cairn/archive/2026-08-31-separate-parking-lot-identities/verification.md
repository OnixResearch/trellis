# Verification

Date: 2026-08-31

## Scope

The change separates queue keys from thread identities in the finite parking-lot
core and raw-mutex models. It also makes the public model names concise and
moves the two compatibility modules to standard nested Rust module paths.

## Evidence

- Baseline focused tests passed before implementation:
  `nix develop -c cargo test --quiet parking_lot_core_model` and
  `nix develop -c cargo test --quiet parking_lot_raw_mutex`.
- Positive and negative identity tests passed:
  `nix develop -c cargo test --quiet --test parking` reported `5 passed`.
- Raw-mutex positive and negative tests passed:
  `nix develop -c cargo test --quiet parking_lot_raw_mutex` reported `4 passed`.
- Compile-fail tests passed:
  `nix develop -c cargo test --quiet --doc parking_lot_core_model` reported
  `2 passed`.
- The full test suite passed:
  `nix develop -c cargo test --quiet` reported `4863 passed` in the primary
  test binary. All other test binaries and doc tests also passed.
- Clippy passed:
  `nix develop -c cargo clippy --all-targets -- -D warnings`.
- Verus passed for the full library:
  `nix develop -c ./scripts/verus.sh src/lib.rs` reported
  `10752 verified, 0 errors`.
- Tracey passed after a foreground daemon start:
  `nix develop -c tracey query uncovered` reported `0 uncovered out of 2062`.
  `nix develop -c tracey query untested` reported `0 untested out of 2062`.
- The recursive coverage checker passed its positive and negative self-test:
  `nix develop -c ./scripts/check-coverage.rs --self-test`.
- The recursive coverage scan passed:
  `nix develop -c ./scripts/check-coverage.rs` checked `2574` public functions
  with no missing `r[impl]` annotations.
- The ensures scan passed:
  `nix develop -c ./scripts/check-ensures.rs` checked `2324` verified public
  functions with no missing ensures clauses.
- The pinned Octet check passed without changing `octet-baseline.json`:
  `nix build .#checks.$(nix eval --raw --impure --expr 'builtins.currentSystem').octet --no-link -L --builders ''`.
  It reported `0 new` findings and `20 stale` accepted identities.
- The README static metrics check passed:
  `nix develop -c ./scripts/check-readme-metrics.rs`.
- The exact test-list command reported `4863 tests, 0 benchmarks`.

## Boundaries

This change proves Rust type separation and preserves the finite model's tested
results. It does not add a Verus proof for these ordinary Rust modules. It does
not prove scheduling, blocking, fairness, wakeup, memory ordering, or operating
system synchronization.

The `check-readme-metrics.rs --full` wrapper encountered an independent Nix
CA-derivation assertion. The same proof, test, requirement, module, and trust
counts were checked with the component commands above. No change-specific
validation blocker remains.
