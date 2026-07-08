# Consensus Primitives Specification

## Purpose

Defines the mathematical primitives underlying Raft consensus: quorum
arithmetic, monotonic advancement, append-only logs with the Log Matching
Property, and lexicographic epoch comparison. These are protocol-independent
building blocks — they apply to any majority-based consensus system.

## Requirements

### Requirement: Majority Threshold

The majority function MUST return the smallest count that constitutes a
strict majority of `n` members. For any `n > 0`, `majority(n)` MUST
equal `n / 2 + 1`. `majority(1)` MUST be 1. `majority(2)` MUST be 2.

#### Scenario: Odd cluster

- GIVEN a cluster of 5 nodes
- WHEN majority is computed
- THEN the result is 3

#### Scenario: Even cluster

- GIVEN a cluster of 4 nodes
- WHEN majority is computed
- THEN the result is 3

### Requirement: Quorum Predicate

`is_quorum(count, total)` MUST return true if and only if `count >= majority(total)`.
A quorum with zero total members MUST be false. A quorum requires `total > 0`.

### Requirement: Quorum Intersection

If `is_quorum(a, n)` and `is_quorum(b, n)` both hold, then `a + b > n`
MUST hold. Two quorums of the same set MUST always share at least one
member. This is the pigeonhole argument.

#### Scenario: Typical overlap

- GIVEN a cluster of 5 nodes
- WHEN two independent quorums of 3 are formed
- THEN they share at least 1 member (3 + 3 = 6 > 5)

### Requirement: Monotonic Advance

`is_advance(old, new)` MUST return true if and only if `new >= old`.
Advance MUST be reflexive (x advances to x), transitive (a ≤ b ≤ c
implies a ≤ c), and antisymmetric (a ≤ b and b ≤ a implies a == b).

### Requirement: Strict Advance

`is_strict_advance(old, new)` MUST return true if and only if `new > old`.
Strict advance MUST be irreflexive and transitive.

### Requirement: Epoch Comparison

An epoch is a (term, index) pair. `epoch_cmp(a, b)` MUST compare epochs
lexicographically: first by term, then by index within the same term.
The ordering MUST be total, transitive, and antisymmetric.

#### Scenario: Different terms

- GIVEN epoch (3, 5) and epoch (4, 0)
- WHEN compared
- THEN (4, 0) is greater (higher term wins regardless of index)

#### Scenario: Same term

- GIVEN epoch (3, 5) and epoch (3, 8)
- WHEN compared
- THEN (3, 8) is greater (same term, higher index wins)

### Requirement: Epoch At-Least-As-Up-To-Date

`epoch_ge(a, b)` MUST return true if and only if epoch `a` is greater
than or equal to epoch `b` in the lexicographic ordering. This is the
predicate Raft uses for vote granting: a candidate's log must be at
least as up-to-date as the voter's.

### Requirement: Log Entry Agreement

For two logs, `agree_at(a, b, i)` MUST be the predicate that both have an entry
at index `i` and the entries have the same term. Agreement does not require
value equality — only term equality.

### Requirement: Log Prefix Matching

If two logs agree at every index in `[0, k]`, they MUST have the same
entries (same terms) at all those indices. `logs_match_prefix(a, b, k)`
MUST hold when `agree_at(a, b, i)` holds for all `0 <= i <= k`.

### Requirement: Log Append

Appending an entry to a log MUST increase its length by 1. The appended
entry MUST appear at the new last index. All prior entries MUST be
unchanged.

### Requirement: Log Truncate

Truncating a log to length `k` MUST discard all entries at index `k`
and beyond. The first `k` entries MUST be unchanged. The resulting
length MUST be `k`.

### Requirement: QuePaxa finite model boundary
r[consensus.quepaxa.boundary]
Trellis MUST model QuePaxa only as finite deterministic crash-stop verified logic over replica/proposer metadata, proposal priorities, quorum summaries, logical steps, phase guards, and schedule admission. The model MUST state explicit non-claims for real network liveness, probabilistic termination, random-number quality, content-oblivious adversary enforcement, TLS secrecy, storage durability, batching, benchmarks, adaptive multi-armed-bandit optimality, production SMR behavior, and Byzantine fault tolerance.

