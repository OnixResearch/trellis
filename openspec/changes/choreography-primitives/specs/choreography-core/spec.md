# Choreography Core Specification

## Purpose

Defines the finite global and local choreography primitives, participant /
well-formedness rules, and branch-merge semantics used by endpoint projection.

## Requirements

### Requirement: Participant and Branch Well-Formedness

r[choreography.participants]
Finite choreography primitives operate on exact `u32` role identifiers,
branch labels, and payload tags. A communication prefix is well formed only if
`from != to`. A choice is well formed only if its branch list is non-empty and
its branch labels are pairwise distinct and stored in strict increasing label
order. `participants(choreo)` MUST contain exactly the roles that appear as
communication endpoints or as a branch decider in `choreo`.

#### Scenario: Self-send is rejected

- GIVEN a communication prefix with `from == to`
- WHEN checking global well-formedness
- THEN the choreography MUST be rejected as not well formed

#### Scenario: Duplicate branch labels are rejected

- GIVEN a choice with two branches carrying the same label
- WHEN checking global well-formedness
- THEN the choreography MUST be rejected as not well formed

#### Scenario: Participants are exact

- GIVEN a finite choreography mentioning only roles `1`, `4`, and `9`
- WHEN computing `participants(choreo)`
- THEN the result MUST contain exactly `1`, `4`, and `9`

### Requirement: Communication Prefix Semantics

r[choreography.communication]
A global communication prefix
`Comm { from, to, label, payload_tag, next }` MUST denote one exact
point-to-point interaction from `from` to `to` carrying the exact symbolic
message metadata `(label, payload_tag)` and continuing with the exact stored
continuation `next`. This requirement is about the global syntax node itself;
local projection shape is defined separately by `r[choreography.projection]`.

#### Scenario: Communication metadata is exact

- GIVEN `Comm { from = A, to = B, label = L, payload_tag = P, next }`
- WHEN reading the global prefix fields
- THEN the prefix MUST expose exact `from = A`, `to = B`, `label = L`, and `payload_tag = P`

#### Scenario: Communication continuation is exact

- GIVEN `Comm { from = A, to = B, label = L, payload_tag = P, next }`
- WHEN reading the stored continuation
- THEN the continuation MUST be exactly `next`

### Requirement: Local Branch Merge Semantics

r[choreography.merge]
For a non-decider role, branch projections over exact local nodes
`End`, `Send { peer, label, payload_tag, next }`,
`Recv { peer, label, payload_tag, next }`,
`InternalChoice { branches }`, and `Offer { from, branches }` MUST merge by a
total helper `merge_local(decider, left, right) -> Option<LocalChoreo>`
parameterized by the exact choice decider. The recursive evaluation MUST
terminate by a structurally decreasing measure over remaining local syntax
size, with branch-list folds additionally decreasing on the remaining
normalized suffix length.
`End` merges only with `End`. Identical `Send` heads merge only when `peer`,
`label`, and `payload_tag` are all equal, and the result MUST keep that exact
`Send` head while recursively merging continuations. Identical `Recv` heads
merge only when `peer`, `label`, and `payload_tag` are all equal, and the
result MUST keep that exact `Recv` head while recursively merging
continuations. `InternalChoice` heads merge only when they carry the same
normalized label set; the result MUST be an `InternalChoice` with that same
ordered label set and recursively merged per-label continuations. Distinct
`Recv` heads from the exact `decider` with pairwise distinct labels merge into
one normalized `Offer` from that decider carrying the per-label continuations
in strict increasing label order. `Offer` heads from the same decider merge by
normalized label-set union: shared labels recursively merge, labels present in
only one input pass through unchanged, and the result keeps strict increasing
label order. Every other head conflict, including matching `peer` / `label`
with different `payload_tag` or `Offer` heads with different `from` roles,
MUST fail the merge.

#### Scenario: Identical prefixes merge recursively

- GIVEN two local choreographies with the same `Recv` head and different but
  mergeable continuations
- WHEN merging them
- THEN the result MUST keep that exact `Recv` head
- AND recursively merge the continuations

#### Scenario: Distinct decider receives form an offer

- GIVEN two local choreographies headed by `Recv` from the same decider with
  distinct labels `L1` and `L2`
- WHEN merging them
- THEN the result MUST be an `Offer` from that decider containing `L1` and `L2`

#### Scenario: Internal choices keep exact label set

- GIVEN two `InternalChoice` heads with the same ordered labels and mergeable
  per-label continuations
- WHEN merging them
- THEN the result MUST be an `InternalChoice` with that same exact ordered
  label set
- AND each matching-label continuation MUST be recursively merged

#### Scenario: Offer union preserves shared and unshared labels

- GIVEN two `Offer` heads from the same decider with shared label `L2` and
  unshared labels `L1` and `L3`
- WHEN merging them
- THEN the result MUST expose exactly `L1`, `L2`, and `L3` in strict
  increasing label order
- AND the `L2` continuation MUST be the recursive merge of the two `L2`
  branches
- AND the `L1` and `L3` branches MUST pass through unchanged

#### Scenario: Conflicting heads fail merge

- GIVEN one local choreography headed by `Send`
- AND another headed by `Recv` with a different shape
- WHEN merging them
- THEN the merge MUST fail

### Requirement: Projectability Is Exact

r[choreography.projectability]
A finite choreography MUST be projectable exactly when it is well formed and,
for every choice and every non-decider role appearing in at least one branch,
the role's branch projections admit the exact projection-side merge: preserve
any shared local `Send` / `Recv` prefix common to every branch in stored order,
and when behavior first diverges, expose an `Offer` from the decider keyed by
the stored global branch labels for the remaining branch-local continuations.
Projectability MUST fail on malformed syntax or on any non-mergeable branch
conflict.

#### Scenario: Common continuation is projectable

- GIVEN a well-formed choice whose branches project to the same local behavior
  for one non-decider role
- WHEN checking projectability
- THEN that role MUST not block projectability

#### Scenario: Non-mergeable branch conflict is not projectable

- GIVEN a well-formed choice whose branches project to incompatible local head
  actions for one non-decider role
- WHEN checking projectability
- THEN the choreography MUST be reported as not projectable

#### Scenario: Three-branch projectability preserves stored outer branch labels

- GIVEN a well-formed choice with stored branch labels `1 < 4 < 9`
- AND a non-decider role whose projections require more than one merge step
- WHEN checking projectability
- THEN successful branch projection MUST preserve the exact stored order `1`,
  then `4`, then `9`
- AND the first exposed `Offer`, if one appears, MUST use those stored outer
  branch labels exactly
