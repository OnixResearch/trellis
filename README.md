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
| `bitmap` | `new`, `get`, `set`, `clear`, `toggle`, `len`, `word_count` | 9 |
| `bounds` | `in_range`, `clamp` | 4 |
| `conversion` | `narrow_u32`, `narrow_u16`, `narrow_u8`, `signed_to_unsigned`, `unsigned_to_signed` | 18 |
| `integer` | `is_even`, `is_odd`, `align_down`, `align_up`, `log2_floor`, `next_power_of_two` | 20 |
| `log` | `agree_at`, `logs_match_prefix`, `log_append`, `log_truncate` | 13 |
| `collections` | `sorted_insert`, `binary_search`, `merge_sorted`, `is_partitioned`, `find_first`, `count_matches` | 10 |
| `option` | `unwrap_or_default`, `map`, `and_then`, `or_else`, `flatten`, `zip`, `filter` | 21 |
| `ordering` | `min`, `max`, `median_of_three`, `sign`, `compare_and_swap` | 15 |
| `epoch` | `epoch_cmp`, `epoch_ge` + total order proofs | 10 |
| `monotonic` | `is_advance`, `is_strict_advance` + ordering proofs | 8 |
| `quorum` | `majority`, `is_quorum`, `fault_tolerance`, `can_remove_safely`, `min_cluster_size` + intersection theorem | 17 |
| `result` | `unwrap_or`, `map_ok`, `map_err`, `and_then`, `or_else` | 14 |
| `ring_buffer` | `new`, `push`, `pop`, `peek`, `len`, `is_empty`, `is_full`, `capacity`, `remaining`, `full_batches` | 2 |
| `slice` | `chunk_count`, `prefix_eq`, `suffix_eq`, `all_equal`, `mismatch_index` | 11 |

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
