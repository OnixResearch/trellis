# ChaosControl Cross-Module Properties Specification

## Purpose

Defines the cross-module theorems tying the ChaosControl-derived kernels
together.

## Requirements

### Requirement: Schedule Replay Preserves Subset Reproducer Order

r[properties.chaoscontrol-subset-replay]
Depends on `r[chaoscontrol-fault-schedule.replay]` and
`r[chaoscontrol-fault-schedule.subset]`.
For normalized schedules, if `subset(source, indices)` is replayed from
`cursor = 0` by any nondecreasing time sequence whose final drain time
`t_final >= max(time_ns)` over all faults in the subset, delivered faults MUST
appear in the same relative order as in `source`.

#### Scenario: Fully replayed subset preserves source relative order

- GIVEN a normalized source schedule and any subset extracted from source indices
- WHEN that subset is replayed from `cursor = 0` by a nondecreasing time sequence whose final drain time `t_final >= max(time_ns)` over all subset faults
- THEN delivered faults appear in the same relative order as in the source schedule

### Requirement: Scheduler Fingerprint Depends Only on Turn Trace

r[properties.chaoscontrol-fingerprint-trace]
Depends on `r[chaoscontrol-scheduler.fingerprint]`.
For schedulers that start from the same full scheduler state, or from the same
full post-`apply_variant` reset state, executions with the same post-advance
`(active, assigned_quantum)` trace MUST produce the same fingerprint even if
the imperative shell path differs. Here "same full post-`apply_variant` reset
state" means the same `num_vcpus`, the same persisted strategy parameters, the
same stored `default_quantum`, and the same reset fields `active = 0`,
`remaining = default_quantum`, and `fingerprint = 0`.

#### Scenario: Equal turn traces force equal fingerprints

- GIVEN two schedulers with the same initial state or the same full post-`apply_variant` reset state, including matching `num_vcpus`, persisted strategy parameters, and stored `default_quantum`
- WHEN both executions produce the same post-advance `(active, assigned_quantum)` trace
- THEN their final fingerprints are equal

### Requirement: Healthy Quorum Gates Liveness Checks

r[properties.chaoscontrol-quorum-gate]
Depends on `r[chaoscontrol-fault-state.quorum-health]` and
`r[chaoscontrol-fault-state.liveness-gate]`.
This is a separate cross-module proof target for `src/properties.rs`, not a
second definition of the gate itself: it records the one-way implication that
other modules may reuse from the gate definition. For valid fault states, a
healthy-period gate MUST never report healthy unless `quorum_reachable` holds
for that same state.

#### Scenario: Healthy period implies quorum reachability

- GIVEN a valid fault state
- WHEN the healthy-period gate reports healthy
- THEN `quorum_reachable` holds for that same state
