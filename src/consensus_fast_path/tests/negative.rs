use super::super::*;

const FIRST_COMMAND: u64 = 1;
const FIRST_KEY: u64 = 5;

pub(super) fn fault_class(result: Result<(), CertificateFaultSite>) -> Option<CertificateFault> {
    result.err().map(|site| site.class)
}

// r[verify consensus.fast-path.quorum]
// r[verify consensus.fast-path.fixtures]
#[test]
fn unsupported_fault_budgets_reject_without_overflow() {
    assert_eq!(cluster_size(0), None);
    assert_eq!(cluster_size(u32::MAX), None);
    assert_eq!(accelerated_quorum(u32::MAX), None);
    assert_eq!(majority(MAX_FAULTS + 1), None);
}

// r[verify consensus.fast-path.certificate]
// r[verify consensus.fast-path.fixtures]
#[test]
fn wrong_size_duplicate_and_out_of_range_replicas_reject() {
    let mut below = super::five_node_certificate(FIRST_COMMAND);
    below.acknowledgements.pop();
    assert_eq!(fault_class(admit_certificate(&below)), Some(CertificateFault::WrongQuorumSize));

    let mut duplicate = super::five_node_certificate(FIRST_COMMAND);
    duplicate.acknowledgements[1] = super::ack(0, FIRST_COMMAND, super::CURRENT_VIEW);
    assert_eq!(fault_class(admit_certificate(&duplicate)), Some(CertificateFault::DuplicateReplica));

    let mut outside = super::five_node_certificate(FIRST_COMMAND);
    outside.acknowledgements[3] = super::ack(5, FIRST_COMMAND, super::CURRENT_VIEW);
    assert_eq!(fault_class(admit_certificate(&outside)), Some(CertificateFault::ReplicaOutsideCluster));
}

// r[verify consensus.fast-path.certificate]
#[test]
fn mixed_command_and_view_hazards_reject() {
    let mut mixed_command = super::five_node_certificate(FIRST_COMMAND);
    mixed_command.acknowledgements[3].command = super::command_id(2);
    assert_eq!(fault_class(admit_certificate(&mixed_command)), Some(CertificateFault::MixedCommand));

    let mut mixed_acceleration = super::five_node_certificate(FIRST_COMMAND);
    mixed_acceleration.acknowledgements[2].acceleration_view = super::view(10);
    assert_eq!(fault_class(admit_certificate(&mixed_acceleration)), Some(CertificateFault::MixedAccelerationView));

    let mut mixed_base = super::five_node_certificate(FIRST_COMMAND);
    mixed_base.acknowledgements[2].base_view = super::view(10);
    assert_eq!(fault_class(admit_certificate(&mixed_base)), Some(CertificateFault::MixedBaseView));

    let mut cross_path = super::five_node_certificate(FIRST_COMMAND);
    for acknowledgement in &mut cross_path.acknowledgements {
        acknowledgement.base_view = super::view(10);
    }
    cross_path.bound_base_view = super::view(10);
    assert_eq!(fault_class(admit_certificate(&cross_path)), Some(CertificateFault::CrossPathViewMismatch));

    let stale = super::certificate(&[0, 1, 2, 3], FIRST_COMMAND, 8, super::FAULT_BUDGET);
    let mut stale = stale;
    stale.bound_base_view = super::view(super::CURRENT_VIEW);
    assert_eq!(fault_class(admit_certificate(&stale)), Some(CertificateFault::StaleView));
}

// r[verify consensus.fast-path.certificate]
#[test]
fn proposer_duplicates_ranges_and_missing_promises_reject() {
    let mut duplicate = super::five_node_certificate(FIRST_COMMAND);
    duplicate.active_proposers.push(ProposerId(super::ACTIVE_PROPOSER));
    assert_eq!(fault_class(admit_certificate(&duplicate)), Some(CertificateFault::DuplicateProposer));

    let mut outside = super::five_node_certificate(FIRST_COMMAND);
    outside.active_proposers = vec![ProposerId(5)];
    assert_eq!(fault_class(admit_certificate(&outside)), Some(CertificateFault::ProposerOutsideCluster));

    let mut missing = super::five_node_certificate(FIRST_COMMAND);
    missing.active_proposers.push(ProposerId(1));
    assert_eq!(fault_class(admit_certificate(&missing)), Some(CertificateFault::MissingProposerPromise));
}

