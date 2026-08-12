## Why

Trellis already uses Nickel for several policy surfaces, but the current contracts mostly validate shape. Repeated local predicates drift across policy files, and reference fields such as rule IDs, validation gates, replay cases, replay groups, stages, modes, and artifact dependencies can still become dangling or inconsistent while the policy continues to typecheck.

## What Changes

- Introduce a shared Nickel contract library for reusable string, enum, path, digest, non-empty collection, and uniqueness predicates.
- Refactor existing Nickel policies to consume the shared contracts instead of re-declaring local copies.
- Strengthen proof-review, verification-policy, and cairn-policy contracts with cross-field integrity checks.
- Add positive and negative fixtures that prove invalid references, duplicate IDs, unsafe paths, and unsupported enum values fail closed.

## Scope

In scope:
- `*.ncl` policy contract factoring and validation.
- Generated policy JSON freshness checks.
- Fixture-backed positive and negative checks for policy admission.
- Integration into the smallest existing verification-policy/proof-review/cairn-policy checkers.

Out of scope:
- Changing policy behavior beyond rejecting malformed policy data.
- Replacing Rust scanners, proof-review ranking logic, or Cairn lifecycle logic with Nickel.
- Adding broad exemptions or weakening validation gates.

## Impact

- **Files**: `verification-policy/`, `proof-review/`, `cairn-policy/`, generated policy JSON, policy checker scripts, and policy fixtures.
- **Risks**: stronger contracts may reveal latent stale policy references; resolve by fixing the policy data, not by relaxing predicates.
- **Testing**: Nickel typecheck/export, policy freshness, positive/negative policy fixtures, proof-review self-tests, and repo verification wrapper.
