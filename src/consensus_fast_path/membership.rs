//! Quorum arithmetic and finite same-view certificate admission.

/// Maximum admitted fault budget.
pub const MAX_FAULTS: u32 = 1_000_000;

const QUORUM_DIVISOR: u32 = 2;

fn budget_admitted(faults: u32) -> bool {
    faults > 0 && faults <= MAX_FAULTS
}

/// Cluster size `n = 2f + 1`.
// r[impl consensus.fast-path.quorum]
#[must_use]
pub fn cluster_size(faults: u32) -> Option<u32> {
    if !budget_admitted(faults) {
        return None;
    }
    faults.checked_mul(QUORUM_DIVISOR)?.checked_add(1)
}

/// Original recovery majority `f + 1`.
// r[impl consensus.fast-path.quorum]
#[must_use]
pub fn majority(faults: u32) -> Option<u32> {
    if !budget_admitted(faults) {
        return None;
    }
    faults.checked_add(1)
}

/// Recovery support `ceil(f/2) + 1`.
// r[impl consensus.fast-path.quorum]
#[must_use]
pub fn recovery_support(faults: u32) -> Option<u32> {
    if !budget_admitted(faults) {
        return None;
    }
    faults.div_ceil(QUORUM_DIVISOR).checked_add(1)
}

/// Accelerated quorum `f + ceil(f/2) + 1`.
// r[impl consensus.fast-path.quorum]
#[must_use]
pub fn accelerated_quorum(faults: u32) -> Option<u32> {
    faults.checked_add(recovery_support(faults)?)
}

/// Accelerated failure tolerance `n - quorum = floor(f/2)`.
// r[impl consensus.fast-path.quorum]
#[must_use]
pub fn accelerated_failure_tolerance(faults: u32) -> Option<u32> {
    cluster_size(faults)?.checked_sub(accelerated_quorum(faults)?)
}

/// Minimum quorum/recovery-majority intersection.
// r[impl consensus.fast-path.quorum]
#[must_use]
pub fn intersection_lower_bound(faults: u32) -> Option<u32> {
    accelerated_quorum(faults)?.checked_add(majority(faults)?)?.checked_sub(cluster_size(faults)?)
}

/// One accelerated replica acknowledgement.
// r[impl consensus.fast-path.certificate]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Acknowledgement {
    pub replica: super::vocabulary::ReplicaId,
    pub command: super::vocabulary::CommandId,
    pub acceleration_view: super::vocabulary::ViewId,
    pub base_view: super::vocabulary::ViewId,
    pub promised_proposers: Vec<super::vocabulary::ProposerId>,
}

/// One candidate certificate and its current base-view binding.
// r[impl consensus.fast-path.certificate]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Certificate {
    pub acknowledgements: Vec<Acknowledgement>,
    pub active_proposers: Vec<super::vocabulary::ProposerId>,
    pub bound_base_view: super::vocabulary::ViewId,
    pub fault_budget: u32,
}

/// Stable certificate rejection classes.
// r[impl consensus.fast-path.certificate]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CertificateFault {
    InvalidFaultBudget,
    EmptyTable,
    WrongQuorumSize,
    DuplicateReplica,
    ReplicaOutsideCluster,
    DuplicateProposer,
    ProposerOutsideCluster,
    MixedCommand,
    MixedAccelerationView,
    MixedBaseView,
    CrossPathViewMismatch,
    StaleView,
    MissingProposerPromise,
    InvalidRecoveryMajority,
}

/// Deterministic certificate rejection metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CertificateFaultSite {
    pub class: CertificateFault,
    pub index: Option<u32>,
}

fn reject(class: CertificateFault, index: Option<u32>) -> Result<(), CertificateFaultSite> {
    Err(CertificateFaultSite { class, index })
}

fn values_unique<T: Eq>(items: &[T]) -> bool {
    items.iter().enumerate().all(|(index, item)| !items[..index].contains(item))
}

fn index_value(index: usize) -> Option<u32> {
    u32::try_from(index).ok()
}

fn validate_replicas(acknowledgements: &[Acknowledgement], cluster: u32) -> Result<(), CertificateFaultSite> {
    if acknowledgements
        .iter()
        .enumerate()
        .any(|(index, ack)| acknowledgements[..index].iter().any(|prior| prior.replica == ack.replica))
    {
        return reject(CertificateFault::DuplicateReplica, None);
    }
    if let Some(index) = acknowledgements.iter().position(|ack| ack.replica.0 >= cluster) {
        return reject(CertificateFault::ReplicaOutsideCluster, index_value(index));
    }
    Ok(())
}

