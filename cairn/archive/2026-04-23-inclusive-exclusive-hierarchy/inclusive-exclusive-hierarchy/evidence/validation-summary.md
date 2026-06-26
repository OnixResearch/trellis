# Validation Summary

Fresh validation captured on 2026-04-23 for the landed `cache_hierarchy` tree:

- `cargo test --quiet` → `4070 passed` plus `13` doc tests (`4083` total)
- `cargo clippy --all-targets -- -D warnings` → passed
- `./scripts/verus.sh src/lib.rs` → `8217 verified, 0 errors`
- `./scripts/check-ensures.sh` → `Checked 1595 pub fn in verus! blocks, 1 exempted`
- `tracey query uncovered` → `0 uncovered out of 1670 rules`
- `tracey query untested` → `0 untested out of 1670 rules`

Focused change-local checks:

- `cargo test --quiet cache_hierarchy` → `12 passed`
- `./scripts/verus.sh src/cache_hierarchy.rs` → `38 verified, 0 errors`

Evidence files:

- `cargo-test.txt`
- `clippy.txt`
- `verus-lib.txt`
- `check-ensures.txt`
- `tracey-uncovered.txt`
- `tracey-untested.txt`
- `cargo-test-cache-hierarchy.txt`
- `verus-cache-hierarchy.txt`
