# UCAN Caveats Specification

## Purpose

Defines correctness properties for composable attenuation constraints
(caveats) in UCAN capabilities. Caveats are additional restrictions
beyond scope and mode that narrow what a capability authorizes.
Delegation can add caveats but never remove them.

## Definitions

- **Caveat**: A typed constraint that restricts capability usage
  (time bound, rate limit, IP range, or custom).
- **Caveat set**: A collection of caveats; all must be satisfied
  (conjunction).
- **Context**: The runtime state checked against caveats (current
  time, request count, source IP, etc.).
- **Caveat containment**: Parent caveat contains child caveat if
  satisfying the child always implies satisfying the parent.
- **Caveat intersection**: The combined constraint of two caveat sets.

## Requirements

### Satisfaction

#### Requirement: Caveat Satisfaction Is Decidable

For each concrete caveat type, satisfaction against a context
MUST be a total, decidable function.

##### Scenario: Time bound check

- GIVEN TimeBound { not_after: 1000 } and context { now: 500 }
- WHEN checking satisfaction
- THEN satisfied (500 <= 1000)

##### Scenario: Rate limit check

- GIVEN RateLimit { max_count: 100, window_secs: 3600 } and context { count_in_window: 50 }
- WHEN checking satisfaction
- THEN satisfied (50 <= 100)

#### Requirement: Caveat Set Satisfaction Is Conjunction

A caveat set is satisfied if and only if every caveat in the
set is individually satisfied.

`caveat_set_satisfied(set, ctx) <==> ∀ c in set: caveat_satisfied(c, ctx)`

##### Scenario: One failing caveat fails the set

- GIVEN set = [TimeBound{ok}, RateLimit{exceeded}]
- WHEN checking set satisfaction
- THEN fails (rate limit not met)

### Containment

#### Requirement: Caveat Containment Is Monotone

If parent contains child, then any context satisfying the child
also satisfies the parent.

`caveat_contains(parent, child) ==> ∀ ctx: caveat_satisfied(child, ctx) ==> caveat_satisfied(parent, ctx)`

##### Scenario: Tighter time bound

- GIVEN parent = TimeBound { not_after: 2000 }, child = TimeBound { not_after: 1000 }
- THEN child containment holds (child is stricter)
- AND any context satisfying child (now <= 1000) also satisfies parent (now <= 2000)

#### Requirement: Caveat Set Containment Is Monotone

If parent set contains child set, then any context satisfying
the child set also satisfies the parent set.

`caveat_set_contains(ps, cs) ==> ∀ ctx: caveat_set_satisfied(cs, ctx) ==> caveat_set_satisfied(ps, ctx)`

### Delegation

#### Requirement: Delegation Only Adds Caveats

A child caveat set in a delegation chain MUST be at least as
restrictive as its parent. No caveat in the parent may be absent
or weakened in the child.

##### Scenario: Child adds rate limit

- GIVEN parent caveats = [TimeBound { not_after: 2000 }]
- AND child caveats = [TimeBound { not_after: 1000 }, RateLimit { max_count: 50 }]
- WHEN checking delegation containment
- THEN valid (child is strictly narrower)

##### Scenario: Child removes time bound rejected

- GIVEN parent caveats = [TimeBound { not_after: 2000 }]
- AND child caveats = [RateLimit { max_count: 50 }]
- WHEN checking delegation containment
- THEN rejected (parent's time bound has no containing child caveat)

### Intersection

#### Requirement: Intersection Is Sound

The intersection of two caveat sets MUST imply both original sets.

`caveat_set_satisfied(intersect(a, b), ctx) ==> caveat_set_satisfied(a, ctx) && caveat_set_satisfied(b, ctx)`

#### Requirement: Intersection Is Minimal

The intersection MUST not be more restrictive than the conjunction
of both sets.

`caveat_set_satisfied(a, ctx) && caveat_set_satisfied(b, ctx) ==> caveat_set_satisfied(intersect(a, b), ctx)`

### Integration

#### Requirement: Caveat Containment Composes With Capability Containment

Capability containment with caveats MUST require both the base
containment (scope + mode + subsumption) and caveat set containment.

`capability_contains_with_caveats(p, c) == base_contains(p, c) && caveat_set_contains(p.caveats, c.caveats)`
