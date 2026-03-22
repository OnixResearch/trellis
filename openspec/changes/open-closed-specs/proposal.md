## Why

11 `external_body` items remain only because their `ensures` clauses
reference `closed spec fn` definitions. Z3 can't see inside a closed spec,
so it can't verify the exec body matches. Opening these specs with actual
definitions unlocks full verification.

## What Changes

Convert `closed spec fn` to `open spec fn` with recursive definitions,
then remove `#[verifier::external_body]` from the corresponding exec fns
and proof fns.

## Scope

### In Scope (simpler specs)
- `parse_drv_name_spec` — linear scan for hyphen-digit
- `remove_attrs_spec` — recursive filter by key exclusion
- `intersect_attrs_spec` — recursive filter by key inclusion
- `ceil_log_spec` — recursive ceiling log with division (termination via nonlinear_arith)
- `convergence_rounds` exec fn — depends on ceil_log_spec

### Stretch (more complex)
- `merge_spec` — two-pointer merge (needs lexicographic comparison spec)
- `compare_versions_spec` — multi-phase byte-level comparison
- `replace_strings_spec` — multi-pattern left-to-right matching
- `list_to_attrs_spec` — sort + dedup

### Out of Scope
- `chain::hash_spec` — genuinely uninterpreted (models blake3)
- Rate limit specs — already open, Z3 can't handle nonlinear chains

## Impact
- Converts trusted axioms into machine-checked proofs
- No API changes — same function signatures, same test behavior
