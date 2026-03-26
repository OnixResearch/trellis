## Why

16 `#[verifier::external_body]` items and 3 `closed spec fn` definitions
remain across 7 modules. Each is a point where Verus trusts an assertion
without proof. Closing these gaps converts trusted axioms into
machine-checked proofs, raising the assurance level of the library.

Some gaps are closeable with known techniques (decomposition, fuel-based
recursion, nonlinear arithmetic helpers). Others are permanent —
they model genuinely uninterpreted functions (cryptographic hashes) or
hit fundamental Z3 theory boundaries.

## What Changes

Classify every remaining gap as **closeable** or **permanent**, then
close what can be closed.

## Inventory

### Cryptographic axioms (permanent — 6 items, 2 closed specs)

These model hash functions as uninterpreted. The axioms
(determinism, collision-resistance) are the standard cryptographic
assumptions. No proof is possible without defining the hash, and
defining it would make proofs intractable.

| Module               | Item                         | Kind       |
|----------------------|------------------------------|------------|
| `chain`              | `hash_spec`                  | closed spec|
| `chain`              | `hash_deterministic`         | proof axiom|
| `chain`              | `hash_collision_free`        | proof axiom|
| `chain`              | `compute_entry_hash`         | exec fn    |
| `snapshot_integrity`  | `dual_hash_spec`             | closed spec|
| `snapshot_integrity`  | `dual_hash_deterministic`    | proof axiom|
| `snapshot_integrity`  | `dual_hash_collision_resistant` | proof axiom|
| `snapshot_integrity`  | `dual_hash`                  | exec fn    |

**Action:** Document as permanent axioms. No code changes.

### Nonlinear arithmetic (closeable? — 3 items)

The rate_limit exec fns split multiplication to avoid u64 overflow.
Proving the split identity `a*c/b == (a/b)*c + (a%b)*c/b` requires
chaining `lemma_fundamental_div_mod` with nonlinear_arith. Prior
attempts failed because Z3 can't bridge the int-spec to the
overflow-guarded exec. Worth one more attempt with vstd's
`lemma_mul_is_distributive` and step-by-step assertions.

| Module       | Item                     | Kind     |
|--------------|--------------------------|----------|
| `rate_limit` | `tokens_after_refill`    | exec fn  |
| `rate_limit` | `tokens_to_wait_ms`      | exec fn  |
| `rate_limit` | `wait_enables_consumption` | proof fn |

**Action:** Attempt closure using vstd division/multiplication lemmas.

### Z3 unfolding limits (permanent — 4 items)

The `cv_fuel` spec for version comparison has 8+ levels of nested
let/if/else. Z3 can't unfold deeply enough for inductive proofs,
even at rlimit(2000). The proof structure is correct — Z3 runs out
of quantifier instantiation budget.

| Module        | Item                       | Kind     |
|---------------|----------------------------|----------|
| `nix_version` | `cv_fuel_antisymmetric`    | proof fn |
| `nix_version` | `cv_fuel_reflexive`        | proof fn |
| `nix_version` | `cv_fuel_range`            | proof fn |
| `nix_version` | `compare_versions`         | exec fn  |

**Action:** Document as permanent. The spec is open and structurally
correct; Z3 just can't process the recursion depth.

### Bit-vector theory bridge (permanent — 1 item)

The buddy allocator's `order_for_size` shift-loop proves
`1u64 << order >= size` via bit_vector, but the spec uses int-level
`<<` via `block_size_spec`. Z3 can't bridge between bitvector and
integer shift theories.

| Module  | Item             | Kind    |
|---------|------------------|---------|
| `buddy` | `order_for_size` | exec fn |

**Action:** Document as permanent theory bridge limitation.

### Degenerate spec (closeable — 1 item)

The glob spec returns `true` for any starred pattern. Writing a full
recursive glob spec would allow closing the exec, but the spec itself
needs work first.

| Module | Item         | Kind    |
|--------|--------------|---------|
| `glob`  | `glob_match` | exec fn |

**Action:** Write a real recursive glob_match_spec, then close the exec.

### Closed-spec dependent (closeable — 1 item)

`replace_strings` depends on the closed `replace_strings_spec`. Opening
the spec with a recursive definition would allow closing the exec.

| Module       | Item              | Kind    |
|--------------|-------------------|---------|
| `nix_string` | `replace_strings` | exec fn |

**Action:** Open `replace_strings_spec` with left-to-right scan
recursive definition, then close the exec fn.

## Summary

| Category            | Count | Action    |
|---------------------|-------|-----------|
| Crypto axioms       | 8     | Permanent |
| Nonlinear arith     | 3     | Attempt   |
| Z3 unfolding        | 4     | Permanent |
| Bit-vector bridge   | 1     | Permanent |
| Degenerate spec     | 1     | Closeable |
| Closed-spec dep     | 1     | Closeable |
| Closed spec (nix)   | 1     | Closeable (with phase 3) |
| **Total**           | **19**| 5 closeable, 14 permanent |

## Impact

- Reduces external_body from 16 to ~11 (best case)
- Opens 1 of 3 closed specs (replace_strings_spec)
- Replaces 1 degenerate spec (glob_match_spec) with real definition
- No API changes
