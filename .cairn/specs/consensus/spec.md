# Consensus Primitives Specification

## Purpose

Defines the mathematical primitives underlying Raft consensus: quorum
arithmetic, monotonic advancement, append-only logs with the Log Matching
Property, and lexicographic epoch comparison. These are protocol-independent
building blocks — they apply to any majority-based consensus system.

## Requirements

### Requirement: Majority Threshold
r[arithmetic.abs-diff]

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
r[quorum.is-quorum]

`is_quorum(count, total)` MUST return true if and only if `count >= majority(total)`.
A quorum with zero total members MUST be false. A quorum requires `total > 0`.

### Requirement: Quorum Intersection
r[quorum.intersection]

If `is_quorum(a, n)` and `is_quorum(b, n)` both hold, then `a + b > n`
MUST hold. Two quorums of the same set MUST always share at least one
member. This is the pigeonhole argument.

#### Scenario: Typical overlap

- GIVEN a cluster of 5 nodes
- WHEN two independent quorums of 3 are formed
- THEN they share at least 1 member (3 + 3 = 6 > 5)

### Requirement: Monotonic Advance
r[monotonic.is-advance]

`is_advance(old, new)` MUST return true if and only if `new >= old`.
Advance MUST be reflexive (x advances to x), transitive (a ≤ b ≤ c
implies a ≤ c), and antisymmetric (a ≤ b and b ≤ a implies a == b).

### Requirement: Strict Advance
r[monotonic.is-strict-advance]

`is_strict_advance(old, new)` MUST return true if and only if `new > old`.
Strict advance MUST be irreflexive and transitive.

### Requirement: Epoch Comparison
r[epoch.cmp]

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
r[paxos.ballot-ge]

`epoch_ge(a, b)` MUST return true if and only if epoch `a` is greater
than or equal to epoch `b` in the lexicographic ordering. This is the
predicate Raft uses for vote granting: a candidate's log must be at
least as up-to-date as the voter's.

### Requirement: Log Entry Agreement
r[log.agree-at]

For two logs, `agree_at(a, b, i)` MUST be the predicate that both have an entry
at index `i` and the entries have the same term. Agreement does not require
value equality — only term equality.

### Requirement: Log Prefix Matching
r[log.prefix-matching]

If two logs agree at every index in `[0, k]`, they MUST have the same
entries (same terms) at all those indices. `logs_match_prefix(a, b, k)`
MUST hold when `agree_at(a, b, i)` holds for all `0 <= i <= k`.

### Requirement: Log Append
r[log.append]

Appending an entry to a log MUST increase its length by 1. The appended
entry MUST appear at the new last index. All prior entries MUST be
unchanged.

### Requirement: Log Truncate
r[log.truncate]

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

### Requirement: Consensus fast-path proof boundary
r[consensus.fast-path.boundary]
Trellis MUST expose consensus fast-path work only as finite deterministic crash-fault arithmetic, certificate, order, conflict, recovery, deduplication, history, and refinement proofs under named assumptions. The proof boundary MUST pin external reference identities and MUST NOT claim correctness of external code or models, arbitrary application classifiers, live consensus, network liveness, storage durability, dynamic membership, Byzantine tolerance, interactive transactions, latency, throughput, adaptive-policy optimality, or production readiness.

#### Scenario: External model checking does not transfer proof
r[consensus.fast-path.boundary.external-model]
- GIVEN a pinned external artifact reports passing TLA+ model checks and geo-distributed benchmarks
- WHEN Trellis proof admission evaluates that evidence without local proof artifacts
- THEN it MUST classify the evidence as external reference material
- AND MUST NOT claim Trellis, downstream implementation, or performance correctness.

#### Scenario: Unsupported capability is rejected
r[consensus.fast-path.boundary.unsupported]
- GIVEN metadata claiming dynamic-membership, Byzantine, interactive-transaction, or cross-group atomicity support from the initial proof slice
- WHEN boundary admission is evaluated
- THEN it MUST reject the stronger claim deterministically.