// r[verify consensus.fast-path.quorum]
#[test]
fn malformed_recovery_majorities_reject() {
    let certificate = super::five_node_certificate(FIRST_COMMAND);
    let too_small = vec![super::replica(0), super::replica(1)];
    assert_eq!(
        recovery_visibility(&certificate, &too_small).expect_err("majority size must be exact").class,
        CertificateFault::InvalidRecoveryMajority
    );
    let duplicate = vec![super::replica(0), super::replica(0), super::replica(1)];
    assert!(recovery_visibility(&certificate, &duplicate).is_err());
}

// r[verify consensus.fast-path.certificate]
#[test]
fn valid_conflicting_certificates_must_share_a_replica() {
    let left = super::five_node_certificate(FIRST_COMMAND);
    let right = super::certificate(&[1, 2, 3, 4], 2, super::CURRENT_VIEW, super::FAULT_BUDGET);
    assert_eq!(admit_certificate(&right), Ok(()));
    assert_eq!(conflicting_certificates_admitted(&left, &right), Ok(false));
}

// r[verify consensus.fast-path.base-order]
#[test]
fn buffering_and_execution_reordering_reject() {
    let buffering = BaseCapability {
        receive_to_propose_ordered: false,
        propose_to_log_ordered: true,
        log_to_execute_ordered: true,
    };
    assert!(!receive_time_ack_valid(buffering));
    assert!(proposal_bound_ack_valid(buffering));

    let execution_reorder = BaseCapability {
        receive_to_propose_ordered: true,
        propose_to_log_ordered: true,
        log_to_execute_ordered: false,
    };
    assert!(!base_admits(execution_reorder));

    let first = Positions {
        receive: 0,
        proposal: 1,
        log: 1,
        execution: 1,
    };
    let second = Positions {
        receive: 1,
        proposal: 0,
        log: 0,
        execution: 0,
    };
    assert!(composed_execution_sequence(super::transparent_capability(), &first, &second).is_err());
}

// r[verify consensus.fast-path.conflict]
#[test]
fn same_key_write_and_false_non_conflict_fail_the_contract() {
    let store = Store::default();
    let write = KeyValue::Write {
        key: FIRST_KEY,
        value: 10,
    };
    let read = KeyValue::Read { key: FIRST_KEY };
    assert!(conflicts(write, read));
    assert!(!commute_under_contract(&store, write, read));
    assert!(adjacent_swap_preserves(true, &store, write, read).is_err());
    assert!(conflicts(KeyValue::Unsupported { effect: 7 }, KeyValue::Read { key: FIRST_KEY }));
}

// r[verify consensus.fast-path.recovery]
#[test]
fn recovery_rejects_unselected_and_interrupted_transitions() {
    let command = super::command_id(FIRST_COMMAND);
    let mut snapshot = RecoverySnapshot::new(super::view(5), super::view(6)).expect("view advances");
    assert!(snapshot.commit_marker(super::BASE_LOG_LENGTH).is_err());
    snapshot.begin(vec![command]).expect("begin");
    assert_eq!(snapshot.accept(vec![super::command_id(2)]), Err(RecoveryFault::CommandOutsideRecoverySet));
    snapshot.accept(vec![command]).expect("accept");
    assert_eq!(snapshot.commit_marker(u64::MAX), Err(RecoveryFault::MarkerIndexOverflow));
    assert!(snapshot.finish().is_err());
    assert!(snapshot.carry_forward(super::view(6)).is_err());
}

