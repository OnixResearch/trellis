## Context

`src/verification_frontier.rs` already models finite admissions for contract implication, type-bound facts, slice access, arena provenance, progress budgets, and external-reference boundaries. The current proofs establish important facts, but the reusable invariant surface is not yet named as a stable bundle for later consumers.

## Goals

- Name the outcome invariants that all admission helpers must respect.
- Provide proof helpers that callers can use without unfolding each admission implementation.
- Keep reason/evidence metadata deterministic and tied to named constants.
- Cover both happy-path and sad-path behavior with tests.

## Non-Goals

- Do not add new trusted proof boundaries.
- Do not change the public meaning of `Proven`, `Rejected`, or `Unknown`.
- Do not depend on external Lattice/Salt code, benchmarks, or solver behavior.

## Decisions

### Outcome invariant bundle

The bundle SHALL include at least these reusable facts: `Proven` implies the modeled safety predicate for its admission family, `Unknown` is not proof evidence, rejected outcomes expose deterministic reason/evidence metadata, and public helpers return only stable outcome variants.

### Admission-family helpers

Each admission family SHOULD have a small proof helper that states its strongest useful invariant without requiring consumers to inspect nested case splits. Helpers may be family-specific when one generic predicate would hide important evidence details.

### Reason-code stability

Reason codes and evidence fields SHALL remain named constants or named input-derived values. Tests MUST include malformed inputs that exercise unsupported predicates, inverted ranges, out-of-backing slices, unbounded dynamic offsets, over-budget paths, no-yield cycles, and untrusted external-reference claims.

### External-reference boundary

Any documentation for this bundle SHALL repeat that Lattice/Salt remains an external design reference only. Local proofs, tests, Tracey coverage, and validation logs are the only accepted Trellis evidence.
