## Context

The QuePaxa paper describes a crash-stop consensus protocol with a randomized asynchronous core, a first-round leader fast path, and hedging delays instead of timeout-triggered view changes. The Trellis slice should model only the finite deterministic proof-friendly pieces:

- prioritized proposals ordered by priority, proposer identity, and value;
- the interval summary register (ISR) recorder abstraction from Algorithms 2 and 3;
- abstract threshold-broadcast propagation where universal proposals are subsets of every common set and common proposals are subsets of every existent set;
- the concrete four-phase round structure from Algorithm 4, including phase-2 asynchronous decision and first-round fast-path decision;
- hedging-schedule admission as metadata that controls redundant proposer activation without becoming a liveness proof.

The paper’s probabilistic liveness arguments depend on private randomness and a content-oblivious network adversary. Trellis should not encode those environmental assumptions as verified facts. Instead, the verified surface should state the deterministic safety conditions under which a finite modeled round may decide.

## Goals / Non-Goals

**Goals:**
- Add a finite QuePaxa model with explicit named constants for phase count, phase roles, nil proposal, and reserved leader priority.
- Keep the pure verified core independent of clocks, sockets, storage, RNG, TLS, Promela, benchmark harnesses, and adaptive leader-selection runtime state.
- Prove local safety lemmas over the modeled state: ISR step monotonicity, propagation subset relationships, decision validity, decision agreement under the modeled guard, integrity via a decided flag, fast-path refinement, and catch-up monotonicity.
- Include positive and negative tests for every public admission helper and every modeled rejection path.

**Non-Goals:**
- No production QuePaxa SMR implementation.
- No proof of probabilistic liveness, expected round count, DoS tolerance, network eventual delivery, TLS/content-oblivious adversary assumptions, or benchmark throughput/latency.
- No cryptographic randomness implementation or common-coin protocol.
- No multi-armed-bandit optimality proof; adaptive leader choice may be documented for future work but is not part of this finite slice.
- No Byzantine fault tolerance claim.

## Decisions

### 1. Start with deterministic finite safety primitives

**Choice:** Model QuePaxa as finite metadata and pure predicates/specs before any runtime shell.

**Rationale:** Trellis verifies local deterministic logic. The paper’s strongest Trellis-fit artifacts are the ISR state transition, proposal ordering, phase/round arithmetic, and decision guards. Liveness/performance depend on runtime assumptions and should remain explicit non-claims.

**Alternative:** Implement a full networked QuePaxa prototype. Rejected because it would make the proof surface depend on I/O, clocks, scheduling, RNG, batching, and benchmarks.

### 2. Use concrete proposal ordering instead of implicit randomness

**Choice:** Treat priorities as caller-supplied finite values and define a deterministic total order using `(priority, proposer_id, value_id)` with a reserved leader priority for fast-path rounds.

**Rationale:** Verus can prove ordering and tie behavior over explicit inputs. Randomness quality is outside Trellis; tests can cover tied and non-tied inputs deterministically.

**Alternative:** Attempt to model probabilistic private coins. Rejected because Trellis does not prove probability distributions or content-oblivious scheduling.

### 3. Keep ISR generic enough for reuse, but instantiate QuePaxa with max aggregation

**Choice:** Provide an ISR model whose QuePaxa instantiation uses nil plus max aggregation over ordered proposal IDs.

**Rationale:** The paper identifies ISR as a reusable abstraction while the concrete protocol only needs constant-space integer/max aggregation. A generic spec plus concrete max helper keeps the functional core small and useful.

**Alternative:** Store historical proposal sets directly. Rejected because the paper’s concrete protocol and Trellis style both prefer bounded constant-space summaries where possible.

### 4. Separate abstract propagation from concrete phase simulation

**Choice:** Prove abstract `universal ⊆ common ⊆ existent` relationships and separately prove concrete phase helpers refine those relationships for the modeled phase outputs.

**Rationale:** This mirrors the paper’s proof structure: Appendix B proves abstract QuePaxa safety, and Appendix C shows concrete QuePaxa simulates the abstract protocol. The split keeps proof obligations reviewable.

**Alternative:** Encode the whole concrete algorithm in one large predicate. Rejected because it would blur functional-core boundaries and make proof failures hard to localize.

### 5. Admit hedging schedules, do not prove real-time liveness

**Choice:** A hedging schedule is valid when proposer entries are ordered by non-decreasing activation delay, the configured leader entry has zero delay, and enough entries are present for the declared crash-fault budget.

**Rationale:** This captures the paper’s non-destructive redundant-activation contract without pretending to verify actual time, scheduling, or MAB behavior.

**Alternative:** Model real delays and recovery time. Rejected because clocks and network progress are outside the Trellis finite verified core.

## Risks / Trade-offs

**Spec overclaiming** → Requirement text must repeat the non-claims: no probabilistic liveness, no network, no RNG, no production SMR, no benchmarks, no Byzantine faults.

**Proof size growth** → Keep functions decomposed into proposal ordering, ISR transition, abstract propagation, concrete phase admission, fast-path guard, and hedging admission.

**Tracey drift** → Add Tracey implementation tags to each public helper, verification tags to proof/test coverage, and sync `docs/requirements.md` with the accepted Cairn spec before archive.

**Magic-number drift** → Use named constants in code for protocol values such as `QUEPAXA_PHASE_COUNT`, `QUEPAXA_PHASE_PROPOSE`, `QUEPAXA_PHASE_SPREAD_EXISTENT`, `QUEPAXA_PHASE_GATHER_EXISTENT`, `QUEPAXA_PHASE_GATHER_COMMON`, `QUEPAXA_INITIAL_ROUND`, and `QUEPAXA_LEADER_PRIORITY`.

## Source Notes from the Paper

- Section 4.1 defines abstract QuePaxa over threshold synchronous broadcast; the core safety relationship is that every universal set is contained in every common set, and every common set is contained in every existent set.
- Section 4.2 introduces concrete recorders as interval summary registers that track a monotone logical step, a first value for the current step, and an aggregate for the prior step.
- Algorithm 4 uses four phases per round: proposal/randomization, spread, gather/decision, and next-round carry-forward.
- Section 4.2.5 gives a first-round leader fast path by reserving the highest priority for the designated leader, then falling back to leaderless asynchronous rounds.
- Section 5 replaces timeout-triggered view changes with hedging schedules; hedging controls redundant work and does not decide safety.
- Appendices B and C prove abstract safety and concrete simulation; Appendix D reports bounded Promela/SPIN safety checks, not a probabilistic liveness proof.

## Validation Plan

1. Run Cairn validation and gates for `add-quepaxa-consensus-primitives`.
2. Run focused Rust tests for the new QuePaxa module(s), including positive and negative cases.
3. Run `./scripts/verus.sh src/lib.rs` for full-library Verus evidence.
4. Run Tracey uncovered and untested checks.
5. Run the repo verification wrapper or the smallest accepted final gate for the implemented slice, then sync and archive when tasks are complete.
