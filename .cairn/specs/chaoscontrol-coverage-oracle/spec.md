# ChaosControl Coverage and Oracle Specification

## Purpose

Defines the pure coverage-bitmap and assertion-oracle kernels derived from
ChaosControl.

## Requirements

### Requirement: Coverage Bitmap Region Layout
r[chaoscontrol-coverage.regions]

A coverage bitmap MUST be a `65536`-byte array partitioned into three exact,
non-overlapping contiguous regions: code `[0, 32768)`, assertion
`[32768, 49152)`, and schedule `[49152, 65536)`. Coverage helpers operate on
that fixed shape. Any public helper that records code-region hits,
assertion-region hits, or schedule-region hits into exactly one of those
regions MUST leave the other two regions unchanged.

#### Scenario: Region partition is exact

- GIVEN the three region cut points
- WHEN the regions are constructed
- THEN their lengths are `32768`, `16384`, and `16384`
- AND together they cover exactly `0 .. 65536`
- AND no valid index belongs to two regions

#### Scenario: Region writes are isolated

- GIVEN a bitmap and a helper for one region
- WHEN the helper updates that region
- THEN every index outside that region is unchanged

### Requirement: Coverage Merge and New-Edge Algebra
r[chaoscontrol-coverage.union]

Coverage merge MUST be exact pointwise `max` over `u8` counters. A new-edge
query MUST return the exact `u32` count of indices `i` with `branch[i] > 0`
and `global[i] == 0`.

#### Scenario: Merge keeps larger count

- GIVEN `A[i] = 3` and `B[i] = 1`
- WHEN `A` is merged with `B`
- THEN the merged count at `i` is `3`

#### Scenario: New-edge count matches missing positions

- GIVEN branch bitmap `B` and global bitmap `G`
- WHEN new coverage is counted
- THEN the result equals the number of indices where `B[i] > 0` and `G[i] == 0`

### Requirement: Coverage Classification Buckets
r[chaoscontrol-coverage.classify]

Coverage classification MUST use the exact AFL bucket map:
`0 -> 0`, `1 -> 1`, `2 -> 2`, `3 -> 3`, `4..7 -> 4`, `8..15 -> 8`,
`16..31 -> 16`, `32..127 -> 32`, and `128..255 -> 128`.

#### Scenario: Midrange counts map to bucket floor

- GIVEN counts `5`, `10`, `20`, and `50`
- WHEN classification is applied
- THEN the bucket values are `4`, `8`, `16`, and `32`

#### Scenario: Saturated counts map to 128

- GIVEN count `255`
- WHEN classification is applied
- THEN the bucket value is `128`

### Requirement: Assertion Oracle Verdict Semantics
r[chaoscontrol-oracle.verdict]

Depends on `r[chaoscontrol-oracle.accounting]`.
Verdict evaluation applies only to assertion kinds
`{always, sometimes, reachable, unreachable}` and aggregate records whose
counters are nonnegative by type and satisfy the valid-record surface below.
Invalid non-boolean shapes are outside contract. It MUST use `hit_count > 0` as the meaning
of “was hit”. `always` passes iff it was hit and no false evaluation occurred.
`sometimes` passes iff it was hit and at least one true evaluation occurred.
`reachable` passes iff it was hit. `unreachable` passes iff it was never hit.
Valid boolean inputs also satisfy the accounting surface from
`r[chaoscontrol-oracle.accounting]`: `hit_count == true_count + false_count`,
`runs_hit` counts runs with at least one evaluation and satisfies
`runs_hit <= hit_count`, and `runs_satisfied` counts hit runs satisfying the
corresponding boolean kind's pass condition. For `sometimes`, valid records
also satisfy `runs_satisfied <= true_count`.
Valid non-boolean inputs are exact: `reachable` requires
`true_count == hit_count`, `false_count == 0`, `runs_satisfied == runs_hit`,
and `runs_hit == 0 <=> hit_count == 0`; `unreachable` requires
`true_count == 0`, `false_count == hit_count`, `runs_satisfied == 0`, and
`runs_hit == 0 <=> hit_count == 0`. Every valid aggregate record MUST map to
exactly one verdict from `{passed, failed}`. Invalid boolean or non-boolean
shapes are outside contract.

#### Scenario: Always fails on one false

- GIVEN an `always` assertion with at least one false evaluation
- WHEN verdict is computed
- THEN the verdict is `failed`

#### Scenario: Unreachable passes when unhit

- GIVEN an `unreachable` assertion with zero hits
- WHEN verdict is computed
- THEN the verdict is `passed`

#### Scenario: Invalid non-boolean record is outside contract

- GIVEN a `reachable` or `unreachable` aggregate with `hit_count == 0` but `runs_hit > 0`
- WHEN verdict evaluation is considered
- THEN that record is outside the valid input contract

### Requirement: Oracle Run Accounting
r[chaoscontrol-oracle.accounting]

Aggregate accounting MUST preserve exact count invariants.
`runs_satisfied <= runs_hit` and `runs_hit == 0 <=> hit_count == 0` hold for
all valid records. For boolean kinds, `runs_hit` counts runs in which the
assertion was evaluated at least once, `runs_hit <= hit_count`, and
`hit_count == true_count + false_count`. For `always`, `runs_satisfied` counts
hit runs with no false evaluation. For `sometimes`, `runs_satisfied` counts hit
runs with at least one true evaluation and valid records satisfy
`runs_satisfied <= true_count`. For non-boolean kinds, valid records are exact: `reachable` requires
`true_count == hit_count`, `false_count == 0`, and `runs_satisfied == runs_hit`;
`unreachable` requires `true_count == 0`, `false_count == hit_count`, and
`runs_satisfied == 0`.

#### Scenario: Satisfied runs are subset of hit runs

- GIVEN any valid aggregate record
- WHEN run counters are inspected
- THEN `runs_satisfied <= runs_hit`

#### Scenario: Boolean hit counts split into true and false

- GIVEN an `always` or `sometimes` record
- WHEN aggregate counters are inspected
- THEN `hit_count == true_count + false_count`
- AND `runs_hit` counts runs where the assertion was evaluated at least once

#### Scenario: Boolean satisfied-run counts follow kind semantics

- GIVEN an `always` or `sometimes` record
- WHEN aggregate counters are inspected
- THEN `runs_satisfied` counts hit runs satisfying that kind's pass condition
- AND `runs_hit <= hit_count`
- AND `sometimes` records satisfy `runs_satisfied <= true_count`

#### Scenario: Non-boolean records use canonical shapes

- GIVEN a valid `reachable` or `unreachable` record
- WHEN aggregate counters are inspected
- THEN the corresponding exact non-boolean shape holds
