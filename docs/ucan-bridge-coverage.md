# UCAN Bridge Coverage

`docs/ucan-bridge-coverage.json` maps UCAN-facing helper names to Trellis primitive/proof manifest rows. The checker is `scripts/check-ucan-bridge-coverage.rs` and the row contract is `docs/ucan-bridge-coverage.contract.ncl`.

Trellis owns primitive proof identity. UCAN owns runtime bridge orchestration, including parser correctness, resolver truth, replay-store durability, token shell behavior, and consumer authorization completeness.

## Boundary

A passing row proves identity coverage only: the named UCAN helper is linked to a Trellis primitive ID and matching source/proof artifact digests. It does not prove UCAN parser correctness, does not prove resolver truth, does not prove replay-store durability, and does not prove consumer authorization completeness.

## Refresh

After changing UCAN-facing Trellis primitives or UCAN vendored bridge rows, refresh the manifest and run:

```bash
./scripts/check-ucan-bridge-coverage.rs
./scripts/check-ucan-bridge-coverage.rs --self-test
```
