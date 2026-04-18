# ChaosControl Fault-State Specification

## Purpose

Defines the fault-state quorum-health helper and the derived healthy-period
gate.

## Requirements

### Requirement: Fault-State Quorum Health

r[chaoscontrol-fault-state.quorum-health]
Fault-state helpers apply only to states with `2 <= num_nodes <= 64`, an exact
`num_nodes x num_nodes` directed partition matrix, diagonal-false entries, and
an `alive_mask` whose bits above `num_nodes - 1` are clear. `alive_mask` MUST
set bit `i` exactly when participant `i` is alive. `partition_count` MUST
count exactly the directed blocked non-diagonal links with
`partitioned[a][b] == true`. Here `majority(n) = floor(n / 2) + 1`.
`quorum_reachable` in this change is the current ChaosControl
majority-alive + direct-bidirectional-pair heuristic: it is false when fewer
than `majority(n)` participants are alive, and otherwise true exactly when
some distinct alive pair `a != b` has
`!partitioned[a][b] && !partitioned[b][a]`. Stronger transitive or quorum-
subgraph notions are out of scope.

#### Scenario: Alive mask matches alive set

- GIVEN a valid fault state
- WHEN the alive mask is encoded
- THEN bit `i` is set exactly when participant `i` is alive

#### Scenario: Partition count ignores diagonal entries

- GIVEN a valid asymmetric partition matrix
- WHEN partition count is computed
- THEN the result equals the number of non-diagonal entries with `partitioned[a][b] == true`

#### Scenario: No quorum health without majority

- GIVEN fewer than a majority of participants are alive
- WHEN quorum reachability is evaluated
- THEN the result is `false`

#### Scenario: Bidirectional alive pair makes quorum reachable

- GIVEN at least a majority of participants are alive
- AND distinct alive participants `a` and `b` satisfy `!partitioned[a][b]` and `!partitioned[b][a]`
- WHEN quorum reachability is evaluated
- THEN the result is `true`

### Requirement: Fault-State Healthy-Period Gate

r[chaoscontrol-fault-state.liveness-gate]
Depends on `r[chaoscontrol-fault-state.quorum-health]`.
For valid fault states, the derived healthy-period gate reports healthy
exactly when all participants are alive and `quorum_reachable` holds for that
same state.

#### Scenario: Healthy gate requires all participants alive

- GIVEN a valid state where one participant is crashed
- WHEN the healthy-period gate is evaluated
- THEN the result is `false`

#### Scenario: Healthy gate follows quorum reachability when all are alive

- GIVEN a valid state where all participants are alive
- WHEN the healthy-period gate is evaluated
- THEN the result equals `quorum_reachable` for that state
