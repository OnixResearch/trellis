## Context

Jetpack composes a one-round-trip fast path with an existing crash-fault consensus path. A client sends the same command to both paths and may return after the fast path obtains a superquorum plus promises from every active original-path proposer. The original path eventually orders the command and handles conflicts or fast-path failure.

The hard proof obligation is not ordinary Raft safety. A promise made by the old proposer may disappear after an election. Safety requires both:

1. every fast-committed command from the last normal view remains recoverable; and
2. each recovered command is committed before conflicting uncommitted work in the new view.

Trellis has useful ingredients but not this composition theorem. `quorum.rs` proves majority count intersection but not the stronger finite-set recovery-support bound. `view_change.rs` proves view monotonicity but not recovery priority. `paxos.rs` proves acceptor promise monotonicity but not the cross-path proposer promise. `client_session.rs` proves duplicate classification but not convergence of two consensus paths. `consistency_history.rs`, `consistency_relations.rs`, and `consistency_object.rs` provide the correct shared history vocabulary for a bounded linearization result.

Reference cohort:

- Paper: `Jetpack: Consensus Made Generally Fast`, OSDI 2026.
- Artifact: `stonysystems/jetpack` commit `c03e318ec355b11edd42aac56c68d0765f88d1d2`, MIT licensed.
- External TLA+ specifications and benchmark reports are design and regression references only.

## Goals / Non-Goals

**Goals:**
- Prove reusable finite crash-fault fast-quorum, certificate, ordering, commutativity, recovery, deduplication, and two-operation history lemmas.
- Give downstream consumers precise assumptions they must discharge for an exact base engine and application command domain.
- Use actual finite acknowledgement membership and uniqueness rather than accepting quorum booleans as proof of intersection.
- Provide direct `proof fn` artifacts for every promised theorem, plus positive and negative runtime/property evidence.

**Non-Goals:**
- No full Jetpack network protocol or production runtime.
- No proof of arbitrary conflict classifiers, application semantics, external TLA+ models, live Raft, transport, persistence, clocks, process failure detection, or whole-system linearizability.
- No liveness, throughput, latency, adaptive-policy optimality, dynamic membership, Byzantine tolerance, interactive transactions, or cross-group atomicity claim.

## Decisions

### 1. Prove a generic fast-path composition boundary, not Jetpack code

**Choice:** Name the module family `consensus_fast_path` and bind Jetpack as a pinned external reference. Independently express Trellis specs, exec refinements, proofs, and fixtures.

**Rationale:** Superquorum recovery, same-view certificates, order preservation, conflict contracts, and recovery priority are reusable beyond one implementation. External source identity supports review but does not transfer proof.

### 2. Use finite acknowledgement vectors with uniqueness

**Choice:** Model replica membership and acknowledgement certificates as exact-length finite sequences or bit vectors with explicit replica identity, uniqueness, command identity, acceleration view, base view, and proposer-promise fields. Define executable count/admission helpers that refine those structures.

**Rationale:** Count-only arithmetic can show that overlap must exist but cannot identify which stored acknowledgements recovery observes. A finite membership model supports a real intersection and recovery-support theorem.

### 3. Derive the fast quorum and recovery threshold from the fault budget

**Choice:** For the initial profile, require `n = 2f + 1`, original recovery majority `f + 1`, fast quorum `f + ceil(f/2) + 1`, and recovery support `ceil(f/2) + 1`, with overflow-safe named helpers. Prove bounds, intersection, threshold sharpness, and `n - fast_quorum = floor(f/2)`.

**Rationale:** These equations explain both recoverability and the availability cost. For three replicas the fast quorum is all replicas; for five replicas it is four.

### 4. Make same-view certificate validity structural

**Choice:** A valid certificate contains unique acknowledgements for one command under one acceleration view and one base view whose numeric identifiers are equal at fast commit, reaches the fast quorum, and carries promises from every active original-path proposer. Mixed-view, cross-path view mismatch, mixed-command, duplicate-replica, stale-view, or incomplete-proposer certificates fail closed.

**Rationale:** Numeric union across views or commitment while the two paths disagree on the active view is the exact recovery hazard. Making view synchronization and proposer scope part of certificate validity prevents later proofs from assuming them implicitly.

### 5. Isolate the base-engine ordering prerequisite

**Choice:** Represent receive, proposal, log, and execution positions explicitly. Prove that receive-order preservation into proposal order plus proposal-order preservation into conflicting-command execution order yields the promised execution order. A buffering base must move the acknowledgement boundary to proposal-order evidence; an engine that can reorder conflicting proposals at execution is incompatible.

**Rationale:** This captures the paper's two base-protocol prerequisites as a reusable theorem rather than a Raft-specific comment.

### 6. Separate generic conflict law from concrete classifier refinements

**Choice:** Define non-conflict as a contract over pure command application and response functions: swapping two non-conflicting commands must preserve final state and each command's response. Provide a conservative finite effect classifier and at least one small key/value refinement proof. Downstream applications must prove their classifier refines the generic contract for their command domain.

**Rationale:** State commutativity alone is insufficient when client responses differ by order. False positives safely force fallback; false negatives are safety failures and cannot be proved away generically.

