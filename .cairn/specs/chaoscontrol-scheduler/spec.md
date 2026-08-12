# ChaosControl Scheduler and Fault-Schedule Specification

## Purpose

Defines the deterministic scheduler and fault-schedule kernels derived from
ChaosControl.

## Requirements

### Requirement: Scheduler Turn Semantics

r[chaoscontrol-scheduler.turns]
Scheduler primitives MUST apply only when `num_vcpus >= 1` and valid scheduler
state. Valid scheduler state means `0 <= active < num_vcpus`, stored
`default_quantum >= 1`, randomized strategies satisfy `1 <= min_quantum <= max_quantum`,
and `remaining` is either the exhausted SMP value `0` immediately after a
`ticked-to-zero` turn and before the next `advance`, or a positive countdown
value for the current turn's assigned quantum. Stored configuration always uses
`default_quantum = max(1, raw_default_quantum)`. `default_quantum` is a
caller-provided scalar independent of strategy: the constructor accepts it
regardless of whether the strategy is round-robin or randomized. Round-robin
advance reloads `remaining` from the stored `default_quantum`; randomized
advance ignores `default_quantum` and instead derives the reload value from
`rng_word` and the strategy's `[min_quantum, max_quantum)` range. If the stored
strategy is randomized, valid state also requires
`1 <= min_quantum <= max_quantum`; constructing or applying a randomized
strategy outside that bound is a precondition failure outside contract. Initial
state is `active = 0`, `remaining = default_quantum`, `fingerprint = 0`. In
single-vCPU mode, `tick` returns `false` and `tick`/`advance` leave
`active`, `remaining`, and `fingerprint` unchanged. In SMP mode, `tick`
requires `remaining > 0`, decrements `remaining` by one without underflow,
and returns `true` exactly on the positive-to-zero transition; after a `true`
result the next legal scheduler transition is `advance`, and any later SMP
`tick` before that `advance` violates the same `remaining > 0` precondition and
is outside contract. SMP `advance` itself is defined on any valid scheduler
state: if called before countdown exhaustion, it ends the current turn early,
then performs the same next-vCPU / quantum-reload transition and fingerprint
update as any other `advance`. `advance` selects `(active + 1) mod num_vcpus`
and reloads `remaining` to the assigned quantum.
Round-robin mode uses `default_quantum`. Randomized mode consumes a caller
`rng_word: u64` and uses the existing ChaosControl lower-inclusive,
upper-exclusive rule: when `min_quantum < max_quantum`, assigned quantum is
`min_quantum + (rng_word mod (max_quantum - min_quantum))`, so results lie in
`[min_quantum, max_quantum)` and `max_quantum` itself is not reached; when
`min_quantum == max_quantum`, assigned quantum is exactly `min_quantum`.
Constructor and `apply_variant` consume no RNG word.

#### Scenario: Countdown reaches switch point exactly once

- GIVEN an SMP scheduler with `remaining = q > 0`
- WHEN it is ticked `q` times without early advance
- THEN the first `q - 1` ticks return `false`
- AND the `q`th tick returns `true`

#### Scenario: Constructor canonicalizes default quantum

- GIVEN `num_vcpus >= 1` and any raw default quantum
- WHEN a scheduler is constructed
- THEN `active == 0`
- AND `remaining == max(1, raw_default_quantum)`
- AND `fingerprint == 0`

#### Scenario: Invalid randomized override is outside contract

- GIVEN a randomized override with `min_quantum > max_quantum`
- WHEN `apply_variant` is invoked
- THEN the `requires` precondition is violated

### Requirement: Scheduler Fingerprint Semantics

