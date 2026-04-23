# UCAN Caveat Specification

## Purpose

Defines the landed caveat model used by `src/ucan_caveat.rs`. Caveats are
additional attenuation constraints beyond scope or ability. In the
landed module, caveats are limited to time bounds and rate limits, caveat
sets are conjunctions, and set intersection is modeled as concatenation.

## Definitions

- **Caveat**: One of:
  - `TimeBound { not_after: u64 }`
  - `RateLimit { max_count: u64, window_secs: u64 }`
- **Caveat set**: A sequence of caveats; all members must be satisfied.
- **Context**: The runtime state checked against caveats:
  - `now: u64`
  - `count_in_window: u64`
- **Caveat containment**: `caveat_contains_spec(parent, child)`.
  Cross-variant containment is always false; only same-variant pairs can
  satisfy the containment relation.
- **Caveat-set containment**: Every parent caveat is contained by some
  child caveat.
- **Intersection**: `caveat_intersect_spec(a, b) == a + b`.

## Requirements

### Satisfaction

#### Requirement: Caveat Satisfaction Is Decidable
`r[ucan-caveat.satisfaction-decidable]`

For each landed caveat variant, satisfaction against a context MUST be a
total, decidable function.

##### Scenario: Time bound satisfied before expiry
`r[ucan-caveat.satisfaction-decidable.scenario-time-bound-ok]`

- GIVEN `TimeBound { not_after: 1000 }` and context `{ now: 500 }`
- WHEN checking satisfaction
- THEN the result is true

##### Scenario: Rate limit exceeded
`r[ucan-caveat.satisfaction-decidable.scenario-rate-limit-exceeded]`

- GIVEN `RateLimit { max_count: 100, window_secs: 3600 }` and context `{ count_in_window: 150 }`
- WHEN checking satisfaction
- THEN the result is false

#### Requirement: Caveat Set Satisfaction Is Conjunction
`r[ucan-caveat.set-conjunction]`

A caveat set is satisfied if and only if every caveat in the set is
individually satisfied.

`caveat_set_satisfied_spec(set, ctx) <==> forall c in set: caveat_satisfied_spec(c, ctx)`

##### Scenario: One failing caveat fails the set
`r[ucan-caveat.set-conjunction.scenario-one-failing-caveat]`

- GIVEN set = `[TimeBound { not_after: 2000 }, RateLimit { max_count: 100, window_secs: 3600 }]`
- AND context `{ now: 3000, count_in_window: 50 }`
- WHEN checking set satisfaction
- THEN the result is false because the time bound fails

### Containment

#### Requirement: Caveat Containment Is Monotone
`r[ucan-caveat.containment-monotone]`

If a parent caveat contains a child caveat, any context satisfying the
child MUST also satisfy the parent.

`caveat_contains_spec(parent, child) ==> forall ctx: caveat_satisfied_spec(child, ctx) ==> caveat_satisfied_spec(parent, ctx)`

##### Scenario: Tighter time bound remains safe
`r[ucan-caveat.containment-monotone.scenario-tighter-time-bound]`

- GIVEN parent = `TimeBound { not_after: 2000 }` and child = `TimeBound { not_after: 1000 }`
- WHEN a context satisfies the child
- THEN that context also satisfies the parent

#### Requirement: Caveat Set Containment Is Monotone
`r[ucan-caveat.set-containment-monotone]`

If every parent caveat is contained by some child caveat, any context
satisfying the child set MUST also satisfy the parent set.

`caveat_set_contains_spec(parent_set, child_set) ==> forall ctx: caveat_set_satisfied_spec(child_set, ctx) ==> caveat_set_satisfied_spec(parent_set, ctx)`

##### Scenario: Child adds a tighter rate limit
`r[ucan-caveat.set-containment-monotone.scenario-child-adds-rate-limit]`

- GIVEN parent set = `[TimeBound { not_after: 2000 }]`
- AND child set = `[TimeBound { not_after: 1000 }, RateLimit { max_count: 50, window_secs: 3600 }]`
- WHEN checking set containment
- THEN the child set is at least as restrictive as the parent set

### Delegation

#### Requirement: Delegation Only Adds Caveats
`r[ucan-caveat.delegation-only-adds]`

A child caveat set in a delegation chain MUST be at least as restrictive
as its parent. No parent caveat may be absent or weakened in the child.
This is the delegation-facing restatement of
`r[ucan-caveat.set-containment-monotone]` for parent/child token chains.

##### Scenario: Child removes time bound is rejected
`r[ucan-caveat.delegation-only-adds.scenario-child-removes-time-bound-rejected]`

- GIVEN parent set = `[TimeBound { not_after: 2000 }]`
- AND child set = `[RateLimit { max_count: 50, window_secs: 3600 }]`
- WHEN checking set containment
- THEN the result is false because no child caveat contains the parent time bound

### Intersection

#### Requirement: Intersection Is Sound
`r[ucan-caveat.intersection-sound]`

The landed intersection model MUST imply both original caveat sets.

`caveat_set_satisfied_spec(caveat_intersect_spec(a, b), ctx) ==> caveat_set_satisfied_spec(a, ctx) && caveat_set_satisfied_spec(b, ctx)`

##### Scenario: Combined set implies both inputs
`r[ucan-caveat.intersection-sound.scenario-combined-set-implies-both]`

- GIVEN caveat sets `a` and `b`
- WHEN a context satisfies `caveat_intersect_spec(a, b)`
- THEN that context also satisfies both `a` and `b`

#### Requirement: Intersection Is Minimal
`r[ucan-caveat.intersection-minimal]`

The landed intersection model MUST not add restrictions beyond requiring
both original sets.

`caveat_set_satisfied_spec(a, ctx) && caveat_set_satisfied_spec(b, ctx) ==> caveat_set_satisfied_spec(caveat_intersect_spec(a, b), ctx)`

##### Scenario: Satisfying both inputs satisfies the combined set
`r[ucan-caveat.intersection-minimal.scenario-satisfy-both-implies-combined]`

- GIVEN caveat sets `a` and `b`
- WHEN a context satisfies both `a` and `b`
- THEN that context satisfies `caveat_intersect_spec(a, b)`
