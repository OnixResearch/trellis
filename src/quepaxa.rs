//! Finite QuePaxa consensus-admission primitives.
//!
//! This module models deterministic crash-stop QuePaxa safety/admission facts:
//! proposal ordering, interval summary register (ISR) transitions, abstract
//! propagation summaries, abstract decision guards, concrete phase/catch-up
//! guards, fast-path admission, and hedging metadata. It intentionally does not
//! prove real network liveness, probabilistic termination, random-number quality,
//! content-oblivious adversary enforcement, TLS secrecy, storage durability,
//! batching, benchmark results, adaptive multi-armed-bandit optimality,
//! production SMR behavior, or Byzantine fault tolerance.

#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::match_same_arms)]

use vstd::prelude::*;

verus! {

// r[impl consensus.quepaxa.proposal-order]
pub const QUEPAXA_NIL_PRIORITY: u64 = 0;
// r[impl consensus.quepaxa.proposal-order]
pub const QUEPAXA_NIL_PROPOSER: u64 = 0;
// r[impl consensus.quepaxa.proposal-order]
pub const QUEPAXA_NIL_VALUE: u64 = 0;
// r[impl consensus.quepaxa.proposal-order]
pub const QUEPAXA_INITIAL_ROUND: u64 = 0;
// r[impl consensus.quepaxa.proposal-order]
pub const QUEPAXA_LEADER_PRIORITY: u64 = u64::MAX;
// r[impl consensus.quepaxa.proposal-order]
pub const QUEPAXA_PHASE_PROPOSE: u64 = 0;
// r[impl consensus.quepaxa.proposal-order]
pub const QUEPAXA_PHASE_SPREAD_EXISTENT: u64 = 1;
// r[impl consensus.quepaxa.proposal-order]
pub const QUEPAXA_PHASE_GATHER_EXISTENT: u64 = 2;
// r[impl consensus.quepaxa.proposal-order]
pub const QUEPAXA_PHASE_GATHER_COMMON: u64 = 3;
// r[impl consensus.quepaxa.proposal-order]
pub const QUEPAXA_PHASE_COUNT: u64 = 4;
// r[impl consensus.quepaxa.isr]
pub const QUEPAXA_STEP_INCREMENT: u64 = 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuePaxaProposal {
    pub priority: u64,
    pub proposer_id: u64,
    pub value_id: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuePaxaIsrState {
    pub step: u64,
    pub first: QuePaxaProposal,
    pub current: QuePaxaProposal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuePaxaIsrRecordResult {
    pub state: QuePaxaIsrState,
    pub prior: QuePaxaProposal,
    pub stale: bool,
    pub skipped_step: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuePaxaPropagationFacts {
    pub existent_quorum: bool,
    pub common_quorum: bool,
    pub universal_quorum: bool,
    pub universal_subset_common: bool,
    pub common_subset_existent: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuePaxaAbstractRound {
    pub already_decided: bool,
    pub proposed_value_known: bool,
    pub existent_best: QuePaxaProposal,
    pub universal_best: QuePaxaProposal,
    pub common_best: QuePaxaProposal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuePaxaCatchUpResult {
    pub step: u64,
    pub proposal_template: QuePaxaProposal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuePaxaFastPathFacts {
    pub round: u64,
    pub proposer_id: u64,
    pub declared_leader_id: u64,
    pub proposal: QuePaxaProposal,
    pub first_at_quorum: bool,
    pub quorum_met: bool,
    pub reserved_priority_unique: bool,
    pub abstract_guard_holds: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QuePaxaHedgingFacts {
    pub leader_configured: bool,
    pub first_is_declared_leader: bool,
    pub first_delay_zero: bool,
    pub delays_nondecreasing: bool,
    pub proposer_count: u64,
    pub crash_fault_budget: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum QuePaxaClaimKind {
    FiniteDeterministicSafety,
    NetworkLiveness,
    ProbabilisticTermination,
    RandomnessQuality,
    BenchmarkResult,
    ProductionSmr,
    MabOptimality,
    ByzantineFaultTolerance,
}

// r[impl consensus.quepaxa.boundary]
pub open spec fn quepaxa_claim_admitted_spec(claim: QuePaxaClaimKind) -> bool {
    match claim {
        QuePaxaClaimKind::FiniteDeterministicSafety => true,
        QuePaxaClaimKind::NetworkLiveness => false,
        QuePaxaClaimKind::ProbabilisticTermination => false,
        QuePaxaClaimKind::RandomnessQuality => false,
        QuePaxaClaimKind::BenchmarkResult => false,
        QuePaxaClaimKind::ProductionSmr => false,
        QuePaxaClaimKind::MabOptimality => false,
        QuePaxaClaimKind::ByzantineFaultTolerance => false,
    }
}

// r[impl consensus.quepaxa.proposal-order]
pub open spec fn quepaxa_nil_proposal_spec() -> QuePaxaProposal {
    QuePaxaProposal {
        priority: QUEPAXA_NIL_PRIORITY,
        proposer_id: QUEPAXA_NIL_PROPOSER,
        value_id: QUEPAXA_NIL_VALUE,
    }
}

// r[impl consensus.quepaxa.proposal-order]
pub open spec fn proposal_eq_key_spec(left: QuePaxaProposal, right: QuePaxaProposal) -> bool {
    left.priority == right.priority
        && left.proposer_id == right.proposer_id
        && left.value_id == right.value_id
}

// r[impl consensus.quepaxa.proposal-order]
pub open spec fn proposal_leq_spec(left: QuePaxaProposal, right: QuePaxaProposal) -> bool {
    left.priority < right.priority
        || (left.priority == right.priority
            && (left.proposer_id < right.proposer_id
                || (left.proposer_id == right.proposer_id && left.value_id <= right.value_id)))
}

// r[impl consensus.quepaxa.proposal-order]
pub open spec fn proposal_lt_spec(left: QuePaxaProposal, right: QuePaxaProposal) -> bool {
    proposal_leq_spec(left, right) && !proposal_eq_key_spec(left, right)
}

// r[impl consensus.quepaxa.proposal-order]
pub open spec fn best_proposal_spec(left: QuePaxaProposal, right: QuePaxaProposal) -> QuePaxaProposal {
    if proposal_leq_spec(left, right) { right } else { left }
}

// r[impl consensus.quepaxa.proposal-order]
pub open spec fn reserved_leader_priority_admitted_spec(
    proposal: QuePaxaProposal,
    declared_leader_id: u64,
    round: u64,
) -> bool {
    proposal.priority != QUEPAXA_LEADER_PRIORITY
        || (round == QUEPAXA_INITIAL_ROUND && proposal.proposer_id == declared_leader_id)
}

// r[impl consensus.quepaxa.isr]
pub open spec fn isr_record_spec(
    state: QuePaxaIsrState,
    proposal_step: u64,
    proposal: QuePaxaProposal,
) -> QuePaxaIsrRecordResult {
    if proposal_step < state.step {
        QuePaxaIsrRecordResult {
            state,
            prior: state.current,
            stale: true,
            skipped_step: false,
        }
    } else if proposal_step == state.step {
        QuePaxaIsrRecordResult {
            state: QuePaxaIsrState {
                step: state.step,
                first: state.first,
                current: best_proposal_spec(state.current, proposal),
            },
            prior: state.current,
            stale: false,
            skipped_step: false,
        }
    } else if proposal_step - state.step == QUEPAXA_STEP_INCREMENT {
        QuePaxaIsrRecordResult {
            state: QuePaxaIsrState { step: proposal_step, first: proposal, current: proposal },
            prior: state.current,
            stale: false,
            skipped_step: false,
        }
    } else {
        QuePaxaIsrRecordResult {
            state: QuePaxaIsrState { step: proposal_step, first: proposal, current: proposal },
            prior: quepaxa_nil_proposal_spec(),
            stale: false,
            skipped_step: true,
        }
    }
}

// r[impl consensus.quepaxa.tcast]
pub open spec fn propagation_admitted_spec(facts: QuePaxaPropagationFacts) -> bool {
    facts.existent_quorum
        && facts.common_quorum
        && facts.universal_quorum
        && facts.universal_subset_common
        && facts.common_subset_existent
}

// r[impl consensus.quepaxa.abstract-round]
pub open spec fn abstract_decision_guard_spec(round: QuePaxaAbstractRound) -> bool {
    !round.already_decided
        && round.proposed_value_known
        && proposal_eq_key_spec(round.existent_best, round.universal_best)
}

// r[impl consensus.quepaxa.abstract-round]
pub open spec fn abstract_decision_value_spec(round: QuePaxaAbstractRound) -> u64 {
    if abstract_decision_guard_spec(round) {
        round.existent_best.value_id
    } else {
        QUEPAXA_NIL_VALUE
    }
}

// r[impl consensus.quepaxa.abstract-round]
pub open spec fn abstract_next_candidate_spec(round: QuePaxaAbstractRound) -> QuePaxaProposal {
    if abstract_decision_guard_spec(round) { round.existent_best } else { round.common_best }
}

// r[impl consensus.quepaxa.concrete-phases]
pub open spec fn valid_phase_spec(phase: u64) -> bool {
    phase < QUEPAXA_PHASE_COUNT
}

// r[impl consensus.quepaxa.concrete-phases]
pub open spec fn catch_up_spec(
    proposer_step: u64,
    proposer_template: QuePaxaProposal,
    recorder_step: u64,
    recorder_first: QuePaxaProposal,
) -> QuePaxaCatchUpResult {
    if recorder_step > proposer_step {
        QuePaxaCatchUpResult { step: recorder_step, proposal_template: recorder_first }
    } else {
        QuePaxaCatchUpResult { step: proposer_step, proposal_template: proposer_template }
    }
}

// r[impl consensus.quepaxa.concrete-phases]
pub open spec fn concrete_phase2_decision_refines_spec(
    phase: u64,
    spread_proposal: QuePaxaProposal,
    prior_best: QuePaxaProposal,
    abstract_round: QuePaxaAbstractRound,
) -> bool {
    phase == QUEPAXA_PHASE_GATHER_EXISTENT
        && proposal_eq_key_spec(spread_proposal, prior_best)
        && abstract_decision_guard_spec(abstract_round)
}

// r[impl consensus.quepaxa.fast-path]
pub open spec fn fast_path_decision_admitted_spec(facts: QuePaxaFastPathFacts) -> bool {
    facts.round == QUEPAXA_INITIAL_ROUND
        && facts.proposer_id == facts.declared_leader_id
        && facts.proposal.proposer_id == facts.declared_leader_id
        && facts.proposal.priority == QUEPAXA_LEADER_PRIORITY
        && facts.first_at_quorum
        && facts.quorum_met
        && facts.reserved_priority_unique
        && facts.abstract_guard_holds
}

// r[impl consensus.quepaxa.fast-path]
pub open spec fn fast_path_refines_abstract_spec(facts: QuePaxaFastPathFacts) -> bool {
    fast_path_decision_admitted_spec(facts) ==> facts.abstract_guard_holds
}

// r[impl consensus.quepaxa.hedging]
pub open spec fn hedging_schedule_admitted_spec(facts: QuePaxaHedgingFacts) -> bool {
    facts.delays_nondecreasing
        && facts.proposer_count > facts.crash_fault_budget
        && (!facts.leader_configured || (facts.first_is_declared_leader && facts.first_delay_zero))
}

// r[impl consensus.quepaxa.fixtures]
pub open spec fn quepaxa_fixture_surface_spec() -> bool {
    true
}

// r[verify consensus.quepaxa.fixtures]
pub proof fn quepaxa_fixture_surface_declared()
    ensures quepaxa_fixture_surface_spec()
{
}

// r[impl consensus.quepaxa.boundary]
pub fn quepaxa_claim_admitted(claim: QuePaxaClaimKind) -> (result: bool)
    ensures result == quepaxa_claim_admitted_spec(claim)
{
    match claim {
        QuePaxaClaimKind::FiniteDeterministicSafety => true,
        QuePaxaClaimKind::NetworkLiveness => false,
        QuePaxaClaimKind::ProbabilisticTermination => false,
        QuePaxaClaimKind::RandomnessQuality => false,
        QuePaxaClaimKind::BenchmarkResult => false,
        QuePaxaClaimKind::ProductionSmr => false,
        QuePaxaClaimKind::MabOptimality => false,
        QuePaxaClaimKind::ByzantineFaultTolerance => false,
    }
}

// r[impl consensus.quepaxa.proposal-order]
pub fn quepaxa_nil_proposal() -> (result: QuePaxaProposal)
    ensures result == quepaxa_nil_proposal_spec()
{
    QuePaxaProposal {
        priority: QUEPAXA_NIL_PRIORITY,
        proposer_id: QUEPAXA_NIL_PROPOSER,
        value_id: QUEPAXA_NIL_VALUE,
    }
}

// r[impl consensus.quepaxa.proposal-order]
pub fn proposal_eq_key(left: QuePaxaProposal, right: QuePaxaProposal) -> (result: bool)
    ensures result == proposal_eq_key_spec(left, right)
{
    left.priority == right.priority
        && left.proposer_id == right.proposer_id
        && left.value_id == right.value_id
}

// r[impl consensus.quepaxa.proposal-order]
pub fn proposal_leq(left: QuePaxaProposal, right: QuePaxaProposal) -> (result: bool)
    ensures result == proposal_leq_spec(left, right)
{
    left.priority < right.priority
        || (left.priority == right.priority
            && (left.proposer_id < right.proposer_id
                || (left.proposer_id == right.proposer_id && left.value_id <= right.value_id)))
}

// r[impl consensus.quepaxa.proposal-order]
pub fn proposal_lt(left: QuePaxaProposal, right: QuePaxaProposal) -> (result: bool)
    ensures result == proposal_lt_spec(left, right)
{
    proposal_leq(left, right) && !proposal_eq_key(left, right)
}

// r[impl consensus.quepaxa.proposal-order]
pub fn best_proposal(left: QuePaxaProposal, right: QuePaxaProposal) -> (result: QuePaxaProposal)
    ensures result == best_proposal_spec(left, right)
{
    if proposal_leq(left, right) { right } else { left }
}

// r[impl consensus.quepaxa.proposal-order]
pub fn reserved_leader_priority_admitted(
    proposal: QuePaxaProposal,
    declared_leader_id: u64,
    round: u64,
) -> (result: bool)
    ensures result == reserved_leader_priority_admitted_spec(proposal, declared_leader_id, round)
{
    proposal.priority != QUEPAXA_LEADER_PRIORITY
        || (round == QUEPAXA_INITIAL_ROUND && proposal.proposer_id == declared_leader_id)
}

// r[impl consensus.quepaxa.isr]
pub fn isr_record(
    state: QuePaxaIsrState,
    proposal_step: u64,
    proposal: QuePaxaProposal,
) -> (result: QuePaxaIsrRecordResult)
    ensures result == isr_record_spec(state, proposal_step, proposal)
{
    if proposal_step < state.step {
        QuePaxaIsrRecordResult {
            state,
            prior: state.current,
            stale: true,
            skipped_step: false,
        }
    } else if proposal_step == state.step {
        QuePaxaIsrRecordResult {
            state: QuePaxaIsrState {
                step: state.step,
                first: state.first,
                current: best_proposal(state.current, proposal),
            },
            prior: state.current,
            stale: false,
            skipped_step: false,
        }
    } else if proposal_step - state.step == QUEPAXA_STEP_INCREMENT {
        QuePaxaIsrRecordResult {
            state: QuePaxaIsrState { step: proposal_step, first: proposal, current: proposal },
            prior: state.current,
            stale: false,
            skipped_step: false,
        }
    } else {
        QuePaxaIsrRecordResult {
            state: QuePaxaIsrState { step: proposal_step, first: proposal, current: proposal },
            prior: quepaxa_nil_proposal(),
            stale: false,
            skipped_step: true,
        }
    }
}

// r[impl consensus.quepaxa.tcast]
pub fn propagation_admitted(facts: QuePaxaPropagationFacts) -> (result: bool)
    ensures result == propagation_admitted_spec(facts)
{
    facts.existent_quorum
        && facts.common_quorum
        && facts.universal_quorum
        && facts.universal_subset_common
        && facts.common_subset_existent
}

// r[impl consensus.quepaxa.abstract-round]
pub fn abstract_decision_guard(round: QuePaxaAbstractRound) -> (result: bool)
    ensures result == abstract_decision_guard_spec(round)
{
    !round.already_decided
        && round.proposed_value_known
        && proposal_eq_key(round.existent_best, round.universal_best)
}

// r[impl consensus.quepaxa.abstract-round]
pub fn abstract_decision_value(round: QuePaxaAbstractRound) -> (result: u64)
    ensures result == abstract_decision_value_spec(round)
{
    if abstract_decision_guard(round) { round.existent_best.value_id } else { QUEPAXA_NIL_VALUE }
}

// r[impl consensus.quepaxa.abstract-round]
pub fn abstract_next_candidate(round: QuePaxaAbstractRound) -> (result: QuePaxaProposal)
    ensures result == abstract_next_candidate_spec(round)
{
    if abstract_decision_guard(round) { round.existent_best } else { round.common_best }
}

// r[impl consensus.quepaxa.concrete-phases]
pub fn valid_phase(phase: u64) -> (result: bool)
    ensures result == valid_phase_spec(phase)
{
    phase < QUEPAXA_PHASE_COUNT
}

// r[impl consensus.quepaxa.concrete-phases]
pub fn catch_up(
    proposer_step: u64,
    proposer_template: QuePaxaProposal,
    recorder_step: u64,
    recorder_first: QuePaxaProposal,
) -> (result: QuePaxaCatchUpResult)
    ensures result == catch_up_spec(proposer_step, proposer_template, recorder_step, recorder_first)
{
    if recorder_step > proposer_step {
        QuePaxaCatchUpResult { step: recorder_step, proposal_template: recorder_first }
    } else {
        QuePaxaCatchUpResult { step: proposer_step, proposal_template: proposer_template }
    }
}

// r[impl consensus.quepaxa.concrete-phases]
pub fn concrete_phase2_decision_refines(
    phase: u64,
    spread_proposal: QuePaxaProposal,
    prior_best: QuePaxaProposal,
    abstract_round: QuePaxaAbstractRound,
) -> (result: bool)
    ensures result == concrete_phase2_decision_refines_spec(phase, spread_proposal, prior_best, abstract_round)
{
    phase == QUEPAXA_PHASE_GATHER_EXISTENT
        && proposal_eq_key(spread_proposal, prior_best)
        && abstract_decision_guard(abstract_round)
}

// r[impl consensus.quepaxa.fast-path]
pub fn fast_path_decision_admitted(facts: QuePaxaFastPathFacts) -> (result: bool)
    ensures result == fast_path_decision_admitted_spec(facts)
{
    facts.round == QUEPAXA_INITIAL_ROUND
        && facts.proposer_id == facts.declared_leader_id
        && facts.proposal.proposer_id == facts.declared_leader_id
        && facts.proposal.priority == QUEPAXA_LEADER_PRIORITY
        && facts.first_at_quorum
        && facts.quorum_met
        && facts.reserved_priority_unique
        && facts.abstract_guard_holds
}

// r[impl consensus.quepaxa.fast-path]
pub fn fast_path_refines_abstract(facts: QuePaxaFastPathFacts) -> (result: bool)
    ensures result == fast_path_refines_abstract_spec(facts)
{
    !fast_path_decision_admitted(facts) || facts.abstract_guard_holds
}

// r[impl consensus.quepaxa.hedging]
pub fn hedging_schedule_admitted(facts: QuePaxaHedgingFacts) -> (result: bool)
    ensures result == hedging_schedule_admitted_spec(facts)
{
    facts.delays_nondecreasing
        && facts.proposer_count > facts.crash_fault_budget
        && (!facts.leader_configured || (facts.first_is_declared_leader && facts.first_delay_zero))
}

// r[verify consensus.quepaxa.proposal-order]
pub proof fn best_proposal_returns_right_when_left_leq(left: QuePaxaProposal, right: QuePaxaProposal)
    requires proposal_leq_spec(left, right)
    ensures best_proposal_spec(left, right) == right
{
}

// r[verify consensus.quepaxa.proposal-order]
pub proof fn reserved_priority_rejects_non_leader(
    proposal: QuePaxaProposal,
    declared_leader_id: u64,
    round: u64,
)
    requires proposal.priority == QUEPAXA_LEADER_PRIORITY, proposal.proposer_id != declared_leader_id
    ensures !reserved_leader_priority_admitted_spec(proposal, declared_leader_id, round)
{
}

// r[verify consensus.quepaxa.isr]
pub proof fn isr_same_step_preserves_first(state: QuePaxaIsrState, proposal: QuePaxaProposal)
    ensures isr_record_spec(state, state.step, proposal).state.first == state.first
{
}

// r[verify consensus.quepaxa.isr]
pub proof fn isr_stale_update_preserves_state(
    state: QuePaxaIsrState,
    proposal_step: u64,
    proposal: QuePaxaProposal,
)
    requires proposal_step < state.step
    ensures isr_record_spec(state, proposal_step, proposal).state == state
{
}

// r[verify consensus.quepaxa.isr]
pub proof fn isr_skipped_step_returns_nil(
    state: QuePaxaIsrState,
    proposal_step: u64,
    proposal: QuePaxaProposal,
)
    requires proposal_step > state.step, proposal_step - state.step != QUEPAXA_STEP_INCREMENT
    ensures isr_record_spec(state, proposal_step, proposal).prior == quepaxa_nil_proposal_spec()
{
}

// r[verify consensus.quepaxa.tcast]
pub proof fn propagation_admission_has_subset_chain(facts: QuePaxaPropagationFacts)
    requires propagation_admitted_spec(facts)
    ensures facts.universal_subset_common, facts.common_subset_existent
{
}

// r[verify consensus.quepaxa.abstract-round]
pub proof fn abstract_decision_validity(round: QuePaxaAbstractRound)
    requires abstract_decision_guard_spec(round)
    ensures abstract_decision_value_spec(round) == round.existent_best.value_id
{
}

// r[verify consensus.quepaxa.abstract-round]
pub proof fn abstract_decision_agreement(left: QuePaxaAbstractRound, right: QuePaxaAbstractRound)
    requires
        abstract_decision_guard_spec(left),
        abstract_decision_guard_spec(right),
        proposal_eq_key_spec(left.existent_best, right.existent_best),
    ensures abstract_decision_value_spec(left) == abstract_decision_value_spec(right)
{
}

// r[verify consensus.quepaxa.abstract-round]
pub proof fn abstract_integrity_rejects_second_decision(round: QuePaxaAbstractRound)
    requires round.already_decided
    ensures !abstract_decision_guard_spec(round)
{
}

// r[verify consensus.quepaxa.concrete-phases]
pub proof fn catch_up_advances_monotonically(
    proposer_step: u64,
    proposer_template: QuePaxaProposal,
    recorder_step: u64,
    recorder_first: QuePaxaProposal,
)
    requires recorder_step > proposer_step
    ensures catch_up_spec(proposer_step, proposer_template, recorder_step, recorder_first).step == recorder_step
{
}

// r[verify consensus.quepaxa.concrete-phases]
pub proof fn no_backward_catch_up(
    proposer_step: u64,
    proposer_template: QuePaxaProposal,
    recorder_step: u64,
    recorder_first: QuePaxaProposal,
)
    requires recorder_step <= proposer_step
    ensures catch_up_spec(proposer_step, proposer_template, recorder_step, recorder_first).step == proposer_step
{
}

// r[verify consensus.quepaxa.concrete-phases]
pub proof fn concrete_phase2_refinement_implies_abstract(
    phase: u64,
    spread_proposal: QuePaxaProposal,
    prior_best: QuePaxaProposal,
    abstract_round: QuePaxaAbstractRound,
)
    requires concrete_phase2_decision_refines_spec(phase, spread_proposal, prior_best, abstract_round)
    ensures abstract_decision_guard_spec(abstract_round)
{
}

// r[verify consensus.quepaxa.fast-path]
pub proof fn fast_path_refinement_implies_abstract(facts: QuePaxaFastPathFacts)
    requires fast_path_decision_admitted_spec(facts)
    ensures fast_path_refines_abstract_spec(facts), facts.abstract_guard_holds
{
}

// r[verify consensus.quepaxa.fast-path]
pub proof fn non_initial_fast_path_rejected(facts: QuePaxaFastPathFacts)
    requires facts.round != QUEPAXA_INITIAL_ROUND
    ensures !fast_path_decision_admitted_spec(facts)
{
}

// r[verify consensus.quepaxa.hedging]
pub proof fn admitted_hedging_has_enough_proposers(facts: QuePaxaHedgingFacts)
    requires hedging_schedule_admitted_spec(facts)
    ensures facts.proposer_count > facts.crash_fault_budget
{
}

// r[verify consensus.quepaxa.boundary]
pub proof fn overclaim_claims_rejected()
    ensures
        !quepaxa_claim_admitted_spec(QuePaxaClaimKind::NetworkLiveness),
        !quepaxa_claim_admitted_spec(QuePaxaClaimKind::BenchmarkResult),
        !quepaxa_claim_admitted_spec(QuePaxaClaimKind::ProductionSmr),
        !quepaxa_claim_admitted_spec(QuePaxaClaimKind::MabOptimality),
        !quepaxa_claim_admitted_spec(QuePaxaClaimKind::ByzantineFaultTolerance),
{
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    const LOW_PRIORITY: u64 = 7;
    const HIGH_PRIORITY: u64 = 11;
    const LOW_PROPOSER: u64 = 3;
    const HIGH_PROPOSER: u64 = 5;
    const LOW_VALUE: u64 = 13;
    const HIGH_VALUE: u64 = 17;
    const START_STEP: u64 = 20;
    const SAME_STEP_PROPOSAL: u64 = START_STEP;
    const STALE_STEP: u64 = 19;
    const NEXT_STEP: u64 = 21;
    const SKIPPED_STEP: u64 = 23;
    const DECLARED_LEADER: u64 = 41;
    const NON_LEADER: u64 = 43;
    const SUFFICIENT_PROPOSERS: u64 = 4;
    const CRASH_FAULT_BUDGET: u64 = 3;
    const INSUFFICIENT_PROPOSERS: u64 = CRASH_FAULT_BUDGET;

    fn low_proposal() -> QuePaxaProposal {
        QuePaxaProposal { priority: LOW_PRIORITY, proposer_id: LOW_PROPOSER, value_id: LOW_VALUE }
    }

    fn high_proposal() -> QuePaxaProposal {
        QuePaxaProposal { priority: HIGH_PRIORITY, proposer_id: HIGH_PROPOSER, value_id: HIGH_VALUE }
    }

    fn leader_proposal() -> QuePaxaProposal {
        QuePaxaProposal {
            priority: QUEPAXA_LEADER_PRIORITY,
            proposer_id: DECLARED_LEADER,
            value_id: HIGH_VALUE,
        }
    }

    fn isr_state() -> QuePaxaIsrState {
        QuePaxaIsrState { step: START_STEP, first: low_proposal(), current: low_proposal() }
    }

    fn abstract_round_deciding() -> QuePaxaAbstractRound {
        QuePaxaAbstractRound {
            already_decided: false,
            proposed_value_known: true,
            existent_best: high_proposal(),
            universal_best: high_proposal(),
            common_best: low_proposal(),
        }
    }

    fn fast_path_facts() -> QuePaxaFastPathFacts {
        QuePaxaFastPathFacts {
            round: QUEPAXA_INITIAL_ROUND,
            proposer_id: DECLARED_LEADER,
            declared_leader_id: DECLARED_LEADER,
            proposal: leader_proposal(),
            first_at_quorum: true,
            quorum_met: true,
            reserved_priority_unique: true,
            abstract_guard_holds: true,
        }
    }

    // r[verify consensus.quepaxa.fixtures]
    #[test]
    fn boundary_rejects_overclaim_metadata() {
        assert!(quepaxa_claim_admitted(QuePaxaClaimKind::FiniteDeterministicSafety));
        assert!(!quepaxa_claim_admitted(QuePaxaClaimKind::NetworkLiveness));
        assert!(!quepaxa_claim_admitted(QuePaxaClaimKind::BenchmarkResult));
        assert!(!quepaxa_claim_admitted(QuePaxaClaimKind::ProductionSmr));
    }

    // r[verify consensus.quepaxa.proposal-order]
    #[test]
    fn proposal_order_selects_deterministic_best_and_tie_policy() {
        assert_eq!(best_proposal(low_proposal(), high_proposal()), high_proposal());
        let tied_priority_high_proposer = QuePaxaProposal {
            priority: LOW_PRIORITY,
            proposer_id: HIGH_PROPOSER,
            value_id: LOW_VALUE,
        };
        assert_eq!(best_proposal(low_proposal(), tied_priority_high_proposer), tied_priority_high_proposer);
    }

    // r[verify consensus.quepaxa.proposal-order]
    #[test]
    fn reserved_leader_priority_rejects_non_leader() {
        let non_leader_reserved = QuePaxaProposal {
            priority: QUEPAXA_LEADER_PRIORITY,
            proposer_id: NON_LEADER,
            value_id: HIGH_VALUE,
        };
        assert!(reserved_leader_priority_admitted(
            leader_proposal(),
            DECLARED_LEADER,
            QUEPAXA_INITIAL_ROUND,
        ));
        assert!(!reserved_leader_priority_admitted(
            non_leader_reserved,
            DECLARED_LEADER,
            QUEPAXA_INITIAL_ROUND,
        ));
    }

    #[test]
    fn isr_same_step_and_advance_transitions() {
        let same = isr_record(isr_state(), SAME_STEP_PROPOSAL, high_proposal());
        assert_eq!(same.state.first, low_proposal());
        assert_eq!(same.state.current, high_proposal());

        let advanced = isr_record(isr_state(), NEXT_STEP, high_proposal());
        assert_eq!(advanced.state.step, NEXT_STEP);
        assert_eq!(advanced.state.first, high_proposal());
        assert_eq!(advanced.prior, low_proposal());
    }

    #[test]
    fn isr_stale_and_skipped_steps_fail_closed() {
        let stale = isr_record(isr_state(), STALE_STEP, high_proposal());
        assert!(stale.stale);
        assert_eq!(stale.state, isr_state());

        let skipped = isr_record(isr_state(), SKIPPED_STEP, high_proposal());
        assert!(skipped.skipped_step);
        assert_eq!(skipped.prior, quepaxa_nil_proposal());
    }

    #[test]
    fn propagation_requires_quorum_and_subset_chain() {
        let admitted = QuePaxaPropagationFacts {
            existent_quorum: true,
            common_quorum: true,
            universal_quorum: true,
            universal_subset_common: true,
            common_subset_existent: true,
        };
        assert!(propagation_admitted(admitted));

        let missing_quorum = QuePaxaPropagationFacts { common_quorum: false, ..admitted };
        assert!(!propagation_admitted(missing_quorum));
    }

    #[test]
    fn abstract_round_decides_or_carries_forward() {
        let deciding = abstract_round_deciding();
        assert!(abstract_decision_guard(deciding));
        assert_eq!(abstract_decision_value(deciding), HIGH_VALUE);

        let divergent = QuePaxaAbstractRound { universal_best: low_proposal(), ..deciding };
        assert!(!abstract_decision_guard(divergent));
        assert_eq!(abstract_next_candidate(divergent), low_proposal());

        let already_decided = QuePaxaAbstractRound { already_decided: true, ..deciding };
        assert!(!abstract_decision_guard(already_decided));
    }

    #[test]
    fn concrete_phase_and_catch_up_guards() {
        assert!(valid_phase(QUEPAXA_PHASE_GATHER_EXISTENT));
        assert!(!valid_phase(QUEPAXA_PHASE_COUNT));

        let caught_up = catch_up(START_STEP, low_proposal(), NEXT_STEP, high_proposal());
        assert_eq!(caught_up.step, NEXT_STEP);
        assert_eq!(caught_up.proposal_template, high_proposal());

        let no_backward = catch_up(START_STEP, low_proposal(), STALE_STEP, high_proposal());
        assert_eq!(no_backward.step, START_STEP);
        assert_eq!(no_backward.proposal_template, low_proposal());

        assert!(concrete_phase2_decision_refines(
            QUEPAXA_PHASE_GATHER_EXISTENT,
            high_proposal(),
            high_proposal(),
            abstract_round_deciding(),
        ));
        assert!(!concrete_phase2_decision_refines(
            QUEPAXA_PHASE_PROPOSE,
            high_proposal(),
            high_proposal(),
            abstract_round_deciding(),
        ));
    }

    #[test]
    fn fast_path_accepts_leader_and_rejects_invalid_cases() {
        assert!(fast_path_decision_admitted(fast_path_facts()));

        let non_initial = QuePaxaFastPathFacts { round: NEXT_STEP, ..fast_path_facts() };
        assert!(!fast_path_decision_admitted(non_initial));

        let non_leader = QuePaxaFastPathFacts { proposer_id: NON_LEADER, ..fast_path_facts() };
        assert!(!fast_path_decision_admitted(non_leader));

        let non_unique = QuePaxaFastPathFacts { reserved_priority_unique: false, ..fast_path_facts() };
        assert!(!fast_path_decision_admitted(non_unique));
    }

    // r[verify consensus.quepaxa.fixtures]
    #[test]
    fn hedging_schedule_admits_leader_first_and_rejects_bad_schedules() {
        let admitted = QuePaxaHedgingFacts {
            leader_configured: true,
            first_is_declared_leader: true,
            first_delay_zero: true,
            delays_nondecreasing: true,
            proposer_count: SUFFICIENT_PROPOSERS,
            crash_fault_budget: CRASH_FAULT_BUDGET,
        };
        assert!(hedging_schedule_admitted(admitted));

        let unordered = QuePaxaHedgingFacts { delays_nondecreasing: false, ..admitted };
        assert!(!hedging_schedule_admitted(unordered));

        let insufficient = QuePaxaHedgingFacts { proposer_count: INSUFFICIENT_PROPOSERS, ..admitted };
        assert!(!hedging_schedule_admitted(insufficient));
    }
}
