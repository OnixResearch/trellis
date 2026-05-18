# Proptest Hardening Plan

## Current State

This plan is source-derived from the current tree and should be refreshed before
using it for release/readiness claims. The release gate and proof-gap inventory
remain authoritative for final counts.

| Metric | Current source-derived count |
|--------|------------------------------|
| Public modules declared in `src/lib.rs` | 581 |
| Rust source files under `src/` | 583 |
| Source-defined unit tests (`fn test_*`) | 2,189 |
| Source-defined proptests (`fn prop_*`) | 1,954 |
| Source-defined test/proptest functions | 4,143 |
| Modules with at least one `fn prop_*` | 321 |
| Cross-module proptests in `src/properties.rs` | 243 |
| Proof-gap inventory gate | 16 `external_body`, 0 `assume(...)`, 0 `assume_specification[...]`, 0 `@verified-logic:no-ensures`, 0 `@verified-logic:exempt` |

The old P0 list for `topo_sort`, `reachability`, `nix_closure`, and
`settlement` is no longer a current gap list: those `assume(...)` and
`no-ensures` markers are closed. Use `scripts/check-proof-gap-inventory.rs` and
`docs/proof-gap-sites.tsv` for the trusted-boundary baseline before adding any
new safety-net work.

## Priority Tiers

### P0 — Keep proof-gap and test-planning evidence synchronized

These are small, high-leverage maintenance rails that prevent stale plans from
sending future work toward already-closed gaps.

1. Run the source-derived count probe when updating this file.
2. Run `scripts/check-proof-gap-inventory.rs` and ensure it reports the expected
   16 permanent trusted boundaries and zero weakened marker categories.
3. Keep README structure/count prose aligned with the same source-derived view,
   while leaving release-baseline counts in `CHANGELOG.md` as historical release
   evidence unless a release refresh is explicitly being cut.

### P1 — Recent primitive-family property coverage

Recent public modules are good candidates for focused, low-blast-radius
proptests because they add visible release value without changing semantics.

- `algebraic_effect`: arity/admission rejection cases, handler coverage
  monotonicity, residual-call preservation, continuation single-use examples.
- `ability_effect`: ability-subsumption and capability-set admission edge cases.
- `content_addressed_code`: dependency-closure and content-id stability examples.
- `typed_durable_reference`: reference admission, stale/fresh generation
  boundaries, and typed-identity round trips.
- `remote_computation`: admission and capability boundary negative cases.
- Parking-lot family (`parking_lot_core_model`, `parking_lot_raw_mutex`,
  `parking_lot_rwlock_model`, `parking_lot_condvar_model`,
  `parking_lot_contracts`): park/unpark uniqueness, timeout/no-spurious wakeup
  cases, raw mutex parked-bit transitions, reader/writer exclusion, and fairness
  contract examples.

### P2 — Cross-module property expansion

Add cross-module tests where a composition claim is more valuable than another
per-module example.

- Effect/computation composition: content-addressed definitions + ability/effect
  admission + remote-computation admission.
- Concurrency composition: futex state + parking-lot queues + raw mutex/RwLock
  transition models.
- Storage/KV composition: WAL recovery + journal + page checksum + block cache.
- Graph/algorithm composition: cycle detection + topo sort + reachability +
  shortest path.
- Financial composition: fee + ledger + escrow + settlement conservation.

### P3 — Boundary, corpus, and regression hardening

Use once the recent primitive-family coverage has direct tests.

- Boundary value proptests for every module with guarded `u64` arithmetic.
- Round-trip/inverse proptests for encode/decode, insert/remove, lock/unlock,
  and push/pop pairs.
- Promote useful shrunk cases from `proptest-regressions/` into named unit tests
  so regressions do not depend only on seed files.
- Add negative tests around permanent trusted boundaries without claiming to
  verify the underlying cryptographic/codec/runtime primitive.

## Refresh Command

```sh
python - <<'PY'
from pathlib import Path
src = Path('src')
modules = sum(1 for line in Path('src/lib.rs').read_text().splitlines() if line.startswith('pub mod '))
rs_files = list(src.glob('*.rs'))
test_fns = 0
prop_fns = 0
modules_with_props = set()
properties_rs_props = 0
for path in rs_files:
    lines = path.read_text(errors='ignore').splitlines()
    tests = sum(1 for line in lines if line.strip().startswith('fn test_'))
    props = sum(1 for line in lines if line.strip().startswith('fn prop_'))
    test_fns += tests
    prop_fns += props
    if props:
        modules_with_props.add(path.stem)
    if path.name == 'properties.rs':
        properties_rs_props = props
print(f'pub_mod_count={modules}')
print(f'src_rs_files={len(rs_files)}')
print(f'source_test_fn_count={test_fns}')
print(f'source_prop_fn_count={prop_fns}')
print(f'source_test_plus_prop_count={test_fns + prop_fns}')
print(f'modules_with_prop_fns={len(modules_with_props)}')
print(f'properties_rs_prop_fns={properties_rs_props}')
PY

CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic \
  nix develop -c bash -lc './scripts/check-proof-gap-inventory.rs'
```

## Conventions

- Proptest functions: `prop_<module>_<property>` in per-module `#[cfg(test)]`
  blocks.
- Cross-module proptests: `prop_<domain>_<property>` in `src/properties.rs`.
- Use the narrowest input ranges that cover the interesting space; avoid broad
  `0..u64::MAX` generators when small bounded domains shrink better.
- Avoid format strings in `prop_assert!` when proptest macro support is limited.
- Spec-only functions cannot be called in proptests; inline the simple logic or
  test through an executable wrapper.
