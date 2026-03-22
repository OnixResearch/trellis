## Why

The crate has 34 `#[verifier::external_body]` items — functions and proofs
where Verus trusts the ensures clause without machine-checking the body.
Each one is a gap in the verification story. Closing them strengthens the
"verified" claim from "842 proofs + 34 trusted" to "870+ proofs, fewer
trusted axioms."

## What Changes

Replace `external_body` with verified implementations where Z3 can handle
the proof obligations. Items that are genuinely axiomatic (hash function
determinism, 64-bit SAT overflow) stay trusted and get documented as
intentional axioms.

## Scope

### In Scope
- Loop-invariant-based exec functions (bloom, consistent_hash, crdt, gossip, merkle, nix_list, backoff)
- Inductive proof functions (bloom insert_monotonic/no_false_negatives, consistent_hash coverage)
- Simple property proofs (nix_version reflexive/range, single_node_all_keys)
- Nix attrset exec functions (sorted-sequence iteration patterns)

### Out of Scope
- `chain::hash_deterministic` / `compute_entry_hash` — genuine axiom (uninterpreted hash)
- `hash_bucket::pow2_mask_is_mod` — 64-bit bit_vector SAT exceeds Z3 rlimit
- `rate_limit` nonlinear division chains — known Z3 limitation
- `nix_version::compare_versions` proofs — recursive byte-level spec too deep for Z3
- `nix_string::replace_strings` — complex byte scanning with backtracking

## Impact

- **Files**: src/*.rs (13 files with external_body)
- **APIs**: No API changes — same specs, verified bodies
- **Testing**: Existing proptest oracles remain as defense-in-depth
