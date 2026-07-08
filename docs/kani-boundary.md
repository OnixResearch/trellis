# Kani complementary verification boundary

Trellis treats Kani receipts as complementary external evidence. They may support review of runtime wrappers, panic-freedom harnesses, and boundary-regression checks, but they do not carry Trellis verified-logic claims.

Verus remains the authority for Trellis proof claims. A claim that a theorem, `ensures` clause, `proof fn`, or trusted-boundary closure is verified must be backed by a Verus proof artifact or by an accepted Trellis proof artifact derived from Verus verification. A Kani receipt can sit next to that proof as runtime evidence, but it cannot replace it.

## Required Kani reference fields

Every Trellis Kani reference must name:

- **Classification:** `complementary-external-evidence`.
- **Source scope:** the file, function, wrapper, harness, or boundary the receipt covers.
- **Receipt identity:** the Kani receipt identifier or BLAKE3 digest.
- **Property class:** one of `runtime-wrapper`, `panic-freedom`, `boundary-regression`, or `consumer-reference`.
- **Trellis proof artifact:** the Verus proof artifact that remains authoritative, or an explicit statement that no Trellis proof claim is being made.
- **Non-claims:** explicit non-claims excluding Verus proof replacement, whole-module correctness, trusted-boundary closure, and ghost/spec theorem evidence.

## Positive use

A runtime wrapper may cite a Kani receipt that checks panic freedom for a concrete harness while the wrapper's `ensures` and any theorem-level claim stay tied to Verus evidence. Octet and Valence may consume the Kani receipt as external runtime evidence and consume Trellis Verus artifacts separately as proof evidence.

## Rejected use

Trellis must reject or flag prose that treats Kani as a Verus proof artifact, uses a scoped receipt to claim whole-module correctness, closes a trusted boundary with only Kani, or says Kani covers ghost-only specifications. Those claims require separate Verus artifacts and accepted Trellis proof-gap review.

## Consumer guidance

Octet may reference Trellis Kani receipts as optional external evidence for runtime-wrapper and panic-freedom rails. Valence may model those receipts under external-evidence roles and non-claim boundaries. Neither consumer should flatten them into Trellis proof artifacts: Verus artifacts carry Trellis proof claims, and Kani receipts remain complementary runtime evidence.

Run the boundary checker after editing this guide or the fixtures:

```bash
cargo -Zscript scripts/check-kani-boundary.rs
cargo -Zscript scripts/check-kani-boundary.rs --self-test
```
