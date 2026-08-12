# Design: Kani complementary verification boundary

## Approach

Trellis treats Kani as external runtime evidence. It may support confidence around exec wrappers, panic-freedom, or boundary regressions, while Verus remains authoritative for checked spec/proof artifacts.

## Evidence roles

Kani evidence in Trellis is classified as:

- runtime-wrapper model-check evidence;
- panic-freedom or boundary-regression evidence;
- external evidence reference for Octet or Valence;
- never a replacement for a Verus `proof fn`, `ensures`, or trusted-boundary closure.

## Documentation boundary

Each Kani reference names source scope, harness or receipt identity, property class, and non-claims. Any prose that mentions Kani must say whether it is complementary evidence or a required proof artifact.

## Fixture strategy

Positive examples show Kani receipts attached to runtime wrapper or boundary evidence. Negative examples show attempted Verus-proof replacement and overbroad whole-module claims.

## What is explicitly not provided

- No requirement to add Kani to the default Trellis verification path.
- No promotion of Kani receipts to Verus proof artifacts.
- No claim that Kani covers ghost-only specifications.
