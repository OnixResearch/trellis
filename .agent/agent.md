# Agent Instructions — verified-logic

This crate is a verified logic library. Every public function is
machine-checked by Verus with requirement traceability via Tracey.
Follow these rules when writing or modifying code.

## The Pattern

Every function follows this three-part structure inside a `verus!` block:

```rust
use vstd::prelude::*;

verus! {

// 1. Spec: mathematical definition (ghost, erased at compile time)
// r[depends requirement.id]
pub open spec fn foo_spec(x: u64) -> u64 {
    // pure mathematical logic — the source of truth
}

// 2. Exec: runtime implementation, tied to spec via ensures
// r[impl requirement.id]
pub fn foo(x: u64) -> (result: u64)
    ensures result == foo_spec(x)
{
    // actual implementation — Verus checks this matches the spec
}

// 3. Proof: proves properties about the spec
// r[verify requirement.id]
proof fn foo_some_property(x: u64)
    ensures foo_spec(x) > 0
{}

} // verus!
```

The `ensures result == foo_spec(x)` is what eliminates drift. Verus
mechanically verifies the function body satisfies this postcondition.
If someone changes the body, Verus rejects it unless the ensures still
holds.

## Rules

### Never separate specs from implementations
All `spec fn`, `exec fn`, and `proof fn` for a function live in the same
file in `src/`. No separate `verus/` directory. The spec is right next to
the code it describes.

### Every public exec fn must have `ensures result == *_spec(args)`
This is the mechanical link. Without it, the spec and implementation can
drift independently. The ensures clause is the contract.

### Proof functions reference spec fns, not exec fns
Verus does not allow calling exec functions from proof-mode `ensures`.
Always write `ensures foo_spec(x) == bar_spec(y)`, never
`ensures foo(x) == bar(y)`.

### Tracey annotations are mandatory
Every function gets a Tracey annotation:

| Verus construct | Tracey verb | Example |
|---|---|---|
| `spec fn` | `depends` | `// r[depends arithmetic.abs-diff]` |
| `exec fn` | `impl` | `// r[impl arithmetic.abs-diff]` |
| `proof fn` | `verify` | `// r[verify arithmetic.abs-diff]` |
| `#[test] fn` | `verify` | `// r[verify arithmetic.abs-diff]` |

The annotation comment goes on the line immediately above the function.

### Requirements come first
Before writing code, add the requirement to `docs/requirements.md`:

```markdown
r[domain.requirement-name]
Description using RFC 2119 keywords (MUST, SHALL, SHOULD, MAY).
```

The requirement ID must contain at least one dot. Then implement the
three-part pattern (spec + exec + proof) with matching Tracey annotations.

### Tests pair with proofs
Every requirement gets both:
- `proof fn` inside `verus!` — machine-checked mathematical evidence
- `#[test]` with proptest — runtime fuzz testing

Property tests verify observable behavior (symmetry, commutativity,
idempotence). They catch bugs in the gap between what Verus can see
(your code) and what it can't (stdlib internals like `checked_add`).

### Use `spec_fn` not `FnSpec` for function types
`FnSpec` is deprecated. Use `spec_fn(T) -> U` in Verus code.

### Keep exec function bodies simple
Verus's SMT solver works best on straight-line code with simple branching.
If the solver times out:
- Break complex logic into smaller functions with their own ensures
- Add `proof { }` blocks with lemma hints
- Use `assert(...)` as intermediate proof steps

## Verification pipeline

Run `./scripts/verify.sh` before committing. It runs five gates in order:

1. **Clippy** — `cargo clippy --all-targets -- -D warnings`
2. **Tests** — `cargo test` (unit + proptest)
3. **Verus** — `./scripts/verus.sh src/lib.rs` (formal verification)
4. **Tracey uncovered** — requirements with no `r[impl]`
5. **Tracey untested** — requirements with `r[impl]` but no `r[verify]`