// r[verify consensus.fast-path.recovery]
#[test]
fn stale_conflicting_predecessor_rejects() {
    let command = super::command_id(FIRST_COMMAND);
    let mut snapshot = RecoverySnapshot::new(super::view(5), super::view(6)).expect("view advances");
    snapshot.begin(vec![command]).expect("begin");
    snapshot.accept(vec![command]).expect("accept");
    snapshot.commit_marker(super::BASE_LOG_LENGTH).expect("marker");
    snapshot.finish().expect("finish");
    assert_eq!(admit_recovery_order(&snapshot, command, 2, 1, true), Err(RecoveryFault::ConflictingPredecessor));
}

// r[verify consensus.fast-path.dual-path]
#[test]
fn identity_mismatch_and_premature_expiry_reject() {
    let record = DedupRecord {
        session: 7,
        sequence: 1,
        command: super::command_id(FIRST_COMMAND),
        response: Some(Response::Ack),
        retain_until: 4,
    };
    let wrong_session = OriginalCompletion {
        session: 8,
        sequence: 1,
        command: super::command_id(FIRST_COMMAND),
        observed_at: 4,
    };
    assert_eq!(converge(&record, &wrong_session), Err(ConvergenceFault::SessionMismatch));

    let expired = OriginalCompletion {
        session: 7,
        sequence: 1,
        command: super::command_id(FIRST_COMMAND),
        observed_at: 5,
    };
    assert_eq!(converge(&record, &expired), Err(ConvergenceFault::DedupExpired));
}

// r[verify consensus.fast-path.linearization]
#[test]
fn missing_real_time_order_rejects_composition() {
    let certificate = super::five_node_certificate(FIRST_COMMAND);
    let store = Store::default();
    let first = Invocation {
        operation: super::history_operation(FIRST_COMMAND, FIRST_KEY, 0, 3),
        positions: super::positions(0),
    };
    let second = Invocation {
        operation: super::history_operation(2, FIRST_KEY, 2, 4),
        positions: super::positions(1),
    };
    let record = DedupRecord {
        session: 7,
        sequence: 1,
        command: super::command_id(FIRST_COMMAND),
        response: Some(Response::Ack),
        retain_until: 10,
    };
    let completion = OriginalCompletion {
        session: 7,
        sequence: 1,
        command: super::command_id(FIRST_COMMAND),
        observed_at: 5,
    };
    let input = CompositionInput {
        certificate: &certificate,
        capability: super::transparent_capability(),
        state: &store,
        first_command: KeyValue::Write {
            key: FIRST_KEY,
            value: 10,
        },
        second_command: KeyValue::Read { key: FIRST_KEY },
        first: &first,
        second: &second,
        view: ViewEvidence::Stable {
            acceleration: super::view(super::CURRENT_VIEW),
            base: super::view(super::CURRENT_VIEW),
        },
        record: &record,
        completion: &completion,
    };
    assert_eq!(admit_composition(&input), Err(CompositionFault::MissingRealtimeOrder));
}

// r[verify consensus.fast-path.boundary]
#[test]
fn unsupported_capability_scopes_reject() {
    let byzantine = CapabilityScope {
        fault_model: FaultModel::Byzantine,
        membership: MembershipMode::Static,
        command_mode: CommandMode::Single,
    };
    assert_eq!(admit_scope(byzantine), Err(ScopeFault::ByzantineFaults));

    let dynamic = CapabilityScope {
        fault_model: FaultModel::Crash,
        membership: MembershipMode::Dynamic,
        command_mode: CommandMode::Single,
    };
    assert_eq!(admit_scope(dynamic), Err(ScopeFault::DynamicMembership));

    let transaction = CapabilityScope {
        fault_model: FaultModel::Crash,
        membership: MembershipMode::Static,
        command_mode: CommandMode::Interactive,
    };
    assert_eq!(admit_scope(transaction), Err(ScopeFault::InteractiveTransaction));

    let cross_group = CapabilityScope {
        fault_model: FaultModel::Crash,
        membership: MembershipMode::Static,
        command_mode: CommandMode::CrossGroup,
    };
    assert_eq!(admit_scope(cross_group), Err(ScopeFault::CrossGroupAtomicity));
}