#### Scenario: Boundary is explicit
r[consensus.quepaxa.boundary.explicit]
- GIVEN the QuePaxa module documentation and requirements
- WHEN a downstream consumer inspects the verified claim boundary
- THEN the documentation MUST state that Trellis verifies only finite deterministic local safety/admission predicates and MUST NOT claim production QuePaxa liveness or performance.

#### Scenario: Overclaim metadata is rejected
r[consensus.quepaxa.boundary.overclaim]
- GIVEN QuePaxa metadata that claims benchmark throughput, probabilistic liveness, network DoS tolerance, or production SMR certification from the Trellis finite model alone
- WHEN boundary validation is evaluated
- THEN validation MUST reject the metadata with deterministic overclaim diagnostics.

### Requirement: QuePaxa proposal ordering
r[consensus.quepaxa.proposal-order]
Trellis MUST provide finite QuePaxa proposal metadata and a deterministic total order over proposals using priority first, proposer identity second, and value identity last. The proposal-order helpers MUST expose named protocol constants for nil proposals, phase roles, initial round, phase count, and reserved leader priority. Leader-priority admission MUST allow the reserved priority only for the declared leader in an admitted first-round fast-path context.

#### Scenario: Highest proposal is selected deterministically
r[consensus.quepaxa.proposal-order.highest]
- GIVEN a finite non-empty set of QuePaxa proposals with distinct ordering keys
- WHEN the best-proposal helper is evaluated
- THEN it MUST return the proposal with the maximum priority/proposer/value ordering key deterministically.

#### Scenario: Non-leader reserved priority is rejected
r[consensus.quepaxa.proposal-order.reserved-priority]
- GIVEN a proposal carrying the reserved leader priority from a proposer that is not the admitted first-round leader
- WHEN leader-priority admission is evaluated
- THEN admission MUST reject the proposal before fast-path decision metadata is exposed.

#### Scenario: Tie policy is explicit
r[consensus.quepaxa.proposal-order.tie-policy]
- GIVEN two proposals with the same priority
- WHEN proposal ordering is evaluated
- THEN proposer identity and value identity MUST break the tie deterministically or an explicit tie/conflict result MUST be returned according to the implemented tie policy.

### Requirement: QuePaxa interval summary register
r[consensus.quepaxa.isr]
Trellis MUST provide QuePaxa interval summary register transition helpers whose logical step never decreases, whose current-step first value is captured once, whose current-step aggregate is the maximum admitted proposal summary for that step, whose returned prior aggregate summarizes the immediately previous step, whose stale updates are discarded, and whose skipped-step advancement resets the prior aggregate to nil.

#### Scenario: Same-step update preserves first value and updates aggregate
r[consensus.quepaxa.isr.same-step]
- GIVEN an ISR state at a logical step with an existing first value and aggregate
- WHEN another admitted proposal for the same step is recorded
- THEN the step and first value MUST remain unchanged and the current aggregate MUST become the deterministic maximum of the old aggregate and the new proposal.

#### Scenario: Step advance returns prior aggregate
r[consensus.quepaxa.isr.advance]
- GIVEN an ISR state at a logical step with a current aggregate
- WHEN an admitted proposal for the immediately next step is recorded
- THEN the ISR step MUST advance, the new first value MUST be the new proposal, and the returned prior aggregate MUST equal the previous step aggregate.

#### Scenario: Stale update is discarded
r[consensus.quepaxa.isr.stale]
- GIVEN an ISR state at a logical step
- WHEN a proposal for an older logical step is recorded
- THEN the ISR state MUST remain unchanged except for returning the current summary response.

#### Scenario: Skipped step resets prior aggregate
r[consensus.quepaxa.isr.skip]
- GIVEN an ISR state at a logical step
- WHEN a proposal advances the ISR by more than one logical step
- THEN the ISR MUST set the new first/current aggregate from the new proposal and MUST return nil for the skipped immediately-prior aggregate.