### 7. Prove recovery as an ordering barrier

**Choice:** Model independent acceleration/base views, last normal view, a possible or accepted recovery set, recovery coordinator phase, marker state, and new-view admission. Prove that a fast certificate remains visible above the recovery-support threshold, accepted recovery state carries forward across coordinator failure, and no normal new-view command is admitted until the recovery set or explicit empty marker commits through the base order.

**Rationale:** Recoverability without marker priority still allows a stale conflicting command to execute first.

### 8. Reuse canonical history and session modules for composition

**Choice:** Use `consistency_history::HistoryOp` and `consistency_relations` for real-time/conflict ordering. Compose with `client_session` for at-most-once behavior and with `noop_commit`/log-order primitives for empty recovery boundaries. The deduplication record must remain valid until original-path convergence or recovery completes; expiry before that boundary rejects the at-most-once theorem's premises. Add cross-module proof functions in the established composition surface.

**Rationale:** Repo-local guidance requires consistency work to use the shared object-history vocabulary. Reuse also keeps claims aligned with existing verified primitives, while an explicit retention premise avoids silently assuming session state survives an unbounded convergence delay.

### 9. End at a bounded two-operation theorem

**Choice:** Prove that, under a valid fast certificate, sound conflict contract, compatible base ordering, valid recovery barrier, and canonical operation identity, a conflicting operation invoked after the fast response cannot execute before the fast-committed operation. Prove that non-conflicting adjacent operations may swap only under the full state-and-response commutativity contract.

**Rationale:** This is a meaningful mechanically checked composition result. A whole distributed trace theorem would require concrete network, durable state, failure, and implementation refinement that Trellis does not own.

## Functional Core / Imperative Shell

- Pure verified core: quorum equations, finite certificate membership/counting, certificate admission, order relations, conflict contract, recovery transitions, marker admission, deduplication composition, history predicates, and proof lemmas.
- Thin shell: optional runtime constructors/checkers for finite metadata and fixture generation. No sockets, files, clocks, processes, storage, RNG, external model checker, or benchmark runner enters the verified core.

## Proof Portfolio

### Family A: Quorum and certificate arithmetic

- Fast quorum and recovery-support bounds.
- Fast quorum intersects every recovery majority in at least the support threshold.
- One fewer fast acknowledgement cannot guarantee that threshold.
- Two valid conflicting same-view certificates cannot coexist when a replica acknowledges at most one conflicting command per view.
- Three-replica and five-replica availability corollaries.

### Family B: Base ordering compatibility

- Receive-before plus PR2 implies propose-before.
- Propose-before plus PR1 implies execute-before for conflicting commands.
- Their transitive composition establishes the proposer promise.
- Counterexample fixtures demonstrate why buffering and execution reordering invalidate the transparent acknowledgement boundary.

### Family C: Conflict semantics

- Non-conflict implies final-state commutativity.
- Non-conflict implies both response-stability equations.
- Conservative conflict classification cannot create a fast-path safety violation.
- A concrete classifier refinement discharges the generic law for a bounded key/value command model.

### Family D: View recovery

- Same-view certificate implies recoverability from any majority response set.
- Accepted recovery set is stable under coordinator/view replacement.
- Recovery/no-op marker is required before normal admission.
- Recovered conflicting command precedes every new-view conflicting command.

### Family E: Cross-module composition

- Same session/sequence/command identity makes the second path completion a duplicate after the first application.
- Fast response followed by a conflicting invocation preserves real-time order in the bounded sequential witness.
- Non-conflicting swap preserves state and responses only under the application contract.

## Risks / Trade-offs

- **Equivalent-gap risk:** A theorem that assumes “recovery is safe” would merely restate the target. Require smaller premises—certificate intersection, accepted-set carry-forward, marker commitment, and order positions—and prove the composition.
- **Weak-certificate risk:** Boolean quorum facts are too weak for recovery claims. Require finite membership and uniqueness evidence.
- **Conflict overclaim:** Generic proofs cannot certify arbitrary application classifiers. Export the law and concrete refinement pattern, and label downstream discharge as mandatory.
- **State-space scope:** Verus proofs are symbolic over finite structures but do not model network schedules. Keep ChaosControl/TLA+ fault exploration complementary rather than calling it redundant.
- **Proof-surface growth:** Decompose into quorum/certificate, ordering/conflict, recovery, and composition modules instead of one large transition predicate.
- **Reference drift:** Pin revision, license, and source identity in docs/profile metadata; external proof and benchmark claims remain non-authoritative.

## Validation Plan

1. Establish the current focused test and Verus baseline before source changes.
2. Add canonical `docs/requirements.md` anchors and public-module Tracey implementation/verification coverage.
3. Run positive and negative focused Rust/property tests for every public admission path.
4. Run full-library `./scripts/verus.sh src/lib.rs` and require zero errors.
5. Run Tracey uncovered/untested checks, module/readme/proof-evidence drift checks, Cairn validation/gates, and the smallest relevant Octet/Nix verification rails.
6. Export proof evidence with exact source, requirements, verifier, assumptions, and non-claims before sync/archive.
