# ChaosControl-Derived Primitives Specification

## Purpose

Defines requirements for verified primitives derived from ChaosControl's pure
exploration kernels: coverage bitmap math, assertion-oracle verdicts,
deterministic scheduler transitions and fingerprints, fault-schedule replay and
subsetting, frontier and input-tree heuristics, and quorum-health fault-state
predicates. Scope is the pure mathematical core, not the hypervisor or explorer
shell.

## Requirements

### Requirement: Coverage Bitmap Region Layout

r[chaoscontrol-coverage.regions]
A ChaosControl coverage bitmap MUST partition its index space into three
non-overlapping contiguous regions: code edges, assertion-state edges, and
schedule-fingerprint edges. Every helper that writes into one region MUST leave
all other regions unaffected.

#### Scenario: Regions are disjoint

- GIVEN a bitmap size `N` and cut points for code and assertion regions
- WHEN the three regions are constructed
- THEN every valid index belongs to exactly one region
- AND no index belongs to two regions at once

#### Scenario: Assertion enrichment stays out of code region

- GIVEN a bitmap and an assertion-state enrichment helper
- WHEN the helper records assertion-state hits
- THEN only indices in the assertion-state region are updated

### Requirement: Coverage Merge and New-Edge Algebra

r[chaoscontrol-coverage.union]
Coverage bitmap merge MUST behave as pointwise union over hit counts, and
new-edge detection MUST report exactly the positions that are hit in the branch
bitmap and absent in the global bitmap.

#### Scenario: Merge is idempotent

- GIVEN a coverage bitmap `A`
- WHEN `A` is merged into itself
- THEN the resulting bitmap is equal to `A`

#### Scenario: New-edge count matches missing positions

- GIVEN branch bitmap `B` and global bitmap `G`
- WHEN new coverage is counted
- THEN the result equals the number of indices where `B[i] > 0` and `G[i] == 0`

### Requirement: Assertion Oracle Verdict Semantics

r[chaoscontrol-oracle.verdict]
Assertion verdict evaluation MUST follow the assertion kind exactly: `always`
passes only when no false evaluation occurred, `sometimes` passes when at least
one true evaluation occurred, `reachable` passes when it was hit, and
`unreachable` passes only when it was never hit.

#### Scenario: Always fails on one false

- GIVEN an `always` assertion with at least one false evaluation
- WHEN its verdict is computed
- THEN the verdict is `failed`

#### Scenario: Unreachable passes when unhit

- GIVEN an `unreachable` assertion with zero hits across all runs
- WHEN its verdict is computed
- THEN the verdict is `passed`

### Requirement: Oracle Run Accounting

r[chaoscontrol-oracle.accounting]
Run-level oracle accounting MUST preserve basic count invariants. In particular,
run-satisfaction count MUST never exceed run-hit count, and boolean assertion
hit counts MUST equal the sum of true and false counts.

#### Scenario: Satisfied runs are a subset of hit runs

- GIVEN any aggregated assertion record
- WHEN run counters are inspected
- THEN `runs_satisfied <= runs_hit`

#### Scenario: Hit count matches true plus false

- GIVEN an `always` or `sometimes` assertion record
- WHEN aggregate counters are inspected
- THEN `hit_count == true_count + false_count`

### Requirement: Scheduler Turn Semantics

r[chaoscontrol-scheduler.turns]
A deterministic scheduler MUST maintain an active turn and remaining quantum.
Ticking MUST decrease the remaining quantum without underflow, and advancing
MUST select the next active participant according to the scheduling rule.

#### Scenario: Countdown reaches switch point

- GIVEN a scheduler with remaining quantum `q > 0`
- WHEN it is ticked `q` times
- THEN the scheduler reaches the switch point exactly once

#### Scenario: Single participant never switches

- GIVEN a scheduler with one participant
- WHEN it is ticked arbitrarily many times
- THEN the active participant remains unchanged

### Requirement: Scheduler Fingerprint Semantics

r[chaoscontrol-scheduler.fingerprint]
The rolling schedule fingerprint MUST be a deterministic function of the
sequence of `(active_turn, assigned_quantum)` transitions. Restoring from the
same scheduler state and replaying the same transitions MUST reproduce the same
fingerprint.

#### Scenario: Same transition sequence reproduces fingerprint

