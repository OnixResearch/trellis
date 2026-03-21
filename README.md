# verified-logic

Reusable logic building blocks with machine-checked correctness.

Every public function has formal pre/postconditions verified by
[Verus](https://verus-lang.github.io/verus/guide/). Requirement coverage
is tracked by [Tracey](https://tracey.bearcove.eu/).

## Modules

| Module | Functions |
|---|---|
| `arithmetic` | `abs_diff`, `saturating_add`, `saturating_sub`, `checked_mul` |
| `bounds` | `in_range`, `clamp` |
| `collections` | `sorted_insert`, `binary_search` |
| `option` | `unwrap_or_default`, `map` |

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
# Formal proofs (requires verus)
verus --crate-type=lib verus/lib.rs

# Requirement coverage
tracey query status

# Combined check
./scripts/verify.sh
```

## Requirements

Requirements live in `docs/requirements.md` with Tracey IDs. Each maps to:

- `src/<module>.rs` — `r[impl <req>]` on the function
- `tests/` or inline `#[test]` — `r[verify <req>]` on tests
- `verus/<module>_spec.rs` — `r[depends <req>]` / `r[verify <req>]` on specs and proofs
