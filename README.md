# verified-logic

Reusable logic building blocks with machine-checked correctness.

Every public function has a `spec fn` (mathematical definition) and an
`exec fn` whose `ensures result == spec(args)` clause is verified by
[Verus](https://verus-lang.github.io/verus/guide/). The specs are embedded
directly in `src/` — drift between spec and implementation is structurally
impossible. Requirement coverage is tracked by [Tracey](https://tracey.bearcove.eu/).

**1047 Verus proofs · 708 tests · ~260 requirements · 58 modules · 11 trusted axioms**

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

### Core arithmetic and data

| Module | What it verifies |
|---|---|
| `arithmetic` | abs diff, saturating add/sub, checked mul, gcd, div ceil |
| `bitwise` | power-of-two, bit get/set/clear/toggle |
| `bitmap` | fixed-size bit set over u64 words |
| `bounds` | range predicates, clamping |
| `collections` | sorted insert, binary search, merge sorted, partitioning, find, count |
| `conversion` | checked narrowing (u32/u16/u8), signed↔unsigned |
| `cost_tracker` | cost accumulation, threshold crossing, budget status, model selection priority |
| `integer` | parity, alignment, log2, next-power-of-two |
| `interval` | half-open interval contains, overlap, split, adjacent, merge |
| `option` | verified Option combinators (map, and_then, or_else, flatten, zip, filter) |
| `ordering` | min, max, median, sign, compare-and-swap |
| `result` | verified Result combinators (map_ok, map_err, and_then, or_else) |
| `ring_buffer` | FIFO queue with power-of-two capacity |
| `slice` | chunk count, prefix/suffix equality, all-equal, mismatch |

### Consensus and replication

| Module | What it verifies |
|---|---|
| `acid` | atomicity, consistency, isolation, durability over state vectors |
| `cas` | compare-and-swap condition evaluation |
| `chain` | chain hash integrity with append-preserves proof |
| `epoch` | lexicographic (term, index) comparison for Raft elections |
| `log` | append-only log with Log Matching Property |
| `monotonic` | advancement predicates for terms, commit indices |
| `quorum` | majority threshold, intersection theorem, fault tolerance |

### Systems decision logic

| Module | What it verifies |
|---|---|
| `approval` | merge gating: approval counting, stale detection, CI checks |
| `batch` | write batch limit checking and flush decisions |
| `bounded_range` | ranges with optional upper bound (None = unbounded) |
| `chunk` | ceiling-division chunk arithmetic, offset↔chunk mapping |
| `cow` | copy-on-write ref counting, snapshot isolation |
| `drain` | graceful shutdown admission protocol |
| `eviction` | LRU cache eviction with protected entries |
| `expiration` | TTL calculation, lease/key expiration |
| `hash_bucket` | hash-to-bucket assignment, modular and power-of-two |
| `partition` | non-overlapping gap-free interval partitions |
| `pressure` | multi-threshold severity classification |
| `rate_limit` | integer token bucket rate limiting |
| `sequential` | sequential access detection, prefetch triggering |

### Cross-module

| Module | What it verifies |
|---|---|
| `properties` | 50 proofs linking modules: triangle inequality, arithmetic inverses, clamp-is-min-max, chunk-ceil-is-div-ceil, partition-no-overlap, hash-bucket-in-interval, pressure-classify-monotonic, and more |

## Usage

```toml
[dependencies]
verified-logic = { git = "https://github.com/yourname/verified-logic" }
```

```rust
use verified_logic::arithmetic::abs_diff;
use verified_logic::bounds::clamp;
use verified_logic::quorum::majority;
use verified_logic::drain::can_start_op;

let d = abs_diff(10, 3);       // 7
let c = clamp(15, 0, 10);      // 10
let m = majority(5);           // 3
let ok = can_start_op(false);  // true (not draining)
```

For normal `cargo build`, the `vstd` crate erases all ghost code. No
runtime cost from specs, proofs, requires, or ensures.

## Development

```bash
nix develop                # enter devshell
cargo check                # compile (ghost code erased)
cargo test                 # 467 tests (unit + property)
cargo clippy -- -D warnings
```

## Verification

```bash
./scripts/verify.sh        # all gates: clippy → tests → verus → coverage
./scripts/verus.sh         # verus only (633 proofs)
./scripts/check-coverage.sh   # every pub fn has r[impl] annotation
./scripts/check-ensures.sh    # every pub fn in verus! has ensures
```

## Structure

```
src/lib.rs              ← crate root, 34 module declarations
src/<module>.rs         ← spec fn + exec fn + proof fn + #[cfg(test)] tests
docs/requirements.md    ← 193 tracey requirements
scripts/verus.sh        ← verus wrapper with nix store paths
scripts/verify.sh       ← full verification pipeline
scripts/check-*.sh      ← annotation enforcement
```

Each module follows the same pattern:

1. `pub open spec fn *_spec(...)` — mathematical definition with `r[depends]` annotation
2. `pub fn *(...)` with `ensures result == *_spec(args)` — runtime code with `r[impl]` annotation
3. `proof fn` — properties of the spec with `r[verify]` annotation
4. `#[cfg(test)]` — unit tests and proptest property tests

## Trusted items

7 functions are marked `#[verifier::external_body]` (Verus trusts the
ensures clause without checking the body):

- 2 in `chain` — blake3 hash modeled as an uninterpreted function with a determinism axiom
- 1 in `hash_bucket` — `x & (n-1) == x % n` for power-of-two n exceeds Z3's 64-bit SAT rlimit
- 3 in `rate_limit` — exec overflow detection (u64 division guards) can't be bridged to int-based specs
- 1 in `properties` — int→u64→int cast chain through two spec functions

All trusted items have proptest oracles verifying the same property at runtime.
