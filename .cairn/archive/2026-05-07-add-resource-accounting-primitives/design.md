## Context

The repository now has finite storage/index, scheduling/calendar, and control
system primitive families. Resource accounting overlaps with each but should
remain separate: it describes exact finite budgets, quotas, leases, counters,
usage windows, reservations, overage metadata, and deterministic admission
predicates, not storage durability, scheduling fairness, or controller behavior.

## Goals / Non-Goals

**Goals:**

- define exact finite resource-accounting primitive requirements
- keep validity boundaries explicit for budgets, counters, quotas, leases, usage
  windows, reservations, overage decisions, and admission decisions
- support later Verus proofs using bounded integers, finite subjects/scopes,
  half-open windows, and deterministic decision metadata
- avoid runtime enforcement, fairness, performance, billing-policy, or distributed
  metering claims

**Non-Goals:**

- implementing modules in this change
- proving OS/container resource isolation, scheduler fairness, or performance
  behavior
- proving billing/pricing/currency/accounting-ledger correctness
- modeling distributed metering, external telemetry, or wall-clock billing
  accuracy

## Decisions

### 1. Model accounting as finite envelopes and decision metadata

**Choice:** Requirements describe budgets, bounded counters, quota scopes, lease
intervals, usage windows, reservations, overage classes, and admission outcomes
over finite integer domains.

**Rationale:** These facts are reusable and Verus-friendly. They support later
rate-limit, capacity, lease, and quota logic without pulling in runtime resource
manager or billing semantics.

**Alternative:** Start with a full quota/billing/rate-limit engine. Rejected
because enforcement, pricing, distributed measurement, and fairness policies are
application-specific and too broad for a reusable primitive baseline.

### 2. Keep admission deterministic and policy-minimal

**Choice:** Resource admission primitives return exact admit/reject metadata based
on finite budgets, quotas, leases, usage windows, reservations, and overage
predicates. They do not claim fairness, optimal allocation, billing correctness,
or runtime enforcement.

**Rationale:** Deterministic structural admission is valuable and provable.
Allocation policies and economic semantics require separate models and objectives.

**Alternative:** Promise generic resource-accounting correctness. Rejected because
useful systems differ in units, quota hierarchy, reservation semantics, lease
renewal, overage policy, and enforcement strategy.

### 3. Separate resource accounting from scheduling and storage

**Choice:** This child change owns `resource_*` primitives. Existing storage/index
changes own page/record/key/cursor/segment facts, and scheduling/calendar owns
interval/resource-slot scheduling facts.

**Rationale:** Resource-accounting primitives can reference storage or schedule
metadata later without duplicating their domain-specific semantics.

## Risks / Trade-offs

**Over-claiming enforcement behavior** → Requirements exclude OS/container,
scheduler, runtime isolation, and performance guarantees.

**Billing-policy drift** → Requirements model finite amounts, units, limits, and
decision metadata only; pricing, currency, invoices, and ledgers belong in later
focused changes if needed.

**Duplicating scheduling/storage requirements** → This change owns accounting
budgets/quotas/leases/usage/reservations and uses scheduling/storage concepts
only as future dependencies.

## Validation Plan

- `openspec validate add-resource-accounting-primitives --strict --json`
- strict validation for all active primitive-family OpenSpecs
- future implementation: focused cargo tests, Verus checks, Tracey uncovered/
  untested checks, and final repo validation before archive