- GIVEN two schedulers with identical initial state
- WHEN they execute the same turn/quantum transition sequence
- THEN their fingerprints are equal

#### Scenario: Variant application resets fingerprint epoch

- GIVEN a scheduler with a non-zero fingerprint
- WHEN a new schedule variant is applied
- THEN the scheduler resets to a fresh epoch before recording new transitions

### Requirement: Fault Schedule Replay Semantics

r[chaoscontrol-fault-schedule.replay]
A fault schedule MUST behave as a time-ordered stream with a monotone cursor.
Draining due faults at time `t` MUST return exactly the not-yet-drained faults
whose scheduled time is `<= t`, in schedule order, and MUST never re-deliver a
fault already drained.

#### Scenario: Due faults drain in order

- GIVEN a schedule with faults at times `t1 <= t2 <= t3`
- WHEN due faults are drained at time `t2`
- THEN the first two faults are returned in schedule order
- AND the third fault remains pending

#### Scenario: Re-drain at same time yields nothing new

- GIVEN a schedule already drained through time `t`
- WHEN due faults are drained again at time `t`
- THEN no previously delivered fault is returned again

### Requirement: Fault Schedule Subset Semantics

r[chaoscontrol-fault-schedule.subset]
Extracting a subset of a fault schedule by source indices MUST preserve the
relative order and exact contents of the selected faults while omitting all
others.

#### Scenario: Relative order preserved

- GIVEN a schedule with faults `[f0, f1, f2, f3]`
- WHEN a subset with indices `[3, 1]` is normalized by schedule order
- THEN the resulting schedule contains `[f1, f3]` in source order

#### Scenario: Empty subset is empty schedule

- GIVEN any schedule
- WHEN the empty index set is selected
- THEN the result is an empty schedule

### Requirement: Frontier Effective Score Ordering

r[chaoscontrol-frontier.selection]
A frontier entry's effective score MUST decrease monotonically as its
selection count increases, and greedy selection MUST choose an entry with
maximal effective score among the available candidates.

#### Scenario: Re-selected entry decays

- GIVEN an entry with base score `s`
- WHEN its selection count increases
- THEN its effective score does not increase

#### Scenario: Greedy choice picks maximal effective score

- GIVEN a non-empty frontier and greedy selection mode
- WHEN the next entry is chosen
- THEN the chosen entry has maximal effective score among all entries

### Requirement: Input-Tree Alternatives Respect Budget

r[chaoscontrol-input-tree.alternatives]
Input-tree alternative generation MUST never emit more than the provided
budget, and every generated alternative MUST differ from the original recorded
choice while remaining inside the choice domain.

#### Scenario: Original value is never regenerated

- GIVEN a recorded bounded choice with original value `v`
- WHEN alternatives are generated
- THEN no generated alternative equals `v`

#### Scenario: Large choice domain still respects budget

- GIVEN a large bounded choice domain and a finite budget `b`
- WHEN alternatives are generated
- THEN at most `b` alternatives are emitted

### Requirement: Fault-State Quorum Health

r[chaoscontrol-fault-state.quorum-health]
Fault-state helpers MUST encode alive membership and partition state
consistently, and quorum-health predicates MUST only report healthy when a
majority of participants are alive and mutually reachable under the chosen
health rule.

#### Scenario: Alive mask matches alive set

- GIVEN a fault state over `n` participants
- WHEN the alive mask is encoded
- THEN bit `i` is set exactly when participant `i` is alive

#### Scenario: No healthy quorum without majority

- GIVEN fewer than a majority of participants are alive
- WHEN quorum health is evaluated
- THEN the result is unhealthy

## Cross-Module Properties

### Requirement: Schedule Replay Preserves Subset Reproducer Order

r[properties.chaoscontrol-subset-replay]
If a fault schedule subset is extracted and then replayed by due-time draining,
the delivered faults MUST appear in the same relative order as in the source
schedule.

### Requirement: Scheduler Fingerprint Depends Only on Turn Trace

r[properties.chaoscontrol-fingerprint-trace]
Two scheduler executions with the same turn/quantum trace MUST produce the
same fingerprint even if they were reached through different imperative shell
paths.

### Requirement: Healthy Quorum Gates Liveness Checks

r[properties.chaoscontrol-quorum-gate]
A liveness gate derived from the fault-state helper MUST never report a healthy
period unless the corresponding quorum-health predicate holds.
