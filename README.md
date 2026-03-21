# verified-logic

Reusable logic building blocks with machine-checked correctness.

Every public function has a `spec fn` (mathematical definition) and an
`exec fn` whose `ensures result == spec(args)` clause is verified by
[Verus](https://verus-lang.github.io/verus/guide/). The specs are embedded
directly in `src/` — drift between spec and implementation is structurally
impossible. Requirement coverage is tracked by [Tracey](https://tracey.bearcove.eu/).

## How it works

```rust
verus! {

// Mathematical definition (ghost code, erased at compile time)
pub open spec fn abs_diff_spec(a: u64, b: u64) -> u64 {
    if a >= b { (a - b) as u64 } else { (b - a) as u64 }
}

// Runtime implementation — Verus checks `ensures` against the body
pub fn abs_diff(a: u64, b: u64) -> (result: u64)
    ensures result == abs_diff_spec(a, b)    // ← mechanically checked
{
    if a >= b { a - b } else { b - a }
}

// Machine-checked proof of a property
proof fn abs_diff_symmetric(a: u64, b: u64)
    ensures abs_diff_spec(a, b) == abs_diff_spec(b, a)
{}

} // verus!
```

## Modules

| Module | Functions | Verus proofs |
|---|---|---|
| `arithmetic` | `abs_diff`, `saturating_add`, `saturating_sub`, `checked_mul`, `gcd`, `div_ceil` | 10 |
| `bitwise` | `is_power_of_two`, `bit_at`, `set_bit`, `clear_bit`, `toggle_bit` | 8 |
| `bounds` | `in_range`, `clamp` | 4 |
| `integer` | `is_even`, `is_odd`, `align_down`, `align_up`, `log2_floor`, `next_power_of_two` | 20 |
| `collections` | `sorted_insert`, `binary_search`, `merge_sorted` | spec-level |
| `option` | `unwrap_or_default`, `map`, `and_then`, `or_else`, `flatten`, `zip`, `filter` | 21 |
| `ordering` | `min`, `max`, `median_of_three`, `sign`, `compare_and_swap` | 15 |
| `result` | `unwrap_or`, `map_ok`, `map_err`, `and_then`, `or_else` | 14 |

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
cargo check          # compile (ghost code erased)
cargo test           # 25 tests (unit + property)
nix flake check      # full CI (build + test + clippy + fmt)
```

## Verification

```bash
./scripts/verify.sh  # all gates: clippy → tests → verus → tracey

./scripts/verus.sh   # verus only (verifies src/lib.rs)
tracey query status  # requirement coverage
```

## Structure

```
src/lib.rs             ← crate root
src/arithmetic.rs      ← spec fn + exec fn + proof fn + tests
src/bounds.rs          ← spec fn + exec fn + proof fn + tests
src/collections.rs     ← spec fn + exec fn + tests (stdlib methods)
src/option.rs          ← spec fn + exec fn + proof fn + tests
docs/requirements.md   ← 10 tracey requirements
scripts/verify.sh      ← full verification pipeline
scripts/verus.sh       ← verus wrapper with nix store paths
```