fn validate_proposers(certificate: &Certificate, cluster: u32) -> Result<(), CertificateFaultSite> {
    if !values_unique(&certificate.active_proposers) {
        return reject(CertificateFault::DuplicateProposer, None);
    }
    if let Some(index) = certificate.active_proposers.iter().position(|proposer| proposer.0 >= cluster) {
        return reject(CertificateFault::ProposerOutsideCluster, index_value(index));
    }
    for (index, proposer) in certificate.active_proposers.iter().enumerate() {
        if !certificate.acknowledgements.iter().any(|ack| ack.promised_proposers.contains(proposer)) {
            return reject(CertificateFault::MissingProposerPromise, index_value(index));
        }
    }
    Ok(())
}

/// Admits one exact finite certificate against all structural gates.
// r[impl consensus.fast-path.certificate]
pub fn admit_certificate(certificate: &Certificate) -> Result<(), CertificateFaultSite> {
    let Some(cluster) = cluster_size(certificate.fault_budget) else {
        return reject(CertificateFault::InvalidFaultBudget, None);
    };
    let acknowledgements = &certificate.acknowledgements;
    if acknowledgements.is_empty() {
        return reject(CertificateFault::EmptyTable, None);
    }
    let Some(required) = accelerated_quorum(certificate.fault_budget) else {
        return reject(CertificateFault::InvalidFaultBudget, None);
    };
    if u32::try_from(acknowledgements.len()).ok() != Some(required) {
        return reject(CertificateFault::WrongQuorumSize, None);
    }
    validate_replicas(acknowledgements, cluster)?;
    if acknowledgements.iter().any(|ack| ack.command != acknowledgements[0].command) {
        return reject(CertificateFault::MixedCommand, None);
    }
    if acknowledgements.iter().any(|ack| ack.acceleration_view != acknowledgements[0].acceleration_view) {
        return reject(CertificateFault::MixedAccelerationView, None);
    }
    if acknowledgements.iter().any(|ack| ack.base_view != acknowledgements[0].base_view) {
        return reject(CertificateFault::MixedBaseView, None);
    }
    if acknowledgements[0].acceleration_view != acknowledgements[0].base_view {
        return reject(CertificateFault::CrossPathViewMismatch, None);
    }
    if acknowledgements[0].base_view != certificate.bound_base_view {
        return reject(CertificateFault::StaleView, None);
    }
    validate_proposers(certificate, cluster)
}

/// Returns the certificate command after admission.
// r[impl consensus.fast-path.certificate]
pub fn certified_command(certificate: &Certificate) -> Result<super::vocabulary::CommandId, CertificateFaultSite> {
    admit_certificate(certificate)?;
    Ok(certificate.acknowledgements[0].command)
}

/// Returns admitted replica identities.
// r[impl consensus.fast-path.certificate]
pub fn replica_set(certificate: &Certificate) -> Result<Vec<super::vocabulary::ReplicaId>, CertificateFaultSite> {
    admit_certificate(certificate)?;
    Ok(certificate.acknowledgements.iter().map(|ack| ack.replica).collect())
}

fn validate_recovery_set(
    certificate: &Certificate,
    replies: &[super::vocabulary::ReplicaId],
) -> Result<(), CertificateFaultSite> {
    let Some(cluster) = cluster_size(certificate.fault_budget) else {
        return reject(CertificateFault::InvalidFaultBudget, None);
    };
    if u32::try_from(replies.len()).ok() != majority(certificate.fault_budget) {
        return reject(CertificateFault::InvalidRecoveryMajority, None);
    }
    if !values_unique(replies) || replies.iter().any(|replica| replica.0 >= cluster) {
        return reject(CertificateFault::InvalidRecoveryMajority, None);
    }
    Ok(())
}

/// Counts the actual overlap with one admitted recovery majority.
// r[impl consensus.fast-path.quorum]
pub fn recovery_visibility(
    certificate: &Certificate,
    recovery_majority: &[super::vocabulary::ReplicaId],
) -> Result<u32, CertificateFaultSite> {
    let replicas = replica_set(certificate)?;
    validate_recovery_set(certificate, recovery_majority)?;
    let overlap = replicas.iter().filter(|replica| recovery_majority.contains(replica)).count();
    u32::try_from(overlap).map_err(|_| CertificateFaultSite {
        class: CertificateFault::InvalidRecoveryMajority,
        index: None,
    })
}

/// Reports whether conflicting same-view certificates can avoid double-ack.
// r[impl consensus.fast-path.certificate]
pub fn conflicting_certificates_admitted(
    left: &Certificate,
    right: &Certificate,
) -> Result<bool, CertificateFaultSite> {
    admit_certificate(left)?;
    admit_certificate(right)?;
    let is_same_view = left.acknowledgements[0].acceleration_view == right.acknowledgements[0].acceleration_view;
    let has_different_commands = left.acknowledgements[0].command != right.acknowledgements[0].command;
    let has_shared_replica = left
        .acknowledgements
        .iter()
        .any(|ack| right.acknowledgements.iter().any(|other| other.replica == ack.replica));
    Ok(is_same_view && has_different_commands && !has_shared_replica)
}