### Requirement: Fast quorum arithmetic and recovery support
r[consensus.fast-path.quorum]
For a declared crash-fault budget `f` with `n = 2f + 1`, Trellis MUST provide overflow-safe named helpers for original majority `f + 1`, fast quorum `f + ceil(f/2) + 1`, recovery support `ceil(f/2) + 1`, and fast-path failure tolerance `n - fast_quorum`. Direct proof functions MUST establish that the fast quorum is bounded by `n`, every fast quorum intersects every original recovery majority in at least the recovery-support threshold, one fewer fast acknowledgement cannot guarantee that threshold, and the fast-path failure tolerance equals `floor(f/2)`.

#### Scenario: Three replicas require every fast acknowledgement
r[consensus.fast-path.quorum.three]
- GIVEN `f = 1` and `n = 3`
- WHEN fast quorum and failure tolerance are derived
- THEN the fast quorum MUST be `3`
- AND the fast path MUST tolerate `0` missing replicas while the original majority remains `2`.

#### Scenario: Five replicas recover from majority replies
r[consensus.fast-path.quorum.five]
- GIVEN `f = 2`, `n = 5`, a fast certificate of `4`, and a recovery majority of `3`
- WHEN finite certificate intersection is evaluated
- THEN at least `2` recovery replies MUST contain the fast-certified command.

#### Scenario: Below-fast threshold is insufficient
r[consensus.fast-path.quorum.sharp]
- GIVEN a certificate with one fewer acknowledgement than the derived fast quorum
- WHEN it is intersected with an arbitrary recovery majority
- THEN Trellis MUST NOT claim the full recovery-support lower bound.

### Requirement: Fast certificates are unique, same-view, and proposer-complete
r[consensus.fast-path.certificate]
Trellis MUST model fast acknowledgements with replica identity, command identity, acceleration view, base view, and proposer-promise metadata. A valid certificate MUST contain unique replica acknowledgements, one command identity, one same-view acknowledgement cohort, equal acceleration and base view identifiers at fast commit, the derived fast quorum, and promises from every active original-path proposer. Mixed-view, acceleration/base view mismatch, mixed-command, duplicate-replica, stale-view, or proposer-incomplete certificates MUST reject.

#### Scenario: Same-view certificate is admitted
r[consensus.fast-path.certificate.valid]
- GIVEN unique acknowledgements for one command under one acceleration view and numerically equal base view that meet the fast quorum and cover every active proposer
- WHEN certificate admission is evaluated
- THEN the certificate MUST be admitted
- AND its same-command, same-view, and proposer-coverage facts MUST be available to later proofs.

#### Scenario: Numeric union across views is rejected
r[consensus.fast-path.certificate.mixed-view]
- GIVEN acknowledgements from multiple acceleration or base views whose numeric union meets the fast quorum
- WHEN certificate admission is evaluated
- THEN admission MUST reject the certificate before exposing a fast-commit fact.

#### Scenario: Acceleration and base view mismatch is rejected
r[consensus.fast-path.certificate.cross-path-view]
- GIVEN all acknowledgements name one acceleration view but the bound base view has a different identifier
- WHEN certificate admission is evaluated
- THEN admission MUST reject the certificate before exposing a fast-commit fact.

#### Scenario: Conflicting same-view certificates cannot coexist
r[consensus.fast-path.certificate.conflicting]
- GIVEN two candidate certificates for conflicting commands in one view and a per-replica rule forbidding acknowledgement of both conflicting commands in that view
- WHEN both candidates are assumed valid
- THEN Trellis MUST prove the assumption contradictory through certificate intersection.

### Requirement: Base-engine ordering prerequisites compose
r[consensus.fast-path.base-order]
Trellis MUST represent proposer receive order, proposal/log order, and conflicting-command execution order explicitly. It MUST prove that receive-order preservation into proposal order plus proposal-order preservation into execution order yields the original-path ordering promise. A receive-reordering buffer MUST require proposal-order evidence before fast acknowledgement, and an engine that reorders conflicting proposals at execution MUST reject transparent fast-path compatibility.

#### Scenario: PR2 and PR1 establish execution order
r[consensus.fast-path.base-order.compose]
- GIVEN command A is received before conflicting command B, the proposer preserves receive order in proposal order, and the base engine preserves proposal order in execution order
- WHEN the compatibility theorem is applied
- THEN A MUST execute before B.

#### Scenario: Buffering moves the acknowledgement boundary
r[consensus.fast-path.base-order.buffer]
- GIVEN a proposer may receive A before B but buffer and propose B first
- WHEN no proposal-order evidence exists
- THEN receive-time fast acknowledgement MUST be rejected
- AND an acknowledgement after compatible proposal-order evidence MAY be considered separately.

