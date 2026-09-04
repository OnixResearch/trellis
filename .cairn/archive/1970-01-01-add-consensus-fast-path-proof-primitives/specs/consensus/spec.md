## ADDED Requirements

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
