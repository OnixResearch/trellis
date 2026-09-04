# Bounded Modular Tree Fold

Trellis provides a verified `u24` wrapping-add domain and a bounded balanced reduction profile.

The local profile is `trellis.balanced-u24-wrapping-sum.v1`. The executable oracle symbol is `verified_logic::bounded_modular_tree_fold::sequential_u24_sum`.

## External references

The profile records these external references:

- Bend 0.2.38 at revision `814453670d0e0d6777c1313c972764dba0491b7f`, licensed Apache-2.0.
- HVM2 2.0.22 at revision `654276018084b8f44a22b562dd68ab18583bfb5b`, licensed Apache-2.0.

These projects define the downstream conformance context. Trellis does not import, build, execute, or prove their source.

## Admitted input

A caller supplies an ordered leaf vector and an explicit nonzero leaf limit. Admission requires all these facts:

- The vector is not empty.
- The vector length is a power of two.
- The vector length does not exceed the limit.
- Every leaf is at most `MAX`.
- The leaf count fits the evidence accounting type.

Admission returns a stable typed rejection for each failed fact. Rejection does not modify the caller input.

## Arithmetic and reduction

`normalize` maps any `u64` into the declared domain. `add` performs wrapping addition without runtime overflow.

`sequential_u24_sum` is the optimized iterative oracle. `balanced_u24_sum` repeatedly combines adjacent pairs without recursive Rust stack growth.

Both paths use the same ordered modular-sum specification. Verus proves one pair round preserves that sum.

For `n` admitted leaves, the abstract balanced schedule has `n - 1` combines. Its round count is the number of exact halvings needed to reach one leaf.

## Proof handoff

The proof-evidence bundle binds these source files:

- `src/u24/mod.rs`
- `src/bounded_modular_tree_fold/mod.rs`
- `src/bounded_modular_tree_fold/core.rs`
- `src/bounded_modular_tree_fold/tests.rs`

The row also binds requirements, proof functions, verifier identity, proof gaps, BLAKE3 identities, profile identity, and the oracle symbol.

A consumer must pin the complete artifact. It must also record its own validation or rerun receipt before assigning a verified role.

Use these checks:

```text
nix develop -c cargo test bounded_modular_tree_fold --lib
nix develop -c ./scripts/verus.sh src/lib.rs
nix develop -c cargo run --quiet --bin proof_evidence -- check
```

## Claim boundary

The proof covers Trellis-owned arithmetic, admission, executable refinements, pair-round equivalence, and abstract work counts.

It does not prove Bend, HVM2, generated C, a host compiler, source effect absence, arbitrary trees, performance, resources, HVM interactions, releases, or input decoding.

Matching external output is differential conformance evidence for the tested run. It is not proof of external semantics.
