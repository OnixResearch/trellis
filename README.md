# verified-logic

Reusable logic building blocks with machine-checked correctness.

Every public function has formal pre/postconditions verified by
[Verus](https://verus-lang.github.io/verus/guide/). Requirement coverage
is tracked by [Tracey](https://tracey.bearcove.eu/).

## Modules

| Module | Functions | Proofs |
|---|---|---|
| `arithmetic` | `abs_diff`, `saturating_add`, `saturating_sub`, `checked_mul` | 8 |
| `bounds` | `in_range`, `clamp` | 5 |
| `collections` | `sorted_insert`, `binary_search` | 5 |
| `option` | `unwrap_or_default`, `map` | 4 |

## Usage

```toml
[dependencies]
verified-logic = { git = "https://github.com/yourname/verified-logic" }
```

```rust
use verified_logic::arithmetic::abs_diff;
use verified_logic::bounds::clamp;

let d = abs_diff(10, 3); // 7
let c = clamp(15, 0, 10); // 10
```

## Development

```bash
nix develop          # enter devshell
cargo check          # verify compilation
cargo nextest run    # run tests
nix flake check      # full CI (build + test + clippy + fmt)
```

## Verification

```bash
# Run everything — verus proofs + tracey coverage
./scripts/verify.sh

# Verus proofs only (22 proofs across 4 files)
./scripts/verus.sh verus/lib.rs

# Tracey coverage
tracey query status     # overview
tracey query uncovered  # missing implementations
tracey query untested   # missing tests/proofs
```

## Structure

```
docs/requirements.md          ← 10 requirements (source of truth)
src/arithmetic.rs              ← r[impl]  runtime code + tests r[verify]
src/bounds.rs                  ← r[impl]  runtime code + tests r[verify]
src/collections.rs             ← r[impl]  runtime code + tests r[verify]
src/option.rs                  ← r[impl]  runtime code + tests r[verify]
verus/arithmetic_spec.rs       ← r[depends] specs + r[verify] proofs
verus/bounds_spec.rs           ← r[depends] specs + r[verify] proofs
verus/collections_spec.rs      ← r[depends] specs + r[verify] proofs
verus/option_spec.rs           ← r[depends] specs + r[verify] proofs
.config/tracey/config.styx     ← tracey config
scripts/verify.sh              ← combined verus + tracey check
scripts/verus.sh               ← verus wrapper with nix store paths
```
