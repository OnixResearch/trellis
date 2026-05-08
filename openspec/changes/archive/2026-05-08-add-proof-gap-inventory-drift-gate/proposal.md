# Add Proof-Gap Inventory Drift Gate

## Why

The proof-gap boundary inventory is now source-derived and archived, but the check is still manually rerun during audits. Future changes could add or remove trusted-boundary markers without updating README, requirements, OpenSpec counts, or evidence.

## What Changes

- Add a repository-owned Rust script that scans `src/` for trusted/weakened proof-gap markers and compares the live counts with the expected inventory.
- Wire the script into the normal verification rail so drift fails before broader checks claim green.
- Add evidence proving the gate passes on the current tree and fails on a controlled marker-injection fixture.
- Keep the current expected inventory unchanged: 16 `#[verifier::external_body]` sites and zero `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, and `@verified-logic:exempt` markers.

## Out of Scope

- Removing any existing cryptographic, codec, runtime, or hash trust boundary.
- Proving external algorithms or changing the 16-site inventory count.
- Replacing the Rust script with Python or ad-hoc shell inventory logic.

## Verification

Run the Rust inventory script directly, its negative fixture mode, strict OpenSpec validation, Tracey uncovered/untested, Verus, and the full `./scripts/verify.sh` gate.
