# Verus Specifications

Formal specs and proofs for `verified-logic`, checked by
[Verus](https://verus-lang.github.io/verus/guide/).

Each `*_spec.rs` file contains `spec fn` (formal definitions) and `proof fn`
(machine-checked proofs) for the corresponding `src/` module.

## Running verification

```bash
verus --crate-type=lib verus/lib.rs
```

Or use the combined check:

```bash
./scripts/verify.sh
```