#### Scenario: Execution reorder is incompatible
r[consensus.fast-path.base-order.execution]
- GIVEN the base engine may execute conflicting B before earlier-proposed A
- WHEN compatibility admission is evaluated
- THEN transparent fast-path compatibility MUST reject.

### Requirement: Non-conflict preserves state and responses
r[consensus.fast-path.conflict]
Trellis MUST define a generic pure application conflict contract under which swapping two non-conflicting commands preserves the final state and preserves each command's response relative to its own execution. A conservative classifier MUST classify unknown or unsupported effects as conflicting. Trellis MUST provide at least one finite concrete classifier refinement and MUST keep arbitrary downstream classifier correctness as an explicit proof obligation.

#### Scenario: Independent commands commute completely
r[consensus.fast-path.conflict.commute]
- GIVEN two commands satisfy the bound non-conflict contract in a starting state
- WHEN they execute in either order
- THEN both orders MUST produce the same final state
- AND each command MUST produce the same response in both orders.

#### Scenario: False positive safely falls back
r[consensus.fast-path.conflict.false-positive]
- GIVEN a conservative classifier marks semantically independent commands as conflicting
- WHEN fast-path admission is evaluated
- THEN fast admission MUST fail or fall back
- AND the classifier decision MUST NOT create a conflicting fast commit.

#### Scenario: False negative violates the contract
r[consensus.fast-path.conflict.false-negative]
- GIVEN a classifier marks commands non-conflicting but a witness state changes final state or either response across execution orders
- WHEN classifier refinement is checked
- THEN refinement MUST fail with the witness
- AND no fast-path safety theorem may consume that classifier as sound.

### Requirement: View recovery is recoverable and ordered
r[consensus.fast-path.recovery]
Trellis MUST model acceleration views independently from base views and MUST provide pure recovery transitions for the last normal view, possible and accepted recovery sets, recovery coordinator replacement, recovery or explicit empty marker commitment, and return to normal admission. Direct proofs MUST establish that a valid fast certificate remains visible above the recovery-support threshold in every recovery majority, accepted recovery state carries forward, and no normal new-view command is admitted before the recovery marker. Recovered commands MUST precede conflicting new-view commands.

#### Scenario: Fast-replied command is recovered after proposer failure
r[consensus.fast-path.recovery.fast-reply]
- GIVEN a valid fast certificate and proposer failure before original-path commitment
- WHEN a recovery majority is collected for the last normal view
- THEN the command MUST meet the recovery-support threshold
- AND the selected recovery set MUST retain it before new-view admission.

#### Scenario: Accepted recovery survives coordinator failure
r[consensus.fast-path.recovery.carry-forward]
- GIVEN a recovery set was accepted but its coordinator failed before the marker committed
- WHEN another coordinator or view resumes recovery
- THEN the accepted set MUST carry forward unchanged
- AND normal admission MUST remain blocked.

#### Scenario: Empty recovery still commits a boundary
r[consensus.fast-path.recovery.empty]
- GIVEN no command reaches the possible-fast-commit recovery threshold
- WHEN recovery completes
- THEN an explicit empty or no-op recovery marker MUST commit before normal new-view work.

#### Scenario: Stale conflicting predecessor is rejected
r[consensus.fast-path.recovery.order]
- GIVEN a candidate new-view order places a stale conflicting command before a recovered fast-certified command
- WHEN recovery-order admission is evaluated
- THEN admission MUST reject that order.

### Requirement: Dual paths converge at most once
r[consensus.fast-path.dual-path]
Trellis MUST compose canonical command identity, client session, sequence metadata, and an explicit deduplication-retention horizon with existing client-session duplicate, expiry, and cached-response primitives. If the fast and original paths carry the same operation identity and the dedup record remains retained until original convergence or recovery completes, applying one completion MUST make the later completion duplicate, prevent a second state-machine application, and preserve the authoritative cached response.

#### Scenario: Original completion after fast application is duplicate
r[consensus.fast-path.dual-path.duplicate]
- GIVEN the fast decision and original path carry the same session, sequence, and command identity, the operation has already been applied once, and its dedup record remains valid through convergence
- WHEN the original path later reaches the operation
- THEN session classification MUST identify it as duplicate
- AND the state machine MUST NOT apply it again.

