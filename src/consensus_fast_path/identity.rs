//! Finite crash-fault accelerated consensus proof primitives.
//!
//! This family proves structural quorum, certificate, ordering, conflict,
//! recovery, convergence, and bounded-history lemmas. It does not verify an
//! external protocol, network, storage system, clock, or live runtime.
// r[impl consensus.fast-path.boundary]
// r[impl consensus.fast-path.fixtures]
// r[impl consensus.fast-path.evidence]
// r[impl consensus.fast-path.non-claims]

mod agreement;
mod barrier;
mod membership;
mod proofs;
mod vocabulary;
mod witness;

pub use agreement::apply;
pub use agreement::commute_under_contract;
pub use agreement::conflicts;
pub use agreement::converge;
pub use agreement::generic_commutativity_holds;
pub use agreement::identity_matches;
pub use agreement::refinement_holds;
pub use agreement::ConvergenceFault;
pub use agreement::DedupRecord;
pub use agreement::OriginalCompletion;
pub use agreement::Store;
pub use barrier::admit_recovery_order;
pub use barrier::recovered_precedes_new_view;
pub use barrier::RecoveryFault;
pub use barrier::RecoverySnapshot;
pub use barrier::RecoveryStage;
pub use barrier::MAX_RECOVERY_COMMANDS;
pub use membership::accelerated_failure_tolerance;
pub use membership::accelerated_quorum;
pub use membership::admit_certificate;
pub use membership::certified_command;
pub use membership::cluster_size;
pub use membership::conflicting_certificates_admitted;
pub use membership::intersection_lower_bound;
pub use membership::majority;
pub use membership::recovery_support;
pub use membership::recovery_visibility;
pub use membership::replica_set;
pub use membership::Acknowledgement;
pub use membership::Certificate;
pub use membership::CertificateFault;
pub use membership::CertificateFaultSite;
pub use membership::MAX_FAULTS;
pub use vocabulary::admit_scope;
pub use vocabulary::BaseCapability;
pub use vocabulary::CapabilityScope;
pub use vocabulary::CommandId;
pub use vocabulary::CommandMode;
pub use vocabulary::FaultModel;
pub use vocabulary::KeyValue;
pub use vocabulary::MembershipMode;
pub use vocabulary::Positions;
pub use vocabulary::ProposerId;
pub use vocabulary::ReplicaId;
pub use vocabulary::Response;
pub use vocabulary::ScopeFault;
pub use vocabulary::ViewId;
pub use witness::adjacent_swap_preserves;
pub use witness::admit_composition;
pub use witness::base_admits;
pub use witness::composed_execution_sequence;
pub use witness::execute_before;
pub use witness::log_before;
pub use witness::proposal_bound_ack_valid;
pub use witness::propose_before;
pub use witness::receive_before;
pub use witness::receive_time_ack_valid;
pub use witness::CompositionFault;
pub use witness::CompositionInput;
pub use witness::Invocation;
pub use witness::ViewEvidence;

#[cfg(test)]
mod tests;
