## Why

Finite scheduling/calendar primitives describe intervals and resource slots, but
reusable resource accounting needs its own focused family. Budgets, counters,
quotas, leases, usage windows, reservations, overage detection, and deterministic
admission decisions are common proof building blocks for storage, orchestration,
rate limiting, capacity planning, billing metadata, and bounded execution.

This change keeps those primitives spec-only for now. It records exact finite
models that later Verus slices can implement without claiming runtime fairness,
performance isolation, economic policy correctness, distributed metering,
wall-clock billing accuracy, or resource-manager enforcement.

## What Changes

- Add focused `add-resource-accounting-primitives` requirements for finite
  resource-accounting primitives.
- Scope the first implementation plan to exact discrete models:
  - `resource_budget`: finite budget limit, remaining amount, and unit metadata.
  - `resource_counter`: bounded monotone or resettable usage counters.
  - `resource_quota`: quota limit, subject, scope, and admission metadata.
  - `resource_lease`: finite lease interval, holder, and expiry metadata.
  - `resource_usage_window`: bounded usage aggregation windows.
  - `resource_reservation`: finite reservation amount and capacity envelope.
  - `resource_overage`: deterministic overage classification and margin metadata.
  - `resource_admission`: deterministic admit/reject envelope from budget,
    quota, lease, usage, and reservation predicates.
- Update the `add-domain-primitives` umbrella so this family is tracked as a
  separate child change.

## Capabilities

### New Capabilities

- `domain-primitives.resource-accounting-family`: reusable finite
  resource-accounting taxonomy and required child-change scope.
- `resource_budget`: finite budget limit, remaining amount, and unit metadata.
- `resource_counter`: bounded monotone/resettable usage counter metadata.
- `resource_quota`: quota subject, scope, limit, and admission metadata.
- `resource_lease`: finite lease holder/interval/expiry metadata.
- `resource_usage_window`: bounded usage aggregation windows.
- `resource_reservation`: finite reservation amount and capacity envelope.
- `resource_overage`: deterministic overage classification and margin metadata.
- `resource_admission`: deterministic resource admission/rejection envelope.

## Impact

- **Files**: future implementation may add `src/resource_*`, `src/lib.rs`,
  `docs/requirements.md`, tests, and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new runtime dependencies.
- **Testing**: current verification is strict OpenSpec validation. Future
  implementation tasks must add focused unit/property tests, Verus proof checks,
  Tracey coverage, and final repo validation evidence.

## Out of Scope

- implementing resource-accounting primitives in this change
- runtime resource-manager enforcement, OS/container isolation, scheduler
  fairness, or performance guarantees
- economic billing policy correctness, currency conversion, pricing, invoicing,
  or accounting-ledger semantics
- distributed metering consensus, wall-clock billing accuracy, or external
  telemetry correctness
