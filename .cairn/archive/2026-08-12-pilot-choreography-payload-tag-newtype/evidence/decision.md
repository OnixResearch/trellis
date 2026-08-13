# Payload-tag pilot decision

## Baseline

- Focused choreography tests: 28 passed.
- Trusted-boundary inventory: 16 external bodies and no other proof gaps.
- Tracey: 0 uncovered and 0 untested rules after the daemon rerun.
- The baseline full-library Verus derivation was already present in the Nix store. The repository headline records 10712 verified checks.

## Final result

- Focused choreography tests: 30 passed.
- Compile-fail doctest: 1 passed.
- Full-library Verus: 10720 verified and 0 errors.
- Trusted-boundary inventory: unchanged at 16 external bodies and no other proof gaps.
- Tracey: 0 uncovered and 0 untested rules.
- Octet exact baseline check: 0 new, 0 stale, and 0 errors.
- Canonical Octet pre-commit gate: passed on the final tree.

## Proof cost

The type migration required these changes:

- wrap runtime and test payload literals in `PayloadTag`;
- retain typed payload values through global and local views;
- add one verified `payload_tags_equal` helper for executable merge branches;
- add no trigger repair;
- change no existing proof function;
- add no trusted bridge, external body, assumption, or proof-gap marker.

## Decision

Continue to a distinct `ValueId` in the separate value-transfer change. The pilot does not authorize a `RoleId` or `LabelId` migration.

This result proves only typed payload-domain separation and preservation of the checked choreography semantics. It does not prove transport, delivery, authority, or runtime behavior.
