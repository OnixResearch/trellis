# ChaosControl Exploration Heuristics Specification

## Purpose

Defines the pure frontier-selection and input-tree alternative kernels derived
from ChaosControl.

## Requirements

### Requirement: Frontier Effective Score Ordering
r[chaoscontrol-frontier.selection]

Greedy frontier selection is defined on non-empty, stable-order input
sequences. Frontier entries use `u32` `base_score` and `u32`
`times_selected` values. Each frontier input sequence also has a stable
zero-based input index given by its position in that sequence; tie-breaking
refers to that position, not a separate stored field. Effective score is the
exact rational
`base_score / (1u64 + times_selected as u64)`. The pure greedy-selection
helper is defined only for non-empty frontiers. Executable comparison may use
equivalent exact `u64` cross-products instead of floating point, but the exec
ordering MUST match the rational ordering exactly. Those cross-products MUST be
proved overflow-safe: both multiplicands come from `u32`-bounded fields
(`base_score` and `1 + times_selected as u64`), so each product stays below
`u64::MAX`. Effective score MUST not increase as `times_selected` increases.
Greedy selection MUST choose an entry with maximal effective score, with ties
broken by the lowest input index.

#### Scenario: Re-selected entry decays

- GIVEN an entry with base score `s`
- WHEN its selection count increases
- THEN its effective score does not increase

#### Scenario: Greedy choice is maximal

- GIVEN a non-empty frontier
- WHEN greedy selection is applied
- THEN the chosen entry has maximal effective score

#### Scenario: Ties break to lower input index

- GIVEN two entries with equal maximal effective score
- WHEN greedy selection is applied
- THEN the lower input index is chosen first

### Requirement: Input-Tree Alternatives Respect Budget
r[chaoscontrol-input-tree.alternatives]

Input-tree generation operates on normalized tuples
`(vm_id: u32, sequence_id: u64, n_options: u32, original_value: u64)` with
`n_options >= 2`, `original_value < n_options`,
`sequence_id <= u64::MAX - 100`,
`n_options as u64 * (100 + sequence_id) <= u64::MAX`, and unique
`(vm_id, sequence_id)` pairs within a batch. Ordering semantics use the exact
rational score `100 / (n_options * (100 + sequence_id))`, with higher scores
first and ties broken lexicographically by lower `(vm_id, sequence_id)`
(`vm_id` first, then `sequence_id`). Because the numerator is constant, the
executable comparator MUST realize the same order by verified no-overflow `u64`
comparison of the denominators
`n_options * (100 + sequence_id)`, and the exec result MUST match the
mathematical ordering exactly. Each ranked choice contributes the ascending
distinct values from `0 .. n_options` excluding `original_value`, preserving
source identity as `(vm_id, sequence_id, alternative_value)`. If
`full_stream_len` is the sum of those per-choice counts, output MUST be the
exact prefix of that deterministic stream with length
`min(budget, full_stream_len)`.

#### Scenario: Original value is never regenerated

- GIVEN a recorded choice with original value `v`
- WHEN alternatives are generated
- THEN no emitted alternative equals `v`

#### Scenario: Higher score is emitted earlier

- GIVEN two normalized choices with different scores
- WHEN alternatives are generated
- THEN the higher-score choice appears earlier in the deterministic stream

#### Scenario: Equal scores break to lower identity pair

- GIVEN two normalized choices with equal scores
- WHEN alternatives are generated
- THEN the lexicographically lower `(vm_id, sequence_id)` pair appears earlier, comparing `vm_id` first and `sequence_id` second

#### Scenario: Budget uses exact deterministic prefix

- GIVEN budget `b` and deterministic full stream length `full_stream_len`
- WHEN alternatives are generated
- THEN exactly the first `min(b, full_stream_len)` alternatives are emitted