r[chaoscontrol-scheduler.fingerprint]
The rolling schedule fingerprint MUST update by exact wrapping-`u64`
transition rule:
`fp' = fp.wrapping_mul(0x517c_c1b7_2722_0a95).bitxor(active as u64).bitxor(assigned_quantum as u64)`,
where `active` is the post-advance vCPU index (the `active` field after the
round-robin step) and `assigned_quantum` is the reload value chosen by that
advance. `tick` is fingerprint-neutral. Each SMP `advance` performs exactly one
mix using the post-advance `(active, assigned_quantum)` pair, so the
fingerprint is a deterministic function of that trace. A schedule variant is
`(strategy_override, quantum_override)`. `apply_variant` persists any present
`strategy_override`. When `quantum_override` is present, it updates the stored
`default_quantum` to `max(1, raw_quantum_override)`; when it is `None`, the
stored `default_quantum` is unchanged. After persisting overrides,
`apply_variant` resets `active = 0`, `remaining = default_quantum`, and
`fingerprint = 0`. If the persisted strategy after reset is randomized, later
randomized `advance` calls still use that strategy's lower-inclusive,
upper-exclusive range `[min_quantum, max_quantum)` when `min_quantum < max_quantum`,
and exact `min_quantum` when `min_quantum == max_quantum`; the persisted
`default_quantum` affects only stored/reset scalar quantum state.

#### Scenario: Same transition sequence reproduces fingerprint

- GIVEN two schedulers with the same initial state
- WHEN they execute the same post-advance `(active, assigned_quantum)` trace
- THEN their fingerprints are equal

#### Scenario: Variant application resets fingerprint epoch

- GIVEN a scheduler with a non-zero fingerprint
- WHEN a new schedule variant is applied
- THEN the fingerprint becomes `0` before later transitions are mixed in

### Requirement: Fault Schedule Replay Semantics

r[chaoscontrol-fault-schedule.replay]
Replay and subset helpers MUST operate on normalized schedules. A normalized fault
schedule has unique ordinals, stable `(time_ns, ordinal)` order, and
`0 <= cursor <= len`. The public `add` helper is defined only on
undrained normalized schedules with `cursor = 0`; adding to a partially drained
schedule is outside contract. On that input surface, `add` allocates fresh
ordinals as `0` on an empty schedule and otherwise `1 + max(existing ordinals)`,
including on subset-produced schedules whose existing ordinals came from carried
source indices, and then normalizes by `(time_ns, ordinal)` while keeping the
result cursor at `0`, so the output remains normalized. `drain_due(t)` returns
exactly the not-yet-drained faults with `time_ns <= t` starting at `cursor`, in
schedule order, advances `cursor` by exactly the number returned, preserves the
undrained suffix, keeps `0 <= cursor <= len`, is monotone in `cursor`, and
never re-delivers an already drained fault.

#### Scenario: Due faults drain in normalized order

- GIVEN faults at times `t1 <= t2 <= t3`
- WHEN due faults are drained at time `t2`
- THEN the first two faults are returned in normalized order
- AND the third fault remains pending

#### Scenario: Re-drain yields nothing new

- GIVEN a schedule already drained through time `t`
- WHEN due faults are drained again at time `t`
- THEN no previously delivered fault is returned again

#### Scenario: Replay operations preserve normalized bounds

- GIVEN a normalized schedule
- WHEN `add` or `drain_due` is applied within contract
- THEN the resulting schedule still satisfies `0 <= cursor <= len`
- AND `drain_due` never decreases `cursor`

### Requirement: Fault Schedule Subset Semantics

r[chaoscontrol-fault-schedule.subset]
Subset extraction MUST range over the full normalized source schedule. It clones
exactly the first occurrence of each selected in-bounds source index, ignores
out-of-bounds indices, ignores repeated selections after the first, carries
that source schedule index forward as the result `ordinal`, resets the result
cursor to `0`, and renormalizes by the same stable `(time_ns, ordinal)` order.

#### Scenario: Relative order is preserved after renormalization

- GIVEN normalized source schedule `[f0, f1, f2, f3]`
- WHEN subset indices are `[3, 1]`
- THEN the result contains `[f1, f3]` in source schedule order

#### Scenario: Out-of-bounds indices are ignored

- GIVEN a schedule with one fault
- WHEN subset indices are `[0, 5]`
- THEN the result contains only the original first fault