All five must pass. The script exits non-zero on any failure.

### Running verus alone

```bash
./scripts/verus.sh              # verify whole crate (src/lib.rs)
./scripts/verus.sh src/foo.rs   # verify single file
```

The wrapper sets `LD_LIBRARY_PATH`, `VERUS_Z3_PATH`, and `--extern` flags
for the nix store verus installation. Do not call `rust_verify` directly.

## Adding a new function

1. Add requirement to `docs/requirements.md`
2. In the appropriate `src/<module>.rs`, inside the `verus!` block:
   - Write `spec fn` with `r[depends req.id]`
   - Write `exec fn` with `ensures result == spec(args)` and `r[impl req.id]`
   - Write `proof fn`(s) with `r[verify req.id]`
3. Below the `verus!` block, in `#[cfg(test)] mod tests`:
   - Write `#[test]` with `r[verify req.id]`
   - Write `proptest!` property test with `r[verify req.id]`
4. Run `./scripts/verify.sh` — all gates must pass
5. Check `tracey query status` — requirement must show covered + verified

## Cross-module proofs

When proving properties that reference `spec fn`s from other modules,
the specs are ghost code — erased in normal cargo builds. Use the
`cfg(verus_keep_ghost)` pattern:

```rust
// src/properties.rs

#[cfg(verus_keep_ghost)]
mod verus_proofs {
    use vstd::prelude::*;
    use crate::arithmetic::{abs_diff_spec, saturating_add_spec};
    use crate::bounds::{clamp_spec, in_range_spec};

    verus! {

    // r[impl properties.my-property]
    pub open spec fn my_property_prop(a: u64) -> bool { ... }

    // r[verify properties.my-property]
    proof fn prove_my_property(a: u64)
        ensures my_property_prop(a)
    {}

    } // verus!
}

#[cfg(test)]
mod tests {
    // Tests go here — they use exec fns (which exist in all builds)
}
```

The `cfg(verus_keep_ghost)` gate means:
- Normal `cargo build/test`: module is skipped, no ghost import errors
- `rust_verify` (Verus): module is compiled and verified, ghost imports resolve

Tests must be outside the cfg-gated module since they run in normal builds.

## Adding a new module

1. Create `src/<module>.rs`
2. Add `pub mod <module>;` to `src/lib.rs`
3. Start the file with `use vstd::prelude::*;` and a `verus!` block
4. Follow the pattern above for each function
5. Tracey config (``.config/tracey/config.styx`) already covers `src/**/*.rs`

## Clippy

Lint policy is in `[lints.clippy]` in `Cargo.toml`. Key points:
- `correctness` and `suspicious` groups: **deny**
- `pedantic` group: **warn**
- Manual-implementation lints (`manual_abs_diff`, `manual_map`,
  `manual_saturating_arithmetic`, `manual_unwrap_or`,
  `implicit_saturating_sub`): **allow** — hand-written logic is the point
- Cast lints (`cast_possible_truncation`, `cast_sign_loss`): **deny**
- `unsafe_code`: **forbid**

## Verus edition

The `verus!` macro requires `--edition 2021`. The `scripts/verus.sh`
wrapper passes this flag. The crate itself uses `edition = "2024"` for
normal cargo builds — the vstd crate handles the edition difference
internally.

## File layout

```
src/lib.rs              ← crate root, module declarations
src/arithmetic.rs       ← verus! { spec + exec + proof } + tests
src/bounds.rs           ← verus! { spec + exec + proof } + tests
src/collections.rs      ← verus! { spec + exec + proof } + tests
src/option.rs           ← verus! { spec + exec + proof } + tests
docs/requirements.md    ← Tracey requirement IDs (source of truth)
.config/tracey/         ← Tracey configuration
scripts/verify.sh       ← full pipeline (clippy + test + verus + tracey)
scripts/verus.sh        ← verus wrapper with nix store paths
```