### Requirement: QuePaxa abstract propagation relation
r[consensus.quepaxa.tcast]
Trellis MUST provide finite abstract threshold-broadcast propagation predicates for QuePaxa rounds. An admitted propagation relation MUST require every returned proposal set used for decision to be backed by a quorum-sized source set, and MUST establish that every modeled universal proposal set is a subset of every modeled common proposal set, and every modeled common proposal set is a subset of every modeled existent proposal set.

#### Scenario: Universal common existent subsets hold
r[consensus.quepaxa.tcast.subsets]
- GIVEN admitted finite QuePaxa propagation metadata for a round
- WHEN the universal, common, and existent proposal sets are inspected for any live proposers
- THEN every universal set MUST be a subset of every common set and every common set MUST be a subset of every existent set.

#### Scenario: Missing quorum is rejected
r[consensus.quepaxa.tcast.missing-quorum]
- GIVEN propagation metadata whose source sets do not meet the declared quorum threshold
- WHEN abstract propagation admission is evaluated
- THEN admission MUST reject the metadata before any decision guard can succeed.

### Requirement: QuePaxa abstract round decision
r[consensus.quepaxa.abstract-round]
Trellis MUST provide abstract QuePaxa round helpers that choose the next candidate value from the best common proposal and admit a decision only when the best existent proposal equals the best universal proposal under the deterministic proposal order. The abstract helpers MUST prove validity for proposed values, agreement for any two admitted decisions in the same finite slot under the modeled guard, and integrity by rejecting a second decision after a decided flag is set.

#### Scenario: Decision guard admits matching existent and universal best
r[consensus.quepaxa.abstract-round.decision]
- GIVEN admitted propagation metadata where the best existent proposal equals the best universal proposal
- WHEN the abstract decision helper is evaluated
- THEN it MUST return that proposal value as the decision and mark the slot decided.

#### Scenario: Divergent best proposals do not decide
r[consensus.quepaxa.abstract-round.no-decision]
- GIVEN admitted propagation metadata where the best existent proposal differs from the best universal proposal
- WHEN the abstract decision helper is evaluated
- THEN it MUST return no decision and MUST carry forward the best common proposal value as the next candidate.

#### Scenario: Second decision is rejected
r[consensus.quepaxa.abstract-round.integrity]
- GIVEN a finite slot state already marked decided
- WHEN a later abstract round attempts to decide the same slot again
- THEN the helper MUST reject the second decision attempt deterministically.

### Requirement: QuePaxa concrete phase admission
r[consensus.quepaxa.concrete-phases]
Trellis MUST provide concrete QuePaxa phase/round admission helpers using named phase constants for proposal, spread, gather/decision, and carry-forward phases. Concrete helpers MUST admit only monotone logical-step advancement, MUST allow proposer catch-up only to a recorder-observed higher step with the recorder first value as proposal template, and MUST expose a phase-2 asynchronous decision only when it refines the abstract decision guard.

#### Scenario: Catch-up advances monotonically
r[consensus.quepaxa.concrete-phases.catch-up]
- GIVEN a proposer step and a recorder reply from a strictly higher logical step
- WHEN catch-up admission is evaluated
- THEN the proposer step MUST advance to the recorder step and the proposal template MUST become the recorder first value.

#### Scenario: Recorder cannot force backward step
r[consensus.quepaxa.concrete-phases.no-backward]
- GIVEN a proposer step and a recorder reply from a lower logical step
- WHEN concrete phase admission is evaluated
- THEN the reply MUST NOT cause the proposer step to decrease.

#### Scenario: Phase-2 decision refines abstract guard
r[consensus.quepaxa.concrete-phases.phase2-refines]
- GIVEN concrete phase-2 replies whose prior-step aggregates identify the same best proposal as the proposer's spread proposal
- WHEN the phase-2 decision helper succeeds
- THEN the corresponding abstract decision guard MUST also hold for the modeled existent and universal proposals.

