## Phase 1: Boundaries, vocabulary, and baseline

- [ ] [serial] r[consensus.fast-path.boundary] Define the finite crash-fault proof boundary, pinned reference cohort, named formulas, supported capabilities, downstream proof obligations, and explicit whole-system/liveness/performance/non-claim matrix.
- [ ] [serial] r[consensus.fast-path.boundary] Run and capture the pre-change focused quorum/view/Paxos/session/consistency tests, full-library Verus result, Tracey status, trusted-boundary inventory, and exact source revision before modifying proof modules.
- [ ] [parallel] r[consensus.fast-path.fixtures] Add positive profile fixtures and negative overflow, malformed-size, unsupported-fault-model, dynamic-membership, Byzantine, transaction, and overclaim fixtures.

## Phase 2: Fast quorum and certificate proofs

- [ ] [serial] r[consensus.fast-path.quorum] Add overflow-safe spec/exec helpers for cluster size, majority, fast quorum, recovery support, and fast-path failure tolerance using named formulas over `f`.
- [ ] [depends:fast-path-quorum-helpers] r[consensus.fast-path.quorum] Add direct `proof fn` artifacts for quorum bounds, fast/recovery intersection lower bound, recovery-support equality, threshold sharpness, and three-replica/five-replica corollaries.
- [ ] [serial] r[consensus.fast-path.certificate] Add finite acknowledgement/certificate structures and spec/exec admission for exact length, unique replicas, one command, same-view acknowledgements, acceleration/base view equality at commit, fast quorum, and all-active-proposer promises.
- [ ] [depends:fast-path-certificate] r[consensus.fast-path.certificate] Prove valid-certificate same-view/same-command facts, mixed-view and duplicate-replica rejection, active-proposer coverage, majority recovery visibility, and no conflicting same-view dual certificate under per-replica non-double-ack.
- [ ] [parallel] r[consensus.fast-path.fixtures] Add positive three-node/five-node certificates and negative below-threshold, mixed-view, mixed-command, duplicate-replica, stale-view, missing-proposer, and conflicting-double-ack fixtures plus randomized membership proptests.

## Phase 3: Base ordering and conflict-contract proofs

- [ ] [serial] r[consensus.fast-path.base-order] Add receive, proposal, log, and execution position predicates plus explicit PR2 and PR1 compatibility admission.
- [ ] [depends:fast-path-base-order] r[consensus.fast-path.base-order] Prove receive-before → propose-before, propose-before → conflicting execute-before, and the transitive proposer-promise theorem; add proposal-bound acknowledgement fallback for reorder-capable buffers.
- [ ] [serial] r[consensus.fast-path.conflict] Define the generic pure application/response commutativity contract and conservative conflict admission over the shared consistency command vocabulary.
- [ ] [depends:fast-path-conflict-contract] r[consensus.fast-path.conflict] Prove state commutativity, both response-stability equations, false-positive fallback safety, and a bounded key/value effect-classifier refinement; keep arbitrary application classifier soundness as an explicit downstream obligation.
- [ ] [parallel] r[consensus.fast-path.fixtures] Add positive ordered-base and independent-command fixtures plus negative receive/propose reorder, proposal/execution reorder, same-key write/read, alias/range unknown, response-dependent, and deliberate false-non-conflict fixtures.

## Phase 4: Recovery and convergence proofs

- [ ] [serial] r[consensus.fast-path.recovery] Add independent acceleration/base view, last-normal-view, recovery-set, accepted-set, marker, and normal-admission state with pure transitions for begin, prepare, accept, marker commit, carry-forward, and finish.
- [ ] [depends:fast-path-recovery-model] r[consensus.fast-path.recovery] Prove valid fast certificate recoverability, accepted-set stability across coordinator failure, empty recovery no-op boundary, marker-before-normal admission, and recovered-command precedence over conflicting new-view work.
- [ ] [serial] r[consensus.fast-path.dual-path] Add canonical command/session/sequence identity and dedup-retention-horizon checks for fast/original convergence and compose them with `client_session` duplicate classification, expiry, and cached response behavior.
- [ ] [depends:fast-path-dual-path] r[consensus.fast-path.dual-path] Prove at-most-once application and stable authoritative response when the original path reaches an operation already applied through the fast decision while its dedup record remains retained.
- [ ] [parallel] r[consensus.fast-path.fixtures] Add leader-failure-after-fast-response, view-straddled ack, acceleration/base view mismatch, stale-conflicting-entry, empty recovery, interrupted recovery, cascading coordinator failure, duplicate completion, premature dedup expiry, and identity-mismatch fixtures.

## Phase 5: Bounded history composition and evidence

- [ ] [serial] r[consensus.fast-path.linearization] Add cross-module proofs over `consistency_history`, `consistency_relations`, `consistency_object`, quorum/certificate, ordering, conflict, recovery, and client-session primitives.
- [ ] [depends:fast-path-history-composition] r[consensus.fast-path.linearization] Prove the bounded two-operation theorem: a conflicting operation invoked after a valid fast response cannot execute before the fast-committed operation in either stable-view or recovered-view cases.
- [ ] [depends:fast-path-history-composition] r[consensus.fast-path.linearization] Prove that an adjacent non-conflicting swap preserves final state and both responses only under the full application commutativity contract.
- [ ] [parallel] r[consensus.fast-path.evidence] Export proof-evidence rows binding each theorem to exact requirement, source, verifier, assumptions, trusted-boundary count, positive/negative tests, and non-claims; external TLA+ evidence remains complementary.
- [ ] [serial] r[consensus.fast-path.boundary] Add the pinned Jetpack reference to README source notes and update module guide/index, invariant catalog, property index, and proof-artifact manifest without transferring external claims.

## Phase 6: Final validation and lifecycle

- [ ] [depends:fast-path-proof-implementation] r[consensus.fast-path.fixtures] Run focused unit/property tests, negative fixtures, coverage and ensures checkers, full-library Verus, Tracey uncovered/untested, proof-gap inventory, module/readme/proof-evidence drift checks, and the smallest relevant Octet/Nix checks.
- [ ] [serial] [depends:fast-path-proof-validation] r[consensus.fast-path.fixtures] Capture non-contradictory validation transcripts, run Cairn validation plus proposal/design/tasks gates, reconcile accepted specs and `docs/requirements.md`, verify clean scoped status, and archive only after every promised `proof fn` exists.

## Current Tooling Blocker

At package creation time, `nix run path:../cairn#cairn -- validate --root .`
is blocked before repository validation by a sibling Cairn compile error in
`cairn-cli`/`ironspec` (`E0505`: `args` is borrowed and then moved). This is a
validation-tool availability blocker, not proof evidence and not permission to
skip final Cairn gates. Preserve the baseline log and rerun all lifecycle gates
after the sibling tool is repaired.
