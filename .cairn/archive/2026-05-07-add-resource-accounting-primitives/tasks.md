## Coverage Index

| Requirement ID | Future module | Purpose |
|---|---|---|
| `domain-primitives.resource-accounting.budget` | `src/resource_budget.rs` | finite budget limits and remaining amounts |
| `domain-primitives.resource-accounting.counter` | `src/resource_counter.rs` | bounded monotone/resettable usage counters |
| `domain-primitives.resource-accounting.quota` | `src/resource_quota.rs` | quota subject, scope, limit, and admission metadata |
| `domain-primitives.resource-accounting.lease` | `src/resource_lease.rs` | finite lease holder, interval, and expiry metadata |
| `domain-primitives.resource-accounting.usage-window` | `src/resource_usage_window.rs` | bounded usage aggregation windows |
| `domain-primitives.resource-accounting.reservation` | `src/resource_reservation.rs` | finite reservation amount and capacity envelopes |
| `domain-primitives.resource-accounting.overage` | `src/resource_overage.rs` | deterministic overage classification and margin metadata |
| `domain-primitives.resource-accounting.admission` | `src/resource_admission.rs` | deterministic resource admission/rejection envelope |

## Phase 1: Spec foundation

- [x] Create resource-accounting primitive OpenSpec with exact finite models and non-goals.

## Phase 2: Future implementation slices

- [x] Implement `resource_budget` with finite budget limit, consumed amount, remaining amount, and unit metadata. `[covers=domain-primitives.resource-accounting.budget]`
- [x] Implement `resource_counter` with bounded monotone and resettable counter predicates. `[covers=domain-primitives.resource-accounting.counter]`
- [x] Implement `resource_quota` with subject, scope, limit, and quota admission predicates. `[covers=domain-primitives.resource-accounting.quota]`
- [x] Implement `resource_lease` with finite holder, interval, renewal, and expiry metadata. `[covers=domain-primitives.resource-accounting.lease]`
- [x] Implement `resource_usage_window` with bounded window aggregation and no-overflow predicates. `[covers=domain-primitives.resource-accounting.usage-window]`
- [x] Implement `resource_reservation` with finite reservation amount and capacity envelope predicates. `[covers=domain-primitives.resource-accounting.reservation]`
- [x] Implement `resource_overage` with deterministic overage classification and margin metadata. `[covers=domain-primitives.resource-accounting.overage]`
- [x] Implement `resource_admission` with deterministic admit/reject envelopes. `[covers=domain-primitives.resource-accounting.admission]`

## Phase 3: Evidence and archive

- [x] Add `docs/requirements.md`, `r[impl]`, `r[depends]`, and `r[verify]` coverage for implemented resource-accounting primitives.
- [x] Run focused tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and final repo validation.
- [x] Sync canonical specs and archive after implementation/evidence tasks complete.