### Requirement: QuePaxa leader fast path
r[consensus.quepaxa.fast-path]
Trellis MUST provide first-round QuePaxa leader fast-path helpers that admit a decision only when the declared leader's reserved-priority proposal is first at a quorum of recorders in the initial round. A successful fast-path decision MUST refine the abstract decision boundary for the same value. Non-leader, non-initial-round, missing-quorum, or non-unique reserved-priority cases MUST be rejected.

#### Scenario: Initial leader quorum decides fast path
r[consensus.quepaxa.fast-path.leader-quorum]
- GIVEN an admitted initial-round leader proposal carrying the reserved leader priority and first recorded at a quorum of recorders
- WHEN fast-path decision admission is evaluated
- THEN it MUST decide the leader proposal value and expose a proof/refinement link to the abstract decision boundary.

#### Scenario: Fast path after initial round is rejected
r[consensus.quepaxa.fast-path.not-initial]
- GIVEN a reserved-priority proposal in a round later than the initial round
- WHEN fast-path admission is evaluated
- THEN admission MUST reject the fast path and require the asynchronous decision path.

#### Scenario: Non-unique reserved priority is rejected
r[consensus.quepaxa.fast-path.non-unique]
- GIVEN more than one distinct proposal carrying the reserved leader priority in a modeled fast-path quorum
- WHEN fast-path admission is evaluated
- THEN admission MUST reject the fast path before any decision value is exposed.

### Requirement: QuePaxa hedging schedule admission
r[consensus.quepaxa.hedging]
Trellis MUST provide finite QuePaxa hedging-schedule admission over proposer identifiers and activation-delay metadata. An admitted schedule MUST place the declared leader first with zero activation delay when a leader is configured, MUST order entries by non-decreasing delay, and MUST include at least the declared crash-fault budget plus one proposer. Hedging admission MUST NOT claim real-time recovery, probabilistic liveness, or adaptive multi-armed-bandit optimality.

#### Scenario: Leader-first non-decreasing schedule is admitted
r[consensus.quepaxa.hedging.valid]
- GIVEN a finite hedging schedule with the declared leader first, zero leader delay, non-decreasing later delays, and enough proposers for the declared crash-fault budget
- WHEN hedging admission is evaluated
- THEN the schedule MUST be accepted as finite activation metadata.

#### Scenario: Unordered schedule is rejected
r[consensus.quepaxa.hedging.unordered]
- GIVEN a hedging schedule where a later entry has a smaller activation delay than an earlier entry
- WHEN hedging admission is evaluated
- THEN admission MUST reject the schedule with deterministic ordering diagnostics.

#### Scenario: Insufficient schedule is rejected
r[consensus.quepaxa.hedging.insufficient]
- GIVEN a hedging schedule with no more proposers than the declared crash-fault budget
- WHEN hedging admission is evaluated
- THEN admission MUST reject the schedule as insufficient for crash-fault activation coverage.

### Requirement: QuePaxa positive and negative fixtures
r[consensus.quepaxa.fixtures]
Trellis MUST include positive and negative tests or fixtures covering QuePaxa proposal ordering, boundary overclaims, ISR transitions, abstract propagation, abstract decisions, concrete phase admission, fast-path admission, and hedging schedule admission.

#### Scenario: Positive fixtures cover admitted paths
r[consensus.quepaxa.fixtures.positive]
- GIVEN valid finite QuePaxa proposal, ISR, propagation, decision, phase, fast-path, and hedging inputs
- WHEN the QuePaxa fixture suite runs
- THEN each admitted path MUST return deterministic success metadata and preserve the modeled proof boundary.

#### Scenario: Negative fixtures cover rejection paths
r[consensus.quepaxa.fixtures.negative]
- GIVEN malformed QuePaxa inputs including stale ISR updates, missing quorums, conflicting reserved priorities, invalid phase transitions, second decisions, unordered hedging schedules, insufficient schedules, and overclaim metadata
- WHEN the QuePaxa fixture suite runs
- THEN each malformed input MUST fail closed with deterministic diagnostics and MUST NOT expose a decision claim.