#### Scenario: Premature dedup expiry blocks the theorem
r[consensus.fast-path.dual-path.retention]
- GIVEN the fast-applied operation's session record may expire before the original path converges or recovery completes
- WHEN at-most-once composition admission is evaluated
- THEN the theorem's premises MUST reject
- AND the runtime MUST require a longer retention boundary or another durable operation-id tombstone.

#### Scenario: Identity mismatch cannot use dedup proof
r[consensus.fast-path.dual-path.identity]
- GIVEN the two paths carry different command, session, or sequence identity
- WHEN convergence admission is evaluated
- THEN the at-most-once composition theorem MUST reject its premises.

### Requirement: Fast-path composition preserves bounded real-time order
r[consensus.fast-path.linearization]
Using the shared `consistency_history` and `consistency_relations` vocabulary, Trellis MUST prove a bounded two-operation composition theorem: under a valid fast certificate, sound conflict contract, compatible base ordering, valid stable-view or recovery-marker order, and matching dual-path identity, a conflicting operation invoked after the fast response cannot execute before the fast-committed operation. An adjacent non-conflicting swap MAY preserve the witness only when the full state-and-response commutativity contract holds.

#### Scenario: Later conflicting invocation remains later
r[consensus.fast-path.linearization.conflicting]
- GIVEN operation A receives a valid fast response before conflicting operation B is invoked
- WHEN A is ordered in the same stable view or recovered through a valid marker before B
- THEN the bounded sequential witness MUST order A before B
- AND preserve their real-time relation.

#### Scenario: Non-conflicting swap requires full contract
r[consensus.fast-path.linearization.non-conflicting]
- GIVEN concurrent operations are proposed in different orders by admissible paths
- WHEN the proof attempts to swap them in a sequential witness
- THEN it MUST require equal final state and stable responses for both operations
- AND state-only commutativity MUST be insufficient.

#### Scenario: Whole-system linearizability is not inferred
r[consensus.fast-path.linearization.nonclaim]
- GIVEN all bounded Trellis fast-path lemmas pass
- WHEN a downstream runtime requests a whole distributed-system linearizability claim
- THEN Trellis evidence alone MUST be insufficient without concrete network, storage, failure, and implementation refinement evidence.

### Requirement: Fast-path proof evidence is explicit
r[consensus.fast-path.evidence]
Trellis MUST export proof evidence binding each fast-path theorem to exact source identity, requirement IDs, verifier/tool identity, assumptions, trusted boundaries, positive and negative tests, and non-claims. External TLA+ or benchmark artifacts MAY be linked as complementary evidence but MUST remain a distinct role.

#### Scenario: Proof bundle separates local and external evidence
r[consensus.fast-path.evidence.roles]
- GIVEN a bundle contains Trellis Verus proofs and a pinned external TLA+ result
- WHEN the bundle is validated
- THEN the local theorem rows MUST identify Trellis source and verifier inputs
- AND the external result MUST be labeled reference or model-checking evidence rather than local proof.

### Requirement: Fast-path proof fixtures cover success and failure
r[consensus.fast-path.fixtures]
Trellis MUST include positive tests, negative tests, property tests, and direct `proof fn` artifacts covering quorum formulas, finite certificate intersection, same-view and proposer admission, base ordering, conflict semantics, recovery carry-forward and markers, dual-path deduplication, bounded history order, evidence roles, unsupported capabilities, and claim boundaries.

#### Scenario: Positive proof corpus verifies
r[consensus.fast-path.fixtures.positive]
- GIVEN valid three-replica and five-replica profiles, certificates, compatible ordering, sound conflicts, recovery transitions, and matching path identities
- WHEN focused tests and full-library Verus verification run
- THEN every promised local theorem MUST verify without new trusted boundaries.

#### Scenario: Negative corpus rejects hazards
r[consensus.fast-path.fixtures.negative]
- GIVEN below-threshold, mixed-view, duplicate-replica, missing-proposer, reordered-base, false-non-conflict, stale-predecessor, interrupted-recovery, identity-mismatch, unsupported-capability, or overclaim fixtures
- WHEN focused validation runs
- THEN each hazard MUST reject or produce the expected counterexample without fabricating a proof result.
